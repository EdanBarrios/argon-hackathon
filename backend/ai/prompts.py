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
7. Lint clean. The host also runs rustfmt and `cargo clippy` on cjson.rs, with the \
default lints plus clippy::indexing_slicing, unwrap_used, expect_used and panic. Write \
rustfmt layout. Read bytes with .get(i) or .get(a..b) and handle None exactly as C \
handles reaching the end of its buffer; never unwrap or expect. Rule 1 still comes \
first: a lint fix that changes any output is a failure.

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
                + (f"\nfirst_diff: output line {d['first_diff']['line']}: "
                   f"C {json.dumps(d['first_diff'].get('c'), ensure_ascii=False)} vs "
                   f"Rust {json.dumps(d['first_diff'].get('rust'), ensure_ascii=False)}"
                   if d.get("first_diff") else "")
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


CONTINUE_INPUT = (
    "Your reply was cut off by the output limit. Continue the file from the exact "
    "character where it stopped: do not repeat anything, do not restart the code "
    "block, and do not add commentary. Close the ```rust block when the file is complete."
)


def fresh_repair_input(reference_input: str, rust_file: str, prev_rust: str, feedback_text: str) -> str:
    """Repair as a new conversation: the full translate message, the previous file and the
    verifier's report. Used when follow-up turns fail on long files."""
    return (
        reference_input
        + f"\n\nA previous attempt at {rust_file} is below, followed by what the verifier and the "
        "compiler reported for it. Keep everything that works, fix every reported problem at its "
        "root cause (the C source is the specification), and do not break behavior that already "
        "matches.\n\n"
        + _fence("rust", f"{rust_file} (previous attempt)", prev_rust)
        + "\n" + feedback_text
        + f"\n\nReply with exactly one ```rust block containing the complete corrected {rust_file}, "
        "the whole file."
    )


PATCH_FORMAT = """\
Reply with SEARCH/REPLACE blocks only, no ```rust block and no whole file. Each block is:

<<<<<<< SEARCH
exact lines copied from the file above
=======
the lines that replace them
>>>>>>> REPLACE

Rules: SEARCH must match the current file character for character (indentation included) and \
must match exactly one place, so include a few unchanged neighbouring lines when the fix is a \
common line. Keep blocks small and give one block per separate fix. To delete lines leave the \
replacement empty; to insert, search for a neighbouring line and repeat it in the replacement. \
Do not elide code with comments like "rest unchanged". Blocks are applied in order, each to \
the file as the blocks before it left it."""


def patch_repair_input(rust_file: str, current_rust: str, feedback_text: str,
                       reference_input: str | None = None) -> str:
    """Repair as edits: the current file in full (the host may have reformatted it, so only
    this text is the one to search in), the verifier's report, and the SEARCH/REPLACE format.
    With reference_input (the translate message) it starts a new conversation."""
    return (
        (reference_input + "\n\n" if reference_input else "")
        + f"The verifier and the compiler reported the problems below for the current {rust_file}. "
        "Fix every reported problem at its root cause (the C source is the specification), and "
        "do not break behavior that already matches. Each divergence is one representative of a "
        "distinct failure pattern.\n\n"
        + _fence("rust", f"{rust_file} (current)", current_rust)
        + "\n" + feedback_text + "\n\n" + PATCH_FORMAT
    )


def polish_input(rust_file: str, current_rust: str, lint_text: str) -> str:
    """Polish-turn message: the build already matches on every loop input; clear lints only.
    The file is sent in full because the host may have reformatted it, so line numbers in
    the lint report refer to this text, not to the model's last reply."""
    return (
        f"Your {rust_file} now matches the C build on every input the verifier ran. Next, make it "
        "lint clean without changing behavior. Below is the current file (after the host ran "
        "rustfmt and cargo clippy --fix), then the lints still reported for it, grouped by lint "
        "with a few example sites each. Fix every site of each lint, not only the examples. For "
        "indexing_slicing use .get() and take the same path C takes at the end of its buffer. "
        "Do not change arithmetic to silence lints. If a site cannot be fixed without changing "
        "output, leave it as it is: the verifier reruns every input, and any changed output "
        "discards this file.\n\n"
        + _fence("rust", f"{rust_file} (current)", current_rust)
        + "\n" + lint_text
        + f"\n\nReply with exactly one ```rust block containing the complete {rust_file}, the whole file."
    )


