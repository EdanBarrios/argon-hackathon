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
from typing import Any, Callable

from google import genai

from . import prompts
from .patch import PatchError, apply_patch
from .feedback_patterns import divergence_pattern, expat_divergence_pattern  # noqa: F401 (re-export)
from .targets import CJSON, Target

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
    # Follow-up turns used because the first reply hit the output cap (0 = single pass).
    continuations: int = 0
    # How a repair produced rust_source: "patch" (SEARCH/REPLACE blocks applied), "patch_fallback"
    # (the patch did not apply, so a full-file call followed; usage and seconds include both),
    # None for a full-file reply.
    repair_mode: str | None = None


class PatchFailed(TranslateError):
    """A patch reply did not apply. Carries what the wasted call cost."""

    def __init__(self, message: str, usage: dict, seconds: float, notes: list[str]):
        super().__init__(message)
        self.usage, self.seconds, self.notes = usage, seconds, notes


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


def _create(client: genai.Client, kwargs: dict, thinking: str | None, notes: list[str],
            max_tokens: int = MAX_OUTPUT_TOKENS):
    """interactions.create, stepping down thinking_level / max_output_tokens if the model rejects them."""
    levels = [thinking] + THINKING_FALLBACK.get(thinking or "", [])
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


_LEADING_FENCE = re.compile(r"\A\s*```[ \t]*(?:rust|rs)?[ \t]*\r?\n", re.IGNORECASE)


def _continue(client: genai.Client, kwargs: dict, interaction: Any, level: str | None,
              notes: list[str], usage: dict, target: Target) -> tuple[Any, str, int]:
    """Resume a reply cut off at the output cap, on the same conversation.

    Returns (last interaction, joined text, continuation turns used). Raises
    TranslateError if the reply is still cut off after target.continue_turns turns.
    """
    text = getattr(interaction, "output_text", None) or ""
    for turn in range(1, target.continue_turns + 1):
        notes.append(f"{kwargs['model']} interaction {interaction.id} hit the output cap "
                     f"({len(text)} chars so far); continuation turn {turn}")
        print(f"[ai.translate] {notes[-1]}", file=sys.stderr, flush=True)
        cont = dict(kwargs, input=prompts.CONTINUE_INPUT, previous_interaction_id=interaction.id)
        interaction, _ = _create(client, cont, level, notes, target.max_output_tokens)
        interaction = _wait(client, interaction, target.max_wait_seconds)
        for k, v in _usage_dict(interaction).items():
            usage[k] += v
        text += _LEADING_FENCE.sub("", getattr(interaction, "output_text", None) or "", count=1)
        status = getattr(interaction, "status", None)
        if status == "completed":
            return interaction, text, turn
        if status != "incomplete":
            raise TranslateError(f"continuation {interaction.id} ended with status {status!r}, usage {usage}")
    raise TranslateError(f"reply still cut off after {target.continue_turns} continuation turns, usage {usage}")


def _run(model: str, input_text: str, thinking: str, previous_id: str | None,
         target: Target = CJSON,
         parse: Callable[[str | None, list[str]], str] = extract_rust) -> TranslateResult:
    """One logical call. If the model runs out of output budget (status "incomplete"),
    retry once at the next lower thinking level. Observed: gemini-3.8-flash at "high"
    spent ~63K thought tokens and stopped at the 65,536 output cap mid-file, even
    with max_output_tokens=131072. Usage and seconds include the failed attempt."""
    client = _get_client()
    notes: list[str] = []
    kwargs: dict = {
        "model": model,
        "input": input_text,
        "system_instruction": target.system_instruction,
        "background": True,
    }
    if previous_id:
        kwargs["previous_interaction_id"] = previous_id
    started = time.monotonic()
    usage = dict.fromkeys(USAGE_FIELDS, 0)
    level: str | None = thinking
    text: str | None = None
    continuations = 0
    for attempt in (1, 2):
        interaction, used_level = _create(client, kwargs, level, notes, target.max_output_tokens)
        interaction = _wait(client, interaction, target.max_wait_seconds)
        status = getattr(interaction, "status", None)
        for k, v in _usage_dict(interaction).items():
            usage[k] += v
        if status == "completed":
            break
        if status == "incomplete" and target.continue_turns:
            interaction, text, continuations = _continue(client, kwargs, interaction, used_level, notes, usage, target)
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
    try:
        rust = parse(text if text is not None else getattr(interaction, "output_text", None), notes)
    except PatchError as exc:
        raise PatchFailed(f"patch reply did not apply: {exc}", usage, seconds, notes) from exc
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
        continuations=continuations,
    )


# ---------------------------------------------------------------- feedback


