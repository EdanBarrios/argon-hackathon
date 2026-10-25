"""Argon translate + repair through the Gemini Interactions API.

    translate(c_source, model="gemini-4-argon") -> TranslateResult
    repair(prev, feedback, model="gemini-4-argon") -> TranslateResult

Both run as background interactions and poll interactions.get every 5 s, so no
HTTP request is held open for the minutes a high-thinking call takes. repair
continues the conversation with previous_interaction_id and re-sends
system_instruction and generation_config, which are per interaction.

The model never grades its own output: it only ever sees what verify.sh and
build_rust.sh reported.
"""

from __future__ import annotations

import os
import re
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from google import genai

from . import prompts

DEFAULT_MODEL = "gemini-4-argon"
POLL_SECONDS = 5
# Wall-clock cap per interaction, so a stuck agent cannot hang a run forever.
MAX_WAIT_SECONDS = 45 * 60
# parse + print in Rust is ~20K output tokens; thinking also counts toward the cap.
# Above 65,536 requires background=True, which we always use.
MAX_OUTPUT_TOKENS = 131072
FALLBACK_MAX_OUTPUT_TOKENS = 65536
MAX_DIVERGENCES = 10
MAX_CRASHES = 10
USAGE_FIELDS = (
    "total_input_tokens",
    "total_output_tokens",
    "total_thought_tokens",
    "total_cached_tokens",
    "total_tokens",
)
# If a model rejects a thinking level, step down in this order.
THINKING_FALLBACK = {"high": ["medium", "low", None], "medium": ["low", None], "low": [None]}

_RUST_BLOCK = re.compile(r"```[ \t]*(?:rust|rs)[ \t]*\r?\n(.*?)\r?\n[ \t]*```", re.DOTALL | re.IGNORECASE)


class TranslateError(RuntimeError):
    """The model call failed or the reply had no usable rust block."""


@dataclass
class TranslateResult:
    rust_source: str
    interaction_id: str
    usage: dict
    seconds: float
    model: str
    # Extras with defaults, so the five fields above stay the contract.
    thinking_level: str | None = None
    notes: list[str] = field(default_factory=list)


# ---------------------------------------------------------------- client


def _load_key() -> str:
    """GEMINI_API_KEY from the environment, else the nearest .env walking up from here.

    Walking up finds the main checkout's .env from a nested git worktree, so the
    key never has to be copied. The key is never printed or logged.
    """
    key = os.environ.get("GEMINI_API_KEY")
    if key:
        return key
    for parent in Path(__file__).resolve().parents:
        env_file = parent / ".env"
        if not env_file.is_file():
            continue
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line.startswith("#") or "=" not in line:
                continue
            name, _, value = line.partition("=")
            if name.strip() == "GEMINI_API_KEY":
                value = value.strip().strip("'\"")
                if value:
                    return value
    raise TranslateError("GEMINI_API_KEY not found in the environment or any parent .env")


_client: genai.Client | None = None


def _get_client() -> genai.Client:
    global _client
    if _client is None:
        _client = genai.Client(api_key=_load_key())
    return _client


# ---------------------------------------------------------------- core call


def _usage_dict(interaction: Any) -> dict:
    usage = getattr(interaction, "usage", None)
    out = {name: int(getattr(usage, name, None) or 0) for name in USAGE_FIELDS}
    return out


def _is_param_error(exc: Exception, word: str) -> bool:
    text = str(exc).lower()
    return word in text and any(s in text for s in ("400", "invalid", "not supported", "unsupported"))


def _create(client: genai.Client, kwargs: dict, thinking: str | None, notes: list[str]):
    """interactions.create, stepping down thinking_level / max_output_tokens if the model rejects them."""
    levels = [thinking] + THINKING_FALLBACK.get(thinking or "", [])
    max_tokens = MAX_OUTPUT_TOKENS
    last: Exception | None = None
    for _ in range(8):
        cfg: dict = {"max_output_tokens": max_tokens}
        if levels[0] is not None:
            cfg["thinking_level"] = levels[0]
        try:
            return client.interactions.create(generation_config=cfg, **kwargs), levels[0]
        except Exception as exc:  # SDK raises several error classes; inspect the message.
            last = exc
            if _is_param_error(exc, "thinking") and len(levels) > 1:
                notes.append(f"{kwargs['model']} rejected thinking_level={levels[0]!r}; retried with {levels[1]!r}")
                levels = levels[1:]
                continue
            if _is_param_error(exc, "max_output_tokens") and max_tokens > FALLBACK_MAX_OUTPUT_TOKENS:
                notes.append(f"{kwargs['model']} rejected max_output_tokens={max_tokens}; retried with {FALLBACK_MAX_OUTPUT_TOKENS}")
                max_tokens = FALLBACK_MAX_OUTPUT_TOKENS
                continue
            raise TranslateError(f"interactions.create failed for {kwargs['model']}: {exc}") from exc
    raise TranslateError(f"interactions.create kept failing for {kwargs['model']}: {last}")