# ---------------------------------------------------------------- libexpat

EXPAT_DIR = REPO_ROOT / "backend" / "harness" / "expat"
EXPAT_CRATE_DIR = EXPAT_DIR / "rust_crate"
EXPAT_DRIVER_C = EXPAT_DIR / "drivers" / "expat_driver.c"

EXPAT_SYSTEM_INSTRUCTION = """\
You are translating C to safe Rust for Theseus, a C to Rust migration tool whose \
output is checked by a deterministic differential verifier, not by you.

Task: translate libexpat R_2_9_0 (the whole expat/lib library you are given: \
xmlparse.c, xmlrole.c, xmltok.c, xmltok_impl.c, xmltok_ns.c, xcs.c and their \
headers) into ONE safe-Rust file, `expat.rs`, that implements INTERFACE.md exactly, \
for the configuration in the C driver: XML_ParserCreate(NULL) (no namespaces), a fixed \
zero hash salt, no external entities, parameter entity parsing NEVER, one \
XML_Parse(isFinal=1) call on the whole input. The file replaces `src/expat.rs` in a \
fixed crate whose `src/main.rs` you are given and cannot change.

Hard rules:
1. Match the C output byte for byte: the same events with the same bytes on success, \
and on failure the same XML_Error code, line and column. The verifier compares raw \
stdout of the C driver and your Rust driver on thousands of inputs.
2. No unsafe code. The build rejects any file containing the word "unsafe", even in \
a comment or string, so never write that word at all.
3. std only. No external crates, no FFI, no build scripts. It must build offline.
4. Never panic: no unwrap on input-dependent values, no unchecked indexing, no \
arithmetic overflow (use checked or wrapping ops where C relies on wrap). A panic \
counts as a crash.
5. No recursion per element or per entity level. Keep the open-element stack and the \
open-entity stack on the heap, as expat 2.7+ does; inputs nest 100,000 deep. A stack \
overflow counts as a crash.
6. Keep the public API from INTERFACE.md and the template: `pub enum Event` and \
`pub struct XmlError` with exactly the given variants and fields, and \
`pub fn parse(input: &[u8]) -> Result<Vec<Event>, XmlError>`.

How to structure the port. Translate the C structure, not your own XML parser: the \
verifier will find every place you deviate. Port xmltok (the encoding tables, the \
byte-type classification, the content/prolog/attribute-value/entity-value \
tokenizers for UTF-8, UTF-16LE/BE, Latin-1 and US-ASCII, and position tracking), \
xmlrole (the prolog state machine), and the xmlparse processors (prologProcessor, \
contentProcessor, doContent, storeAtts, entity storage and expansion, \
epilogProcessor, cdata handling) as state machines over byte offsets. Generic code \
over an encoding trait or enum is fine. Omit what is out of scope: namespaces, \
external entity parsing, suspend/resume, XML_GetBuffer/ParseBuffer, reparse deferral, \
custom memory suites, the allocation tracker, unknown-encoding handlers.

Known traps:
- Error codes are the enum XML_Error numbers in expat.h. Which code a document gets \
depends on which processor and tokenizer state sees the problem first; follow the C \
control flow, including XML_ERROR_NO_ELEMENTS (3) for empty or whitespace-only input, \
JUNK_AFTER_DOC_ELEMENT (9), UNCLOSED_TOKEN (5) versus PARTIAL_CHAR (6) at end of \
input, and INVALID_TOKEN (4).
- Error position: line is 1-based, column 0-based in characters of the input \
encoding (not bytes). expat reports the position of eventPtr, usually the start of \
the offending token, updated by the encoding's updatePosition (CR, LF and CRLF are \
one line break each). Reproduce where eventPtr points on every error path, including \
errors raised inside entity expansion (where the position is the reference in the \
document, not inside the entity text).
- Encoding detection: BOM sniffing and the encoding in the XML declaration, \
UTF-16 without BOM detected from the first bytes, Latin-1 and US-ASCII converted to \
UTF-8 for callbacks, any other name fails with UNKNOWN_ENCODING (18), a declaration \
that contradicts the detected encoding fails with INCORRECT_ENCODING (19). Callbacks \
always receive UTF-8. Invalid UTF-8 bytes are INVALID_TOKEN, and surrogates and \
non-characters follow xmltok's tables exactly.
- Character data: newline normalization (CR and CRLF become LF in text and in \
attribute values before normalization), character references, predefined entities, \
internal general entities expanded in content (events from the replacement text, \
markup included) and in attribute values. Text chunking does not matter.
- Attributes: duplicate attributes fail with DUPLICATE_ATTRIBUTE (8); values are \
normalized (whitespace characters to spaces; non-CDATA types declared in ATTLIST also \
collapse and trim spaces); defaulted attributes from ATTLIST come after the specified \
ones, in declaration order; the first ATTLIST declaration of an attribute wins.
- DTD: entity declarations (first declaration wins; EntityDecl events are reported \
for every declaration as expat does), the value after character-reference processing \
with entity references left as written, comments and PIs inside the DTD are reported \
through the same handlers, `<!ELEMENT>` and notation declarations are parsed without \
events. Undefined entity references: UNDEFINED_ENTITY (11) in a standalone document \
or one without external subset, or Skipped events where expat skips them. Follow \
expat's dtd->hasParamEntityRefs / standalone / keepProcessing logic.
- Recursive entity references fail with RECURSIVE_ENTITY_REF (12). Entity text \
must be properly nested (ASYNC_ENTITY, 13).
- Billion-laughs protection is required: port accountingDiffTolerated, \
accountingOnAbort, entityTrackingOnOpen/Close and every call site that accounts \
direct and indirect bytes, with the defaults from internal.h (maximum amplification \
100.0, activation threshold 8388608 bytes). A 9-level billion-laughs document must \
fail with AMPLIFICATION_LIMIT_BREACH (43) at the same line and column as C, and \
smaller expansions must succeed.
- `<a/>` gives Start then End. The XML declaration gives XmlDecl with standalone -1, \
0 or 1. A doctype gives DoctypeStart (has_internal_subset true when there is a [...] \
subset) and DoctypeEnd.

Reply format: think as long as you need, then reply with exactly one fenced code block \
tagged rust that contains the complete expat.rs, from the first line to the last. No \
other code blocks. Write compact code without long doc comments; this file is large. \
Never elide code with comments like "rest unchanged" or "same as above"; the block is \
compiled as is.
"""

