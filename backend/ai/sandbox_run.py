"""Managed-agent pre-run: translate, build, verify and repair inside a remote sandbox.

    cd backend && uv run python -m ai.sandbox_run --model gemini-4-argon --rounds 5
    cd backend && uv run python -m ai.sandbox_run --environment <id>   # reuse a sandbox (skips ~46 s setup)
    cd backend && uv run python -m ai.sandbox_run --dry-run            # print the request, no API call

One background antigravity interaction does the whole loop in the sandbox, using the
same harness scripts as the local loop (prompts.SANDBOX_TASK). Nothing secret goes in:
the sandbox clones the public repo, so no credentials are needed.

The agent's own numbers are claims. The host takes the final cjson.rs from the reply,
rebuilds it locally, verifies it on the same seed (does the claim hold?) and on a
held-out seed the agent never saw (the number to quote). Results go to
<root>/runs/ai/sandbox-<timestamp>-<model>/summary.json.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
from datetime import datetime
from pathlib import Path

from . import prompts
from .loop import ensure_c_driver, ensure_inputs, find_root, holdout_inputs, run_json, write_json
from .translate import TranslateError, _get_client, _usage_dict, _wait, extract_rust

AGENT = "antigravity-preview-09-2026"
# Same hosts as scripts/sandbox_smoke.py --apt, the configuration proven at 11:22
# (clang 18 + ASan + Rust + harness all ran in the sandbox).
ALLOWLIST = [
    "github.com",
    "static.rust-lang.org",
    "sh.rustup.rs",
    "crates.io",
    "index.crates.io",
    "static.crates.io",
    "archive.ubuntu.com",
    "security.ubuntu.com",
    "ports.ubuntu.com",
    "dl.google.com",
    "packages.cloud.google.com",
    "deb.nodesource.com",
]
ENVIRONMENT = {"type": "remote", "network": {"allowlist": [{"domain": d} for d in ALLOWLIST]}}

_ROUND_LINE = re.compile(r"^\s*ROUND\s+(\d+)\s+build=(ok|fail)\s+match_rate=(\S+)", re.MULTILINE)


def parse_claims(text: str | None) -> list[dict]:
    """The agent's per-round ROUND lines. Claims only: the host re-verifies."""
    claims = []
    for n, build, rate in _ROUND_LINE.findall(text or ""):
        try:
            value: float | None = float(rate)
        except ValueError:
            value = None
        claims.append({"round": int(n), "build_ok": build == "ok", "match_rate": value})
    return claims


def request(args: argparse.Namespace, c_source: str) -> dict:
    """interactions.create kwargs. A reused sandbox is named by its environment id string."""
    return {
        "agent": AGENT,
        "agent_config": {"type": "antigravity", "model": args.model},
        "system_instruction": prompts.SYSTEM_INSTRUCTION,
        "input": prompts.sandbox_input(
            c_source, branch=args.branch, seed=args.seed, count=args.count, rounds=args.rounds
        ),
        "environment": args.environment or ENVIRONMENT,
        "background": True,
    }


