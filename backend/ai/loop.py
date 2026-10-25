"""Translate -> build -> verify -> repair loop for cJSON parse + print.

    cd backend && uv run python -m ai.loop --model gemini-4-argon --rounds 5 [--out DIR]

Round 1 is translate(); every later round is repair() fed with what the harness
reported: compile errors from build_rust.sh, or divergences and crashes from
verify.sh. Stops at match_rate 1.0 or when the round budget (model calls) runs out.

Harness tools, the cJSON sources, the C driver and runs/ are taken from the
"harness root": the nearest parent checkout that has backend/harness/vendor/cJSON
(the main checkout when this runs from a git worktree). Override with --root or
THESEUS_ROOT. Each round saves cjson.rs, build.json, verify.json, the feedback
sent, usage and seconds under <out>/round_N/, and summary.json at the end.

Held-out check: after the last round, the last good build is verified once more on
inputs from --holdout-seed (default seed + 1000), minus any input whose bytes also
appear in the loop's input set. The model never sees these inputs or their results,
so summary.json's "holdout" rate is the number to quote, not the loop's own rate.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path

from . import prompts
from .translate import TranslateError, TranslateResult, repair, translate, trim_feedback


def find_root() -> Path:
    env = os.environ.get("THESEUS_ROOT")
    if env:
        return Path(env).resolve()
    for parent in Path(__file__).resolve().parents:
        if (parent / "backend/harness/vendor/cJSON/cJSON.c").is_file():
            return parent
    return prompts.REPO_ROOT


def run_json(cmd: list[str], timeout: float, cwd: Path | None = None) -> tuple[dict | None, str]:
    """Run a harness tool that prints JSON on stdout. Returns (parsed or None, raw stderr+stdout tail)."""
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, cwd=cwd)
    except subprocess.TimeoutExpired:
        return None, f"{cmd[0]} timed out after {timeout}s"
    out = proc.stdout.strip()
    try:
        return json.loads(out), proc.stderr
    except json.JSONDecodeError:
        return None, (proc.stderr + "\n" + out)[-4000:]


def ensure_c_driver(root: Path) -> Path:
    driver = root / "backend/harness/build/cjson_driver"
    if not driver.exists():
        subprocess.run([str(root / "backend/harness/build_cjson.sh")], check=True)
    return driver


def ensure_inputs(root: Path, count: int, seed: int) -> Path:
    out = root / "runs/ai" / f"inputs-c{count}-s{seed}"
    if out.is_dir() and any(out.iterdir()):
        return out
    subprocess.run(
        [sys.executable, "-I", str(root / "backend/harness/gen_inputs.py"),
         "--seed", str(seed), "--count", str(count), "--out", str(out),
         "--cjson", str(root / "backend/harness/vendor/cJSON")],
        check=True,
    )
    return out


def holdout_inputs(root: Path, count: int, seed: int, seen_dir: Path) -> tuple[Path, int]:
    """Inputs generated at `seed`, minus any whose bytes appear in seen_dir.

    Returns (directory, number removed as overlap). gen_inputs.py always includes
    cJSON's own test files and the hand-written edge cases, so two seeds overlap;
    removing the overlap keeps the held-out set truly unseen.
    """
    generated = ensure_inputs(root, count, seed)
    seen = {hashlib.sha256(p.read_bytes()).digest() for p in seen_dir.iterdir() if p.is_file()}
    out = root / "runs/ai" / f"holdout-c{count}-s{seed}-minus-{seen_dir.name}"
    if out.is_dir():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    removed = 0
    for p in sorted(generated.iterdir()):
        if not p.is_file():
            continue
        data = p.read_bytes()
        if hashlib.sha256(data).digest() in seen:
            removed += 1
            continue
        (out / p.name).write_bytes(data)
    return out, removed


def tokens(usage: dict) -> int:
    return sum(int(usage.get(k) or 0) for k in ("total_input_tokens", "total_output_tokens", "total_thought_tokens"))


def write_json(path: Path, data) -> None:
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--model", default="gemini-4-argon")
    ap.add_argument("--rounds", type=int, default=5, help="model calls: 1 translate + (rounds-1) repairs")
    ap.add_argument("--out", help="output dir (default <root>/runs/ai/<timestamp>-<model>)")
    ap.add_argument("--root", help="harness root (default: auto, see module doc)")
    ap.add_argument("--inputs", help="input dir (default: generate --count/--seed into runs/ai/)")
    ap.add_argument("--count", type=int, default=300, help="generated inputs per verify (harness README: 300 for loops)")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--final-inputs", help="optional bigger input dir to verify the last good build against")
    ap.add_argument("--holdout-seed", type=int, help="seed for the held-out check (default: --seed + 1000)")
    ap.add_argument("--holdout-count", type=int, default=1000,
                    help="inputs generated for the held-out check before overlap removal; 236 are fixed "
                         "seeds/edge cases shared by every seed, so 300 would leave only 64 unseen")
    ap.add_argument("--timeout", type=float, default=20.0,
                    help="verify.sh seconds per run (its default 5 s gives false timeouts under ASan on macOS)")
    ap.add_argument("--no-holdout", action="store_true", help="skip the held-out check")
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    args = ap.parse_args(argv)

    root = Path(args.root).resolve() if args.root else find_root()
    build_sh = root / "scripts/build_rust.sh"
    verify_sh = root / "backend/harness/verify.sh"
    vendor = root / "backend/harness/vendor/cJSON"
    for p in (build_sh, verify_sh, vendor / "cJSON.c"):
        if not p.exists():
            print(f"missing harness piece: {p}", file=sys.stderr)
            return 2
    c_driver = ensure_c_driver(root)
    inputs = Path(args.inputs).resolve() if args.inputs else ensure_inputs(root, args.count, args.seed)

    stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    out = Path(args.out).resolve() if args.out else root / "runs/ai" / f"{stamp}-{args.model}"
    out.mkdir(parents=True, exist_ok=True)
    c_source = prompts.load_cjson_source(vendor)
    print(f"[loop] model={args.model} rounds={args.rounds} inputs={inputs} out={out}", flush=True)

    rounds: list[dict] = []
    prev: TranslateResult | None = None
    feedback: dict | None = None
    last_good_binary: str | None = None
    final_rate = None
    for n in range(1, args.rounds + 1):
        rdir = out / f"round_{n}"
        rdir.mkdir(exist_ok=True)
        rec: dict = {"round": n, "kind": "translate" if prev is None else "repair"}
        try:
            if prev is None:
                res = translate(c_source, model=args.model)
            else:
                write_json(rdir / "feedback_sent.json", trim_feedback(feedback or {}))
                res = repair(prev, feedback or {}, model=args.model)
        except TranslateError as exc:
            rec["error"] = str(exc)
            rounds.append(rec)
            print(f"round {n}: model call failed: {exc}", flush=True)
            break
        prev = res
        (rdir / "cjson.rs").write_text(res.rust_source, encoding="utf-8")
        rec.update(interaction_id=res.interaction_id, seconds=res.seconds, usage=res.usage,
                   thinking_level=res.thinking_level, notes=res.notes, rust_lines=res.rust_source.count("\n"))

        build_id = re.sub(r"[^A-Za-z0-9._-]", "_", f"ai-{stamp}-{args.model}-r{n}")
        build, raw = run_json([str(build_sh), str(rdir / "cjson.rs"), build_id], timeout=600)
        if build is None:
            build = {"ok": False, "errors": f"build_rust.sh produced no JSON: {raw}"}
        write_json(rdir / "build.json", build)
        rec["compile_ok"] = bool(build.get("ok"))
        rec["build_seconds"] = build.get("build_seconds")

        verify = None
        if build.get("ok"):
            last_good_binary = build["binary"]
            verify, raw = run_json(
                [str(verify_sh), "--c", str(c_driver), "--rust", build["binary"],
                 "--inputs", str(inputs), "--timeout", str(args.timeout), "--jobs", str(args.jobs)],
                timeout=1800,
            )
            if verify is None:
                print(f"round {n}: verify.sh produced no JSON: {raw[-500:]}", file=sys.stderr)
                rec["verify_error"] = raw[-2000:]
                verify = {}
            write_json(rdir / "verify.json", verify)
            feedback = dict(verify, compile_errors=None)
            counts = verify.get("counts") or {}
            rec.update(match_rate=verify.get("match_rate"), divergences=counts.get("divergence"),
                       crashes=counts.get("rust_crash", 0) + counts.get("both_crash", 0),
                       timeouts=counts.get("timeout"), safety_improvements=counts.get("safety_improvement"),
                       behaviors_tested=verify.get("behaviors_tested"))
            final_rate = verify.get("match_rate")
        else:
            feedback = {"compile_errors": build.get("errors") or "build failed", "divergences": [], "crashes": []}
            rec.update(match_rate=None, divergences=None, crashes=None)
        write_json(rdir / "meta.json", rec)
        rounds.append(rec)
        print(
            f"round {n} | compile {'ok' if rec['compile_ok'] else 'FAIL'} | match_rate "
            f"{rec['match_rate'] if rec['match_rate'] is not None else '-'} | divergences {rec['divergences'] if rec['divergences'] is not None else '-'} "
            f"| crashes {rec['crashes'] if rec['crashes'] is not None else '-'} | {res.seconds}s | tokens {tokens(res.usage)} "
            f"(in {res.usage.get('total_input_tokens')}, out {res.usage.get('total_output_tokens')}, "
            f"thought {res.usage.get('total_thought_tokens')}, cached {res.usage.get('total_cached_tokens')})",
            flush=True,
        )
        if verify and verify.get("match_rate") == 1.0:
            break

    summary: dict = {
        "model": args.model,
        "rounds_budget": args.rounds,
        "rounds_run": len(rounds),
        "inputs": str(inputs),
        "verify_timeout": args.timeout,
        "final_match_rate": final_rate,
        "rounds_to_100": next((r["round"] for r in rounds if r.get("match_rate") == 1.0), None),
        "total_model_seconds": round(sum(r.get("seconds") or 0 for r in rounds), 1),
        "total_tokens": {k: sum(int((r.get("usage") or {}).get(k) or 0) for r in rounds)
                         for k in ("total_input_tokens", "total_output_tokens", "total_thought_tokens", "total_cached_tokens")},
        "rounds": rounds,
    }
    if args.final_inputs and last_good_binary:
        final, _ = run_json(
            [str(verify_sh), "--c", str(c_driver), "--rust", last_good_binary,
             "--inputs", str(Path(args.final_inputs).resolve()), "--timeout", str(args.timeout), "--jobs", str(args.jobs)],
            timeout=3600,
        )
        if final is not None:
            write_json(out / "final_verify.json", final)
            summary["final_full"] = {"match_rate": final.get("match_rate"),
                                     "behaviors_tested": final.get("behaviors_tested"),
                                     "counts": final.get("counts")}
            print(f"[loop] final verify on {args.final_inputs}: match_rate {final.get('match_rate')} {final.get('counts')}")
    if not args.no_holdout and last_good_binary:
        seed = args.holdout_seed if args.holdout_seed is not None else args.seed + 1000
        hdir, overlap = holdout_inputs(root, args.holdout_count, seed, inputs)
        held = {"seed": seed, "inputs": str(hdir), "overlap_removed": overlap,
                "behaviors_tested": sum(1 for p in hdir.iterdir() if p.is_file())}
        if held["behaviors_tested"]:
            hv, raw = run_json(
                [str(verify_sh), "--c", str(c_driver), "--rust", last_good_binary,
                 "--inputs", str(hdir), "--timeout", str(args.timeout), "--jobs", str(args.jobs)],
                timeout=1800,
            )
            if hv is None:
                held["error"] = raw[-2000:]
            else:
                write_json(out / "holdout_verify.json", hv)
                held.update(match_rate=hv.get("match_rate"), counts=hv.get("counts"))
        summary["holdout"] = held
        print(f"[loop] held-out seed {seed} ({held['behaviors_tested']} unseen inputs, {overlap} overlap removed): "
              f"match_rate {held.get('match_rate')} {held.get('counts')}", flush=True)
    write_json(out / "summary.json", summary)
    print(f"[loop] summary: {out / 'summary.json'}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
