"""Sandboxed translate-and-verify as a function, plus parallel runs.

    run_module_remote(library="cjson", model="gemini-4-argon", rounds=4, inputs=300) -> RemoteResult
    run_parallel([RemoteSpec(...), ...]) -> list[RemoteResult]      (threads, one sandbox each)
    resume_remote(interaction_id, job_dir, ...) -> RemoteResult      (re-attach after a client crash)

This wraps Brendan's backend/ai/sandbox_run.py (the team's sandbox pre-run) in a
function interface the API pipeline can call, and adds what it lacked: parallel
jobs, a per-job directory, the step transcript, progress events, and a pull of
the sandbox's own verify reports through the Environments files API.

Reused from backend/ai (not duplicated): prompts.SYSTEM_INSTRUCTION and
prompts.sandbox_input (the sandbox task: install clang and rustup, clone the
public repo, fetch cJSON, build the ASan driver, generate inputs, write
cjson.rs, scripts/build_rust.sh, verify.sh, repair), sandbox_run.ENVIRONMENT
(the apt-mirror allowlist proven at 11:22), sandbox_run.parse_claims,
sandbox_run.host_verify, translate.extract_rust, loop.ensure_c_driver and
the held-out rule of loop.holdout_inputs (seed + 1000 minus overlap).

What we add to the prompt: verify.sh is the only judge, and the agent must
paste the final verify.sh JSON summary verbatim between markers.

Trust but verify: the returned cjson.rs is rebuilt LOCALLY with
scripts/build_rust.sh and re-verified LOCALLY with verify.sh on the same seed
(local_match_rate, compared with the sandbox's claim) and on a held-out seed the
agent never saw (holdout_match_rate, the number to quote).

`rounds` follows the team convention (DECISIONS 11:05): total rounds, round 1 is
the first translation, the rest are repairs.
"""

from __future__ import annotations

import concurrent.futures
import dataclasses
import hashlib
import io
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import threading
import time
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path

from ai import prompts
from ai.loop import ensure_c_driver, run_json
from ai.sandbox_run import AGENT, ENVIRONMENT, host_verify, parse_claims
from ai.translate import TranslateError, extract_rust

from ._common import (REPO_ROOT, Event, PreflightError, cost_usd, emit, get_client, poll, run_preflight, step_types,
                      usage_dict, write_transcript)

SUPPORTED = {"cjson"}
DEFAULT_MAX_WAIT = 90 * 60
DEFAULT_SEED = 1  # DECISIONS 11:05: loops verify on seed 1; held-out is seed + 1000

BEGIN_VERIFY, END_VERIFY = "=== VERIFY_JSON_BEGIN ===", "=== VERIFY_JSON_END ==="

VERIFY_ADDENDUM = f"""
Addendum from the Theseus host (this changes the final reply format):

backend/harness/verify.sh is the ONLY judge of correctness. Never state a match rate,
a fix, or a pass from reading code or from your own reasoning; a number counts only
if verify.sh printed it. The ROUND lines must be copied from verify.sh output.

In the final reply, put this block between the ROUND lines and the ```rust block,
containing the verbatim output of this command for the last round that built
(<N> is that round's number), not a summary and not your opinion:
  jq -c '{{match_rate, behaviors_tested, counts, divergences: (.divergences[:5]), crashes: (.crashes[:5])}}' /work/verify_<N>.json
{BEGIN_VERIFY}
<that one line of JSON, verbatim>
{END_VERIFY}
"""

_local_lock = threading.Lock()  # one local C driver build at a time


@dataclass
class RemoteSpec:
    library: str = "cjson"
    model: str = "gemini-4-argon"
    rounds: int = 4
    inputs: int = 300
    seed: int = DEFAULT_SEED
    branch: str = "main"


@dataclass
class RemoteResult:
    library: str
    model: str
    status: str                               # interaction status, or "error"
    interaction_id: str | None
    environment_id: str | None
    seconds: float
    usage: dict
    cost_usd: float
    sandbox_match_rate: float | None          # the agent's claim: verify.sh JSON it pasted
    local_match_rate: float | None            # host rebuild + verify.sh, same seed as the sandbox
    holdout_match_rate: float | None          # host verify.sh on unseen inputs (quote this)
    match_rates_agree: bool | None            # sandbox claim == local re-verify
    rust_source: str | None                   # path to the returned cjson.rs
    transcript: str | None                    # path to the step-by-step transcript (jsonl)
    job_dir: str
    rounds_used: int | None = None
    claimed_rounds: list = field(default_factory=list)
    sandbox_verify: dict | None = None        # the verify.sh summary the agent pasted
    sandbox_verify_file: dict | None = None   # the verify JSON pulled from the sandbox filesystem
    local_verify: dict | None = None
    holdout_verify: dict | None = None
    local_build: dict | None = None
    notes: list[str] = field(default_factory=list)
    error: str | None = None

    def to_dict(self) -> dict:
        return dataclasses.asdict(self)


