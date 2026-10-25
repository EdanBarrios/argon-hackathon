"""Ask Argon to write ffi.rs, the C-ABI bridge from libexpat 2.9.0's API to Argon's expat.rs.

One call through ai.translate._run, with a shim-specific system instruction. Saves
the reply's Rust as ffi.argon.rs plus meta.json (usage, seconds, interaction id).

usage (from backend/): uv run python <this file> <repo root> <worktree root> <out dir>
"""
import dataclasses, json, re, sys
from pathlib import Path

from ai import translate
from ai.targets import EXPAT

repo, wt, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
out.mkdir(parents=True, exist_ok=True)
lib = repo / "backend/harness/expat/vendor/libexpat/expat/lib"
core = wt / "backend/ai/results/expat-long-decode/argon-fresh-b/round_4/expat.rs"
core_lines = core.read_text().splitlines()
api = "\n".join(core_lines[:90])  # Event, XmlError, error-code consts
api += "\n...\n" + next(l for l in core_lines if l.startswith("pub fn parse(")) + " ... }\n"
xmlparse = (lib / "xmlparse.c").read_text()
m = re.search(r"const XML_LChar \*XMLCALL\nXML_ErrorString\(enum XML_Error code\) \{.*?\n\}\n", xmlparse, re.S)
error_string = m.group(0) if m else ""
contract = (wt / "contracts/pyexpat-shim.md").read_text() if (wt / "contracts/pyexpat-shim.md").exists() else ""

SYSTEM = """\
You are writing the C-ABI bridge for Theseus, a C to Rust migration tool. An earlier \
model call translated libexpat 2.9.0 into one safe-Rust module, `expat.rs`, whose \
public API you are given. That module is verified against C and must not change. \
Your job: write ONE file, `ffi.rs`, that exports libexpat's C API (the exact symbols \
and signatures in expat.h) on top of `crate::expat::parse`, so CPython's unmodified \
pyexpat module can link to it as a drop-in libexpat.

Hard rules:
1. The crate is `lib.rs` = `#[forbid(unsafe_code)] pub mod expat; pub mod ffi;`, \
crate-type cdylib, std only, no external crates, builds offline with Rust 1.87. \
`unsafe` is allowed in ffi.rs only, and only where needed (raw pointers, CStr, calling \
C function pointers, exports).
2. Every exported function is `#[no_mangle] pub unsafe extern "C" fn XML_...` (or \
`pub extern "C" fn` when no pointer is dereferenced) with exactly the C signature from \
expat.h, built with XML_DTD, XML_NS, XML_GE=1, XML_Char = c_char (UTF-8), \
XML_LChar = c_char, XML_Size = c_ulong, XML_Index = c_long, XML_Bool = c_uchar, \
enums as c_int. Handler typedefs become `Option<unsafe extern "C" fn(...)>`.
3. Never panic or unwind across the FFI boundary. Wrap the call to `parse` in \
std::panic::catch_unwind and turn a panic into XML_STATUS_ERROR with \
XML_ERROR_UNEXPECTED_STATE. Null parser pointers return an error or default value.
4. Export every symbol in the contract's required list. Where the core has no support, \
setters store and ignore, getters return 0 / NULL, as the contract says.

Design:
- XML_ParserCreate / XML_ParserCreate_MM / XML_ParserCreateNS allocate a Box'd \
parser struct (handlers, userData, input buffer, a separate GetBuffer buffer, error \
code/line/col, stopped flag, namespace separator, current attribute count) and return \
it as an opaque pointer; XML_ParserFree drops it. Ignore the memory suite.
- XML_Parse(p, s, len, isFinal) appends to the buffer; when isFinal == 0 return \
XML_STATUS_OK. When final: call parse(&buffer). On Err(e) store code/line/col and \
return XML_STATUS_ERROR. On Ok(events) replay the events in order into the C handlers.
- Namespace parsers (nsSep != NULL): before replaying anything, if any Start event \
has a name containing ':' or any attribute named `xmlns`, starting with `xmlns:` or \
containing ':', deliver no events and fail with XML_ERROR_UNBOUND_PREFIX. Otherwise \
names pass through unchanged.
- Replay discipline: re-read each handler pointer from the parser struct right before \
each event (the caller clears and swaps handlers mid-parse). After every callback, \
check the stopped flag set by XML_StopParser; if set, stop and return \
XML_STATUS_ERROR with error code XML_ERROR_ABORTED. Strings passed to handlers are \
NUL-terminated copies kept alive for the duration of the callback. Character data, \
comments, entity values are passed with explicit lengths as the typedefs say. \
Start-element gets a NULL-terminated `name, value, name, value, ..., NULL` array; \
record 2 * attr count for XML_GetSpecifiedAttributeCount. Map each Event variant to \
the handler the C driver (given below) shows it comes from. Handlers with no Event \
(default, notation, unparsed entity, element/attlist decl, namespace decl, not \
standalone, external entity ref) are stored and never called.
- XML_GetBuffer(p, len) returns a writable buffer of at least len bytes owned by the \
parser; XML_ParseBuffer(p, len, isFinal) appends its first len bytes, then behaves \
like XML_Parse.
- XML_GetErrorCode / XML_GetCurrentLineNumber / XML_GetCurrentColumnNumber / \
XML_GetCurrentByteIndex / XML_GetErrorLineNumber / XML_GetErrorColumnNumber return \
the stored error state (line 1, column 0, byte index -1 when there is none).
- XML_ErrorString(code): a static NUL-terminated string for every code, copied from \
the C XML_ErrorString below; NULL for 0 and past the last code.
- XML_ExpatVersion returns "expat_2.9.0-theseus-argon"; XML_ExpatVersionInfo returns \
{2, 9, 0}; XML_GetFeatureList returns a static list with XML_FEATURE_SIZEOF_XML_CHAR, \
XML_FEATURE_SIZEOF_XML_LCHAR, XML_FEATURE_DTD, XML_FEATURE_NS, terminated by \
XML_FEATURE_END with a NULL name.
- XML_ExternalEntityParserCreate and XML_GetInputContext return NULL. \
XML_FreeContentModel, XML_SetHashSalt, XML_SetReturnNSTriplet, \
XML_SetReparseDeferralEnabled, XML_UseForeignDTD, XML_SetParamEntityParsing, \
XML_SetEncoding, XML_SetBase store or ignore and return success.

Reply with the complete ffi.rs in one ```rust block and nothing else.
"""