# Library files sent to the model: expat/lib minus the platform entropy shims
# (the driver fixes the hash salt) and the Windows config.
_EXPAT_SKIP_PREFIXES = ("random_", "winconfig")


def load_expat_source(vendor_dir: Path) -> str:
    """expat/lib *.h then *.c as one labeled text blob. vendor_dir is vendor/libexpat."""
    lib = vendor_dir / "expat" / "lib"
    files = sorted(p for p in lib.iterdir()
                   if p.suffix in (".h", ".c") and not p.name.startswith(_EXPAT_SKIP_PREFIXES))
    files.sort(key=lambda p: (p.suffix != ".h", p.name))
    return "\n".join(_fence("c", f"expat/lib/{p.name}", p.read_text(encoding="utf-8", errors="replace"))
                     for p in files)


def _expat_reference(c_source: str) -> list[str]:
    interface = (EXPAT_CRATE_DIR / "INTERFACE.md").read_text(encoding="utf-8")
    main_rs = (EXPAT_CRATE_DIR / "src" / "main.rs").read_text(encoding="utf-8")
    stub_rs = (EXPAT_CRATE_DIR / "src" / "expat.rs").read_text(encoding="utf-8")
    driver_c = EXPAT_DRIVER_C.read_text(encoding="utf-8")
    return [
        "Translate libexpat into expat.rs.\n",
        "Below: the full expat/lib source, the Rust interface contract, the fixed Rust "
        "driver, the C driver it must match, and the template expat.rs (keep its Event "
        "and XmlError definitions).\n",
        c_source,
        _fence("markdown", "backend/harness/expat/rust_crate/INTERFACE.md", interface),
        _fence("rust", "backend/harness/expat/rust_crate/src/main.rs (fixed, do not change)", main_rs),
        _fence("c", "backend/harness/expat/drivers/expat_driver.c (the C side)", driver_c),
        _fence("rust", "backend/harness/expat/rust_crate/src/expat.rs (template to replace)", stub_rs),
    ]