# ---------------------------------------------------------------- request


def build_request(spec: RemoteSpec) -> dict:
    vendor = REPO_ROOT / "backend/harness/vendor/cJSON"
    if not (vendor / "cJSON.c").exists():
        raise RuntimeError(f"missing {vendor}/cJSON.c: run backend/harness/fetch_cjson.sh")
    task = prompts.sandbox_input(prompts.load_cjson_source(vendor), branch=spec.branch, seed=spec.seed,
                                 count=spec.inputs, rounds=spec.rounds)
    return {
        "agent": AGENT,
        "agent_config": {"type": "antigravity", "model": spec.model},
        "system_instruction": prompts.SYSTEM_INSTRUCTION,
        "input": task + "\n" + VERIFY_ADDENDUM,
        "environment": ENVIRONMENT,
        "background": True,
    }


def parse_verify_block(text: str | None) -> dict | None:
    t = text or ""
    i = t.rfind(BEGIN_VERIFY)
    j = t.find(END_VERIFY, i + 1) if i >= 0 else -1
    if i < 0 or j < 0:
        return None
    m = re.search(r"\{.*\}", t[i + len(BEGIN_VERIFY):j], re.DOTALL)
    if not m:
        return None
    try:
        return json.loads(m.group(0))
    except json.JSONDecodeError:
        return None


# ---------------------------------------------------------------- sandbox files


def _pull_sandbox_reports(client, env_id: str, dest: Path, notes: list[str]) -> list[Path]:
    """Download /work/verify_*.json and /work/cjson.rs from the sandbox. Best effort."""
    got: list[Path] = []
    for path in ("/work", "work", "../work"):
        try:
            data = client.environments.files.download(environment=env_id, path=path)
        except Exception as exc:
            notes.append(f"files.download({path!r}): {type(exc).__name__}: {str(exc)[:160]}")
            continue
        try:
            with tarfile.open(fileobj=io.BytesIO(data)) as tar:
                for m in tar.getmembers():
                    name = Path(m.name).name
                    if m.isfile() and (name == "cjson.rs" or re.fullmatch(r"verify_\d+\.json", name)):
                        fh = tar.extractfile(m)
                        if fh and m.name.strip("./").count("/") <= 1:  # /work/<name>, not the repo clone
                            dest.mkdir(parents=True, exist_ok=True)
                            (dest / name).write_bytes(fh.read())
                            got.append(dest / name)
        except tarfile.TarError:
            notes.append(f"files.download({path!r}) returned non-tar data ({len(data)} bytes)")
            continue
        notes.append(f"pulled {len(got)} files from sandbox {path}")
        return got
    return got


# ---------------------------------------------------------------- run


def _job_dir(spec: RemoteSpec, job_dir: str | Path | None) -> Path:
    p = Path(job_dir) if job_dir else (
        REPO_ROOT / "runs/remote" / datetime.now().strftime("%Y%m%d-%H%M%S") / f"{spec.library}-{spec.model}")
    p.mkdir(parents=True, exist_ok=True)
    return p


def _error_result(spec: RemoteSpec, jd: Path, seconds: float, msg: str) -> RemoteResult:
    r = RemoteResult(spec.library, spec.model, "error", None, None, round(seconds, 1), {}, 0.0,
                     None, None, None, None, None, None, str(jd), error=msg)
    _save(r)
    return r