def trim_feedback(feedback: dict, pattern=divergence_pattern) -> dict:
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
        groups.setdefault(pattern(d), []).append(d)
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
            **({"first_diff": best["first_diff"]} if best.get("first_diff") else {}),
        })
    out["divergences"] = picked
    out["divergence_total"] = (feedback.get("counts") or {}).get("divergence") or len(feedback.get("divergences") or [])
    crashes = [c for c in feedback.get("crashes") or [] if c.get("class") != "safety_improvement"]
    crashes.sort(key=lambda c: len(c.get("input") or ""))
    out["crashes"] = crashes[:MAX_CRASHES]
    out["timeouts"] = (feedback.get("timeouts") or [])[:MAX_CRASHES]
    return out


# ---------------------------------------------------------------- public API


def translate(c_source: str, model: str = DEFAULT_MODEL, target: Target = CJSON) -> TranslateResult:
    """One-shot translation of the target (default cJSON parse + print) into one complete Rust file (thinking high)."""
    return _run(model, target.translate_input(c_source), "high", previous_id=None, target=target)


def repair(prev: TranslateResult, feedback: dict, model: str = DEFAULT_MODEL,
           target: Target = CJSON, *, fresh_source: str | None = None,
           patch: bool = False) -> TranslateResult:
    """Continue prev's conversation with verifier feedback; returns a full new cjson.rs (thinking medium).

    feedback is {"compile_errors": str | None, "divergences": [...], "crashes": [...]}
    in the shape of verify.sh output (extra verify.sh keys such as counts are used
    for the summary if present).

    With fresh_source (the C source), the repair is a new conversation that resends the
    whole translate message plus prev's file, instead of a previous_interaction_id turn.

    With patch=True the model answers with SEARCH/REPLACE blocks that are applied to
    prev.rust_source, so a small fix costs a small reply. If the blocks do not apply, the
    full-file repair below runs instead and the result adds up both calls.
    """
    text = prompts.format_feedback(trim_feedback(feedback, target.divergence_pattern))
    if patch:
        try:
            return _patch_repair(prev, text, model, target, fresh_source)
        except PatchFailed as failed:
            print(f"[ai.translate] {failed}; falling back to a full-file repair", file=sys.stderr)
            res = repair(prev, feedback, model, target, fresh_source=fresh_source)
            res.usage = {k: res.usage.get(k, 0) + failed.usage.get(k, 0) for k in USAGE_FIELDS}
            res.seconds = round(res.seconds + failed.seconds, 1)
            res.notes = [*failed.notes, str(failed), *res.notes]
            res.repair_mode = "patch_fallback"
            return res
    if fresh_source is not None:
        message = prompts.fresh_repair_input(target.translate_input(fresh_source), target.rust_file,
                                             prev.rust_source, text)
        return _run(model, message, "medium", previous_id=None, target=target)
    return _run(model, target.repair_input(text), "medium", previous_id=prev.interaction_id, target=target)


def _patch_repair(prev: TranslateResult, feedback_text: str, model: str, target: Target,
                  fresh_source: str | None) -> TranslateResult:
    """One patch-mode call. Continues prev's conversation unless fresh_source is given (or prev
    has no conversation, as after --start-from). The file is always resent in full."""
    new_conversation = fresh_source is not None
    message = prompts.patch_repair_input(
        target.rust_file, prev.rust_source, feedback_text,
        reference_input=target.translate_input(fresh_source) if new_conversation else None)

    def parse(text: str | None, notes: list[str]) -> str:
        patched, stats = apply_patch(prev.rust_source, text)
        notes.append(f"patch: {stats.blocks} block(s), -{stats.lines_removed}/+{stats.lines_added} lines")
        return patched

    res = _run(model, message, "medium", previous_id=None if new_conversation else prev.interaction_id,
               target=target, parse=parse)
    res.repair_mode = "patch"
    return res


def polish(prev: TranslateResult, current_source: str, lint_text: str, model: str = DEFAULT_MODEL,
           target: Target = CJSON, *, fresh_source: str | None = None) -> TranslateResult:
    """A lint-only repair turn for a build that already matches (thinking medium).

    current_source is the file the lints were measured on (possibly host-fixed), sent in
    full. Continues prev's conversation, or with fresh_source starts a new one that
    resends the translate message first.
    """
    message = prompts.polish_input(target.rust_file, current_source, lint_text)
    if fresh_source is not None:
        return _run(model, target.translate_input(fresh_source) + "\n\n" + message, "medium",
                    previous_id=None, target=target)
    return _run(model, message, "medium", previous_id=prev.interaction_id, target=target)
