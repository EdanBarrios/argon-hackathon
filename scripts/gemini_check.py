"""Check that GEMINI_API_KEY works and both models answer.

Usage: cd backend && uv run python ../scripts/gemini_check.py

Key and model check only. The managed-agent sandbox smoke test lives with A.
"""

import os
import sys
import time
from pathlib import Path

from google import genai

ROOT = Path(__file__).resolve().parent.parent
MODELS = ["gemini-4-argon", "gemini-3.8-flash"]


def load_key() -> str | None:
    env_file = ROOT / ".env"
    if env_file.exists():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line.startswith("#") or "=" not in line:
                continue
            name, _, value = line.partition("=")
            if name.strip() == "GEMINI_API_KEY":
                value = value.strip().strip("'\"")
                if value:
                    return value
    return os.environ.get("GEMINI_API_KEY") or None


def check(client: genai.Client, model: str) -> bool:
    start = time.monotonic()
    try:
        interaction = client.interactions.create(
            model=model,
            input="Reply with exactly: ok",
            generation_config={"thinking_level": "low"},
        )
    except Exception as e:
        print(f"FAIL {model}: {type(e).__name__}: {e}")
        return False
    elapsed = time.monotonic() - start
    usage = interaction.usage
    tokens = (
        f" in={usage.total_input_tokens} out={usage.total_output_tokens} thought={usage.total_thought_tokens}"
        if usage
        else ""
    )
    print(f"OK   {model}: {interaction.output_text!r} {elapsed:.1f}s{tokens}")
    return True


def main() -> int:
    key = load_key()
    if not key:
        print("GEMINI_API_KEY not found: add GEMINI_API_KEY to .env (copy .env.example)")
        return 1
    client = genai.Client(api_key=key)
    results = [check(client, model) for model in MODELS]
    return 0 if all(results) else 1


if __name__ == "__main__":
    sys.exit(main())
