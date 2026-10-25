"""Prompts for the cJSON parse + print translation.

All prompt text lives here so the Argon and Flash runs use identical words.
translate.py only assembles these pieces and talks to the API.
"""

from __future__ import annotations

import json
from pathlib import Path

# backend/ai/prompts.py -> repo root is two levels up from backend/.
REPO_ROOT = Path(__file__).resolve().parents[2]
CRATE_DIR = REPO_ROOT / "backend" / "harness" / "rust_crate"
DRIVER_C = REPO_ROOT / "backend" / "harness" / "drivers" / "cjson_driver.c"

SYSTEM_INSTRUCTION = """\
You are translating C to safe Rust for Theseus, a C to Rust migration tool whose \
output is checked by a deterministic differential verifier, not by you.

Task: translate the parse path and the print path of cJSON v1.7.19 (cJSON.c and \
cJSON.h) into ONE safe-Rust file, `cjson.rs`, that implements INTERFACE.md exactly. \
The file replaces `src/cjson.rs` in a fixed crate whose `src/main.rs` you are given \
and cannot change.

Hard rules:
1. Match the C output byte for byte. The verifier feeds the same bytes to the C \
driver and to your Rust driver and compares raw stdout. Any difference is a failure.
2. No unsafe code. The build rejects any file containing the word "unsafe", even in \
a comment or string, so never write that word at all.
3. std only. No external crates, no FFI, no build scripts. It must build offline.
4. Nesting limit 1000 (CJSON_NESTING_LIMIT). Deeper input is a parse error with the \
same error offset as C, never a stack overflow.
5. Never panic: no unwrap on fallible data, no out-of-bounds indexing, no arithmetic \
overflow. A panic counts as a crash.
6. Keep the public API from INTERFACE.md: `pub enum Value`, \
`pub struct ParseError { pub offset: Option<usize>, .. }`, \
`pub fn parse(input: &[u8]) -> Result<Value, ParseError>`, \
`pub fn print_unformatted(value: &Value) -> Vec<u8>`.

Known traps. Each one has broken earlier translations:
- Number printing. cJSON's print_number uses sprintf "%1.15g", then re-parses the \
text with sscanf "%lg" and, if compare_double (a relative DBL_EPSILON tolerance, \
not exact equality) says the round trip failed, prints with "%1.17g" instead. Rust has no %g. Emulate %g exactly: choose between fixed and exponential \
notation by the decimal exponent (exponential when exp < -4 or exp >= precision), \
strip trailing zeros and a trailing decimal point, and print the exponent as C does \
(sign always, at least two digits: 1e+20, 1e-05). Get the correctly rounded digits \
from Rust's own exact formatting (for example format!("{:.*e}", precision - 1, d)) and \
rebuild the %g layout from them.
- inf and NaN (including -inf) print as the four bytes `null`.
- Integer-valued doubles: when valuedouble is integral and fits, C prints it via the \
integer path (%d on valueint, which is saturated to INT_MAX/INT_MIN). Read \
print_number carefully and reproduce exactly which doubles take that path and what \
they print, including -0.0.
- Number parsing: parse_number copies the number characters (digits, + - . e E) into \
a buffer and calls strtod, which stops at the first invalid character. The parse \
pointer advances by what strtod consumed, not by what was copied. Reproduce strtod's \
acceptance (for example "1." "1e" "-" "01") and its value.
- Strings are bytes. Use Vec<u8>, never String or str for JSON string data. cJSON does \
not validate UTF-8; invalid bytes round-trip unchanged.
- Surrogate pairs: follow utf16_literal_to_utf8 exactly. A high surrogate must be \
followed by a backslash-u low surrogate or the whole string fails; a lone low \
surrogate fails; check the exact failure offset that C reports.
- Escapes on print: print_string_ptr escapes quote, backslash, \\b \\f \\n \\r \\t, \
and other bytes below 0x20 as \\u00xx with lowercase hex. Bytes >= 0x7f are copied \
raw. "/" is not escaped. Strings are C strings internally, so printing stops at the \
first NUL byte in the string data (a parsed "a\\u0000b" prints as "a").
- require_null_terminated semantics. The driver calls \
cJSON_ParseWithLengthOpts(input + "\\0", len + 1, &end, 1): the buffer handed to cJSON \
is the input with one NUL byte appended. After the value, cJSON skips whitespace and \
then requires the current byte to be NUL; otherwise it fails. So an embedded NUL after \
the value ends the document, and anything else after the value is an error. Also \
reproduce the UTF-8 BOM skip and the whitespace definition (bytes <= 32).
- ParseError.offset must equal C's error position: cJSON_GetErrorPtr() minus the \
input start. Trace how C sets global_error.position on every failure path (it uses \
buffer.offset at the moment of failure, or length - 1 when offset >= length), including failures deep inside \
nested values and the trailing-garbage check. The driver prints "PARSE_ERROR at <n>".
- Objects keep key order and duplicate keys. Empty array prints [] and empty object \
prints {}.

Reply format: think as long as you need, then reply with exactly one fenced code block \
tagged rust that contains the complete cjson.rs, from the first line to the last. No \
other code blocks. Never elide code with comments like "rest unchanged"; the block is \
compiled as is.
"""


