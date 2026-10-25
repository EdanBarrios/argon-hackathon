"""Pipeline seam: mock replay of the contract fixtures, or the real Argon pipeline.

THESEUS_MODE=mock (default) replays contracts/fixtures/job-events.json.
THESEUS_MODE=real uses RealPipeline (not wired yet).

Every pipeline runs in a background thread. Requests never wait on pipeline work;
the UI polls GET /jobs/{id} and GET /jobs/{id}/events.
"""

import json
import os
import threading
import time
from datetime import datetime
from pathlib import Path
from typing import Any, Callable, Protocol

from . import cost
from .models import Agent, CreateJobRequest, Job
from .store import JobStore

# emit(event_type, agent_id, data): appends an event and folds it into the job state.
Emit = Callable[[str, str | None, dict[str, Any]], None]

FIXTURES_DIR = Path(
    os.environ.get("THESEUS_FIXTURES_DIR", Path(__file__).resolve().parents[2] / "contracts" / "fixtures")
)
DEFAULT_MOCK_DURATION = 30.0


def load_fixture(name: str) -> Any:
    """Read a fixture from disk on every call, so fixture corrections flow through."""
    return json.loads((FIXTURES_DIR / name).read_text())


def mode() -> str:
    return os.environ.get("THESEUS_MODE", "mock").strip().lower() or "mock"


class Pipeline(Protocol):
    runner: str

    def initial_agents(self, request: CreateJobRequest) -> list[Agent]:
        """Agents to show at creation time, before the planner has run."""
        ...

    def run(self, job_id: str, request: CreateJobRequest, emit: Emit) -> None:
        """Do the work, reporting progress only through emit. Runs in a background thread."""
        ...

    def open_pr(self, job: Job, approved_by: str) -> str:
        """Open the PR for an approved job and return its URL."""
        ...


def _replace_id(value: Any, old: str, new: str) -> Any:
    if isinstance(value, str):
        return value.replace(old, new)
    if isinstance(value, list):
        return [_replace_id(v, old, new) for v in value]
    if isinstance(value, dict):
        return {k: _replace_id(v, old, new) for k, v in value.items()}
    return value


def _ts(s: str) -> float:
    return datetime.fromisoformat(s).timestamp()


