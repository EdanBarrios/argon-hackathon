# libexpat Rust translation interface

Argon produces one file, `expat.rs`: a safe-Rust translation of libexpat R_2_9_0 (`expat/lib/xmlparse.c`, `xmlrole.c`, `xmltok.c`, `xmltok_impl.c`, `xmltok_ns.c`, `xcs.c` and their headers). `scripts/build_rust_expat.sh <expat.rs>` drops it into this crate as `src/expat.rs` next to the fixed driver `src/main.rs` and runs `cargo build --release`.

## Signatures (must exist, public, exactly these shapes)

```rust
pub enum Event {
    XmlDecl { version: Option<Vec<u8>>, encoding: Option<Vec<u8>>, standalone: i32 },
    DoctypeStart { name: Vec<u8>, sysid: Option<Vec<u8>>, pubid: Option<Vec<u8>>, has_internal_subset: bool },
    DoctypeEnd,
    EntityDecl { name: Vec<u8>, is_pe: bool, value: Option<Vec<u8>>,
                 sysid: Option<Vec<u8>>, pubid: Option<Vec<u8>>, notation: Option<Vec<u8>> },
    Start { name: Vec<u8>, attrs: Vec<(Vec<u8>, Vec<u8>)> },
    End { name: Vec<u8> },
    Text(Vec<u8>),
    Pi { target: Vec<u8>, data: Vec<u8> },
    Comment(Vec<u8>),
    CdataStart,
    CdataEnd,
    Skipped { name: Vec<u8>, is_pe: bool },
}
pub struct XmlError { pub code: u32, pub line: u64, pub col: u64 }
pub fn parse(input: &[u8]) -> Result<Vec<Event>, XmlError>;
```

The template `src/expat.rs` already declares `Event` and `XmlError` with these fields. Keep them; add any private items you need.

## What `parse` must reproduce

`parse(input)` must behave like this C sequence (see `backend/harness/expat/drivers/expat_driver.c`):

```c
XML_Parser p = XML_ParserCreate(NULL);          /* no namespaces */
XML_SetHashSalt16Bytes(p, zero_salt);          /* fixed salt */
/* handlers: XmlDecl, DoctypeDecl start/end, EntityDecl, Element start/end,
   CharacterData, ProcessingInstruction, Comment, CdataSection start/end,
   SkippedEntity. No default handler, no external entity ref handler,
   no unknown encoding handler, parameter entity parsing left at NEVER. */
XML_Parse(p, input, len, /*isFinal=*/1);       /* one call, whole input */
```

- `Ok(events)`: every callback expat made, in order, mapped 1:1 to `Event`. Strings are the exact UTF-8 bytes expat passes to the callback (expat always reports UTF-8, whatever the input encoding).
- `Err(e)`: `XML_Parse` returned `XML_STATUS_ERROR`. Events produced before the error are discarded by both drivers, so only `code`, `line` and `col` matter.

### Error fields

- `code` = `XML_GetErrorCode(p)`, the numeric value of `enum XML_Error` in `expat/lib/expat.h`. Use the same numbers. The ones the inputs hit most:

| code | name | code | name |
|---|---|---|---|
| 2 | XML_ERROR_SYNTAX | 11 | XML_ERROR_UNDEFINED_ENTITY |
| 3 | XML_ERROR_NO_ELEMENTS | 12 | XML_ERROR_RECURSIVE_ENTITY_REF |
| 4 | XML_ERROR_INVALID_TOKEN | 13 | XML_ERROR_ASYNC_ENTITY |
| 5 | XML_ERROR_UNCLOSED_TOKEN | 14 | XML_ERROR_BAD_CHAR_REF |
| 6 | XML_ERROR_PARTIAL_CHAR | 15 | XML_ERROR_BINARY_ENTITY_REF |
| 7 | XML_ERROR_TAG_MISMATCH | 16 | XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF |
| 8 | XML_ERROR_DUPLICATE_ATTRIBUTE | 17 | XML_ERROR_MISPLACED_XML_PI |
| 9 | XML_ERROR_JUNK_AFTER_DOC_ELEMENT | 18 | XML_ERROR_UNKNOWN_ENCODING |
| 10 | XML_ERROR_PARAM_ENTITY_REF | 19 | XML_ERROR_INCORRECT_ENCODING |
| 20 | XML_ERROR_UNCLOSED_CDATA_SECTION | 22 | XML_ERROR_NOT_STANDALONE |
| 30 | XML_ERROR_XML_DECL | 32 | XML_ERROR_PUBLICID |
| 43 | XML_ERROR_AMPLIFICATION_LIMIT_BREACH | 1 | XML_ERROR_NO_MEMORY |