def _wait(client: genai.Client, interaction: Any, max_seconds: float = MAX_WAIT_SECONDS) -> Any:
    """Poll interactions.get every POLL_SECONDS until the interaction leaves queued/in_progress.

    Raises TranslateError after max_seconds of wall clock."""
    transient = 0
    deadline = time.monotonic() + max_seconds
    while getattr(interaction, "status", None) in ("queued", "in_progress", None):
        if time.monotonic() > deadline:
            raise TranslateError(
                f"interaction {interaction.id} still {getattr(interaction, 'status', None)!r} after {max_seconds:.0f}s"
            )
        time.sleep(POLL_SECONDS)
        try:
            interaction = client.interactions.get(interaction.id)
            transient = 0
        except Exception as exc:
            transient += 1
            if transient > 6:
                raise TranslateError(f"interactions.get({interaction.id}) failed repeatedly: {exc}") from exc
    return interaction


def extract_rust(text: str | None, notes: list[str] | None = None) -> str:
    """The single ```rust block from a reply. Raises TranslateError if there is none."""
    if not text:
        raise TranslateError("model reply was empty: no ```rust block to extract")
    blocks = _RUST_BLOCK.findall(text)
    if not blocks:
        if re.search(r"```[ \t]*(?:rust|rs)\b", text, re.IGNORECASE):
            raise TranslateError("model reply has an opening ```rust fence but no closing fence (truncated output?)")
        raise TranslateError("model reply contains no ```rust block")
    if len(blocks) > 1:
        if notes is not None:
            notes.append(f"reply had {len(blocks)} rust blocks; took the longest")
        return max(blocks, key=len).rstrip() + "\n"
    return blocks[0].rstrip() + "\n"


def _run(model: str, input_text: str, thinking: str, previous_id: str | None) -> TranslateResult:
    """One logical call. If the model runs out of output budget (status "incomplete"),
    retry once at the next lower thinking level. Observed: gemini-3.8-flash at "high"
    spent ~63K thought tokens and stopped at the 65,536 output cap mid-file, even
    with max_output_tokens=131072. Usage and seconds include the failed attempt."""
    client = _get_client()
    notes: list[str] = []
    kwargs: dict = {
        "model": model,
        "input": input_text,
        "system_instruction": prompts.SYSTEM_INSTRUCTION,
        "background": True,
    }
    if previous_id:
        kwargs["previous_interaction_id"] = previous_id
    started = time.monotonic()
    usage = dict.fromkeys(USAGE_FIELDS, 0)
    level: str | None = thinking
    for attempt in (1, 2):
        interaction, used_level = _create(client, kwargs, level, notes)
        interaction = _wait(client, interaction)
        status = getattr(interaction, "status", None)
        for k, v in _usage_dict(interaction).items():
            usage[k] += v
        if status == "completed":
            break
        seconds = round(time.monotonic() - started, 1)
        lower = (THINKING_FALLBACK.get(used_level or "") or [None])[0]
        if status == "incomplete" and attempt == 1 and used_level is not None:
            u = _usage_dict(interaction)
            notes.append(
                f"{model} interaction {interaction.id} incomplete at thinking_level={used_level!r} "
                f"(thought {u['total_thought_tokens']}, output {u['total_output_tokens']} tokens, "
                f"{seconds}s); retried with {lower!r}"
            )
            print(f"[ai.translate] {notes[-1]}", file=sys.stderr, flush=True)
            level = lower
            continue
        errors = getattr(interaction, "errors", None)
        raise TranslateError(
            f"interaction {interaction.id} ended with status {status!r} after {seconds}s, usage {usage}"
            + (f": {errors}" if errors else "")
        )
    seconds = round(time.monotonic() - started, 1)
    rust = extract_rust(getattr(interaction, "output_text", None), notes)
    for note in notes:
        print(f"[ai.translate] {note}", file=sys.stderr)
    return TranslateResult(
        rust_source=rust,
        interaction_id=interaction.id,
        usage=usage,
        seconds=seconds,
        model=model,
        thinking_level=used_level,
        notes=notes,
    )


# ---------------------------------------------------------------- feedback


def _kind(line: str | None) -> str:
    line = line or ""
    if line.startswith("OK\t"):
        return "OK"
    if line.startswith("PARSE_ERROR at"):
        return "ERR_AT"
    if line == "PARSE_ERROR":
        return "ERR"
    if line == "TIMEOUT":
        return "TIMEOUT"
    return "OTHER"


