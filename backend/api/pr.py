"""Approve -> PR for a real-mode job, through the managed-agent release step.

The live pipeline writes runs/ai/live-<stamp>-<job_id>/ (round_N/cjson.rs,
round_N/verify.json, result.json). open_pr_remote (backend/harness/remote/open_pr.py)
expects a run_remote job directory, so we write a small adapter directory
<out_dir>/release/ with a result.json in that shape, then hand it over.

No GITHUB_TOKEN (or THESEUS_PR_DRY_RUN=1): nothing leaves the laptop. We build the
exact payload (crate + EVIDENCE.md) locally and return status "dry_run" with the
branch and file list. We never return a made-up PR URL.
"""

from __future__ import annotations

import json
import os
import shutil
from pathlib import Path
from typing import Any

from . import cost, harness


class PRStepError(RuntimeError):
    pass


def job_dir(job_id: str) -> Path:
    """The live run directory for job_id (runs/ai/live-*-<job_id>)."""
    hits = sorted(harness.RUNS_AI.glob(f"live-*-{job_id}"))
    if not hits:
        raise PRStepError(f"no run directory for {job_id} under {harness.RUNS_AI}")
    return hits[-1]


def best_round(out_dir: Path) -> tuple[Path, dict]:
    """The round the pipeline kept: highest match rate, latest on ties (same rule as RealPipeline)."""
    best: tuple[float, int, Path, dict] | None = None
    for vf in out_dir.glob("round_*/verify.json"):
        n = int(vf.parent.name.split("_")[1])
        report = json.loads(vf.read_text())
        rate = float(report.get("match_rate") or 0.0)
        if best is None or (rate, n) >= (best[0], best[1]):
            best = (rate, n, vf.parent, report)
    if best is None:
        raise PRStepError(f"no verified round in {out_dir}")
    return best[2], best[3]


def _read(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return {}


def write_release_dir(job_id: str, model: str | None) -> Path:
    """Write <out_dir>/release/result.json in the shape open_pr_remote reads."""
    out_dir = job_dir(job_id)
    rdir, report = best_round(out_dir)
    saved = json.loads((out_dir / "result.json").read_text()) if (out_dir / "result.json").exists() else {}
    holdout = saved.get("holdout") or {}
    usage = {"total_input_tokens": 0, "total_output_tokens": 0, "total_thought_tokens": 0, "total_cached_tokens": 0}
    spent = cost.job_cost(job_id)
    for m in spent["models"]:
        usage["total_input_tokens"] += m["input_tokens"]
        usage["total_output_tokens"] += m["output_tokens"]
        usage["total_thought_tokens"] += m["thought_tokens"]
        usage["total_cached_tokens"] += m["cached_tokens"]
    rel = out_dir / "release"
    rel.mkdir(exist_ok=True)
    # Per-round verify.sh reports, in the layout open_pr reads for the "fixed per round" table.
    work = rel / "sandbox_work"
    work.mkdir(exist_ok=True)
    for vf in out_dir.glob("round_*/verify.json"):
        shutil.copyfile(vf, work / f"verify_{vf.parent.name.split('_')[1]}.json")
    inputs_dir = next((t.get("inputs_dir") for t in saved.get("timings") or [] if t.get("stage") == "inputs"), None)
    result = {
        "model": model,
        "rust_source": str((rdir / "cjson.rs").resolve()),
        "local_match_rate": report.get("match_rate"),
        "local_verify": {k: report.get(k) for k in ("match_rate", "behaviors_tested", "counts")},
        "holdout_verify": {**holdout, "seed": 1001} if holdout else {},
        "local_build": _read(rdir / "build.json"),
        # Live jobs run on the host, not in a sandbox: there is no sandbox claim to compare.
        "sandbox_match_rate": None,
        "match_rates_agree": "n/a (verified on the host; no sandbox claim)",
        "claimed_rounds": [],
        "usage": usage,
        "cost_usd": spent["total_usd"],
        "seconds": None,
        "interaction_id": None,
        "environment_id": None,
        "verify_label": f"Host verify, live job {job_id} ({Path(inputs_dir).name if inputs_dir else 'generated inputs'})",
        "rounds_label": "verify.sh reports from the live job",
        "source_job": job_id,
        "source_round": rdir.name,
    }
    (rel / "result.json").write_text(json.dumps(result, indent=2))
    (rel / "spec.json").write_text(json.dumps({"seed": 1, "source": f"live job {job_id}"}))
    return rel


def open_pr_for_job(job_id: str, approved_by: str, model: str | None) -> dict[str, Any]:
    """Run the release step. Returns {"status", "pr_url", "branch", "files", "detail"}."""
    from harness.remote import open_pr as op

    rel = write_release_dir(job_id, model)
    token = op.env_value("GITHUB_TOKEN")
    force_dry = os.environ.get("THESEUS_PR_DRY_RUN", "").strip() not in ("", "0", "false")
    if not token or force_dry:
        result = json.loads((rel / "result.json").read_text())
        if result["local_match_rate"] != 1.0:
            raise PRStepError(f"best build matched {result['local_match_rate']}, not 1.0; refusing to prepare a PR")
        branch = f"theseus/{rel.parent.name}-release"
        files = op.build_payload(rel, result, approved_by, branch)
        out = rel / "pr-dry-run-local"
        for name, data in files.items():
            (out / "crate" / name).parent.mkdir(parents=True, exist_ok=True)
            (out / "crate" / name).write_bytes(data)
        why = "THESEUS_PR_DRY_RUN is set" if force_dry else "GITHUB_TOKEN is not set"
        return {
            "status": "dry_run",
            "pr_url": None,
            "branch": branch,
            "files": [f"migrated/cjson/{n}" for n in sorted(files)],
            "detail": f"Dry run ({why}): no PR opened. Would push branch {branch} with "
                      f"{len(files)} files to {op.PUBLIC_REPO}; payload at {out}.",
        }
    try:
        url = op.open_pr_remote(rel, approved_by, model="gemini-4-argon")
    except op.PRError as exc:
        raise PRStepError(str(exc)) from None
    return {"status": "opened", "pr_url": url, "branch": f"theseus/{rel.parent.name}-release",
            "files": None, "detail": f"Managed agent pushed the verified crate and opened {url}."}