user = (
    "## expat.h\n```c\n" + (lib / "expat.h").read_text() + "\n```\n\n"
    "## expat_external.h\n```c\n" + (lib / "expat_external.h").read_text() + "\n```\n\n"
    "## Public API of crate::expat (expat.rs, first 90 lines and the entry point)\n```rust\n" + api + "\n```\n\n"
    "## The C driver used to verify expat.rs: which C callback each Event comes from\n```c\n"
    + (repo / "backend/harness/expat/drivers/expat_driver.c").read_text() + "\n```\n\n"
    "## XML_ErrorString from xmlparse.c\n```c\n" + error_string + "\n```\n\n"
    "## Contract\n" + contract + "\n\nWrite ffi.rs now."
)
(out / "prompt_system.txt").write_text(SYSTEM)
(out / "prompt_user.txt").write_text(user)
target = dataclasses.replace(EXPAT, system_instruction=SYSTEM, max_output_tokens=131072,
                             max_wait_seconds=1800)
r = translate._run("gemini-4-argon", user, "medium", None, target)
(out / "ffi.argon.rs").write_text(r.rust_source)
(out / "meta.json").write_text(json.dumps({
    "model": r.model, "interaction_id": r.interaction_id, "usage": r.usage, "seconds": r.seconds,
    "thinking_level": r.thinking_level, "notes": r.notes, "lines": r.rust_source.count("\n")}, indent=2))
print(json.dumps({"seconds": r.seconds, "usage": r.usage, "lines": r.rust_source.count("\n")}))
