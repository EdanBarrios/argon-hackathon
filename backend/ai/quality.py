"""Tool-only code quality for one generated Rust file: rustfmt, clippy, panic-shaped lints.

Every number comes from rustfmt / cargo clippy output, never from model text. Each
call copies the target's crate template (Cargo.toml, Cargo.lock, src/main.rs) into a
temp dir with the generated file as src/<rust_file>, so the harness build scripts
stay untouched. Only diagnostics whose primary span is in the generated file count;
main.rs is the harness's.

Counts:
- clippy_warnings: every warning under `cargo clippy` default lints (clippy's and
  rustc's, as `-D warnings` would fail on), excluding the restriction lints below.
- panic_lints: indexing_slicing, unwrap_used, expect_used, panic. These are sites that
  can panic; the gate for "no panics" is this at zero plus no Rust crash at runtime.
- arithmetic_side_effects: reported only. Most hits are bounded `i += 1` steps, and
  forcing them to zero pushes the code toward wrapping ops that can change behavior.
"""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

from .targets import Target

PANIC_LINTS = ("clippy::indexing_slicing", "clippy::unwrap_used", "clippy::expect_used", "clippy::panic")
ARITH_LINT = "clippy::arithmetic_side_effects"
RESTRICTION_LINTS = (*PANIC_LINTS, ARITH_LINT)
# Most examples per lint shown in a polish prompt, and most lints shown.
EXAMPLES_PER_LINT = 3
MAX_LINTS_SHOWN = 15
CARGO_TIMEOUT = 600


def _crate(root: Path, target: Target, source: str, tmp: Path) -> Path:
    template = root / target.crate_template
    crate = tmp / "crate"
    (crate / "src").mkdir(parents=True)
    for name in ("Cargo.toml", "Cargo.lock"):
        if (template / name).is_file():
            shutil.copy(template / name, crate / name)
    shutil.copy(template / "src/main.rs", crate / "src/main.rs")
    (crate / "src" / target.rust_file).write_text(source, encoding="utf-8")
    return crate


def _cargo(crate: Path, *args: str) -> subprocess.CompletedProcess:
    return subprocess.run(["cargo", *args], cwd=crate, capture_output=True, text=True, timeout=CARGO_TIMEOUT)


def _rustfmt(crate: Path, rust_file: str, *args: str) -> subprocess.CompletedProcess:
    """rustfmt on the generated file only: `cargo fmt` would also judge the harness's main.rs."""
    m = re.search(r'^edition\s*=\s*"(\d+)"', (crate / "Cargo.toml").read_text(), re.MULTILINE)
    cmd = ["rustfmt", "--edition", m.group(1) if m else "2021", *args, f"src/{rust_file}"]
    return subprocess.run(cmd, cwd=crate, capture_output=True, text=True, timeout=CARGO_TIMEOUT)


def parse_clippy(stdout: str, rust_file: str) -> list[dict]:
    """Warnings and errors from `cargo clippy --message-format=json` that point into rust_file."""
    out = []
    for line in stdout.splitlines():
        if not line.startswith("{"):
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError:
            continue
        msg = rec.get("message") if rec.get("reason") == "compiler-message" else None
        if not msg or msg.get("level") not in ("warning", "error"):
            continue
        span = next((s for s in msg.get("spans") or [] if s.get("is_primary")), None)
        if not span or Path(span.get("file_name") or "").name != rust_file:
            continue
        out.append({
            "level": msg["level"],
            "lint": (msg.get("code") or {}).get("code") or "(no code)",
            "line": span.get("line_start"),
            "message": msg.get("message"),
            "rendered": msg.get("rendered") or "",
        })
    return out


def summarize(diags: list[dict]) -> dict:
    by_lint: dict[str, int] = {}
    for d in diags:
        by_lint[d["lint"]] = by_lint.get(d["lint"], 0) + 1
    return {
        "clippy_errors": sum(1 for d in diags if d["level"] == "error"),
        "clippy_warnings": sum(1 for d in diags if d["level"] == "warning" and d["lint"] not in RESTRICTION_LINTS),
        "panic_lints": sum(n for k, n in by_lint.items() if k in PANIC_LINTS),
        "arithmetic_side_effects": by_lint.get(ARITH_LINT, 0),
        "by_lint": dict(sorted(by_lint.items(), key=lambda kv: -kv[1])),
    }


