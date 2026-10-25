"""Managed Agents sandbox smoke test (feeds the 11:15 remote vs local runner decision).

Usage:
  cd backend && uv run python ../scripts/sandbox_smoke.py
  cd backend && uv run python ../scripts/sandbox_smoke.py --followup <interaction_id>

Turn 1 makes ONE background managed-agent call (antigravity + gemini-4-argon) in a
remote sandbox with a network allowlist, asks it to check toolchains, ASan, rustup,
and a git clone, then polls until done.

--followup sends one more turn with previous_interaction_id to check whether the
files and tools from turn 1 are still in the sandbox.

API shape checked against google-genai 2.33.0 source
(_gaos/types/interactions/createagentinteraction.py, environment.py,
environmentnetworkegressallowlist.py, allowlistentry.py): allowlist entries are
objects {"domain": ...}, not bare strings.
"""

import argparse
import os
import sys
import time
from pathlib import Path

from google import genai

ROOT = Path(__file__).resolve().parent.parent
AGENT = "antigravity-preview-09-2026"
AGENT_CONFIG = {"type": "antigravity", "model": "gemini-4-argon"}
ALLOWLIST = [
    "github.com",
    "static.rust-lang.org",
    "sh.rustup.rs",
    "crates.io",
    "index.crates.io",
    "static.crates.io",
]
ENVIRONMENT = {
    "type": "remote",
    "network": {"allowlist": [{"domain": d} for d in ALLOWLIST]},
}
POLL_SECONDS = 10
MAX_WAIT_SECONDS = 20 * 60
TERMINAL = {"completed", "failed", "cancelled", "incomplete", "budget_exceeded", "requires_action"}

TURN1_PROMPT = """You are running a smoke test of this sandbox. Run each step below in the
sandbox shell. For every command, report the exact command, its output VERBATIM
(stdout and stderr, in a code block), its exit code, and its wall-clock time
(wrap each with `time` or record `date +%s.%N` before and after). Do not
summarize or paraphrase output. If a step fails, report the error verbatim and
continue with the next step.

1. `uname -a`
2. `which clang rustc cargo`, then for each one that exists print its version
   (`clang --version`, `rustc --version`, `cargo --version`).
3. If rustc is missing, install it with rustup using the minimal profile:
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal`
   then `. "$HOME/.cargo/env"`. Time the whole install. Then print `rustc --version` and `cargo --version`.
   If clang is missing, try to install it (apt-get, if available) and report what happened.
4. In /tmp/smoke, write a C hello world (hello.c), compile it with
   `clang -fsanitize=address -g hello.c -o hello_c` and run `./hello_c`.
   Then also compile and run this deliberately buggy program with the same flags
   to prove ASan really reports errors:
   `int main(){int a[2]; volatile int i=2; return a[i];}`
   Report whether the ASan report appeared.
5. In /tmp/smoke, write a Rust hello world (hello.rs), compile it with `rustc hello.rs -o hello_rs`, and run `./hello_rs`.
6. `git clone --depth 1 https://github.com/EdanBarrios/argon-hackathon /tmp/smoke/argon-hackathon`
   then `ls -la /tmp/smoke/argon-hackathon/backend/harness`.
7. If time allows: read /tmp/smoke/argon-hackathon/backend/harness/README.md, run
   `backend/harness/fetch_cjson.sh`, then the C driver build command the README gives,
   from the repo root. Report output verbatim and timing.

At the end, print a short table: step, ok or failed, seconds. Then state plainly:
clang preinstalled (yes/no), rustc preinstalled (yes/no), rustup install seconds,
ASan works (yes/no), git clone works (yes/no), cJSON harness build works (yes/no/skipped).
"""

FOLLOWUP_PROMPT = """Follow-up check on the same sandbox. Run these and report output verbatim
with exit codes:

1. `ls -la /tmp/smoke /tmp/smoke/argon-hackathon/backend/harness`
2. `which clang rustc cargo; ls -la "$HOME/.cargo/bin" 2>&1; . "$HOME/.cargo/env" 2>/dev/null; which rustc && rustc --version`
3. `cat /proc/1/stat 2>/dev/null | cut -d' ' -f22; uptime`

Then answer plainly: do the files from the previous turn still exist (yes/no)?
Is rustc still installed (yes/no)?
"""


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


def fmt_usage(usage) -> str:
    if not usage:
        return "none"
    fields = [
        "total_input_tokens",
        "total_output_tokens",
        "total_thought_tokens",
        "total_cached_tokens",
        "total_tool_use_tokens",
        "total_tokens",
    ]
    return " ".join(f"{f.replace('total_', '')}={getattr(usage, f, None)}" for f in fields)


def run(client: genai.Client, prompt: str, previous_id: str | None) -> int:
    environment: dict | str = ENVIRONMENT
    if previous_id:
        # Reuse turn 1's sandbox. Sending the config dict again may provision a
        # fresh environment; a string is an existing environment ID (SDK type
        # CreateAgentInteractionEnvironmentParam = EnvironmentParam | str).
        prev = client.interactions.get(previous_id)
        if prev.environment_id:
            environment = prev.environment_id
        print(f"previous environment id: {prev.environment_id}")
    kwargs = dict(
        agent=AGENT,
        agent_config=AGENT_CONFIG,
        input=prompt,
        environment=environment,
        background=True,
    )
    if previous_id:
        kwargs["previous_interaction_id"] = previous_id

    start = time.monotonic()
    try:
        interaction = client.interactions.create(**kwargs)
    except Exception as e:
        print(f"CREATE FAILED after {time.monotonic() - start:.1f}s: {type(e).__name__}: {e}")
        return 1
    print(f"interaction id: {interaction.id}")
    print(f"status: {interaction.status} (create returned in {time.monotonic() - start:.1f}s)")

    last_status = interaction.status
    while interaction.status not in TERMINAL:
        if time.monotonic() - start > MAX_WAIT_SECONDS:
            print(f"TIMEOUT after {MAX_WAIT_SECONDS}s, last status {interaction.status}")
            break
        time.sleep(POLL_SECONDS)
        try:
            interaction = client.interactions.get(interaction.id)
        except Exception as e:
            print(f"poll error at {time.monotonic() - start:.0f}s: {type(e).__name__}: {e}")
            continue
        if interaction.status != last_status:
            print(f"status: {interaction.status} at {time.monotonic() - start:.0f}s")
            last_status = interaction.status

    total = time.monotonic() - start
    print()
    print(f"total wall-clock seconds: {total:.1f}")
    print(f"interaction id: {interaction.id}")
    print(f"final status: {interaction.status}")
    print(f"environment id: {getattr(interaction, 'environment_id', None)}")
    print(f"usage: {fmt_usage(interaction.usage)}")
    if interaction.errors:
        print(f"errors: {interaction.errors}")
    steps = interaction.steps or []
    print(f"steps: {len(steps)} ({', '.join(sorted({getattr(s, 'type', '?') for s in steps}))})")
    print()
    print("===== agent output text =====")
    print(interaction.output_text)
    return 0 if interaction.status == "completed" else 1


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--followup", metavar="INTERACTION_ID", help="send the persistence check turn")
    args = parser.parse_args()

    key = load_key()
    if not key:
        print("GEMINI_API_KEY not found: add GEMINI_API_KEY to .env (copy .env.example)")
        return 1
    client = genai.Client(api_key=key)
    if args.followup:
        return run(client, FOLLOWUP_PROMPT, args.followup)
    return run(client, TURN1_PROMPT, None)


if __name__ == "__main__":
    sys.exit(main())