def run_module_remote(
    library: str = "cjson",
    model: str = "gemini-4-argon",
    rounds: int = 4,
    inputs: int = 300,
    *,
    seed: int = DEFAULT_SEED,
    branch: str = "main",
    job_dir: str | Path | None = None,
    on_event: Event | None = None,
    max_wait: float = DEFAULT_MAX_WAIT,
    skip_preflight: bool = False,
) -> RemoteResult:
    """One module translate-and-verify in a fresh managed-agent sandbox, then local re-verify.

    Blocking (tens of minutes). Never raises for agent, build or verify failures:
    check result.status, result.error, result.local_match_rate. on_event(msg)
    receives short progress lines for a UI event log.

    Runs the verifier preflight gate (scripts/preflight.sh) before the model
    call; if it fails, returns status "error" with a PREFLIGHT message and makes
    no model call. skip_preflight=True only when the caller already ran it.
    """
    spec = RemoteSpec(library, model, rounds, inputs, seed, branch)
    if library not in SUPPORTED:
        raise ValueError(f"library {library!r} not supported yet (supported: {sorted(SUPPORTED)})")
    jd = _job_dir(spec, job_dir)
    tag = f"[{library}/{model}] "
    started = time.monotonic()
    if not skip_preflight:
        try:
            pf = run_preflight(library, on_event=on_event)
            (jd / "preflight.json").write_text(json.dumps(pf, indent=2))
        except PreflightError as exc:
            return _error_result(spec, jd, time.monotonic() - started, f"PREFLIGHT: {exc}")
    try:
        kwargs = build_request(spec)
    except Exception as exc:
        return _error_result(spec, jd, 0, f"{type(exc).__name__}: {exc}")
    (jd / "prompt.md").write_text(kwargs["system_instruction"] + "\n\n-----\n\n" + kwargs["input"])
    (jd / "spec.json").write_text(json.dumps(dataclasses.asdict(spec), indent=2))

    client = get_client()
    try:
        interaction = client.interactions.create(**kwargs)
    except Exception as exc:
        return _error_result(spec, jd, time.monotonic() - started, f"create failed: {type(exc).__name__}: {exc}")
    emit(on_event, f"{tag}sandbox interaction {interaction.id} {interaction.status}")
    (jd / "interaction.json").write_text(json.dumps(
        {"interaction_id": interaction.id, "created_at": datetime.now().isoformat()}, indent=2))
    interaction = poll(client, interaction, max_wait=max_wait, started=started, on_event=on_event, tag=tag)
    return _finish(client, interaction, spec, jd, time.monotonic() - started, on_event)


def resume_remote(interaction_id: str, job_dir: str | Path, library: str = "cjson", model: str = "gemini-4-argon",
                  rounds: int = 4, inputs: int = 300, *, seed: int = DEFAULT_SEED, on_event: Event | None = None,
                  max_wait: float = DEFAULT_MAX_WAIT) -> RemoteResult:
    """Re-attach to a sandbox run by interaction id (for example after the client process died)."""
    spec = RemoteSpec(library, model, rounds, inputs, seed)
    jd = _job_dir(spec, job_dir)
    client = get_client()
    started = time.monotonic()
    interaction = poll(client, client.interactions.get(interaction_id), max_wait=max_wait, started=started,
                       on_event=on_event, tag=f"[{library}/{model}] ")
    seconds = time.monotonic() - started
    try:
        seconds = (interaction.updated - interaction.created).total_seconds()
    except Exception:
        pass
    return _finish(client, interaction, spec, jd, seconds, on_event)


