"""Pipeline seam: mock replay of the contract fixtures, or the real Argon pipeline.

THESEUS_MODE=mock (default) replays contracts/fixtures/job-events.json.
THESEUS_MODE=real uses RealPipeline: Argon translate/repair, build_rust.sh, verify.sh.

Every pipeline runs in a background thread. Requests never wait on pipeline work;
the UI polls GET /jobs/{id} and GET /jobs/{id}/events.
"""

import json
import os
import re
import threading
import time
from datetime import datetime
from pathlib import Path
from dataclasses import dataclass
from typing import Any, Callable, Protocol

from . import cost, harness, try_input
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


AGENT_ID = "agent_cjson"
AGENT_MODULE = "cjson parse+print"
MAX_EVENT_DIVERGENCES = 10


_NUMBERISH = re.compile(r"^(-?[0-9]|-?inf|-?nan|null)", re.IGNORECASE)


def guess_function(pattern: str, d: dict | None = None) -> str:
    """Best-guess C function for a divergence pattern (from ai.translate.divergence_pattern).

    verify.sh compares bytes and does not know which function split; this mapping
    only names where to look, and the event says so in function_source.
    """
    p = pattern
    if p.startswith("OK/OK") and d:
        c, r = d.get("c_output") or "", d.get("rust_output") or ""
        i = next((j for j, (a, b) in enumerate(zip(c, r)) if a != b), min(len(c), len(r)))
        if _NUMBERISH.match(c[i:]) or _NUMBERISH.match(r[i:]):
            return "print_number"
    if p.startswith("OK/OK"):
        if "in number" in p:
            return "print_number"
        if "in string" in p:
            return "print_string_ptr"
        return "print_value"
    for key, fn in (("num", "parse_number"), ("quote", "parse_string"), ("backslash", "parse_string"),
                    ("[", "parse_array"), ("{", "parse_object")):
        if p.endswith(f" {key}"):
            return fn
    return "parse_value"


@dataclass
class _Prev:
    """Stand-in for ai.translate.TranslateResult when starting from a saved candidate."""

    rust_source: str
    interaction_id: str | None
    usage: dict
    seconds: float
    model: str | None


