# Rust translation interface

Argon produces one file, `cjson.rs`. `scripts/build_rust.sh <cjson.rs>` drops it into this crate as `src/cjson.rs` next to the fixed driver `src/main.rs` and runs `cargo build --release`.

## Signatures (must exist, public)

```rust
pub enum Value { /* any variants; the template mirrors cJSON types */ }
pub struct ParseError { pub offset: Option<usize> /* more fields allowed */ }
pub fn parse(input: &[u8]) -> Result<Value, ParseError>;
pub fn print_unformatted(value: &Value) -> Vec<u8>;
```

String data is `Vec<u8>`, not `String`. cJSON does not validate UTF-8.

## Output contract

The driver reads all of stdin and prints exactly one line, then exits 0:

| Case | Line |
|---|---|
| `Ok(v)` | `OK\t` + `print_unformatted(&v)` (cut at first NUL, like `fputs`) + `\n` |
| `Err(e)`, `e.offset == Some(n)`, `n <= len` | `PARSE_ERROR at <n>\n` |
| `Err(e)`, otherwise | `PARSE_ERROR\n` |

This matches `backend/harness/drivers/cjson_driver.c` byte for byte. That driver calls `cJSON_ParseWithLengthOpts(input + "\0", len + 1, &end, 1)` and prints the offset of `cJSON_GetErrorPtr()`. `parse` must reproduce both the accept/reject decision and the error offset.

## Rules

- No unsafe code. The crate has `#![forbid(unsafe_code)]`, and the build script rejects any file containing the word `unsafe`, even in comments.
- No escape hatches. The build script also rejects `std::process`, `std::fs`, `std::net`, `std::env`, `std::os`, `Command`, `include!`/`include_str!`/`include_bytes!`, `extern crate`, `#[path]` and out-of-line `mod name;` (outside comments). The module must translate cJSON, not run the C driver or read files. Fixtures in `fixtures/` are checked by `scripts/test_build_guard.sh`.
- No external crates, no FFI. std only, builds offline.
- Nesting limit 1000, mirroring `CJSON_NESTING_LIMIT`. Deeper input is a parse error, never a stack overflow.
- Never panic. A panic or abort counts as a crash.
- Output must match the C build byte for byte: number formatting, string escapes, key order, duplicate keys.

## Build script result

`scripts/build_rust.sh path/to/cjson.rs [build_id]` prints one JSON line:

- `{"ok":true,"binary":"<abs path>","build_seconds":N}`
- `{"ok":false,"errors":"<first ~60 lines of cargo stderr>","build_seconds":N}` with exit 1
