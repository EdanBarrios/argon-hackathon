"""Run Theseus translate-and-verify in managed-agent sandboxes, in parallel, then re-verify locally.

Usage:
  cd backend && uv run python ../scripts/run_remote.py --library cjson --models gemini-4-argon,gemini-3.8-flash --rounds 4
  cd backend && uv run python ../scripts/run_remote.py --resume <interaction_id> --job-dir runs/remote/<stamp>/cjson-gemini-4-argon

One sandbox per (library, model). Everything lands in runs/remote/<timestamp>/:
summary.json plus one directory per job (prompt, transcript, agent output,
returned cjson.rs, sandbox out/ dir, local verify report, result.json).
"""

from __future__ import annotations

import argparse
import json
import sys
import time
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "backend"))

from harness.remote._common import PreflightError, run_preflight  # noqa: E402
from harness.remote.run_remote import RemoteSpec, resume_remote, run_parallel  # noqa: E402


def fmt_rate(x) -> str:
    return "-" if x is None else f"{x:.4f}"


def table(results) -> str:
    head = ["library", "model", "status", "seconds", "sandbox_rate", "local_rate", "holdout_rate", "agree", "rounds",
            "in_tok", "out_tok", "thought_tok", "cost_usd"]
    rows = [head]
    for r in results:
        u = r.usage or {}
        rows.append([
            r.library, r.model, r.status, f"{r.seconds:.0f}", fmt_rate(r.sandbox_match_rate),
            fmt_rate(r.local_match_rate), fmt_rate(r.holdout_match_rate), "-" if r.match_rates_agree is None else ("yes" if r.match_rates_agree else "NO"),
            "-" if r.rounds_used is None else str(r.rounds_used),
            str(u.get("total_input_tokens", 0)), str(u.get("total_output_tokens", 0)),
            str(u.get("total_thought_tokens", 0)), f"{r.cost_usd:.2f}",
        ])
    widths = [max(len(row[i]) for row in rows) for i in range(len(head))]
    return "\n".join("  ".join(c.ljust(w) for c, w in zip(row, widths)) for row in rows)


def log(msg: str) -> None:
    print(f"{datetime.now().strftime('%H:%M:%S')} {msg}", flush=True)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--library", default="cjson", help="comma-separated libraries (cjson)")
    ap.add_argument("--models", default="gemini-4-argon", help="comma-separated models")
    ap.add_argument("--rounds", type=int, default=4, help="total rounds: 1 translate + (rounds-1) repairs")
    ap.add_argument("--inputs", type=int, default=300, help="generated inputs per verify run")
    ap.add_argument("--seed", type=int, default=1, help="held-out check uses seed + 1000")
    ap.add_argument("--max-wait", type=float, default=90 * 60, help="seconds per sandbox before giving up")
    ap.add_argument("--resume", metavar="INTERACTION_ID", help="re-attach to an existing sandbox run")
    ap.add_argument("--job-dir", help="with --resume: the job directory to write into")
    ap.add_argument("--skip-preflight", action="store_true",
                    help="do NOT run scripts/preflight.sh before the model call (not recommended)")
    args = ap.parse_args()

    if not args.resume:
        if args.skip_preflight:
            log("WARNING: --skip-preflight: the verifier gate was not run; match rates are unchecked")
        else:
            for lib in dict.fromkeys(args.library.split(",")):
                try:
                    run_preflight(lib, on_event=log)
                except PreflightError as exc:
                    print(f"\n!! {exc}", file=sys.stderr)
                    return 3

    t0 = time.monotonic()
    if args.resume:
        if not args.job_dir:
            ap.error("--resume needs --job-dir")
        model = args.models.split(",")[0]
        results = [resume_remote(args.resume, args.job_dir, args.library.split(",")[0], model, args.rounds,
                                 args.inputs, seed=args.seed, on_event=log, max_wait=args.max_wait)]
        root = Path(args.job_dir).parent
    else:
        specs = [RemoteSpec(lib, m, args.rounds, args.inputs, args.seed)
                 for lib in args.library.split(",") for m in args.models.split(",")]
        root = ROOT / "runs" / "remote" / datetime.now().strftime("%Y%m%d-%H%M%S")
        log(f"{len(specs)} sandbox job(s) in parallel, saving to {root}")
        results = run_parallel(specs, job_root=root, on_event=log, max_wait=args.max_wait,
                               skip_preflight=True)  # gate already ran above (or was skipped explicitly)

    summary = {"wall_seconds": round(time.monotonic() - t0, 1), "results": [r.to_dict() for r in results]}
    root.mkdir(parents=True, exist_ok=True)
    (root / "summary.json").write_text(json.dumps(summary, indent=2, default=str))
    print()
    print(table(results))
    for r in results:
        for n in r.notes:
            if n.startswith(("MISMATCH", "WARNING")):
                print(f"!! {r.library}/{r.model}: {n}")
        if r.error:
            print(f"!! {r.library}/{r.model}: {r.error}")
    print(f"\nsaved: {root / 'summary.json'}")
    return 0 if all(r.local_match_rate is not None for r in results) else 1


if __name__ == "__main__":
    sys.exit(main())
