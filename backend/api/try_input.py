"""POST /try: run one user input through the C driver(s) and the current Rust binary.

Uses backend/harness/verify.py's run_one / classify / crashed directly, so a
verdict here means exactly what it means in a full verify run. Children get
LC_ALL=C (verify.child_env) and a 20 s timeout (macOS ASan symbolization of a
crash takes about 5 s).
"""

from __future__ import annotations

import concurrent.futures
import os
from pathlib import Path
from typing import Any

from . import harness

MAX_INPUT_BYTES = 64 * 1024
TIMEOUT_SECONDS = 20.0

# Set by RealPipeline when a live run finishes at 100%, so /try uses the binary
# the audience just watched get verified. THESEUS_RUST_BIN always wins.
_live_best: dict[str, str] = {}


def set_live_best(binary: str, label: str) -> None:
    _live_best.update(binary=binary, label=label)


def rust_binary() -> tuple[Path, str]:
    """(binary, label) for the Rust side. Raises HarnessError if none is available."""
    env = os.environ.get("THESEUS_RUST_BIN")
    if env:
        if not os.access(env, os.X_OK):
            raise harness.HarnessError(f"THESEUS_RUST_BIN={env} is not an executable file")
        return Path(env), "THESEUS_RUST_BIN"
    live = _live_best.get("binary")
    if live and os.access(live, os.X_OK):
        return Path(live), _live_best["label"]
    built = harness.candidate_binary("argon-1")
    if built is None:
        raise harness.HarnessError("no Rust binary: runs/ai/argon-1 has no buildable cjson.rs; set THESEUS_RUST_BIN")
    return built, "Argon's translation (runs/ai/argon-1)"


def known_cve(data: bytes) -> str | None:
    if not harness.OLD_CVE_INPUTS.is_dir():
        return None
    for path in sorted(harness.OLD_CVE_INPUTS.iterdir()):
        if path.is_file() and path.read_bytes() == data:
            return harness.KNOWN_CVES.get(path.stem, path.stem)
    return None


def _printable(data: bytes) -> bool:
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        return False
    return all(ch >= " " or ch in "\t\n" for ch in text) and "\x7f" not in text


def _sanitizer_summary(stderr: bytes) -> str | None:
    lines = stderr.decode("utf-8", errors="backslashreplace").splitlines()
    for prefix in ("SUMMARY: AddressSanitizer", "SUMMARY: UndefinedBehaviorSanitizer", "SUMMARY:"):
        for line in lines:
            if line.strip().startswith(prefix):
                return line.strip()
    for marker in ("ERROR: AddressSanitizer", "runtime error:", "panicked at", "has overflowed its stack"):
        for line in lines:
            if marker in line:
                return line.strip()
    return None


def side_view(res: dict[str, Any], side: str, label: str, version: str, binary: Path) -> dict[str, Any]:
    v = harness.verify_module()
    out: bytes = res["stdout"]
    return {
        "label": label,
        "version": version,
        "binary": str(binary),
        "stdout": out.decode("utf-8", errors="backslashreplace"),
        "stdout_hex": None if _printable(out) else out.hex(" "),
        "exit_code": res["exit_code"],
        "signal": res["signal"],
        "timed_out": res["timed_out"],
        "crashed": v.crashed(res, side),
        "exit": v.describe_exit(res),
        "sanitizer_summary": _sanitizer_summary(res["stderr"]),
        "stderr_head": v.first_lines(res["stderr"]) or None,
        "seconds": res["seconds"],
    }


def first_difference(c_out: bytes, r_out: bytes) -> dict[str, Any] | None:
    """Where C and Rust stdout first differ (byte offset, 1-based line), None if identical."""
    if c_out == r_out:
        return None
    i = next((k for k, (a, b) in enumerate(zip(c_out, r_out)) if a != b), min(len(c_out), len(r_out)))
    return {"offset": i, "line": c_out[:i].count(b"\n") + 1, "c_bytes": len(c_out), "rust_bytes": len(r_out)}


def run_expat(data: bytes) -> dict[str, Any]:
    """libexpat 2.9.0 C driver (ASan+UBSan) vs Argon's Rust port. No old-C / CVE side."""
    v = harness.verify_module()
    c_bin = harness.ensure_expat_c_driver()
    r_bin, r_label = harness.expat_rust_binary()
    env = v.child_env()
    jobs = {"c": [str(c_bin)], "rust": [str(r_bin)]}
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
        futures = {k: pool.submit(v.run_one, argv, data, TIMEOUT_SECONDS, env) for k, argv in jobs.items()}
        raw = {k: f.result() for k, f in futures.items()}
    return {
        "library": "expat",
        "input_bytes": len(data),
        "input": data.decode("utf-8", errors="backslashreplace"),
        "input_hex": None if _printable(data) else data.hex(" "),
        "cve": None,
        "c": side_view(raw["c"], "c", "C (libexpat, ASan+UBSan)", harness.EXPAT_C_VERSION, c_bin),
        "rust": side_view(raw["rust"], "rust", r_label, "safe Rust", r_bin),
        "old_c": None,
        "verdict": v.classify(raw["c"], raw["rust"]),
        "old_verdict": None,
        "first_difference": first_difference(raw["c"]["stdout"], raw["rust"]["stdout"]),
    }


def run_try(data: bytes, compare_old: bool = False, rust: tuple[Path, str] | None = None,
            library: str = "cjson") -> dict[str, Any]:
    if len(data) > MAX_INPUT_BYTES:
        raise ValueError(f"input is {len(data)} bytes; the limit is {MAX_INPUT_BYTES}")
    if library == "expat":
        return run_expat(data)
    v = harness.verify_module()
    c_bin = harness.ensure_c_driver()
    r_bin, r_label = rust or rust_binary()
    cve = known_cve(data)
    use_old = (compare_old or cve is not None) and os.access(harness.OLD_C_DRIVER, os.X_OK)
    env = v.child_env()  # LC_ALL=C, ASan/UBSan options

    jobs = {"c": [str(c_bin)], "rust": [str(r_bin)]}
    if use_old:
        jobs["old_c"] = [str(harness.OLD_C_DRIVER)]
    with concurrent.futures.ThreadPoolExecutor(max_workers=len(jobs)) as pool:
        futures = {k: pool.submit(v.run_one, argv, data, TIMEOUT_SECONDS, env) for k, argv in jobs.items()}
        raw = {k: f.result() for k, f in futures.items()}

    result: dict[str, Any] = {
        "library": "cjson",
        "input_bytes": len(data),
        "input": data.decode("utf-8", errors="backslashreplace"),
        "input_hex": None if _printable(data) else data.hex(" "),
        "cve": cve,
        "c": side_view(raw["c"], "c", "C (current)", harness.C_VERSION, c_bin),
        "rust": side_view(raw["rust"], "rust", r_label, "safe Rust", r_bin),
        "old_c": None,
        "verdict": v.classify(raw["c"], raw["rust"]),
        "old_verdict": None,
        "first_difference": first_difference(raw["c"]["stdout"], raw["rust"]["stdout"]),
    }
    if use_old:
        result["old_c"] = side_view(raw["old_c"], "c", "C (old, vulnerable)", harness.OLD_C_VERSION,
                                    harness.OLD_C_DRIVER)
        result["old_verdict"] = v.classify(raw["old_c"], raw["rust"])
    elif compare_old or cve:
        result["old_c_missing"] = f"{harness.OLD_C_DRIVER} is not built (bash backend/harness/oldcve/build_old.sh)"
    return result
