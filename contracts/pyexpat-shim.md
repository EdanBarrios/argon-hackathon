# Contract: pyexpat on Argon's Rust expat (C-ABI shim)

Goal: run CPython 3.13.5's own, unmodified `Modules/pyexpat.c` (and through it
ElementTree, minidom, SAX and the stdlib XML test suites) on top of Argon's safe-Rust
libexpat translation. Two sides:

- **ai (Brendan)**: Rust C-ABI shim dylib exporting the symbols below.
- **systems (Edan)**: builds pyexpat.c and _elementtree.c against it, runs the tests.

## The dylib

| Item | Value |
|---|---|
| Path (main checkout) | `runs/shim/libtheseus_expat.dylib` (gitignored) |
| Install name | `@rpath/libtheseus_expat.dylib` (`-C link-arg=-Wl,-install_name,@rpath/libtheseus_expat.dylib`) |
| Arch | host default arch of the demo machine (x86_64 macOS, `/opt/anaconda3` Python 3.13.5). Brendan delivers source + `build.sh`; the dylib is built on the demo machine, not copied in. |
| ABI headers | libexpat 2.9.0 `expat.h` / `expat_external.h` from `backend/harness/expat/vendor/libexpat/expat/lib/` |
| Build flags assumed | `XML_DTD`, `XML_NS`, `XML_GE=1`; `XML_Char` = `char` (UTF-8); `XML_Size`/`XML_Index` = `long` |
| Build | `backend/ai/shim/build.sh` (branch `feat/ai-pyexpat-shim`) |

Symbols must be exported unmangled (`#[no_mangle] pub extern "C"`), with the exact
signatures from expat.h 2.9.0. Check: `nm -gU runs/shim/libtheseus_expat.dylib | grep ' _XML_'`.

## Required symbols (52)

Exact output of `nm -u pyexpat.o | grep XML_` for v3.13.5 `Modules/pyexpat.c`
compiled with `-DXML_DTD -DXML_NS -DXML_GE=1` against expat.h 2.9.0. Every one must
exist or the module fails to load. Ones we knowingly do not support may be stubs
(setters that store and ignore, getters that return 0/NULL) as noted below.

```
XML_ErrorString XML_ExpatVersion XML_ExpatVersionInfo XML_ExternalEntityParserCreate
XML_FreeContentModel XML_GetBase XML_GetBuffer XML_GetCurrentByteIndex
XML_GetCurrentColumnNumber XML_GetCurrentLineNumber XML_GetErrorCode
XML_GetFeatureList XML_GetInputContext XML_GetSpecifiedAttributeCount XML_Parse
XML_ParseBuffer XML_ParserCreate_MM XML_ParserFree XML_SetAttlistDeclHandler
XML_SetBase XML_SetCharacterDataHandler XML_SetCommentHandler XML_SetDefaultHandler
XML_SetDefaultHandlerExpand XML_SetElementDeclHandler XML_SetElementHandler
XML_SetEncoding XML_SetEndCdataSectionHandler XML_SetEndDoctypeDeclHandler
XML_SetEndElementHandler XML_SetEndNamespaceDeclHandler XML_SetEntityDeclHandler
XML_SetExternalEntityRefHandler XML_SetHashSalt XML_SetNamespaceDeclHandler
XML_SetNotStandaloneHandler XML_SetNotationDeclHandler XML_SetParamEntityParsing
XML_SetProcessingInstructionHandler XML_SetReparseDeferralEnabled
XML_SetReturnNSTriplet XML_SetSkippedEntityHandler XML_SetStartCdataSectionHandler
XML_SetStartDoctypeDeclHandler XML_SetStartElementHandler
XML_SetStartNamespaceDeclHandler XML_SetUnknownEncodingHandler
XML_SetUnparsedEntityDeclHandler XML_SetUserData XML_SetXmlDeclHandler
XML_StopParser XML_UseForeignDTD
```

Notes that matter for pyexpat behaviour:

- **Namespace parsers come through `XML_ParserCreate_MM`, not `XML_ParserCreateNS`.**
  Verified: `XML_ParserCreateNS` is not referenced by pyexpat.c or _elementtree.c in
  v3.13.5. `pyexpat.ParserCreate(namespace_separator=...)` and minidom/expatbuilder pass
  the separator as `XML_ParserCreate_MM`'s third arg; `_elementtree.c:3673` calls
  `ParserCreate_MM(encoding, &ExpatMemoryHandler, "}")` through the capsule, so
  **every ElementTree parser is a namespace parser**. Agreed behavior (option c): when
  `nsSep != NULL`, the parser returns a clean `XML_STATUS_ERROR` as soon as the document
  has an `xmlns`/`xmlns:*` attribute or a prefixed element/attribute name; namespace-free
  documents parse normally and names are passed through unchanged. Exporting
  `XML_ParserCreateNS` as well is harmless (other consumers may want it) but not required.
- The memsuite argument may be ignored.
- **Replay discipline**: re-read the handler pointer from the parser struct before every
  event (pyexpat clears/swaps handlers mid-parse), and check a stopped flag after every
  callback. pyexpat calls `XML_StopParser` when a Python handler raises; the shim must
  then stop delivering events and return `XML_STATUS_ERROR` with
  `XML_GetErrorCode == XML_ERROR_ABORTED`.
