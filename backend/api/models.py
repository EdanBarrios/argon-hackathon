"""Pydantic models mirroring contracts/fixtures/*.json. Field names must match the fixtures."""

from typing import Any, Literal

from pydantic import BaseModel, Field

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
    library: str = "cjson"
    modules: list[str] | None = None
    # Real-mode knobs (ignored by the mock replay).
    model: str = "gemini-4-argon"
    thinking: Literal["low", "medium", "high"] = "medium"
    rounds: int = Field(4, ge=1, le=10)  # model calls: translate (if fresh) + repairs
    inputs: int = Field(300, ge=1, le=2000)
    final_full: bool = False  # also verify the best build on 2000 inputs at the end
    start_from: str = "fresh"  # fresh | argon-1 | argon-2 | flash-2 | path to a cjson.rs
    inputs_dir: str | None = None  # override the generated input set with a directory
    holdout: bool = True  # verify the best build on unseen inputs (seed 1001, overlap removed)
    cve_check: bool = True  # run the CVE-2023-53154 input on old C vs the final Rust


class TryRequest(BaseModel):
    library: str = "cjson"
    input: str | None = None
    input_b64: str | None = None
    compare_old: bool = False


class ApproveRequest(BaseModel):
    approved_by: str


# GET /jobs/{id}/cost (contracts/README.md)
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