class RealPipeline:
    """Live pipeline: Argon translate/repair, build_rust.sh, verify.sh, in a background thread.

    Request knobs (CreateJobRequest): model, thinking, rounds (model-call budget),
    inputs (generated input count, cached under runs/ai/), final_full (verify the
    best build on 2000 inputs at the end), start_from (fresh, a saved run such as
    argon-1 / flash-2, or a path to a cjson.rs), inputs_dir, cve_check.

    The model only ever sees what the harness reported. Every rate in an event
    comes straight from verify.sh.

    The *_step methods are the seams tests replace with fakes (no network, no cargo).
    """

    runner = "local"

    def __init__(self, store: JobStore):
        self.store = store

    def initial_agents(self, request: CreateJobRequest) -> list[Agent]:
        return [Agent(id=AGENT_ID, module=AGENT_MODULE, status="queued", iteration=0, match_rate=0.0)]

    # ---- seams (real implementations)

    def translate_step(self, c_source: str, model: str, thinking: str) -> Any:
        from ai import prompts
        from ai import translate as t

        return t._run(model, prompts.translate_input(c_source), thinking, previous_id=None)

    def repair_step(self, prev: Any, feedback: dict, model: str, thinking: str, c_source: str) -> Any:
        """Continue prev's interaction with verifier feedback. If that interaction cannot be
        continued (expired, other model), resend the C source plus prev's Rust once."""
        from ai import prompts
        from ai import translate as t

        text = prompts.repair_input(prompts.format_feedback(t.trim_feedback(feedback)))
        if prev.interaction_id:
            try:
                return t._run(model, text, thinking, previous_id=prev.interaction_id)
            except t.TranslateError as exc:
                print(f"[pipeline] continuing {prev.interaction_id} failed ({exc}); resending context", flush=True)
        fresh = (
            prompts.translate_input(c_source)
            + "\n\nYour previous cjson.rs was:\n```rust\n" + prev.rust_source.rstrip() + "\n```\n\n" + text
        )
        return t._run(model, fresh, thinking, previous_id=None)

    def build_step(self, rust_path: Path, build_id: str) -> dict:
        return harness.build_rust(rust_path, build_id)

    def verify_step(self, binary: Path, inputs_dir: Path) -> dict:
        return harness.verify(binary, inputs_dir)

    def inputs_step(self, count: int) -> Path:
        return harness.ensure_inputs(count)

    def cve_step(self, binary: Path) -> dict | None:
        """Run each known CVE input on old C (v1.7.17), current C and this Rust binary."""
        if not harness.OLD_CVE_INPUTS.is_dir():
            return None
        for path in sorted(harness.OLD_CVE_INPUTS.iterdir()):
            if path.is_file() and path.stem in harness.KNOWN_CVES:
                return try_input.run_try(path.read_bytes(), compare_old=True, rust=(binary, "this run's Rust"))
        return None

    def holdout_step(self, count: int, seen_dir: Path) -> tuple[Path, int]:
        """Inputs from seed 1001 minus any whose bytes the loop already used (ai.loop.holdout_inputs)."""
        from ai.loop import holdout_inputs

        return holdout_inputs(harness.ROOT, count, 1001, seen_dir)

    def c_source_step(self) -> str:
        return harness.load_c_source()

    # ---- the loop

    def run(self, job_id: str, request: CreateJobRequest, emit: Emit) -> None:
        if request.library != "cjson":
            raise ValueError(f"real mode supports library 'cjson' only, got {request.library!r}")
        started = time.monotonic()
        timings: list[dict[str, Any]] = []

        def timed(stage: str, t0: float, **extra: Any) -> float:
            secs = round(time.monotonic() - t0, 1)
            timings.append({"stage": stage, "seconds": secs, **extra})
            return secs

        model, thinking = request.model, request.thinking
        emit("job_status", None, {"status": "running", "plan": [AGENT_MODULE]})

        t0 = time.monotonic()
        if request.inputs_dir:
            inputs_dir = Path(request.inputs_dir).expanduser().resolve()
            if not inputs_dir.is_dir():
                raise ValueError(f"inputs_dir {inputs_dir} is not a directory")
        else:
            inputs_dir = self.inputs_step(request.inputs)
        timed("inputs", t0, inputs_dir=str(inputs_dir))

        c_source = self.c_source_step()
        calls_left = request.rounds
        iteration = 1
        stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
        out_dir = harness.RUNS_AI / f"live-{stamp}-{job_id}"
        out_dir.mkdir(parents=True, exist_ok=True)

        # ---- round 1 source: a fresh translation or a saved candidate
        if request.start_from == "fresh":
            emit("translate", AGENT_ID, {
                "module": AGENT_MODULE, "iteration": iteration, "phase": "started", "model": model,
                "thinking": thinking, "start_from": "fresh",
                "summary": f"{model} is translating cJSON parse + print from C (thinking {thinking}).",
                "functions": ["parse_value", "print_value"],
            })
            t0 = time.monotonic()
            try:
                prev = self.translate_step(c_source, model, thinking)
            except Exception as exc:  # noqa: BLE001
                self._fail(emit, "translate_failed", f"{model} translate failed: {exc}", retryable=True)
                return
            calls_left -= 1
            cost.record_usage(job_id, model, prev.usage)
            secs = timed("translate", t0, model=model)
            emit("translate", AGENT_ID, {
                "module": AGENT_MODULE, "iteration": iteration, "phase": "done", "model": model,
                "seconds": secs, "interaction_id": prev.interaction_id,
                "rust_lines": prev.rust_source.count("\n"),
                "summary": f"{model} returned {prev.rust_source.count(chr(10))} lines of safe Rust in {secs}s.",
            })
        else:
            cand = harness.load_candidate(request.start_from)
            prev = _Prev(cand.rust_path.read_text(encoding="utf-8"), cand.interaction_id, {}, 0.0, cand.model)
            prior = ""
            if cand.prior.get("match_rate") is not None:
                prior = (f" When it was produced it scored {cand.prior['match_rate']:.0%} on "
                         f"{cand.prior.get('behaviors_tested')} inputs; it is re-verified now from scratch.")
            emit("translate", AGENT_ID, {
                "module": AGENT_MODULE, "iteration": iteration, "phase": "loaded",
                "start_from": cand.name, "source_label": cand.label, "source_model": cand.model,
                "source_path": str(cand.rust_path), "interaction_id": cand.interaction_id,
                "rust_lines": prev.rust_source.count("\n"), "model": model,
                "summary": f"Starting from {cand.label} ({cand.model or 'unknown model'}, "
                           f"{prev.rust_source.count(chr(10))} lines, saved earlier today, no model call).{prior}",
            })

        # ---- build -> verify -> repair
        records: dict[str, dict[str, Any]] = {}  # input file -> divergence record (for result + resolved)
        crashes: list[dict[str, Any]] = []
        baseline: float | None = None
        best: tuple[float, Path, int] | None = None  # (rate, binary, iteration)
        behaviors = 0
        while True:
            rdir = out_dir / f"round_{iteration}"
            rdir.mkdir(exist_ok=True)
            rust_path = rdir / "cjson.rs"
            rust_path.write_text(prev.rust_source, encoding="utf-8")

            emit("job_status", None, {"status": "verifying"})
            t0 = time.monotonic()
            build = self.build_step(rust_path, f"live-{job_id}-r{iteration}")
            build_secs = timed("build", t0, iteration=iteration, ok=bool(build.get("ok")))
            (rdir / "build.json").write_text(json.dumps({k: build.get(k) for k in ("ok", "binary")}))
            feedback: dict[str, Any]
            if not build.get("ok"):
                errors = build.get("errors") or "build failed"
                emit("verify", AGENT_ID, {
                    "iteration": iteration, "compile_ok": False, "build_seconds": build_secs,
                    "errors": errors[:2000],
                    "summary": "cargo build failed; the compiler errors go back to the model.",
                })
                feedback = {"compile_errors": errors, "divergences": [], "crashes": []}
            else:
                binary = Path(build["binary"])
                t0 = time.monotonic()
                report = self.verify_step(binary, inputs_dir)
                verify_secs = timed("verify", t0, iteration=iteration, inputs=report.get("behaviors_tested"))
                rate = float(report.get("match_rate") or 0.0)
                counts = report.get("counts") or {}
                behaviors = int(report.get("behaviors_tested") or 0)
                if baseline is None:
                    baseline = rate
                if best is None or rate >= best[0]:
                    best = (rate, binary, iteration)
                (rdir / "verify.json").write_text(json.dumps(report, indent=2, ensure_ascii=False))
                n_crash = counts.get("rust_crash", 0) + counts.get("both_crash", 0)
                emit("verify", AGENT_ID, {
                    "iteration": iteration, "compile_ok": True, "match_rate": rate,
                    "behaviors_tested": behaviors, "divergences": counts.get("divergence", 0),
                    "crashes": n_crash, "timeouts": counts.get("timeout", 0),
                    "safety_improvements": counts.get("safety_improvement", 0),
                    "build_seconds": build_secs, "seconds": verify_secs,
                    "summary": f"verify.sh: {counts.get('match', 0)}/{behaviors} inputs byte-identical "
                               f"({rate:.1%}) in {verify_secs}s.",
                })
                self._track(emit, report, records, crashes)
                feedback = dict(report, compile_errors=None)
                if rate >= 1.0:
                    break

            if calls_left <= 0:
                break
            # ---- repair
            emit("job_status", None, {"status": "running"})
            sent = self._feedback_preview(feedback)
            functions = sorted({guess_function(d["pattern"], d) for d in sent}) or (
                ["(compile errors)"] if feedback.get("compile_errors") else [])
            iteration += 1
            emit("repair", AGENT_ID, {
                "iteration": iteration, "phase": "started", "model": model, "thinking": thinking,
                "function": functions[0] if functions else None, "functions": functions,
                "patterns": [d["pattern"] for d in sent],
                "summary": (f"Sending {len(sent)} deduped divergence pattern(s) to {model}."
                            if sent else f"Sending compiler/crash feedback to {model}."),
            })
            t0 = time.monotonic()
            try:
                res = self.repair_step(prev, feedback, model, thinking, c_source)
            except Exception as exc:  # noqa: BLE001
                calls_left = 0
                emit("repair", AGENT_ID, {"iteration": iteration, "phase": "failed",
                                          "summary": f"repair call failed: {exc}"})
                iteration -= 1
                break
            calls_left -= 1
            cost.record_usage(job_id, model, res.usage)
            secs = timed("repair", t0, iteration=iteration, model=model)
            emit("repair", AGENT_ID, {
                "iteration": iteration, "phase": "done", "seconds": secs, "interaction_id": res.interaction_id,
                "function": functions[0] if functions else None,
                "summary": f"{model} returned a corrected cjson.rs in {secs}s.",
            })
            prev = res

        if best is None:
            self._fail(emit, "no_working_build",
                       f"no candidate compiled within {request.rounds} model call(s)", retryable=True)
            return
        final_rate, best_bin, best_iter = best

        if request.final_full and not request.inputs_dir:
            emit("job_status", None, {"status": "verifying"})
            t0 = time.monotonic()
            full_dir = self.inputs_step(2000)
            report = self.verify_step(best_bin, full_dir)
            secs = timed("final_verify", t0, inputs=report.get("behaviors_tested"))
            final_rate = float(report.get("match_rate") or 0.0)
            behaviors = int(report.get("behaviors_tested") or 0)
            counts = report.get("counts") or {}
            emit("verify", AGENT_ID, {
                "iteration": best_iter, "final_full": True, "match_rate": final_rate,
                "behaviors_tested": behaviors, "divergences": counts.get("divergence", 0),
                "crashes": counts.get("rust_crash", 0) + counts.get("both_crash", 0), "seconds": secs,
                "summary": f"Final check on {behaviors} inputs: {final_rate:.1%} byte-identical.",
            })
            self._track(emit, report, records, crashes, emit_events=False)

        holdout: dict[str, Any] | None = None
        if request.holdout and not request.inputs_dir:
            emit("job_status", None, {"status": "verifying"})
            t0 = time.monotonic()
            hdir, overlap = self.holdout_step(request.inputs, inputs_dir)
            report = self.verify_step(best_bin, hdir)
            secs = timed("holdout_verify", t0, inputs=report.get("behaviors_tested"))
            counts = report.get("counts") or {}
            holdout = {"match_rate": report.get("match_rate"), "behaviors_tested": report.get("behaviors_tested"),
                       "overlap_removed": overlap, "counts": counts, "inputs": str(hdir)}
            emit("verify", AGENT_ID, {
                "iteration": best_iter, "holdout": True, "match_rate": float(report.get("match_rate") or 0.0),
                "behaviors_tested": report.get("behaviors_tested"), "divergences": counts.get("divergence", 0),
                "crashes": counts.get("rust_crash", 0) + counts.get("both_crash", 0), "seconds": secs,
                "summary": f"Held-out check: {counts.get('match', 0)}/{report.get('behaviors_tested')} inputs the "
                           f"model never saw are byte-identical ({float(report.get('match_rate') or 0):.1%}).",
            })
            self._track(emit, report, records, crashes)

        if request.cve_check:
            t0 = time.monotonic()
            cve = self.cve_step(best_bin)
            timed("cve_check", t0)
            if cve and cve.get("old_verdict") == "safety_improvement":
                old = cve["old_c"]
                crash = {
                    "input": cve["input"],
                    "sanitizer_report": (old.get("stderr_head") or old.get("sanitizer_summary") or old["exit"]),
                    "rust_behavior": f"clean error: {cve['rust']['stdout'].strip()}",
                }
                crashes.append(crash)
                emit("crash", None, {
                    "input": crash["input"], "side": "c", "cve": cve["cve"],
                    "sanitizer": f"{old.get('sanitizer_summary') or old['exit']} ({harness.OLD_C_VERSION}, {cve['cve']})",
                    "rust_behavior": crash["rust_behavior"], "c_version": harness.OLD_C_VERSION,
                    "current_c": cve["c"]["stdout"].strip(),
                })

        if final_rate >= 1.0:
            try_input.set_live_best(str(best_bin), f"live run {job_id} (iteration {best_iter})")
        result = {
            "baseline_match_rate": baseline if baseline is not None else 0.0,
            "final_match_rate": final_rate,
            "behaviors_tested": behaviors,
            "divergences": [
                {k: r[k] for k in ("input", "c_output", "rust_output", "function", "resolved")}
                for r in records.values()
            ],
            "crashes": crashes,
        }
        (out_dir / "result.json").write_text(json.dumps({"result": result, "timings": timings, "holdout": holdout}, indent=2,
                                                        ensure_ascii=False))
        agents = [a.model_dump() for a in self.store.get(job_id).agents]
        for a in agents:
            a.update(status="completed", match_rate=final_rate, iteration=iteration)
        self.store.apply_snapshot(job_id, {"agents": agents, "result": result})
        emit("job_status", None, {
            "status": "completed", "final_match_rate": final_rate, "behaviors_tested": behaviors,
            "result": result, "timings": timings, "seconds": round(time.monotonic() - started, 1),
            "best_binary": str(best_bin), "holdout": holdout,
        })
        emit("approval_required", None, {"action": "open_pr", "evidence": f"/jobs/{job_id}/result"})

    # ---- helpers

    @staticmethod
    def _fail(emit: Emit, code: str, message: str, retryable: bool) -> None:
        emit("job_status", None, {
            "status": "failed",
            "error": {"code": code, "message": message, "agent_id": AGENT_ID, "retryable": retryable},
        })

    @staticmethod
    def _feedback_preview(feedback: dict) -> list[dict]:
        """The deduped divergences repair will send (same trim the model sees)."""
        from ai.translate import trim_feedback

        return trim_feedback(feedback).get("divergences") or []

    def _track(self, emit: Emit, report: dict, records: dict, crashes: list, emit_events: bool = True) -> None:
        """Emit the top deduped divergences and crashes; mark earlier divergences resolved."""
        from ai.translate import divergence_pattern

        current = {d.get("file") for d in report.get("divergences") or []}
        for key, rec in records.items():
            if not rec["resolved"] and key not in current:
                rec["resolved"] = True
        if emit_events:
            for d in self._feedback_preview(dict(report, compile_errors=None))[:MAX_EVENT_DIVERGENCES]:
                key = d.get("file") or d.get("input")
                if key in records:
                    continue
                fn = guess_function(d["pattern"] if "pattern" in d else divergence_pattern(d), d)
                records[key] = {"input": d.get("input") or "", "c_output": d.get("c_output") or "",
                                "rust_output": d.get("rust_output") or "", "function": fn, "resolved": False}
                emit("divergence", AGENT_ID, {
                    "input": d.get("input"), "c_output": d.get("c_output"), "rust_output": d.get("rust_output"),
                    "function": fn, "function_source": "guessed from the divergence pattern",
                    "pattern": d.get("pattern"), "pattern_count": d.get("pattern_count"),
                })
        seen = {c["input"] for c in crashes}
        for c in report.get("crashes") or []:
            if c.get("input") in seen:
                continue
            seen.add(c.get("input"))
            crashes.append({"input": c.get("input") or "", "sanitizer_report": c.get("sanitizer_report") or "",
                            "rust_behavior": c.get("rust_behavior") or ""})
            if emit_events:
                emit("crash", AGENT_ID if c.get("side") != "c" else None, {
                    "input": c.get("input"), "side": "c" if c.get("side") == "c" else "rust",
                    "class": c.get("class"),
                    "sanitizer": (c.get("sanitizer_report") or "").splitlines()[0] if c.get("sanitizer_report") else "",
                    "rust_behavior": c.get("rust_behavior"),
                })

    def open_pr(self, job: Job, approved_by: str) -> dict[str, Any]:
        """Managed-agent release step (backend/api/pr.py). Without GITHUB_TOKEN this is an
        honest local dry run: status "dry_run", pr_url None, plus what would be pushed."""
        from . import pr

        model = None
        for e in self.store.events(job.id):
            if e.type in ("translate", "repair") and e.data.get("phase") in ("done", "loaded"):
                model = e.data.get("source_model") or e.data.get("model") or model
        return pr.open_pr_for_job(job.id, approved_by, model)


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
