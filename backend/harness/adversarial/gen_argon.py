"""Ask Argon for adversarial inputs against a safe-Rust port of cJSON.

One gemini-4-argon call (Interactions API, thinking_level high, background=True
plus polling) with cJSON.c and cJSON.h in the input. Argon only GENERATES
inputs; the differential verifier alone judges them.

Writes corpus/argon/argon_NNN.bin (raw bytes) and corpus/argon/REASONS.md.
The raw response JSON goes to runs/adversarial/argon_response.json (untracked).

Usage: cd backend && uv run python harness/adversarial/gen_argon.py [--count 200]
"""

import argparse
import json
import os
import sys
import time
from pathlib import Path

from google import genai

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent.parent
VENDOR = HERE.parent / "vendor" / "cJSON"
OUT = HERE / "corpus" / "argon"
MODEL = "gemini-4-argon"
POLL_SECONDS = 5

PROMPT = """You are a red-team tester for a C-to-Rust translation.

Below are cJSON.c and cJSON.h (v1.7.19). Someone has ported cJSON's parser and
cJSON_PrintUnformatted to safe Rust (std only, string data as Vec<u8>, no UTF-8
validation, nesting limit 1000). The test driver does exactly this:

  read all of stdin into buf (len bytes), append one NUL byte, then call
  cJSON_ParseWithLengthOpts(buf, len + 1, &end, require_null_terminated = 1).
  On failure print "PARSE_ERROR at <offset>" where offset = cJSON_GetErrorPtr() - buf.
  On success print "OK\\t" + cJSON_PrintUnformatted(root) (cut at the first NUL, like fputs).

Raw stdout bytes of the C and Rust drivers are compared. Your job: produce {count}
distinct inputs that are MOST likely to make a faithful-looking Rust port
produce different bytes, a different accept/reject decision, or a different
error offset than the C code. Think about: number parsing via strtod and the
locale decimal point, the %1.15g / %1.17g round-trip check in print_number,
valueint saturation, NaN/Infinity printing as null, buffer_skip_whitespace and
what counts as whitespace, the UTF-8 BOM skip, parse_string's two-pass length
computation, utf16_literal_to_utf8 and surrogate handling, the error pointer
position for every failure path, require_null_terminated and the trailing NUL,
embedded NUL bytes, nesting depth accounting, print buffer growth, and anything
else in the code that a reimplementation would likely get subtly wrong.

Rules:
- Each input is raw bytes, hex-encoded (lowercase, no spaces) in input_hex.
- Keep each input under 4096 bytes (deep nesting up to ~1100 levels is fine).
- Give a one-sentence reason naming the cJSON function and the suspected difference.
- No duplicates. Prefer variety across functions over many variants of one idea.

=== cJSON.h ===
{header}

=== cJSON.c ===
{source}
"""

SCHEMA = {
    "type": "object",
    "properties": {
        "inputs": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "input_hex": {"type": "string"},
                    "reason": {"type": "string"},
                },
                "required": ["input_hex", "reason"],
            },
        }
    },
    "required": ["inputs"],
}


def load_key() -> str:
    env_file = ROOT / ".env"
    if env_file.exists():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line.startswith("#") or "=" not in line:
                continue
            name, _, value = line.partition("=")
            if name.strip() == "GEMINI_API_KEY" and value.strip().strip("'\""):
                return value.strip().strip("'\"")
    key = os.environ.get("GEMINI_API_KEY")
    if not key:
        sys.exit("GEMINI_API_KEY not found in .env or environment")
    return key


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--count", type=int, default=200)
    args = ap.parse_args()

    prompt = PROMPT.format(
        count=args.count,
        header=(VENDOR / "cJSON.h").read_text(errors="replace"),
        source=(VENDOR / "cJSON.c").read_text(errors="replace"),
    )
    client = genai.Client(api_key=load_key())
    started = time.monotonic()
    interaction = client.interactions.create(
        model=MODEL,
        input=prompt,
        background=True,
        generation_config={"thinking_level": "high", "max_output_tokens": 131072},
        response_format={"type": "text", "mime_type": "application/json", "schema": SCHEMA},
    )
    print(f"created {interaction.id}", flush=True)
    while getattr(interaction, "status", None) in ("queued", "in_progress", None):
        time.sleep(POLL_SECONDS)
        try:
            interaction = client.interactions.get(interaction.id)
        except Exception as exc:  # transient network errors: keep polling
            print(f"poll error: {type(exc).__name__}: {exc}", flush=True)
    seconds = round(time.monotonic() - started, 1)
    usage = interaction.usage
    print(f"status={interaction.status} seconds={seconds} "
          f"in={getattr(usage, 'total_input_tokens', None)} "
          f"out={getattr(usage, 'total_output_tokens', None)} "
          f"thought={getattr(usage, 'total_thought_tokens', None)}", flush=True)
    if interaction.status != "completed":
        print(f"error: {getattr(interaction, 'error', None)}")
        return 1

    text = interaction.output_text or ""
    OUT.mkdir(parents=True, exist_ok=True)
    raw = ROOT / "runs" / "adversarial"
    raw.mkdir(parents=True, exist_ok=True)
    (raw / "argon_response.json").write_text(text)
    items = json.loads(text)["inputs"]

    for old in OUT.glob("argon_*.bin"):
        old.unlink()
    seen = set()
    rows = []
    bad = 0
    for item in items:
        try:
            data = bytes.fromhex(item["input_hex"].strip())
        except ValueError:
            bad += 1
            continue
        if data in seen:
            continue
        seen.add(data)
        name = "argon_%03d.bin" % len(rows)
        (OUT / name).write_bytes(data)
        rows.append((name, data, item["reason"].replace("|", "\\|").replace("\n", " ")))

    with open(OUT / "REASONS.md", "w") as fh:
        fh.write("# Argon-written adversarial inputs\n\n")
        fh.write(f"Model {MODEL}, thinking_level high, interaction {interaction.id}, "
                 f"{seconds}s wall. {len(items)} returned, {len(rows)} unique, "
                 f"{bad} with invalid hex dropped. Argon only generated these; "
                 "the verifier judges them.\n\n")
        fh.write("| file | bytes | preview | reason |\n|---|---|---|---|\n")
        for name, data, reason in rows:
            preview = data[:40].decode("latin-1").encode("unicode_escape").decode()
            preview = preview.replace("|", "\\|").replace("`", "'")
            fh.write(f"| {name} | {len(data)} | `{preview}` | {reason} |\n")
    print(f"wrote {len(rows)} inputs ({bad} bad hex) to {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