def expat_translate_input(c_source: str) -> str:
    return "\n".join(_expat_reference(c_source) + [
        "Now write the complete expat.rs. Reply with exactly one ```rust block "
        "containing the whole file."
    ])


def expat_repair_input(feedback_text: str) -> str:
    return (
        "The verifier ran your previous expat.rs against the C build. Results below. "
        "Fix every reported problem. Each divergence is one representative of a "
        "distinct failure pattern; where shown, first_diff is the first output line "
        "where the C and Rust drivers differ. Fix the root cause in the ported C logic, "
        "not only that input. Do not break behavior that already matches.\n\n"
        + feedback_text
        + "\n\nReply with exactly one ```rust block containing the complete corrected "
        "expat.rs, the whole file."
    )


# ---------------------------------------------------------------- Lua 5.4

LUA_DIR = REPO_ROOT / "backend" / "harness" / "lua"
LUA_CRATE_DIR = LUA_DIR / "rust_crate"
LUA_DRIVER_C = LUA_DIR / "drivers" / "lua_driver.c"

LUA_SYSTEM_INSTRUCTION = """\
You are translating C to safe Rust for Theseus, a C to Rust migration tool whose \
output is checked by a deterministic differential verifier, not by you.

Task: translate the Lua 5.4.7 interpreter (the core and the base, coroutine, table, \
string, math and utf8 libraries you are given) into ONE safe-Rust file, `lua.rs`, \
that implements INTERFACE.md exactly, for the configuration in the C driver: one \
script read from stdin, loaded as a text chunk named "=input", run with only those \
libraries opened, a fixed hash seed and a fixed math.random seed. The file replaces \
`src/lua.rs` in a fixed crate whose `src/main.rs` you are given and cannot change.

Hard rules:
1. Match the C output byte for byte: everything the script prints, and on failure \
the same error line, message text included. The verifier compares raw stdout of the \
C driver and your Rust driver on thousands of scripts.
2. No unsafe code. The build rejects any file containing the word "unsafe", even in \
a comment or string, so never write that word at all.
3. std only. No external crates, no FFI, no build scripts. It must build offline.
4. Never panic: no unwrap on script-dependent values, no unchecked indexing, no \
unintended arithmetic overflow (use wrapping ops exactly where C relies on unsigned \
wrap, as lvm.c does with l_castU2S). A panic counts as a crash.
5. No Rust recursion per Lua call. A Lua function calling a Lua function must not \
recurse in Rust, exactly as luaV_execute reuses its loop with a new CallInfo. Only \
the places where C recurses through luaD_callnoyield (metamethods, pcall, C library \
functions calling back into Lua, the parser) may recurse in Rust, and they stop at \
LUAI_MAXCCALLS (200) with the same "C stack overflow" / "stack overflow" errors as C. \
A Rust stack overflow counts as a crash.
6. Keep the public API from INTERFACE.md and the template: \
`pub fn run(script: &[u8]) -> Vec<u8>` returns the bytes the C driver writes to stdout.
7. Lint clean. The host also runs rustfmt and `cargo clippy` on lua.rs, with the \
default lints plus clippy::indexing_slicing, unwrap_used, expect_used and panic. Write \
rustfmt layout. Read bytes and slots with .get(i) or .get(a..b) and handle None exactly \
as C handles reaching the end of its buffer; never unwrap or expect. Rule 1 still comes \
first: a lint fix that changes any output is a failure.

How to structure the port. Translate the C structure, not your own Lua: the verifier \
will find every place you deviate. Port llex (the lexer), lparser and lcode (the \
single-pass compiler to Lua 5.4 bytecode, with the same register allocation and \
constant folding), lvm (the interpreter over the same opcodes), ldo (calls, returns, \
varargs, protected calls, coroutines as separate stacks with resume and yield), \
ltable (array part plus hash part with the same node layout, main positions, free \
pointer, rehash sizes and border search for #), lstring (short strings interned with \
the seeded hash, long strings hashed lazily), lobject (number to string and string to \
number), ltm (metamethods and their lookup order), ldebug (error messages with variable \
names, from the same symbolic execution of the bytecode as getobjname / varinfo / \
funcnamefromcode) and the libraries. Represent GC objects as indices into arenas or as \
reference-counted cells; memory management may differ from C as long as nothing visible \
does. Errors are a Rust Result unwound to the nearest protected call, as luaD_throw \
longjmps to the nearest luaD_rawrunprotected.

Out of scope (the driver never opens or uses them): io, os, package/require, debug, \
string.dump and binary chunks, load of binary chunks, __gc metamethods, weak tables \
(__mode), collectgarbage options other than "collect", "step", "isrunning" and \
"incremental"/"generational" (return the same values C returns), the C API as an API.

Known traps:
- Number formatting: floats print with LUAI_NUMFFORMAT "%.14g" and get ".0" appended \
when the result looks like an integer (lobject.c tostringbuff); inf, -inf, nan and \
-nan print as C's printf does on this platform ("inf", "-inf", "nan", "-nan"). Rust \
has no %g: emulate it exactly (fixed versus exponential by the decimal exponent, \
trailing zeros removed, two-digit minimum exponent). string.format implements C \
printf conversions %d %i %u %c %x %X %o %e %E %f %F %g %G %a %A %q %s %% with flags, \
width and precision as lstrlib.c checks them.
- Integers and floats are distinct subtypes: 3 // 2 and 3.0 // 2 differ, math.type, \
integer division and modulo floor toward negative infinity, integer overflow wraps, \
float to integer conversion follows luaV_flttointns and lua_Number limits, comparisons \
between integers and floats are exact (LTnum / LEnum), for-loops precompute the \
iteration count as forprep does.
- String to number coercion: lobject.c l_str2int and l_str2d, hex integers wrap, hex \
floats, leading and trailing spaces, "0x" alone is not a number, and string arithmetic \
goes through the string metamethods in lstrlib.c (tonum / trymt).
- Table iteration order: next/pairs walk the array part then the hash nodes in node \
order. Mirror ltable.c exactly (mainposition hashing of every key type, collision \
chaining through the free pointer from the top, luaH_resize and computesizes / numusearray \
/ setlimittosize), with the fixed seed, so pairs prints the same order as C. # uses \
luaH_getn's border search, including tables with holes.
- Error messages: runtime errors are prefixed with "input:<line>: " from the current \
line info; messages name the variable as ldebug.c does ("attempt to index a nil value \
(global 'x')", "(local 'y')", "(field 'z')", "(upvalue 'w')", "(constant ...)", \
"attempt to call a nil value (method 'm')", "attempt to compare number with nil", \
"attempt to perform arithmetic on a string value", "number has no integer \
representation", "bad argument #1 to 'insert' (table expected, got nil)", and so on). \
Syntax errors use llex's luaX_syntaxerror format with the near-token text \
("input:3: '=' expected near 'x'", "<eof>"). error() with a table, nil or number value \
and the levels of error(msg, level) follow luaB_error and luaL_where.
- Strings are byte strings, not UTF-8: escapes \\ddd, \\xXX, \\z, \\u{XXX} up to \
2^31, long brackets with levels, and lstrlib's patterns (%a %d %l %s %u %w %x %p %c \
%g, sets, %b, %f, captures, position captures, anchors, the MAXCCALLS limit on \
pattern recursion) over bytes using C's ctype in the "C" locale.
- string.rep, string.format("%q"), table.concat, table.sort (the same quicksort with \
the same pivot choice, so comparator call order and "invalid order function for \
sorting" errors match), table.unpack limits, select('#'), select negative indices.
- math.random uses xoshiro256** seeded as math_randomseed does with the driver's seed; \
math.random(m, n) projects with the same project() algorithm. math.tointeger, \
math.ult, math.fmod (integer and float cases), math.abs of minint.
- tostring of a table, function, coroutine or userdata prints "<type>: " followed by \
an address; scripts never print addresses, so any address format is fine, but \
__tostring and __name are honored exactly as luaL_tolstring does.
- Coroutines: coroutine.wrap errors propagate with position info, resume of a dead or \
running coroutine, yield across pcall, coroutine.close, coroutine.isyieldable, \
coroutine.running in the main thread, and the "attempt to yield from outside a \
coroutine" error.
- goto/labels and their errors ("jumps into the scope of local"), <const> and <close> \
variables (__close runs on scope exit and on error), integer for-loops with overflow, \
varargs and table.pack.n, multiple assignment evaluation order, method calls, string \
methods through the string metatable.

Reply format: think as long as you need, then reply with exactly one fenced code block \
tagged rust that contains the complete lua.rs, from the first line to the last. No \
other code blocks. Write compact code without long doc comments; this file is large. \
Never elide code with comments like "rest unchanged" or "same as above"; the block is \
compiled as is.
"""

