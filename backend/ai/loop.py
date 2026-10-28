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

Quality (ai.quality): every round that builds gets rustfmt and clippy counts. The host
then runs `cargo clippy --fix` and rustfmt on it, rebuilds and reverifies; the fixed
file replaces the model's as that round's candidate only if its loop match rate is no
lower, it has no more Rust crashes and its lint counts are no worse. The best build is
chosen by loop match rate, then lint counts, then the later round. Once the best build
matches every loop input, leftover rounds (at most --polish-rounds) are polish turns
fed with the remaining lints. A polish that changes behavior is never chosen; instead
the next round is a normal repair of that polished file (its divergences or compile
errors), which counts against the same polish budget. A matching round that is not
cleaner than the best build ends the loop.

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

from . import prompts, quality
from .targets import CJSON, TARGETS, Target
from .translate import TranslateError, TranslateResult, polish, repair, translate, trim_feedback


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


def candidate(n: int, source: str, binary: str, verify: dict, q: dict, host_fixed: bool = False) -> dict:
    counts = verify.get("counts") or {}
    rate = float(verify.get("match_rate") or 0.0)
    return {"round": n, "source": source, "binary": binary, "rate": rate, "counts": counts,
            "quality": q, "host_fixed": host_fixed, "key": (rate, *quality.rank(q), n)}


def _crashes(counts: dict | None) -> int:
    counts = counts or {}
    return int(counts.get("rust_crash") or 0) + int(counts.get("both_crash") or 0)


