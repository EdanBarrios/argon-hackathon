"""Thin glue between the API and the verification harness.

Paths, candidate cjson.rs lookup, build_rust.sh / verify.sh calls, and the
verify.py module itself (imported by file path, never edited). Nothing here
grades anything: every number comes from the harness scripts.
"""

from __future__ import annotations

import importlib.util
import json
import os
import re
import subprocess
import shutil
import sys
import threading
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Any

ROOT = Path(os.environ.get("THESEUS_ROOT") or Path(__file__).resolve().parents[2]).resolve()
HARNESS = ROOT / "backend" / "harness"
BUILD_RUST = ROOT / "scripts" / "build_rust.sh"
VERIFY_SH = HARNESS / "verify.sh"
VERIFY_PY = HARNESS / "verify.py"
C_DRIVER = HARNESS / "build" / "cjson_driver"
OLD_C_DRIVER = HARNESS / "oldcve" / "build" / "cjson_driver_old"
OLD_CVE_INPUTS = HARNESS / "oldcve" / "inputs"
VENDOR = HARNESS / "vendor" / "cJSON"
RUNS_AI = ROOT / "runs" / "ai"
GEN_INPUTS = HARNESS / "gen_inputs.py"

# Known CVE inputs (file stem -> CVE id). The old driver is cJSON v1.7.17.
KNOWN_CVES = {"cve_2023_53154": "CVE-2023-53154"}
C_VERSION = "cJSON v1.7.19"
OLD_C_VERSION = "cJSON v1.7.17"


# backend/ai is imported as the top-level package `ai`; make sure backend/ is on the path.
_BACKEND = str(Path(__file__).resolve().parents[1])
if _BACKEND not in sys.path:
    sys.path.insert(0, _BACKEND)


class HarnessError(RuntimeError):
    pass


# ---------------------------------------------------------------- verify.py


_verify_mod: ModuleType | None = None
_verify_lock = threading.Lock()


def verify_module() -> ModuleType:
    """backend/harness/verify.py loaded by path (it is a script, not a package)."""
    global _verify_mod
    with _verify_lock:
        if _verify_mod is None:
            spec = importlib.util.spec_from_file_location("theseus_harness_verify", VERIFY_PY)
            if spec is None or spec.loader is None:
                raise HarnessError(f"cannot load {VERIFY_PY}")
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)
            _verify_mod = mod
        return _verify_mod


# ---------------------------------------------------------------- candidates


@dataclass
class Candidate:
    name: str          # "argon-1", "flash-2", or "path"
    label: str         # human text for the UI
    model: str | None
    rust_path: Path
    interaction_id: str | None
    prior: dict[str, Any]  # what meta.json recorded about this candidate at the time


CANDIDATE_LABELS = {
    "argon-1": "Argon's first attempt",
    "argon-2": "Argon's second independent attempt",
    "flash-2": "Flash's first attempt",
}


def _read_json(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError):
        return {}


def load_candidate(start_from: str) -> Candidate:
    """Resolve start_from (a known run name or a path to a cjson.rs) to a candidate."""
    if start_from in CANDIDATE_LABELS:
        rdir = RUNS_AI / start_from / "round_1"
        rust = rdir / "cjson.rs"
        if not rust.is_file():
            raise HarnessError(f"no candidate for start_from={start_from!r}: {rust} is missing")
        meta = _read_json(rdir / "meta.json")
        summary = _read_json(RUNS_AI / start_from / "summary.json")
        return Candidate(
            name=start_from,
            label=CANDIDATE_LABELS[start_from],
            model=summary.get("model"),
            rust_path=rust,
            interaction_id=meta.get("interaction_id"),
            prior={k: meta.get(k) for k in ("match_rate", "behaviors_tested", "seconds", "thinking_level")},
        )
    path = Path(start_from).expanduser()
    if not path.is_absolute():
        path = ROOT / path
    if path.suffix != ".rs" or not path.is_file():
        raise HarnessError(
            f"start_from={start_from!r} is not fresh, a known run ({', '.join(CANDIDATE_LABELS)}) or a .rs file"
        )
    meta = _read_json(path.parent / "meta.json")
    return Candidate(
        name="path",
        label=f"the candidate at {path}",
        model=None,
        rust_path=path.resolve(),
        interaction_id=meta.get("interaction_id"),
        prior={},
    )


