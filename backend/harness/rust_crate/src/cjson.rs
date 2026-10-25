//! cJSON v1.7.19 parse + print, translated to safe Rust.
//!
//! THIS FILE IS A STUB. Argon replaces it with a full translation.
//!
//! CONTRACT (the fixed driver in main.rs depends on exactly this):
//!
//! ```text
//! pub enum Value { .. }                          // any shape you like
//! pub struct ParseError { pub offset: Option<usize>, .. }
//! pub fn parse(input: &[u8]) -> Result<Value, ParseError>
//! pub fn print_unformatted(value: &Value) -> Vec<u8>
//! ```
//!
//! Rules:
//! 1. Safe Rust only. The crate root has #![forbid(unsafe_code)] and the build
//!    script rejects any file containing that keyword, even in a comment.
//! 2. std only. No external crates, no `extern crate`, no FFI.
//! 3. `parse(input)` must behave exactly like
//!    `cJSON_ParseWithLengthOpts(input + "\0", len + 1, &end, 1)`:
//!    the driver's C side appends one NUL byte and requires the value to be
//!    followed only by optional whitespace and then a NUL. So an embedded NUL
//!    after the value ends the document, just as in C.
//! 4. On failure, `offset` is the byte offset of cJSON's error pointer
//!    (cJSON_GetErrorPtr() - input), or None if C would have no error pointer.
//!    The driver prints `PARSE_ERROR at <offset>` or `PARSE_ERROR`.
//! 5. `print_unformatted` must return the exact bytes of cJSON_PrintUnformatted
//!    (number formatting via %1.15g / %1.17g round-trip, string escaping,
//!    key order, duplicate keys kept). The driver stops output at the first
//!    NUL byte, like fputs.
//! 6. Nesting deeper than CJSON_NESTING_LIMIT (1000) is a parse error, as in C.
//!    Never overflow the stack or panic on any input; a panic counts as a crash.
//! 7. String data is raw bytes (Vec<u8>), never String: cJSON does not
//!    validate UTF-8, and invalid bytes must round-trip unchanged.
//!
//! Output is compared byte for byte against the C build. Formatting
//! differences are failures.

#![allow(dead_code)]

/// Mirrors CJSON_NESTING_LIMIT in cJSON.h.
pub const CJSON_NESTING_LIMIT: usize = 1000;

/// One cJSON node. Variants mirror the cJSON type flags.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    False,
    True,
    Null,
    /// cJSON keeps valuedouble (used for printing) and a saturated valueint.
    Number(f64),
    /// Raw bytes, not necessarily UTF-8.
    String(Vec<u8>),
    /// cJSON_Raw: printed verbatim. Never produced by parse.
    Raw(Vec<u8>),
    Array(Vec<Value>),
    /// Insertion order preserved, duplicate keys kept.
    Object(Vec<(Vec<u8>, Value)>),
}

/// Parse failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Byte offset of cJSON's error pointer into the input, if any.
    pub offset: Option<usize>,
}

/// Stub: rejects every input so the crate builds and runs deterministically.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let _ = input;
    Err(ParseError { offset: None })
}

/// Stub: never reached while `parse` always fails.
pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let _ = value;
    Vec::new()
}
