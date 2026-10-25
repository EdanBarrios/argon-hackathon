"""In-memory job store guarded by a single lock.

Pipelines never touch Job objects directly. They call `emit(...)`, which appends an
Event and folds it into the job state, so a polling UI sees agent lanes move.
"""

import secrets
import threading
from datetime import datetime
from typing import Any

from .models import TERMINAL, Agent, Approval, Error, Event, Job, Result


def now_iso() -> str:
    return datetime.now().astimezone().isoformat(timespec="seconds")


class JobNotFound(KeyError):
    pass


class JobStore:
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._jobs: dict[str, Job] = {}
        self._events: dict[str, list[Event]] = {}

    # ---- reads (return deep copies so callers never see a half-applied update)

    def get(self, job_id: str) -> Job:
        with self._lock:
            return self._job(job_id).model_copy(deep=True)

    def events(self, job_id: str, after: int = 0) -> list[Event]:
        with self._lock:
            self._job(job_id)
            return [e.model_copy(deep=True) for e in self._events[job_id] if e.seq > after]

    def all_jobs(self) -> list[Job]:
        """All jobs, newest first."""
        with self._lock:
            return [j.model_copy(deep=True) for j in reversed(self._jobs.values())]

    def exists(self, job_id: str) -> bool:
        with self._lock:
            return job_id in self._jobs

    # ---- writes

    def create(self, library: str, runner: str, agents: list[Agent]) -> Job:
        ts = now_iso()
        job = Job(
            id=f"job_{secrets.token_hex(3)}",
            library=library,
            status="queued",
            created_at=ts,
            updated_at=ts,
            runner=runner,
            agents=agents,
            result=None,
            approval=Approval(required=True),
            error=None,
        )
        with self._lock:
            while job.id in self._jobs:
                job.id = f"job_{secrets.token_hex(3)}"
            self._jobs[job.id] = job
            self._events[job.id] = []
            return job.model_copy(deep=True)

    def apply_snapshot(self, job_id: str, fields: dict[str, Any]) -> None:
        """Overwrite job fields wholesale (used for final result, agents, error)."""
        with self._lock:
            job = self._job(job_id)
            keep = {"id": job.id, "created_at": job.created_at, "updated_at": now_iso()}
            self._jobs[job_id] = Job.model_validate({**job.model_dump(), **fields, **keep})

    def emit(self, job_id: str, event_type: str, agent_id: str | None, data: dict[str, Any]) -> Event:
        with self._lock:
            job = self._job(job_id)
            events = self._events[job_id]
            event = Event(
                seq=len(events) + 1,
                at=now_iso(),
                job_id=job_id,
                agent_id=agent_id,
                type=event_type,
                data=data,
            )
            events.append(event)
            self._reduce(job, event)
            job.updated_at = event.at
            return event.model_copy(deep=True)

    # ---- internals

    def _job(self, job_id: str) -> Job:
        try:
            return self._jobs[job_id]
        except KeyError:
            raise JobNotFound(job_id) from None

    @staticmethod
    def _agent(job: Job, agent_id: str | None, data: dict[str, Any]) -> Agent | None:
        if agent_id is None:
            return None
        for agent in job.agents:
            if agent.id == agent_id:
                return agent
        # Unknown agent: a real pipeline may spawn agents the plan did not list up front.
        agent = Agent(
            id=agent_id,
            module=data.get("module", agent_id.removeprefix("agent_")),
            status="queued",
            iteration=0,
            match_rate=0.0,
        )
        job.agents.append(agent)
        return agent

    def _reduce(self, job: Job, event: Event) -> None:
        data = event.data
        agent = self._agent(job, event.agent_id, data)
        if agent is not None and data.get("interaction_id"):
            agent.interaction_id = data["interaction_id"]

        if event.type == "job_status":
            status = data["status"]
            job.status = status
            if status in ("running", "verifying"):
                plan = data.get("plan")
                for a in job.agents:
                    if a.status in TERMINAL:
                        continue
                    if plan is None or a.module in plan:
                        a.status = status
            if status == "failed" and data.get("error"):
                job.error = Error.model_validate(data["error"])
            if status == "completed" and data.get("result"):
                job.result = Result.model_validate(data["result"])
        elif event.type in ("translate", "repair") and agent is not None:
            agent.status = "running"
            agent.iteration = data.get("iteration", agent.iteration)
        elif event.type == "verify" and agent is not None:
            agent.iteration = data.get("iteration", agent.iteration)
            agent.match_rate = data.get("match_rate", agent.match_rate)
            agent.status = "completed" if agent.match_rate >= 1.0 else "running"
        elif event.type == "approval_required":
            job.approval.required = True
        elif event.type == "approved":
            job.approval.approved_by = data.get("approved_by")
            job.approval.pr_url = data.get("pr_url")
            job.approval.pr_status = data.get("pr_status")
            job.approval.pr_branch = data.get("pr_branch")
            job.approval.pr_detail = data.get("pr_detail")
