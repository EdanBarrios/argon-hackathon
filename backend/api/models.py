"""Pydantic models mirroring contracts/fixtures/*.json. Field names must match the fixtures."""

from typing import Any, Literal

from pydantic import BaseModel

Status = Literal["queued", "running", "verifying", "completed", "failed", "cancelled"]
EventType = Literal[
    "job_status",
    "translate",
    "verify",
    "divergence",
    "crash",
    "repair",
    "approval_required",
    "approved",
]
TERMINAL: set[str] = {"completed", "failed", "cancelled"}


class Agent(BaseModel):
    id: str
    module: str
    status: Status
    iteration: int
    match_rate: float
    interaction_id: str | None = None


class Divergence(BaseModel):
    input: str
    c_output: str
    rust_output: str
    function: str
    resolved: bool


class Crash(BaseModel):
    input: str
    sanitizer_report: str
    rust_behavior: str


class Result(BaseModel):
    baseline_match_rate: float
    final_match_rate: float
    behaviors_tested: int
    divergences: list[Divergence]
    crashes: list[Crash]


class Approval(BaseModel):
    required: bool
    approved_by: str | None = None
    pr_url: str | None = None


class Error(BaseModel):
    code: str
    message: str
    agent_id: str | None = None
    retryable: bool


class Job(BaseModel):
    id: str
    library: str
    status: Status
    created_at: str
    updated_at: str
    runner: Literal["local", "remote"]
    agents: list[Agent]
    result: Result | None = None
    approval: Approval
    error: Error | None = None


class Event(BaseModel):
    seq: int
    at: str
    job_id: str
    agent_id: str | None = None
    type: EventType
    data: dict[str, Any]


class CreateJobRequest(BaseModel):
    library: str
    modules: list[str] | None = None


class ApproveRequest(BaseModel):
    approved_by: str


# Proposed contract addition (not yet in contracts/README.md): GET /jobs/{id}/cost
class ModelCost(BaseModel):
    model: str
    input_tokens: int
    cached_tokens: int
    output_tokens: int
    thought_tokens: int
    usd: float


class JobCost(BaseModel):
    job_id: str
    models: list[ModelCost]
    total_usd: float
