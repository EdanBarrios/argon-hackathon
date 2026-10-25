"""Translation targets: one per C library the loop can migrate.

A target ties the prompt pieces (prompts.py) to the harness pieces C owns (vendor
sources, C driver, build script, input generator) for one library. cJSON is the
only one wired today. libexpat gets its own entry once its harness lands; it needs a
larger output cap and a longer wait, since its Rust is several times the 65,536
output tokens Flash stops at.

Paths are relative to the harness root (loop.find_root).
"""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Callable

from . import prompts


@dataclass(frozen=True)
class Target:
    name: str
    rust_file: str
    system_instruction: str
    load_source: Callable[[Path], str]  # vendor dir -> c_source for translate()
    translate_input: Callable[[str], str]
    repair_input: Callable[[str], str]
    vendor: str
    vendor_check: str  # a file under vendor that must exist
    c_driver: str
    build_c: str
    build_rust: str
    gen_inputs: Callable[[Path, int, int, Path], list[str]]  # (root, seed, count, out) -> argv
    inputs_prefix: str = "inputs"
    max_output_tokens: int = 131072
    max_wait_seconds: float = 45 * 60


def _cjson_gen(root: Path, seed: int, count: int, out: Path) -> list[str]:
    return [sys.executable, "-I", str(root / "backend/harness/gen_inputs.py"),
            "--seed", str(seed), "--count", str(count), "--out", str(out),
            "--cjson", str(root / "backend/harness/vendor/cJSON")]


CJSON = Target(
    name="cjson",
    rust_file="cjson.rs",
    system_instruction=prompts.SYSTEM_INSTRUCTION,
    load_source=prompts.load_cjson_source,
    translate_input=prompts.translate_input,
    repair_input=prompts.repair_input,
    vendor="backend/harness/vendor/cJSON",
    vendor_check="cJSON.c",
    c_driver="backend/harness/build/cjson_driver",
    build_c="backend/harness/build_cjson.sh",
    build_rust="scripts/build_rust.sh",
    gen_inputs=_cjson_gen,
)

TARGETS: dict[str, Target] = {CJSON.name: CJSON}
