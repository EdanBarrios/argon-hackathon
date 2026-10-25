//! libexpat R_2_9_0 (XML_ParserCreate(NULL), no namespaces, no external
//! entities), translated to safe Rust.
//!
//! THIS FILE IS A STUB. Argon replaces it with a full translation.
//! The exact contract is in backend/harness/expat/rust_crate/INTERFACE.md.
//!
//! ```text
//! pub enum Event { .. }          // exactly the variants below
//! pub struct XmlError { pub code: u32, pub line: u64, pub col: u64 }
//! pub fn parse(input: &[u8]) -> Result<Vec<Event>, XmlError>
//! ```
//!
//! Strings are raw UTF-8 bytes (Vec<u8>), exactly what expat hands its
//! callbacks (expat always reports UTF-8, whatever the input encoding).

#![allow(dead_code)]

/// One parser callback, in the order expat would call it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// XmlDeclHandler. `standalone` is -1 (absent), 0 (no) or 1 (yes).
    XmlDecl { version: Option<Vec<u8>>, encoding: Option<Vec<u8>>, standalone: i32 },
    /// StartDoctypeDeclHandler.
    DoctypeStart {
        name: Vec<u8>,
        sysid: Option<Vec<u8>>,
        pubid: Option<Vec<u8>>,
        has_internal_subset: bool,
    },
    /// EndDoctypeDeclHandler.
    DoctypeEnd,
    /// EntityDeclHandler. Internal entity: `value` is Some (sysid, pubid,
    /// notation are None). External entity: `value` is None.
    EntityDecl {
        name: Vec<u8>,
        is_pe: bool,
        value: Option<Vec<u8>>,
        sysid: Option<Vec<u8>>,
        pubid: Option<Vec<u8>>,
        notation: Option<Vec<u8>>,
    },
    /// StartElementHandler: attributes in the order expat passes them
    /// (specified attributes in document order, then defaulted ones).
    Start { name: Vec<u8>, attrs: Vec<(Vec<u8>, Vec<u8>)> },
    /// EndElementHandler (also emitted for empty-element tags `<a/>`).
    End { name: Vec<u8> },
    /// CharacterDataHandler. Any chunking is fine: the driver coalesces
    /// adjacent Text events and drops empty ones.
    Text(Vec<u8>),
    /// ProcessingInstructionHandler.
    Pi { target: Vec<u8>, data: Vec<u8> },
    /// CommentHandler.
    Comment(Vec<u8>),
    /// StartCdataSectionHandler.
    CdataStart,
    /// EndCdataSectionHandler.
    CdataEnd,
    /// SkippedEntityHandler.
    Skipped { name: Vec<u8>, is_pe: bool },
}

/// Parse failure. `code` is the numeric enum XML_Error value from expat.h
/// (XML_GetErrorCode); `line` and `col` are XML_GetCurrentLineNumber64 and
/// XML_GetCurrentColumnNumber64 at the moment of the error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlError {
    pub code: u32,
    pub line: u64,
    pub col: u64,
}

/// enum XML_Error values used by the stub.
pub const XML_ERROR_SYNTAX: u32 = 2;

/// Stub: rejects everything. Replace with the real parser.
pub fn parse(input: &[u8]) -> Result<Vec<Event>, XmlError> {
    let _ = input;
    Err(XmlError { code: XML_ERROR_SYNTAX, line: 1, col: 0 })
}