- `line` = `XML_GetCurrentLineNumber64(p)`: 1-based. CR, LF and CRLF each count as one line break.
- `col` = `XML_GetCurrentColumnNumber64(p)`: 0-based, counted in characters of the input encoding, not bytes (`<a>ééé</b>` fails at col 8).
- Both are taken at the error position expat reports (the start of the offending token in most cases). Reproduce expat's position tracking, not your own idea of where the error is.

## Output contract (fixed driver, for reference)

`src/main.rs` prints, on `Ok`, one line per event and then `OK`; on `Err`, only `ERROR <code> line <line> col <col>`. Adjacent `Text` events are joined into one `TEXT` line and empty ones are dropped, so you may emit character data in any chunking. Strings print as `"..."` with bytes 0x20..0x7E literal except `"` and `\` (escaped as `\"`, `\\`), and every other byte as `\xNN`. `None` prints as `null`. Example (input `<a x="1">t<b/></a>`):

```
START "a" "x"="1"
TEXT "t"
START "b"
END "b"
END "a"
OK
```

Event details that matter:
- `Start.attrs`: specified attributes in document order, then attributes defaulted from `<!ATTLIST>`, with expat's value normalization (whitespace to spaces; non-CDATA types also collapse spaces).
- `<a/>` produces `Start` then `End`.
- `EntityDecl`: internal entity has `value: Some(..)` (the literal value after character-reference processing, entity references left as written) and `None` for the rest; external entity has `value: None`.
- `standalone`: -1 absent, 0 `no`, 1 `yes`.
- `Skipped`: a reference to an undeclared entity that expat skips instead of failing (for example when the document has an external subset it does not load).

## Scope

- Namespaces: off (`XML_ParserCreate`, not `XML_ParserCreateNS`). `xmlns` attributes are plain attributes. You may omit namespace code.
- External entities and external DTD subsets are never loaded. Parameter entity parsing stays at `XML_PARAM_ENTITY_PARSING_NEVER`.
- Internal DTD: entity declarations and expansion in content and attribute values, `<!ATTLIST>` defaults, `<!ELEMENT>` parsing (no callback), comments and PIs inside the DTD (DTD comments and PIs are reported through the same handlers).
- Encodings: whatever expat supports built in: UTF-8, UTF-16 (LE/BE, with or without BOM detection rules), ISO-8859-1, US-ASCII. Any other declared encoding fails with `XML_ERROR_UNKNOWN_ENCODING` (no unknown encoding handler).
- Not exercised, may be omitted: suspend/resume, `XML_ParseBuffer`, `XML_GetBuffer`, reparse deferral, the Properties API, `XML_ParserReset`, `XML_ExternalEntityParserCreate`, custom memory suites, `XML_GetInputContext`.

## Security limits you must preserve

- **Billion-laughs protection** (`XML_ERROR_AMPLIFICATION_LIMIT_BREACH`, code 43): expat's accounting of direct vs indirect (entity-expanded) bytes with defaults `EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT = 100.0` and `..._ACTIVATION_THRESHOLD_DEFAULT = 8388608` (8 MiB) from `internal.h`. Port the accounting in `accountingDiffTolerated` and its callers exactly: a classic 9-level billion-laughs document must fail with code 43 at the same line and column as C, and smaller expansions under the threshold must succeed.
- **Entity recursion**: a self-referencing or mutually recursive entity fails with code 12, never loops or overflows the stack.
- **No recursion depth limit exists** in expat for element nesting, and the C build handles 100,000 nested elements. Your port must too: keep the element stack on the heap, never recurse per element. Stack overflow is a crash.
- **Allocation tracker** (`EXPAT_ALLOC_TRACKER_*`, 64 MiB activation, 100x): not reproducible in Rust byte for byte. The harness inputs stay far below its threshold; you may omit it, but never let memory grow without bound on small inputs.
- **Hash salt**: the driver fixes the salt, so hashing is deterministic. Results must never depend on hash iteration order (expat's never do).

## Rules

- No unsafe code. The crate has `#![forbid(unsafe_code)]`, and the build script rejects any file containing the word `unsafe`, even in comments.
- No external crates, no FFI. std only, builds offline.
- Never panic: index with checks, no `unwrap` on input-dependent values, no integer overflow in debug-style arithmetic (use checked or wrapping ops where C relies on wrap). A panic counts as a crash.
- Output must match the C build byte for byte, including error codes and positions.

## Build script result

`scripts/build_rust_expat.sh path/to/expat.rs [build_id]` builds in `runs/build-expat/<build_id>/` and prints one JSON line:

- `{"ok":true,"binary":"<abs path>","build_seconds":N}`
- `{"ok":false,"errors":"<first ~150 lines of cargo stderr>","build_seconds":N}` with exit 1

Then verify with:

```bash
backend/harness/verify.sh --c backend/harness/expat/build/expat_driver --rust <binary> \
  --inputs backend/harness/expat/inputs --timeout 20
```