class MockPipeline:
    """Replays job-events.json over ~duration seconds, scaled from the fixture timestamps.

    scenario="completed" ends equal to job-completed.json (with this job's id).
    scenario="failed" replays the first verify round, then ends in the job-failed.json shape.
    """

    runner = "local"

    def __init__(self, store: JobStore, scenario: str = "completed", duration: float | None = None):
        self.store = store
        self.scenario = scenario
        if duration is None:
            duration = float(os.environ.get("THESEUS_MOCK_DURATION", DEFAULT_MOCK_DURATION))
        self.duration = duration

    def initial_agents(self, request: CreateJobRequest) -> list[Agent]:
        completed = load_fixture("job-completed.json")
        return [
            Agent(id=a["id"], module=a["module"], status="queued", iteration=0, match_rate=0.0,
                  interaction_id=a["interaction_id"])
            for a in completed["agents"]
        ]

    def _timeline(self, job_id: str) -> tuple[list[dict[str, Any]], dict[str, Any]]:
        events = load_fixture("job-events.json")
        final_name = "job-failed.json" if self.scenario == "failed" else "job-completed.json"
        final = load_fixture(final_name)
        fixture_id = events[0]["job_id"]
        events = _replace_id(events, fixture_id, job_id)
        final = _replace_id(final, final["id"], job_id)

        t0 = _ts(events[0]["at"])
        span = max(_ts(events[-1]["at"]) - t0, 1e-9)
        for e in events:
            e["offset"] = (_ts(e["at"]) - t0) / span

        if self.scenario == "failed":
            first_repair = next(i for i, e in enumerate(events) if e["type"] == "repair")
            events = events[:first_repair]
            events.append({
                "offset": 1.0,
                "agent_id": None,
                "type": "job_status",
                "data": {"status": "failed", "error": final["error"]},
            })
        return events, final

    def run(self, job_id: str, request: CreateJobRequest, emit: Emit) -> None:
        events, final = self._timeline(job_id)
        start = time.monotonic()
        for e in events:
            delay = start + e["offset"] * self.duration - time.monotonic()
            if delay > 0:
                time.sleep(delay)
            is_terminal = e["type"] == "job_status" and e["data"].get("status") in ("completed", "failed")
            if is_terminal:
                # Fill result/agents/error before the status flips, so a poller that
                # sees "completed" always sees the result too.
                self.store.apply_snapshot(job_id, {
                    k: final[k] for k in ("agents", "result", "approval", "error", "runner")
                })
            self._fake_usage(job_id, e)
            emit(e["type"], e["agent_id"], e["data"])

    @staticmethod
    def _fake_usage(job_id: str, e: dict[str, Any]) -> None:
        """Deterministic fake token usage so GET /jobs/{id}/cost shows numbers in mock mode."""
        if e["type"] == "job_status" and "plan" in e["data"]:
            cost.record_usage(job_id, "gemini-4-argon", {
                "total_input_tokens": 120_000, "total_cached_tokens": 0,
                "total_output_tokens": 4_000, "total_thought_tokens": 12_000,
            })
        elif e["type"] in ("translate", "repair"):
            cost.record_usage(job_id, "gemini-4-argon", {
                "total_input_tokens": 40_000, "total_cached_tokens": 30_000,
                "total_output_tokens": 6_000, "total_thought_tokens": 8_000,
            })

    def open_pr(self, job: Job, approved_by: str) -> str:
        return f"https://github.com/theseus-demo/{job.library}-rs/pull/1"


class RealPipeline:
    """Real Argon pipeline. Not wired yet.

    C will wire run() to A's translate/repair functions in backend/ai/, plus
    scripts/build_rust.sh and backend/harness/verify.sh, reporting progress via emit
    and token usage via cost.record_usage.
    """

    runner = "local"

    def __init__(self, store: JobStore):
        self.store = store

    def initial_agents(self, request: CreateJobRequest) -> list[Agent]:
        return []

    def run(self, job_id: str, request: CreateJobRequest, emit: Emit) -> None:
        raise NotImplementedError(
            "RealPipeline is not wired yet: it needs backend/ai translate/repair, "
            "scripts/build_rust.sh and backend/harness/verify.sh. Use THESEUS_MODE=mock."
        )

    def open_pr(self, job: Job, approved_by: str) -> str:
        raise NotImplementedError("RealPipeline.open_pr is not wired yet.")


def get_pipeline(store: JobStore, scenario: str = "completed") -> Pipeline:
    m = mode()
    if m == "real":
        return RealPipeline(store)
    if m == "mock":
        return MockPipeline(store, scenario=scenario)
    raise ValueError(f"Unknown THESEUS_MODE={m!r}; expected 'mock' or 'real'.")


def start(store: JobStore, pipeline: Pipeline, job_id: str, request: CreateJobRequest) -> threading.Thread:
    """Run the pipeline in a daemon thread. Any exception fails the job instead of hanging it."""

    def emit(event_type: str, agent_id: str | None, data: dict[str, Any]) -> None:
        store.emit(job_id, event_type, agent_id, data)

    def target() -> None:
        try:
            pipeline.run(job_id, request, emit)
        except Exception as exc:  # noqa: BLE001 - surface every failure to the UI
            code = "not_implemented" if isinstance(exc, NotImplementedError) else "pipeline_error"
            emit("job_status", None, {
                "status": "failed",
                "error": {"code": code, "message": str(exc), "agent_id": None, "retryable": False},
            })

    thread = threading.Thread(target=target, name=f"pipeline-{job_id}", daemon=True)
    thread.start()
    return thread
