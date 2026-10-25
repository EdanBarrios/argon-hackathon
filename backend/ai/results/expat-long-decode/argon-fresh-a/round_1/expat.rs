#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    XmlDecl { version: Option<Vec<u8>>, encoding: Option<Vec<u8>>, standalone: i32 },
    DoctypeStart {
        name: Vec<u8>,
        sysid: Option<Vec<u8>>,
        pubid: Option<Vec<u8>>,
        has_internal_subset: bool,
    },
    DoctypeEnd,
    EntityDecl {
        name: Vec<u8>,
        is_pe: bool,
        value: Option<Vec<u8>>,
        sysid: Option<Vec<u8>>,
        pubid: Option<Vec<u8>>,
        notation: Option<Vec<u8>>,
    },
    Start { name: Vec<u8>, attrs: Vec<(Vec<u8>, Vec<u8>)> },
    End { name: Vec<u8> },
    Text(Vec<u8>),
    Pi { target: Vec<u8>, data: Vec<u8> },
    Comment(Vec<u8>),
    CdataStart,
    CdataEnd,
    Skipped { name: Vec<u8>, is_pe: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlError {
    pub code: u32,
    pub line: u64,
    pub col: u64,
}

pub const XML_ERROR_NONE: u32 = 0;
pub const XML_ERROR_NO_MEMORY: u32 = 1;
pub const XML_ERROR_SYNTAX: u32 = 2;
pub const XML_ERROR_NO_ELEMENTS: u32 = 3;
pub const XML_ERROR_INVALID_TOKEN: u32 = 4;
pub const XML_ERROR_UNCLOSED_TOKEN: u32 = 5;
pub const XML_ERROR_PARTIAL_CHAR: u32 = 6;
pub const XML_ERROR_TAG_MISMATCH: u32 = 7;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: u32 = 8;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: u32 = 9;
pub const XML_ERROR_PARAM_ENTITY_REF: u32 = 10;
pub const XML_ERROR_UNDEFINED_ENTITY: u32 = 11;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: u32 = 12;
pub const XML_ERROR_ASYNC_ENTITY: u32 = 13;
pub const XML_ERROR_BAD_CHAR_REF: u32 = 14;
pub const XML_ERROR_BINARY_ENTITY_REF: u32 = 15;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: u32 = 16;
pub const XML_ERROR_MISPLACED_XML_PI: u32 = 17;
pub const XML_ERROR_UNKNOWN_ENCODING: u32 = 18;
pub const XML_ERROR_INCORRECT_ENCODING: u32 = 19;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: u32 = 20;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: u32 = 21;
pub const XML_ERROR_NOT_STANDALONE: u32 = 22;
pub const XML_ERROR_UNEXPECTED_STATE: u32 = 23;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: u32 = 24;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: u32 = 25;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: u32 = 26;
pub const XML_ERROR_UNBOUND_PREFIX: u32 = 27;
pub const XML_ERROR_UNDECLARING_PREFIX: u32 = 28;
pub const XML_ERROR_INCOMPLETE_PE: u32 = 29;
pub const XML_ERROR_XML_DECL: u32 = 30;
pub const XML_ERROR_TEXT_DECL: u32 = 31;
pub const XML_ERROR_PUBLICID: u32 = 32;
pub const XML_ERROR_SUSPENDED: u32 = 33;
pub const XML_ERROR_NOT_SUSPENDED: u32 = 34;
pub const XML_ERROR_ABORTED: u32 = 35;
pub const XML_ERROR_FINISHED: u32 = 36;
pub const XML_ERROR_SUSPEND_PE: u32 = 37;
pub const XML_ERROR_RESERVED_PREFIX_XML: u32 = 38;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: u32 = 39;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: u32 = 40;
pub const XML_ERROR_INVALID_ARGUMENT: u32 = 41;
pub const XML_ERROR_NO_BUFFER: u32 = 42;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: u32 = 43;
pub const XML_ERROR_NOT_STARTED: u32 = 44;

const BT_NONXML: u8 = 0;
const BT_MALFORM: u8 = 1;
const BT_LT: u8 = 2;
const BT_AMP: u8 = 3;
const BT_RSQB: u8 = 4;
const BT_LEAD2: u8 = 5;
const BT_LEAD3: u8 = 6;
const BT_LEAD4: u8 = 7;
const BT_TRAIL: u8 = 8;
const BT_CR: u8 = 9;
const BT_LF: u8 = 10;
const BT_GT: u8 = 11;
const BT_QUOT: u8 = 12;
const BT_APOS: u8 = 13;
const BT_EQUALS: u8 = 14;
const BT_QUEST: u8 = 15;
const BT_EXCL: u8 = 16;
const BT_SOL: u8 = 17;
const BT_SEMI: u8 = 18;
const BT_NUM: u8 = 19;
const BT_LSQB: u8 = 20;
const BT_S: u8 = 21;
const BT_NMSTRT: u8 = 22;
const BT_COLON: u8 = 22; // Non-NS mode: BT_COLON is mapped to BT_NMSTRT
const BT_HEX: u8 = 24;
const BT_DIGIT: u8 = 25;
const BT_NAME: u8 = 26;
const BT_MINUS: u8 = 27;
const BT_OTHER: u8 = 28;
const BT_NONASCII: u8 = 29;
const BT_PERCNT: u8 = 30;
const BT_LPAR: u8 = 31;
const BT_RPAR: u8 = 32;
const BT_AST: u8 = 33;
const BT_PLUS: u8 = 34;
const BT_COMMA: u8 = 35;
const BT_VERBAR: u8 = 36;

const XML_TOK_TRAILING_RSQB: i32 = -5;
const XML_TOK_NONE: i32 = -4;
const XML_TOK_TRAILING_CR: i32 = -3;
const XML_TOK_PARTIAL_CHAR: i32 = -2;
const XML_TOK_PARTIAL: i32 = -1;
const XML_TOK_INVALID: i32 = 0;
const XML_TOK_START_TAG_WITH_ATTS: i32 = 1;
const XML_TOK_START_TAG_NO_ATTS: i32 = 2;
const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: i32 = 3;
const XML_TOK_EMPTY_ELEMENT_NO_ATTS: i32 = 4;
const XML_TOK_END_TAG: i32 = 5;
const XML_TOK_DATA_CHARS: i32 = 6;
const XML_TOK_DATA_NEWLINE: i32 = 7;
const XML_TOK_CDATA_SECT_OPEN: i32 = 8;
const XML_TOK_ENTITY_REF: i32 = 9;
const XML_TOK_CHAR_REF: i32 = 10;
const XML_TOK_PI: i32 = 11;
const XML_TOK_XML_DECL: i32 = 12;
const XML_TOK_COMMENT: i32 = 13;
const XML_TOK_BOM: i32 = 14;
const XML_TOK_PROLOG_S: i32 = 15;
const XML_TOK_DECL_OPEN: i32 = 16;
const XML_TOK_DECL_CLOSE: i32 = 17;
const XML_TOK_NAME: i32 = 18;
const XML_TOK_NMTOKEN: i32 = 19;
const XML_TOK_POUND_NAME: i32 = 20;
const XML_TOK_OR: i32 = 21;
const XML_TOK_PERCENT: i32 = 22;
const XML_TOK_OPEN_PAREN: i32 = 23;
const XML_TOK_CLOSE_PAREN: i32 = 24;
const XML_TOK_OPEN_BRACKET: i32 = 25;
const XML_TOK_CLOSE_BRACKET: i32 = 26;
const XML_TOK_LITERAL: i32 = 27;
const XML_TOK_PARAM_ENTITY_REF: i32 = 28;
const XML_TOK_INSTANCE_START: i32 = 29;
const XML_TOK_NAME_QUESTION: i32 = 30;
const XML_TOK_NAME_ASTERISK: i32 = 31;
const XML_TOK_NAME_PLUS: i32 = 32;
const XML_TOK_COND_SECT_OPEN: i32 = 33;
const XML_TOK_COND_SECT_CLOSE: i32 = 34;
const XML_TOK_CLOSE_PAREN_QUESTION: i32 = 35;
const XML_TOK_CLOSE_PAREN_ASTERISK: i32 = 36;
const XML_TOK_CLOSE_PAREN_PLUS: i32 = 37;
const XML_TOK_COMMA: i32 = 38;
const XML_TOK_ATTRIBUTE_VALUE_S: i32 = 39;
const XML_TOK_CDATA_SECT_CLOSE: i32 = 40;
const XML_TOK_PREFIXED_NAME: i32 = 41;
const XML_TOK_IGNORE_SECT: i32 = 42;

const XML_PROLOG_STATE: usize = 0;
const XML_CONTENT_STATE: usize = 1;

const XML_ROLE_ERROR: i32 = -1;
const XML_ROLE_NONE: i32 = 0;
const XML_ROLE_XML_DECL: i32 = 1;
const XML_ROLE_INSTANCE_START: i32 = 2;
const XML_ROLE_DOCTYPE_NONE: i32 = 3;
const XML_ROLE_DOCTYPE_NAME: i32 = 4;
const XML_ROLE_DOCTYPE_SYSTEM_ID: i32 = 5;
const XML_ROLE_DOCTYPE_PUBLIC_ID: i32 = 6;
const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: i32 = 7;
const XML_ROLE_DOCTYPE_CLOSE: i32 = 8;
const XML_ROLE_GENERAL_ENTITY_NAME: i32 = 9;
const XML_ROLE_PARAM_ENTITY_NAME: i32 = 10;
const XML_ROLE_ENTITY_NONE: i32 = 11;
const XML_ROLE_ENTITY_VALUE: i32 = 12;
const XML_ROLE_ENTITY_SYSTEM_ID: i32 = 13;
const XML_ROLE_ENTITY_PUBLIC_ID: i32 = 14;
const XML_ROLE_ENTITY_COMPLETE: i32 = 15;
const XML_ROLE_ENTITY_NOTATION_NAME: i32 = 16;
const XML_ROLE_NOTATION_NONE: i32 = 17;
const XML_ROLE_NOTATION_NAME: i32 = 18;
const XML_ROLE_NOTATION_SYSTEM_ID: i32 = 19;
const XML_ROLE_NOTATION_NO_SYSTEM_ID: i32 = 20;
const XML_ROLE_NOTATION_PUBLIC_ID: i32 = 21;
const XML_ROLE_ATTRIBUTE_NAME: i32 = 22;
const XML_ROLE_ATTRIBUTE_TYPE_CDATA: i32 = 23;
const XML_ROLE_ATTRIBUTE_TYPE_ID: i32 = 24;
const XML_ROLE_ATTRIBUTE_TYPE_IDREF: i32 = 25;
const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: i32 = 26;
const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: i32 = 27;
const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: i32 = 28;
const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: i32 = 29;
const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: i32 = 30;
const XML_ROLE_ATTRIBUTE_ENUM_VALUE: i32 = 31;
const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: i32 = 32;
const XML_ROLE_ATTLIST_NONE: i32 = 33;
const XML_ROLE_ATTLIST_ELEMENT_NAME: i32 = 34;
const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: i32 = 35;
const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: i32 = 36;
const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: i32 = 37;
const XML_ROLE_FIXED_ATTRIBUTE_VALUE: i32 = 38;
const XML_ROLE_ELEMENT_NONE: i32 = 39;
const XML_ROLE_ELEMENT_NAME: i32 = 40;
const XML_ROLE_CONTENT_ANY: i32 = 41;
const XML_ROLE_CONTENT_EMPTY: i32 = 42;
const XML_ROLE_CONTENT_PCDATA: i32 = 43;
const XML_ROLE_GROUP_OPEN: i32 = 44;
const XML_ROLE_GROUP_CLOSE: i32 = 45;
const XML_ROLE_GROUP_CLOSE_REP: i32 = 46;
const XML_ROLE_GROUP_CLOSE_OPT: i32 = 47;
const XML_ROLE_GROUP_CLOSE_PLUS: i32 = 48;
const XML_ROLE_GROUP_CHOICE: i32 = 49;
const XML_ROLE_GROUP_SEQUENCE: i32 = 50;
const XML_ROLE_CONTENT_ELEMENT: i32 = 51;
const XML_ROLE_CONTENT_ELEMENT_REP: i32 = 52;
const XML_ROLE_CONTENT_ELEMENT_OPT: i32 = 53;
const XML_ROLE_CONTENT_ELEMENT_PLUS: i32 = 54;
const XML_ROLE_PI: i32 = 55;
const XML_ROLE_COMMENT: i32 = 56;
const XML_ROLE_TEXT_DECL: i32 = 57;
const XML_ROLE_IGNORE_SECT: i32 = 58;
const XML_ROLE_INNER_PARAM_ENTITY_REF: i32 = 59;
const XML_ROLE_PARAM_ENTITY_REF: i32 = 60;

const UTF8_TABLE: [u8; 256] = [
    /* 0x00 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x04 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x08 */ BT_NONXML, BT_S, BT_LF, BT_NONXML,
    /* 0x0C */ BT_NONXML, BT_CR, BT_NONXML, BT_NONXML,
    /* 0x10 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x14 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x18 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x1C */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x20 */ BT_S, BT_EXCL, BT_QUOT, BT_NUM,
    /* 0x24 */ BT_OTHER, BT_PERCNT, BT_AMP, BT_APOS,
    /* 0x28 */ BT_LPAR, BT_RPAR, BT_AST, BT_PLUS,
    /* 0x2C */ BT_COMMA, BT_MINUS, BT_NAME, BT_SOL,
    /* 0x30 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x34 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x38 */ BT_DIGIT, BT_DIGIT, BT_COLON, BT_SEMI,
    /* 0x3C */ BT_LT, BT_EQUALS, BT_GT, BT_QUEST,
    /* 0x40 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x44 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x48 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x4C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x50 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x54 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x58 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_LSQB,
    /* 0x5C */ BT_OTHER, BT_RSQB, BT_OTHER, BT_NMSTRT,
    /* 0x60 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x64 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x68 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x6C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x70 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x74 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x78 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0x7C */ BT_VERBAR, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x80 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x84 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x88 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x8C */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x90 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x94 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x98 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x9C */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xA0 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xA4 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xA8 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xAC */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xB0 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xB4 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xB8 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xBC */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xC0 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xC4 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xC8 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xCC */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xD0 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xD4 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xD8 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xDC */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xE0 */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xE4 */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xE8 */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xEC */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xF0 */ BT_LEAD4, BT_LEAD4, BT_LEAD4, BT_LEAD4,
    /* 0xF4 */ BT_LEAD4, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0xF8 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0xFC */ BT_NONXML, BT_NONXML, BT_MALFORM, BT_MALFORM,
];

const INTERNAL_UTF8_TABLE: [u8; 256] = [
    /* 0x00 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x04 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x08 */ BT_NONXML, BT_S, BT_LF, BT_NONXML,
    /* 0x0C */ BT_NONXML, BT_S, BT_NONXML, BT_NONXML,
    /* 0x10 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x14 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x18 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x1C */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x20 */ BT_S, BT_EXCL, BT_QUOT, BT_NUM,
    /* 0x24 */ BT_OTHER, BT_PERCNT, BT_AMP, BT_APOS,
    /* 0x28 */ BT_LPAR, BT_RPAR, BT_AST, BT_PLUS,
    /* 0x2C */ BT_COMMA, BT_MINUS, BT_NAME, BT_SOL,
    /* 0x30 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x34 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x38 */ BT_DIGIT, BT_DIGIT, BT_COLON, BT_SEMI,
    /* 0x3C */ BT_LT, BT_EQUALS, BT_GT, BT_QUEST,
    /* 0x40 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x44 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x48 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x4C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x50 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x54 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x58 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_LSQB,
    /* 0x5C */ BT_OTHER, BT_RSQB, BT_OTHER, BT_NMSTRT,
    /* 0x60 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x64 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x68 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x6C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x70 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x74 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x78 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0x7C */ BT_VERBAR, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x80 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x84 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x88 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x8C */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x90 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x94 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x98 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0x9C */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xA0 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xA4 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xA8 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xAC */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xB0 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xB4 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xB8 */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xBC */ BT_TRAIL, BT_TRAIL, BT_TRAIL, BT_TRAIL,
    /* 0xC0 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xC4 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xC8 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xCC */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xD0 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xD4 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xD8 */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xDC */ BT_LEAD2, BT_LEAD2, BT_LEAD2, BT_LEAD2,
    /* 0xE0 */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xE4 */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xE8 */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xEC */ BT_LEAD3, BT_LEAD3, BT_LEAD3, BT_LEAD3,
    /* 0xF0 */ BT_LEAD4, BT_LEAD4, BT_LEAD4, BT_LEAD4,
    /* 0xF4 */ BT_LEAD4, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0xF8 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0xFC */ BT_NONXML, BT_NONXML, BT_MALFORM, BT_MALFORM,
];

const LATIN1_TABLE: [u8; 256] = [
    /* 0x00 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x04 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x08 */ BT_NONXML, BT_S, BT_LF, BT_NONXML,
    /* 0x0C */ BT_NONXML, BT_CR, BT_NONXML, BT_NONXML,
    /* 0x10 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x14 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x18 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x1C */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x20 */ BT_S, BT_EXCL, BT_QUOT, BT_NUM,
    /* 0x24 */ BT_OTHER, BT_PERCNT, BT_AMP, BT_APOS,
    /* 0x28 */ BT_LPAR, BT_RPAR, BT_AST, BT_PLUS,
    /* 0x2C */ BT_COMMA, BT_MINUS, BT_NAME, BT_SOL,
    /* 0x30 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x34 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x38 */ BT_DIGIT, BT_DIGIT, BT_COLON, BT_SEMI,
    /* 0x3C */ BT_LT, BT_EQUALS, BT_GT, BT_QUEST,
    /* 0x40 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x44 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x48 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x4C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x50 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x54 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x58 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_LSQB,
    /* 0x5C */ BT_OTHER, BT_RSQB, BT_OTHER, BT_NMSTRT,
    /* 0x60 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x64 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x68 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x6C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x70 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x74 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x78 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0x7C */ BT_VERBAR, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x80 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x84 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x88 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x8C */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x90 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x94 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x98 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0x9C */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0xA0 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0xA4 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0xA8 */ BT_OTHER, BT_OTHER, BT_NMSTRT, BT_OTHER,
    /* 0xAC */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0xB0 */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0xB4 */ BT_OTHER, BT_NMSTRT, BT_OTHER, BT_NAME,
    /* 0xB8 */ BT_OTHER, BT_OTHER, BT_NMSTRT, BT_OTHER,
    /* 0xBC */ BT_OTHER, BT_OTHER, BT_OTHER, BT_OTHER,
    /* 0xC0 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xC4 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xC8 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xCC */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xD0 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xD4 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0xD8 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xDC */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xE0 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xE4 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xE8 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xEC */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xF0 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xF4 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0xF8 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0xFC */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
];

const ASCII_TABLE: [u8; 256] = [
    /* 0x00 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x04 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x08 */ BT_NONXML, BT_S, BT_LF, BT_NONXML,
    /* 0x0C */ BT_NONXML, BT_CR, BT_NONXML, BT_NONXML,
    /* 0x10 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x14 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x18 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x1C */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x20 */ BT_S, BT_EXCL, BT_QUOT, BT_NUM,
    /* 0x24 */ BT_OTHER, BT_PERCNT, BT_AMP, BT_APOS,
    /* 0x28 */ BT_LPAR, BT_RPAR, BT_AST, BT_PLUS,
    /* 0x2C */ BT_COMMA, BT_MINUS, BT_NAME, BT_SOL,
    /* 0x30 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x34 */ BT_DIGIT, BT_DIGIT, BT_DIGIT, BT_DIGIT,
    /* 0x38 */ BT_DIGIT, BT_DIGIT, BT_COLON, BT_SEMI,
    /* 0x3C */ BT_LT, BT_EQUALS, BT_GT, BT_QUEST,
    /* 0x40 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x44 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x48 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x4C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x50 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x54 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x58 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_LSQB,
    /* 0x5C */ BT_OTHER, BT_RSQB, BT_OTHER, BT_NMSTRT,
    /* 0x60 */ BT_OTHER, BT_HEX, BT_HEX, BT_HEX,
    /* 0x64 */ BT_HEX, BT_HEX, BT_HEX, BT_NMSTRT,
    /* 0x68 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x6C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x70 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x74 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x78 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0x7C */ BT_VERBAR, BT_OTHER, BT_OTHER, BT_OTHER,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const NAMING_BITMAP: [u32; 320] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x04000000,
    0x87FFFFFE, 0x07FFFFFE, 0x00000000, 0x00000000, 0xFF7FFFFF, 0xFF7FFFFF,
    0xFFFFFFFF, 0x7FF3FFFF, 0xFFFFFDFE, 0x7FFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
    0xFFFFE00F, 0xFC31FFFF, 0x00FFFFFF, 0x00000000, 0xFFFF0000, 0xFFFFFFFF,
    0xFFFFFFFF, 0xF80001FF, 0x00000003, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0xFFFFD740, 0xFFFFFFFB, 0x547F7FFF, 0x000FFFFD,
    0xFFFFDFFE, 0xFFFFFFFF, 0xDFFEFFFF, 0xFFFFFFFF, 0xFFFF0003, 0xFFFFFFFF,
    0xFFFF199F, 0x033FCFFF, 0x00000000, 0xFFFE0000, 0x027FFFFF, 0xFFFFFFFE,
    0x0000007F, 0x00000000, 0xFFFF0000, 0x000707FF, 0x00000000, 0x07FFFFFE,
    0x000007FE, 0xFFFE0000, 0xFFFFFFFF, 0x7CFFFFFF, 0x002F7FFF, 0x00000060,
    0xFFFFFFE0, 0x23FFFFFF, 0xFF000000, 0x00000003, 0xFFF99FE0, 0x03C5FDFF,
    0xB0000000, 0x00030003, 0xFFF987E0, 0x036DFDFF, 0x5E000000, 0x001C0000,
    0xFFFBAFE0, 0x23EDFDFF, 0x00000000, 0x00000001, 0xFFF99FE0, 0x23CDFDFF,
    0xB0000000, 0x00000003, 0xD63DC7E0, 0x03BFC718, 0x00000000, 0x00000000,
    0xFFFDDFE0, 0x03EFFDFF, 0x00000000, 0x00000003, 0xFFFDDFE0, 0x03EFFDFF,
    0x40000000, 0x00000003, 0xFFFDDFE0, 0x03FFFDFF, 0x00000000, 0x00000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFE, 0x000D7FFF,
    0x0000003F, 0x00000000, 0xFEF02596, 0x200D6CAE, 0x0000001F, 0x00000000,
    0x00000000, 0x00000000, 0xFFFFFEFF, 0x000003FF, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0xFFFFFFFF, 0xFFFF003F, 0x007FFFFF, 0x0007DAED, 0x50000000,
    0x82315001, 0x002C62AB, 0x40000000, 0xF580C900, 0x00000007, 0x02010800,
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x0FFFFFFF, 0xFFFFFFFF,
    0xFFFFFFFF, 0x03FFFFFF, 0x3F3FFFFF, 0xFFFFFFFF, 0xAAFF3F3F, 0x3FFFFFFF,
    0xFFFFFFFF, 0x5FDFFFFF, 0x0FCF1FDC, 0x1FDC1FFF, 0x00000000, 0x00004C40,
    0x00000000, 0x00000000, 0x00000007, 0x00000000, 0x00000000, 0x00000000,
    0x00000080, 0x000003FE, 0xFFFFFFFE, 0xFFFFFFFF, 0x001FFFFF, 0xFFFFFFFE,
    0xFFFFFFFF, 0x07FFFFFF, 0xFFFFFFE0, 0x00001FFF, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF,
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x0000003F, 0x00000000, 0x00000000,
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x0000000F,
    0x00000000, 0x00000000, 0x00000000, 0x07FF6000, 0x87FFFFFE, 0x07FFFFFE,
    0x00000000, 0x00800000, 0xFF7FFFFF, 0xFF7FFFFF, 0x00FFFFFF, 0x00000000,
    0xFFFF0000, 0xFFFFFFFF, 0xFFFFFFFF, 0xF80001FF, 0x00030003, 0x00000000,
    0xFFFFFFFF, 0xFFFFFFFF, 0x0000003F, 0x00000003, 0xFFFFD7C0, 0xFFFFFFFB,
    0x547F7FFF, 0x000FFFFD, 0xFFFFDFFE, 0xFFFFFFFF, 0xDFFEFFFF, 0xFFFFFFFF,
    0xFFFF007B, 0xFFFFFFFF, 0xFFFF199F, 0x033FCFFF, 0x00000000, 0xFFFE0000,
    0x027FFFFF, 0xFFFFFFFE, 0xFFFE007F, 0xBBFFFFFB, 0xFFFF0016, 0x000707FF,
    0x00000000, 0x07FFFFFE, 0x0007FFFF, 0xFFFF03FF, 0xFFFFFFFF, 0x7CFFFFFF,
    0xFFEF7FFF, 0x03FF3DFF, 0xFFFFFFEE, 0xF3FFFFFF, 0xFF1E3FFF, 0x0000FFCF,
    0xFFF99FEE, 0xD3C5FDFF, 0xB080399F, 0x0003FFCF, 0xFFF987E4, 0xD36DFDFF,
    0x5E003987, 0x001FFFC0, 0xFFFBAFEE, 0xF3EDFDFF, 0x00003BBF, 0x0000FFC1,
    0xFFF99FEE, 0xF3CDFDFF, 0xB0C0398F, 0x0000FFC3, 0xD63DC7EC, 0xC3BFC718,
    0x00803DC7, 0x0000FF80, 0xFFFDDFEE, 0xC3EFFDFF, 0x00603DDF, 0x0000FFC3,
    0xFFFDDFEC, 0xC3EFFDFF, 0x40603DDF, 0x0000FFC3, 0xFFFDDFEC, 0xC3FFFDFF,
    0x00803DCF, 0x0000FFC3, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xFFFFFFFE, 0x07FF7FFF, 0x03FF7FFF, 0x00000000, 0xFEF02596, 0x3BFF6CAE,
    0x03FF3F5F, 0x00000000, 0x03000000, 0xC2A003FF, 0xFFFFFEFF, 0xFFFE03FF,
    0xFEBF0FDF, 0x02FE3FFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x1FFF0000, 0x00000002,
    0x000000A0, 0x003EFFFE, 0xFFFFFFFE, 0xFFFFFFFF, 0x661FFFFF, 0xFFFFFFFE,
    0xFFFFFFFF, 0x77FFFFFF,
];

const NMSTRT_PAGES: [u8; 256] = [
    0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x00, 0x09, 0x0A, 0x0B,
    0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x13, 0x00, 0x14, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x15, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x18,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

const NAME_PAGES: [u8; 256] = [
    0x19, 0x03, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x00, 0x00, 0x1F, 0x20, 0x21,
    0x22, 0x23, 0x24, 0x25, 0x10, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x13, 0x26, 0x14, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x27, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x18,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

#[inline]
fn ucs2_get_naming(pages: &[u8; 256], hi: u8, lo: u8) -> bool {
    let idx = ((pages[hi as usize] as usize) << 3) + ((lo as usize) >> 5);
    (NAMING_BITMAP[idx] & (1u32 << (lo & 0x1F))) != 0
}

#[inline]
fn utf8_get_naming2(pages: &[u8; 256], byte: &[u8]) -> bool {
    let b0 = byte[0] as usize;
    let b1 = byte[1] as usize;
    let idx = ((pages[(b0 >> 2) & 7] as usize) << 3) + ((b0 & 3) << 1) + ((b1 >> 5) & 1);
    (NAMING_BITMAP[idx] & (1u32 << (b1 & 0x1F))) != 0
}

#[inline]
fn utf8_get_naming3(pages: &[u8; 256], byte: &[u8]) -> bool {
    let b0 = byte[0] as usize;
    let b1 = byte[1] as usize;
    let b2 = byte[2] as usize;
    let page_idx = ((b0 & 0xF) << 4) + ((b1 >> 2) & 0xF);
    let idx = ((pages[page_idx] as usize) << 3) + ((b1 & 3) << 1) + ((b2 >> 5) & 1);
    (NAMING_BITMAP[idx] & (1u32 << (b2 & 0x1F))) != 0
}

#[inline]
fn utf8_invalid2(p: &[u8]) -> bool {
    p[0] < 0xC2 || (p[1] & 0x80) == 0 || (p[1] & 0xC0) == 0xC0
}

#[inline]
fn utf8_invalid3(p: &[u8]) -> bool {
    (p[2] & 0x80) == 0
        || if p[0] == 0xEF && p[1] == 0xBF {
            p[2] > 0xBD
        } else {
            (p[2] & 0xC0) == 0xC0
        }
        || if p[0] == 0xE0 {
            p[1] < 0xA0 || (p[1] & 0xC0) == 0xC0
        } else {
            (p[1] & 0x80) == 0
                || if p[0] == 0xED {
                    p[1] > 0x9F
                } else {
                    (p[1] & 0xC0) == 0xC0
                }
        }
}

#[inline]
fn utf8_invalid4(p: &[u8]) -> bool {
    (p[3] & 0x80) == 0
        || (p[3] & 0xC0) == 0xC0
        || (p[2] & 0x80) == 0
        || (p[2] & 0xC0) == 0xC0
        || if p[0] == 0xF0 {
            p[1] < 0x90 || (p[1] & 0xC0) == 0xC0
        } else {
            (p[1] & 0x80) == 0
                || if p[0] == 0xF4 {
                    p[1] > 0x8F
                } else {
                    (p[1] & 0xC0) == 0xC0
                }
        }
}

#[inline]
fn unicode_byte_type(hi: u8, lo: u8) -> u8 {
    match hi {
        0xD8..=0xDB => BT_LEAD4,
        0xDC..=0xDF => BT_TRAIL,
        0xFF => match lo {
            0xFF | 0xFE => BT_NONXML,
            _ => BT_NONASCII,
        },
        _ => BT_NONASCII,
    }
}

#[inline]
fn check_char_ref_number(result: i32) -> i32 {
    if result < 0 {
        return -1;
    }
    match result >> 8 {
        0xD8..=0xDF => -1,
        0 => {
            if LATIN1_TABLE[(result & 0xFF) as usize] == BT_NONXML {
                -1
            } else {
                result
            }
        }
        0xFF => {
            if result == 0xFFFE || result == 0xFFFF {
                -1
            } else {
                result
            }
        }
        _ => result,
    }
}

fn xml_utf8_encode(c: i32, buf: &mut [u8; 4]) -> usize {
    if c < 0 {
        return 0;
    }
    let c = c as u32;
    if c < 0x80 {
        buf[0] = c as u8;
        1
    } else if c < 0x800 {
        buf[0] = ((c >> 6) as u8) | 0xC0;
        buf[1] = ((c & 0x3F) as u8) | 0x80;
        2
    } else if c < 0x10000 {
        buf[0] = ((c >> 12) as u8) | 0xE0;
        buf[1] = (((c >> 6) & 0x3F) as u8) | 0x80;
        buf[2] = ((c & 0x3F) as u8) | 0x80;
        3
    } else if c < 0x110000 {
        buf[0] = ((c >> 18) as u8) | 0xF0;
        buf[1] = (((c >> 12) & 0x3F) as u8) | 0x80;
        buf[2] = (((c >> 6) & 0x3F) as u8) | 0x80;
        buf[3] = ((c & 0x3F) as u8) | 0x80;
        4
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Encoding {
    Init(InitEncIndex),
    Utf8,
    InternalUtf8,
    Latin1,
    Ascii,
    Little2,
    Big2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InitEncIndex {
    Iso8859_1,
    UsAscii,
    Utf8,
    Utf16,
    Utf16Be,
    Utf16Le,
    NoEnc,
}

impl InitEncIndex {
    fn to_encoding(self) -> Encoding {
        match self {
            InitEncIndex::Iso8859_1 => Encoding::Latin1,
            InitEncIndex::UsAscii => Encoding::Ascii,
            InitEncIndex::Utf8 => Encoding::Utf8,
            InitEncIndex::Utf16 => Encoding::Big2,
            InitEncIndex::Utf16Be => Encoding::Big2,
            InitEncIndex::Utf16Le => Encoding::Little2,
            InitEncIndex::NoEnc => Encoding::Utf8,
        }
    }
}

fn get_encoding_index(name: &[u8]) -> Option<InitEncIndex> {
    if streqci(name, b"ISO-8859-1") {
        Some(InitEncIndex::Iso8859_1)
    } else if streqci(name, b"US-ASCII") {
        Some(InitEncIndex::UsAscii)
    } else if streqci(name, b"UTF-8") {
        Some(InitEncIndex::Utf8)
    } else if streqci(name, b"UTF-16") {
        Some(InitEncIndex::Utf16)
    } else if streqci(name, b"UTF-16BE") {
        Some(InitEncIndex::Utf16Be)
    } else if streqci(name, b"UTF-16LE") {
        Some(InitEncIndex::Utf16Le)
    } else {
        None
    }
}

fn streqci(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    for (&b1, &b2) in s1.iter().zip(s2.iter()) {
        let c1 = if b1.is_ascii_lowercase() { b1 - 32 } else { b1 };
        let c2 = if b2.is_ascii_lowercase() { b2 - 32 } else { b2 };
        if c1 != c2 {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy, Default)]
struct Position {
    line_number: u64,
    column_number: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct RawAttribute {
    name: usize,
    value_ptr: usize,
    value_end: usize,
    normalized: bool,
}

impl Encoding {
    #[inline]
    fn min_bytes_per_char(self) -> usize {
        match self {
            Encoding::Little2 | Encoding::Big2 => 2,
            _ => 1,
        }
    }

    #[inline]
    fn byte_type(self, buf: &[u8], p: usize) -> u8 {
        match self {
            Encoding::Utf8 | Encoding::Init(_) => UTF8_TABLE[buf[p] as usize],
            Encoding::InternalUtf8 => INTERNAL_UTF8_TABLE[buf[p] as usize],
            Encoding::Latin1 => LATIN1_TABLE[buf[p] as usize],
            Encoding::Ascii => ASCII_TABLE[buf[p] as usize],
            Encoding::Little2 => {
                if buf[p + 1] == 0 {
                    LATIN1_TABLE[buf[p] as usize]
                } else {
                    unicode_byte_type(buf[p + 1], buf[p])
                }
            }
            Encoding::Big2 => {
                if buf[p] == 0 {
                    LATIN1_TABLE[buf[p + 1] as usize]
                } else {
                    unicode_byte_type(buf[p], buf[p + 1])
                }
            }
        }
    }

    #[inline]
    fn byte_to_ascii(self, buf: &[u8], p: usize) -> i32 {
        match self {
            Encoding::Little2 => {
                if buf[p + 1] == 0 {
                    buf[p] as i32
                } else {
                    -1
                }
            }
            Encoding::Big2 => {
                if buf[p] == 0 {
                    buf[p + 1] as i32
                } else {
                    -1
                }
            }
            _ => buf[p] as i32,
        }
    }

    #[inline]
    fn char_matches(self, buf: &[u8], p: usize, c: u8) -> bool {
        match self {
            Encoding::Little2 => buf[p + 1] == 0 && buf[p] == c,
            Encoding::Big2 => buf[p] == 0 && buf[p + 1] == c,
            _ => buf[p] == c,
        }
    }

    #[inline]
    fn is_name_char(self, buf: &[u8], p: usize, n: usize) -> bool {
        match self {
            Encoding::Utf8 | Encoding::InternalUtf8 | Encoding::Init(_) => match n {
                2 => utf8_get_naming2(&NAME_PAGES, &buf[p..p + 2]),
                3 => utf8_get_naming3(&NAME_PAGES, &buf[p..p + 3]),
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn is_nmstrt_char(self, buf: &[u8], p: usize, n: usize) -> bool {
        match self {
            Encoding::Utf8 | Encoding::InternalUtf8 | Encoding::Init(_) => match n {
                2 => utf8_get_naming2(&NMSTRT_PAGES, &buf[p..p + 2]),
                3 => utf8_get_naming3(&NMSTRT_PAGES, &buf[p..p + 3]),
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn is_invalid_char(self, buf: &[u8], p: usize, n: usize) -> bool {
        match self {
            Encoding::Utf8 | Encoding::InternalUtf8 | Encoding::Init(_) => match n {
                2 => utf8_invalid2(&buf[p..p + 2]),
                3 => utf8_invalid3(&buf[p..p + 3]),
                4 => utf8_invalid4(&buf[p..p + 4]),
                _ => true,
            },
            Encoding::Little2 => n == 4 && (buf[p + 3] & 0xFC) != 0xDC,
            Encoding::Big2 => n == 4 && (buf[p + 2] & 0xFC) != 0xDC,
            _ => false,
        }
    }

    #[inline]
    fn is_name_char_minbpc(self, buf: &[u8], p: usize) -> bool {
        match self {
            Encoding::Little2 => ucs2_get_naming(&NAME_PAGES, buf[p + 1], buf[p]),
            Encoding::Big2 => ucs2_get_naming(&NAME_PAGES, buf[p], buf[p + 1]),
            _ => false,
        }
    }

    #[inline]
    fn is_nmstrt_char_minbpc(self, buf: &[u8], p: usize) -> bool {
        match self {
            Encoding::Little2 => ucs2_get_naming(&NMSTRT_PAGES, buf[p + 1], buf[p]),
            Encoding::Big2 => ucs2_get_naming(&NMSTRT_PAGES, buf[p], buf[p + 1]),
            _ => false,
        }
    }

    fn convert_to_utf8(self, buf: &[u8], start: usize, end: usize, out: &mut Vec<u8>) {
        if start >= end {
            return;
        }
        let slice = &buf[start..end];
        match self {
            Encoding::Utf8 | Encoding::InternalUtf8 | Encoding::Init(_) => {
                let lim = trim_to_complete_utf8_characters(slice);
                out.extend_from_slice(&slice[..lim]);
            }
            Encoding::Ascii => {
                out.extend_from_slice(slice);
            }
            Encoding::Latin1 => {
                for &c in slice {
                    if (c & 0x80) != 0 {
                        out.push((c >> 6) | 0xC0);
                        out.push((c & 0x3F) | 0x80);
                    } else {
                        out.push(c);
                    }
                }
            }
            Encoding::Little2 => {
                let even_len = slice.len() & !1;
                let mut i = 0;
                while i < even_len {
                    let lo = slice[i];
                    let hi = slice[i + 1];
                    match hi {
                        0 => {
                            if lo < 0x80 {
                                out.push(lo);
                            } else {
                                out.push((lo >> 6) | 0xC0);
                                out.push((lo & 0x3F) | 0x80);
                            }
                        }
                        0x01..=0x07 => {
                            out.push((lo >> 6) | (hi << 2) | 0xC0);
                            out.push((lo & 0x3F) | 0x80);
                        }
                        0xD8..=0xDB => {
                            if even_len - i < 4 {
                                break;
                            }
                            let plane = (((hi & 0x3) << 2) | ((lo >> 6) & 0x3)) + 1;
                            out.push((plane >> 2) | 0xF0);
                            out.push(((lo >> 2) & 0xF) | ((plane & 0x3) << 4) | 0x80);
                            i += 2;
                            let lo2 = slice[i];
                            let hi2 = slice[i + 1];
                            out.push(((lo & 0x3) << 4) | ((hi2 & 0x3) << 2) | (lo2 >> 6) | 0x80);
                            out.push((lo2 & 0x3F) | 0x80);
                        }
                        _ => {
                            out.push((hi >> 4) | 0xE0);
                            out.push(((hi & 0xF) << 2) | (lo >> 6) | 0x80);
                            out.push((lo & 0x3F) | 0x80);
                        }
                    }
                    i += 2;
                }
            }
            Encoding::Big2 => {
                let even_len = slice.len() & !1;
                let mut i = 0;
                while i < even_len {
                    let hi = slice[i];
                    let lo = slice[i + 1];
                    match hi {
                        0 => {
                            if lo < 0x80 {
                                out.push(lo);
                            } else {
                                out.push((lo >> 6) | 0xC0);
                                out.push((lo & 0x3F) | 0x80);
                            }
                        }
                        0x01..=0x07 => {
                            out.push((lo >> 6) | (hi << 2) | 0xC0);
                            out.push((lo & 0x3F) | 0x80);
                        }
                        0xD8..=0xDB => {
                            if even_len - i < 4 {
                                break;
                            }
                            let plane = (((hi & 0x3) << 2) | ((lo >> 6) & 0x3)) + 1;
                            out.push((plane >> 2) | 0xF0);
                            out.push(((lo >> 2) & 0xF) | ((plane & 0x3) << 4) | 0x80);
                            i += 2;
                            let hi2 = slice[i];
                            let lo2 = slice[i + 1];
                            out.push(((lo & 0x3) << 4) | ((hi2 & 0x3) << 2) | (lo2 >> 6) | 0x80);
                            out.push((lo2 & 0x3F) | 0x80);
                        }
                        _ => {
                            out.push((hi >> 4) | 0xE0);
                            out.push(((hi & 0xF) << 2) | (lo >> 6) | 0x80);
                            out.push((lo & 0x3F) | 0x80);
                        }
                    }
                    i += 2;
                }
            }
        }
    }

    fn store_string(self, buf: &[u8], start: usize, end: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(end.saturating_sub(start));
        self.convert_to_utf8(buf, start, end, &mut out);
        out
    }

    fn name_matches_ascii(self, buf: &[u8], mut ptr1: usize, end1: usize, ascii: &[u8]) -> bool {
        let min = self.min_bytes_per_char();
        for &ch in ascii {
            if end1.saturating_sub(ptr1) < min {
                return false;
            }
            if !self.char_matches(buf, ptr1, ch) {
                return false;
            }
            ptr1 += min;
        }
        ptr1 == end1
    }

    fn name_length(self, buf: &[u8], ptr: usize) -> usize {
        let min = self.min_bytes_per_char();
        let mut p = ptr;
        while p + min <= buf.len() {
            match self.byte_type(buf, p) {
                BT_LEAD2 => p += 2,
                BT_LEAD3 => p += 3,
                BT_LEAD4 => p += 4,
                BT_NONASCII | BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => p += min,
                _ => break,
            }
        }
        p - ptr
    }

    fn skip_s(self, buf: &[u8], mut ptr: usize) -> usize {
        let min = self.min_bytes_per_char();
        while ptr + min <= buf.len() {
            match self.byte_type(buf, ptr) {
                BT_LF | BT_CR | BT_S => ptr += min,
                _ => break,
            }
        }
        ptr
    }

    fn update_position(self, buf: &[u8], mut ptr: usize, end: usize, pos: &mut Position) {
        let enc = match self {
            Encoding::Init(_) => Encoding::Utf8,
            other => other,
        };
        let min = enc.min_bytes_per_char();
        while end.saturating_sub(ptr) >= min {
            match enc.byte_type(buf, ptr) {
                BT_LEAD2 => {
                    ptr += 2;
                    pos.column_number += 1;
                }
                BT_LEAD3 => {
                    ptr += 3;
                    pos.column_number += 1;
                }
                BT_LEAD4 => {
                    ptr += 4;
                    pos.column_number += 1;
                }
                BT_LF => {
                    pos.column_number = 0;
                    pos.line_number += 1;
                    ptr += min;
                }
                BT_CR => {
                    pos.line_number += 1;
                    ptr += min;
                    if end.saturating_sub(ptr) >= min && enc.byte_type(buf, ptr) == BT_LF {
                        ptr += min;
                    }
                    pos.column_number = 0;
                }
                _ => {
                    ptr += min;
                    pos.column_number += 1;
                }
            }
        }
    }

    fn is_public_id(self, buf: &[u8], mut ptr: usize, mut end: usize, bad_ptr: &mut usize) -> bool {
        let min = self.min_bytes_per_char();
        ptr += min;
        end -= min;
        while end.saturating_sub(ptr) >= min {
            match self.byte_type(buf, ptr) {
                BT_DIGIT | BT_HEX | BT_MINUS | BT_APOS | BT_LPAR | BT_RPAR | BT_PLUS
                | BT_COMMA | BT_SOL | BT_EQUALS | BT_QUEST | BT_CR | BT_LF | BT_SEMI
                | BT_EXCL | BT_AST | BT_PERCNT | BT_NUM => {}
                BT_S => {
                    if self.char_matches(buf, ptr, b'\t') {
                        *bad_ptr = ptr;
                        return false;
                    }
                }
                BT_NAME | BT_NMSTRT => {
                    if (self.byte_to_ascii(buf, ptr) & !0x7F) == 0 {
                        // valid ASCII name char
                    } else {
                        match self.byte_to_ascii(buf, ptr) {
                            0x24 | 0x40 => {}
                            _ => {
                                *bad_ptr = ptr;
                                return false;
                            }
                        }
                    }
                }
                _ => match self.byte_to_ascii(buf, ptr) {
                    0x24 | 0x40 => {}
                    _ => {
                        *bad_ptr = ptr;
                        return false;
                    }
                },
            }
            ptr += min;
        }
        true
    }

    fn get_atts(self, buf: &[u8], mut ptr: usize, atts: &mut Vec<RawAttribute>) {
        atts.clear();
        #[derive(PartialEq, Eq)]
        enum State {
            Other,
            InName,
            InValue,
        }
        let mut state = State::InName;
        let mut open: u8 = 0;
        let min = self.min_bytes_per_char();
        let mut curr = RawAttribute::default();

        ptr += min;
        while ptr + min <= buf.len() {
            let bt = self.byte_type(buf, ptr);
            match bt {
                BT_LEAD2 => {
                    if state == State::Other {
                        curr.name = ptr;
                        curr.normalized = true;
                        state = State::InName;
                    }
                    ptr += 2 - min;
                }
                BT_LEAD3 => {
                    if state == State::Other {
                        curr.name = ptr;
                        curr.normalized = true;
                        state = State::InName;
                    }
                    ptr += 3 - min;
                }
                BT_LEAD4 => {
                    if state == State::Other {
                        curr.name = ptr;
                        curr.normalized = true;
                        state = State::InName;
                    }
                    ptr += 4 - min;
                }
                BT_NONASCII | BT_NMSTRT | BT_HEX => {
                    if state == State::Other {
                        curr.name = ptr;
                        curr.normalized = true;
                        state = State::InName;
                    }
                }
                BT_QUOT => {
                    if state != State::InValue {
                        curr.value_ptr = ptr + min;
                        state = State::InValue;
                        open = BT_QUOT;
                    } else if open == BT_QUOT {
                        state = State::Other;
                        curr.value_end = ptr;
                        atts.push(curr);
                    }
                }
                BT_APOS => {
                    if state != State::InValue {
                        curr.value_ptr = ptr + min;
                        state = State::InValue;
                        open = BT_APOS;
                    } else if open == BT_APOS {
                        state = State::Other;
                        curr.value_end = ptr;
                        atts.push(curr);
                    }
                }
                BT_AMP => {
                    curr.normalized = false;
                }
                BT_S => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue
                        && curr.normalized
                        && (ptr == curr.value_ptr
                            || self.byte_to_ascii(buf, ptr) != 0x20
                            || (ptr + min * 2 <= buf.len()
                                && (self.byte_to_ascii(buf, ptr + min) == 0x20
                                    || self.byte_type(buf, ptr + min) == open)))
                    {
                        curr.normalized = false;
                    }
                }
                BT_CR | BT_LF => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue {
                        curr.normalized = false;
                    }
                }
                BT_GT | BT_SOL => {
                    if state != State::InValue {
                        return;
                    }
                }
                _ => {}
            }
            ptr += min;
        }
    }

    fn char_ref_number(self, buf: &[u8], mut ptr: usize) -> i32 {
        let min = self.min_bytes_per_char();
        let mut result: i32 = 0;
        ptr += 2 * min;
        if self.char_matches(buf, ptr, b'x') {
            ptr += min;
            while ptr + min <= buf.len() && !self.char_matches(buf, ptr, b';') {
                let c = self.byte_to_ascii(buf, ptr);
                match c as u8 {
                    b'0'..=b'9' => {
                        result <<= 4;
                        result |= c - (b'0' as i32);
                    }
                    b'A'..=b'F' => {
                        result <<= 4;
                        result += 10 + (c - (b'A' as i32));
                    }
                    b'a'..=b'f' => {
                        result <<= 4;
                        result += 10 + (c - (b'a' as i32));
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -1;
                }
                ptr += min;
            }
        } else {
            while ptr + min <= buf.len() && !self.char_matches(buf, ptr, b';') {
                let c = self.byte_to_ascii(buf, ptr);
                result = result.saturating_mul(10).saturating_add(c - (b'0' as i32));
                if result >= 0x110000 {
                    return -1;
                }
                ptr += min;
            }
        }
        check_char_ref_number(result)
    }

    fn predefined_entity_name(self, buf: &[u8], mut ptr: usize, end: usize) -> u8 {
        let min = self.min_bytes_per_char();
        match (end.saturating_sub(ptr)) / min {
            2 => {
                if self.char_matches(buf, ptr + min, b't') {
                    match self.byte_to_ascii(buf, ptr) {
                        0x6C => return b'<',
                        0x67 => return b'>',
                        _ => {}
                    }
                }
            }
            3 => {
                if self.char_matches(buf, ptr, b'a') {
                    ptr += min;
                    if self.char_matches(buf, ptr, b'm') {
                        ptr += min;
                        if self.char_matches(buf, ptr, b'p') {
                            return b'&';
                        }
                    }
                }
            }
            4 => match self.byte_to_ascii(buf, ptr) {
                0x71 => {
                    ptr += min;
                    if self.char_matches(buf, ptr, b'u') {
                        ptr += min;
                        if self.char_matches(buf, ptr, b'o') {
                            ptr += min;
                            if self.char_matches(buf, ptr, b't') {
                                return b'"';
                            }
                        }
                    }
                }
                0x61 => {
                    ptr += min;
                    if self.char_matches(buf, ptr, b'p') {
                        ptr += min;
                        if self.char_matches(buf, ptr, b'o') {
                            ptr += min;
                            if self.char_matches(buf, ptr, b's') {
                                return b'\'';
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        0
    }
}

fn trim_to_complete_utf8_characters(slice: &[u8]) -> usize {
    let mut from_lim = slice.len();
    let mut walked = 0usize;
    while from_lim > 0 {
        let prev = slice[from_lim - 1];
        if (prev & 0xF8) == 0xF0 {
            if walked + 1 >= 4 {
                from_lim += 4 - 1;
                break;
            } else {
                walked = 0;
            }
        } else if (prev & 0xF0) == 0xE0 {
            if walked + 1 >= 3 {
                from_lim += 3 - 1;
                break;
            } else {
                walked = 0;
            }
        } else if (prev & 0xE0) == 0xC0 {
            if walked + 1 >= 2 {
                from_lim += 2 - 1;
                break;
            } else {
                walked = 0;
            }
        } else if (prev & 0x80) == 0x00 {
            break;
        }
        from_lim -= 1;
        walked += 1;
    }
    from_lim
}

macro_rules! invalid_lead_case {
    ($n:expr, $enc:expr, $buf:expr, $ptr:ident, $end:expr, $next_tok_ptr:ident) => {{
        if $end - $ptr < $n {
            return XML_TOK_PARTIAL_CHAR;
        }
        if $enc.is_invalid_char($buf, $ptr, $n) {
            *$next_tok_ptr = $ptr;
            return XML_TOK_INVALID;
        }
        $ptr += $n;
    }};
}

macro_rules! check_name_case {
    ($n:expr, $enc:expr, $buf:expr, $ptr:ident, $end:expr, $next_tok_ptr:ident) => {{
        if $end - $ptr < $n {
            return XML_TOK_PARTIAL_CHAR;
        }
        if $enc.is_invalid_char($buf, $ptr, $n) || !$enc.is_name_char($buf, $ptr, $n) {
            *$next_tok_ptr = $ptr;
            return XML_TOK_INVALID;
        }
        $ptr += $n;
    }};
}

macro_rules! check_nmstrt_case {
    ($n:expr, $enc:expr, $buf:expr, $ptr:ident, $end:expr, $next_tok_ptr:ident) => {{
        if $end - $ptr < $n {
            return XML_TOK_PARTIAL_CHAR;
        }
        if $enc.is_invalid_char($buf, $ptr, $n) || !$enc.is_nmstrt_char($buf, $ptr, $n) {
            *$next_tok_ptr = $ptr;
            return XML_TOK_INVALID;
        }
        $ptr += $n;
    }};
}

#[inline]
fn has_chars(min: usize, ptr: usize, end: usize, count: usize) -> bool {
    end.saturating_sub(ptr) >= count * min
}

#[inline]
fn has_char(min: usize, ptr: usize, end: usize) -> bool {
    end.saturating_sub(ptr) >= min
}

fn scan_comment(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if has_char(min, ptr, end) {
        if !enc.char_matches(buf, ptr, b'-') {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
        ptr += min;
        while has_char(min, ptr, end) {
            match enc.byte_type(buf, ptr) {
                BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
                BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
                BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                BT_MINUS => {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_PARTIAL;
                    }
                    if enc.char_matches(buf, ptr, b'-') {
                        ptr += min;
                        if !has_char(min, ptr, end) {
                            return XML_TOK_PARTIAL;
                        }
                        if !enc.char_matches(buf, ptr, b'>') {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        *next_tok_ptr = ptr + min;
                        return XML_TOK_COMMENT;
                    }
                }
                _ => {
                    ptr += min;
                }
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_decl(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_MINUS => return scan_comment(enc, buf, ptr + min, end, next_tok_ptr),
        BT_LSQB => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_COND_SECT_OPEN;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_PERCNT => {
                if !has_chars(min, ptr, end, 2) {
                    return XML_TOK_PARTIAL;
                }
                match enc.byte_type(buf, ptr + min) {
                    BT_S | BT_CR | BT_LF | BT_PERCNT => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_INVALID;
                    }
                    _ => {}
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DECL_OPEN;
            }
            BT_S | BT_CR | BT_LF => {
                *next_tok_ptr = ptr;
                return XML_TOK_DECL_OPEN;
            }
            BT_NMSTRT | BT_HEX => {
                ptr += min;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn check_pi_target(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    tok_ptr: &mut i32,
) -> bool {
    let min = enc.min_bytes_per_char();
    let mut upper = false;
    *tok_ptr = XML_TOK_PI;
    if end - ptr != min * 3 {
        return true;
    }
    match enc.byte_to_ascii(buf, ptr) {
        0x78 => {}
        0x58 => upper = true,
        _ => return true,
    }
    ptr += min;
    match enc.byte_to_ascii(buf, ptr) {
        0x6D => {}
        0x4D => upper = true,
        _ => return true,
    }
    ptr += min;
    match enc.byte_to_ascii(buf, ptr) {
        0x6C => {}
        0x4C => upper = true,
        _ => return true,
    }
    if upper {
        return false;
    }
    *tok_ptr = XML_TOK_XML_DECL;
    true
}

fn scan_pi(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    let mut tok = 0;
    let target = ptr;
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF => {
                if !check_pi_target(enc, buf, target, ptr, &mut tok) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
                while has_char(min, ptr, end) {
                    match enc.byte_type(buf, ptr) {
                        BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        BT_QUEST => {
                            ptr += min;
                            if !has_char(min, ptr, end) {
                                return XML_TOK_PARTIAL;
                            }
                            if enc.char_matches(buf, ptr, b'>') {
                                *next_tok_ptr = ptr + min;
                                return tok;
                            }
                        }
                        _ => {
                            ptr += min;
                        }
                    }
                }
                return XML_TOK_PARTIAL;
            }
            BT_QUEST => {
                if !check_pi_target(enc, buf, target, ptr, &mut tok) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_PARTIAL;
                }
                if enc.char_matches(buf, ptr, b'>') {
                    *next_tok_ptr = ptr + min;
                    return tok;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_cdata_section(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    const CDATA_LSQB: [u8; 6] = [b'C', b'D', b'A', b'T', b'A', b'['];
    let min = enc.min_bytes_per_char();
    if !has_chars(min, ptr, end, 6) {
        return XML_TOK_PARTIAL;
    }
    for &ch in &CDATA_LSQB {
        if !enc.char_matches(buf, ptr, ch) {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
        ptr += min;
    }
    *next_tok_ptr = ptr;
    XML_TOK_CDATA_SECT_OPEN
}

fn cdata_section_tok(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min = enc.min_bytes_per_char();
    if min > 1 {
        let mut n = end - ptr;
        if (n & (min - 1)) != 0 {
            n &= !(min - 1);
            if n == 0 {
                return XML_TOK_PARTIAL;
            }
            end = ptr + n;
        }
    }
    match enc.byte_type(buf, ptr) {
        BT_RSQB => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return XML_TOK_PARTIAL;
            }
            if enc.char_matches(buf, ptr, b']') {
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_PARTIAL;
                }
                if !enc.char_matches(buf, ptr, b'>') {
                    ptr -= min;
                } else {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_CDATA_SECT_CLOSE;
                }
            }
        }
        BT_CR => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return XML_TOK_PARTIAL;
            }
            if enc.byte_type(buf, ptr) == BT_LF {
                ptr += min;
            }
            *next_tok_ptr = ptr;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_LF => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_NONXML | BT_MALFORM | BT_TRAIL => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
        _ => {
            ptr += min;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => {
                if end - ptr < 2 || enc.is_invalid_char(buf, ptr, 2) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                ptr += 2;
            }
            BT_LEAD3 => {
                if end - ptr < 3 || enc.is_invalid_char(buf, ptr, 3) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                ptr += 3;
            }
            BT_LEAD4 => {
                if end - ptr < 4 || enc.is_invalid_char(buf, ptr, 4) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                ptr += 4;
            }
            BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr += min;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn scan_end_tag(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF => {
                ptr += min;
                while has_char(min, ptr, end) {
                    match enc.byte_type(buf, ptr) {
                        BT_S | BT_CR | BT_LF => {}
                        BT_GT => {
                            *next_tok_ptr = ptr + min;
                            return XML_TOK_END_TAG;
                        }
                        _ => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    ptr += min;
                }
                return XML_TOK_PARTIAL;
            }
            BT_GT => {
                *next_tok_ptr = ptr + min;
                return XML_TOK_END_TAG;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_hex_char_ref(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_DIGIT | BT_HEX => {}
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        ptr += min;
        while has_char(min, ptr, end) {
            match enc.byte_type(buf, ptr) {
                BT_DIGIT | BT_HEX => {}
                BT_SEMI => {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_CHAR_REF;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr += min;
        }
    }
    XML_TOK_PARTIAL
}

fn scan_char_ref(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if has_char(min, ptr, end) {
        if enc.char_matches(buf, ptr, b'x') {
            return scan_hex_char_ref(enc, buf, ptr + min, end, next_tok_ptr);
        }
        match enc.byte_type(buf, ptr) {
            BT_DIGIT => {}
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        ptr += min;
        while has_char(min, ptr, end) {
            match enc.byte_type(buf, ptr) {
                BT_DIGIT => {}
                BT_SEMI => {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_CHAR_REF;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr += min;
        }
    }
    XML_TOK_PARTIAL
}

fn scan_ref(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_NUM => return scan_char_ref(enc, buf, ptr + min, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_SEMI => {
                *next_tok_ptr = ptr + min;
                return XML_TOK_ENTITY_REF;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_atts(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF | BT_EQUALS => {
                if enc.byte_type(buf, ptr) != BT_EQUALS {
                    loop {
                        ptr += min;
                        if !has_char(min, ptr, end) {
                            return XML_TOK_PARTIAL;
                        }
                        let t = enc.byte_type(buf, ptr);
                        if t == BT_EQUALS {
                            break;
                        }
                        match t {
                            BT_S | BT_LF | BT_CR => {}
                            _ => {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                }
                let open;
                loop {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_PARTIAL;
                    }
                    let t = enc.byte_type(buf, ptr);
                    if t == BT_QUOT || t == BT_APOS {
                        open = t;
                        break;
                    }
                    match t {
                        BT_S | BT_LF | BT_CR => {}
                        _ => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                }
                ptr += min;
                loop {
                    if !has_char(min, ptr, end) {
                        return XML_TOK_PARTIAL;
                    }
                    let t = enc.byte_type(buf, ptr);
                    if t == open {
                        break;
                    }
                    match t {
                        BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
                        BT_NONXML | BT_MALFORM | BT_TRAIL | BT_LT => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        BT_AMP => {
                            let tok = scan_ref(enc, buf, ptr + min, end, &mut ptr);
                            if tok <= 0 {
                                if tok == XML_TOK_INVALID {
                                    *next_tok_ptr = ptr;
                                }
                                return tok;
                            }
                        }
                        _ => {
                            ptr += min;
                        }
                    }
                }
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_PARTIAL;
                }
                match enc.byte_type(buf, ptr) {
                    BT_S | BT_CR | BT_LF => {}
                    BT_SOL => {
                        ptr += min;
                        if !has_char(min, ptr, end) {
                            return XML_TOK_PARTIAL;
                        }
                        if !enc.char_matches(buf, ptr, b'>') {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        *next_tok_ptr = ptr + min;
                        return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                    }
                    BT_GT => {
                        *next_tok_ptr = ptr + min;
                        return XML_TOK_START_TAG_WITH_ATTS;
                    }
                    _ => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                loop {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_PARTIAL;
                    }
                    match enc.byte_type(buf, ptr) {
                        BT_NONASCII => {
                            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr += min;
                            break;
                        }
                        BT_NMSTRT | BT_HEX => {
                            ptr += min;
                            break;
                        }
                        BT_LEAD2 => {
                            check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr);
                            break;
                        }
                        BT_LEAD3 => {
                            check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr);
                            break;
                        }
                        BT_LEAD4 => {
                            check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr);
                            break;
                        }
                        BT_S | BT_CR | BT_LF => continue,
                        BT_GT => {
                            *next_tok_ptr = ptr + min;
                            return XML_TOK_START_TAG_WITH_ATTS;
                        }
                        BT_SOL => {
                            ptr += min;
                            if !has_char(min, ptr, end) {
                                return XML_TOK_PARTIAL;
                            }
                            if !enc.char_matches(buf, ptr, b'>') {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *next_tok_ptr = ptr + min;
                            return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                        }
                        _ => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                }
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_lt(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_EXCL => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return XML_TOK_PARTIAL;
            }
            match enc.byte_type(buf, ptr) {
                BT_MINUS => return scan_comment(enc, buf, ptr + min, end, next_tok_ptr),
                BT_LSQB => return scan_cdata_section(enc, buf, ptr + min, end, next_tok_ptr),
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
        }
        BT_QUEST => return scan_pi(enc, buf, ptr + min, end, next_tok_ptr),
        BT_SOL => return scan_end_tag(enc, buf, ptr + min, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF => {
                ptr += min;
                while has_char(min, ptr, end) {
                    match enc.byte_type(buf, ptr) {
                        BT_NONASCII => {
                            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr += min;
                        }
                        BT_NMSTRT | BT_HEX => {
                            ptr += min;
                        }
                        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
                        BT_GT => {
                            *next_tok_ptr = ptr + min;
                            return XML_TOK_START_TAG_NO_ATTS;
                        }
                        BT_SOL => {
                            ptr += min;
                            if !has_char(min, ptr, end) {
                                return XML_TOK_PARTIAL;
                            }
                            if !enc.char_matches(buf, ptr, b'>') {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *next_tok_ptr = ptr + min;
                            return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                        }
                        BT_S | BT_CR | BT_LF => {
                            ptr += min;
                            continue;
                        }
                        _ => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    return scan_atts(enc, buf, ptr, end, next_tok_ptr);
                }
                return XML_TOK_PARTIAL;
            }
            BT_GT => {
                *next_tok_ptr = ptr + min;
                return XML_TOK_START_TAG_NO_ATTS;
            }
            BT_SOL => {
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_PARTIAL;
                }
                if !enc.char_matches(buf, ptr, b'>') {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min;
                return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn content_tok_normal(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min = enc.min_bytes_per_char();
    if min > 1 {
        let mut n = end - ptr;
        if (n & (min - 1)) != 0 {
            n &= !(min - 1);
            if n == 0 {
                return XML_TOK_PARTIAL;
            }
            end = ptr + n;
        }
    }
    match enc.byte_type(buf, ptr) {
        BT_LT => return scan_lt(enc, buf, ptr + min, end, next_tok_ptr),
        BT_AMP => return scan_ref(enc, buf, ptr + min, end, next_tok_ptr),
        BT_CR => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return XML_TOK_TRAILING_CR;
            }
            if enc.byte_type(buf, ptr) == BT_LF {
                ptr += min;
            }
            *next_tok_ptr = ptr;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_LF => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_RSQB => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return XML_TOK_TRAILING_RSQB;
            }
            if enc.char_matches(buf, ptr, b']') {
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_TRAILING_RSQB;
                }
                if !enc.char_matches(buf, ptr, b'>') {
                    ptr -= min;
                } else {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
        }
        BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_NONXML | BT_MALFORM | BT_TRAIL => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
        _ => {
            ptr += min;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => {
                if end - ptr < 2 || enc.is_invalid_char(buf, ptr, 2) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                ptr += 2;
            }
            BT_LEAD3 => {
                if end - ptr < 3 || enc.is_invalid_char(buf, ptr, 3) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                ptr += 3;
            }
            BT_LEAD4 => {
                if end - ptr < 4 || enc.is_invalid_char(buf, ptr, 4) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                ptr += 4;
            }
            BT_RSQB => {
                if has_chars(min, ptr, end, 2) {
                    if !enc.char_matches(buf, ptr + min, b']') {
                        ptr += min;
                        continue;
                    }
                    if has_chars(min, ptr, end, 3) {
                        if !enc.char_matches(buf, ptr + 2 * min, b'>') {
                            ptr += min;
                            continue;
                        }
                        *next_tok_ptr = ptr + 2 * min;
                        return XML_TOK_INVALID;
                    }
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr += min;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn scan_percent(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_S | BT_LF | BT_CR | BT_PERCNT => {
            *next_tok_ptr = ptr;
            return XML_TOK_PERCENT;
        }
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_SEMI => {
                *next_tok_ptr = ptr + min;
                return XML_TOK_PARAM_ENTITY_REF;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_pound_name(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                *next_tok_ptr = ptr;
                return XML_TOK_POUND_NAME;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    -XML_TOK_POUND_NAME
}

fn scan_lit(
    open: u8,
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min = enc.min_bytes_per_char();
    while has_char(min, ptr, end) {
        let t = enc.byte_type(buf, ptr);
        match t {
            BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            BT_QUOT | BT_APOS => {
                ptr += min;
                if t == open {
                    if !has_char(min, ptr, end) {
                        return -XML_TOK_LITERAL;
                    }
                    *next_tok_ptr = ptr;
                    match enc.byte_type(buf, ptr) {
                        BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                            return XML_TOK_LITERAL;
                        }
                        _ => return XML_TOK_INVALID,
                    }
                }
            }
            _ => {
                ptr += min;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn prolog_tok_normal(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min = enc.min_bytes_per_char();
    if min > 1 {
        let mut n = end - ptr;
        if (n & (min - 1)) != 0 {
            n &= !(min - 1);
            if n == 0 {
                return XML_TOK_PARTIAL;
            }
            end = ptr + n;
        }
    }
    let tok: i32;
    match enc.byte_type(buf, ptr) {
        BT_QUOT => return scan_lit(BT_QUOT, enc, buf, ptr + min, end, next_tok_ptr),
        BT_APOS => return scan_lit(BT_APOS, enc, buf, ptr + min, end, next_tok_ptr),
        BT_LT => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return XML_TOK_PARTIAL;
            }
            match enc.byte_type(buf, ptr) {
                BT_EXCL => return scan_decl(enc, buf, ptr + min, end, next_tok_ptr),
                BT_QUEST => return scan_pi(enc, buf, ptr + min, end, next_tok_ptr),
                BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                    *next_tok_ptr = ptr - min;
                    return XML_TOK_INSTANCE_START;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
        }
        BT_CR => {
            if ptr + min == end {
                *next_tok_ptr = end;
                return -XML_TOK_PROLOG_S;
            }
            loop {
                ptr += min;
                if !has_char(min, ptr, end) {
                    break;
                }
                match enc.byte_type(buf, ptr) {
                    BT_S | BT_LF => {}
                    BT_CR => {
                        if ptr + min == end {
                            *next_tok_ptr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                    _ => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_PROLOG_S;
                    }
                }
            }
            *next_tok_ptr = ptr;
            return XML_TOK_PROLOG_S;
        }
        BT_S | BT_LF => {
            loop {
                ptr += min;
                if !has_char(min, ptr, end) {
                    break;
                }
                match enc.byte_type(buf, ptr) {
                    BT_S | BT_LF => {}
                    BT_CR => {
                        if ptr + min == end {
                            *next_tok_ptr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                    _ => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_PROLOG_S;
                    }
                }
            }
            *next_tok_ptr = ptr;
            return XML_TOK_PROLOG_S;
        }
        BT_PERCNT => return scan_percent(enc, buf, ptr + min, end, next_tok_ptr),
        BT_COMMA => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_COMMA;
        }
        BT_LSQB => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_OPEN_BRACKET;
        }
        BT_RSQB => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return -XML_TOK_CLOSE_BRACKET;
            }
            if enc.char_matches(buf, ptr, b']') {
                if !has_chars(min, ptr, end, 2) {
                    return XML_TOK_PARTIAL;
                }
                if enc.char_matches(buf, ptr + min, b'>') {
                    *next_tok_ptr = ptr + 2 * min;
                    return XML_TOK_COND_SECT_CLOSE;
                }
            }
            *next_tok_ptr = ptr;
            return XML_TOK_CLOSE_BRACKET;
        }
        BT_LPAR => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_OPEN_PAREN;
        }
        BT_RPAR => {
            ptr += min;
            if !has_char(min, ptr, end) {
                return -XML_TOK_CLOSE_PAREN;
            }
            match enc.byte_type(buf, ptr) {
                BT_AST => {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_CLOSE_PAREN_ASTERISK;
                }
                BT_QUEST => {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_CLOSE_PAREN_QUESTION;
                }
                BT_PLUS => {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_CLOSE_PAREN_PLUS;
                }
                BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_CLOSE_PAREN;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
        }
        BT_VERBAR => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_OR;
        }
        BT_GT => {
            *next_tok_ptr = ptr + min;
            return XML_TOK_DECL_CLOSE;
        }
        BT_NUM => return scan_pound_name(enc, buf, ptr + min, end, next_tok_ptr),
        BT_LEAD2 => {
            if end - ptr < 2 {
                return XML_TOK_PARTIAL_CHAR;
            }
            if enc.is_invalid_char(buf, ptr, 2) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            if enc.is_nmstrt_char(buf, ptr, 2) {
                ptr += 2;
                tok = XML_TOK_NAME;
            } else if enc.is_name_char(buf, ptr, 2) {
                ptr += 2;
                tok = XML_TOK_NMTOKEN;
            } else {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        BT_LEAD3 => {
            if end - ptr < 3 {
                return XML_TOK_PARTIAL_CHAR;
            }
            if enc.is_invalid_char(buf, ptr, 3) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            if enc.is_nmstrt_char(buf, ptr, 3) {
                ptr += 3;
                tok = XML_TOK_NAME;
            } else if enc.is_name_char(buf, ptr, 3) {
                ptr += 3;
                tok = XML_TOK_NMTOKEN;
            } else {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        BT_LEAD4 => {
            if end - ptr < 4 {
                return XML_TOK_PARTIAL_CHAR;
            }
            if enc.is_invalid_char(buf, ptr, 4) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            if enc.is_nmstrt_char(buf, ptr, 4) {
                ptr += 4;
                tok = XML_TOK_NAME;
            } else if enc.is_name_char(buf, ptr, 4) {
                ptr += 4;
                tok = XML_TOK_NMTOKEN;
            } else {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        BT_NMSTRT | BT_HEX => {
            tok = XML_TOK_NAME;
            ptr += min;
        }
        BT_DIGIT | BT_NAME | BT_MINUS => {
            tok = XML_TOK_NMTOKEN;
            ptr += min;
        }
        BT_NONASCII => {
            if enc.is_nmstrt_char_minbpc(buf, ptr) {
                ptr += min;
                tok = XML_TOK_NAME;
            } else if enc.is_name_char_minbpc(buf, ptr) {
                ptr += min;
                tok = XML_TOK_NMTOKEN;
            } else {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR
            | BT_LF => {
                *next_tok_ptr = ptr;
                return tok;
            }
            BT_PLUS => {
                if tok == XML_TOK_NMTOKEN {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min;
                return XML_TOK_NAME_PLUS;
            }
            BT_AST => {
                if tok == XML_TOK_NMTOKEN {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min;
                return XML_TOK_NAME_ASTERISK;
            }
            BT_QUEST => {
                if tok == XML_TOK_NMTOKEN {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min;
                return XML_TOK_NAME_QUESTION;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    -tok
}

fn attribute_value_tok(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    let start = ptr;
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => ptr += 2,
            BT_LEAD3 => ptr += 3,
            BT_LEAD4 => ptr += 4,
            BT_AMP => {
                if ptr == start {
                    return scan_ref(enc, buf, ptr + min, end, next_tok_ptr);
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_LT => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            BT_LF => {
                if ptr == start {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_CR => {
                if ptr == start {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_TRAILING_CR;
                    }
                    if enc.byte_type(buf, ptr) == BT_LF {
                        ptr += min;
                    }
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_S => {
                if ptr == start {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_ATTRIBUTE_VALUE_S;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr += min;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn entity_value_tok(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min = enc.min_bytes_per_char();
    if !has_char(min, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    let start = ptr;
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => ptr += 2,
            BT_LEAD3 => ptr += 3,
            BT_LEAD4 => ptr += 4,
            BT_AMP => {
                if ptr == start {
                    return scan_ref(enc, buf, ptr + min, end, next_tok_ptr);
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_PERCNT => {
                if ptr == start {
                    let tok = scan_percent(enc, buf, ptr + min, end, next_tok_ptr);
                    return if tok == XML_TOK_PERCENT {
                        XML_TOK_INVALID
                    } else {
                        tok
                    };
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_LF => {
                if ptr == start {
                    *next_tok_ptr = ptr + min;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_CR => {
                if ptr == start {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_TRAILING_CR;
                    }
                    if enc.byte_type(buf, ptr) == BT_LF {
                        ptr += min;
                    }
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr += min;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn ignore_section_tok(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let mut level = 0i32;
    let min = enc.min_bytes_per_char();
    if min > 1 {
        let mut n = end.saturating_sub(ptr);
        if (n & (min - 1)) != 0 {
            n &= !(min - 1);
            end = ptr + n;
        }
    }
    while has_char(min, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            BT_LT => {
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_PARTIAL;
                }
                if enc.char_matches(buf, ptr, b'!') {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_PARTIAL;
                    }
                    if enc.char_matches(buf, ptr, b'[') {
                        level += 1;
                        ptr += min;
                    }
                }
            }
            BT_RSQB => {
                ptr += min;
                if !has_char(min, ptr, end) {
                    return XML_TOK_PARTIAL;
                }
                if enc.char_matches(buf, ptr, b']') {
                    ptr += min;
                    if !has_char(min, ptr, end) {
                        return XML_TOK_PARTIAL;
                    }
                    if enc.char_matches(buf, ptr, b'>') {
                        ptr += min;
                        if level == 0 {
                            *next_tok_ptr = ptr;
                            return XML_TOK_IGNORE_SECT;
                        }
                        level -= 1;
                    }
                }
            }
            _ => {
                ptr += min;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn init_scan(
    init_idx: InitEncIndex,
    state: usize,
    buf: &[u8],
    ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
    enc_out: &mut Encoding,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    if ptr + 1 == end {
        match init_idx {
            InitEncIndex::Utf16 | InitEncIndex::Utf16Le | InitEncIndex::Utf16Be => {
                return XML_TOK_PARTIAL;
            }
            _ => {}
        }
        match buf[ptr] {
            0xFE | 0xFF | 0xEF => {
                if init_idx == InitEncIndex::Iso8859_1 && state == XML_CONTENT_STATE {
                    // fallthrough to normal scan
                } else {
                    return XML_TOK_PARTIAL;
                }
            }
            0x00 | 0x3C => return XML_TOK_PARTIAL,
            _ => {}
        }
    } else {
        let pair = ((buf[ptr] as u16) << 8) | (buf[ptr + 1] as u16);
        match pair {
            0xFEFF => {
                if !(init_idx == InitEncIndex::Iso8859_1 && state == XML_CONTENT_STATE) {
                    *next_tok_ptr = ptr + 2;
                    *enc_out = Encoding::Big2;
                    return XML_TOK_BOM;
                }
            }
            0x3C00 => {
                if !((init_idx == InitEncIndex::Utf16Be || init_idx == InitEncIndex::Utf16)
                    && state == XML_CONTENT_STATE)
                {
                    *enc_out = Encoding::Little2;
                    return xml_tok(*enc_out, state, buf, ptr, end, next_tok_ptr, enc_out);
                }
            }
            0xFFFE => {
                if !(init_idx == InitEncIndex::Iso8859_1 && state == XML_CONTENT_STATE) {
                    *next_tok_ptr = ptr + 2;
                    *enc_out = Encoding::Little2;
                    return XML_TOK_BOM;
                }
            }
            0xEFBB => {
                let skip = state == XML_CONTENT_STATE
                    && matches!(
                        init_idx,
                        InitEncIndex::Iso8859_1
                            | InitEncIndex::Utf16Be
                            | InitEncIndex::Utf16Le
                            | InitEncIndex::Utf16
                    );
                if !skip {
                    if ptr + 2 == end {
                        return XML_TOK_PARTIAL;
                    }
                    if buf[ptr + 2] == 0xBF {
                        *next_tok_ptr = ptr + 3;
                        *enc_out = Encoding::Utf8;
                        return XML_TOK_BOM;
                    }
                }
            }
            _ => {
                if buf[ptr] == 0 {
                    if !(state == XML_CONTENT_STATE && init_idx == InitEncIndex::Utf16Le) {
                        *enc_out = Encoding::Big2;
                        return xml_tok(*enc_out, state, buf, ptr, end, next_tok_ptr, enc_out);
                    }
                } else if buf[ptr + 1] == 0 && state != XML_CONTENT_STATE {
                    *enc_out = Encoding::Little2;
                    return xml_tok(*enc_out, state, buf, ptr, end, next_tok_ptr, enc_out);
                }
            }
        }
    }
    *enc_out = init_idx.to_encoding();
    xml_tok(*enc_out, state, buf, ptr, end, next_tok_ptr, enc_out)
}

fn xml_tok(
    enc: Encoding,
    state: usize,
    buf: &[u8],
    ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
    enc_out: &mut Encoding,
) -> i32 {
    if let Encoding::Init(idx) = enc {
        return init_scan(idx, state, buf, ptr, end, next_tok_ptr, enc_out);
    }
    if state == XML_PROLOG_STATE {
        prolog_tok_normal(enc, buf, ptr, end, next_tok_ptr)
    } else {
        content_tok_normal(enc, buf, ptr, end, next_tok_ptr)
    }
}

#[inline]
fn to_ascii(enc: Encoding, buf: &[u8], ptr: usize, end: usize) -> i32 {
    let min = enc.min_bytes_per_char();
    if end.saturating_sub(ptr) < min {
        return -1;
    }
    let c = enc.byte_to_ascii(buf, ptr);
    if (0..0x80).contains(&c) {
        c
    } else {
        -1
    }
}

#[inline]
fn is_space(c: i32) -> bool {
    matches!(c, 0x20 | 0x0D | 0x0A | 0x09)
}

fn parse_pseudo_attribute(
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    name_ptr: &mut Option<usize>,
    name_end_ptr: &mut usize,
    val_ptr: &mut usize,
    next_tok_ptr: &mut usize,
) -> bool {
    let min = enc.min_bytes_per_char();
    if ptr == end {
        *name_ptr = None;
        return true;
    }
    if !is_space(to_ascii(enc, buf, ptr, end)) {
        *next_tok_ptr = ptr;
        return false;
    }
    loop {
        ptr += min;
        if !is_space(to_ascii(enc, buf, ptr, end)) {
            break;
        }
    }
    if ptr == end {
        *name_ptr = None;
        return true;
    }
    *name_ptr = Some(ptr);
    let name_start = ptr;
    loop {
        let mut c = to_ascii(enc, buf, ptr, end);
        if c == -1 {
            *next_tok_ptr = ptr;
            return false;
        }
        if c == (b'=' as i32) {
            *name_end_ptr = ptr;
            break;
        }
        if is_space(c) {
            *name_end_ptr = ptr;
            loop {
                ptr += min;
                c = to_ascii(enc, buf, ptr, end);
                if !is_space(c) {
                    break;
                }
            }
            if c != (b'=' as i32) {
                *next_tok_ptr = ptr;
                return false;
            }
            break;
        }
        ptr += min;
    }
    if ptr == name_start {
        *next_tok_ptr = ptr;
        return false;
    }
    ptr += min;
    let mut c = to_ascii(enc, buf, ptr, end);
    while is_space(c) {
        ptr += min;
        c = to_ascii(enc, buf, ptr, end);
    }
    if c != (b'"' as i32) && c != (b'\'' as i32) {
        *next_tok_ptr = ptr;
        return false;
    }
    let open = c;
    ptr += min;
    *val_ptr = ptr;
    loop {
        c = to_ascii(enc, buf, ptr, end);
        if c == open {
            break;
        }
        let cu = c as u8;
        if !cu.is_ascii_lowercase()
            && !cu.is_ascii_uppercase()
            && !cu.is_ascii_digit()
            && cu != b'.'
            && cu != b'-'
            && cu != b'_'
        {
            *next_tok_ptr = ptr;
            return false;
        }
        ptr += min;
    }
    *next_tok_ptr = ptr + min;
    true
}

fn check_xml_decl_version_num(enc: Encoding, buf: &[u8], mut val: usize, val_end: usize) -> bool {
    let min = enc.min_bytes_per_char();
    if val_end.saturating_sub(val) < 2 * min
        || !enc.name_matches_ascii(buf, val, val + 2 * min, b"1.")
    {
        return false;
    }
    val += 2 * min;
    if val == val_end {
        return false;
    }
    while val != val_end {
        let c = to_ascii(enc, buf, val, val_end);
        if c < (b'0' as i32) || c > (b'9' as i32) {
            return false;
        }
        val += min;
    }
    true
}

fn find_encoding(enc: Encoding, buf: &[u8], ptr: usize, end: usize) -> Option<Encoding> {
    const ENCODING_MAX: usize = 128;
    let mut utf8_buf = Vec::new();
    enc.convert_to_utf8(buf, ptr, end, &mut utf8_buf);
    if utf8_buf.len() >= ENCODING_MAX {
        return None;
    }
    if streqci(&utf8_buf, b"UTF-16") && enc.min_bytes_per_char() == 2 {
        return Some(enc);
    }
    get_encoding_index(&utf8_buf).map(|idx| idx.to_encoding())
}

#[allow(clippy::too_many_arguments)]
fn xml_parse_xml_decl(
    is_general_text_entity: bool,
    enc: Encoding,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    bad_ptr: &mut Option<usize>,
    version_ptr: &mut Option<(usize, usize)>,
    encoding_name_ptr: &mut Option<usize>,
    named_encoding_ptr: &mut Option<Encoding>,
    standalone_ptr: &mut i32,
) -> bool {
    let min = enc.min_bytes_per_char();
    let mut val = 0usize;
    let mut name: Option<usize> = None;
    let mut name_end = 0usize;

    ptr += 5 * min;
    end -= 2 * min;
    if !parse_pseudo_attribute(
        enc,
        buf,
        ptr,
        end,
        &mut name,
        &mut name_end,
        &mut val,
        &mut ptr,
    ) || name.is_none()
    {
        *bad_ptr = Some(ptr);
        return false;
    }
    let nm = name.unwrap_or(ptr);
    if !enc.name_matches_ascii(buf, nm, name_end, b"version") {
        if !is_general_text_entity {
            *bad_ptr = Some(nm);
            return false;
        }
    } else {
        *version_ptr = Some((val, ptr));
        if val == ptr - min {
            *bad_ptr = Some(val);
            return false;
        }
        if !check_xml_decl_version_num(enc, buf, val, ptr - min) {
            *bad_ptr = Some(val);
            return false;
        }
        if !parse_pseudo_attribute(
            enc,
            buf,
            ptr,
            end,
            &mut name,
            &mut name_end,
            &mut val,
            &mut ptr,
        ) {
            *bad_ptr = Some(ptr);
            return false;
        }
        if name.is_none() {
            if is_general_text_entity {
                *bad_ptr = Some(ptr);
                return false;
            }
            return true;
        }
    }
    let nm = name.unwrap_or(ptr);
    if enc.name_matches_ascii(buf, nm, name_end, b"encoding") {
        let c = to_ascii(enc, buf, val, end);
        let cu = c as u8;
        if !(c >= 0 && (cu.is_ascii_lowercase() || cu.is_ascii_uppercase())) {
            *bad_ptr = Some(val);
            return false;
        }
        *encoding_name_ptr = Some(val);
        *named_encoding_ptr = find_encoding(enc, buf, val, ptr - min);
        if !parse_pseudo_attribute(
            enc,
            buf,
            ptr,
            end,
            &mut name,
            &mut name_end,
            &mut val,
            &mut ptr,
        ) {
            *bad_ptr = Some(ptr);
            return false;
        }
        if name.is_none() {
            return true;
        }
    }
    let nm = name.unwrap_or(ptr);
    if !enc.name_matches_ascii(buf, nm, name_end, b"standalone") || is_general_text_entity {
        *bad_ptr = Some(nm);
        return false;
    }
    if enc.name_matches_ascii(buf, val, ptr - min, b"yes") {
        *standalone_ptr = 1;
    } else if enc.name_matches_ascii(buf, val, ptr - min, b"no") {
        *standalone_ptr = 0;
    } else {
        *bad_ptr = Some(val);
        return false;
    }
    while is_space(to_ascii(enc, buf, ptr, end)) {
        ptr += min;
    }
    if ptr != end {
        *bad_ptr = Some(ptr);
        return false;
    }
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrologHandler {
    Prolog0,
    Prolog1,
    Prolog2,
    Doctype0,
    Doctype1,
    Doctype2,
    Doctype3,
    Doctype4,
    Doctype5,
    InternalSubset,
    ExternalSubset0,
    ExternalSubset1,
    CondSect0,
    CondSect1,
    CondSect2,
    Entity0,
    Entity1,
    Entity2,
    Entity3,
    Entity4,
    Entity5,
    Entity6,
    Entity7,
    Entity8,
    Entity9,
    Entity10,
    Notation0,
    Notation1,
    Notation2,
    Notation3,
    Notation4,
    Attlist0,
    Attlist1,
    Attlist2,
    Attlist3,
    Attlist4,
    Attlist5,
    Attlist6,
    Attlist7,
    Attlist8,
    Attlist9,
    Element0,
    Element1,
    Element2,
    Element3,
    Element4,
    Element5,
    Element6,
    Element7,
    DeclClose,
    Error,
}

struct PrologState {
    handler: PrologHandler,
    level: u32,
    role_none: i32,
    include_level: u32,
    document_entity: bool,
    in_entity_value: bool,
}

impl PrologState {
    fn new() -> Self {
        Self {
            handler: PrologHandler::Prolog0,
            level: 0,
            role_none: 0,
            include_level: 0,
            document_entity: true,
            in_entity_value: false,
        }
    }

    fn set_top_level(&mut self) {
        self.handler = if self.document_entity {
            PrologHandler::InternalSubset
        } else {
            PrologHandler::ExternalSubset1
        };
    }

    fn common(&mut self, tok: i32) -> i32 {
        if !self.document_entity && tok == XML_TOK_PARAM_ENTITY_REF {
            return XML_ROLE_INNER_PARAM_ENTITY_REF;
        }
        self.handler = PrologHandler::Error;
        XML_ROLE_ERROR
    }

    fn token_role(&mut self, tok: i32, buf: &[u8], ptr: usize, end: usize, enc: Encoding) -> i32 {
        let min = enc.min_bytes_per_char();
        match self.handler {
            PrologHandler::Prolog0 => match tok {
                XML_TOK_PROLOG_S => {
                    self.handler = PrologHandler::Prolog1;
                    XML_ROLE_NONE
                }
                XML_TOK_XML_DECL => {
                    self.handler = PrologHandler::Prolog1;
                    XML_ROLE_XML_DECL
                }
                XML_TOK_PI => {
                    self.handler = PrologHandler::Prolog1;
                    XML_ROLE_PI
                }
                XML_TOK_COMMENT => {
                    self.handler = PrologHandler::Prolog1;
                    XML_ROLE_COMMENT
                }
                XML_TOK_BOM => XML_ROLE_NONE,
                XML_TOK_DECL_OPEN => {
                    if !enc.name_matches_ascii(buf, ptr + 2 * min, end, b"DOCTYPE") {
                        self.common(tok)
                    } else {
                        self.handler = PrologHandler::Doctype0;
                        XML_ROLE_DOCTYPE_NONE
                    }
                }
                XML_TOK_INSTANCE_START => {
                    self.handler = PrologHandler::Error;
                    XML_ROLE_INSTANCE_START
                }
                _ => self.common(tok),
            },
            PrologHandler::Prolog1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_PI => XML_ROLE_PI,
                XML_TOK_COMMENT => XML_ROLE_COMMENT,
                XML_TOK_BOM => XML_ROLE_NONE,
                XML_TOK_DECL_OPEN => {
                    if !enc.name_matches_ascii(buf, ptr + 2 * min, end, b"DOCTYPE") {
                        self.common(tok)
                    } else {
                        self.handler = PrologHandler::Doctype0;
                        XML_ROLE_DOCTYPE_NONE
                    }
                }
                XML_TOK_INSTANCE_START => {
                    self.handler = PrologHandler::Error;
                    XML_ROLE_INSTANCE_START
                }
                _ => self.common(tok),
            },
            PrologHandler::Prolog2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_PI => XML_ROLE_PI,
                XML_TOK_COMMENT => XML_ROLE_COMMENT,
                XML_TOK_INSTANCE_START => {
                    self.handler = PrologHandler::Error;
                    XML_ROLE_INSTANCE_START
                }
                _ => self.common(tok),
            },
            PrologHandler::Doctype0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Doctype1;
                    XML_ROLE_DOCTYPE_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Doctype1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = PrologHandler::InternalSubset;
                    XML_ROLE_DOCTYPE_INTERNAL_SUBSET
                }
                XML_TOK_DECL_CLOSE => {
                    self.handler = PrologHandler::Prolog2;
                    XML_ROLE_DOCTYPE_CLOSE
                }
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"SYSTEM") {
                        self.handler = PrologHandler::Doctype3;
                        return XML_ROLE_DOCTYPE_NONE;
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"PUBLIC") {
                        self.handler = PrologHandler::Doctype2;
                        return XML_ROLE_DOCTYPE_NONE;
                    }
                    self.common(tok)
                }
                _ => self.common(tok),
            },
            PrologHandler::Doctype2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Doctype3;
                    XML_ROLE_DOCTYPE_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Doctype3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Doctype4;
                    XML_ROLE_DOCTYPE_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Doctype4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = PrologHandler::InternalSubset;
                    XML_ROLE_DOCTYPE_INTERNAL_SUBSET
                }
                XML_TOK_DECL_CLOSE => {
                    self.handler = PrologHandler::Prolog2;
                    XML_ROLE_DOCTYPE_CLOSE
                }
                _ => self.common(tok),
            },
            PrologHandler::Doctype5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.handler = PrologHandler::Prolog2;
                    XML_ROLE_DOCTYPE_CLOSE
                }
                _ => self.common(tok),
            },
            PrologHandler::InternalSubset => self.internal_subset(tok, buf, ptr, end, enc),
            PrologHandler::ExternalSubset0 => {
                self.handler = PrologHandler::ExternalSubset1;
                if tok == XML_TOK_XML_DECL {
                    XML_ROLE_TEXT_DECL
                } else {
                    self.external_subset1(tok, buf, ptr, end, enc)
                }
            }
            PrologHandler::ExternalSubset1 => self.external_subset1(tok, buf, ptr, end, enc),
            PrologHandler::CondSect0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"INCLUDE") {
                        self.handler = PrologHandler::CondSect1;
                        return XML_ROLE_NONE;
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"IGNORE") {
                        self.handler = PrologHandler::CondSect2;
                        return XML_ROLE_NONE;
                    }
                    self.common(tok)
                }
                _ => self.common(tok),
            },
            PrologHandler::CondSect1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = PrologHandler::ExternalSubset1;
                    self.include_level += 1;
                    XML_ROLE_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::CondSect2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = PrologHandler::ExternalSubset1;
                    XML_ROLE_IGNORE_SECT
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_PERCENT => {
                    self.handler = PrologHandler::Entity1;
                    XML_ROLE_ENTITY_NONE
                }
                XML_TOK_NAME => {
                    self.handler = PrologHandler::Entity2;
                    XML_ROLE_GENERAL_ENTITY_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    self.handler = PrologHandler::Entity7;
                    XML_ROLE_PARAM_ENTITY_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"SYSTEM") {
                        self.handler = PrologHandler::Entity4;
                        return XML_ROLE_ENTITY_NONE;
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"PUBLIC") {
                        self.handler = PrologHandler::Entity3;
                        return XML_ROLE_ENTITY_NONE;
                    }
                    self.common(tok)
                }
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_ENTITY_NONE;
                    XML_ROLE_ENTITY_VALUE
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Entity4;
                    XML_ROLE_ENTITY_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Entity5;
                    XML_ROLE_ENTITY_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_ENTITY_COMPLETE
                }
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"NDATA") {
                        self.handler = PrologHandler::Entity6;
                        return XML_ROLE_ENTITY_NONE;
                    }
                    self.common(tok)
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity6 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_ENTITY_NONE;
                    XML_ROLE_ENTITY_NOTATION_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity7 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"SYSTEM") {
                        self.handler = PrologHandler::Entity9;
                        return XML_ROLE_ENTITY_NONE;
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"PUBLIC") {
                        self.handler = PrologHandler::Entity8;
                        return XML_ROLE_ENTITY_NONE;
                    }
                    self.common(tok)
                }
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_ENTITY_NONE;
                    XML_ROLE_ENTITY_VALUE
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity8 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Entity9;
                    XML_ROLE_ENTITY_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity9 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Entity10;
                    XML_ROLE_ENTITY_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Entity10 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_ENTITY_COMPLETE
                }
                _ => self.common(tok),
            },
            PrologHandler::Notation0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_NAME => {
                    self.handler = PrologHandler::Notation1;
                    XML_ROLE_NOTATION_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Notation1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"SYSTEM") {
                        self.handler = PrologHandler::Notation3;
                        return XML_ROLE_NOTATION_NONE;
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"PUBLIC") {
                        self.handler = PrologHandler::Notation2;
                        return XML_ROLE_NOTATION_NONE;
                    }
                    self.common(tok)
                }
                _ => self.common(tok),
            },
            PrologHandler::Notation2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Notation4;
                    XML_ROLE_NOTATION_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Notation3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_NOTATION_NONE;
                    XML_ROLE_NOTATION_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Notation4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_NOTATION_NONE;
                    XML_ROLE_NOTATION_SYSTEM_ID
                }
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_NOTATION_NO_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Attlist1;
                    XML_ROLE_ATTLIST_ELEMENT_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_ATTLIST_NONE
                }
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Attlist2;
                    XML_ROLE_ATTRIBUTE_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NAME => {
                    const TYPES: [&[u8]; 8] = [
                        b"CDATA",
                        b"ID",
                        b"IDREF",
                        b"IDREFS",
                        b"ENTITY",
                        b"ENTITIES",
                        b"NMTOKEN",
                        b"NMTOKENS",
                    ];
                    for (i, &t) in TYPES.iter().enumerate() {
                        if enc.name_matches_ascii(buf, ptr, end, t) {
                            self.handler = PrologHandler::Attlist8;
                            return XML_ROLE_ATTRIBUTE_TYPE_CDATA + (i as i32);
                        }
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"NOTATION") {
                        self.handler = PrologHandler::Attlist5;
                        return XML_ROLE_ATTLIST_NONE;
                    }
                    self.common(tok)
                }
                XML_TOK_OPEN_PAREN => {
                    self.handler = PrologHandler::Attlist3;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Attlist4;
                    XML_ROLE_ATTRIBUTE_ENUM_VALUE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.handler = PrologHandler::Attlist8;
                    XML_ROLE_ATTLIST_NONE
                }
                XML_TOK_OR => {
                    self.handler = PrologHandler::Attlist3;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_OPEN_PAREN => {
                    self.handler = PrologHandler::Attlist6;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist6 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NAME => {
                    self.handler = PrologHandler::Attlist7;
                    XML_ROLE_ATTRIBUTE_NOTATION_VALUE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist7 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.handler = PrologHandler::Attlist8;
                    XML_ROLE_ATTLIST_NONE
                }
                XML_TOK_OR => {
                    self.handler = PrologHandler::Attlist6;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist8 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_POUND_NAME => {
                    if enc.name_matches_ascii(buf, ptr + min, end, b"IMPLIED") {
                        self.handler = PrologHandler::Attlist1;
                        return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
                    }
                    if enc.name_matches_ascii(buf, ptr + min, end, b"REQUIRED") {
                        self.handler = PrologHandler::Attlist1;
                        return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
                    }
                    if enc.name_matches_ascii(buf, ptr + min, end, b"FIXED") {
                        self.handler = PrologHandler::Attlist9;
                        return XML_ROLE_ATTLIST_NONE;
                    }
                    self.common(tok)
                }
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Attlist1;
                    XML_ROLE_DEFAULT_ATTRIBUTE_VALUE
                }
                _ => self.common(tok),
            },
            PrologHandler::Attlist9 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_LITERAL => {
                    self.handler = PrologHandler::Attlist1;
                    XML_ROLE_FIXED_ATTRIBUTE_VALUE
                }
                _ => self.common(tok),
            },
            PrologHandler::Element0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Element1;
                    XML_ROLE_ELEMENT_NAME
                }
                _ => self.common(tok),
            },
            PrologHandler::Element1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_NAME => {
                    if enc.name_matches_ascii(buf, ptr, end, b"EMPTY") {
                        self.handler = PrologHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                        return XML_ROLE_CONTENT_EMPTY;
                    }
                    if enc.name_matches_ascii(buf, ptr, end, b"ANY") {
                        self.handler = PrologHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                        return XML_ROLE_CONTENT_ANY;
                    }
                    self.common(tok)
                }
                XML_TOK_OPEN_PAREN => {
                    self.handler = PrologHandler::Element2;
                    self.level = 1;
                    XML_ROLE_GROUP_OPEN
                }
                _ => self.common(tok),
            },
            PrologHandler::Element2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_POUND_NAME => {
                    if enc.name_matches_ascii(buf, ptr + min, end, b"PCDATA") {
                        self.handler = PrologHandler::Element3;
                        return XML_ROLE_CONTENT_PCDATA;
                    }
                    self.common(tok)
                }
                XML_TOK_OPEN_PAREN => {
                    self.level = 2;
                    self.handler = PrologHandler::Element6;
                    XML_ROLE_GROUP_OPEN
                }
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT
                }
                XML_TOK_NAME_QUESTION => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_OPT
                }
                XML_TOK_NAME_ASTERISK => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_REP
                }
                XML_TOK_NAME_PLUS => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_PLUS
                }
                _ => self.common(tok),
            },
            PrologHandler::Element3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_ELEMENT_NONE;
                    XML_ROLE_GROUP_CLOSE
                }
                XML_TOK_CLOSE_PAREN_ASTERISK => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_ELEMENT_NONE;
                    XML_ROLE_GROUP_CLOSE_REP
                }
                XML_TOK_OR => {
                    self.handler = PrologHandler::Element4;
                    XML_ROLE_ELEMENT_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::Element4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Element5;
                    XML_ROLE_CONTENT_ELEMENT
                }
                _ => self.common(tok),
            },
            PrologHandler::Element5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_CLOSE_PAREN_ASTERISK => {
                    self.handler = PrologHandler::DeclClose;
                    self.role_none = XML_ROLE_ELEMENT_NONE;
                    XML_ROLE_GROUP_CLOSE_REP
                }
                XML_TOK_OR => {
                    self.handler = PrologHandler::Element4;
                    XML_ROLE_ELEMENT_NONE
                }
                _ => self.common(tok),
            },
            PrologHandler::Element6 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_OPEN_PAREN => {
                    self.level += 1;
                    XML_ROLE_GROUP_OPEN
                }
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT
                }
                XML_TOK_NAME_QUESTION => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_OPT
                }
                XML_TOK_NAME_ASTERISK => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_REP
                }
                XML_TOK_NAME_PLUS => {
                    self.handler = PrologHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_PLUS
                }
                _ => self.common(tok),
            },
            PrologHandler::Element7 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = PrologHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE
                }
                XML_TOK_CLOSE_PAREN_ASTERISK => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = PrologHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE_REP
                }
                XML_TOK_CLOSE_PAREN_QUESTION => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = PrologHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE_OPT
                }
                XML_TOK_CLOSE_PAREN_PLUS => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = PrologHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE_PLUS
                }
                XML_TOK_COMMA => {
                    self.handler = PrologHandler::Element6;
                    XML_ROLE_GROUP_SEQUENCE
                }
                XML_TOK_OR => {
                    self.handler = PrologHandler::Element6;
                    XML_ROLE_GROUP_CHOICE
                }
                _ => self.common(tok),
            },
            PrologHandler::DeclClose => match tok {
                XML_TOK_PROLOG_S => self.role_none,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    self.role_none
                }
                _ => self.common(tok),
            },
            PrologHandler::Error => XML_ROLE_NONE,
        }
    }

    fn internal_subset(&mut self, tok: i32, buf: &[u8], ptr: usize, end: usize, enc: Encoding) -> i32 {
        let min = enc.min_bytes_per_char();
        match tok {
            XML_TOK_PROLOG_S => XML_ROLE_NONE,
            XML_TOK_DECL_OPEN => {
                if enc.name_matches_ascii(buf, ptr + 2 * min, end, b"ENTITY") {
                    self.handler = PrologHandler::Entity0;
                    return XML_ROLE_ENTITY_NONE;
                }
                if enc.name_matches_ascii(buf, ptr + 2 * min, end, b"ATTLIST") {
                    self.handler = PrologHandler::Attlist0;
                    return XML_ROLE_ATTLIST_NONE;
                }
                if enc.name_matches_ascii(buf, ptr + 2 * min, end, b"ELEMENT") {
                    self.handler = PrologHandler::Element0;
                    return XML_ROLE_ELEMENT_NONE;
                }
                if enc.name_matches_ascii(buf, ptr + 2 * min, end, b"NOTATION") {
                    self.handler = PrologHandler::Notation0;
                    return XML_ROLE_NOTATION_NONE;
                }
                self.common(tok)
            }
            XML_TOK_PI => XML_ROLE_PI,
            XML_TOK_COMMENT => XML_ROLE_COMMENT,
            XML_TOK_PARAM_ENTITY_REF => XML_ROLE_PARAM_ENTITY_REF,
            XML_TOK_CLOSE_BRACKET => {
                self.handler = PrologHandler::Doctype5;
                XML_ROLE_DOCTYPE_NONE
            }
            XML_TOK_NONE => XML_ROLE_NONE,
            _ => self.common(tok),
        }
    }

    fn external_subset1(&mut self, tok: i32, buf: &[u8], ptr: usize, end: usize, enc: Encoding) -> i32 {
        match tok {
            XML_TOK_COND_SECT_OPEN => {
                self.handler = PrologHandler::CondSect0;
                XML_ROLE_NONE
            }
            XML_TOK_COND_SECT_CLOSE => {
                if self.include_level == 0 {
                    self.common(tok)
                } else {
                    self.include_level -= 1;
                    XML_ROLE_NONE
                }
            }
            XML_TOK_PROLOG_S => XML_ROLE_NONE,
            XML_TOK_CLOSE_BRACKET => self.common(tok),
            XML_TOK_NONE => {
                if self.include_level != 0 {
                    self.common(tok)
                } else {
                    XML_ROLE_NONE
                }
            }
            _ => self.internal_subset(tok, buf, ptr, end, enc),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XmlAccount {
    Direct,
    EntityExpansion,
    None,
}

#[derive(Debug, Clone)]
struct Entity {
    name: Vec<u8>,
    text: Option<Vec<u8>>,
    processed: usize,
    system_id: Option<Vec<u8>>,
    base: Option<Vec<u8>>,
    public_id: Option<Vec<u8>>,
    notation: Option<Vec<u8>>,
    open: bool,
    has_more: bool,
    is_param: bool,
    is_internal: bool,
}

impl Entity {
    fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            text: None,
            processed: 0,
            system_id: None,
            base: None,
            public_id: None,
            notation: None,
            open: false,
            has_more: false,
            is_param: false,
            is_internal: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntityRefId {
    General(usize),
    Param(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntityType {
    Internal,
    Attribute,
    Value,
}

#[derive(Debug, Clone)]
struct OpenInternalEntity {
    internal_event_ptr: usize,
    internal_event_end_ptr: usize,
    entity: EntityRefId,
    start_tag_level: i32,
    between_decl: bool,
    entity_type: EntityType,
}

#[derive(Debug, Clone)]
struct AttributeId {
    name: Vec<u8>,
    maybe_tokenized: bool,
    mark: u8,
}

#[derive(Debug, Clone)]
struct DefaultAttribute {
    id: usize,
    is_cdata: bool,
    value: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
struct ElementType {
    name: Vec<u8>,
    id_att: Option<usize>,
    default_atts: Vec<DefaultAttribute>,
    default_att_for_name: HashMap<Vec<u8>, usize>,
}

struct Dtd {
    general_entities: Vec<Entity>,
    general_entity_map: HashMap<Vec<u8>, usize>,
    param_entities: Vec<Entity>,
    param_entity_map: HashMap<Vec<u8>, usize>,
    element_types: Vec<ElementType>,
    element_type_map: HashMap<Vec<u8>, usize>,
    attribute_ids: Vec<AttributeId>,
    attribute_id_map: HashMap<Vec<u8>, usize>,
    keep_processing: bool,
    has_param_entity_refs: bool,
    standalone: bool,
}

impl Dtd {
    fn new() -> Self {
        Self {
            general_entities: Vec::new(),
            general_entity_map: HashMap::new(),
            param_entities: Vec::new(),
            param_entity_map: HashMap::new(),
            element_types: Vec::new(),
            element_type_map: HashMap::new(),
            attribute_ids: Vec::new(),
            attribute_id_map: HashMap::new(),
            keep_processing: true,
            has_param_entity_refs: false,
            standalone: false,
        }
    }

    fn lookup_general_entity(&self, name: &[u8]) -> Option<usize> {
        self.general_entity_map.get(name).copied()
    }

    fn insert_general_entity(&mut self, name: Vec<u8>) -> (usize, bool) {
        if let Some(&idx) = self.general_entity_map.get(&name) {
            (idx, false)
        } else {
            let idx = self.general_entities.len();
            self.general_entities.push(Entity::new(name.clone()));
            self.general_entity_map.insert(name, idx);
            (idx, true)
        }
    }

    fn insert_param_entity(&mut self, name: Vec<u8>) -> (usize, bool) {
        if let Some(&idx) = self.param_entity_map.get(&name) {
            (idx, false)
        } else {
            let idx = self.param_entities.len();
            self.param_entities.push(Entity::new(name.clone()));
            self.param_entity_map.insert(name, idx);
            (idx, true)
        }
    }

    fn get_or_create_element_type(&mut self, name: &[u8]) -> usize {
        if let Some(&idx) = self.element_type_map.get(name) {
            idx
        } else {
            let idx = self.element_types.len();
            let owned = name.to_vec();
            self.element_types.push(ElementType {
                name: owned.clone(),
                id_att: None,
                default_atts: Vec::new(),
                default_att_for_name: HashMap::new(),
            });
            self.element_type_map.insert(owned, idx);
            idx
        }
    }

    fn get_or_create_attribute_id(&mut self, name: &[u8]) -> usize {
        if let Some(&idx) = self.attribute_id_map.get(name) {
            idx
        } else {
            let idx = self.attribute_ids.len();
            let owned = name.to_vec();
            self.attribute_ids.push(AttributeId {
                name: owned.clone(),
                maybe_tokenized: false,
                mark: 0,
            });
            self.attribute_id_map.insert(owned, idx);
            idx
        }
    }

    fn entity(&self, id: EntityRefId) -> &Entity {
        match id {
            EntityRefId::General(idx) => &self.general_entities[idx],
            EntityRefId::Param(idx) => &self.param_entities[idx],
        }
    }

    fn entity_mut(&mut self, id: EntityRefId) -> &mut Entity {
        match id {
            EntityRefId::General(idx) => &mut self.general_entities[idx],
            EntityRefId::Param(idx) => &mut self.param_entities[idx],
        }
    }
}

struct Tag {
    raw_name: Vec<u8>,
    name_str: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessorKind {
    PrologInit,
    Prolog,
    Content,
    CdataSection,
    IgnoreSection,
    Epilog,
    InternalEntity,
    Error,
}

struct Accounting {
    count_bytes_direct: u64,
    count_bytes_indirect: u64,
    maximum_amplification_factor: f32,
    activation_threshold_bytes: u64,
}

impl Accounting {
    fn new() -> Self {
        Self {
            count_bytes_direct: 0,
            count_bytes_indirect: 0,
            maximum_amplification_factor: 100.0f32,
            activation_threshold_bytes: 8_388_608,
        }
    }

    fn current_amplification(&self) -> f32 {
        const LEN_OF_SHORTEST_INCLUDE: u64 = 22; // sizeof("<!ENTITY a SYSTEM 'b'>") - 1
        let count_bytes_output = self
            .count_bytes_direct
            .wrapping_add(self.count_bytes_indirect);
        if self.count_bytes_direct != 0 {
            (count_bytes_output as f32) / (self.count_bytes_direct as f32)
        } else {
            ((LEN_OF_SHORTEST_INCLUDE + self.count_bytes_indirect) as f32)
                / (LEN_OF_SHORTEST_INCLUDE as f32)
        }
    }

    fn diff_tolerated(&mut self, tok: i32, before: usize, after: usize, account: XmlAccount) -> bool {
        match tok {
            XML_TOK_INVALID | XML_TOK_PARTIAL | XML_TOK_PARTIAL_CHAR | XML_TOK_NONE => return true,
            _ => {}
        }
        if account == XmlAccount::None {
            return true;
        }
        let is_direct = account == XmlAccount::Direct;
        let bytes_more = after.saturating_sub(before) as u64;
        let target = if is_direct {
            &mut self.count_bytes_direct
        } else {
            &mut self.count_bytes_indirect
        };
        if *target > u64::MAX - bytes_more {
            return false;
        }
        *target += bytes_more;

        let count_bytes_output = self
            .count_bytes_direct
            .wrapping_add(self.count_bytes_indirect);
        let amplification_factor = self.current_amplification();
        (count_bytes_output < self.activation_threshold_bytes)
            || (amplification_factor <= self.maximum_amplification_factor)
    }
}

struct Parser<'a> {
    input: &'a [u8],
    events: Vec<Event>,
    encoding: Encoding,
    internal_encoding: Encoding,
    prolog_state: PrologState,
    processor: ProcessorKind,
    error_code: u32,
    event_ptr: Option<usize>,
    event_end_ptr: Option<usize>,
    position_ptr: usize,
    position: Position,
    open_internal_entities: Vec<OpenInternalEntity>,
    open_attribute_entities: Vec<OpenInternalEntity>,
    open_value_entities: Vec<OpenInternalEntity>,
    tag_level: i32,
    tag_stack: Vec<Tag>,
    decl_entity: Option<EntityRefId>,
    doctype_name: Option<Vec<u8>>,
    doctype_sysid: Option<Vec<u8>>,
    doctype_pubid: Option<Vec<u8>>,
    decl_element_type: Option<usize>,
    decl_attribute_id: Option<usize>,
    decl_attribute_is_cdata: bool,
    decl_attribute_is_id: bool,
    dtd: Dtd,
    atts: Vec<RawAttribute>,
    group_connector: Vec<u8>,
    accounting: Accounting,
    reenter: bool,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            events: Vec::new(),
            encoding: Encoding::Init(InitEncIndex::NoEnc),
            internal_encoding: Encoding::InternalUtf8,
            prolog_state: PrologState::new(),
            processor: ProcessorKind::PrologInit,
            error_code: XML_ERROR_NONE,
            event_ptr: None,
            event_end_ptr: None,
            position_ptr: 0,
            position: Position::default(),
            open_internal_entities: Vec::new(),
            open_attribute_entities: Vec::new(),
            open_value_entities: Vec::new(),
            tag_level: 0,
            tag_stack: Vec::new(),
            decl_entity: None,
            doctype_name: None,
            doctype_sysid: None,
            doctype_pubid: None,
            decl_element_type: None,
            decl_attribute_id: None,
            decl_attribute_is_cdata: false,
            decl_attribute_is_id: false,
            dtd: Dtd::new(),
            atts: Vec::with_capacity(16),
            group_connector: Vec::new(),
            accounting: Accounting::new(),
            reenter: false,
        }
    }

    fn set_event_ptr(&mut self, is_doc_enc: bool, ptr: usize) {
        if is_doc_enc {
            self.event_ptr = Some(ptr);
        } else if let Some(top) = self.open_internal_entities.last_mut() {
            top.internal_event_ptr = ptr;
        }
    }

    fn set_event_end_ptr(&mut self, is_doc_enc: bool, ptr: usize) {
        if is_doc_enc {
            self.event_end_ptr = Some(ptr);
        } else if let Some(top) = self.open_internal_entities.last_mut() {
            top.internal_event_end_ptr = ptr;
        }
    }

    fn get_event_end_ptr(&self, is_doc_enc: bool) -> usize {
        if is_doc_enc {
            self.event_end_ptr.unwrap_or(0)
        } else if let Some(top) = self.open_internal_entities.last() {
            top.internal_event_end_ptr
        } else {
            0
        }
    }

    fn current_line_and_col(&mut self) -> (u64, u64) {
        if let Some(ev_ptr) = self.event_ptr {
            if ev_ptr >= self.position_ptr {
                self.encoding
                    .update_position(self.input, self.position_ptr, ev_ptr, &mut self.position);
                self.position_ptr = ev_ptr;
            }
        }
        (self.position.line_number + 1, self.position.column_number)
    }

    fn run(&mut self) -> u32 {
        let end = self.input.len();
        let mut end_ptr = 0usize;
        loop {
            let ret = match self.processor {
                ProcessorKind::PrologInit => {
                    self.processor = ProcessorKind::Prolog;
                    self.prolog_processor(end_ptr, end, &mut end_ptr)
                }
                ProcessorKind::Prolog => self.prolog_processor(end_ptr, end, &mut end_ptr),
                ProcessorKind::Content => self.content_processor(end_ptr, end, &mut end_ptr),
                ProcessorKind::CdataSection => {
                    self.cdata_section_processor(end_ptr, end, &mut end_ptr)
                }
                ProcessorKind::IgnoreSection => {
                    self.ignore_section_processor(end_ptr, end, &mut end_ptr)
                }
                ProcessorKind::Epilog => self.epilog_processor(end_ptr, end, &mut end_ptr),
                ProcessorKind::InternalEntity => self.internal_entity_processor(),
                ProcessorKind::Error => self.error_code,
            };

            if !self.reenter {
                return ret;
            }
            self.reenter = false;
            if ret != XML_ERROR_NONE {
                return ret;
            }
        }
    }

    fn prolog_processor(&mut self, s: usize, end: usize, next_ptr: &mut usize) -> u32 {
        let mut next = s;
        let mut enc_out = self.encoding;
        let tok = xml_tok(
            self.encoding,
            XML_PROLOG_STATE,
            self.input,
            s,
            end,
            &mut next,
            &mut enc_out,
        );
        self.encoding = enc_out;
        self.do_prolog(
            self.encoding,
            true,
            self.input,
            s,
            end,
            tok,
            next,
            next_ptr,
            false,
            true,
            XmlAccount::Direct,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn do_prolog(
        &mut self,
        mut enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        mut s: usize,
        end: usize,
        mut tok: i32,
        mut next: usize,
        next_ptr: &mut usize,
        have_more: bool,
        allow_closing_doctype: bool,
        account: XmlAccount,
    ) -> u32 {
        loop {
            self.set_event_ptr(is_doc_enc, s);
            self.set_event_end_ptr(is_doc_enc, next);

            if tok <= 0 {
                if have_more && tok != XML_TOK_INVALID {
                    *next_ptr = s;
                    return XML_ERROR_NONE;
                }
                match tok {
                    XML_TOK_INVALID => {
                        self.set_event_ptr(is_doc_enc, next);
                        return XML_ERROR_INVALID_TOKEN;
                    }
                    XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                    XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                    t if t == -XML_TOK_PROLOG_S => {
                        tok = -tok;
                    }
                    XML_TOK_NONE => {
                        if !is_doc_enc {
                            let between_decl = self
                                .open_internal_entities
                                .last()
                                .map(|e| e.between_decl)
                                .unwrap_or(false);
                            if !between_decl {
                                *next_ptr = s;
                                return XML_ERROR_NONE;
                            }
                            if self
                                .prolog_state
                                .token_role(XML_TOK_NONE, buf, end, end, enc)
                                == XML_ROLE_ERROR
                            {
                                return XML_ERROR_INCOMPLETE_PE;
                            }
                            *next_ptr = s;
                            return XML_ERROR_NONE;
                        }
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    _ => {
                        tok = -tok;
                        next = end;
                    }
                }
            }

            let role = self.prolog_state.token_role(tok, buf, s, next, enc);
            match role {
                XML_ROLE_INSTANCE_START | XML_ROLE_XML_DECL | XML_ROLE_TEXT_DECL => {}
                _ => {
                    if !self.accounting.diff_tolerated(tok, s, next, account) {
                        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                    }
                }
            }

            match role {
                XML_ROLE_XML_DECL => {
                    let res = self.process_xml_decl(false, buf, s, next);
                    if res != XML_ERROR_NONE {
                        return res;
                    }
                    enc = self.encoding;
                }
                XML_ROLE_TEXT_DECL => {
                    let res = self.process_xml_decl(true, buf, s, next);
                    if res != XML_ERROR_NONE {
                        return res;
                    }
                    enc = self.encoding;
                }
                XML_ROLE_DOCTYPE_NAME => {
                    self.doctype_name = Some(enc.store_string(buf, s, next));
                    self.doctype_pubid = None;
                    self.doctype_sysid = None;
                }
                XML_ROLE_DOCTYPE_INTERNAL_SUBSET => {
                    if let Some(name) = self.doctype_name.take() {
                        let sysid = self.doctype_sysid.take();
                        let pubid = self.doctype_pubid.take();
                        self.events.push(Event::DoctypeStart {
                            name,
                            sysid,
                            pubid,
                            has_internal_subset: true,
                        });
                    }
                }
                XML_ROLE_DOCTYPE_PUBLIC_ID => {
                    let (idx, _) = self.dtd.insert_param_entity(b"#".to_vec());
                    self.decl_entity = Some(EntityRefId::Param(idx));
                    self.dtd.has_param_entity_refs = true;

                    let mut bad_ptr = s;
                    if !enc.is_public_id(buf, s, next, &mut bad_ptr) {
                        self.set_event_ptr(is_doc_enc, bad_ptr);
                        return XML_ERROR_PUBLICID;
                    }
                    let min = enc.min_bytes_per_char();
                    let mut pub_id = enc.store_string(buf, s + min, next - min);
                    normalize_public_id(&mut pub_id);
                    self.doctype_pubid = Some(pub_id.clone());

                    if self.dtd.keep_processing {
                        if let Some(ent_id) = self.decl_entity {
                            self.dtd.entity_mut(ent_id).public_id = Some(pub_id);
                        }
                    }
                }
                XML_ROLE_ENTITY_PUBLIC_ID => {
                    let mut bad_ptr = s;
                    if !enc.is_public_id(buf, s, next, &mut bad_ptr) {
                        self.set_event_ptr(is_doc_enc, bad_ptr);
                        return XML_ERROR_PUBLICID;
                    }
                    if self.dtd.keep_processing {
                        if let Some(ent_id) = self.decl_entity {
                            let min = enc.min_bytes_per_char();
                            let mut pub_id = enc.store_string(buf, s + min, next - min);
                            normalize_public_id(&mut pub_id);
                            self.dtd.entity_mut(ent_id).public_id = Some(pub_id);
                        }
                    }
                }
                XML_ROLE_DOCTYPE_CLOSE => {
                    if !allow_closing_doctype {
                        return XML_ERROR_INVALID_TOKEN;
                    }
                    let had_sysid = self.doctype_sysid.is_some();
                    if let Some(name) = self.doctype_name.take() {
                        let sysid = self.doctype_sysid.take();
                        let pubid = self.doctype_pubid.take();
                        self.events.push(Event::DoctypeStart {
                            name,
                            sysid,
                            pubid,
                            has_internal_subset: false,
                        });
                    }
                    if had_sysid {
                        self.dtd.has_param_entity_refs = true;
                    }
                    self.events.push(Event::DoctypeEnd);
                }
                XML_ROLE_INSTANCE_START => {
                    self.processor = ProcessorKind::Content;
                    return self.content_processor(s, end, next_ptr);
                }
                XML_ROLE_ATTLIST_ELEMENT_NAME => {
                    let name = enc.store_string(buf, s, next);
                    let idx = self.dtd.get_or_create_element_type(&name);
                    self.decl_element_type = Some(idx);
                }
                XML_ROLE_ATTRIBUTE_NAME => {
                    let name = enc.store_string(buf, s, next);
                    let idx = self.dtd.get_or_create_attribute_id(&name);
                    self.decl_attribute_id = Some(idx);
                    self.decl_attribute_is_cdata = false;
                    self.decl_attribute_is_id = false;
                }
                XML_ROLE_ATTRIBUTE_TYPE_CDATA => {
                    self.decl_attribute_is_cdata = true;
                }
                XML_ROLE_ATTRIBUTE_TYPE_ID => {
                    self.decl_attribute_is_id = true;
                }
                XML_ROLE_IMPLIED_ATTRIBUTE_VALUE | XML_ROLE_REQUIRED_ATTRIBUTE_VALUE => {
                    if self.dtd.keep_processing {
                        if let (Some(el_idx), Some(att_idx)) =
                            (self.decl_element_type, self.decl_attribute_id)
                        {
                            self.define_attribute(
                                el_idx,
                                att_idx,
                                self.decl_attribute_is_cdata,
                                self.decl_attribute_is_id,
                                None,
                            );
                        }
                    }
                }
                XML_ROLE_DEFAULT_ATTRIBUTE_VALUE | XML_ROLE_FIXED_ATTRIBUTE_VALUE => {
                    if self.dtd.keep_processing {
                        let min = enc.min_bytes_per_char();
                        let mut att_val = Vec::new();
                        let res = self.store_attribute_value(
                            enc,
                            is_doc_enc,
                            buf,
                            self.decl_attribute_is_cdata,
                            s + min,
                            next - min,
                            &mut att_val,
                            XmlAccount::None,
                            true,
                        );
                        if res != XML_ERROR_NONE {
                            return res;
                        }
                        if let (Some(el_idx), Some(att_idx)) =
                            (self.decl_element_type, self.decl_attribute_id)
                        {
                            self.define_attribute(
                                el_idx,
                                att_idx,
                                self.decl_attribute_is_cdata,
                                false,
                                Some(att_val),
                            );
                        }
                    }
                }
                XML_ROLE_ENTITY_VALUE => {
                    if self.dtd.keep_processing {
                        let min = enc.min_bytes_per_char();
                        let mut entity_val = Vec::new();
                        let res = self.call_store_entity_value(
                            enc,
                            is_doc_enc,
                            buf,
                            s + min,
                            next - min,
                            XmlAccount::None,
                            &mut entity_val,
                        );
                        if let Some(ent_id) = self.decl_entity {
                            let ent = self.dtd.entity_mut(ent_id);
                            ent.text = Some(entity_val.clone());
                            let name = ent.name.clone();
                            let is_pe = ent.is_param;
                            self.set_event_end_ptr(is_doc_enc, s);
                            self.events.push(Event::EntityDecl {
                                name,
                                is_pe,
                                value: Some(entity_val),
                                sysid: None,
                                pubid: None,
                                notation: None,
                            });
                        }
                        if res != XML_ERROR_NONE {
                            return res;
                        }
                    }
                }
                XML_ROLE_DOCTYPE_SYSTEM_ID => {
                    self.dtd.has_param_entity_refs = true;
                    let min = enc.min_bytes_per_char();
                    let sysid = enc.store_string(buf, s + min, next - min);
                    self.doctype_sysid = Some(sysid.clone());
                    if self.decl_entity.is_none() {
                        let (idx, _) = self.dtd.insert_param_entity(b"#".to_vec());
                        self.dtd.param_entities[idx].public_id = None;
                        self.decl_entity = Some(EntityRefId::Param(idx));
                    }
                    if self.dtd.keep_processing {
                        if let Some(ent_id) = self.decl_entity {
                            self.dtd.entity_mut(ent_id).system_id = Some(sysid);
                        }
                    }
                }
                XML_ROLE_ENTITY_SYSTEM_ID => {
                    if self.dtd.keep_processing {
                        if let Some(ent_id) = self.decl_entity {
                            let min = enc.min_bytes_per_char();
                            let sysid = enc.store_string(buf, s + min, next - min);
                            self.dtd.entity_mut(ent_id).system_id = Some(sysid);
                        }
                    }
                }
                XML_ROLE_ENTITY_COMPLETE => {
                    if self.dtd.keep_processing {
                        if let Some(ent_id) = self.decl_entity {
                            let ent = self.dtd.entity(ent_id);
                            let name = ent.name.clone();
                            let is_pe = ent.is_param;
                            let sysid = ent.system_id.clone();
                            let pubid = ent.public_id.clone();
                            self.set_event_end_ptr(is_doc_enc, s);
                            self.events.push(Event::EntityDecl {
                                name,
                                is_pe,
                                value: None,
                                sysid,
                                pubid,
                                notation: None,
                            });
                        }
                    }
                }
                XML_ROLE_ENTITY_NOTATION_NAME => {
                    if self.dtd.keep_processing {
                        if let Some(ent_id) = self.decl_entity {
                            let notation = enc.store_string(buf, s, next);
                            let ent = self.dtd.entity_mut(ent_id);
                            ent.notation = Some(notation.clone());
                            let name = ent.name.clone();
                            let sysid = ent.system_id.clone();
                            let pubid = ent.public_id.clone();
                            self.set_event_end_ptr(is_doc_enc, s);
                            self.events.push(Event::EntityDecl {
                                name,
                                is_pe: false,
                                value: None,
                                sysid,
                                pubid,
                                notation: Some(notation),
                            });
                        }
                    }
                }
                XML_ROLE_GENERAL_ENTITY_NAME => {
                    if enc.predefined_entity_name(buf, s, next) != 0 {
                        self.decl_entity = None;
                    } else if self.dtd.keep_processing {
                        let name = enc.store_string(buf, s, next);
                        let (idx, created) = self.dtd.insert_general_entity(name);
                        if !created {
                            self.decl_entity = None;
                        } else {
                            let is_internal = self.open_internal_entities.is_empty();
                            let ent = &mut self.dtd.general_entities[idx];
                            ent.public_id = None;
                            ent.is_param = false;
                            ent.is_internal = is_internal;
                            self.decl_entity = Some(EntityRefId::General(idx));
                        }
                    } else {
                        self.decl_entity = None;
                    }
                }
                XML_ROLE_PARAM_ENTITY_NAME => {
                    if self.dtd.keep_processing {
                        let name = enc.store_string(buf, s, next);
                        let (idx, created) = self.dtd.insert_param_entity(name);
                        if !created {
                            self.decl_entity = None;
                        } else {
                            let is_internal = self.open_internal_entities.is_empty();
                            let ent = &mut self.dtd.param_entities[idx];
                            ent.public_id = None;
                            ent.is_param = true;
                            ent.is_internal = is_internal;
                            self.decl_entity = Some(EntityRefId::Param(idx));
                        }
                    } else {
                        self.decl_entity = None;
                    }
                }
                XML_ROLE_NOTATION_PUBLIC_ID => {
                    let mut bad_ptr = s;
                    if !enc.is_public_id(buf, s, next, &mut bad_ptr) {
                        self.set_event_ptr(is_doc_enc, bad_ptr);
                        return XML_ERROR_PUBLICID;
                    }
                }
                XML_ROLE_ERROR => match tok {
                    XML_TOK_PARAM_ENTITY_REF => return XML_ERROR_PARAM_ENTITY_REF,
                    XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
                    _ => return XML_ERROR_SYNTAX,
                },
                XML_ROLE_IGNORE_SECT => {
                    let mut next_opt = Some(next);
                    let res = self.do_ignore_section(
                        enc,
                        is_doc_enc,
                        buf,
                        &mut next_opt,
                        end,
                        next_ptr,
                        have_more,
                    );
                    if res != XML_ERROR_NONE {
                        return res;
                    }
                    match next_opt {
                        Some(n) => next = n,
                        None => {
                            self.processor = ProcessorKind::IgnoreSection;
                            return res;
                        }
                    }
                }
                XML_ROLE_GROUP_OPEN => {
                    let lvl = self.prolog_state.level as usize;
                    if lvl >= self.group_connector.len() {
                        self.group_connector.resize(lvl + 32, 0);
                    }
                    self.group_connector[lvl] = 0;
                }
                XML_ROLE_GROUP_SEQUENCE => {
                    let lvl = self.prolog_state.level as usize;
                    if self.group_connector[lvl] == b'|' {
                        return XML_ERROR_SYNTAX;
                    }
                    self.group_connector[lvl] = b',';
                }
                XML_ROLE_GROUP_CHOICE => {
                    let lvl = self.prolog_state.level as usize;
                    if self.group_connector[lvl] == b',' {
                        return XML_ERROR_SYNTAX;
                    }
                    self.group_connector[lvl] = b'|';
                }
                XML_ROLE_PARAM_ENTITY_REF | XML_ROLE_INNER_PARAM_ENTITY_REF => {
                    self.dtd.has_param_entity_refs = true;
                    // m_paramEntityParsing is XML_PARAM_ENTITY_PARSING_NEVER
                    self.dtd.keep_processing = self.dtd.standalone;
                }
                XML_ROLE_PI => {
                    self.report_processing_instruction(enc, buf, s, next);
                }
                XML_ROLE_COMMENT => {
                    self.report_comment(enc, buf, s, next);
                }
                _ => {}
            }

            if self.reenter {
                *next_ptr = next;
                return XML_ERROR_NONE;
            }
            s = next;
            if is_doc_enc {
                let mut enc_out = self.encoding;
                tok = xml_tok(
                    enc,
                    XML_PROLOG_STATE,
                    buf,
                    s,
                    end,
                    &mut next,
                    &mut enc_out,
                );
                self.encoding = enc_out;
                enc = enc_out;
            } else {
                tok = prolog_tok_normal(enc, buf, s, end, &mut next);
            }
        }
    }

    fn process_xml_decl(&mut self, is_general_text_entity: bool, buf: &[u8], s: usize, next: usize) -> u32 {
        if !self
            .accounting
            .diff_tolerated(XML_TOK_XML_DECL, s, next, XmlAccount::Direct)
        {
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }

        let mut bad_ptr = self.event_ptr;
        let mut version_range: Option<(usize, usize)> = None;
        let mut encoding_name: Option<usize> = None;
        let mut new_encoding: Option<Encoding> = None;
        let mut standalone: i32 = -1;

        if !xml_parse_xml_decl(
            is_general_text_entity,
            self.encoding,
            buf,
            s,
            next,
            &mut bad_ptr,
            &mut version_range,
            &mut encoding_name,
            &mut new_encoding,
            &mut standalone,
        ) {
            self.event_ptr = bad_ptr;
            return if is_general_text_entity {
                XML_ERROR_TEXT_DECL
            } else {
                XML_ERROR_XML_DECL
            };
        }

        if !is_general_text_entity && standalone == 1 {
            self.dtd.standalone = true;
        }

        let stored_enc_name = encoding_name.map(|enc_pos| {
            let len = self.encoding.name_length(buf, enc_pos);
            self.encoding.store_string(buf, enc_pos, enc_pos + len)
        });
        let stored_version = version_range.map(|(v_start, v_end)| {
            let min = self.encoding.min_bytes_per_char();
            self.encoding.store_string(buf, v_start, v_end - min)
        });

        self.events.push(Event::XmlDecl {
            version: stored_version,
            encoding: stored_enc_name,
            standalone,
        });

        if let Some(new_enc) = new_encoding {
            if new_enc.min_bytes_per_char() != self.encoding.min_bytes_per_char()
                || (new_enc.min_bytes_per_char() == 2 && new_enc != self.encoding)
            {
                self.event_ptr = encoding_name;
                return XML_ERROR_INCORRECT_ENCODING;
            }
            self.encoding = new_enc;
        } else if let Some(enc_pos) = encoding_name {
            self.event_ptr = Some(enc_pos);
            return XML_ERROR_UNKNOWN_ENCODING;
        }

        XML_ERROR_NONE
    }

    fn content_processor(&mut self, start: usize, end: usize, end_ptr: &mut usize) -> u32 {
        self.do_content(
            0,
            self.encoding,
            true,
            self.input,
            start,
            end,
            end_ptr,
            false,
            XmlAccount::Direct,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn do_content(
        &mut self,
        start_tag_level: i32,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        mut s: usize,
        end: usize,
        next_ptr: &mut usize,
        have_more: bool,
        account: XmlAccount,
    ) -> u32 {
        self.set_event_ptr(is_doc_enc, s);
        let min = enc.min_bytes_per_char();

        loop {
            let mut next = s;
            let tok = content_tok_normal(enc, buf, s, end, &mut next);
            let account_after =
                if tok == XML_TOK_TRAILING_RSQB || tok == XML_TOK_TRAILING_CR {
                    if have_more {
                        s
                    } else {
                        end
                    }
                } else {
                    next
                };
            if !self.accounting.diff_tolerated(tok, s, account_after, account) {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            self.set_event_end_ptr(is_doc_enc, next);

            match tok {
                XML_TOK_TRAILING_CR => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    self.set_event_end_ptr(is_doc_enc, end);
                    self.events.push(Event::Text(vec![b'\n']));
                    if start_tag_level == 0 {
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    if self.tag_level != start_tag_level {
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *next_ptr = end;
                    return XML_ERROR_NONE;
                }
                XML_TOK_NONE => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    if start_tag_level > 0 {
                        if self.tag_level != start_tag_level {
                            return XML_ERROR_ASYNC_ENTITY;
                        }
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_NO_ELEMENTS;
                }
                XML_TOK_INVALID => {
                    self.set_event_ptr(is_doc_enc, next);
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_UNCLOSED_TOKEN;
                }
                XML_TOK_PARTIAL_CHAR => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_PARTIAL_CHAR;
                }
                XML_TOK_ENTITY_REF => {
                    let ch = enc.predefined_entity_name(buf, s + min, next - min);
                    if ch != 0 {
                        self.accounting
                            .diff_tolerated(tok, 0, 1, XmlAccount::EntityExpansion);
                        self.events.push(Event::Text(vec![ch]));
                    } else {
                        let name = enc.store_string(buf, s + min, next - min);
                        let ent_idx = self.dtd.lookup_general_entity(&name);
                        if !self.dtd.has_param_entity_refs || self.dtd.standalone {
                            match ent_idx {
                                None => return XML_ERROR_UNDEFINED_ENTITY,
                                Some(idx) => {
                                    if !self.dtd.general_entities[idx].is_internal {
                                        return XML_ERROR_ENTITY_DECLARED_IN_PE;
                                    }
                                }
                            }
                        } else if ent_idx.is_none() {
                            self.events.push(Event::Skipped {
                                name,
                                is_pe: false,
                            });
                            self.set_event_ptr(is_doc_enc, next);
                            s = next;
                            continue;
                        }
                        if let Some(idx) = ent_idx {
                            let ent = &self.dtd.general_entities[idx];
                            if ent.open {
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if ent.notation.is_some() {
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if ent.text.is_some() {
                                let res = self.process_entity(
                                    EntityRefId::General(idx),
                                    false,
                                    EntityType::Internal,
                                );
                                if res != XML_ERROR_NONE {
                                    return res;
                                }
                            }
                        }
                    }
                }
                XML_TOK_START_TAG_NO_ATTS | XML_TOK_START_TAG_WITH_ATTS => {
                    let raw_name_pos = s + min;
                    let raw_name_len = enc.name_length(buf, raw_name_pos);
                    let raw_name = buf[raw_name_pos..raw_name_pos + raw_name_len].to_vec();
                    let name_str =
                        enc.store_string(buf, raw_name_pos, raw_name_pos + raw_name_len);
                    self.tag_level += 1;
                    self.tag_stack.push(Tag {
                        raw_name,
                        name_str: name_str.clone(),
                    });
                    let mut attrs = Vec::new();
                    let res = self.store_atts(enc, is_doc_enc, buf, s, &name_str, &mut attrs, account);
                    if res != XML_ERROR_NONE {
                        return res;
                    }
                    self.events.push(Event::Start {
                        name: name_str,
                        attrs,
                    });
                }
                XML_TOK_EMPTY_ELEMENT_NO_ATTS | XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                    let raw_name_pos = s + min;
                    let raw_name_len = enc.name_length(buf, raw_name_pos);
                    let name_str =
                        enc.store_string(buf, raw_name_pos, raw_name_pos + raw_name_len);
                    let mut attrs = Vec::new();
                    let res = self.store_atts(
                        enc,
                        is_doc_enc,
                        buf,
                        s,
                        &name_str,
                        &mut attrs,
                        XmlAccount::None,
                    );
                    if res != XML_ERROR_NONE {
                        return res;
                    }
                    self.events.push(Event::Start {
                        name: name_str.clone(),
                        attrs,
                    });
                    let end_ev = self.get_event_end_ptr(is_doc_enc);
                    self.set_event_ptr(is_doc_enc, end_ev);
                    self.events.push(Event::End { name: name_str });
                    if self.tag_level == 0 {
                        if self.reenter {
                            self.processor = ProcessorKind::Epilog;
                        } else {
                            return self.epilog_processor(next, end, next_ptr);
                        }
                    }
                }
                XML_TOK_END_TAG => {
                    if self.tag_level == start_tag_level {
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    let raw_name_pos = s + min * 2;
                    let len = enc.name_length(buf, raw_name_pos);
                    let Some(tag) = self.tag_stack.last() else {
                        return XML_ERROR_ASYNC_ENTITY;
                    };
                    if len != tag.raw_name.len()
                        || buf[raw_name_pos..raw_name_pos + len] != tag.raw_name[..]
                    {
                        self.set_event_ptr(is_doc_enc, raw_name_pos);
                        return XML_ERROR_TAG_MISMATCH;
                    }
                    let tag = self.tag_stack.pop().unwrap();
                    self.tag_level -= 1;
                    self.events.push(Event::End { name: tag.name_str });
                    if self.tag_level == 0 {
                        if self.reenter {
                            self.processor = ProcessorKind::Epilog;
                        } else {
                            return self.epilog_processor(next, end, next_ptr);
                        }
                    }
                }
                XML_TOK_CHAR_REF => {
                    let n = enc.char_ref_number(buf, s);
                    if n < 0 {
                        return XML_ERROR_BAD_CHAR_REF;
                    }
                    let mut utf8_buf = [0u8; 4];
                    let encoded_len = xml_utf8_encode(n, &mut utf8_buf);
                    self.events
                        .push(Event::Text(utf8_buf[..encoded_len].to_vec()));
                }
                XML_TOK_XML_DECL => {
                    return XML_ERROR_MISPLACED_XML_PI;
                }
                XML_TOK_DATA_NEWLINE => {
                    self.events.push(Event::Text(vec![b'\n']));
                }
                XML_TOK_CDATA_SECT_OPEN => {
                    self.events.push(Event::CdataStart);
                    let mut next_opt = Some(next);
                    let res = self.do_cdata_section(
                        enc,
                        is_doc_enc,
                        buf,
                        &mut next_opt,
                        end,
                        next_ptr,
                        have_more,
                        account,
                    );
                    if res != XML_ERROR_NONE {
                        return res;
                    }
                    match next_opt {
                        Some(n) => next = n,
                        None => {
                            self.processor = ProcessorKind::CdataSection;
                            return res;
                        }
                    }
                }
                XML_TOK_TRAILING_RSQB => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    let mut text = Vec::new();
                    enc.convert_to_utf8(buf, s, end, &mut text);
                    self.events.push(Event::Text(text));
                    if start_tag_level == 0 {
                        self.set_event_ptr(is_doc_enc, end);
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    if self.tag_level != start_tag_level {
                        self.set_event_ptr(is_doc_enc, end);
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *next_ptr = end;
                    return XML_ERROR_NONE;
                }
                XML_TOK_DATA_CHARS => {
                    let mut text = Vec::new();
                    enc.convert_to_utf8(buf, s, next, &mut text);
                    self.events.push(Event::Text(text));
                }
                XML_TOK_PI => {
                    self.report_processing_instruction(enc, buf, s, next);
                }
                XML_TOK_COMMENT => {
                    self.report_comment(enc, buf, s, next);
                }
                _ => {}
            }

            if self.reenter {
                *next_ptr = next;
                return XML_ERROR_NONE;
            }
            self.set_event_ptr(is_doc_enc, next);
            s = next;
        }
    }

    fn cdata_section_processor(&mut self, start: usize, end: usize, end_ptr: &mut usize) -> u32 {
        let mut start_opt = Some(start);
        let res = self.do_cdata_section(
            self.encoding,
            true,
            self.input,
            &mut start_opt,
            end,
            end_ptr,
            false,
            XmlAccount::Direct,
        );
        if res != XML_ERROR_NONE {
            return res;
        }
        if let Some(s) = start_opt {
            self.processor = ProcessorKind::Content;
            return self.content_processor(s, end, end_ptr);
        }
        res
    }

    #[allow(clippy::too_many_arguments)]
    fn do_cdata_section(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        start_ptr: &mut Option<usize>,
        end: usize,
        next_ptr: &mut usize,
        have_more: bool,
        account: XmlAccount,
    ) -> u32 {
        let mut s = start_ptr.unwrap_or(0);
        self.set_event_ptr(is_doc_enc, s);
        *start_ptr = None;

        loop {
            let mut next = s;
            let tok = cdata_section_tok(enc, buf, s, end, &mut next);
            if !self.accounting.diff_tolerated(tok, s, next, account) {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            self.set_event_end_ptr(is_doc_enc, next);

            match tok {
                XML_TOK_CDATA_SECT_CLOSE => {
                    self.events.push(Event::CdataEnd);
                    *start_ptr = Some(next);
                    *next_ptr = next;
                    return XML_ERROR_NONE;
                }
                XML_TOK_DATA_NEWLINE => {
                    self.events.push(Event::Text(vec![b'\n']));
                }
                XML_TOK_DATA_CHARS => {
                    let mut text = Vec::new();
                    enc.convert_to_utf8(buf, s, next, &mut text);
                    self.events.push(Event::Text(text));
                }
                XML_TOK_INVALID => {
                    self.set_event_ptr(is_doc_enc, next);
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL_CHAR => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_PARTIAL_CHAR;
                }
                XML_TOK_PARTIAL | XML_TOK_NONE => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_UNCLOSED_CDATA_SECTION;
                }
                _ => {
                    self.set_event_ptr(is_doc_enc, next);
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }

            if self.reenter {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            self.set_event_ptr(is_doc_enc, next);
            s = next;
        }
    }

    fn ignore_section_processor(&mut self, start: usize, end: usize, end_ptr: &mut usize) -> u32 {
        let mut start_opt = Some(start);
        let res = self.do_ignore_section(
            self.encoding,
            true,
            self.input,
            &mut start_opt,
            end,
            end_ptr,
            false,
        );
        if res != XML_ERROR_NONE {
            return res;
        }
        if let Some(s) = start_opt {
            self.processor = ProcessorKind::Prolog;
            return self.prolog_processor(s, end, end_ptr);
        }
        res
    }

    #[allow(clippy::too_many_arguments)]
    fn do_ignore_section(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        start_ptr: &mut Option<usize>,
        end: usize,
        next_ptr: &mut usize,
        have_more: bool,
    ) -> u32 {
        let s = start_ptr.unwrap_or(0);
        let mut next = s;
        self.set_event_ptr(is_doc_enc, s);
        *start_ptr = None;
        let tok = ignore_section_tok(enc, buf, s, end, &mut next);
        if !self
            .accounting
            .diff_tolerated(tok, s, next, XmlAccount::Direct)
        {
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        self.set_event_end_ptr(is_doc_enc, next);

        match tok {
            XML_TOK_IGNORE_SECT => {
                *start_ptr = Some(next);
                *next_ptr = next;
                XML_ERROR_NONE
            }
            XML_TOK_INVALID => {
                self.set_event_ptr(is_doc_enc, next);
                XML_ERROR_INVALID_TOKEN
            }
            XML_TOK_PARTIAL_CHAR => {
                if have_more {
                    *next_ptr = s;
                    return XML_ERROR_NONE;
                }
                XML_ERROR_PARTIAL_CHAR
            }
            XML_TOK_PARTIAL | XML_TOK_NONE => {
                if have_more {
                    *next_ptr = s;
                    return XML_ERROR_NONE;
                }
                XML_ERROR_SYNTAX
            }
            _ => {
                self.set_event_ptr(is_doc_enc, next);
                XML_ERROR_UNEXPECTED_STATE
            }
        }
    }

    fn epilog_processor(&mut self, mut s: usize, end: usize, next_ptr: &mut usize) -> u32 {
        self.processor = ProcessorKind::Epilog;
        self.event_ptr = Some(s);
        loop {
            let mut next = s;
            let mut enc_out = self.encoding;
            let tok = xml_tok(
                self.encoding,
                XML_PROLOG_STATE,
                self.input,
                s,
                end,
                &mut next,
                &mut enc_out,
            );
            self.encoding = enc_out;
            if !self
                .accounting
                .diff_tolerated(tok, s, next, XmlAccount::Direct)
            {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            self.event_end_ptr = Some(next);
            match tok {
                t if t == -XML_TOK_PROLOG_S => {
                    *next_ptr = next;
                    return XML_ERROR_NONE;
                }
                XML_TOK_NONE => {
                    *next_ptr = s;
                    return XML_ERROR_NONE;
                }
                XML_TOK_PROLOG_S => {}
                XML_TOK_PI => {
                    self.report_processing_instruction(self.encoding, self.input, s, next);
                }
                XML_TOK_COMMENT => {
                    self.report_comment(self.encoding, self.input, s, next);
                }
                XML_TOK_INVALID => {
                    self.event_ptr = Some(next);
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL => {
                    return XML_ERROR_UNCLOSED_TOKEN;
                }
                XML_TOK_PARTIAL_CHAR => {
                    return XML_ERROR_PARTIAL_CHAR;
                }
                _ => {
                    return XML_ERROR_JUNK_AFTER_DOC_ELEMENT;
                }
            }
            if self.reenter {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            self.event_ptr = Some(next);
            s = next;
        }
    }

    fn process_entity(
        &mut self,
        entity_id: EntityRefId,
        between_decl: bool,
        entity_type: EntityType,
    ) -> u32 {
        let ent = self.dtd.entity_mut(entity_id);
        ent.open = true;
        ent.has_more = true;
        ent.processed = 0;

        let open_entity = OpenInternalEntity {
            internal_event_ptr: 0,
            internal_event_end_ptr: 0,
            entity: entity_id,
            start_tag_level: self.tag_level,
            between_decl,
            entity_type,
        };

        match entity_type {
            EntityType::Internal => {
                self.processor = ProcessorKind::InternalEntity;
                self.open_internal_entities.push(open_entity);
                self.reenter = true;
            }
            EntityType::Attribute => {
                self.open_attribute_entities.push(open_entity);
            }
            EntityType::Value => {
                self.open_value_entities.push(open_entity);
            }
        }
        XML_ERROR_NONE
    }

    fn internal_entity_processor(&mut self) -> u32 {
        let Some(open_entity) = self.open_internal_entities.last().cloned() else {
            return XML_ERROR_UNEXPECTED_STATE;
        };
        let entity_id = open_entity.entity;
        let (has_more, is_param, processed, text_opt) = {
            let ent = self.dtd.entity(entity_id);
            (ent.has_more, ent.is_param, ent.processed, ent.text.clone())
        };

        if has_more {
            let text = text_opt.unwrap_or_default();
            let text_start = processed;
            let text_end = text.len();
            let mut next = text_start;
            let result = if is_param {
                let tok = prolog_tok_normal(
                    self.internal_encoding,
                    &text,
                    text_start,
                    text_end,
                    &mut next,
                );
                self.do_prolog(
                    self.internal_encoding,
                    false,
                    &text,
                    text_start,
                    text_end,
                    tok,
                    next,
                    &mut next,
                    false,
                    false,
                    XmlAccount::EntityExpansion,
                )
            } else {
                self.do_content(
                    open_entity.start_tag_level,
                    self.internal_encoding,
                    false,
                    &text,
                    text_start,
                    text_end,
                    &mut next,
                    false,
                    XmlAccount::EntityExpansion,
                )
            };

            if result != XML_ERROR_NONE {
                return result;
            }
            if text_end != next && self.reenter {
                self.dtd.entity_mut(entity_id).processed = next;
                return result;
            }

            self.dtd.entity_mut(entity_id).has_more = false;
            if !is_param && open_entity.start_tag_level != self.tag_level {
                return XML_ERROR_ASYNC_ENTITY;
            }
            self.reenter = true;
            return result;
        }

        self.dtd.entity_mut(entity_id).open = false;
        self.open_internal_entities.pop();

        if self.open_internal_entities.is_empty() {
            self.processor = if is_param {
                ProcessorKind::Prolog
            } else {
                ProcessorKind::Content
            };
        }
        self.reenter = true;
        XML_ERROR_NONE
    }

    #[allow(clippy::too_many_arguments)]
    fn store_atts(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        att_str: usize,
        tag_name_str: &[u8],
        out_attrs: &mut Vec<(Vec<u8>, Vec<u8>)>,
        account: XmlAccount,
    ) -> u32 {
        let el_idx = self.dtd.get_or_create_element_type(tag_name_str);
        let mut raw_atts = std::mem::take(&mut self.atts);
        enc.get_atts(buf, att_str, &mut raw_atts);

        let mut used_att_ids = Vec::with_capacity(raw_atts.len());

        for curr_att in &raw_atts {
            let att_name_len = enc.name_length(buf, curr_att.name);
            let att_name = enc.store_string(buf, curr_att.name, curr_att.name + att_name_len);
            let att_id_idx = self.dtd.get_or_create_attribute_id(&att_name);

            if self.dtd.attribute_ids[att_id_idx].mark != 0 {
                for &idx in &used_att_ids {
                    self.dtd.attribute_ids[idx].mark = 0;
                }
                self.atts = raw_atts;
                if is_doc_enc {
                    self.event_ptr = Some(curr_att.name);
                }
                return XML_ERROR_DUPLICATE_ATTRIBUTE;
            }
            self.dtd.attribute_ids[att_id_idx].mark = 1;
            used_att_ids.push(att_id_idx);

            let val_vec = if !curr_att.normalized {
                let mut is_cdata = true;
                if self.dtd.attribute_ids[att_id_idx].maybe_tokenized {
                    let el_type = &self.dtd.element_types[el_idx];
                    if let Some(&da_idx) =
                        el_type.default_att_for_name.get(&self.dtd.attribute_ids[att_id_idx].name)
                    {
                        is_cdata = el_type.default_atts[da_idx].is_cdata;
                    }
                }
                let mut norm_val = Vec::new();
                let res = self.store_attribute_value(
                    enc,
                    is_doc_enc,
                    buf,
                    is_cdata,
                    curr_att.value_ptr,
                    curr_att.value_end,
                    &mut norm_val,
                    account,
                    false,
                );
                if res != XML_ERROR_NONE {
                    for &idx in &used_att_ids {
                        self.dtd.attribute_ids[idx].mark = 0;
                    }
                    self.atts = raw_atts;
                    return res;
                }
                norm_val
            } else {
                enc.store_string(buf, curr_att.value_ptr, curr_att.value_end)
            };

            out_attrs.push((self.dtd.attribute_ids[att_id_idx].name.clone(), val_vec));
        }

        let n_default_atts = self.dtd.element_types[el_idx].default_atts.len();
        for i in 0..n_default_atts {
            let da_id = self.dtd.element_types[el_idx].default_atts[i].id;
            if self.dtd.attribute_ids[da_id].mark == 0 {
                if let Some(val) = self.dtd.element_types[el_idx].default_atts[i].value.clone() {
                    self.dtd.attribute_ids[da_id].mark = 1;
                    used_att_ids.push(da_id);
                    out_attrs.push((self.dtd.attribute_ids[da_id].name.clone(), val));
                }
            }
        }

        for idx in used_att_ids {
            self.dtd.attribute_ids[idx].mark = 0;
        }
        self.atts = raw_atts;
        XML_ERROR_NONE
    }

    #[allow(clippy::too_many_arguments)]
    fn store_attribute_value(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        is_cdata: bool,
        ptr: usize,
        end: usize,
        pool: &mut Vec<u8>,
        account: XmlAccount,
        from_prolog: bool,
    ) -> u32 {
        let mut next = ptr;
        let mut result;

        loop {
            if self.open_attribute_entities.is_empty() {
                result = self.append_attribute_value(
                    enc,
                    is_doc_enc,
                    buf,
                    is_cdata,
                    next,
                    end,
                    pool,
                    account,
                    &mut next,
                    from_prolog,
                );
            } else {
                let Some(open_entity) = self.open_attribute_entities.last().cloned() else {
                    return XML_ERROR_UNEXPECTED_STATE;
                };
                let entity_id = open_entity.entity;
                let (has_more, processed, text_opt) = {
                    let ent = self.dtd.entity(entity_id);
                    (ent.has_more, ent.processed, ent.text.clone())
                };
                if has_more {
                    let text = text_opt.unwrap_or_default();
                    let text_start = processed;
                    let text_end = text.len();
                    let mut next_in_entity = text_start;
                    result = self.append_attribute_value(
                        self.internal_encoding,
                        false,
                        &text,
                        is_cdata,
                        text_start,
                        text_end,
                        pool,
                        XmlAccount::EntityExpansion,
                        &mut next_in_entity,
                        from_prolog,
                    );
                    if result != XML_ERROR_NONE {
                        break;
                    }
                    if next_in_entity < text_end {
                        self.dtd.entity_mut(entity_id).processed = next_in_entity;
                        continue;
                    }
                    self.dtd.entity_mut(entity_id).has_more = false;
                    continue;
                }

                self.dtd.entity_mut(entity_id).open = false;
                self.open_attribute_entities.pop();
                result = XML_ERROR_NONE;
            }

            if result != XML_ERROR_NONE || (self.open_attribute_entities.is_empty() && end == next)
            {
                break;
            }
        }

        if result != XML_ERROR_NONE {
            return result;
        }
        if !is_cdata && pool.last() == Some(&b' ') {
            pool.pop();
        }
        XML_ERROR_NONE
    }

    #[allow(clippy::too_many_arguments)]
    fn append_attribute_value(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        is_cdata: bool,
        mut ptr: usize,
        end: usize,
        pool: &mut Vec<u8>,
        account: XmlAccount,
        next_ptr: &mut usize,
        from_prolog: bool,
    ) -> u32 {
        let min = enc.min_bytes_per_char();
        loop {
            let mut next = ptr;
            let tok = attribute_value_tok(enc, buf, ptr, end, &mut next);
            if !self.accounting.diff_tolerated(tok, ptr, next, account) {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            match tok {
                XML_TOK_NONE => {
                    *next_ptr = next;
                    return XML_ERROR_NONE;
                }
                XML_TOK_INVALID => {
                    if is_doc_enc {
                        self.event_ptr = Some(next);
                    }
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL => {
                    if is_doc_enc {
                        self.event_ptr = Some(ptr);
                    }
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_CHAR_REF => {
                    let n = enc.char_ref_number(buf, ptr);
                    if n < 0 {
                        if is_doc_enc {
                            self.event_ptr = Some(ptr);
                        }
                        return XML_ERROR_BAD_CHAR_REF;
                    }
                    if !is_cdata && n == 0x20 && (pool.is_empty() || pool.last() == Some(&b' ')) {
                        // collapse space
                    } else {
                        let mut utf8_buf = [0u8; 4];
                        let len = xml_utf8_encode(n, &mut utf8_buf);
                        pool.extend_from_slice(&utf8_buf[..len]);
                    }
                }
                XML_TOK_DATA_CHARS => {
                    enc.convert_to_utf8(buf, ptr, next, pool);
                }
                XML_TOK_TRAILING_CR => {
                    next = ptr + min;
                    if !(!is_cdata && (pool.is_empty() || pool.last() == Some(&b' '))) {
                        pool.push(b' ');
                    }
                }
                XML_TOK_ATTRIBUTE_VALUE_S | XML_TOK_DATA_NEWLINE => {
                    if !(!is_cdata && (pool.is_empty() || pool.last() == Some(&b' '))) {
                        pool.push(b' ');
                    }
                }
                XML_TOK_ENTITY_REF => {
                    let ch = enc.predefined_entity_name(buf, ptr + min, next - min);
                    if ch != 0 {
                        self.accounting
                            .diff_tolerated(tok, 0, 1, XmlAccount::EntityExpansion);
                        pool.push(ch);
                    } else {
                        let name = enc.store_string(buf, ptr + min, next - min);
                        let ent_idx = self.dtd.lookup_general_entity(&name);
                        let check_entity_decl = if from_prolog {
                            self.prolog_state.document_entity
                                && if self.dtd.standalone {
                                    self.open_internal_entities.is_empty()
                                } else {
                                    !self.dtd.has_param_entity_refs
                                }
                        } else {
                            !self.dtd.has_param_entity_refs || self.dtd.standalone
                        };
                        if check_entity_decl {
                            match ent_idx {
                                None => return XML_ERROR_UNDEFINED_ENTITY,
                                Some(idx) => {
                                    if !self.dtd.general_entities[idx].is_internal {
                                        return XML_ERROR_ENTITY_DECLARED_IN_PE;
                                    }
                                }
                            }
                        } else if ent_idx.is_none() {
                            ptr = next;
                            continue;
                        }
                        if let Some(idx) = ent_idx {
                            let ent = &self.dtd.general_entities[idx];
                            if ent.open {
                                if is_doc_enc {
                                    self.event_ptr = Some(ptr);
                                }
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if ent.notation.is_some() {
                                if is_doc_enc {
                                    self.event_ptr = Some(ptr);
                                }
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if ent.text.is_none() {
                                if is_doc_enc {
                                    self.event_ptr = Some(ptr);
                                }
                                return XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            }
                            let res = self.process_entity(
                                EntityRefId::General(idx),
                                false,
                                EntityType::Attribute,
                            );
                            if res == XML_ERROR_NONE {
                                *next_ptr = next;
                            }
                            return res;
                        }
                    }
                }
                _ => {
                    if is_doc_enc {
                        self.event_ptr = Some(ptr);
                    }
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            ptr = next;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn call_store_entity_value(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        entity_text_ptr: usize,
        entity_text_end: usize,
        account: XmlAccount,
        pool: &mut Vec<u8>,
    ) -> u32 {
        let mut next = entity_text_ptr;
        let mut result;
        loop {
            if self.open_value_entities.is_empty() {
                result = self.store_entity_value(
                    enc,
                    is_doc_enc,
                    buf,
                    next,
                    entity_text_end,
                    account,
                    pool,
                    &mut next,
                );
            } else {
                let Some(open_entity) = self.open_value_entities.last().cloned() else {
                    return XML_ERROR_UNEXPECTED_STATE;
                };
                let entity_id = open_entity.entity;
                let (has_more, processed, text_opt) = {
                    let ent = self.dtd.entity(entity_id);
                    (ent.has_more, ent.processed, ent.text.clone())
                };
                if has_more {
                    let text = text_opt.unwrap_or_default();
                    let text_start = processed;
                    let text_end = text.len();
                    let mut next_in_entity = text_start;
                    result = self.store_entity_value(
                        self.internal_encoding,
                        false,
                        &text,
                        text_start,
                        text_end,
                        XmlAccount::EntityExpansion,
                        pool,
                        &mut next_in_entity,
                    );
                    if result != XML_ERROR_NONE {
                        break;
                    }
                    if text_end != next_in_entity {
                        self.dtd.entity_mut(entity_id).processed = next_in_entity;
                        continue;
                    }
                    self.dtd.entity_mut(entity_id).has_more = false;
                    continue;
                }

                self.dtd.entity_mut(entity_id).open = false;
                self.open_value_entities.pop();
                result = XML_ERROR_NONE;
            }

            if result != XML_ERROR_NONE
                || (self.open_value_entities.is_empty() && entity_text_end == next)
            {
                break;
            }
        }
        result
    }

    #[allow(clippy::too_many_arguments)]
    fn store_entity_value(
        &mut self,
        enc: Encoding,
        is_doc_enc: bool,
        buf: &[u8],
        mut entity_text_ptr: usize,
        entity_text_end: usize,
        account: XmlAccount,
        pool: &mut Vec<u8>,
        next_ptr: &mut usize,
    ) -> u32 {
        let old_in_entity_value = self.prolog_state.in_entity_value;
        self.prolog_state.in_entity_value = true;
        let mut next = entity_text_ptr;
        let min = enc.min_bytes_per_char();

        if entity_text_ptr >= entity_text_end {
            self.prolog_state.in_entity_value = old_in_entity_value;
            *next_ptr = next;
            return XML_ERROR_NONE;
        }

        let result;
        loop {
            next = entity_text_ptr;
            let tok = entity_value_tok(enc, buf, entity_text_ptr, entity_text_end, &mut next);
            if !self
                .accounting
                .diff_tolerated(tok, entity_text_ptr, next, account)
            {
                result = XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                break;
            }

            match tok {
                XML_TOK_PARAM_ENTITY_REF => {
                    self.event_ptr = Some(entity_text_ptr);
                    result = XML_ERROR_PARAM_ENTITY_REF;
                    break;
                }
                XML_TOK_NONE => {
                    result = XML_ERROR_NONE;
                    break;
                }
                XML_TOK_ENTITY_REF | XML_TOK_DATA_CHARS => {
                    enc.convert_to_utf8(buf, entity_text_ptr, next, pool);
                }
                XML_TOK_TRAILING_CR => {
                    next = entity_text_ptr + min;
                    pool.push(b'\n');
                }
                XML_TOK_DATA_NEWLINE => {
                    pool.push(b'\n');
                }
                XML_TOK_CHAR_REF => {
                    let n = enc.char_ref_number(buf, entity_text_ptr);
                    if n < 0 {
                        if is_doc_enc {
                            self.event_ptr = Some(entity_text_ptr);
                        }
                        result = XML_ERROR_BAD_CHAR_REF;
                        break;
                    }
                    let mut utf8_buf = [0u8; 4];
                    let len = xml_utf8_encode(n, &mut utf8_buf);
                    pool.extend_from_slice(&utf8_buf[..len]);
                }
                XML_TOK_PARTIAL => {
                    if is_doc_enc {
                        self.event_ptr = Some(entity_text_ptr);
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                XML_TOK_INVALID => {
                    if is_doc_enc {
                        self.event_ptr = Some(next);
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                _ => {
                    if is_doc_enc {
                        self.event_ptr = Some(entity_text_ptr);
                    }
                    result = XML_ERROR_UNEXPECTED_STATE;
                    break;
                }
            }
            entity_text_ptr = next;
        }

        self.prolog_state.in_entity_value = old_in_entity_value;
        *next_ptr = next;
        result
    }

    fn define_attribute(
        &mut self,
        el_idx: usize,
        att_id_idx: usize,
        is_cdata: bool,
        is_id: bool,
        value: Option<Vec<u8>>,
    ) {
        let att_name = self.dtd.attribute_ids[att_id_idx].name.clone();
        let el_type = &mut self.dtd.element_types[el_idx];
        if value.is_some() || is_id {
            if el_type.default_att_for_name.contains_key(&att_name) {
                return;
            }
            if is_id && el_type.id_att.is_none() {
                el_type.id_att = Some(att_id_idx);
            }
        }
        let new_att_index = el_type.default_atts.len();
        el_type.default_atts.push(DefaultAttribute {
            id: att_id_idx,
            is_cdata,
            value,
        });
        el_type
            .default_att_for_name
            .entry(att_name)
            .or_insert(new_att_index);
        if !is_cdata {
            self.dtd.attribute_ids[att_id_idx].maybe_tokenized = true;
        }
    }

    fn report_processing_instruction(
        &mut self,
        enc: Encoding,
        buf: &[u8],
        mut start: usize,
        end: usize,
    ) {
        let min = enc.min_bytes_per_char();
        start += min * 2;
        let tem = start + enc.name_length(buf, start);
        let target = enc.store_string(buf, start, tem);
        let data_start = enc.skip_s(buf, tem);
        let mut data = enc.store_string(buf, data_start, end - min * 2);
        normalize_lines(&mut data);
        self.events.push(Event::Pi { target, data });
    }

    fn report_comment(&mut self, enc: Encoding, buf: &[u8], start: usize, end: usize) {
        let min = enc.min_bytes_per_char();
        let mut data = enc.store_string(buf, start + min * 4, end - min * 3);
        normalize_lines(&mut data);
        self.events.push(Event::Comment(data));
    }
}

fn normalize_lines(s: &mut Vec<u8>) {
    let Some(first_cr) = s.iter().position(|&b| b == b'\r') else {
        return;
    };
    let mut p = first_cr;
    let mut i = first_cr;
    let len = s.len();
    while i < len {
        if s[i] == b'\r' {
            s[p] = b'\n';
            p += 1;
            i += 1;
            if i < len && s[i] == b'\n' {
                i += 1;
            }
        } else {
            s[p] = s[i];
            p += 1;
            i += 1;
        }
    }
    s.truncate(p);
}

fn normalize_public_id(s: &mut Vec<u8>) {
    let mut p = 0usize;
    let len = s.len();
    for i in 0..len {
        match s[i] {
            0x20 | 0x0D | 0x0A => {
                if p != 0 && s[p - 1] != 0x20 {
                    s[p] = 0x20;
                    p += 1;
                }
            }
            b => {
                s[p] = b;
                p += 1;
            }
        }
    }
    if p != 0 && s[p - 1] == 0x20 {
        p -= 1;
    }
    s.truncate(p);
}

pub fn parse(input: &[u8]) -> Result<Vec<Event>, XmlError> {
    let mut parser = Parser::new(input);
    let err = parser.run();
    if err == XML_ERROR_NONE {
        Ok(parser.events)
    } else {
        let (line, col) = parser.current_line_and_col();
        Err(XmlError {
            code: err,
            line,
            col,
        })
    }
}