def host_verify(verify_sh: Path, c_driver: Path, binary: str, inputs: Path, jobs: int) -> dict:
    report, raw = run_json(
        [str(verify_sh), "--c", str(c_driver), "--rust", binary, "--inputs", str(inputs),
         "--timeout", "20", "--jobs", str(jobs)],
        timeout=1800,
    )
    if report is None:
        return {"error": raw[-2000:]}
    return report


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--model", default="gemini-4-argon", help="model inside the antigravity agent")
    ap.add_argument("--rounds", type=int, default=5)
    ap.add_argument("--count", type=int, default=300)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--holdout-seed", type=int, help="default: --seed + 1000")
    ap.add_argument("--branch", default="main", help="branch of the public repo the sandbox clones")
    ap.add_argument("--environment", help="existing sandbox environment id to reuse")
    ap.add_argument("--root", help="harness root (default: auto, as in ai.loop)")
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    ap.add_argument("--dry-run", action="store_true", help="print the request and exit; no API call")
    args = ap.parse_args(argv)

    root = Path(args.root).resolve() if args.root else find_root()
    vendor = root / "backend/harness/vendor/cJSON"
    if not (vendor / "cJSON.c").exists():
        print(f"missing {vendor}/cJSON.c: run backend/harness/fetch_cjson.sh", file=sys.stderr)
        return 2
    kwargs = request(args, prompts.load_cjson_source(vendor))
    if args.dry_run:
        shown = dict(kwargs, input=f"<{len(kwargs['input'])} chars>",
                     system_instruction=f"<{len(kwargs['system_instruction'])} chars>")
        print(json.dumps(shown, indent=2))
        print(kwargs["input"].split("Reference material")[0])
        return 0

    stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    out = root / "runs/ai" / f"sandbox-{stamp}-{args.model}"
    out.mkdir(parents=True, exist_ok=True)
    summary: dict = {"model": args.model, "agent": AGENT, "rounds_budget": args.rounds,
                     "seed": args.seed, "count": args.count, "branch": args.branch}

    print(f"[sandbox] {AGENT} model={args.model} rounds={args.rounds} out={out}", flush=True)
    started = time.monotonic()
    client = _get_client()
    try:
        interaction = _wait(client, client.interactions.create(**kwargs))
    except Exception as exc:  # noqa: BLE001 - record any API failure in the summary
        summary["error"] = f"{type(exc).__name__}: {exc}"
        write_json(out / "summary.json", summary)
        print(f"[sandbox] model call failed: {summary['error']}", file=sys.stderr)
        return 1
    text = getattr(interaction, "output_text", None) or ""
    (out / "agent_output.md").write_text(text, encoding="utf-8")
    summary.update(
        interaction_id=interaction.id,
        environment_id=getattr(interaction, "environment_id", None),
        status=interaction.status,
        seconds=round(time.monotonic() - started, 1),
        usage=_usage_dict(interaction),
        claimed_rounds=parse_claims(text),
    )
    print(f"[sandbox] {interaction.status} in {summary['seconds']}s, environment {summary['environment_id']}",
          flush=True)
    if interaction.status != "completed":
        summary["error"] = f"interaction ended {interaction.status}: {getattr(interaction, 'errors', None)}"
        write_json(out / "summary.json", summary)
        return 1
    try:
        rust = extract_rust(text)
    except TranslateError as exc:
        summary["error"] = str(exc)
        write_json(out / "summary.json", summary)
        print(f"[sandbox] {exc}", file=sys.stderr)
        return 1
    (out / "cjson.rs").write_text(rust, encoding="utf-8")

    build, raw = run_json([str(root / "scripts/build_rust.sh"), str(out / "cjson.rs"), f"sandbox-{stamp}"],
                          timeout=600)
    summary["host_build"] = build or {"ok": False, "errors": raw[-2000:]}
    if not (build and build.get("ok")):
        write_json(out / "summary.json", summary)
        print("[sandbox] host build of the agent's final cjson.rs failed; see summary.json", file=sys.stderr)
        return 1

    verify_sh = root / "backend/harness/verify.sh"
    c_driver = ensure_c_driver(root)
    same = ensure_inputs(root, args.count, args.seed)
    seed = args.holdout_seed if args.holdout_seed is not None else args.seed + 1000
    held_dir, overlap = holdout_inputs(root, args.count, seed, same)
    for key, inputs in (("host_same_seed", same), ("host_holdout", held_dir)):
        report = host_verify(verify_sh, c_driver, build["binary"], inputs, args.jobs)
        write_json(out / f"{key}_verify.json", report)
        summary[key] = {"inputs": str(inputs), "match_rate": report.get("match_rate"),
                        "behaviors_tested": report.get("behaviors_tested"), "counts": report.get("counts"),
                        **({"error": report["error"]} if "error" in report else {})}
    summary["host_holdout"].update(seed=seed, overlap_removed=overlap)

    claimed = next((c["match_rate"] for c in reversed(summary["claimed_rounds"]) if c["build_ok"]), None)
    host = summary["host_same_seed"].get("match_rate")
    summary["claim_holds"] = claimed is not None and host is not None and abs(claimed - host) < 1e-9
    write_json(out / "summary.json", summary)
    print(f"[sandbox] agent claimed {claimed}; host same seed {host}; host held-out "
          f"{summary['host_holdout'].get('match_rate')} ({summary['host_holdout'].get('behaviors_tested')} unseen)",
          flush=True)
    print(f"[sandbox] summary: {out / 'summary.json'}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