def _finish(client, interaction, spec: RemoteSpec, jd: Path, seconds: float, on_event: Event | None) -> RemoteResult:
    tag = f"[{spec.library}/{spec.model}] "
    notes: list[str] = []
    env_id = getattr(interaction, "environment_id", None)
    usage = usage_dict(interaction)
    text = getattr(interaction, "output_text", None) or ""
    transcript = jd / "transcript.jsonl"
    write_transcript(interaction, transcript)
    (jd / "agent_output.md").write_text(text)
    notes.append(f"steps: {step_types(interaction)}")
    if getattr(interaction, "errors", None):
        notes.append(f"interaction errors: {interaction.errors}")

    res = RemoteResult(spec.library, spec.model, str(interaction.status), interaction.id, env_id,
                       round(seconds, 1), usage, cost_usd(usage), None, None, None, None, None,
                       str(transcript), str(jd), notes=notes)
    res.claimed_rounds = parse_claims(text)
    res.rounds_used = res.claimed_rounds[-1]["round"] if res.claimed_rounds else None
    res.sandbox_verify = parse_verify_block(text)
    if res.sandbox_verify and isinstance(res.sandbox_verify.get("match_rate"), (int, float)):
        res.sandbox_match_rate = float(res.sandbox_verify["match_rate"])
    else:
        claimed = next((c["match_rate"] for c in reversed(res.claimed_rounds) if c["build_ok"]), None)
        res.sandbox_match_rate = claimed
        notes.append("no VERIFY_JSON block; sandbox_match_rate taken from the agent's ROUND lines")

    # The sandbox's own verify reports, straight from its filesystem.
    pulled = _pull_sandbox_reports(client, env_id, jd / "sandbox_work", notes) if env_id else []
    reports = sorted((p for p in pulled if p.name.startswith("verify_")),
                     key=lambda p: int(re.search(r"\d+", p.name).group(0)))
    if reports:
        fv = json.loads(reports[-1].read_text())
        res.sandbox_verify_file = {"file": reports[-1].name, "match_rate": fv.get("match_rate"),
                                   "behaviors_tested": fv.get("behaviors_tested"), "counts": fv.get("counts")}
        if res.sandbox_match_rate is not None and fv.get("match_rate") != res.sandbox_match_rate:
            notes.append(f"WARNING: agent-reported match_rate {res.sandbox_match_rate} != sandbox "
                         f"{reports[-1].name} {fv.get('match_rate')}")

    try:
        rust = extract_rust(text, notes)
    except TranslateError as exc:
        file_rs = next((p for p in pulled if p.name == "cjson.rs"), None)
        if file_rs is None:
            res.error = f"no cjson.rs returned ({exc}; status {interaction.status})"
            emit(on_event, f"{tag}{res.error}")
            _save(res)
            return res
        notes.append(f"{exc}; using /work/cjson.rs pulled from the sandbox")
        rust = file_rs.read_text(encoding="utf-8", errors="replace")
    file_rs = next((p for p in pulled if p.name == "cjson.rs"), None)
    if file_rs is not None:
        same = hashlib.sha256(rust.strip().encode()).digest() == hashlib.sha256(
            file_rs.read_text(encoding="utf-8", errors="replace").strip().encode()).digest()
        notes.append("returned cjson.rs is identical to the sandbox's /work/cjson.rs" if same else
                     "note: returned cjson.rs differs from the sandbox's /work/cjson.rs (agent may return the last version that built)")
    rs_path = jd / "cjson.rs"
    rs_path.write_text(rust)
    res.rust_source = str(rs_path)

    emit(on_event, f"{tag}trust but verify: local rebuild and verify.sh")
    _trust_but_verify(res, spec, jd)
    emit(on_event, f"{tag}done in {res.seconds:.0f}s: sandbox {res.sandbox_match_rate}, local "
                   f"{res.local_match_rate}, held-out {res.holdout_match_rate}")
    _save(res)
    return res


def _gen(out: Path, count: int, seed: int) -> None:
    subprocess.run([sys.executable, "-I", str(REPO_ROOT / "backend/harness/gen_inputs.py"), "--seed", str(seed),
                    "--count", str(count), "--out", str(out), "--cjson", str(REPO_ROOT / "backend/harness/vendor/cJSON")],
                   check=True, capture_output=True)


def _job_inputs(jd: Path, spec: RemoteSpec) -> tuple[Path, Path, int]:
    """Same-seed inputs and held-out inputs, both private to this job directory.

    Same rule as ai.loop.holdout_inputs (seed + 1000, minus any input whose bytes
    the loop saw), but written under the job dir: the shared runs/ai/holdout-* path
    is rebuilt with rmtree by other runs, which raced with ours (FRICTION-LOG 12:01).
    """
    same, held = jd / "inputs", jd / "holdout_inputs"
    if not (same.is_dir() and any(same.iterdir())):
        _gen(same, spec.inputs, spec.seed)
    if held.is_dir():
        shutil.rmtree(held)
    raw = jd / "holdout_raw"
    if raw.is_dir():
        shutil.rmtree(raw)
    _gen(raw, spec.inputs, spec.seed + 1000)
    seen = {hashlib.sha256(p.read_bytes()).digest() for p in same.iterdir() if p.is_file()}
    held.mkdir()
    removed = 0
    for p in sorted(raw.iterdir()):
        data = p.read_bytes()
        if hashlib.sha256(data).digest() in seen:
            removed += 1
        else:
            (held / p.name).write_bytes(data)
    shutil.rmtree(raw)
    return same, held, removed


