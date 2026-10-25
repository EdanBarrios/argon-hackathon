"""Count Gemini tokens for the libexpat library sources.

Usage: backend/.venv/bin/python backend/harness/expat/count_tokens.py <expat/lib dir>

Reads GEMINI_API_KEY from the repo .env (or the environment) and never prints
it. Calls models.count_tokens (no generation, no cost) on two sets:
  all    every expat/lib/*.c and *.h
  core   the same minus the platform-specific entropy shims
         (random_*.c/h, winconfig.h), which a Rust port replaces
         with std or a fixed hash salt
"""

import glob
import os
import sys
from pathlib import Path

from google import genai

ROOT = Path(__file__).resolve().parents[3]
MODEL = "gemini-4-argon"
PLATFORM_PREFIXES = ("random_", "winconfig.")


def load_key():
    env_file = ROOT / ".env"
    if env_file.exists():
        for line in env_file.read_text().splitlines():
            name, sep, value = line.strip().partition("=")
            if sep and name.strip() == "GEMINI_API_KEY" and value.strip().strip("'\""):
                return value.strip().strip("'\"")
    return os.environ.get("GEMINI_API_KEY")


def bundle(paths):
    parts = []
    for p in paths:
        with open(p, encoding="utf-8", errors="replace") as fh:
            parts.append("==> %s <==\n%s" % (os.path.basename(p), fh.read()))
    return "\n".join(parts)


def main():
    lib = sys.argv[1]
    files = sorted(glob.glob(os.path.join(lib, "*.c")) + glob.glob(os.path.join(lib, "*.h")))
    core = [p for p in files if not os.path.basename(p).startswith(PLATFORM_PREFIXES)]
    key = load_key()
    if not key:
        sys.exit("GEMINI_API_KEY not set")
    client = genai.Client(api_key=key)
    for label, paths in (("all", files), ("core", core)):
        text = bundle(paths)
        res = client.models.count_tokens(model=MODEL, contents=text)
        lines = text.count("\n")
        print("%s: %d files, %d lines, %d chars, %d Gemini tokens (%s)"
              % (label, len(paths), lines, len(text), res.total_tokens, MODEL))
    return 0


if __name__ == "__main__":
    sys.exit(main())
