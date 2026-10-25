from typing import Literal

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware

from . import cost, pipeline
from .models import ApproveRequest, CreateJobRequest, Event, Job, JobCost
from .store import JobNotFound, JobStore

app = FastAPI(title="Theseus API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000"],
    allow_methods=["*"],
    allow_headers=["*"],
)

store = JobStore()


def _get(job_id: str) -> Job:
    try:
        return store.get(job_id)
    except JobNotFound:
        raise HTTPException(status_code=404, detail=f"job {job_id} not found") from None


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.post("/jobs", response_model=Job)
def create_job(
    request: CreateJobRequest,
    scenario: Literal["completed", "failed"] = Query("completed"),
) -> Job:
    try:
        pipe = pipeline.get_pipeline(store, scenario=scenario)
    except ValueError as exc:
        raise HTTPException(status_code=500, detail=str(exc)) from None
    job = store.create(request.library, pipe.runner, pipe.initial_agents(request))
    pipeline.start(store, pipe, job.id, request)
    return job


@app.get("/jobs/{job_id}", response_model=Job)
def get_job(job_id: str) -> Job:
    return _get(job_id)


@app.get("/jobs/{job_id}/events", response_model=list[Event])
def get_events(job_id: str, after: int = Query(0, ge=0)) -> list[Event]:
    try:
        return store.events(job_id, after=after)
    except JobNotFound:
        raise HTTPException(status_code=404, detail=f"job {job_id} not found") from None


@app.get("/jobs/{job_id}/result", response_model=Job)
def get_result(job_id: str) -> Job:
    job = _get(job_id)
    if job.status != "completed" or job.result is None:
        raise HTTPException(status_code=409, detail=f"job {job_id} is {job.status}; result not ready")
    return job


@app.post("/jobs/{job_id}/approve", response_model=Job)
def approve(job_id: str, body: ApproveRequest) -> Job:
    job = _get(job_id)
    if job.status != "completed":
        raise HTTPException(status_code=409, detail=f"job {job_id} is {job.status}, not completed")
    if not job.approval.required:
        raise HTTPException(status_code=409, detail=f"job {job_id} does not require approval")
    if job.approval.approved_by is not None:
        raise HTTPException(status_code=409, detail=f"job {job_id} already approved")
    pipe = pipeline.get_pipeline(store)
    try:
        pr_url = pipe.open_pr(job, body.approved_by)
    except NotImplementedError as exc:
        raise HTTPException(status_code=501, detail=str(exc)) from None
    store.emit(job_id, "approved", None, {"approved_by": body.approved_by, "pr_url": pr_url})
    return store.get(job_id)


# Proposed contract addition, not yet in contracts/README.md.
@app.get("/jobs/{job_id}/cost", response_model=JobCost)
def get_cost(job_id: str) -> dict:
    _get(job_id)
    return cost.job_cost(job_id)