- Handlers receive the `userData` set by `XML_SetUserData`. Attribute arrays passed to
  start-element are NULL-terminated `name, value, name, value, ..., NULL`.
- `XML_ExpatVersion()` must return a string that identifies the shim (e.g.
  `"expat_2.9.0-theseus-argon"`), and `XML_ExpatVersionInfo()` should return `{2,9,0}`,
  so `pyexpat.EXPAT_VERSION` proves which library is loaded.
- `XML_ErrorString(code)` must return a non-NULL static string for every code pyexpat
  can see (it builds `pyexpat.errors` from codes 1..N at import time; NULL is fine
  only past the last code).
- `XML_GetFeatureList()` must return a list terminated by `XML_FEATURE_END`.
- `XML_GetErrorCode`, `XML_GetCurrentLineNumber/ColumnNumber/ByteIndex` must be
  correct after a failed `XML_Parse`; pyexpat builds `ExpatError` from them.
- `XML_GetBuffer(p, len)` + `XML_ParseBuffer(p, len, isFinal)` are used by
  `ParseFile`; the shim must hand back a writable buffer of at least `len` bytes.
- `XML_SetHashSalt` is called unconditionally; a no-op is acceptable.
- `XML_FreeContentModel` is only reached if an element-decl handler fires.

## Rules

1. **Argon's core `expat.rs` stays byte-identical** to the Argon-produced artifact it
   was copied from, checked by sha256 in `build.sh`. Source: the final passing round of
   `backend/ai/results/expat-long-decode/argon-fresh-b/` (the one that replaces an
   `unwrap` with an error return). If that file is not on main yet, fall back to
   `argon-fresh-a/round_3/expat.rs` and say so. No edits, not even formatting.
2. **`unsafe` lives only in the FFI layer** (the shim crate's `ffi.rs` or equivalent):
   raw pointer conversion, `CStr`, callbacks into C, the `#[no_mangle]` exports. The
   core parser stays `#![forbid(unsafe_code)]`-clean.
3. pyexpat.c and _elementtree.c are compiled from CPython v3.13.5 unmodified.
4. Results are reported as measured: tests run/passed/failed/errored/skipped per suite,
   ours vs the stock pyexpat baseline. No inflation.

## Symbols checked against 2.9.0's expat.h

Present in 2.9.0 but **not** referenced by v3.13.5 pyexpat.c, so not required:
`XML_ParserCreateNS`, `XML_SetHashSalt16Bytes`,
`XML_SetBillionLaughsAttackProtectionMaximumAmplification`,
`XML_SetBillionLaughsAttackProtectionActivationThreshold`,
`XML_SetAllocTrackerMaximumAmplification`, `XML_SetAllocTrackerActivationThreshold`.
(Later 3.13.x/3.14 pyexpat references some of these; we pin v3.13.5.)

## Knowingly unsupported (expected test failures)

- Namespaced documents in a namespace parser (option c above: clean error). This
  includes any ElementTree input with `xmlns`.
- Positions during callbacks: `CurrentLineNumber`/`ColumnNumber`/`ByteIndex` are only
  meaningful after an error.
- Incremental output: `iterparse` / `XMLPullParser` yield events only at close.
- `GetSpecifiedAttributeCount` / `specified_attributes` count defaulted attributes too.
- Handlers with no corresponding Argon event never fire.

- `XML_SetReturnNSTriplet` and namespace decl handlers.
- Incremental parsing semantics: the shim may buffer all input until `isFinal` and
  parse once, so handlers fire late and position/`buffer_text` tests may differ.
- `XML_StopParser` / resume and `XML_SetReparseDeferralEnabled` (stored, ignored).
- External entities (`XML_ExternalEntityParserCreate` may return NULL,
  `XML_SetParamEntityParsing` stored, ignored), `XML_UseForeignDTD`.
- DTD content models (`XML_SetElementDeclHandler` may never fire), attlist/entity/
  notation decl handlers where the Rust core does not surface them.
- Unknown encodings (`XML_SetUnknownEncodingHandler` ignored), `XML_GetInputContext`
  may return NULL.

## Build and run

Rust side (ai): `backend/ai/shim/build.sh` -> `runs/shim/libtheseus_expat.dylib`.

Python side (systems, branch `feat/systems-pyexpat`):

```
backend/harness/pyexpat/fetch_cpython.sh                 # v3.13.5 sources + tests
backend/harness/pyexpat/build_pyexpat.sh runs/shim/libtheseus_expat.dylib runs/pyexpat/build
PYTHONPATH=runs/pyexpat/build python3 -c 'import pyexpat; print(pyexpat.__file__, pyexpat.EXPAT_VERSION)'
backend/harness/pyexpat/run_tests.sh runs/pyexpat/build   # ours + baseline summary
```

`build_pyexpat.sh` links the dylib by path with `-Wl,-rpath,<dylib dir>`, so the
`@rpath` install name resolves without `DYLD_LIBRARY_PATH`. It also rebuilds
`_elementtree.c` against the same expat.h, because the stock `_elementtree` rejects
a pyexpat capsule whose `XML_MINOR_VERSION` differs from its own (2.7 vs 2.9).
Symbol list regeneration: `build_pyexpat.sh --object-only <dir>; nm -u <dir>/pyexpat.o | grep XML_`.

## Scope claim

"pyexpat and SAX on Argon's libexpat, plus ElementTree on namespace-free XML."