def _fence(lang: str, name: str, text: str) -> str:
    return f"=== FILE: {name} ===\n```{lang}\n{text.rstrip()}\n```\n"


def load_cjson_source(vendor_dir: Path) -> str:
    """cJSON.h and cJSON.c as one labeled text blob, the `c_source` for translate()."""
    header = (vendor_dir / "cJSON.h").read_text(encoding="utf-8", errors="replace")
    body = (vendor_dir / "cJSON.c").read_text(encoding="utf-8", errors="replace")
    return _fence("c", "cJSON.h", header) + "\n" + _fence("c", "cJSON.c", body)


def _reference(c_source: str) -> list[str]:
    """The C library, the Rust contract and both drivers: everything a translation needs."""
    interface = (CRATE_DIR / "INTERFACE.md").read_text(encoding="utf-8")
    main_rs = (CRATE_DIR / "src" / "main.rs").read_text(encoding="utf-8")
    stub_rs = (CRATE_DIR / "src" / "cjson.rs").read_text(encoding="utf-8")
    driver_c = DRIVER_C.read_text(encoding="utf-8") if DRIVER_C.exists() else ""
    parts = [
        "Translate the cJSON parse and print paths into cjson.rs.\n",
        "Below: the full C source, the Rust interface contract, the fixed Rust driver, "
        "the C driver it must match, and the current stub cjson.rs (you may keep or "
        "change its Value shape as long as the public API holds).\n",
        c_source,
        _fence("markdown", "backend/harness/rust_crate/INTERFACE.md", interface),
        _fence("rust", "backend/harness/rust_crate/src/main.rs (fixed, do not change)", main_rs),
    ]
    if driver_c:
        parts.append(_fence("c", "backend/harness/drivers/cjson_driver.c (the C side)", driver_c))
    parts.append(_fence("rust", "backend/harness/rust_crate/src/cjson.rs (stub to replace)", stub_rs))
    return parts


def translate_input(c_source: str) -> str:
    """First-turn user message: the whole C library plus the Rust contract."""
    parts = _reference(c_source)
    parts.append(
        "Now write the complete cjson.rs. Reply with exactly one ```rust block "
        "containing the whole file."
    )
    return "\n".join(parts)


REPO_URL = "https://github.com/EdanBarrios/argon-hackathon"

SANDBOX_TASK = """\
You are in a remote Linux sandbox with a shell. Translate cJSON as described below, \
but this time you can build your translation and run it through the team's \
deterministic verifier yourself. The verifier decides what matches; your own reading \
of the code does not. Shell state does not persist between commands, so start every \
command that needs Rust with `. "$HOME/.cargo/env";`.

Setup. Skip a step whose result already exists (this sandbox may be reused). If a \
step fails, report its command and output verbatim and stop.
1. Unless `clang` exists: `DEBIAN_FRONTEND=noninteractive apt-get update -qq && \
DEBIAN_FRONTEND=noninteractive apt-get install -y -qq clang`
2. Unless `$HOME/.cargo/bin/rustc` exists: `curl --proto '=https' --tlsv1.2 -sSf \
https://sh.rustup.rs | sh -s -- -y --profile minimal`
3. Unless /work/theseus exists: `git clone --depth 1 --branch {branch} {repo} /work/theseus`. \
Run every later command from /work/theseus.
4. `backend/harness/fetch_cjson.sh && backend/harness/build_cjson.sh`
5. `python3 -I backend/harness/gen_inputs.py --seed {seed} --count {count} --out /work/inputs \
--cjson backend/harness/vendor/cJSON`

Loop, at most {rounds} rounds, numbered from 1:
a. Write your complete translation to /work/cjson.rs.
b. `scripts/build_rust.sh /work/cjson.rs round<N>` prints one JSON line. If "ok" is \
false, its "errors" are your feedback for the next round.
c. If it built: `backend/harness/verify.sh --c backend/harness/build/cjson_driver \
--rust <binary from b> --inputs /work/inputs --timeout 20 --out /work/verify_<N>.json`. \
Read match_rate, divergences and crashes. Ignore crashes of class safety_improvement \
(C crashed, Rust did not: intended).
d. Stop when match_rate is 1.0 or the rounds run out. Otherwise fix the root causes \
(the C source is the specification) and go to a.

Rules: change only /work/cjson.rs. Never edit anything under /work/theseus, the \
inputs, or the verifier. The host rebuilds your final file and verifies it again on \
inputs you never see, so tuning to these inputs does not help.

Final reply, in this format and nothing else:
1. One line per round, copied from the tool output: \
`ROUND <N> build=<ok|fail> match_rate=<number, or - if it did not build>`
2. The complete final /work/cjson.rs (the last version that built) in exactly one \
```rust block, with nothing after it.
"""


