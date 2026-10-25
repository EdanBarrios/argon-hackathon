"""Rebuild each libexpat call's reported input tokens from its own stored steps.

    cd backend && uv run python ai/results/expat-long-decode/token_accounting.py

Model: a long reply is generated in segments of up to ~65.5K tokens (thought + output),
and every segment re-reads the prompt plus everything generated before it. So
    input = segments * prompt + sum over segments k>1 of tokens generated before k.
prompt = count_tokens(user input) + count_tokens(system instruction). Read-only API calls.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3]))  # backend/, for `ai`
from ai import translate as T
from ai.targets import EXPAT

HERE = Path(__file__).parent
CALLS = ["gemini-4-argon/round_1", "argon-fresh-a/round_2", "argon-fresh-a/round_3"]

c = T._get_client()
count = lambda t: c.models.count_tokens(model="gemini-4-argon", contents=t).total_tokens if t else 0
system = count(EXPAT.system_instruction)


def text(x) -> str:
    """Every "text" field in a step, in order (thought steps carry only a signature)."""
    if isinstance(x, dict):
        return "".join(v if k == "text" and isinstance(v, str) else text(v) for k, v in x.items())
    if isinstance(x, list):
        return "".join(text(v) for v in x)
    return ""


for call in CALLS:
    it = c.interactions.get(json.loads((HERE / call / "meta.json").read_text())["interaction_id"])
    u = it.usage
    texts = [(s.type, text(s.model_dump())) for s in it.steps]
    prompt = system + sum(count(t) for k, t in texts if k == "user_input")
    outputs = [count(t) for k, t in texts if k == "model_output"]
    # Segment sizes: the first segment holds all thinking plus the first output piece.
    segs = [u.total_thought_tokens + outputs[0], *outputs[1:]]
    segs = [s for s in segs if s] or [0]
    rebuilt = sum(prompt + sum(segs[:k]) for k in range(len(segs)))
    print(f"{call}: prompt {prompt:,}, segments {segs} ({len(segs)}), rebuilt input {rebuilt:,}, "
          f"reported {u.total_input_tokens:,}, diff {u.total_input_tokens - rebuilt:+,}, cached {u.total_cached_tokens:,}")
