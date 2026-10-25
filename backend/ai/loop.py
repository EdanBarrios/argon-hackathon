"""Translate -> build -> verify -> repair loop for cJSON parse + print.

    cd backend && uv run python -m ai.loop --model gemini-4-argon --rounds 5 [--target cjson] [--out DIR]

Round 1 is translate(); every later round is repair() fed with what the harness
reported: compile errors from build_rust.sh, or divergences and crashes from
verify.sh. Stops at match_rate 1.0 or when the round budget (model calls) runs out.

Harness tools, the cJSON sources, the C driver and runs/ are taken from the
"harness root": the nearest parent checkout that has backend/harness/vendor/cJSON
(the main checkout when this runs from a git worktree). Override with --root or
THESEUS_ROOT. Each round saves cjson.rs, build.json, verify.json, the feedback
sent, usage and seconds under <out>/round_N/, and summary.json at the end.

Held-out check: after the last round, the best build (highest loop match rate; the
later round wins a tie, as in api.pipeline) is verified once more on
inputs from --holdout-seed (default seed + 1000), minus any input whose bytes also
appear in the loop's input set. The model never sees these inputs or their results,
so summary.json's "holdout" rate is the number to quote, not the loop's own rate.

Adversarial check: when the target has committed adversarial corpora (cJSON), the
same best build also runs through C's adversarial/run.sh, and summary.json gets
per-corpus rates under "adversarial". Generated inputs alone cannot tell Argon from
Flash on cJSON (both 1.0 held-out); the targeted corpus can (980/980 vs 961/980).
"""

from __future__ import annotations

import argparse
import dataclasses
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime
from pathlib import Path

from . import prompts
from .targets import CJSON, TARGETS, Target
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


def ensure_c_driver(root: Path, target: Target = CJSON) -> Path:
    driver = root / target.c_driver
    if not driver.exists():
        subprocess.run([str(root / target.build_c)], check=True)
    return driver


def ensure_inputs(root: Path, count: int, seed: int, target: Target = CJSON) -> Path:
    out = root / "runs/ai" / f"{target.inputs_prefix}-c{count}-s{seed}"
    if out.is_dir() and any(out.iterdir()):
        return out
    subprocess.run(target.gen_inputs(root, seed, count, out), check=True)
    return out


def holdout_inputs(root: Path, count: int, seed: int, seen_dir: Path,
                   target: Target = CJSON) -> tuple[Path, int]:
    """Inputs generated at `seed`, minus any whose bytes appear in seen_dir.

    Returns (directory, number removed as overlap). gen_inputs.py always includes
    cJSON's own test files and the hand-written edge cases, so two seeds overlap;
    removing the overlap keeps the held-out set truly unseen.
    """
    generated = ensure_inputs(root, count, seed, target)
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


def first_diff(c_out: bytes, r_out: bytes) -> dict:
    """First output line where the two drivers differ (1-based), each side cut to 300 chars."""
    cl, rl = c_out.decode("utf-8", "backslashreplace").split("\n"), r_out.decode("utf-8", "backslashreplace").split("\n")
    i = next((j for j, (a, b) in enumerate(zip(cl, rl)) if a != b), min(len(cl), len(rl)))
    pick = lambda lines: lines[i][:300] if i < len(lines) else None
    return {"line": i + 1, "c": pick(cl), "rust": pick(rl)}


def add_first_diffs(divergences: list[dict], inputs: Path, c_driver: Path, rust_bin: str,
                    jobs: int, timeout: float) -> list[dict]:
    """Copies of verify.sh's divergences with first_diff added, for the repair prompt only.

    verify.sh keeps only the first 300 chars of each output, so for long event streams
    the difference is often past the cut. This reruns both drivers on the same input
    file. It changes what the model is shown, never the counts or the match rate.
    """
    def one(d: dict) -> dict:
        d = dict(d)
        path = inputs / (d.get("file") or "")
        if not path.is_file():
            return d
        data = path.read_bytes()
        try:
            c = subprocess.run([str(c_driver)], input=data, capture_output=True, timeout=timeout).stdout
            r = subprocess.run([rust_bin], input=data, capture_output=True, timeout=timeout).stdout
        except subprocess.TimeoutExpired:
            return d
        d["first_diff"] = first_diff(c, r)
        return d

    with ThreadPoolExecutor(max_workers=max(1, jobs)) as pool:
        return list(pool.map(one, divergences))