def host_fix(cand: dict, root: Path, target: Target, rdir: Path, build_verify, build_id: str) -> tuple[dict, dict]:
    """clippy --fix + rustfmt on cand's file, rebuilt and reverified on the loop inputs.

    Returns (the candidate to keep, a record of what happened). The fixed file is kept
    only if its behavior is no worse (same or higher loop match rate, no more Rust
    crashes) and its lint counts are no worse.
    """
    fixed, info = quality.autofix(root, target, cand["source"])
    if not info["changed"]:
        return cand, dict(info, applied=False)
    hdir = rdir / "host_fix"
    hdir.mkdir(exist_ok=True)
    (hdir / target.rust_file).write_text(fixed, encoding="utf-8")
    build, verify = build_verify(hdir / target.rust_file, build_id)
    write_json(hdir / "build.json", build)
    if verify is None or verify.get("verify_error"):
        return cand, dict(info, applied=False, compile_ok=bool(build.get("ok")))
    write_json(hdir / "verify.json", verify)
    q = quality.measure(root, target, fixed)
    new = candidate(cand["round"], fixed, build["binary"], verify, q, host_fixed=True)
    ok = (new["rate"] >= cand["rate"] and _crashes(new["counts"]) <= _crashes(cand["counts"])
          and quality.rank(q) >= quality.rank(cand["quality"]) and (q["fmt_ok"] or not cand["quality"]["fmt_ok"]))
    info.update(applied=ok, compile_ok=True, match_rate=new["rate"], counts=new["counts"],
                quality=quality.public(q))
    return (new if ok else cand), info


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
    ap.add_argument("--start-from", help="existing Rust file to use as round 1 instead of a translate call")
    ap.add_argument("--fresh-repair", action="store_true",
                    help="send each repair as a new conversation (C source + previous file + feedback)")
    ap.add_argument("--patch-repair", action="store_true",
                    help="repairs reply with SEARCH/REPLACE blocks applied on the host instead of the whole file "
                         "(falls back to a full-file repair when a block does not apply)")
    ap.add_argument("--polish-rounds", type=int,
                    help="max lint-only rounds after the best build matches every loop input (default: the target's)")
    ap.add_argument("--no-host-fix", action="store_true", help="skip the host's clippy --fix + rustfmt pass")
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
    best: dict | None = None  # candidate(): the build the held-out and adversarial checks use
    first_quality: dict | None = None  # the first model file that built, before any host fix
    final_rate = None
    polish_left = args.polish_rounds if args.polish_rounds is not None else target.polish_rounds
    # "repair" until the best build matches every loop input; then "polish" (lint feedback on
    # the best build) and, when a polish breaks behavior, "polish_repair" (the usual verifier
    # feedback on that polished file). Both of the last two spend polish_left.
    phase = "repair"

    def build_verify(src: Path, build_id: str) -> tuple[dict, dict | None]:
        build, raw = run_json([str(build_sh), str(src), build_id], timeout=600)
        if build is None:
            build = {"ok": False, "errors": f"build_rust.sh produced no JSON: {raw}"}
        if not build.get("ok"):
            return build, None
        verify, raw = run_json(
            [str(verify_sh), "--c", str(c_driver), "--rust", build["binary"],
             "--inputs", str(inputs), "--timeout", str(args.timeout), "--jobs", str(args.jobs)],
            timeout=1800,
        )
        if verify is None:
            print(f"verify.sh produced no JSON for {src}: {raw[-500:]}", file=sys.stderr)
            verify = {"verify_error": raw[-2000:]}
        return build, verify

    for n in range(1, args.rounds + 1):
        rdir = out / f"round_{n}"
        rdir.mkdir(exist_ok=True)
        rec: dict = {"round": n, "kind": "translate" if prev is None else phase}
        try:
            if prev is None and args.start_from:
                rec["kind"] = "start_from"
                rec["start_from"] = str(Path(args.start_from).resolve())
                res = TranslateResult(rust_source=Path(args.start_from).read_text(encoding="utf-8"),
                                      interaction_id="", usage={}, seconds=0.0, model=args.model)
            elif prev is None:
                res = translate(c_source, model=args.model, target=target)
            elif phase == "polish":
                lint_text = quality.format_lints(best["quality"])
                (rdir / "lints_sent.md").write_text(lint_text, encoding="utf-8")
                rec["polish_of_round"] = best["round"]
                fresh = c_source if (args.fresh_repair or not prev.interaction_id) else None
                res = polish(prev, best["source"], lint_text, model=args.model, target=target, fresh_source=fresh)
            else:
                write_json(rdir / "feedback_sent.json", trim_feedback(feedback or {}, target.divergence_pattern))
                res = repair(prev, feedback or {}, model=args.model, target=target,
                             fresh_source=c_source if (args.fresh_repair or (args.patch_repair and not prev.interaction_id)) else None,
                             patch=args.patch_repair)
        except TranslateError as exc:
            rec["error"] = str(exc)
            rounds.append(rec)
            print(f"round {n}: model call failed: {exc}", flush=True)
            break
        prev = res
        (rdir / target.rust_file).write_text(res.rust_source, encoding="utf-8")
        rec.update(interaction_id=res.interaction_id, seconds=res.seconds, usage=res.usage,
                   thinking_level=res.thinking_level, notes=res.notes, continuations=res.continuations,
                   repair_mode=res.repair_mode, rust_lines=res.rust_source.count("\n"))

        build_id = re.sub(r"[^A-Za-z0-9._-]", "_", f"ai-{stamp}-{args.model}-r{n}")
        build, verify = build_verify(rdir / target.rust_file, build_id)
        write_json(rdir / "build.json", build)
        rec["compile_ok"] = bool(build.get("ok"))
        rec["build_seconds"] = build.get("build_seconds")

        improved = False
        if verify is not None:
            if verify.get("verify_error"):
                rec["verify_error"] = verify["verify_error"]
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

            q = quality.measure(root, target, res.rust_source)
            first_quality = first_quality or quality.public(q)
            cand = candidate(n, res.rust_source, build["binary"], verify, q)
            fix_info = None
            if not args.no_host_fix:
                cand, fix_info = host_fix(cand, root, target, rdir, build_verify, f"{build_id}h")
            rec["quality"] = quality.public(q)
            if fix_info is not None:
                rec["host_fix"] = fix_info
            write_json(rdir / "quality.json", {"model": quality.public(q), "host_fix": fix_info,
                                               "diagnostics": q["diagnostics"]})
            # Behavior first, then lint counts, then the later round (as in api.pipeline),
            # so neither a regressing repair nor a regressing polish becomes the held-out build.
            improved = best is None or cand["key"][:-1] > best["key"][:-1]
            if best is None or cand["key"] >= best["key"]:
                best = cand
        else:
            feedback = {"compile_errors": build.get("errors") or "build failed", "divergences": [], "crashes": []}
            rec.update(match_rate=None, divergences=None, crashes=None)
        write_json(rdir / "meta.json", rec)
        rounds.append(rec)
        q_line = rec.get("quality") or {}
        print(
            f"round {n} ({rec['kind']}) | compile {'ok' if rec['compile_ok'] else 'FAIL'} | match_rate "
            f"{rec['match_rate'] if rec['match_rate'] is not None else '-'} | divergences {rec['divergences'] if rec['divergences'] is not None else '-'} "
            f"| crashes {rec['crashes'] if rec['crashes'] is not None else '-'} "
            f"| clippy {q_line.get('clippy_warnings', '-')} panic_lints {q_line.get('panic_lints', '-')} "
            f"host_fix {'applied' if (rec.get('host_fix') or {}).get('applied') else '-'} | {res.seconds}s | tokens {tokens(res.usage)} "
            f"(in {res.usage.get('total_input_tokens')}, out {res.usage.get('total_output_tokens')}, "
            f"thought {res.usage.get('total_thought_tokens')}, cached {res.usage.get('total_cached_tokens')})",
            flush=True,
        )
        if phase == "repair":
            if best and best["rate"] == 1.0:
                if quality.clean(best["quality"]) or polish_left <= 0:
                    break
                phase = "polish"
            continue
        polish_left -= 1
        broke = rec.get("match_rate") != 1.0  # did not build, or changed behavior on a loop input
        if not broke and not improved:
            print(f"[loop] round {n} matches but is not cleaner than round {best['round']}; stopping", flush=True)
            break
        if quality.clean(best["quality"]) or polish_left <= 0:
            break
        phase = "polish_repair" if broke else "polish"

    best_binary = best["binary"] if best else None
    best_file = None
    if best:
        best_file = out / "best" / target.rust_file
        best_file.parent.mkdir(exist_ok=True)
        best_file.write_text(best["source"], encoding="utf-8")
    summary: dict = {
        "model": args.model,
        "target": target.name,
        "rounds_budget": args.rounds,
        "start_from": args.start_from,
        "fresh_repair": args.fresh_repair,
        "patch_repair": args.patch_repair,
        "rounds_run": len(rounds),
        "inputs": str(inputs),
        "verify_timeout": args.timeout,
        "final_match_rate": final_rate,
        "best_round": best["round"] if best else None,
        "best_match_rate": best["rate"] if best else None,
        "best_host_fixed": best["host_fixed"] if best else None,
        "best_file": str(best_file) if best_file else None,
        "rounds_to_100": next((r["round"] for r in rounds if r.get("match_rate") == 1.0), None),
        "polish_rounds_run": sum(1 for r in rounds if r.get("kind") == "polish"),
        "quality": dict(quality.public(best["quality"]), host_fixed=best["host_fixed"],
                        clean=quality.clean(best["quality"])) if best else None,
        "quality_first_build": first_quality,
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
    # Any Rust crash (panic, abort, stack overflow) on any input set the best build ran.
    if best:
        sets = {"loop": best["counts"], "holdout": (summary.get("holdout") or {}).get("counts"),
                **{k: v.get("counts") for k, v in (summary.get("adversarial") or {}).items() if isinstance(v, dict)}}
        summary["panics"] = {**{k: _crashes(v) for k, v in sets.items() if v is not None},
                             "total": sum(_crashes(v) for v in sets.values() if v is not None)}
    write_json(out / "summary.json", summary)
    print(f"[loop] summary: {out / 'summary.json'}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
