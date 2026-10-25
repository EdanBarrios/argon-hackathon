# pyexpat shim: libexpat's C API over Argon's Rust port

Builds `runs/shim/libtheseus_expat.dylib`, a drop-in libexpat 2.9.0 for CPython's unmodified `pyexpat` (contract: `contracts/pyexpat-shim.md`).

```
backend/ai/shim/build.sh       # sha256-checks the core, cargo build --release --offline, copies the dylib
backend/ai/shim/test/run.sh    # all 52 contract symbols exported? then the C smoke test
```

Set `THESEUS_ROOT=<main checkout>` when building from a worktree: the dylib goes to `$THESEUS_ROOT/runs/shim/`, and the smoke test reads `expat.h` from the vendored libexpat there. The build uses the host's default arch, so build it on the demo machine.

## Who wrote what

| File | Author | Check |
|---|---|---|
| `src/expat.rs` | Argon (libexpat port, `argon-fresh-b/round_4`), unchanged | `build.sh` fails unless sha256 is `b3a97cbf…bfccf` |
| `src/ffi.rs` | Argon, one call, used exactly as returned | `cmp src/ffi.rs argon/ffi.argon.rs` |
| `src/lib.rs`, `Cargo.toml`, `build.sh`, `test/` | us | |

`argon/` holds the call:
- `ask_argon.py`, plus the exact prompts that were sent (`prompt_system.txt`, `prompt_user.txt`):
  - `expat.h` and `expat_external.h`;
  - the core's public API;
  - the C driver that maps callbacks to events;
  - `XML_ErrorString` from xmlparse.c;
  - the contract.
- `meta.json`: `gemini-4-argon`, thinking medium, 127.5 s, 24,803 input and 15,873 output tokens, interaction id.
- Argon's reply (1,591 lines). It compiled on the first try, with no fixes.

## Results (`test/smoke.txt`, arm64 dev machine, 2026-10-25 14:30 PT)

- Exports: 52 of 52 contract symbols. `XML_ParserCreateNS` is also exported, though it isn't required.
- C smoke test: 14 of 14 checks pass:
  - events across a split `XML_Parse`;
  - a tag-mismatch error with the same code, line and column as the C driver (`ERROR 7 line 2 col 5`);
  - `XML_StopParser` inside a handler gives `XML_ERROR_ABORTED`, and no further events fire;
  - a handler cleared mid-parse is not called again;
  - a namespace parser parses a namespace-free document and returns a clean error on `xmlns`;
  - `XML_GetBuffer` / `XML_ParseBuffer`;
  - the version string `expat_2.9.0-theseus-argon`;
  - non-NULL error strings for codes 1..44;
  - a terminated feature list.
- `unsafe` appears only in `ffi.rs`. `expat.rs` is under `#[forbid(unsafe_code)]`, and `ffi.rs` contains no `unwrap`, `expect` or `panic!`. `parse` runs inside `catch_unwind`.
- `cargo clippy` on the crate: 88 warnings, all style or docs (82 missing `# Safety` sections, 5 manual NUL strings, 1 `too_many_arguments`). Not polished.

## Known limits

These are as the contract lists them; the pyexpat test counts on Edan's side will measure them.
- Events fire only at `isFinal`, after one whole-document parse.
- Line, column and byte positions are meaningful only after an error.
- Namespaced documents in a namespace parser get a clean `XML_ERROR_UNBOUND_PREFIX`.
- Handlers that have no Argon event never fire.
- On a parse error, no events are delivered (C expat would have delivered the ones before the error).
- Callbacks run while the replay loop holds the parser struct. `XML_StopParser` and the handler setters write to the same struct from inside a callback. That works in practice, but it is not a pattern Rust's aliasing rules bless.