def measure(root: Path, target: Target, source: str) -> dict:
    """fmt and clippy numbers for one file, plus the diagnostics a polish prompt needs."""
    with tempfile.TemporaryDirectory(prefix="theseus-quality-") as tmp:
        crate = _crate(root, target, source, Path(tmp))
        fmt = _rustfmt(crate, target.rust_file, "--check")
        args = ["clippy", "--offline", "--message-format=json", "--"]
        for lint in RESTRICTION_LINTS:
            args += ["-W", lint]
        clippy = _cargo(crate, *args)
    diags = parse_clippy(clippy.stdout, target.rust_file)
    q = {"fmt_ok": fmt.returncode == 0, "fmt_diff_lines": len(fmt.stdout.splitlines()), **summarize(diags)}
    if clippy.returncode != 0 and not q["clippy_errors"]:
        q["clippy_failed"] = clippy.stderr[-2000:]
    return {**q, "diagnostics": diags}


def public(q: dict | None) -> dict | None:
    """measure() without the per-diagnostic list, for meta.json and summary.json."""
    return None if q is None else {k: v for k, v in q.items() if k != "diagnostics"}


def clean(q: dict | None) -> bool:
    """The polish target: fmt, default clippy and panic lints all at zero."""
    return bool(q) and q["fmt_ok"] and not q["clippy_errors"] and not q["clippy_warnings"] and not q["panic_lints"]


def rank(q: dict | None) -> tuple:
    """Higher is better. Behavior is ranked before this, so it only breaks ties."""
    if not q:
        return (float("-inf"),) * 3
    return (-q["clippy_warnings"] - q["clippy_errors"], -q["panic_lints"], -q["arithmetic_side_effects"])


def autofix(root: Path, target: Target, source: str) -> tuple[str, dict]:
    """Host-side fixes that need no model: `cargo clippy --fix` (default lints, machine-applicable
    suggestions only), then rustfmt. Returns (new source, what ran).

    The caller must rebuild and reverify the result; nothing here is trusted to keep behavior.
    """
    with tempfile.TemporaryDirectory(prefix="theseus-autofix-") as tmp:
        crate = _crate(root, target, source, Path(tmp))
        fix = _cargo(crate, "clippy", "--offline", "--fix", "--allow-no-vcs", "--allow-dirty")
        fmt = _rustfmt(crate, target.rust_file)
        fixed = (crate / "src" / target.rust_file).read_text(encoding="utf-8")
    info = {"clippy_fix_ok": fix.returncode == 0, "fmt_ok": fmt.returncode == 0, "changed": fixed != source}
    if fix.returncode != 0:
        info["clippy_fix_stderr"] = fix.stderr[-2000:]
    return fixed, info


def format_lints(q: dict) -> str:
    """Prompt text for a polish round: default-clippy and panic lints, grouped by lint, a few
    examples each. arithmetic_side_effects is left out on purpose (see module doc)."""
    groups: dict[str, list[dict]] = {}
    for d in q.get("diagnostics") or []:
        if d["lint"] == ARITH_LINT:
            continue
        groups.setdefault(d["lint"], []).append(d)
    ranked = sorted(groups.items(), key=lambda kv: -len(kv[1]))[:MAX_LINTS_SHOWN]
    lines = [
        "## Lint summary\n" + json.dumps({k: v for k, v in public(q).items() if k != "by_lint"}, indent=2)
        + f"\nShowing {len(ranked)} of {len(groups)} lints, at most {EXAMPLES_PER_LINT} examples each."
    ]
    for lint, items in ranked:
        lines.append(f"## {lint} ({len(items)} sites)")
        for d in sorted(items, key=lambda d: d["line"] or 0)[:EXAMPLES_PER_LINT]:
            lines.append("```text\n" + d["rendered"].rstrip()[:1500] + "\n```")
    return "\n\n".join(lines)
