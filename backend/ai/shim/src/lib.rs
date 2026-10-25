//! libexpat 2.9.0 C ABI over Argon's safe-Rust libexpat translation.
//!
//! `expat.rs` is Argon's verified port, copied byte for byte (build.sh checks its sha256).
//! `ffi.rs` is the C bridge, also written by Argon; it is the only module allowed raw-pointer code.

#[forbid(unsafe_code)]
pub mod expat;
pub mod ffi;