ADVERSARIAL_CORPORA = ("targeted", "argon", "human")


def run_adversarial(root: Path, script: str, binary: str, label: str) -> dict:
    """Run C's adversarial/run.sh on one binary; returns {corpus: rates} from its reports.

    run.sh stages each corpus (dropping README/REASONS files), runs verify.sh at
    --timeout 20 and writes runs/adversarial/<label>-<corpus>.json. A corpus with no
    inputs gets no report and is left out.
    """
    reports = {c: root / "runs/adversarial" / f"{label}-{c}.json" for c in ADVERSARIAL_CORPORA}
    for report in reports.values():
        report.unlink(missing_ok=True)  # a reused --out must not pick up an old report
    proc = subprocess.run([str(root / script), binary, label, *ADVERSARIAL_CORPORA],
                          capture_output=True, text=True, timeout=3600)
    out: dict = {}
    for corpus, report in reports.items():
        if not report.is_file():
            continue
        data = json.loads(report.read_text())
        out[corpus] = {"match_rate": data.get("match_rate"),
                                             "behaviors_tested": data.get("behaviors_tested"),
                                             "counts": data.get("counts"), "report": str(report)}
    if not out:
        out["error"] = (proc.stderr + proc.stdout)[-2000:]
    return out


def usd(usage: dict) -> float:
    """Cost of one call's usage at the prices in api/cost.py (one source for pricing)."""
    from api.cost import usd_for

    return usd_for(input_tokens=int(usage.get("total_input_tokens") or 0),
                   cached_tokens=int(usage.get("total_cached_tokens") or 0),
                   output_tokens=int(usage.get("total_output_tokens") or 0),
                   thought_tokens=int(usage.get("total_thought_tokens") or 0))


def tokens(usage: dict) -> int:
    return sum(int(usage.get(k) or 0) for k in ("total_input_tokens", "total_output_tokens", "total_thought_tokens"))


