"""One table of every loop and sandbox run, read only from harness-written summary.json files.

    cd backend && uv run python -m ai.report [DIR ...]      # default: <root>/runs/ai <root>/results

Every number in the pitch should trace to a row here. Rates come from verify.sh
reports the loop saved (held-out, adversarial, sandbox host re-verify), never from
model text. Cost uses api/cost.py's prices; runs written before summary.json had
"usd" are priced from their per-round usage. Quality columns (fmt, clippy, panic_lints,
panics) come from ai.quality's tool output in the same summary.json; older runs show "-".

The second table groups runs by (target, model, round budget), so repeated runs
show n, the mean and minimum held-out rate, and how many reached 1.0.
"""

from __future__ import annotations

import argparse
import ast
import json
import re
import sys
from pathlib import Path

from .loop import find_root, usd


def _frac(block: dict | None) -> str:
    if not block or block.get("match_rate") is None:
        return "-"
    counts = block.get("counts") or {}
    tested = block.get("behaviors_tested")
    if tested is None and counts:
        tested = sum(counts.values())
    return f"{counts.get('match', '?')}/{tested}" if counts else f"{block['match_rate']:.4f}"


_STATUS = re.compile(r"ended with status '(\w+)'(?: after ([\d.]+)s)?")
_USAGE = re.compile(r"usage (\{.*?\})")


def failure(error: str) -> tuple[str, float | None]:
    """Short reason for a failed call, plus its cost when the message carries usage.

    A call that ends 'incomplete' spent its whole output budget (mostly thinking), so
    it still costs money; leaving it out would understate a model's real cost.
    """
    m = _STATUS.search(error)
    reason = f"{m.group(1)} after {m.group(2)}s" if m and m.group(2) else (m.group(1) if m else error[:70])
    u = _USAGE.search(error)
    try:
        return reason, usd(ast.literal_eval(u.group(1))) if u else None
    except (ValueError, SyntaxError):
        return reason, None


def row(path: Path, s: dict) -> dict:
    """Flatten one summary.json (ai.loop or ai.sandbox_run) into report columns."""
    rounds = s.get("rounds") or []
    cost = s.get("usd")
    if cost is None:
        cost = sum(usd(r.get("usage") or {}) for r in rounds) or (usd(s["usage"]) if s.get("usage") else None)
    holdout = s.get("holdout") or s.get("host_holdout")
    adv = s.get("adversarial") or {}
    error = s.get("error") or next((r["error"] for r in rounds if r.get("error")), None)
    if error:
        error, failed_cost = failure(error)
        if failed_cost is not None:
            cost = (cost or 0) + failed_cost
    q = s.get("quality") or {}
    return {
        "run": str(path.parent),
        "kind": "sandbox" if "agent" in s else "loop",
        "target": s.get("target", "cjson"),
        "model": s.get("model"),
        "budget": s.get("rounds_budget"),
        "rounds": s.get("rounds_run", len(rounds) or None),
        "loop_rate": s.get("best_match_rate", s.get("final_match_rate")),
        "holdout_rate": (holdout or {}).get("match_rate"),
        "holdout": _frac(holdout),
        "targeted": _frac(adv.get("targeted")),
        "argon_corpus": _frac(adv.get("argon")),
        "fmt": {True: "ok", False: "FAIL"}.get(q.get("fmt_ok"), "-"),
        "clippy": q.get("clippy_warnings"),
        "panic_lints": q.get("panic_lints"),
        "panics": (s.get("panics") or {}).get("total"),
        "host_fixed": q.get("host_fixed"),
        "seconds": s.get("total_model_seconds", s.get("seconds")),
        "usd": round(cost, 2) if cost is not None else None,
        "error": error or "",
    }


def collect(dirs: list[Path]) -> list[dict]:
    rows = []
    for d in dirs:
        for path in sorted(d.rglob("summary.json")):
            try:
                rows.append(row(path, json.loads(path.read_text())))
            except (json.JSONDecodeError, OSError) as exc:
                print(f"skipped {path}: {exc}", file=sys.stderr)
    return rows


def groups(rows: list[dict]) -> list[dict]:
    by: dict[tuple, list[dict]] = {}
    for r in rows:
        by.setdefault((r["target"], r["model"], r["kind"], r["budget"]), []).append(r)
    out = []
    for (target, model, kind, budget), rs in sorted(by.items(), key=lambda kv: tuple(map(str, kv[0]))):
        rates = [r["holdout_rate"] for r in rs if r["holdout_rate"] is not None]
        out.append({
            "target": target, "model": model, "kind": kind, "budget": budget, "n": len(rs),
            "failed": sum(1 for r in rs if r["error"]),
            "holdout_mean": round(sum(rates) / len(rates), 4) if rates else None,
            "holdout_min": min(rates) if rates else None,
            "reached_1.0": sum(1 for x in rates if x == 1.0),
            "clippy_max": max((r["clippy"] for r in rs if r["clippy"] is not None), default=None),
            "panic_lints_max": max((r["panic_lints"] for r in rs if r["panic_lints"] is not None), default=None),
            "panics_max": max((r["panics"] for r in rs if r["panics"] is not None), default=None),
            "usd": round(sum(r["usd"] or 0 for r in rs), 2),
        })
    return out


def table(rows: list[dict], cols: list[str]) -> str:
    cell = lambda v: "-" if v is None or v == "" else str(v)
    lines = ["| " + " | ".join(cols) + " |", "|" + "---|" * len(cols)]
    lines += ["| " + " | ".join(cell(r[c]).replace("|", "/") for c in cols) + " |" for r in rows]
    return "\n".join(lines)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("dirs", nargs="*", help="directories to search for summary.json")
    ap.add_argument("--json", action="store_true", help="print rows and groups as JSON")
    args = ap.parse_args(argv)
    root = find_root()
    dirs = [Path(d).resolve() for d in args.dirs] or [root / "runs/ai", root / "results"]
    rows = collect([d for d in dirs if d.is_dir()])
    for r in rows:
        r["run"] = r["run"].replace(str(root) + "/", "")
    if args.json:
        print(json.dumps({"runs": rows, "groups": groups(rows)}, indent=2))
        return 0
    print(table(rows, ["run", "kind", "model", "budget", "rounds", "loop_rate", "holdout",
                       "targeted", "argon_corpus", "fmt", "clippy", "panic_lints", "panics", "host_fixed",
                       "seconds", "usd", "error"]))
    print()
    print(table(groups(rows), ["target", "model", "kind", "budget", "n", "failed",
                               "holdout_mean", "holdout_min", "reached_1.0", "clippy_max", "panic_lints_max",
                               "panics_max", "usd"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