def _char_class(ch: str) -> str:
    if ch == "":
        return "end"
    if ch.isdigit() or ch in "+-.eE":
        return "num"
    if ch == '"':
        return "quote"
    if ch == "\\":
        return "backslash"
    if ch in "{}[],:":
        return ch
    if ch.isalpha():
        return "alpha"
    return "punct"


def divergence_pattern(d: dict) -> str:
    """Coarse signature of a divergence, so 50 copies of one bug count as one pattern."""
    c, r = d.get("c_output") or "", d.get("rust_output") or ""
    kc, kr = _kind(c), _kind(r)
    if kc == "OK" and kr == "OK":
        i = next((j for j, (a, b) in enumerate(zip(c, r)) if a != b), min(len(c), len(r)))
        before = c[max(0, i - 1):i]
        in_string = c[:i].replace("\\\\", "").replace('\\"', "").count('"') % 2 == 1
        where = "string" if in_string else ("number" if before and _char_class(before) == "num" else "structure")
        return f"OK/OK print differs in {where}: C next={_char_class(c[i:i+1])} Rust next={_char_class(r[i:i+1])}"
    inp = d.get("input") or ""
    if kc == "ERR_AT" and kr == "ERR_AT":
        try:
            oc, orr = int(c.rsplit(" ", 1)[1]), int(r.rsplit(" ", 1)[1])
            side = "earlier" if orr < oc else "later"
        except ValueError:
            oc, side = 0, "?"
        # Byte class at C's error offset (approximate: the report input is text).
        return f"both reject, Rust error offset {side} than C, C stops at {_char_class(inp[oc:oc+1])}"
    # Accept/reject disagreements: split by what the document starts with.
    first = inp.lstrip(" \t\r\n")[:1]
    return f"C {kc} vs Rust {kr}, input starts with {_char_class(first)}"


def trim_feedback(feedback: dict) -> dict:
    """Cut a raw verify.sh report (plus compile_errors) down to what goes in the prompt.

    Compile errors in full. Divergences deduped by pattern, the shortest input per
    pattern, most frequent patterns first, at most MAX_DIVERGENCES. Rust crashes and
    timeouts capped at MAX_CRASHES. safety_improvement entries (C crashed, Rust was
    clean) are not problems and are dropped.
    """
    out: dict = {"compile_errors": feedback.get("compile_errors")}
    if feedback.get("match_rate") is not None or feedback.get("counts"):
        out["summary"] = {
            "match_rate": feedback.get("match_rate"),
            "behaviors_tested": feedback.get("behaviors_tested"),
            "counts": feedback.get("counts"),
        }
    groups: dict[str, list[dict]] = {}
    for d in feedback.get("divergences") or []:
        groups.setdefault(divergence_pattern(d), []).append(d)
    ranked = sorted(groups.items(), key=lambda kv: -len(kv[1]))
    picked = []
    for pattern, items in ranked[:MAX_DIVERGENCES]:
        best = min(items, key=lambda d: len(d.get("input") or ""))
        picked.append({
            "input": best.get("input"),
            "c_output": best.get("c_output"),
            "rust_output": best.get("rust_output"),
            "file": best.get("file"),
            "pattern": pattern,
            "pattern_count": len(items),
        })
    out["divergences"] = picked
    out["divergence_total"] = (feedback.get("counts") or {}).get("divergence") or len(feedback.get("divergences") or [])
    crashes = [c for c in feedback.get("crashes") or [] if c.get("class") != "safety_improvement"]
    crashes.sort(key=lambda c: len(c.get("input") or ""))
    out["crashes"] = crashes[:MAX_CRASHES]
    out["timeouts"] = (feedback.get("timeouts") or [])[:MAX_CRASHES]
    return out


# ---------------------------------------------------------------- public API


def translate(c_source: str, model: str = DEFAULT_MODEL) -> TranslateResult:
    """One-shot translation of cJSON parse + print into a complete cjson.rs (thinking high)."""
    return _run(model, prompts.translate_input(c_source), "high", previous_id=None)


def repair(prev: TranslateResult, feedback: dict, model: str = DEFAULT_MODEL) -> TranslateResult:
    """Continue prev's conversation with verifier feedback; returns a full new cjson.rs (thinking medium).

    feedback is {"compile_errors": str | None, "divergences": [...], "crashes": [...]}
    in the shape of verify.sh output (extra verify.sh keys such as counts are used
    for the summary if present).
    """
    text = prompts.format_feedback(trim_feedback(feedback))
    return _run(model, prompts.repair_input(text), "medium", previous_id=prev.interaction_id)
