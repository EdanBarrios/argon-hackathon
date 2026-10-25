import base64
import binascii
from typing import Any, Literal

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware

from . import cost, harness, pipeline, try_input
from .models import ApproveRequest, CreateJobRequest, Event, Job, JobCost, TryRequest
from .store import JobNotFound, JobStore

app = FastAPI(title="Theseus API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000"],
    # Any local port, so a second UI (e.g. next dev -p 3001) can reach a demo backend.
    allow_origin_regex=r"http://(localhost|127\.0\.0\.1)(:\d+)?",
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
    if pipeline.mode() == "real" and request.start_from != "fresh":
        try:
            harness.load_candidate(request.start_from)
        except harness.HarnessError as exc:
            raise HTTPException(status_code=400, detail=str(exc)) from None
    job = store.create(request.library, pipe.runner, pipe.initial_agents(request))
    pipeline.start(store, pipe, job.id, request)
    return job


@app.get("/jobs", response_model=list[Job])
def list_jobs() -> list[Job]:
    """Jobs this server has run, newest first (in memory; lets the UI load a pre-run)."""
    return store.all_jobs()


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


@app.get("/jobs/{job_id}/cost", response_model=JobCost)
def get_cost(job_id: str) -> dict:
    _get(job_id)
    return cost.job_cost(job_id)


@app.post("/try")
def try_one(body: TryRequest) -> dict[str, Any]:
    """Run one input through C v1.7.19, the current Rust binary and (for CVE inputs) old C v1.7.17.

    Plain `def`, so FastAPI runs it in its threadpool; it takes about 0.3 s, or about
    5 s when an ASan report has to be symbolized.
    """
    if body.library != "cjson":
        raise HTTPException(status_code=400, detail=f"/try supports library 'cjson' only, got {body.library!r}")
    if (body.input is None) == (body.input_b64 is None):
        raise HTTPException(status_code=400, detail="send exactly one of input or input_b64")
    if body.input is not None:
        data = body.input.encode("utf-8")
    else:
        try:
            data = base64.b64decode(body.input_b64 or "", validate=True)
        except (binascii.Error, ValueError):
            raise HTTPException(status_code=400, detail="input_b64 is not valid base64") from None
    if len(data) > try_input.MAX_INPUT_BYTES:
        raise HTTPException(
            status_code=413, detail=f"input is {len(data)} bytes; the limit is {try_input.MAX_INPUT_BYTES}"
        )
    try:
        return try_input.run_try(data, compare_old=body.compare_old)
    except harness.HarnessError as exc:
        raise HTTPException(status_code=503, detail=str(exc)) from None
