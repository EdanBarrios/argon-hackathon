"""Differential verifier: run every input through a C side and a Rust side.

Stdlib only. Run with `python3 -I verify.py ...` (verify.sh does this).

Each input file is fed on stdin to both commands. We capture stdout bytes,
stderr, exit code, signal and timeout separately, then classify the input:

  match               both clean, identical stdout bytes and exit code
  divergence          both clean, stdout bytes (or exit code) differ
  safety_improvement  C crashed or a sanitizer fired, Rust exited cleanly
                      (an error message counts as clean). Not a mismatch.
  rust_crash          Rust crashed (signal, sanitizer or panic), C was clean
  both_crash          both crashed
  timeout             either side exceeded --timeout before any sanitizer
                      report (a side that printed a report and then ran
                      out of time, e.g. while symbolizing, counts as
                      crashed, flagged timed_out_after_report)

stdout is compared as raw bytes. It is never decoded or normalized through a
JSON library, because formatting differences ARE the behavior under test.
"""

import argparse
import concurrent.futures
import hashlib
import json
import os
import platform
import shlex
import shutil
import signal
import subprocess
import sys
import time

REPORT_TEXT_LIMIT = 300      # chars of input/output kept per report entry
SANITIZER_REPORT_LINES = 15  # lines of sanitizer/crash output kept
MAX_LISTED = 200             # max entries per list in the report
CJSON_GIT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "vendor", "cJSON", ".git")

SANITIZER_MARKERS = (
    b"ERROR: AddressSanitizer",
    b"ERROR: LeakSanitizer",
    b"SUMMARY: AddressSanitizer",
    b"SUMMARY: UndefinedBehaviorSanitizer",
    b"runtime error:",  # UBSan diagnostic line
)
RUST_PANIC_MARKERS = (b"panicked at", b"thread 'main' has overflowed its stack")
RUST_PANIC_EXIT = 101


def child_env():
    env = dict(os.environ)
    env["LC_ALL"] = "C"
    # Make UBSan findings fatal and visible; keep any options the caller set.
    env.setdefault("UBSAN_OPTIONS", "print_stacktrace=1:halt_on_error=1")
    # Leak detection is unsupported on macOS clang and is not a behavior we
    # compare; an ASan error must still abort with a nonzero exit.
    env.setdefault("ASAN_OPTIONS", "detect_leaks=0:abort_on_error=0")
    env.setdefault("RUST_BACKTRACE", "0")
    return env


