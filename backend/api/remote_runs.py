"""Managed-agent sandbox runs (runs/remote/<stamp>/<library>-<model>/result.json), read only.

The sandbox agent runs verify.sh itself, so its number is a claim. The host rebuilds
the returned cjson.rs and re-verifies it on the same seed (local) and on unseen
inputs (holdout). GET /remote-runs puts the three side by side so the UI can show
"sandbox claims X -> host re-verified X".
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from . import harness

RUNS_REMOTE = harness.ROOT / "runs" / "remote"


def _n(d: Any) -> int | None:
    return d.get("behaviors_tested") if isinstance(d, dict) else None


def summarize(path: Path) -> dict[str, Any]:
    r = json.loads(path.read_text())
    hold = r.get("holdout_verify") or {}
    return {
        "run_id": f"{path.parent.parent.name}/{path.parent.name}",
        "library": r.get("library"),
        "model": r.get("model"),
        "status": r.get("status"),
        "seconds": r.get("seconds"),
        "cost_usd": r.get("cost_usd"),
        "rounds_used": r.get("rounds_used"),
        "claimed_rounds": r.get("claimed_rounds") or [],
        "sandbox_match_rate": r.get("sandbox_match_rate"),
        "sandbox_behaviors": _n(r.get("sandbox_verify")),
        "host_match_rate": r.get("local_match_rate"),
        "host_behaviors": _n(r.get("local_verify")),
        "holdout_match_rate": r.get("holdout_match_rate", hold.get("match_rate")),
        "holdout_behaviors": _n(hold),
        "holdout_seed": hold.get("seed"),
        "match_rates_agree": r.get("match_rates_agree"),
        "interaction_id": r.get("interaction_id"),
        "environment_id": r.get("environment_id"),
        "error": r.get("error"),
    }


def list_runs(include_stubs: bool = False) -> list[dict[str, Any]]:
    """Newest first. Unreadable files are skipped; stub dev runs are hidden unless asked for."""
    out = []
    if not RUNS_REMOTE.is_dir():
        return out
    for path in sorted(RUNS_REMOTE.glob("*/*/result.json"), key=lambda p: p.stat().st_mtime, reverse=True):
        try:
            row = summarize(path)
        except (OSError, ValueError):
            continue
        if row["model"] == "stub" and not include_stubs:
            continue
        out.append(row)
    return out