def write_json(path: Path, data) -> None:
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--model", default="gemini-4-argon")
    ap.add_argument("--target", default=CJSON.name, choices=sorted(TARGETS))
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
    ap.add_argument("--continue-turns", type=int,
                    help="follow-up turns when a reply hits the output cap (default: the target's, 0 for cjson)")
    ap.add_argument("--no-holdout", action="store_true", help="skip the held-out check")
    ap.add_argument("--no-adversarial", action="store_true", help="skip the adversarial corpora check")
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    args = ap.parse_args(argv)

    target = TARGETS[args.target]
    if args.continue_turns is not None:
        target = dataclasses.replace(target, continue_turns=args.continue_turns)
    root = Path(args.root).resolve() if args.root else find_root()
    build_sh = root / target.build_rust
    verify_sh = root / "backend/harness/verify.sh"
    vendor = root / target.vendor
    for p in (build_sh, verify_sh, vendor / target.vendor_check):
        if not p.exists():
            print(f"missing harness piece: {p}", file=sys.stderr)
            return 2
    c_driver = ensure_c_driver(root, target)
    inputs = Path(args.inputs).resolve() if args.inputs else ensure_inputs(root, args.count, args.seed, target)

    stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    out = Path(args.out).resolve() if args.out else root / "runs/ai" / f"{stamp}-{args.model}"
    out.mkdir(parents=True, exist_ok=True)
    c_source = target.load_source(vendor)
    print(f"[loop] target={target.name} model={args.model} rounds={args.rounds} inputs={inputs} out={out}", flush=True)

    rounds: list[dict] = []
    prev: TranslateResult | None = None
    feedback: dict | None = None
    best: tuple[float, str, int] | None = None  # (loop match rate, binary, round)
    final_rate = None
    for n in range(1, args.rounds + 1):
        rdir = out / f"round_{n}"
        rdir.mkdir(exist_ok=True)
        rec: dict = {"round": n, "kind": "translate" if prev is None else "repair"}
        try:
            if prev is None:
                res = translate(c_source, model=args.model, target=target)
            else:
                write_json(rdir / "feedback_sent.json", trim_feedback(feedback or {}, target.divergence_pattern))
                res = repair(prev, feedback or {}, model=args.model, target=target)
        except TranslateError as exc:
            rec["error"] = str(exc)
            rounds.append(rec)
            print(f"round {n}: model call failed: {exc}", flush=True)
            break
        prev = res
        (rdir / target.rust_file).write_text(res.rust_source, encoding="utf-8")
        rec.update(interaction_id=res.interaction_id, seconds=res.seconds, usage=res.usage,
                   thinking_level=res.thinking_level, notes=res.notes, continuations=res.continuations, rust_lines=res.rust_source.count("\n"))

        build_id = re.sub(r"[^A-Za-z0-9._-]", "_", f"ai-{stamp}-{args.model}-r{n}")
        build, raw = run_json([str(build_sh), str(rdir / target.rust_file), build_id], timeout=600)
        if build is None:
            build = {"ok": False, "errors": f"build_rust.sh produced no JSON: {raw}"}
        write_json(rdir / "build.json", build)
        rec["compile_ok"] = bool(build.get("ok"))
        rec["build_seconds"] = build.get("build_seconds")

        verify = None
        if build.get("ok"):
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
            if target.diff_context and verify.get("divergences"):
                feedback["divergences"] = add_first_diffs(verify["divergences"], inputs, c_driver,
                                                          build["binary"], args.jobs, args.timeout)
            counts = verify.get("counts") or {}
            rec.update(match_rate=verify.get("match_rate"), divergences=counts.get("divergence"),
                       crashes=counts.get("rust_crash", 0) + counts.get("both_crash", 0),
                       timeouts=counts.get("timeout"), safety_improvements=counts.get("safety_improvement"),
                       behaviors_tested=verify.get("behaviors_tested"))
            final_rate = verify.get("match_rate")
            # Same rule as api.pipeline: keep the best build, later round wins a tie,
            # so a repair that regresses never becomes the build we hold out.
            rate = float(final_rate or 0.0)
            if best is None or rate >= best[0]:
                best = (rate, build["binary"], n)
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

    best_binary = best[1] if best else None
    summary: dict = {
        "model": args.model,
        "target": target.name,
        "rounds_budget": args.rounds,
        "rounds_run": len(rounds),
        "inputs": str(inputs),
        "verify_timeout": args.timeout,
        "final_match_rate": final_rate,
        "best_round": best[2] if best else None,
        "best_match_rate": best[0] if best else None,
        "rounds_to_100": next((r["round"] for r in rounds if r.get("match_rate") == 1.0), None),
        "total_model_seconds": round(sum(r.get("seconds") or 0 for r in rounds), 1),
        "total_tokens": {k: sum(int((r.get("usage") or {}).get(k) or 0) for r in rounds)
                         for k in ("total_input_tokens", "total_output_tokens", "total_thought_tokens", "total_cached_tokens")},
        "usd": round(sum(usd(r.get("usage") or {}) for r in rounds), 4),
        "rounds": rounds,
    }
    if args.final_inputs and best_binary:
        final, _ = run_json(
            [str(verify_sh), "--c", str(c_driver), "--rust", best_binary,
             "--inputs", str(Path(args.final_inputs).resolve()), "--timeout", str(args.timeout), "--jobs", str(args.jobs)],
            timeout=3600,
        )
        if final is not None:
            write_json(out / "final_verify.json", final)
            summary["final_full"] = {"match_rate": final.get("match_rate"),
                                     "behaviors_tested": final.get("behaviors_tested"),
                                     "counts": final.get("counts")}
            print(f"[loop] final verify on {args.final_inputs}: match_rate {final.get('match_rate')} {final.get('counts')}")
    if not args.no_holdout and best_binary:
        seed = args.holdout_seed if args.holdout_seed is not None else args.seed + 1000
        hdir, overlap = holdout_inputs(root, args.holdout_count, seed, inputs, target)
        held = {"seed": seed, "inputs": str(hdir), "overlap_removed": overlap,
                "behaviors_tested": sum(1 for p in hdir.iterdir() if p.is_file())}
        if held["behaviors_tested"]:
            hv, raw = run_json(
                [str(verify_sh), "--c", str(c_driver), "--rust", best_binary,
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
    if target.adversarial_run and not args.no_adversarial and best_binary:
        summary["adversarial"] = run_adversarial(root, target.adversarial_run, best_binary,
                                                 f"loop-{out.name}")
        print(f"[loop] adversarial: " + ", ".join(
            f"{k} {v.get('counts', {}).get('match')}/{v.get('behaviors_tested')}"
            for k, v in summary["adversarial"].items() if isinstance(v, dict)), flush=True)
    write_json(out / "summary.json", summary)
    print(f"[loop] summary: {out / 'summary.json'}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