def run_one(argv, data, timeout, env):
    """Run argv with data on stdin. Never raises for child failures."""
    started = time.monotonic()
    try:
        proc = subprocess.Popen(
            argv,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=env,
            start_new_session=True,  # so a timeout can kill the whole group
        )
    except OSError as exc:
        return {
            "stdout": b"", "stderr": ("spawn failed: %s" % exc).encode(),
            "exit_code": None, "signal": None, "timed_out": False,
            "spawn_error": True, "seconds": 0.0,
        }
    timed_out = False
    try:
        out, err = proc.communicate(data, timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except OSError:
            proc.kill()
        out, err = proc.communicate()
    rc = proc.returncode
    return {
        "stdout": out,
        "stderr": err,
        "exit_code": rc if rc is not None and rc >= 0 else None,
        "signal": -rc if rc is not None and rc < 0 else None,
        "timed_out": timed_out,
        "spawn_error": False,
        "seconds": round(time.monotonic() - started, 4),
    }


def crashed(res, side):
    """True if this side crashed rather than exiting cleanly."""
    if res["spawn_error"] or res["signal"] is not None:
        return True
    err = res["stderr"]
    if any(m in err for m in SANITIZER_MARKERS):
        return True
    if side == "rust" and (res["exit_code"] == RUST_PANIC_EXIT
                           or any(m in err for m in RUST_PANIC_MARKERS)):
        return True
    return False


def reported_then_timed_out(res):
    """Timed out, but only after a sanitizer report was already on stderr.

    On macOS, symbolizing an ASan report (atos) can take longer than
    --timeout, so a real crash would otherwise be classified as a timeout.
    The report is the verdict; the slow symbolizer is not.
    """
    return res["timed_out"] and any(m in res["stderr"] for m in SANITIZER_MARKERS)


def classify(c, r):
    if any(x["timed_out"] and not reported_then_timed_out(x) for x in (c, r)):
        return "timeout"
    c_bad, r_bad = crashed(c, "c"), crashed(r, "rust")
    if c_bad and r_bad:
        return "both_crash"
    if c_bad:
        return "safety_improvement"
    if r_bad:
        return "rust_crash"
    if c["stdout"] == r["stdout"] and c["exit_code"] == r["exit_code"]:
        return "match"
    return "divergence"


def show(data, limit=REPORT_TEXT_LIMIT):
    """Readable, truncated view of raw bytes for the report only."""
    text = data.decode("utf-8", errors="backslashreplace")
    if len(text) > limit:
        return "%s... [truncated, %d bytes total]" % (text[:limit], len(data))
    return text


def show_output(res):
    if res["timed_out"]:
        return "TIMEOUT"
    out = res["stdout"]
    if out.endswith(b"\n"):
        out = out[:-1]  # driver protocol is one line; keep the report tidy
    return show(out)


def first_lines(data, n=SANITIZER_REPORT_LINES):
    lines = data.decode("utf-8", errors="backslashreplace").splitlines()
    keep = [ln for ln in lines if ln.strip()][:n]
    return "\n".join(keep)


def describe_exit(res):
    if res["timed_out"]:
        return "timeout"
    if res["signal"] is not None:
        return "killed by signal %d" % res["signal"]
    return "exit %s" % res["exit_code"]


def crash_report(res):
    report = first_lines(res["stderr"])
    if not report:
        report = describe_exit(res)
    return report


def behavior_summary(res):
    """One-line description of what a side did (used for rust_behavior)."""
    if crashed(res, "rust"):
        detail = first_lines(res["stderr"], 1)
        return "crash (%s)%s" % (describe_exit(res), (": " + detail) if detail else "")
    return "%s, stdout: %s" % (describe_exit(res), show_output(res))


def list_inputs(directory, max_bytes):
    files, skipped = [], []
    for name in sorted(os.listdir(directory)):
        path = os.path.join(directory, name)
        if not os.path.isfile(path) or name.startswith("."):
            continue
        if os.path.getsize(path) > max_bytes:
            skipped.append(name)
            continue
        files.append(path)
    return files, skipped


def executable_info(argv):
    """Resolved path and sha256 of the program a side runs (argv[0])."""
    path = shutil.which(argv[0])
    if path is None:
        return {"executable": argv[0], "sha256": None}
    digest = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            digest.update(chunk)
    return {"executable": os.path.realpath(path), "sha256": digest.hexdigest()}


def cjson_commit():
    """Commit of the vendored cJSON checkout, read from .git without git."""
    try:
        with open(os.path.join(CJSON_GIT, "HEAD"), encoding="ascii") as fh:
            head = fh.read().strip()
        if head.startswith("ref: "):
            with open(os.path.join(CJSON_GIT, head[5:]), encoding="ascii") as fh:
                head = fh.read().strip()
    except OSError:
        return None
    return head


def provenance(c_argv, r_argv):
    return {
        "c": executable_info(c_argv),
        "rust": executable_info(r_argv),
        "platform": platform.platform(),
        "cjson_commit": cjson_commit(),
    }


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--c", required=True, help="C side command (shell-quoted string)")
    ap.add_argument("--rust", required=True, help="Rust side command (shell-quoted string)")
    ap.add_argument("--inputs", required=True, help="directory of input files")
    ap.add_argument("--timeout", type=float, default=5.0, help="seconds per run")
    ap.add_argument("--max-bytes", type=int, default=1048576,
                    help="skip input files larger than this")
    ap.add_argument("--out", help="also write the JSON report here")
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    args = ap.parse_args(argv)

    if not os.path.isdir(args.inputs):
        ap.error("--inputs is not a directory: %s" % args.inputs)
    c_argv, r_argv = shlex.split(args.c), shlex.split(args.rust)
    if not c_argv or not r_argv:
        ap.error("--c and --rust must be non-empty commands")

    env = child_env()
    files, skipped = list_inputs(args.inputs, args.max_bytes)

    def work(path):
        with open(path, "rb") as fh:
            data = fh.read()
        c = run_one(c_argv, data, args.timeout, env)
        r = run_one(r_argv, data, args.timeout, env)
        return path, data, c, r, classify(c, r)

    started = time.monotonic()
    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, args.jobs)) as pool:
        results = list(pool.map(work, files))  # map keeps sorted input order

    counts = dict.fromkeys(
        ["match", "divergence", "safety_improvement", "rust_crash", "both_crash", "timeout"], 0)
    divergences, crashes, timeouts = [], [], []
    for path, data, c, r, cls in results:
        counts[cls] += 1
        name = os.path.basename(path)
        if cls == "divergence":
            divergences.append({
                "input": show(data), "file": name,
                "c_output": show_output(c), "rust_output": show_output(r),
                "c_exit": c["exit_code"], "rust_exit": r["exit_code"],
                "function": None,
            })
        elif cls in ("safety_improvement", "rust_crash", "both_crash"):
            side = {"safety_improvement": "c", "rust_crash": "rust", "both_crash": "both"}[cls]
            report_src = r if side == "rust" else c
            crashes.append({
                "input": show(data), "file": name, "class": cls, "side": side,
                "sanitizer_report": crash_report(report_src),
                "rust_behavior": behavior_summary(r),
                "timed_out_after_report": c["timed_out"] or r["timed_out"],
            })
        elif cls == "timeout":
            timeouts.append({"input": show(data), "file": name,
                             "c": describe_exit(c), "rust": describe_exit(r)})

    # match_rate = match / (match + divergence + rust_crash + both_crash + timeout)
    # safety_improvement is excluded from the denominator: C crashing where
    # Rust exits cleanly is an intended improvement, not a behavior mismatch.
    denom = (counts["match"] + counts["divergence"] + counts["rust_crash"]
             + counts["both_crash"] + counts["timeout"])
    match_rate = (counts["match"] / denom) if denom else None

    report = {
        "match_rate": match_rate,
        "behaviors_tested": len(results),
        "counts": counts,
        "divergences": divergences[:MAX_LISTED],
        "crashes": crashes[:MAX_LISTED],
        "timeouts": timeouts[:MAX_LISTED],
        "truncated_lists": any(len(x) > MAX_LISTED for x in (divergences, crashes, timeouts)),
        "skipped_inputs": skipped,
        "config": {
            "c": args.c, "rust": args.rust, "inputs": os.path.abspath(args.inputs),
            "timeout": args.timeout, "max_bytes": args.max_bytes,
        },
        "provenance": provenance(c_argv, r_argv),
        "elapsed_seconds": round(time.monotonic() - started, 2),
    }
    text = json.dumps(report, indent=2, ensure_ascii=False)
    sys.stdout.write(text + "\n")
    if args.out:
        parent = os.path.dirname(os.path.abspath(args.out))
        os.makedirs(parent, exist_ok=True)
        with open(args.out, "w", encoding="utf-8") as fh:
            fh.write(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
