"""Shared pieces for the managed-agent runs: keys, client, polling, transcripts.

Secrets are read from the environment or the repo .env, passed only to the SDK,
and never printed, logged, or put into a prompt.
"""

from __future__ import annotations

import json
import os
import subprocess
import time
from pathlib import Path
from typing import Any, Callable

from google import genai

REPO_ROOT = Path(__file__).resolve().parents[3]
HARNESS = REPO_ROOT / "backend" / "harness"
PUBLIC_REPO = "EdanBarrios/theseus"
PUBLIC_REPO_URL = f"https://github.com/{PUBLIC_REPO}"

AGENT = "antigravity-preview-09-2026"
POLL_SECONDS = 15
TERMINAL = {"completed", "failed", "cancelled", "incomplete", "budget_exceeded", "requires_action"}
USAGE_FIELDS = (
    "total_input_tokens",
    "total_output_tokens",
    "total_thought_tokens",
    "total_cached_tokens",
    "total_tool_use_tokens",
    "total_tokens",
)

Event = Callable[[str], None]


def env_value(name: str) -> str | None:
    """Value of NAME from the process environment, else from the repo .env. Never logged."""
    val = os.environ.get(name)
    if val:
        return val
    env_file = REPO_ROOT / ".env"
    if env_file.is_file():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line.startswith("#") or "=" not in line:
                continue
            key, _, value = line.partition("=")
            if key.strip() == name:
                value = value.strip().strip("'\"")
                return value or None
    return None


_client: genai.Client | None = None


def get_client() -> genai.Client:
    global _client
    if _client is None:
        key = env_value("GEMINI_API_KEY")
        if not key:
            raise RuntimeError("GEMINI_API_KEY not found in the environment or the repo .env")
        _client = genai.Client(api_key=key)
    return _client


def usage_dict(interaction: Any) -> dict:
    usage = getattr(interaction, "usage", None)
    return {f: int(getattr(usage, f, None) or 0) for f in USAGE_FIELDS}


def cost_usd(usage: dict) -> float:
    """Estimate at $2/M input, $10/M output (thoughts billed as output), cached input at 5%."""
    cached = usage.get("total_cached_tokens", 0)
    fresh = max(0, usage.get("total_input_tokens", 0) - cached)
    out = usage.get("total_output_tokens", 0) + usage.get("total_thought_tokens", 0)
    return round((fresh * 2 + cached * 0.1 + out * 10) / 1e6, 4)


def poll(
    client: genai.Client,
    interaction: Any,
    *,
    max_wait: float,
    started: float,
    on_event: Event | None = None,
    tag: str = "",
) -> Any:
    """Poll interactions.get until a terminal status or max_wait seconds since `started`."""
    last = getattr(interaction, "status", None)
    while getattr(interaction, "status", None) not in TERMINAL:
        if time.monotonic() - started > max_wait:
            emit(on_event, f"{tag}timeout after {max_wait:.0f}s, last status {interaction.status}")
            break
        time.sleep(POLL_SECONDS)
        try:
            interaction = client.interactions.get(interaction.id)
        except Exception as exc:  # transient network errors: keep polling
            emit(on_event, f"{tag}poll error: {type(exc).__name__}: {exc}")
            continue
        if interaction.status != last:
            emit(on_event, f"{tag}status {interaction.status} at {time.monotonic() - started:.0f}s")
            last = interaction.status
    return interaction


class PreflightError(RuntimeError):
    """The verifier preflight gate failed: no match rate from this harness can be trusted."""


PREFLIGHT_LIBS = {"cjson": "cjson", "expat": "expat", "libexpat": "expat"}


def run_preflight(library: str, *, on_event: Event | None = None, extra_args: list[str] | None = None) -> dict:
    """Run scripts/preflight.sh for `library` and return its JSON summary.

    Raises PreflightError (with a clear message) if any check fails. Called
    before any model call; skip only with an explicit skip_preflight=True.
    """
    target = PREFLIGHT_LIBS.get(library)
    if target is None:
        raise PreflightError(f"no preflight gate for library {library!r}")
    emit(on_event, f"[preflight] {target}: canary, C vs C, reject-everything stub")
    p = subprocess.run([str(REPO_ROOT / "scripts/preflight.sh"), target, *(extra_args or [])],
                       capture_output=True, text=True, timeout=1800)
    line = (p.stdout.strip().splitlines() or [""])[-1]
    try:
        summary = json.loads(line)
    except json.JSONDecodeError:
        raise PreflightError(f"preflight {target} printed no JSON (exit {p.returncode}): {p.stderr[-800:]}") from None
    if p.returncode != 0 or not summary.get("ok"):
        why = summary.get("error") or ", ".join(summary.get("failed") or []) or f"exit {p.returncode}"
        raise PreflightError(
            f"verifier preflight FAILED for {target} ({why}); refusing to call a model. The verifier "
            f"cannot be trusted until scripts/preflight.sh {target} passes. Details: {summary.get('work_dir')}")
    emit(on_event, f"[preflight] {target} ok in {summary.get('seconds')}s")
    return summary


def emit(on_event: Event | None, msg: str) -> None:
    if on_event:
        try:
            on_event(msg)
        except Exception:
            pass


def _jsonable(obj: Any) -> Any:
    if hasattr(obj, "model_dump"):
        try:
            return obj.model_dump(mode="json", exclude_none=True)
        except Exception:
            pass
    return repr(obj)


def write_transcript(interaction: Any, path: Path) -> None:
    """One JSON line per step (thoughts, tool calls, tool results, model output)."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as fh:
        for i, step in enumerate(getattr(interaction, "steps", None) or []):
            fh.write(json.dumps({"index": i, "step": _jsonable(step)}, ensure_ascii=False) + "\n")


def step_types(interaction: Any) -> dict:
    counts: dict[str, int] = {}
    for step in getattr(interaction, "steps", None) or []:
        t = getattr(step, "type", "?")
        counts[t] = counts.get(t, 0) + 1
    return counts
