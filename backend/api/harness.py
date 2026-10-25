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
        subprocess.run([str(HARNESS / "build_cjson.sh")], check=True, capture_output=True)
    return C_DRIVER


_inputs_lock = threading.Lock()


def ensure_inputs(count: int, seed: int = 1) -> Path:
    """Generated inputs, created once and cached under runs/ai/ (same layout as ai.loop)."""
    out = RUNS_AI / f"inputs-c{count}-s{seed}"
    with _inputs_lock:
        if out.is_dir() and any(out.iterdir()):
            return out
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