def candidate_binary(name: str, build: bool = True) -> Path | None:
    """The built binary for a known run, rebuilding it from cjson.rs if it is missing."""
    rdir = RUNS_AI / name / "round_1"
    binary = _read_json(rdir / "build.json").get("binary")
    if binary and os.access(binary, os.X_OK):
        return Path(binary)
    if not build or not (rdir / "cjson.rs").is_file():
        return None
    res = build_rust(rdir / "cjson.rs", f"cand-{name}")
    return Path(res["binary"]) if res.get("ok") else None


# ---------------------------------------------------------------- scripts


def run_json(cmd: list[str], timeout: float) -> tuple[dict[str, Any] | None, str]:
    """Run a harness script that prints JSON on stdout. Returns (parsed or None, error text)."""
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return None, f"{Path(cmd[0]).name} timed out after {timeout}s"
    out = proc.stdout.strip()
    try:
        return json.loads(out), proc.stderr
    except json.JSONDecodeError:
        return None, (proc.stderr + "\n" + out)[-4000:]


def build_rust(rust_path: Path, build_id: str) -> dict[str, Any]:
    build_id = re.sub(r"[^A-Za-z0-9._-]", "_", build_id)
    res, raw = run_json([str(BUILD_RUST), str(rust_path), build_id], timeout=600)
    if res is None:
        return {"ok": False, "errors": f"build_rust.sh produced no JSON: {raw}"}
    return res


def verify(rust_binary: Path, inputs_dir: Path, timeout: float = 20.0) -> dict[str, Any]:
    cmd = [
        str(VERIFY_SH), "--c", str(C_DRIVER), "--rust", str(rust_binary),
        "--inputs", str(inputs_dir), "--timeout", str(timeout), "--jobs", str(os.cpu_count() or 4),
    ]
    res, raw = run_json(cmd, timeout=3600)
    if res is None:
        raise HarnessError(f"verify.sh produced no JSON: {raw[-1000:]}")
    return res


def ensure_c_driver() -> Path:
    if not os.access(C_DRIVER, os.X_OK):
        # build_cjson.sh needs the vendored source, which a fresh clone lacks
        subprocess.run([str(HARNESS / "fetch_cjson.sh")], check=True, capture_output=True)
        subprocess.run([str(HARNESS / "build_cjson.sh")], check=True, capture_output=True)
    return C_DRIVER


_inputs_lock = threading.Lock()


def ensure_inputs(count: int, seed: int = 1) -> Path:
    """Generated inputs, created once and cached under runs/ai/ (same layout as ai.loop)."""
    out = RUNS_AI / f"inputs-c{count}-s{seed}"
    with _inputs_lock:
        if out.is_dir() and any(out.iterdir()):
            return out
        subprocess.run([str(HARNESS / "fetch_cjson.sh")], check=True, capture_output=True)
        subprocess.run(
            [sys.executable, "-I", str(GEN_INPUTS), "--seed", str(seed), "--count", str(count),
             "--out", str(out), "--cjson", str(VENDOR)],
            check=True, capture_output=True,
        )
    return out


ADVERSARIAL = HARNESS / "adversarial" / "corpus"
_SKIP = ("README", "REASONS.md", ".")


def corpus_files(spec: str) -> list[Path]:
    """Files of an adversarial corpus. spec is "<corpus>" or "<corpus>:<name prefix>",
    e.g. "targeted" (980 inputs) or "targeted:str_" (the 278 string inputs)."""
    name, _, prefix = spec.partition(":")
    src = ADVERSARIAL / name
    if not re.fullmatch(r"[a-z0-9_-]+", name) or not src.is_dir():
        known = sorted(p.name for p in ADVERSARIAL.iterdir() if p.is_dir()) if ADVERSARIAL.is_dir() else []
        raise HarnessError(f"unknown extra_inputs corpus {name!r}; known: {known}")
    files = [p for p in sorted(src.iterdir())
             if p.is_file() and not p.name.startswith(_SKIP) and p.name.startswith(prefix)]
    if not files:
        raise HarnessError(f"extra_inputs {spec!r} matched no files")
    return files