def _local_verify(res: RemoteResult, spec: RemoteSpec, jd: Path) -> None:
    build_id = re.sub(r"[^A-Za-z0-9._-]", "_", f"remote-{jd.parent.name}-{jd.name}")
    build, raw = run_json([str(REPO_ROOT / "scripts/build_rust.sh"), res.rust_source, build_id], timeout=600)
    res.local_build = build or {"ok": False, "errors": raw[-2000:]}
    (jd / "local_build.json").write_text(json.dumps(res.local_build, indent=2))
    if not res.local_build.get("ok"):
        res.error = "local build of the returned cjson.rs failed"
        return
    with _local_lock:
        c_driver = ensure_c_driver(REPO_ROOT)
    same, held_copy, overlap = _job_inputs(jd, spec)
    verify_sh = REPO_ROOT / "backend/harness/verify.sh"
    jobs = os.cpu_count() or 4
    for key, inputs in (("local", same), ("holdout", held_copy)):
        report = host_verify(verify_sh, c_driver, res.local_build["binary"], inputs, jobs)
        (jd / f"verify_{key}.json").write_text(json.dumps(report, indent=2))
        summary = {"inputs": str(inputs), "match_rate": report.get("match_rate"),
                   "behaviors_tested": report.get("behaviors_tested"), "counts": report.get("counts"),
                   **({"error": report["error"]} if "error" in report else {})}
        if key == "local":
            res.local_verify, res.local_match_rate = summary, report.get("match_rate")
        else:
            summary.update(seed=spec.seed + 1000, overlap_removed=overlap)
            res.holdout_verify, res.holdout_match_rate = summary, report.get("match_rate")


def _trust_but_verify(res: RemoteResult, spec: RemoteSpec, jd: Path) -> None:
    try:
        _local_verify(res, spec, jd)
    except Exception as exc:
        res.error = f"local re-verify failed: {type(exc).__name__}: {exc}"
    if res.sandbox_match_rate is not None and res.local_match_rate is not None:
        res.match_rates_agree = abs(res.sandbox_match_rate - res.local_match_rate) < 1e-9
        if not res.match_rates_agree:
            res.notes.append(f"MISMATCH: sandbox claimed match_rate {res.sandbox_match_rate}, local re-verify got "
                             f"{res.local_match_rate} on the same seed. Trust the local number.")


def reverify_job(job_dir: str | Path) -> RemoteResult:
    """Re-run only the local trust-but-verify step for a finished job directory."""
    jd = Path(job_dir).resolve()
    data = json.loads((jd / "result.json").read_text())
    res = RemoteResult(**{k: v for k, v in data.items() if k in RemoteResult.__dataclass_fields__})
    spec = RemoteSpec(**{k: v for k, v in json.loads((jd / "spec.json").read_text()).items()
                         if k in RemoteSpec.__dataclass_fields__})
    res.error, res.local_match_rate, res.holdout_match_rate, res.match_rates_agree = None, None, None, None
    res.notes = [n for n in res.notes if not n.startswith("MISMATCH")] + ["local re-verify re-run with reverify_job"]
    _trust_but_verify(res, spec, jd)
    _save(res)
    return res


def _save(res: RemoteResult) -> None:
    Path(res.job_dir, "result.json").write_text(json.dumps(res.to_dict(), indent=2, default=str))


def run_parallel(specs: list[RemoteSpec | dict], job_root: str | Path | None = None,
                 on_event: Event | None = None, max_wait: float = DEFAULT_MAX_WAIT,
                 skip_preflight: bool = False) -> list[RemoteResult]:
    """Several sandbox jobs at once (one thread and one sandbox each). Results keep the order of specs.

    Runs the verifier preflight gate once per library before any sandbox starts
    and raises PreflightError if it fails (no model call is made).
    """
    norm = [s if isinstance(s, RemoteSpec) else RemoteSpec(**s) for s in specs]
    root = Path(job_root) if job_root else REPO_ROOT / "runs/remote" / datetime.now().strftime("%Y%m%d-%H%M%S")
    root.mkdir(parents=True, exist_ok=True)
    if not skip_preflight:
        for lib in dict.fromkeys(s.library for s in norm):
            pf = run_preflight(lib, on_event=on_event)
            (root / f"preflight-{lib}.json").write_text(json.dumps(pf, indent=2))
    names = [f"{s.library}-{s.model}" for s in norm]
    dirs = [root / (n if names.count(n) == 1 else f"{n}-{i}") for i, n in enumerate(names)]

    def one(i: int) -> RemoteResult:
        s = norm[i]
        try:
            return run_module_remote(s.library, s.model, s.rounds, s.inputs, seed=s.seed, branch=s.branch,
                                     job_dir=dirs[i], on_event=on_event, max_wait=max_wait,
                                     skip_preflight=True)
        except Exception as exc:
            dirs[i].mkdir(parents=True, exist_ok=True)
            return _error_result(s, dirs[i], 0, f"{type(exc).__name__}: {exc}")

    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, len(norm))) as pool:
        return list(pool.map(one, range(len(norm))))