def sandbox_input(c_source: str, *, branch: str, seed: int, count: int, rounds: int) -> str:
    """Single-turn task for a managed agent that runs translate, build, verify, repair itself."""
    task = SANDBOX_TASK.format(branch=branch, repo=REPO_URL, seed=seed, count=count, rounds=rounds)
    return "\n".join([task, "Reference material for the translation follows.\n", *_reference(c_source)])


def repair_input(feedback_text: str) -> str:
    """Repair-turn user message. The previous turn (with our cjson.rs) is server-side state."""
    return (
        "The verifier ran your previous cjson.rs against the C build. Results below. "
        "Fix every reported problem. Each divergence is one representative of a "
        "distinct failure pattern; fix the root cause, not only that input. Do not "
        "break behavior that already matches.\n\n"
        + feedback_text
        + "\n\nReply with exactly one ```rust block containing the complete corrected "
        "cjson.rs, the whole file."
    )


def format_feedback(fb: dict) -> str:
    """Render the trimmed feedback dict (see translate.trim_feedback) as prompt text."""
    lines: list[str] = []
    summary = fb.get("summary")
    if summary:
        lines.append("## Summary\n" + json.dumps(summary, indent=2))
    if fb.get("compile_errors"):
        lines.append(
            "## Compile errors (cargo build --release, full text)\n```text\n"
            + fb["compile_errors"].rstrip()
            + "\n```"
        )
    divs = fb.get("divergences") or []
    if divs:
        lines.append(
            f"## Divergences ({len(divs)} distinct patterns shown"
            + (f", {fb['divergence_total']} divergent inputs total" if fb.get("divergence_total") else "")
            + ")\nEach shows the raw input (as text, invalid UTF-8 shown as \\x escapes), "
            "the C driver line, and your Rust driver line."
        )
        for i, d in enumerate(divs, 1):
            lines.append(
                f"### Divergence {i}"
                + (f" (pattern seen {d['pattern_count']}x: {d['pattern']})" if d.get("pattern") else "")
                + "\n"
                + f"input: {json.dumps(d.get('input'), ensure_ascii=False)}\n"
                + f"c_output:    {json.dumps(d.get('c_output'), ensure_ascii=False)}\n"
                + f"rust_output: {json.dumps(d.get('rust_output'), ensure_ascii=False)}"
            )
    crashes = fb.get("crashes") or []
    if crashes:
        lines.append(f"## Crashes in the Rust build ({len(crashes)} shown)")
        for i, c in enumerate(crashes, 1):
            lines.append(
                f"### Crash {i} ({c.get('class')})\n"
                + f"input: {json.dumps(c.get('input'), ensure_ascii=False)}\n"
                + f"rust_behavior: {c.get('rust_behavior')}\n"
                + f"report:\n```text\n{(c.get('sanitizer_report') or '').rstrip()}\n```"
            )
    timeouts = fb.get("timeouts") or []
    if timeouts:
        lines.append(f"## Timeouts ({len(timeouts)} shown)")
        for t in timeouts:
            lines.append(f"input: {json.dumps(t.get('input'), ensure_ascii=False)} c: {t.get('c')} rust: {t.get('rust')}")
    if not lines:
        lines.append("No problems reported.")
    return "\n\n".join(lines)