def merged_inputs(base: Path, specs: list[str]) -> Path:
    """base inputs plus adversarial corpus files in one cached directory (verify.py reads one dir).
    Corpus files are prefixed adv-<corpus>- so names cannot collide (also on case-insensitive disks)."""
    tag = "+".join(re.sub(r"[^A-Za-z0-9_-]", "_", s) for s in specs)
    out = RUNS_AI / f"{base.name}+{tag}"
    with _inputs_lock:
        if out.is_dir() and any(out.iterdir()):
            return out
        tmp = out.with_name(out.name + ".tmp")
        shutil.rmtree(tmp, ignore_errors=True)
        tmp.mkdir(parents=True)
        for p in base.iterdir():
            if p.is_file() and not p.name.startswith("."):
                shutil.copyfile(p, tmp / p.name)
        for spec in specs:
            corpus = spec.partition(":")[0]
            for p in corpus_files(spec):
                shutil.copyfile(p, tmp / f"adv-{corpus}-{p.name}")
        tmp.rename(out)
    return out


def load_c_source() -> str:
    from ai import prompts  # noqa: PLC0415

    return prompts.load_cjson_source(VENDOR)


# ---------------------------------------------------------------- libexpat

EXPAT_HARNESS = HARNESS / "expat"
EXPAT_C_DRIVER = EXPAT_HARNESS / "build" / "expat_driver"
EXPAT_VENDOR_LIB = EXPAT_HARNESS / "vendor" / "libexpat" / "expat" / "lib"
BUILD_RUST_EXPAT = ROOT / "scripts" / "build_rust_expat.sh"
EXPAT_RESULTS = ROOT / "backend" / "ai" / "results" / "expat-long-decode"
EXPAT_REVERIFY = ROOT / "results" / "libexpat-host-reverify"
# Argon's port that passed 747/747, 1,253/1,253 held out and 2,000/2,000 on the host re-verify.
EXPAT_RUST_SRC = EXPAT_RESULTS / "argon-fresh-a" / "round_3" / "expat.rs"
EXPAT_RUST_BIN = ROOT / "runs" / "build-expat" / "edan-reverify-a" / "target" / "release" / "theseus_expat"
EXPAT_C_VERSION = "libexpat 2.9.0 (R_2_9_0)"

_expat_lock = threading.Lock()


def ensure_expat_c_driver() -> Path:
    with _expat_lock:
        if not os.access(EXPAT_C_DRIVER, os.X_OK):
            if not (EXPAT_VENDOR_LIB / "xmlparse.c").is_file():
                subprocess.run([str(EXPAT_HARNESS / "fetch_expat.sh")], capture_output=True, timeout=600)
            subprocess.run([str(EXPAT_HARNESS / "build_expat.sh")], capture_output=True, timeout=600)
        if not os.access(EXPAT_C_DRIVER, os.X_OK):
            raise HarnessError(f"{EXPAT_C_DRIVER} is not built (bash backend/harness/expat/build_expat.sh)")
    return EXPAT_C_DRIVER


def expat_rust_binary() -> tuple[Path, str]:
    """THESEUS_EXPAT_RUST_BIN, else the re-verified build, else build Argon's port once."""
    label = "Argon's libexpat port (argon-fresh-a round 3)"
    env = os.environ.get("THESEUS_EXPAT_RUST_BIN")
    if env:
        if not os.access(env, os.X_OK):
            raise HarnessError(f"THESEUS_EXPAT_RUST_BIN={env} is not an executable file")
        return Path(env), "THESEUS_EXPAT_RUST_BIN"
    with _expat_lock:
        if os.access(EXPAT_RUST_BIN, os.X_OK):
            return EXPAT_RUST_BIN, label
        res, raw = run_json([str(BUILD_RUST_EXPAT), str(EXPAT_RUST_SRC), "edan-reverify-a"], timeout=600)
    if res is None or not res.get("ok") or not os.access(res.get("binary", ""), os.X_OK):
        detail = (res or {}).get("errors") or raw
        raise HarnessError(f"cannot build Argon's libexpat port: {str(detail)[-1000:]}")
    return Path(res["binary"]), label
