"""Per-job token and USD tracking for Gemini Interactions API usage.

Pricing (docs/GEMINI-NOTES.md, valid through Nov 30, 2026):
- $2 per 1M input tokens, $10 per 1M output tokens.
- Cached input tokens are billed at 5% of the input price (implicit caching, 95% off).

Assumptions, since the SDK docs do not spell them out:
- total_input_tokens INCLUDES total_cached_tokens, so uncached input = input - cached.
- total_output_tokens EXCLUDES total_thought_tokens, and thought tokens are billed
  at the output rate, so billed output = output + thought.
"""

import threading
from typing import Any

INPUT_USD_PER_TOKEN = 2.0 / 1_000_000
OUTPUT_USD_PER_TOKEN = 10.0 / 1_000_000
CACHED_INPUT_FRACTION = 0.05

FIELDS = ("total_input_tokens", "total_output_tokens", "total_thought_tokens", "total_cached_tokens")

_lock = threading.Lock()
# job_id -> model -> {"input_tokens", "cached_tokens", "output_tokens", "thought_tokens"}
_totals: dict[str, dict[str, dict[str, int]]] = {}


def _field(usage: Any, name: str) -> int:
    """Read a usage field from a dict or an SDK object; missing or None counts as 0."""
    value = usage.get(name) if isinstance(usage, dict) else getattr(usage, name, None)
    return int(value or 0)


def usd_for(input_tokens: int, cached_tokens: int, output_tokens: int, thought_tokens: int) -> float:
    cached = min(cached_tokens, input_tokens)
    uncached = input_tokens - cached
    usd = (
        uncached * INPUT_USD_PER_TOKEN
        + cached * INPUT_USD_PER_TOKEN * CACHED_INPUT_FRACTION
        + (output_tokens + thought_tokens) * OUTPUT_USD_PER_TOKEN
    )
    return round(usd, 6)


def record_usage(job_id: str, model: str, usage: Any) -> None:
    """Add one interaction's usage (interaction.usage or an equivalent dict) to the job's totals."""
    with _lock:
        t = _totals.setdefault(job_id, {}).setdefault(
            model, {"input_tokens": 0, "cached_tokens": 0, "output_tokens": 0, "thought_tokens": 0}
        )
        t["input_tokens"] += _field(usage, "total_input_tokens")
        t["cached_tokens"] += _field(usage, "total_cached_tokens")
        t["output_tokens"] += _field(usage, "total_output_tokens")
        t["thought_tokens"] += _field(usage, "total_thought_tokens")


def job_cost(job_id: str) -> dict[str, Any]:
    """Shape of the proposed GET /jobs/{id}/cost response."""
    with _lock:
        per_model = {m: dict(t) for m, t in _totals.get(job_id, {}).items()}
    models = [{"model": m, **t, "usd": usd_for(**t)} for m, t in sorted(per_model.items())]
    return {"job_id": job_id, "models": models, "total_usd": round(sum(m["usd"] for m in models), 6)}