# Files sent to the model: lua-5.4.7/src minus the standalone programs, binary chunks,
# and the io/os/package/debug libraries the driver never opens.
_LUA_SKIP = {"lua.c", "luac.c", "ldump.c", "lundump.c", "lundump.h", "loadlib.c",
             "liolib.c", "loslib.c", "ldblib.c"}


def load_lua_source(vendor_dir: Path) -> str:
    """lua-5.4.7/src *.h then *.c as one labeled text blob. vendor_dir is vendor/lua-5.4.7."""
    src = vendor_dir / "src"
    files = sorted(p for p in src.iterdir() if p.suffix in (".h", ".c") and p.name not in _LUA_SKIP)
    files.sort(key=lambda p: (p.suffix != ".h", p.name))
    return "\n".join(_fence("c", f"lua-5.4.7/src/{p.name}", p.read_text(encoding="utf-8", errors="replace"))
                     for p in files)


def _lua_reference(c_source: str) -> list[str]:
    interface = (LUA_CRATE_DIR / "INTERFACE.md").read_text(encoding="utf-8")
    main_rs = (LUA_CRATE_DIR / "src" / "main.rs").read_text(encoding="utf-8")
    stub_rs = (LUA_CRATE_DIR / "src" / "lua.rs").read_text(encoding="utf-8")
    driver_c = LUA_DRIVER_C.read_text(encoding="utf-8")
    return [
        "Translate the Lua 5.4.7 interpreter into lua.rs.\n",
        "Below: the Lua source in scope, the Rust interface contract, the fixed Rust "
        "driver, the C driver it must match, and the template lua.rs.\n",
        c_source,
        _fence("markdown", "backend/harness/lua/rust_crate/INTERFACE.md", interface),
        _fence("rust", "backend/harness/lua/rust_crate/src/main.rs (fixed, do not change)", main_rs),
        _fence("c", "backend/harness/lua/drivers/lua_driver.c (the C side)", driver_c),
        _fence("rust", "backend/harness/lua/rust_crate/src/lua.rs (template to replace)", stub_rs),
    ]


def lua_translate_input(c_source: str) -> str:
    return "\n".join(_lua_reference(c_source) + [
        "Now write the complete lua.rs. Reply with exactly one ```rust block "
        "containing the whole file."
    ])


def lua_repair_input(feedback_text: str) -> str:
    return (
        "The verifier ran your previous lua.rs against the C build. Results below. "
        "Fix every reported problem. Each divergence is one representative of a "
        "distinct failure pattern; where shown, first_diff is the first output line "
        "where the C and Rust drivers differ. Fix the root cause in the ported C logic, "
        "not only that script. Do not break behavior that already matches.\n\n"
        + feedback_text
        + "\n\nReply with exactly one ```rust block containing the complete corrected "
        "lua.rs, the whole file."
    )
