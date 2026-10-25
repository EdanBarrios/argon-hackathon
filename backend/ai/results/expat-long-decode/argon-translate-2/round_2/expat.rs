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

const ASCII_A: u8 = 0x41;
const ASCII_B: u8 = 0x42;
const ASCII_C: u8 = 0x43;
const ASCII_D: u8 = 0x44;
const ASCII_E: u8 = 0x45;
const ASCII_F: u8 = 0x46;
const ASCII_G: u8 = 0x47;
const ASCII_H: u8 = 0x48;
const ASCII_I: u8 = 0x49;
const ASCII_J: u8 = 0x4A;
const ASCII_K: u8 = 0x4B;
const ASCII_L: u8 = 0x4C;
const ASCII_M: u8 = 0x4D;
const ASCII_N: u8 = 0x4E;
const ASCII_O: u8 = 0x4F;
const ASCII_P: u8 = 0x50;
const ASCII_Q: u8 = 0x51;
const ASCII_R: u8 = 0x52;
const ASCII_S: u8 = 0x53;
const ASCII_T: u8 = 0x54;
const ASCII_U: u8 = 0x55;
const ASCII_V: u8 = 0x56;
const ASCII_W: u8 = 0x57;
const ASCII_X: u8 = 0x58;
const ASCII_Y: u8 = 0x59;
const ASCII_Z: u8 = 0x5A;

const ASCII_LOWER_A: u8 = 0x61;
const ASCII_LOWER_B: u8 = 0x62;
const ASCII_LOWER_C: u8 = 0x63;
const ASCII_LOWER_D: u8 = 0x64;
const ASCII_LOWER_E: u8 = 0x65;
const ASCII_LOWER_F: u8 = 0x66;
const ASCII_LOWER_G: u8 = 0x67;
const ASCII_LOWER_H: u8 = 0x68;
const ASCII_LOWER_I: u8 = 0x69;
const ASCII_LOWER_J: u8 = 0x6A;
const ASCII_LOWER_K: u8 = 0x6B;
const ASCII_LOWER_L: u8 = 0x6C;
const ASCII_LOWER_M: u8 = 0x6D;
const ASCII_LOWER_N: u8 = 0x6E;
const ASCII_LOWER_O: u8 = 0x6F;
const ASCII_LOWER_P: u8 = 0x70;
const ASCII_LOWER_Q: u8 = 0x71;
const ASCII_LOWER_R: u8 = 0x72;
const ASCII_LOWER_S: u8 = 0x73;
const ASCII_LOWER_T: u8 = 0x74;
const ASCII_LOWER_U: u8 = 0x75;
const ASCII_LOWER_V: u8 = 0x76;
const ASCII_LOWER_W: u8 = 0x77;
const ASCII_LOWER_X: u8 = 0x78;
const ASCII_LOWER_Y: u8 = 0x79;
const ASCII_LOWER_Z: u8 = 0x7A;

const ASCII_0: u8 = 0x30;
const ASCII_1: u8 = 0x31;
const ASCII_2: u8 = 0x32;
const ASCII_3: u8 = 0x33;
const ASCII_4: u8 = 0x34;
const ASCII_5: u8 = 0x35;
const ASCII_6: u8 = 0x36;
const ASCII_7: u8 = 0x37;
const ASCII_8: u8 = 0x38;
const ASCII_9: u8 = 0x39;

const ASCII_TAB: u8 = 0x09;
const ASCII_SPACE: u8 = 0x20;
const ASCII_EXCL: u8 = 0x21;
const ASCII_QUOT: u8 = 0x22;
const ASCII_AMP: u8 = 0x26;
const ASCII_APOS: u8 = 0x27;
const ASCII_MINUS: u8 = 0x2D;
const ASCII_PERIOD: u8 = 0x2E;
const ASCII_COLON: u8 = 0x3A;
const ASCII_SEMI: u8 = 0x3B;
const ASCII_LT: u8 = 0x3C;
const ASCII_EQUALS: u8 = 0x3D;
const ASCII_GT: u8 = 0x3E;
const ASCII_LSQB: u8 = 0x5B;
const ASCII_RSQB: u8 = 0x5D;
const ASCII_UNDERSCORE: u8 = 0x5F;
const ASCII_LPAREN: u8 = 0x28;
const ASCII_RPAREN: u8 = 0x29;
const ASCII_FF: u8 = 0x0C;
const ASCII_SLASH: u8 = 0x2F;
const ASCII_HASH: u8 = 0x23;
const ASCII_PIPE: u8 = 0x7C;
const ASCII_COMMA: u8 = 0x2C;

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
const XML_CDATA_SECTION_STATE: usize = 2;
const XML_IGNORE_SECTION_STATE: usize = 3;

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
const BT_COLON: u8 = 23;
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

#[rustfmt::skip]
static NAMING_BITMAP: [u32; 320] = [
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

#[rustfmt::skip]
static NMSTRT_PAGES: [u8; 256] = [
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

#[rustfmt::skip]
static NAME_PAGES: [u8; 256] = [
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

// asciitab.h with BT_COLON replaced by BT_NMSTRT (non-NS mode)
#[rustfmt::skip]
const ASCII_TAB_NO_NS: [u8; 128] = [
    /* 0x00 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x04 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x08 */ BT_NONXML, BT_S,      BT_LF,     BT_NONXML,
    /* 0x0C */ BT_NONXML, BT_CR,     BT_NONXML, BT_NONXML,
    /* 0x10 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x14 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x18 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x1C */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x20 */ BT_S,      BT_EXCL,   BT_QUOT,   BT_NUM,
    /* 0x24 */ BT_OTHER,  BT_PERCNT, BT_AMP,    BT_APOS,
    /* 0x28 */ BT_LPAR,   BT_RPAR,   BT_AST,    BT_PLUS,
    /* 0x2C */ BT_COMMA,  BT_MINUS,  BT_NAME,   BT_SOL,
    /* 0x30 */ BT_DIGIT,  BT_DIGIT,  BT_DIGIT,  BT_DIGIT,
    /* 0x34 */ BT_DIGIT,  BT_DIGIT,  BT_DIGIT,  BT_DIGIT,
    /* 0x38 */ BT_DIGIT,  BT_DIGIT,  BT_NMSTRT, BT_SEMI,
    /* 0x3C */ BT_LT,     BT_EQUALS, BT_GT,     BT_QUEST,
    /* 0x40 */ BT_OTHER,  BT_HEX,    BT_HEX,    BT_HEX,
    /* 0x44 */ BT_HEX,    BT_HEX,    BT_HEX,    BT_NMSTRT,
    /* 0x48 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x4C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x50 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x54 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x58 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_LSQB,
    /* 0x5C */ BT_OTHER,  BT_RSQB,   BT_OTHER,  BT_NMSTRT,
    /* 0x60 */ BT_OTHER,  BT_HEX,    BT_HEX,    BT_HEX,
    /* 0x64 */ BT_HEX,    BT_HEX,    BT_HEX,    BT_NMSTRT,
    /* 0x68 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x6C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x70 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x74 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x78 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0x7C */ BT_VERBAR, BT_OTHER,  BT_OTHER,  BT_OTHER,
];

// iasciitab.h with BT_COLON replaced by BT_NMSTRT (0x0D is BT_S)
#[rustfmt::skip]
const IASCII_TAB_NO_NS: [u8; 128] = [
    /* 0x00 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x04 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x08 */ BT_NONXML, BT_S,      BT_LF,     BT_NONXML,
    /* 0x0C */ BT_NONXML, BT_S,      BT_NONXML, BT_NONXML,
    /* 0x10 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x14 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x18 */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x1C */ BT_NONXML, BT_NONXML, BT_NONXML, BT_NONXML,
    /* 0x20 */ BT_S,      BT_EXCL,   BT_QUOT,   BT_NUM,
    /* 0x24 */ BT_OTHER,  BT_PERCNT, BT_AMP,    BT_APOS,
    /* 0x28 */ BT_LPAR,   BT_RPAR,   BT_AST,    BT_PLUS,
    /* 0x2C */ BT_COMMA,  BT_MINUS,  BT_NAME,   BT_SOL,
    /* 0x30 */ BT_DIGIT,  BT_DIGIT,  BT_DIGIT,  BT_DIGIT,
    /* 0x34 */ BT_DIGIT,  BT_DIGIT,  BT_DIGIT,  BT_DIGIT,
    /* 0x38 */ BT_DIGIT,  BT_DIGIT,  BT_NMSTRT, BT_SEMI,
    /* 0x3C */ BT_LT,     BT_EQUALS, BT_GT,     BT_QUEST,
    /* 0x40 */ BT_OTHER,  BT_HEX,    BT_HEX,    BT_HEX,
    /* 0x44 */ BT_HEX,    BT_HEX,    BT_HEX,    BT_NMSTRT,
    /* 0x48 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x4C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x50 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x54 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x58 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_LSQB,
    /* 0x5C */ BT_OTHER,  BT_RSQB,   BT_OTHER,  BT_NMSTRT,
    /* 0x60 */ BT_OTHER,  BT_HEX,    BT_HEX,    BT_HEX,
    /* 0x64 */ BT_HEX,    BT_HEX,    BT_HEX,    BT_NMSTRT,
    /* 0x68 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x6C */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x70 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x74 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_NMSTRT,
    /* 0x78 */ BT_NMSTRT, BT_NMSTRT, BT_NMSTRT, BT_OTHER,
    /* 0x7C */ BT_VERBAR, BT_OTHER,  BT_OTHER,  BT_OTHER,
];

#[rustfmt::skip]
const UTF8_HIGH_TAB: [u8; 128] = [
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

#[rustfmt::skip]
const LATIN1_HIGH_TAB: [u8; 128] = [
    /* 0x80 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x84 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x88 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x8C */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x90 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x94 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x98 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0x9C */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0xA0 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0xA4 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0xA8 */ BT_OTHER,  BT_OTHER,  BT_NMSTRT, BT_OTHER,
    /* 0xAC */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0xB0 */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
    /* 0xB4 */ BT_OTHER,  BT_NMSTRT, BT_OTHER,  BT_NAME,
    /* 0xB8 */ BT_OTHER,  BT_OTHER,  BT_NMSTRT, BT_OTHER,
    /* 0xBC */ BT_OTHER,  BT_OTHER,  BT_OTHER,  BT_OTHER,
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

const ASCII_HIGH_TAB: [u8; 128] = [BT_NONXML; 128];

const fn make_byte_table(low: &[u8; 128], high: &[u8; 128]) -> [u8; 256] {
    let mut out = [0u8; 256];
    let mut i = 0;
    while i < 128 {
        out[i] = low[i];
        out[i + 128] = high[i];
        i += 1;
    }
    out
}

static UTF8_TABLE: [u8; 256] = make_byte_table(&ASCII_TAB_NO_NS, &UTF8_HIGH_TAB);
static INTERNAL_UTF8_TABLE: [u8; 256] = make_byte_table(&IASCII_TAB_NO_NS, &UTF8_HIGH_TAB);
static LATIN1_TABLE: [u8; 256] = make_byte_table(&ASCII_TAB_NO_NS, &LATIN1_HIGH_TAB);
static ASCII_TABLE: [u8; 256] = make_byte_table(&ASCII_TAB_NO_NS, &ASCII_HIGH_TAB);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum EncId {
    Init,
    Utf8,
    InternalUtf8,
    Latin1,
    Ascii,
    Utf16Be,
    Utf16Le,
}

impl EncId {
    #[inline]
    fn min_bpc(self) -> usize {
        match self {
            EncId::Utf16Be | EncId::Utf16Le => 2,
            _ => 1,
        }
    }

    #[inline]
    fn is_utf8(self) -> bool {
        matches!(self, EncId::Utf8 | EncId::InternalUtf8 | EncId::Ascii)
    }

    #[inline]
    fn byte_type(self, buf: &[u8], ptr: usize) -> u8 {
        match self {
            EncId::Utf8 | EncId::Init => UTF8_TABLE[buf[ptr] as usize],
            EncId::InternalUtf8 => INTERNAL_UTF8_TABLE[buf[ptr] as usize],
            EncId::Latin1 => LATIN1_TABLE[buf[ptr] as usize],
            EncId::Ascii => ASCII_TABLE[buf[ptr] as usize],
            EncId::Utf16Le => {
                let p0 = buf[ptr];
                let p1 = buf[ptr + 1];
                if p1 == 0 {
                    LATIN1_TABLE[p0 as usize]
                } else {
                    unicode_byte_type(p1, p0)
                }
            }
            EncId::Utf16Be => {
                let p0 = buf[ptr];
                let p1 = buf[ptr + 1];
                if p0 == 0 {
                    LATIN1_TABLE[p1 as usize]
                } else {
                    unicode_byte_type(p0, p1)
                }
            }
        }
    }

    #[inline]
    fn byte_to_ascii(self, buf: &[u8], ptr: usize) -> i32 {
        match self {
            EncId::Utf16Le => {
                if buf[ptr + 1] == 0 {
                    buf[ptr] as i32
                } else {
                    -1
                }
            }
            EncId::Utf16Be => {
                if buf[ptr] == 0 {
                    buf[ptr + 1] as i32
                } else {
                    -1
                }
            }
            _ => buf[ptr] as i32,
        }
    }

    #[inline]
    fn char_matches(self, buf: &[u8], ptr: usize, c: u8) -> bool {
        match self {
            EncId::Utf16Le => buf[ptr + 1] == 0 && buf[ptr] == c,
            EncId::Utf16Be => buf[ptr] == 0 && buf[ptr + 1] == c,
            _ => buf[ptr] == c,
        }
    }

    #[inline]
    fn is_name_char(self, buf: &[u8], ptr: usize, n: usize) -> bool {
        match self {
            EncId::Utf8 | EncId::InternalUtf8 | EncId::Init => match n {
                2 => utf8_get_naming2(&NAME_PAGES, &buf[ptr..ptr + 2]),
                3 => utf8_get_naming3(&NAME_PAGES, &buf[ptr..ptr + 3]),
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn is_nmstrt_char(self, buf: &[u8], ptr: usize, n: usize) -> bool {
        match self {
            EncId::Utf8 | EncId::InternalUtf8 | EncId::Init => match n {
                2 => utf8_get_naming2(&NMSTRT_PAGES, &buf[ptr..ptr + 2]),
                3 => utf8_get_naming3(&NMSTRT_PAGES, &buf[ptr..ptr + 3]),
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn is_invalid_char(self, buf: &[u8], ptr: usize, n: usize) -> bool {
        match self {
            EncId::Utf8 | EncId::InternalUtf8 | EncId::Init => match n {
                2 => utf8_invalid2(&buf[ptr..ptr + 2]),
                3 => utf8_invalid3(&buf[ptr..ptr + 3]),
                4 => utf8_invalid4(&buf[ptr..ptr + 4]),
                _ => false,
            },
            EncId::Utf16Le => n == 4 && (buf[ptr + 3] & 0xFC) != 0xDC,
            EncId::Utf16Be => n == 4 && (buf[ptr + 2] & 0xFC) != 0xDC,
            _ => false,
        }
    }

    #[inline]
    fn is_name_char_minbpc(self, buf: &[u8], ptr: usize) -> bool {
        match self {
            EncId::Utf16Le => ucs2_get_naming(&NAME_PAGES, buf[ptr + 1], buf[ptr]),
            EncId::Utf16Be => ucs2_get_naming(&NAME_PAGES, buf[ptr], buf[ptr + 1]),
            _ => false,
        }
    }

    #[inline]
    fn is_nmstrt_char_minbpc(self, buf: &[u8], ptr: usize) -> bool {
        match self {
            EncId::Utf16Le => ucs2_get_naming(&NMSTRT_PAGES, buf[ptr + 1], buf[ptr]),
            EncId::Utf16Be => ucs2_get_naming(&NMSTRT_PAGES, buf[ptr], buf[ptr + 1]),
            _ => false,
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
fn ucs2_get_naming(pages: &[u8; 256], hi: u8, lo: u8) -> bool {
    let idx = ((pages[hi as usize] as usize) << 3) + ((lo as usize) >> 5);
    (NAMING_BITMAP[idx] & (1u32 << (lo & 0x1F))) != 0
}

#[inline]
fn utf8_get_naming2(pages: &[u8; 256], byte: &[u8]) -> bool {
    let idx = ((pages[((byte[0] >> 2) & 7) as usize] as usize) << 3)
        + (((byte[0] & 3) as usize) << 1)
        + (((byte[1] >> 5) & 1) as usize);
    (NAMING_BITMAP[idx] & (1u32 << (byte[1] & 0x1F))) != 0
}

#[inline]
fn utf8_get_naming3(pages: &[u8; 256], byte: &[u8]) -> bool {
    let page_idx = (((byte[0] & 0xF) as usize) << 4) + (((byte[1] >> 2) & 0xF) as usize);
    let idx = ((pages[page_idx] as usize) << 3)
        + (((byte[1] & 3) as usize) << 1)
        + (((byte[2] >> 5) & 1) as usize);
    (NAMING_BITMAP[idx] & (1u32 << (byte[2] & 0x1F))) != 0
}

#[inline]
fn utf8_invalid2(p: &[u8]) -> bool {
    p[0] < 0xC2 || (p[1] & 0x80) == 0 || (p[1] & 0xC0) == 0xC0
}

#[inline]
fn utf8_invalid3(p: &[u8]) -> bool {
    (p[2] & 0x80) == 0
        || (if p[0] == 0xEF && p[1] == 0xBF {
            p[2] > 0xBD
        } else {
            (p[2] & 0xC0) == 0xC0
        })
        || (if p[0] == 0xE0 {
            p[1] < 0xA0 || (p[1] & 0xC0) == 0xC0
        } else {
            (p[1] & 0x80) == 0
                || (if p[0] == 0xED {
                    p[1] > 0x9F
                } else {
                    (p[1] & 0xC0) == 0xC0
                })
        })
}

#[inline]
fn utf8_invalid4(p: &[u8]) -> bool {
    (p[3] & 0x80) == 0
        || (p[3] & 0xC0) == 0xC0
        || (p[2] & 0x80) == 0
        || (p[2] & 0xC0) == 0xC0
        || (if p[0] == 0xF0 {
            p[1] < 0x90 || (p[1] & 0xC0) == 0xC0
        } else {
            (p[1] & 0x80) == 0
                || (if p[0] == 0xF4 {
                    p[1] > 0x8F
                } else {
                    (p[1] & 0xC0) == 0xC0
                })
        })
}

#[inline]
fn has_chars(enc: EncId, ptr: usize, end: usize, count: usize) -> bool {
    end >= ptr && (end - ptr) >= count * enc.min_bpc()
}

#[inline]
fn has_char(enc: EncId, ptr: usize, end: usize) -> bool {
    has_chars(enc, ptr, end, 1)
}

macro_rules! require_chars {
    ($enc:expr, $ptr:expr, $end:expr, $count:expr) => {
        if !has_chars($enc, $ptr, $end, $count) {
            return XML_TOK_PARTIAL;
        }
    };
}

macro_rules! require_char {
    ($enc:expr, $ptr:expr, $end:expr) => {
        require_chars!($enc, $ptr, $end, 1);
    };
}

macro_rules! invalid_lead_case {
    ($n:expr, $enc:expr, $buf:expr, $ptr:ident, $end:expr, $next_tok_ptr:expr) => {{
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
    ($n:expr, $enc:expr, $buf:expr, $ptr:ident, $end:expr, $next_tok_ptr:expr) => {{
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
    ($n:expr, $enc:expr, $buf:expr, $ptr:ident, $end:expr, $next_tok_ptr:expr) => {{
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

fn scan_comment(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    if has_char(enc, ptr, end) {
        if !enc.char_matches(buf, ptr, ASCII_MINUS) {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
        ptr += min_bpc;
        while has_char(enc, ptr, end) {
            match enc.byte_type(buf, ptr) {
                BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
                BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
                BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                BT_MINUS => {
                    ptr += min_bpc;
                    require_char!(enc, ptr, end);
                    if enc.char_matches(buf, ptr, ASCII_MINUS) {
                        ptr += min_bpc;
                        require_char!(enc, ptr, end);
                        if !enc.char_matches(buf, ptr, ASCII_GT) {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        *next_tok_ptr = ptr + min_bpc;
                        return XML_TOK_COMMENT;
                    }
                }
                _ => {
                    ptr += min_bpc;
                }
            }
        }
    }
    XML_TOK_PARTIAL
}

fn scan_decl(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_MINUS => return scan_comment(enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_LSQB => {
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_COND_SECT_OPEN;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
        }
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_PERCNT => {
                require_chars!(enc, ptr, end, 2);
                match enc.byte_type(buf, ptr + min_bpc) {
                    BT_S | BT_CR | BT_LF | BT_PERCNT => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_INVALID;
                    }
                    _ => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_DECL_OPEN;
                    }
                }
            }
            BT_S | BT_CR | BT_LF => {
                *next_tok_ptr = ptr;
                return XML_TOK_DECL_OPEN;
            }
            BT_NMSTRT | BT_HEX => {
                ptr += min_bpc;
            }
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn check_pi_target(enc: EncId, buf: &[u8], mut ptr: usize, end: usize, tok_ptr: &mut i32) -> bool {
    let min_bpc = enc.min_bpc();
    let mut upper = false;
    *tok_ptr = XML_TOK_PI;
    if end - ptr != min_bpc * 3 {
        return true;
    }
    match enc.byte_to_ascii(buf, ptr) {
        x if x == ASCII_LOWER_X as i32 => {}
        x if x == ASCII_X as i32 => upper = true,
        _ => return true,
    }
    ptr += min_bpc;
    match enc.byte_to_ascii(buf, ptr) {
        x if x == ASCII_LOWER_M as i32 => {}
        x if x == ASCII_M as i32 => upper = true,
        _ => return true,
    }
    ptr += min_bpc;
    match enc.byte_to_ascii(buf, ptr) {
        x if x == ASCII_LOWER_L as i32 => {}
        x if x == ASCII_L as i32 => upper = true,
        _ => return true,
    }
    if upper {
        return false;
    }
    *tok_ptr = XML_TOK_XML_DECL;
    true
}

fn scan_pi(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    let mut tok = 0;
    let target = ptr;
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min_bpc;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF => {
                if !check_pi_target(enc, buf, target, ptr, &mut tok) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
                while has_char(enc, ptr, end) {
                    match enc.byte_type(buf, ptr) {
                        BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        BT_QUEST => {
                            ptr += min_bpc;
                            require_char!(enc, ptr, end);
                            if enc.char_matches(buf, ptr, ASCII_GT) {
                                *next_tok_ptr = ptr + min_bpc;
                                return tok;
                            }
                        }
                        _ => {
                            ptr += min_bpc;
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
                ptr += min_bpc;
                require_char!(enc, ptr, end);
                if enc.char_matches(buf, ptr, ASCII_GT) {
                    *next_tok_ptr = ptr + min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    const CDATA_LSQB: [u8; 6] = [ASCII_C, ASCII_D, ASCII_A, ASCII_T, ASCII_A, ASCII_LSQB];
    let min_bpc = enc.min_bpc();
    require_chars!(enc, ptr, end, 6);
    for &c in &CDATA_LSQB {
        if !enc.char_matches(buf, ptr, c) {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
        ptr += min_bpc;
    }
    *next_tok_ptr = ptr;
    XML_TOK_CDATA_SECT_OPEN
}

fn cdata_section_tok(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    end = end.min(buf.len());
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min_bpc = enc.min_bpc();
    if min_bpc > 1 {
        let mut n = end - ptr;
        if (n & (min_bpc - 1)) != 0 {
            n &= !(min_bpc - 1);
            if n == 0 {
                return XML_TOK_PARTIAL;
            }
            end = ptr + n;
        }
    }
    match enc.byte_type(buf, ptr) {
        BT_RSQB => {
            ptr += min_bpc;
            require_char!(enc, ptr, end);
            if enc.char_matches(buf, ptr, ASCII_RSQB) {
                ptr += min_bpc;
                require_char!(enc, ptr, end);
                if !enc.char_matches(buf, ptr, ASCII_GT) {
                    ptr -= min_bpc;
                } else {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_CDATA_SECT_CLOSE;
                }
            }
        }
        BT_CR => {
            ptr += min_bpc;
            require_char!(enc, ptr, end);
            if enc.byte_type(buf, ptr) == BT_LF {
                ptr += min_bpc;
            }
            *next_tok_ptr = ptr;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_LF => {
            *next_tok_ptr = ptr + min_bpc;
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
            ptr += min_bpc;
        }
    }
    while has_char(enc, ptr, end) {
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
                ptr += min_bpc;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn scan_end_tag(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min_bpc;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF => {
                ptr += min_bpc;
                while has_char(enc, ptr, end) {
                    match enc.byte_type(buf, ptr) {
                        BT_S | BT_CR | BT_LF => {}
                        BT_GT => {
                            *next_tok_ptr = ptr + min_bpc;
                            return XML_TOK_END_TAG;
                        }
                        _ => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    ptr += min_bpc;
                }
                return XML_TOK_PARTIAL;
            }
            BT_GT => {
                *next_tok_ptr = ptr + min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    if has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_DIGIT | BT_HEX => {}
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        ptr += min_bpc;
        while has_char(enc, ptr, end) {
            match enc.byte_type(buf, ptr) {
                BT_DIGIT | BT_HEX => {}
                BT_SEMI => {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_CHAR_REF;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr += min_bpc;
        }
    }
    XML_TOK_PARTIAL
}

fn scan_char_ref(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    if has_char(enc, ptr, end) {
        if enc.char_matches(buf, ptr, ASCII_LOWER_X) {
            return scan_hex_char_ref(enc, buf, ptr + min_bpc, end, next_tok_ptr);
        }
        match enc.byte_type(buf, ptr) {
            BT_DIGIT => {}
            _ => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
        }
        ptr += min_bpc;
        while has_char(enc, ptr, end) {
            match enc.byte_type(buf, ptr) {
                BT_DIGIT => {}
                BT_SEMI => {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_CHAR_REF;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr += min_bpc;
        }
    }
    XML_TOK_PARTIAL
}

fn scan_ref(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min_bpc;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_NUM => {
            return scan_char_ref(enc, buf, ptr + min_bpc, end, next_tok_ptr);
        }
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_SEMI => {
                *next_tok_ptr = ptr + min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            t @ (BT_S | BT_CR | BT_LF | BT_EQUALS) => {
                if t != BT_EQUALS {
                    loop {
                        ptr += min_bpc;
                        require_char!(enc, ptr, end);
                        let t2 = enc.byte_type(buf, ptr);
                        if t2 == BT_EQUALS {
                            break;
                        }
                        match t2 {
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
                    ptr += min_bpc;
                    require_char!(enc, ptr, end);
                    let b = enc.byte_type(buf, ptr);
                    if b == BT_QUOT || b == BT_APOS {
                        open = b;
                        break;
                    }
                    match b {
                        BT_S | BT_LF | BT_CR => {}
                        _ => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                }
                ptr += min_bpc;
                // in attribute value
                loop {
                    require_char!(enc, ptr, end);
                    let b = enc.byte_type(buf, ptr);
                    if b == open {
                        break;
                    }
                    match b {
                        BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        BT_AMP => {
                            let tok = scan_ref(enc, buf, ptr + min_bpc, end, &mut ptr);
                            if tok <= 0 {
                                if tok == XML_TOK_INVALID {
                                    *next_tok_ptr = ptr;
                                }
                                return tok;
                            }
                        }
                        BT_LT => {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        _ => {
                            ptr += min_bpc;
                        }
                    }
                }
                ptr += min_bpc;
                require_char!(enc, ptr, end);
                match enc.byte_type(buf, ptr) {
                    BT_S | BT_CR | BT_LF => {}
                    BT_SOL => {
                        ptr += min_bpc;
                        require_char!(enc, ptr, end);
                        if !enc.char_matches(buf, ptr, ASCII_GT) {
                            *next_tok_ptr = ptr;
                            return XML_TOK_INVALID;
                        }
                        *next_tok_ptr = ptr + min_bpc;
                        return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                    }
                    BT_GT => {
                        *next_tok_ptr = ptr + min_bpc;
                        return XML_TOK_START_TAG_WITH_ATTS;
                    }
                    _ => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                // ptr points to whitespace after closing quote
                loop {
                    ptr += min_bpc;
                    require_char!(enc, ptr, end);
                    match enc.byte_type(buf, ptr) {
                        BT_NONASCII => {
                            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr += min_bpc;
                            break;
                        }
                        BT_NMSTRT | BT_HEX => {
                            ptr += min_bpc;
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
                            *next_tok_ptr = ptr + min_bpc;
                            return XML_TOK_START_TAG_WITH_ATTS;
                        }
                        BT_SOL => {
                            ptr += min_bpc;
                            require_char!(enc, ptr, end);
                            if !enc.char_matches(buf, ptr, ASCII_GT) {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *next_tok_ptr = ptr + min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min_bpc;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        BT_EXCL => {
            ptr += min_bpc;
            require_char!(enc, ptr, end);
            match enc.byte_type(buf, ptr) {
                BT_MINUS => return scan_comment(enc, buf, ptr + min_bpc, end, next_tok_ptr),
                BT_LSQB => return scan_cdata_section(enc, buf, ptr + min_bpc, end, next_tok_ptr),
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
        }
        BT_QUEST => return scan_pi(enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_SOL => return scan_end_tag(enc, buf, ptr + min_bpc, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_S | BT_CR | BT_LF => {
                ptr += min_bpc;
                while has_char(enc, ptr, end) {
                    match enc.byte_type(buf, ptr) {
                        BT_NONASCII => {
                            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr += min_bpc;
                        }
                        BT_NMSTRT | BT_HEX => {
                            ptr += min_bpc;
                        }
                        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
                        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
                        BT_GT => {
                            *next_tok_ptr = ptr + min_bpc;
                            return XML_TOK_START_TAG_NO_ATTS;
                        }
                        BT_SOL => {
                            ptr += min_bpc;
                            require_char!(enc, ptr, end);
                            if !enc.char_matches(buf, ptr, ASCII_GT) {
                                *next_tok_ptr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *next_tok_ptr = ptr + min_bpc;
                            return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                        }
                        BT_S | BT_CR | BT_LF => {
                            ptr += min_bpc;
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
                *next_tok_ptr = ptr + min_bpc;
                return XML_TOK_START_TAG_NO_ATTS;
            }
            BT_SOL => {
                ptr += min_bpc;
                require_char!(enc, ptr, end);
                if !enc.char_matches(buf, ptr, ASCII_GT) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min_bpc;
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

fn content_tok(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    end = end.min(buf.len());
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min_bpc = enc.min_bpc();
    if min_bpc > 1 {
        let mut n = end - ptr;
        if (n & (min_bpc - 1)) != 0 {
            n &= !(min_bpc - 1);
            if n == 0 {
                return XML_TOK_PARTIAL;
            }
            end = ptr + n;
        }
    }
    match enc.byte_type(buf, ptr) {
        BT_LT => return scan_lt(enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_AMP => return scan_ref(enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_CR => {
            ptr += min_bpc;
            if !has_char(enc, ptr, end) {
                return XML_TOK_TRAILING_CR;
            }
            if enc.byte_type(buf, ptr) == BT_LF {
                ptr += min_bpc;
            }
            *next_tok_ptr = ptr;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_LF => {
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_DATA_NEWLINE;
        }
        BT_RSQB => {
            ptr += min_bpc;
            if !has_char(enc, ptr, end) {
                return XML_TOK_TRAILING_RSQB;
            }
            if enc.char_matches(buf, ptr, ASCII_RSQB) {
                ptr += min_bpc;
                if !has_char(enc, ptr, end) {
                    return XML_TOK_TRAILING_RSQB;
                }
                if !enc.char_matches(buf, ptr, ASCII_GT) {
                    ptr -= min_bpc;
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
            ptr += min_bpc;
        }
    }
    while has_char(enc, ptr, end) {
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
                if has_chars(enc, ptr, end, 2) {
                    if !enc.char_matches(buf, ptr + min_bpc, ASCII_RSQB) {
                        ptr += min_bpc;
                        continue;
                    }
                    if has_chars(enc, ptr, end, 3) {
                        if !enc.char_matches(buf, ptr + 2 * min_bpc, ASCII_GT) {
                            ptr += min_bpc;
                            continue;
                        }
                        *next_tok_ptr = ptr + 2 * min_bpc;
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
                ptr += min_bpc;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn scan_percent(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min_bpc;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
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
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_SEMI => {
                *next_tok_ptr = ptr + min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    require_char!(enc, ptr, end);
    match enc.byte_type(buf, ptr) {
        BT_NONASCII => {
            if !enc.is_nmstrt_char_minbpc(buf, ptr) {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            ptr += min_bpc;
        }
        BT_NMSTRT | BT_HEX => {
            ptr += min_bpc;
        }
        BT_LEAD2 => check_nmstrt_case!(2, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD3 => check_nmstrt_case!(3, enc, buf, ptr, end, next_tok_ptr),
        BT_LEAD4 => check_nmstrt_case!(4, enc, buf, ptr, end, next_tok_ptr),
        _ => {
            *next_tok_ptr = ptr;
            return XML_TOK_INVALID;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    let min_bpc = enc.min_bpc();
    while has_char(enc, ptr, end) {
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
                ptr += min_bpc;
                if t != open {
                    continue;
                }
                if !has_char(enc, ptr, end) {
                    return -XML_TOK_LITERAL;
                }
                *next_tok_ptr = ptr;
                match enc.byte_type(buf, ptr) {
                    BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => return XML_TOK_LITERAL,
                    _ => return XML_TOK_INVALID,
                }
            }
            _ => {
                ptr += min_bpc;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn prolog_tok(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    end = end.min(buf.len());
    if ptr >= end {
        return XML_TOK_NONE;
    }
    let min_bpc = enc.min_bpc();
    if min_bpc > 1 {
        let mut n = end - ptr;
        if (n & (min_bpc - 1)) != 0 {
            n &= !(min_bpc - 1);
            if n == 0 {
                return XML_TOK_PARTIAL;
            }
            end = ptr + n;
        }
    }
    let tok: i32;
    match enc.byte_type(buf, ptr) {
        BT_QUOT => return scan_lit(BT_QUOT, enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_APOS => return scan_lit(BT_APOS, enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_LT => {
            ptr += min_bpc;
            require_char!(enc, ptr, end);
            match enc.byte_type(buf, ptr) {
                BT_EXCL => return scan_decl(enc, buf, ptr + min_bpc, end, next_tok_ptr),
                BT_QUEST => return scan_pi(enc, buf, ptr + min_bpc, end, next_tok_ptr),
                BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                    *next_tok_ptr = ptr - min_bpc;
                    return XML_TOK_INSTANCE_START;
                }
                _ => {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
            }
        }
        BT_CR if ptr + min_bpc == end => {
            *next_tok_ptr = end;
            return -XML_TOK_PROLOG_S;
        }
        BT_CR | BT_S | BT_LF => {
            loop {
                ptr += min_bpc;
                if !has_char(enc, ptr, end) {
                    break;
                }
                match enc.byte_type(buf, ptr) {
                    BT_S | BT_LF => {}
                    BT_CR if ptr + min_bpc != end => {}
                    _ => {
                        *next_tok_ptr = ptr;
                        return XML_TOK_PROLOG_S;
                    }
                }
            }
            *next_tok_ptr = ptr;
            return XML_TOK_PROLOG_S;
        }
        BT_PERCNT => return scan_percent(enc, buf, ptr + min_bpc, end, next_tok_ptr),
        BT_COMMA => {
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_COMMA;
        }
        BT_LSQB => {
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_OPEN_BRACKET;
        }
        BT_RSQB => {
            ptr += min_bpc;
            if !has_char(enc, ptr, end) {
                return -XML_TOK_CLOSE_BRACKET;
            }
            if enc.char_matches(buf, ptr, ASCII_RSQB) {
                require_chars!(enc, ptr, end, 2);
                if enc.char_matches(buf, ptr + min_bpc, ASCII_GT) {
                    *next_tok_ptr = ptr + 2 * min_bpc;
                    return XML_TOK_COND_SECT_CLOSE;
                }
            }
            *next_tok_ptr = ptr;
            return XML_TOK_CLOSE_BRACKET;
        }
        BT_LPAR => {
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_OPEN_PAREN;
        }
        BT_RPAR => {
            ptr += min_bpc;
            if !has_char(enc, ptr, end) {
                return -XML_TOK_CLOSE_PAREN;
            }
            match enc.byte_type(buf, ptr) {
                BT_AST => {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_CLOSE_PAREN_ASTERISK;
                }
                BT_QUEST => {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_CLOSE_PAREN_QUESTION;
                }
                BT_PLUS => {
                    *next_tok_ptr = ptr + min_bpc;
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
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_OR;
        }
        BT_GT => {
            *next_tok_ptr = ptr + min_bpc;
            return XML_TOK_DECL_CLOSE;
        }
        BT_NUM => return scan_pound_name(enc, buf, ptr + min_bpc, end, next_tok_ptr),
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
            ptr += min_bpc;
        }
        BT_DIGIT | BT_NAME | BT_MINUS => {
            tok = XML_TOK_NMTOKEN;
            ptr += min_bpc;
        }
        BT_NONASCII => {
            if enc.is_nmstrt_char_minbpc(buf, ptr) {
                ptr += min_bpc;
                tok = XML_TOK_NAME;
            } else if enc.is_name_char_minbpc(buf, ptr) {
                ptr += min_bpc;
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
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_NONASCII => {
                if !enc.is_name_char_minbpc(buf, ptr) {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr += min_bpc;
            }
            BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            BT_LEAD2 => check_name_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => check_name_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => check_name_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR | BT_LF => {
                *next_tok_ptr = ptr;
                return tok;
            }
            BT_PLUS => {
                if tok == XML_TOK_NMTOKEN {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min_bpc;
                return XML_TOK_NAME_PLUS;
            }
            BT_AST => {
                if tok == XML_TOK_NMTOKEN {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min_bpc;
                return XML_TOK_NAME_ASTERISK;
            }
            BT_QUEST => {
                if tok == XML_TOK_NMTOKEN {
                    *next_tok_ptr = ptr;
                    return XML_TOK_INVALID;
                }
                *next_tok_ptr = ptr + min_bpc;
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
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    end = end.min(buf.len());
    if ptr >= end {
        return XML_TOK_NONE;
    } else if !has_char(enc, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    let min_bpc = enc.min_bpc();
    let start = ptr;
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => ptr += 2,
            BT_LEAD3 => ptr += 3,
            BT_LEAD4 => ptr += 4,
            BT_AMP => {
                if ptr == start {
                    return scan_ref(enc, buf, ptr + min_bpc, end, next_tok_ptr);
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
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_CR => {
                if ptr == start {
                    ptr += min_bpc;
                    if !has_char(enc, ptr, end) {
                        return XML_TOK_TRAILING_CR;
                    }
                    if enc.byte_type(buf, ptr) == BT_LF {
                        ptr += min_bpc;
                    }
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_S => {
                if ptr == start {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_ATTRIBUTE_VALUE_S;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr += min_bpc;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn entity_value_tok(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    end = end.min(buf.len());
    if ptr >= end {
        return XML_TOK_NONE;
    } else if !has_char(enc, ptr, end) {
        return XML_TOK_PARTIAL;
    }
    let min_bpc = enc.min_bpc();
    let start = ptr;
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => ptr += 2,
            BT_LEAD3 => ptr += 3,
            BT_LEAD4 => ptr += 4,
            BT_AMP => {
                if ptr == start {
                    return scan_ref(enc, buf, ptr + min_bpc, end, next_tok_ptr);
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_PERCNT => {
                if ptr == start {
                    let tok = scan_percent(enc, buf, ptr + min_bpc, end, next_tok_ptr);
                    return if tok == XML_TOK_PERCENT { XML_TOK_INVALID } else { tok };
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_LF => {
                if ptr == start {
                    *next_tok_ptr = ptr + min_bpc;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            BT_CR => {
                if ptr == start {
                    ptr += min_bpc;
                    if !has_char(enc, ptr, end) {
                        return XML_TOK_TRAILING_CR;
                    }
                    if enc.byte_type(buf, ptr) == BT_LF {
                        ptr += min_bpc;
                    }
                    *next_tok_ptr = ptr;
                    return XML_TOK_DATA_NEWLINE;
                }
                *next_tok_ptr = ptr;
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr += min_bpc;
            }
        }
    }
    *next_tok_ptr = ptr;
    XML_TOK_DATA_CHARS
}

fn ignore_section_tok(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    end = end.min(buf.len());
    let mut level = 0;
    let min_bpc = enc.min_bpc();
    if min_bpc > 1 {
        let mut n = end.saturating_sub(ptr);
        if (n & (min_bpc - 1)) != 0 {
            n &= !(min_bpc - 1);
            end = ptr + n;
        }
    }
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => invalid_lead_case!(2, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD3 => invalid_lead_case!(3, enc, buf, ptr, end, next_tok_ptr),
            BT_LEAD4 => invalid_lead_case!(4, enc, buf, ptr, end, next_tok_ptr),
            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                *next_tok_ptr = ptr;
                return XML_TOK_INVALID;
            }
            BT_LT => {
                ptr += min_bpc;
                require_char!(enc, ptr, end);
                if enc.char_matches(buf, ptr, ASCII_EXCL) {
                    ptr += min_bpc;
                    require_char!(enc, ptr, end);
                    if enc.char_matches(buf, ptr, ASCII_LSQB) {
                        level += 1;
                        ptr += min_bpc;
                    }
                }
            }
            BT_RSQB => {
                ptr += min_bpc;
                require_char!(enc, ptr, end);
                if enc.char_matches(buf, ptr, ASCII_RSQB) {
                    ptr += min_bpc;
                    require_char!(enc, ptr, end);
                    if enc.char_matches(buf, ptr, ASCII_GT) {
                        ptr += min_bpc;
                        if level == 0 {
                            *next_tok_ptr = ptr;
                            return XML_TOK_IGNORE_SECT;
                        }
                        level -= 1;
                    }
                }
            }
            _ => {
                ptr += min_bpc;
            }
        }
    }
    XML_TOK_PARTIAL
}

fn is_public_id(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    bad_ptr: &mut usize,
) -> bool {
    let min_bpc = enc.min_bpc();
    ptr += min_bpc;
    end -= min_bpc;
    while has_char(enc, ptr, end) {
        match enc.byte_type(buf, ptr) {
            BT_DIGIT | BT_HEX | BT_MINUS | BT_APOS | BT_LPAR | BT_RPAR | BT_PLUS | BT_COMMA
            | BT_SOL | BT_EQUALS | BT_QUEST | BT_CR | BT_LF | BT_SEMI | BT_EXCL | BT_AST
            | BT_PERCNT | BT_NUM => {}
            BT_S => {
                if enc.char_matches(buf, ptr, ASCII_TAB) {
                    *bad_ptr = ptr;
                    return false;
                }
            }
            BT_NAME | BT_NMSTRT => {
                if (enc.byte_to_ascii(buf, ptr) & !0x7f) == 0 {
                    ptr += min_bpc;
                    continue;
                }
                match enc.byte_to_ascii(buf, ptr) {
                    0x24 | 0x40 => {}
                    _ => {
                        *bad_ptr = ptr;
                        return false;
                    }
                }
            }
            _ => match enc.byte_to_ascii(buf, ptr) {
                0x24 | 0x40 => {}
                _ => {
                    *bad_ptr = ptr;
                    return false;
                }
            },
        }
        ptr += min_bpc;
    }
    true
}

#[derive(Copy, Clone, Debug)]
struct RawAttribute {
    name: usize,
    value_ptr: usize,
    value_end: usize,
    normalized: bool,
}

fn get_atts(enc: EncId, buf: &[u8], mut ptr: usize, atts: &mut Vec<RawAttribute>) {
    atts.clear();
    #[derive(PartialEq, Eq)]
    enum State {
        Other,
        InName,
        InValue,
    }
    let mut state = State::InName;
    let mut open: u8 = 0;
    let min_bpc = enc.min_bpc();
    let mut cur_name = 0;
    let mut cur_val_ptr = 0;
    let mut cur_norm = true;

    ptr += min_bpc;
    loop {
        if ptr + min_bpc > buf.len() {
            return;
        }
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => {
                if state == State::Other {
                    cur_name = ptr;
                    cur_norm = true;
                    state = State::InName;
                }
                ptr += 2 - min_bpc;
            }
            BT_LEAD3 => {
                if state == State::Other {
                    cur_name = ptr;
                    cur_norm = true;
                    state = State::InName;
                }
                ptr += 3 - min_bpc;
            }
            BT_LEAD4 => {
                if state == State::Other {
                    cur_name = ptr;
                    cur_norm = true;
                    state = State::InName;
                }
                ptr += 4 - min_bpc;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX => {
                if state == State::Other {
                    cur_name = ptr;
                    cur_norm = true;
                    state = State::InName;
                }
            }
            BT_QUOT => {
                if state != State::InValue {
                    cur_val_ptr = ptr + min_bpc;
                    state = State::InValue;
                    open = BT_QUOT;
                } else if open == BT_QUOT {
                    state = State::Other;
                    atts.push(RawAttribute {
                        name: cur_name,
                        value_ptr: cur_val_ptr,
                        value_end: ptr,
                        normalized: cur_norm,
                    });
                }
            }
            BT_APOS => {
                if state != State::InValue {
                    cur_val_ptr = ptr + min_bpc;
                    state = State::InValue;
                    open = BT_APOS;
                } else if open == BT_APOS {
                    state = State::Other;
                    atts.push(RawAttribute {
                        name: cur_name,
                        value_ptr: cur_val_ptr,
                        value_end: ptr,
                        normalized: cur_norm,
                    });
                }
            }
            BT_AMP => {
                cur_norm = false;
            }
            BT_S => {
                if state == State::InName {
                    state = State::Other;
                } else if state == State::InValue
                    && cur_norm
                    && (ptr == cur_val_ptr
                        || enc.byte_to_ascii(buf, ptr) != ASCII_SPACE as i32
                        || enc.byte_to_ascii(buf, ptr + min_bpc) == ASCII_SPACE as i32
                        || enc.byte_type(buf, ptr + min_bpc) == open)
                {
                    cur_norm = false;
                }
            }
            BT_CR | BT_LF => {
                if state == State::InName {
                    state = State::Other;
                } else if state == State::InValue {
                    cur_norm = false;
                }
            }
            BT_GT | BT_SOL => {
                if state != State::InValue {
                    return;
                }
            }
            _ => {}
        }
        ptr += min_bpc;
    }
}

fn check_char_ref_number(result: i32) -> i32 {
    if result < 0 {
        return -1;
    }
    match result >> 8 {
        0xD8..=0xDF => return -1,
        0 => {
            if LATIN1_TABLE[result as usize] == BT_NONXML {
                return -1;
            }
        }
        0xFF => {
            if result == 0xFFFE || result == 0xFFFF {
                return -1;
            }
        }
        _ => {}
    }
    result
}

fn char_ref_number(enc: EncId, buf: &[u8], mut ptr: usize) -> i32 {
    let min_bpc = enc.min_bpc();
    let mut result: i32 = 0;
    ptr += 2 * min_bpc;
    if enc.char_matches(buf, ptr, ASCII_LOWER_X) {
        ptr += min_bpc;
        while !enc.char_matches(buf, ptr, ASCII_SEMI) {
            let c = enc.byte_to_ascii(buf, ptr);
            if c >= ASCII_0 as i32 && c <= ASCII_9 as i32 {
                result <<= 4;
                result |= c - ASCII_0 as i32;
            } else if c >= ASCII_A as i32 && c <= ASCII_F as i32 {
                result <<= 4;
                result += 10 + (c - ASCII_A as i32);
            } else if c >= ASCII_LOWER_A as i32 && c <= ASCII_LOWER_F as i32 {
                result <<= 4;
                result += 10 + (c - ASCII_LOWER_A as i32);
            }
            if result >= 0x110000 {
                return -1;
            }
            ptr += min_bpc;
        }
    } else {
        while !enc.char_matches(buf, ptr, ASCII_SEMI) {
            let c = enc.byte_to_ascii(buf, ptr);
            result *= 10;
            result += c - ASCII_0 as i32;
            if result >= 0x110000 {
                return -1;
            }
            ptr += min_bpc;
        }
    }
    check_char_ref_number(result)
}

fn predefined_entity_name(enc: EncId, buf: &[u8], mut ptr: usize, end: usize) -> u8 {
    let min_bpc = enc.min_bpc();
    if end < ptr {
        return 0;
    }
    match (end - ptr) / min_bpc {
        2 => {
            if enc.char_matches(buf, ptr + min_bpc, ASCII_LOWER_T) {
                match enc.byte_to_ascii(buf, ptr) {
                    x if x == ASCII_LOWER_L as i32 => return ASCII_LT,
                    x if x == ASCII_LOWER_G as i32 => return ASCII_GT,
                    _ => {}
                }
            }
        }
        3 => {
            if enc.char_matches(buf, ptr, ASCII_LOWER_A) {
                ptr += min_bpc;
                if enc.char_matches(buf, ptr, ASCII_LOWER_M) {
                    ptr += min_bpc;
                    if enc.char_matches(buf, ptr, ASCII_LOWER_P) {
                        return ASCII_AMP;
                    }
                }
            }
        }
        4 => match enc.byte_to_ascii(buf, ptr) {
            x if x == ASCII_LOWER_Q as i32 => {
                ptr += min_bpc;
                if enc.char_matches(buf, ptr, ASCII_LOWER_U) {
                    ptr += min_bpc;
                    if enc.char_matches(buf, ptr, ASCII_LOWER_O) {
                        ptr += min_bpc;
                        if enc.char_matches(buf, ptr, ASCII_LOWER_T) {
                            return ASCII_QUOT;
                        }
                    }
                }
            }
            x if x == ASCII_LOWER_A as i32 => {
                ptr += min_bpc;
                if enc.char_matches(buf, ptr, ASCII_LOWER_P) {
                    ptr += min_bpc;
                    if enc.char_matches(buf, ptr, ASCII_LOWER_O) {
                        ptr += min_bpc;
                        if enc.char_matches(buf, ptr, ASCII_LOWER_S) {
                            return ASCII_APOS;
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

fn name_matches_ascii(enc: EncId, buf: &[u8], mut ptr1: usize, end1: usize, kw: &[u8]) -> bool {
    let min_bpc = enc.min_bpc();
    for &c in kw {
        if end1 < ptr1 || end1 - ptr1 < min_bpc {
            return false;
        }
        if !enc.char_matches(buf, ptr1, c) {
            return false;
        }
        ptr1 += min_bpc;
    }
    ptr1 == end1
}

fn name_length(enc: EncId, buf: &[u8], mut ptr: usize) -> usize {
    let start = ptr;
    let min_bpc = enc.min_bpc();
    loop {
        if ptr + min_bpc > buf.len() {
            return ptr - start;
        }
        match enc.byte_type(buf, ptr) {
            BT_LEAD2 => ptr += 2,
            BT_LEAD3 => ptr += 3,
            BT_LEAD4 => ptr += 4,
            BT_NONASCII | BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                ptr += min_bpc;
            }
            _ => return ptr - start,
        }
    }
}

fn skip_s(enc: EncId, buf: &[u8], mut ptr: usize) -> usize {
    let min_bpc = enc.min_bpc();
    loop {
        if ptr + min_bpc > buf.len() {
            return ptr;
        }
        match enc.byte_type(buf, ptr) {
            BT_LF | BT_CR | BT_S => ptr += min_bpc,
            _ => return ptr,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
struct Position {
    line_number: u64,
    column_number: u64,
}

fn update_position(enc: EncId, buf: &[u8], mut ptr: usize, mut end: usize, pos: &mut Position) {
    let actual_enc = if enc == EncId::Init { EncId::Utf8 } else { enc };
    end = end.min(buf.len());
    let min_bpc = actual_enc.min_bpc();
    while has_char(actual_enc, ptr, end) {
        match actual_enc.byte_type(buf, ptr) {
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
                ptr += min_bpc;
            }
            BT_CR => {
                pos.line_number += 1;
                ptr += min_bpc;
                if has_char(actual_enc, ptr, end) && actual_enc.byte_type(buf, ptr) == BT_LF {
                    ptr += min_bpc;
                }
                pos.column_number = 0;
            }
            _ => {
                ptr += min_bpc;
                pos.column_number += 1;
            }
        }
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
        buf[0] = ((c >> 6) | 0xC0) as u8;
        buf[1] = ((c & 0x3F) | 0x80) as u8;
        2
    } else if c < 0x10000 {
        buf[0] = ((c >> 12) | 0xE0) as u8;
        buf[1] = (((c >> 6) & 0x3F) | 0x80) as u8;
        buf[2] = ((c & 0x3F) | 0x80) as u8;
        3
    } else if c < 0x110000 {
        buf[0] = ((c >> 18) | 0xF0) as u8;
        buf[1] = (((c >> 12) & 0x3F) | 0x80) as u8;
        buf[2] = (((c >> 6) & 0x3F) | 0x80) as u8;
        buf[3] = ((c & 0x3F) | 0x80) as u8;
        4
    } else {
        0
    }
}

fn trim_to_complete_utf8_characters(from: usize, from_lim_ref: &mut usize, buf: &[u8]) {
    let mut from_lim = *from_lim_ref;
    let mut walked: usize = 0;
    while from_lim > from {
        let prev = buf[from_lim - 1];
        if (prev & 0xf8) == 0xf0 {
            if walked + 1 >= 4 {
                from_lim += 4 - 1;
                break;
            } else {
                walked = 0;
            }
        } else if (prev & 0xf0) == 0xe0 {
            if walked + 1 >= 3 {
                from_lim += 3 - 1;
                break;
            } else {
                walked = 0;
            }
        } else if (prev & 0xe0) == 0xc0 {
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
    *from_lim_ref = from_lim;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ConvertResult {
    Completed,
    InputIncomplete,
    OutputExhausted,
}

fn utf8_convert_bounded(
    enc: EncId,
    buf: &[u8],
    from_p: &mut usize,
    mut from_lim: usize,
    out: &mut [u8],
    to_p: &mut usize,
) -> ConvertResult {
    let to_lim = out.len();
    match enc {
        EncId::Utf8 | EncId::InternalUtf8 | EncId::Init => {
            let mut input_incomplete = false;
            let mut output_exhausted = false;
            let bytes_available = from_lim.saturating_sub(*from_p);
            let bytes_storable = to_lim.saturating_sub(*to_p);
            if bytes_available > bytes_storable {
                from_lim = *from_p + bytes_storable;
                output_exhausted = true;
            }
            let from_lim_before = from_lim;
            trim_to_complete_utf8_characters(*from_p, &mut from_lim, buf);
            if from_lim < from_lim_before {
                input_incomplete = true;
            }
            let bytes_to_copy = from_lim - *from_p;
            out[*to_p..*to_p + bytes_to_copy].copy_from_slice(&buf[*from_p..from_lim]);
            *from_p += bytes_to_copy;
            *to_p += bytes_to_copy;
            if output_exhausted {
                ConvertResult::OutputExhausted
            } else if input_incomplete {
                ConvertResult::InputIncomplete
            } else {
                ConvertResult::Completed
            }
        }
        EncId::Ascii => {
            while *from_p < from_lim && *to_p < to_lim {
                out[*to_p] = buf[*from_p];
                *from_p += 1;
                *to_p += 1;
            }
            if *to_p == to_lim && *from_p < from_lim {
                ConvertResult::OutputExhausted
            } else {
                ConvertResult::Completed
            }
        }
        EncId::Latin1 => {
            loop {
                if *from_p == from_lim {
                    return ConvertResult::Completed;
                }
                let c = buf[*from_p];
                if (c & 0x80) != 0 {
                    if to_lim - *to_p < 2 {
                        return ConvertResult::OutputExhausted;
                    }
                    out[*to_p] = (c >> 6) | 0xC0;
                    out[*to_p + 1] = (c & 0x3F) | 0x80;
                    *to_p += 2;
                    *from_p += 1;
                } else {
                    if *to_p == to_lim {
                        return ConvertResult::OutputExhausted;
                    }
                    out[*to_p] = c;
                    *to_p += 1;
                    *from_p += 1;
                }
            }
        }
        EncId::Utf16Le | EncId::Utf16Be => {
            let is_le = enc == EncId::Utf16Le;
            let mut from = *from_p;
            from_lim = from + (((from_lim - from) >> 1) << 1);
            while from < from_lim {
                let (lo, hi) = if is_le {
                    (buf[from], buf[from + 1])
                } else {
                    (buf[from + 1], buf[from])
                };
                match hi {
                    0 if lo < 0x80 => {
                        if *to_p == to_lim {
                            *from_p = from;
                            return ConvertResult::OutputExhausted;
                        }
                        out[*to_p] = lo;
                        *to_p += 1;
                    }
                    0x00..=0x07 => {
                        if to_lim - *to_p < 2 {
                            *from_p = from;
                            return ConvertResult::OutputExhausted;
                        }
                        out[*to_p] = (lo >> 6) | (hi << 2) | 0xC0;
                        out[*to_p + 1] = (lo & 0x3F) | 0x80;
                        *to_p += 2;
                    }
                    0xD8..=0xDB => {
                        if to_lim - *to_p < 4 {
                            *from_p = from;
                            return ConvertResult::OutputExhausted;
                        }
                        if from_lim - from < 4 {
                            *from_p = from;
                            return ConvertResult::InputIncomplete;
                        }
                        let plane = (((hi & 0x3) << 2) | ((lo >> 6) & 0x3)) + 1;
                        out[*to_p] = (plane >> 2) | 0xF0;
                        out[*to_p + 1] = ((lo >> 2) & 0xF) | ((plane & 0x3) << 4) | 0x80;
                        from += 2;
                        let (lo2, hi2) = if is_le {
                            (buf[from], buf[from + 1])
                        } else {
                            (buf[from + 1], buf[from])
                        };
                        out[*to_p + 2] = ((lo & 0x3) << 4) | ((hi2 & 0x3) << 2) | (lo2 >> 6) | 0x80;
                        out[*to_p + 3] = (lo2 & 0x3F) | 0x80;
                        *to_p += 4;
                    }
                    _ => {
                        if to_lim - *to_p < 3 {
                            *from_p = from;
                            return ConvertResult::OutputExhausted;
                        }
                        out[*to_p] = (hi >> 4) | 0xE0;
                        out[*to_p + 1] = ((hi & 0xF) << 2) | (lo >> 6) | 0x80;
                        out[*to_p + 2] = (lo & 0x3F) | 0x80;
                        *to_p += 3;
                    }
                }
                from += 2;
            }
            *from_p = from;
            if from < from_lim {
                ConvertResult::InputIncomplete
            } else {
                ConvertResult::Completed
            }
        }
    }
}

fn append_converted_to_vec(enc: EncId, buf: &[u8], start: usize, end: usize, dst: &mut Vec<u8>) {
    if enc.is_utf8() {
        let mut lim = end;
        trim_to_complete_utf8_characters(start, &mut lim, buf);
        dst.extend_from_slice(&buf[start..lim]);
        return;
    }
    let mut from_p = start;
    let mut tmp = [0u8; 1024];
    loop {
        let mut to_p = 0;
        let res = utf8_convert_bounded(enc, buf, &mut from_p, end, &mut tmp, &mut to_p);
        dst.extend_from_slice(&tmp[..to_p]);
        if res == ConvertResult::Completed || res == ConvertResult::InputIncomplete {
            break;
        }
    }
}

fn store_string_to_vec(enc: EncId, buf: &[u8], start: usize, end: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(end.saturating_sub(start));
    append_converted_to_vec(enc, buf, start, end, &mut v);
    v
}

fn to_ascii(enc: EncId, buf: &[u8], mut ptr: usize, end: usize) -> i32 {
    let mut out = [0u8; 1];
    let mut to_p = 0;
    utf8_convert_bounded(enc, buf, &mut ptr, end, &mut out, &mut to_p);
    if to_p == 0 {
        -1
    } else {
        out[0] as i32
    }
}

#[inline]
fn is_space(c: i32) -> bool {
    matches!(c, 0x20 | 0x0D | 0x0A | 0x09)
}

fn streqci(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    for (&b1, &b2) in s1.iter().zip(s2.iter()) {
        let c1 = if b1 >= ASCII_LOWER_A && b1 <= ASCII_LOWER_Z {
            b1 - (ASCII_LOWER_A - ASCII_A)
        } else {
            b1
        };
        let c2 = if b2 >= ASCII_LOWER_A && b2 <= ASCII_LOWER_Z {
            b2 - (ASCII_LOWER_A - ASCII_A)
        } else {
            b2
        };
        if c1 != c2 {
            return false;
        }
    }
    true
}

fn get_encoding_by_name(name: &[u8]) -> Option<EncId> {
    if streqci(name, b"ISO-8859-1") {
        Some(EncId::Latin1)
    } else if streqci(name, b"US-ASCII") {
        Some(EncId::Ascii)
    } else if streqci(name, b"UTF-8") {
        Some(EncId::Utf8)
    } else if streqci(name, b"UTF-16") {
        Some(EncId::Utf16Be)
    } else if streqci(name, b"UTF-16BE") {
        Some(EncId::Utf16Be)
    } else if streqci(name, b"UTF-16LE") {
        Some(EncId::Utf16Le)
    } else {
        None
    }
}

fn find_encoding(enc: EncId, buf: &[u8], mut ptr: usize, end: usize) -> Option<EncId> {
    const ENCODING_MAX: usize = 128;
    let mut out = [0u8; ENCODING_MAX - 1];
    let mut to_p = 0;
    utf8_convert_bounded(enc, buf, &mut ptr, end, &mut out, &mut to_p);
    if ptr != end {
        return None;
    }
    let name = &out[..to_p];
    if streqci(name, b"UTF-16") && enc.min_bpc() == 2 {
        return Some(enc);
    }
    get_encoding_by_name(name)
}

fn parse_pseudo_attribute(
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    end: usize,
    name_ptr: &mut Option<usize>,
    name_end_ptr: &mut usize,
    val_ptr: &mut usize,
    next_tok_ptr: &mut usize,
) -> bool {
    let min_bpc = enc.min_bpc();
    if ptr == end {
        *name_ptr = None;
        return true;
    }
    if !is_space(to_ascii(enc, buf, ptr, end)) {
        *next_tok_ptr = ptr;
        return false;
    }
    loop {
        ptr += min_bpc;
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
        if c == ASCII_EQUALS as i32 {
            *name_end_ptr = ptr;
            break;
        }
        if is_space(c) {
            *name_end_ptr = ptr;
            loop {
                ptr += min_bpc;
                c = to_ascii(enc, buf, ptr, end);
                if !is_space(c) {
                    break;
                }
            }
            if c != ASCII_EQUALS as i32 {
                *next_tok_ptr = ptr;
                return false;
            }
            break;
        }
        ptr += min_bpc;
    }
    if ptr == name_start {
        *next_tok_ptr = ptr;
        return false;
    }
    ptr += min_bpc;
    let mut c = to_ascii(enc, buf, ptr, end);
    while is_space(c) {
        ptr += min_bpc;
        c = to_ascii(enc, buf, ptr, end);
    }
    if c != ASCII_QUOT as i32 && c != ASCII_APOS as i32 {
        *next_tok_ptr = ptr;
        return false;
    }
    let open = c;
    ptr += min_bpc;
    *val_ptr = ptr;
    loop {
        c = to_ascii(enc, buf, ptr, end);
        if c == open {
            break;
        }
        let cu = c as u8;
        if !(c >= ASCII_LOWER_A as i32 && c <= ASCII_LOWER_Z as i32)
            && !(c >= ASCII_A as i32 && c <= ASCII_Z as i32)
            && !(c >= ASCII_0 as i32 && c <= ASCII_9 as i32)
            && cu != ASCII_PERIOD
            && cu != ASCII_MINUS
            && cu != ASCII_UNDERSCORE
        {
            *next_tok_ptr = ptr;
            return false;
        }
        ptr += min_bpc;
    }
    *next_tok_ptr = ptr + min_bpc;
    true
}

fn check_xml_decl_version_num(enc: EncId, buf: &[u8], mut val: usize, val_end: usize) -> bool {
    let min_bpc = enc.min_bpc();
    if val_end < val
        || val_end - val < 2 * min_bpc
        || !name_matches_ascii(enc, buf, val, val + 2 * min_bpc, b"1.")
    {
        return false;
    }
    val += 2 * min_bpc;
    if val == val_end {
        return false;
    }
    while val != val_end {
        let c = to_ascii(enc, buf, val, val_end);
        if c < ASCII_0 as i32 || c > ASCII_9 as i32 {
            return false;
        }
        val += min_bpc;
    }
    true
}

fn do_parse_xml_decl(
    is_general_text_entity: bool,
    enc: EncId,
    buf: &[u8],
    mut ptr: usize,
    mut end: usize,
    bad_ptr: &mut Option<usize>,
    version_ptr: &mut Option<(usize, usize)>,
    encoding_name: &mut Option<usize>,
    encoding: &mut Option<EncId>,
    standalone: &mut i32,
) -> bool {
    let min_bpc = enc.min_bpc();
    ptr += 5 * min_bpc;
    end -= 2 * min_bpc;
    let mut name: Option<usize> = None;
    let mut name_end: usize = 0;
    let mut val: usize = 0;
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
    let mut name_pos = name.unwrap_or(0);
    if !name_matches_ascii(enc, buf, name_pos, name_end, b"version") {
        if !is_general_text_entity {
            *bad_ptr = Some(name_pos);
            return false;
        }
    } else {
        *version_ptr = Some((val, ptr));
        if val == ptr - min_bpc || !check_xml_decl_version_num(enc, buf, val, ptr - min_bpc) {
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
        let Some(np) = name else {
            if is_general_text_entity {
                *bad_ptr = Some(ptr);
                return false;
            }
            return true;
        };
        name_pos = np;
    }
    if name_matches_ascii(enc, buf, name_pos, name_end, b"encoding") {
        let c = to_ascii(enc, buf, val, end);
        if !(c >= ASCII_LOWER_A as i32 && c <= ASCII_LOWER_Z as i32)
            && !(c >= ASCII_A as i32 && c <= ASCII_Z as i32)
        {
            *bad_ptr = Some(val);
            return false;
        }
        *encoding_name = Some(val);
        *encoding = find_encoding(enc, buf, val, ptr - min_bpc);
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
        let Some(np) = name else {
            return true;
        };
        name_pos = np;
    }
    if !name_matches_ascii(enc, buf, name_pos, name_end, b"standalone") || is_general_text_entity {
        *bad_ptr = Some(name_pos);
        return false;
    }
    if name_matches_ascii(enc, buf, val, ptr - min_bpc, b"yes") {
        *standalone = 1;
    } else if name_matches_ascii(enc, buf, val, ptr - min_bpc, b"no") {
        *standalone = 0;
    } else {
        *bad_ptr = Some(val);
        return false;
    }
    while is_space(to_ascii(enc, buf, ptr, end)) {
        ptr += min_bpc;
    }
    if ptr != end {
        *bad_ptr = Some(ptr);
        return false;
    }
    true
}

fn init_scan(
    enc_ptr: &mut EncId,
    state: usize,
    buf: &[u8],
    ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if ptr >= end {
        return XML_TOK_NONE;
    }
    if ptr + 1 == end {
        // XML_DTD is defined and INIT_ENC_INDEX is NO_ENC
        match buf[ptr] {
            0xFE | 0xFF | 0xEF | 0x00 | 0x3C => return XML_TOK_PARTIAL,
            _ => {}
        }
    } else {
        let first_two = ((buf[ptr] as u16) << 8) | (buf[ptr + 1] as u16);
        match first_two {
            0xFEFF => {
                *next_tok_ptr = ptr + 2;
                *enc_ptr = EncId::Utf16Be;
                return XML_TOK_BOM;
            }
            0x3C00 => {
                *enc_ptr = EncId::Utf16Le;
                return xml_tok(*enc_ptr, state, buf, ptr, end, next_tok_ptr);
            }
            0xFFFE => {
                *next_tok_ptr = ptr + 2;
                *enc_ptr = EncId::Utf16Le;
                return XML_TOK_BOM;
            }
            0xEFBB => {
                if ptr + 2 == end {
                    return XML_TOK_PARTIAL;
                }
                if buf[ptr + 2] == 0xBF {
                    *next_tok_ptr = ptr + 3;
                    *enc_ptr = EncId::Utf8;
                    return XML_TOK_BOM;
                }
            }
            _ => {
                if buf[ptr] == 0 {
                    *enc_ptr = EncId::Utf16Be;
                    return xml_tok(*enc_ptr, state, buf, ptr, end, next_tok_ptr);
                } else if buf[ptr + 1] == 0 {
                    if state != XML_CONTENT_STATE {
                        *enc_ptr = EncId::Utf16Le;
                        return xml_tok(*enc_ptr, state, buf, ptr, end, next_tok_ptr);
                    }
                }
            }
        }
    }
    *enc_ptr = EncId::Utf8;
    xml_tok(*enc_ptr, state, buf, ptr, end, next_tok_ptr)
}

fn xml_tok(
    enc: EncId,
    state: usize,
    buf: &[u8],
    ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    match state {
        XML_PROLOG_STATE => prolog_tok(enc, buf, ptr, end, next_tok_ptr),
        XML_CONTENT_STATE => content_tok(enc, buf, ptr, end, next_tok_ptr),
        XML_CDATA_SECTION_STATE => cdata_section_tok(enc, buf, ptr, end, next_tok_ptr),
        XML_IGNORE_SECTION_STATE => ignore_section_tok(enc, buf, ptr, end, next_tok_ptr),
        _ => XML_TOK_INVALID,
    }
}

fn xml_prolog_tok(
    enc: &mut EncId,
    buf: &[u8],
    ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if *enc == EncId::Init {
        init_scan(enc, XML_PROLOG_STATE, buf, ptr, end, next_tok_ptr)
    } else {
        prolog_tok(*enc, buf, ptr, end, next_tok_ptr)
    }
}

fn xml_content_tok(
    enc: &mut EncId,
    buf: &[u8],
    ptr: usize,
    end: usize,
    next_tok_ptr: &mut usize,
) -> i32 {
    if *enc == EncId::Init {
        init_scan(enc, XML_CONTENT_STATE, buf, ptr, end, next_tok_ptr)
    } else {
        content_tok(*enc, buf, ptr, end, next_tok_ptr)
    }
}

// ---- xmlrole.h / xmlrole.c ----

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum RoleHandler {
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
    CondSect0,
    CondSect1,
    CondSect2,
    DeclClose,
    Error,
}

struct PrologState {
    handler: RoleHandler,
    level: usize,
    role_none: i32,
    include_level: u32,
    document_entity: bool,
    in_entity_value: bool,
}

impl PrologState {
    fn new() -> Self {
        Self {
            handler: RoleHandler::Prolog0,
            level: 0,
            role_none: 0,
            include_level: 0,
            document_entity: true,
            in_entity_value: false,
        }
    }

    #[inline]
    fn set_top_level(&mut self) {
        self.handler = if self.document_entity {
            RoleHandler::InternalSubset
        } else {
            RoleHandler::ExternalSubset1
        };
    }

    #[inline]
    fn common(&mut self, tok: i32) -> i32 {
        if !self.document_entity && tok == XML_TOK_PARAM_ENTITY_REF {
            return XML_ROLE_INNER_PARAM_ENTITY_REF;
        }
        self.handler = RoleHandler::Error;
        XML_ROLE_ERROR
    }

    fn token_role(&mut self, tok: i32, buf: &[u8], ptr: usize, end: usize, enc: EncId) -> i32 {
        let min_bpc = enc.min_bpc();
        match self.handler {
            RoleHandler::Prolog0 => match tok {
                XML_TOK_PROLOG_S => {
                    self.handler = RoleHandler::Prolog1;
                    XML_ROLE_NONE
                }
                XML_TOK_XML_DECL => {
                    self.handler = RoleHandler::Prolog1;
                    XML_ROLE_XML_DECL
                }
                XML_TOK_PI => {
                    self.handler = RoleHandler::Prolog1;
                    XML_ROLE_PI
                }
                XML_TOK_COMMENT => {
                    self.handler = RoleHandler::Prolog1;
                    XML_ROLE_COMMENT
                }
                XML_TOK_BOM => XML_ROLE_NONE,
                XML_TOK_DECL_OPEN => {
                    if !name_matches_ascii(enc, buf, ptr + 2 * min_bpc, end, b"DOCTYPE") {
                        self.common(tok)
                    } else {
                        self.handler = RoleHandler::Doctype0;
                        XML_ROLE_DOCTYPE_NONE
                    }
                }
                XML_TOK_INSTANCE_START => {
                    self.handler = RoleHandler::Error;
                    XML_ROLE_INSTANCE_START
                }
                _ => self.common(tok),
            },
            RoleHandler::Prolog1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_PI => XML_ROLE_PI,
                XML_TOK_COMMENT => XML_ROLE_COMMENT,
                XML_TOK_BOM => XML_ROLE_NONE,
                XML_TOK_DECL_OPEN => {
                    if !name_matches_ascii(enc, buf, ptr + 2 * min_bpc, end, b"DOCTYPE") {
                        self.common(tok)
                    } else {
                        self.handler = RoleHandler::Doctype0;
                        XML_ROLE_DOCTYPE_NONE
                    }
                }
                XML_TOK_INSTANCE_START => {
                    self.handler = RoleHandler::Error;
                    XML_ROLE_INSTANCE_START
                }
                _ => self.common(tok),
            },
            RoleHandler::Prolog2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_PI => XML_ROLE_PI,
                XML_TOK_COMMENT => XML_ROLE_COMMENT,
                XML_TOK_INSTANCE_START => {
                    self.handler = RoleHandler::Error;
                    XML_ROLE_INSTANCE_START
                }
                _ => self.common(tok),
            },
            RoleHandler::Doctype0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Doctype1;
                    XML_ROLE_DOCTYPE_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Doctype1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = RoleHandler::InternalSubset;
                    XML_ROLE_DOCTYPE_INTERNAL_SUBSET
                }
                XML_TOK_DECL_CLOSE => {
                    self.handler = RoleHandler::Prolog2;
                    XML_ROLE_DOCTYPE_CLOSE
                }
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"SYSTEM") {
                        self.handler = RoleHandler::Doctype3;
                        XML_ROLE_DOCTYPE_NONE
                    } else if name_matches_ascii(enc, buf, ptr, end, b"PUBLIC") {
                        self.handler = RoleHandler::Doctype2;
                        XML_ROLE_DOCTYPE_NONE
                    } else {
                        self.common(tok)
                    }
                }
                _ => self.common(tok),
            },
            RoleHandler::Doctype2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Doctype3;
                    XML_ROLE_DOCTYPE_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Doctype3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Doctype4;
                    XML_ROLE_DOCTYPE_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Doctype4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = RoleHandler::InternalSubset;
                    XML_ROLE_DOCTYPE_INTERNAL_SUBSET
                }
                XML_TOK_DECL_CLOSE => {
                    self.handler = RoleHandler::Prolog2;
                    XML_ROLE_DOCTYPE_CLOSE
                }
                _ => self.common(tok),
            },
            RoleHandler::Doctype5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.handler = RoleHandler::Prolog2;
                    XML_ROLE_DOCTYPE_CLOSE
                }
                _ => self.common(tok),
            },
            RoleHandler::InternalSubset => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_DECL_OPEN => {
                    if name_matches_ascii(enc, buf, ptr + 2 * min_bpc, end, b"ENTITY") {
                        self.handler = RoleHandler::Entity0;
                        XML_ROLE_ENTITY_NONE
                    } else if name_matches_ascii(enc, buf, ptr + 2 * min_bpc, end, b"ATTLIST") {
                        self.handler = RoleHandler::Attlist0;
                        XML_ROLE_ATTLIST_NONE
                    } else if name_matches_ascii(enc, buf, ptr + 2 * min_bpc, end, b"ELEMENT") {
                        self.handler = RoleHandler::Element0;
                        XML_ROLE_ELEMENT_NONE
                    } else if name_matches_ascii(enc, buf, ptr + 2 * min_bpc, end, b"NOTATION") {
                        self.handler = RoleHandler::Notation0;
                        XML_ROLE_NOTATION_NONE
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_PI => XML_ROLE_PI,
                XML_TOK_COMMENT => XML_ROLE_COMMENT,
                XML_TOK_PARAM_ENTITY_REF => XML_ROLE_PARAM_ENTITY_REF,
                XML_TOK_CLOSE_BRACKET => {
                    self.handler = RoleHandler::Doctype5;
                    XML_ROLE_DOCTYPE_NONE
                }
                XML_TOK_NONE => XML_ROLE_NONE,
                _ => self.common(tok),
            },
            RoleHandler::ExternalSubset0 => {
                self.handler = RoleHandler::ExternalSubset1;
                if tok == XML_TOK_XML_DECL {
                    XML_ROLE_TEXT_DECL
                } else {
                    self.token_role(tok, buf, ptr, end, enc)
                }
            }
            RoleHandler::ExternalSubset1 => match tok {
                XML_TOK_COND_SECT_OPEN => {
                    self.handler = RoleHandler::CondSect0;
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
                _ => {
                    self.handler = RoleHandler::InternalSubset;
                    let res = self.token_role(tok, buf, ptr, end, enc);
                    if self.handler == RoleHandler::InternalSubset {
                        self.handler = RoleHandler::ExternalSubset1;
                    }
                    res
                }
            },
            RoleHandler::Entity0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_PERCENT => {
                    self.handler = RoleHandler::Entity1;
                    XML_ROLE_ENTITY_NONE
                }
                XML_TOK_NAME => {
                    self.handler = RoleHandler::Entity2;
                    XML_ROLE_GENERAL_ENTITY_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    self.handler = RoleHandler::Entity7;
                    XML_ROLE_PARAM_ENTITY_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"SYSTEM") {
                        self.handler = RoleHandler::Entity4;
                        XML_ROLE_ENTITY_NONE
                    } else if name_matches_ascii(enc, buf, ptr, end, b"PUBLIC") {
                        self.handler = RoleHandler::Entity3;
                        XML_ROLE_ENTITY_NONE
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_ENTITY_NONE;
                    XML_ROLE_ENTITY_VALUE
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Entity4;
                    XML_ROLE_ENTITY_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Entity5;
                    XML_ROLE_ENTITY_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_ENTITY_COMPLETE
                }
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"NDATA") {
                        self.handler = RoleHandler::Entity6;
                        XML_ROLE_ENTITY_NONE
                    } else {
                        self.common(tok)
                    }
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity6 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_ENTITY_NONE;
                    XML_ROLE_ENTITY_NOTATION_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity7 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"SYSTEM") {
                        self.handler = RoleHandler::Entity9;
                        XML_ROLE_ENTITY_NONE
                    } else if name_matches_ascii(enc, buf, ptr, end, b"PUBLIC") {
                        self.handler = RoleHandler::Entity8;
                        XML_ROLE_ENTITY_NONE
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_ENTITY_NONE;
                    XML_ROLE_ENTITY_VALUE
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity8 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Entity9;
                    XML_ROLE_ENTITY_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity9 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Entity10;
                    XML_ROLE_ENTITY_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Entity10 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ENTITY_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_ENTITY_COMPLETE
                }
                _ => self.common(tok),
            },
            RoleHandler::Notation0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_NAME => {
                    self.handler = RoleHandler::Notation1;
                    XML_ROLE_NOTATION_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Notation1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"SYSTEM") {
                        self.handler = RoleHandler::Notation3;
                        XML_ROLE_NOTATION_NONE
                    } else if name_matches_ascii(enc, buf, ptr, end, b"PUBLIC") {
                        self.handler = RoleHandler::Notation2;
                        XML_ROLE_NOTATION_NONE
                    } else {
                        self.common(tok)
                    }
                }
                _ => self.common(tok),
            },
            RoleHandler::Notation2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Notation4;
                    XML_ROLE_NOTATION_PUBLIC_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Notation3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_NOTATION_NONE;
                    XML_ROLE_NOTATION_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Notation4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NOTATION_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_NOTATION_NONE;
                    XML_ROLE_NOTATION_SYSTEM_ID
                }
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_NOTATION_NO_SYSTEM_ID
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Attlist1;
                    XML_ROLE_ATTLIST_ELEMENT_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    XML_ROLE_ATTLIST_NONE
                }
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Attlist2;
                    XML_ROLE_ATTRIBUTE_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist2 => match tok {
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
                    for (i, kw) in TYPES.iter().enumerate() {
                        if name_matches_ascii(enc, buf, ptr, end, kw) {
                            self.handler = RoleHandler::Attlist8;
                            return XML_ROLE_ATTRIBUTE_TYPE_CDATA + (i as i32);
                        }
                    }
                    if name_matches_ascii(enc, buf, ptr, end, b"NOTATION") {
                        self.handler = RoleHandler::Attlist5;
                        XML_ROLE_ATTLIST_NONE
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_OPEN_PAREN => {
                    self.handler = RoleHandler::Attlist3;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Attlist4;
                    XML_ROLE_ATTRIBUTE_ENUM_VALUE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.handler = RoleHandler::Attlist8;
                    XML_ROLE_ATTLIST_NONE
                }
                XML_TOK_OR => {
                    self.handler = RoleHandler::Attlist3;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_OPEN_PAREN => {
                    self.handler = RoleHandler::Attlist6;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist6 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_NAME => {
                    self.handler = RoleHandler::Attlist7;
                    XML_ROLE_ATTRIBUTE_NOTATION_VALUE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist7 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.handler = RoleHandler::Attlist8;
                    XML_ROLE_ATTLIST_NONE
                }
                XML_TOK_OR => {
                    self.handler = RoleHandler::Attlist6;
                    XML_ROLE_ATTLIST_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist8 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_POUND_NAME => {
                    if name_matches_ascii(enc, buf, ptr + min_bpc, end, b"IMPLIED") {
                        self.handler = RoleHandler::Attlist1;
                        XML_ROLE_IMPLIED_ATTRIBUTE_VALUE
                    } else if name_matches_ascii(enc, buf, ptr + min_bpc, end, b"REQUIRED") {
                        self.handler = RoleHandler::Attlist1;
                        XML_ROLE_REQUIRED_ATTRIBUTE_VALUE
                    } else if name_matches_ascii(enc, buf, ptr + min_bpc, end, b"FIXED") {
                        self.handler = RoleHandler::Attlist9;
                        XML_ROLE_ATTLIST_NONE
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Attlist1;
                    XML_ROLE_DEFAULT_ATTRIBUTE_VALUE
                }
                _ => self.common(tok),
            },
            RoleHandler::Attlist9 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE,
                XML_TOK_LITERAL => {
                    self.handler = RoleHandler::Attlist1;
                    XML_ROLE_FIXED_ATTRIBUTE_VALUE
                }
                _ => self.common(tok),
            },
            RoleHandler::Element0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Element1;
                    XML_ROLE_ELEMENT_NAME
                }
                _ => self.common(tok),
            },
            RoleHandler::Element1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"EMPTY") {
                        self.handler = RoleHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                        XML_ROLE_CONTENT_EMPTY
                    } else if name_matches_ascii(enc, buf, ptr, end, b"ANY") {
                        self.handler = RoleHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                        XML_ROLE_CONTENT_ANY
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_OPEN_PAREN => {
                    self.handler = RoleHandler::Element2;
                    self.level = 1;
                    XML_ROLE_GROUP_OPEN
                }
                _ => self.common(tok),
            },
            RoleHandler::Element2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_POUND_NAME => {
                    if name_matches_ascii(enc, buf, ptr + min_bpc, end, b"PCDATA") {
                        self.handler = RoleHandler::Element3;
                        XML_ROLE_CONTENT_PCDATA
                    } else {
                        self.common(tok)
                    }
                }
                XML_TOK_OPEN_PAREN => {
                    self.level = 2;
                    self.handler = RoleHandler::Element6;
                    XML_ROLE_GROUP_OPEN
                }
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT
                }
                XML_TOK_NAME_QUESTION => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_OPT
                }
                XML_TOK_NAME_ASTERISK => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_REP
                }
                XML_TOK_NAME_PLUS => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_PLUS
                }
                _ => self.common(tok),
            },
            RoleHandler::Element3 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_ELEMENT_NONE;
                    XML_ROLE_GROUP_CLOSE
                }
                XML_TOK_CLOSE_PAREN_ASTERISK => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_ELEMENT_NONE;
                    XML_ROLE_GROUP_CLOSE_REP
                }
                XML_TOK_OR => {
                    self.handler = RoleHandler::Element4;
                    XML_ROLE_ELEMENT_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::Element4 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Element5;
                    XML_ROLE_CONTENT_ELEMENT
                }
                _ => self.common(tok),
            },
            RoleHandler::Element5 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_CLOSE_PAREN_ASTERISK => {
                    self.handler = RoleHandler::DeclClose;
                    self.role_none = XML_ROLE_ELEMENT_NONE;
                    XML_ROLE_GROUP_CLOSE_REP
                }
                XML_TOK_OR => {
                    self.handler = RoleHandler::Element4;
                    XML_ROLE_ELEMENT_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::Element6 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_OPEN_PAREN => {
                    self.level += 1;
                    XML_ROLE_GROUP_OPEN
                }
                XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT
                }
                XML_TOK_NAME_QUESTION => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_OPT
                }
                XML_TOK_NAME_ASTERISK => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_REP
                }
                XML_TOK_NAME_PLUS => {
                    self.handler = RoleHandler::Element7;
                    XML_ROLE_CONTENT_ELEMENT_PLUS
                }
                _ => self.common(tok),
            },
            RoleHandler::Element7 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_ELEMENT_NONE,
                XML_TOK_CLOSE_PAREN => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = RoleHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE
                }
                XML_TOK_CLOSE_PAREN_ASTERISK => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = RoleHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE_REP
                }
                XML_TOK_CLOSE_PAREN_QUESTION => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = RoleHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE_OPT
                }
                XML_TOK_CLOSE_PAREN_PLUS => {
                    self.level -= 1;
                    if self.level == 0 {
                        self.handler = RoleHandler::DeclClose;
                        self.role_none = XML_ROLE_ELEMENT_NONE;
                    }
                    XML_ROLE_GROUP_CLOSE_PLUS
                }
                XML_TOK_COMMA => {
                    self.handler = RoleHandler::Element6;
                    XML_ROLE_GROUP_SEQUENCE
                }
                XML_TOK_OR => {
                    self.handler = RoleHandler::Element6;
                    XML_ROLE_GROUP_CHOICE
                }
                _ => self.common(tok),
            },
            RoleHandler::CondSect0 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_NAME => {
                    if name_matches_ascii(enc, buf, ptr, end, b"INCLUDE") {
                        self.handler = RoleHandler::CondSect1;
                        XML_ROLE_NONE
                    } else if name_matches_ascii(enc, buf, ptr, end, b"IGNORE") {
                        self.handler = RoleHandler::CondSect2;
                        XML_ROLE_NONE
                    } else {
                        self.common(tok)
                    }
                }
                _ => self.common(tok),
            },
            RoleHandler::CondSect1 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = RoleHandler::ExternalSubset1;
                    self.include_level += 1;
                    XML_ROLE_NONE
                }
                _ => self.common(tok),
            },
            RoleHandler::CondSect2 => match tok {
                XML_TOK_PROLOG_S => XML_ROLE_NONE,
                XML_TOK_OPEN_BRACKET => {
                    self.handler = RoleHandler::ExternalSubset1;
                    XML_ROLE_IGNORE_SECT
                }
                _ => self.common(tok),
            },
            RoleHandler::DeclClose => match tok {
                XML_TOK_PROLOG_S => self.role_none,
                XML_TOK_DECL_CLOSE => {
                    self.set_top_level();
                    self.role_none
                }
                _ => self.common(tok),
            },
            RoleHandler::Error => XML_ROLE_NONE,
        }
    }
}

// ---- xmlparse.c structures and processors ----

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum XmlAccount {
    Direct,
    EntityExpansion,
    None,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum EntityRefId {
    General(usize),
    Param(usize),
}

#[derive(Clone, Debug)]
struct Entity {
    name: Vec<u8>,
    text: Option<Vec<u8>>,
    processed: usize,
    system_id: Option<Vec<u8>>,
    public_id: Option<Vec<u8>>,
    notation: Option<Vec<u8>>,
    open: bool,
    has_more: bool,
    is_param: bool,
    is_internal: bool,
}

impl Entity {
    fn new(name: Vec<u8>, is_param: bool, is_internal: bool) -> Self {
        Self {
            name,
            text: None,
            processed: 0,
            system_id: None,
            public_id: None,
            notation: None,
            open: false,
            has_more: false,
            is_param,
            is_internal,
        }
    }
}

#[derive(Clone, Debug)]
struct AttributeId {
    name: Vec<u8>,
    in_use: u8,
    maybe_tokenized: bool,
}

#[derive(Clone, Debug)]
struct DefaultAttribute {
    att_id_idx: usize,
    is_cdata: bool,
    value: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
struct ElementType {
    name: Vec<u8>,
    id_att_idx: Option<usize>,
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

    fn insert_general_entity(&mut self, name: Vec<u8>, is_internal: bool) -> (usize, bool) {
        if let Some(&idx) = self.general_entity_map.get(&name) {
            (idx, false)
        } else {
            let idx = self.general_entities.len();
            self.general_entities
                .push(Entity::new(name.clone(), false, is_internal));
            self.general_entity_map.insert(name, idx);
            (idx, true)
        }
    }

    fn insert_param_entity(&mut self, name: Vec<u8>, is_internal: bool) -> (usize, bool) {
        if let Some(&idx) = self.param_entity_map.get(&name) {
            (idx, false)
        } else {
            let idx = self.param_entities.len();
            self.param_entities
                .push(Entity::new(name.clone(), true, is_internal));
            self.param_entity_map.insert(name, idx);
            (idx, true)
        }
    }

    fn get_or_insert_element_type(&mut self, name: &[u8]) -> usize {
        if let Some(&idx) = self.element_type_map.get(name) {
            idx
        } else {
            let idx = self.element_types.len();
            let owned = name.to_vec();
            self.element_types.push(ElementType {
                name: owned.clone(),
                id_att_idx: None,
                default_atts: Vec::new(),
                default_att_for_name: HashMap::new(),
            });
            self.element_type_map.insert(owned, idx);
            idx
        }
    }

    fn get_or_insert_attribute_id(&mut self, name: &[u8]) -> usize {
        if let Some(&idx) = self.attribute_id_map.get(name) {
            idx
        } else {
            let idx = self.attribute_ids.len();
            let owned = name.to_vec();
            self.attribute_ids.push(AttributeId {
                name: owned.clone(),
                in_use: 0,
                maybe_tokenized: false,
            });
            self.attribute_id_map.insert(owned, idx);
            idx
        }
    }

    fn get_entity(&self, id: EntityRefId) -> &Entity {
        match id {
            EntityRefId::General(idx) => &self.general_entities[idx],
            EntityRefId::Param(idx) => &self.param_entities[idx],
        }
    }

    fn get_entity_mut(&mut self, id: EntityRefId) -> &mut Entity {
        match id {
            EntityRefId::General(idx) => &mut self.general_entities[idx],
            EntityRefId::Param(idx) => &mut self.param_entities[idx],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum EntityType {
    Internal,
    Attribute,
    Value,
}

#[derive(Clone, Debug)]
struct OpenInternalEntity {
    internal_event_ptr: Option<usize>,
    internal_event_end_ptr: Option<usize>,
    entity_id: EntityRefId,
    start_tag_level: i32,
    between_decl: bool,
    entity_type: EntityType,
}

#[derive(Clone, Debug)]
struct Tag {
    raw_name: Vec<u8>,
    name_str: Vec<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
            maximum_amplification_factor: 100.0,
            activation_threshold_bytes: 8_388_608,
        }
    }

    fn get_current_amplification(&self) -> f32 {
        const LEN_OF_SHORTEST_INCLUDE: u64 = 22; // sizeof("<!ENTITY a SYSTEM 'b'>") - 1
        let count_bytes_output = self.count_bytes_direct + self.count_bytes_indirect;
        if self.count_bytes_direct != 0 {
            (count_bytes_output as f32) / (self.count_bytes_direct as f32)
        } else {
            ((LEN_OF_SHORTEST_INCLUDE + self.count_bytes_indirect) as f32)
                / (LEN_OF_SHORTEST_INCLUDE as f32)
        }
    }

    fn diff_tolerated(
        &mut self,
        tok: i32,
        before: usize,
        after: usize,
        account: XmlAccount,
    ) -> bool {
        match tok {
            XML_TOK_INVALID | XML_TOK_PARTIAL | XML_TOK_PARTIAL_CHAR | XML_TOK_NONE => {
                return true;
            }
            _ => {}
        }
        if account == XmlAccount::None {
            return true;
        }
        let is_direct = account == XmlAccount::Direct;
        let bytes_more = (after as u64).wrapping_sub(before as u64);
        let addition_target = if is_direct {
            &mut self.count_bytes_direct
        } else {
            &mut self.count_bytes_indirect
        };
        if *addition_target > u64::MAX - bytes_more {
            return false;
        }
        *addition_target += bytes_more;

        let count_bytes_output = self.count_bytes_direct + self.count_bytes_indirect;
        let amplification_factor = self.get_current_amplification();
        (count_bytes_output < self.activation_threshold_bytes)
            || (amplification_factor <= self.maximum_amplification_factor)
    }
}

struct Parser<'a> {
    input: &'a [u8],
    encoding: EncId,
    internal_encoding: EncId,
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
    free_tag_list: Vec<Tag>,
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
    events: Vec<Event>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            encoding: EncId::Init,
            internal_encoding: EncId::InternalUtf8,
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
            free_tag_list: Vec::new(),
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
            events: Vec::new(),
        }
    }

    #[inline]
    fn set_event_ptr(&mut self, is_main_enc: bool, ptr: usize) {
        if is_main_enc {
            self.event_ptr = Some(ptr);
        } else if let Some(top) = self.open_internal_entities.last_mut() {
            top.internal_event_ptr = Some(ptr);
        }
    }

    #[inline]
    fn set_event_end_ptr(&mut self, is_main_enc: bool, ptr: usize) {
        if is_main_enc {
            self.event_end_ptr = Some(ptr);
        } else if let Some(top) = self.open_internal_entities.last_mut() {
            top.internal_event_end_ptr = Some(ptr);
        }
    }

    fn get_current_line_and_col(&mut self) -> (u64, u64) {
        if let Some(ev_ptr) = self.event_ptr {
            if ev_ptr >= self.position_ptr {
                update_position(
                    self.encoding,
                    self.input,
                    self.position_ptr,
                    ev_ptr,
                    &mut self.position,
                );
                self.position_ptr = ev_ptr;
            }
        }
        (self.position.line_number + 1, self.position.column_number)
    }

    fn call_processor(&mut self, start: usize, end: usize) -> u32 {
        let mut end_ptr = start;
        loop {
            let ret = match self.processor {
                ProcessorKind::PrologInit => self.prolog_init_processor(end_ptr, end, &mut end_ptr),
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

    fn prolog_init_processor(&mut self, s: usize, end: usize, next_ptr: &mut usize) -> u32 {
        self.processor = ProcessorKind::Prolog;
        self.prolog_processor(s, end, next_ptr)
    }

    fn prolog_processor(&mut self, s: usize, end: usize, next_ptr: &mut usize) -> u32 {
        let mut next = s;
        let tok = xml_prolog_tok(&mut self.encoding, self.input, s, end, &mut next);
        let enc = self.encoding;
        self.do_prolog(
            enc,
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

    fn content_processor(&mut self, start: usize, end: usize, end_ptr: &mut usize) -> u32 {
        let enc = self.encoding;
        self.do_content(
            0,
            enc,
            self.input,
            start,
            end,
            end_ptr,
            false,
            XmlAccount::Direct,
        )
    }

    fn cdata_section_processor(&mut self, mut start: usize, end: usize, end_ptr: &mut usize) -> u32 {
        let enc = self.encoding;
        let mut start_opt = Some(start);
        let result = self.do_cdata_section(
            enc,
            self.input,
            &mut start_opt,
            end,
            end_ptr,
            false,
            XmlAccount::Direct,
        );
        if result != XML_ERROR_NONE {
            return result;
        }
        if let Some(new_start) = start_opt {
            start = new_start;
            self.processor = ProcessorKind::Content;
            return self.content_processor(start, end, end_ptr);
        }
        result
    }

    fn ignore_section_processor(
        &mut self,
        mut start: usize,
        end: usize,
        end_ptr: &mut usize,
    ) -> u32 {
        let enc = self.encoding;
        let mut start_opt = Some(start);
        let result = self.do_ignore_section(enc, self.input, &mut start_opt, end, end_ptr, false);
        if result != XML_ERROR_NONE {
            return result;
        }
        if let Some(new_start) = start_opt {
            start = new_start;
            self.processor = ProcessorKind::Prolog;
            return self.prolog_processor(start, end, end_ptr);
        }
        result
    }

    fn epilog_processor(&mut self, mut s: usize, end: usize, next_ptr: &mut usize) -> u32 {
        self.processor = ProcessorKind::Epilog;
        self.event_ptr = Some(s);
        loop {
            let mut next = s;
            let tok = xml_prolog_tok(&mut self.encoding, self.input, s, end, &mut next);
            if !self
                .accounting
                .diff_tolerated(tok, s, next, XmlAccount::Direct)
            {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            self.event_end_ptr = Some(next);
            match tok {
                x if x == -XML_TOK_PROLOG_S => {
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
        let open_entity = OpenInternalEntity {
            internal_event_ptr: None,
            internal_event_end_ptr: None,
            entity_id,
            start_tag_level: self.tag_level,
            between_decl,
            entity_type,
        };
        let ent = self.dtd.get_entity_mut(entity_id);
        ent.open = true;
        ent.has_more = true;
        ent.processed = 0;

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
        let entity_id = open_entity.entity_id;
        let (has_more, is_param, processed, text_opt) = {
            let ent = self.dtd.get_entity(entity_id);
            (ent.has_more, ent.is_param, ent.processed, ent.text.clone())
        };

        if has_more {
            let text_bytes = text_opt.unwrap_or_default();
            let text_start = processed;
            let text_end = text_bytes.len();
            let mut next = text_start;
            let internal_enc = self.internal_encoding;

            let result = if is_param {
                let tok = prolog_tok(internal_enc, &text_bytes, text_start, text_end, &mut next);
                self.do_prolog(
                    internal_enc,
                    &text_bytes,
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
                    internal_enc,
                    &text_bytes,
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
                self.dtd.get_entity_mut(entity_id).processed = next;
                return result;
            }

            self.dtd.get_entity_mut(entity_id).has_more = false;
            if !is_param && open_entity.start_tag_level != self.tag_level {
                return XML_ERROR_ASYNC_ENTITY;
            }
            self.reenter = true;
            return result;
        }

        self.dtd.get_entity_mut(entity_id).open = false;
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
    fn do_content(
        &mut self,
        start_tag_level: i32,
        mut enc: EncId,
        buf: &[u8],
        mut s: usize,
        end: usize,
        next_ptr: &mut usize,
        have_more: bool,
        account: XmlAccount,
    ) -> u32 {
        let is_main_enc = enc != EncId::InternalUtf8;
        self.set_event_ptr(is_main_enc, s);

        loop {
            let mut next = s;
            let tok = if is_main_enc {
                let t = xml_content_tok(&mut self.encoding, buf, s, end, &mut next);
                enc = self.encoding;
                t
            } else {
                content_tok(enc, buf, s, end, &mut next)
            };

            let account_after =
                if tok == XML_TOK_TRAILING_RSQB || tok == XML_TOK_TRAILING_CR {
                    if have_more { s } else { end }
                } else {
                    next
                };
            if !self
                .accounting
                .diff_tolerated(tok, s, account_after, account)
            {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            self.set_event_end_ptr(is_main_enc, next);

            match tok {
                XML_TOK_TRAILING_CR => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    self.set_event_end_ptr(is_main_enc, end);
                    self.events.push(Event::Text(vec![0x0A]));
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
                    self.set_event_ptr(is_main_enc, next);
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
                    let min_bpc = enc.min_bpc();
                    let ch = predefined_entity_name(enc, buf, s + min_bpc, next - min_bpc);
                    if ch != 0 {
                        self.accounting
                            .diff_tolerated(tok, 0, 1, XmlAccount::EntityExpansion);
                        self.events.push(Event::Text(vec![ch]));
                    } else {
                        let name = store_string_to_vec(enc, buf, s + min_bpc, next - min_bpc);
                        let entity_idx_opt = self.dtd.lookup_general_entity(&name);
                        if !self.dtd.has_param_entity_refs || self.dtd.standalone {
                            let Some(ent_idx) = entity_idx_opt else {
                                return XML_ERROR_UNDEFINED_ENTITY;
                            };
                            if !self.dtd.general_entities[ent_idx].is_internal {
                                return XML_ERROR_ENTITY_DECLARED_IN_PE;
                            }
                        } else if entity_idx_opt.is_none() {
                            self.events.push(Event::Skipped {
                                name,
                                is_pe: false,
                            });
                            if self.reenter {
                                *next_ptr = next;
                                return XML_ERROR_NONE;
                            }
                            self.set_event_ptr(is_main_enc, next);
                            s = next;
                            continue;
                        }
                        let ent_idx = entity_idx_opt.unwrap();
                        if self.dtd.general_entities[ent_idx].open {
                            return XML_ERROR_RECURSIVE_ENTITY_REF;
                        }
                        if self.dtd.general_entities[ent_idx].notation.is_some() {
                            return XML_ERROR_BINARY_ENTITY_REF;
                        }
                        if self.dtd.general_entities[ent_idx].text.is_some() {
                            let res = self.process_entity(
                                EntityRefId::General(ent_idx),
                                false,
                                EntityType::Internal,
                            );
                            if res != XML_ERROR_NONE {
                                return res;
                            }
                        }
                    }
                }
                XML_TOK_START_TAG_NO_ATTS | XML_TOK_START_TAG_WITH_ATTS => {
                    let min_bpc = enc.min_bpc();
                    let raw_name_pos = s + min_bpc;
                    let raw_name_len = name_length(enc, buf, raw_name_pos);
                    let mut tag = self.free_tag_list.pop().unwrap_or(Tag {
                        raw_name: Vec::new(),
                        name_str: Vec::new(),
                    });
                    tag.raw_name.clear();
                    tag.raw_name
                        .extend_from_slice(&buf[raw_name_pos..raw_name_pos + raw_name_len]);
                    tag.name_str.clear();
                    append_converted_to_vec(
                        enc,
                        buf,
                        raw_name_pos,
                        raw_name_pos + raw_name_len,
                        &mut tag.name_str,
                    );
                    self.tag_level += 1;
                    let elem_name = tag.name_str.clone();
                    self.tag_stack.push(tag);
                    let mut attrs = Vec::new();
                    let result =
                        self.store_atts(enc, buf, s, &elem_name, &mut attrs, account);
                    if result != XML_ERROR_NONE {
                        return result;
                    }
                    self.events.push(Event::Start {
                        name: elem_name,
                        attrs,
                    });
                }
                XML_TOK_EMPTY_ELEMENT_NO_ATTS | XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                    let min_bpc = enc.min_bpc();
                    let raw_name_pos = s + min_bpc;
                    let raw_name_len = name_length(enc, buf, raw_name_pos);
                    let elem_name =
                        store_string_to_vec(enc, buf, raw_name_pos, raw_name_pos + raw_name_len);
                    let mut attrs = Vec::new();
                    let result =
                        self.store_atts(enc, buf, s, &elem_name, &mut attrs, XmlAccount::None);
                    if result != XML_ERROR_NONE {
                        return result;
                    }
                    self.events.push(Event::Start {
                        name: elem_name.clone(),
                        attrs,
                    });
                    self.set_event_ptr(is_main_enc, next);
                    self.events.push(Event::End { name: elem_name });
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
                    let min_bpc = enc.min_bpc();
                    let raw_name_pos = s + min_bpc * 2;
                    let len = name_length(enc, buf, raw_name_pos);
                    let Some(tag) = self.tag_stack.pop() else {
                        return XML_ERROR_ASYNC_ENTITY;
                    };
                    if len != tag.raw_name.len()
                        || tag.raw_name.as_slice() != &buf[raw_name_pos..raw_name_pos + len]
                    {
                        self.set_event_ptr(is_main_enc, raw_name_pos);
                        return XML_ERROR_TAG_MISMATCH;
                    }
                    self.tag_level -= 1;
                    self.events.push(Event::End {
                        name: tag.name_str.clone(),
                    });
                    self.free_tag_list.push(tag);
                    if self.tag_level == 0 {
                        if self.reenter {
                            self.processor = ProcessorKind::Epilog;
                        } else {
                            return self.epilog_processor(next, end, next_ptr);
                        }
                    }
                }
                XML_TOK_CHAR_REF => {
                    let n = char_ref_number(enc, buf, s);
                    if n < 0 {
                        return XML_ERROR_BAD_CHAR_REF;
                    }
                    let mut enc_buf = [0u8; 4];
                    let enc_len = xml_utf8_encode(n, &mut enc_buf);
                    self.events.push(Event::Text(enc_buf[..enc_len].to_vec()));
                }
                XML_TOK_XML_DECL => {
                    return XML_ERROR_MISPLACED_XML_PI;
                }
                XML_TOK_DATA_NEWLINE => {
                    self.events.push(Event::Text(vec![0x0A]));
                }
                XML_TOK_CDATA_SECT_OPEN => {
                    self.events.push(Event::CdataStart);
                    let mut next_opt = Some(next);
                    let result = self.do_cdata_section(
                        enc,
                        buf,
                        &mut next_opt,
                        end,
                        next_ptr,
                        have_more,
                        account,
                    );
                    if result != XML_ERROR_NONE {
                        return result;
                    } else if let Some(new_next) = next_opt {
                        next = new_next;
                    } else {
                        self.processor = ProcessorKind::CdataSection;
                        return result;
                    }
                }
                XML_TOK_TRAILING_RSQB => {
                    if have_more {
                        *next_ptr = s;
                        return XML_ERROR_NONE;
                    }
                    let text = store_string_to_vec(enc, buf, s, end);
                    self.events.push(Event::Text(text));
                    if start_tag_level == 0 {
                        self.set_event_ptr(is_main_enc, end);
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    if self.tag_level != start_tag_level {
                        self.set_event_ptr(is_main_enc, end);
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *next_ptr = end;
                    return XML_ERROR_NONE;
                }
                XML_TOK_DATA_CHARS => {
                    let text = store_string_to_vec(enc, buf, s, next);
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
            self.set_event_ptr(is_main_enc, next);
            s = next;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn do_cdata_section(
        &mut self,
        enc: EncId,
        buf: &[u8],
        start_ptr: &mut Option<usize>,
        end: usize,
        next_ptr: &mut usize,
        have_more: bool,
        account: XmlAccount,
    ) -> u32 {
        let mut s = start_ptr.unwrap_or(0);
        let is_main_enc = enc != EncId::InternalUtf8;
        self.set_event_ptr(is_main_enc, s);
        *start_ptr = None;

        loop {
            let mut next = s;
            let tok = cdata_section_tok(enc, buf, s, end, &mut next);
            if !self.accounting.diff_tolerated(tok, s, next, account) {
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            self.set_event_end_ptr(is_main_enc, next);

            match tok {
                XML_TOK_CDATA_SECT_CLOSE => {
                    self.events.push(Event::CdataEnd);
                    *start_ptr = Some(next);
                    *next_ptr = next;
                    return XML_ERROR_NONE;
                }
                XML_TOK_DATA_NEWLINE => {
                    self.events.push(Event::Text(vec![0x0A]));
                }
                XML_TOK_DATA_CHARS => {
                    let text = store_string_to_vec(enc, buf, s, next);
                    self.events.push(Event::Text(text));
                }
                XML_TOK_INVALID => {
                    self.set_event_ptr(is_main_enc, next);
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
                    self.set_event_ptr(is_main_enc, next);
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            if self.reenter {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            self.set_event_ptr(is_main_enc, next);
            s = next;
        }
    }

    fn do_ignore_section(
        &mut self,
        enc: EncId,
        buf: &[u8],
        start_ptr: &mut Option<usize>,
        end: usize,
        next_ptr: &mut usize,
        have_more: bool,
    ) -> u32 {
        let s = start_ptr.unwrap_or(0);
        let mut next = s;
        let is_main_enc = enc != EncId::InternalUtf8;
        self.set_event_ptr(is_main_enc, s);
        *start_ptr = None;
        let tok = ignore_section_tok(enc, buf, s, end, &mut next);
        if !self
            .accounting
            .diff_tolerated(tok, s, next, XmlAccount::Direct)
        {
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        self.set_event_end_ptr(is_main_enc, next);
        match tok {
            XML_TOK_IGNORE_SECT => {
                *start_ptr = Some(next);
                *next_ptr = next;
                XML_ERROR_NONE
            }
            XML_TOK_INVALID => {
                self.set_event_ptr(is_main_enc, next);
                XML_ERROR_INVALID_TOKEN
            }
            XML_TOK_PARTIAL_CHAR => {
                if have_more {
                    *next_ptr = s;
                    XML_ERROR_NONE
                } else {
                    XML_ERROR_PARTIAL_CHAR
                }
            }
            XML_TOK_PARTIAL | XML_TOK_NONE => {
                if have_more {
                    *next_ptr = s;
                    XML_ERROR_NONE
                } else {
                    XML_ERROR_SYNTAX
                }
            }
            _ => {
                self.set_event_ptr(is_main_enc, next);
                XML_ERROR_UNEXPECTED_STATE
            }
        }
    }

    fn process_xml_decl(&mut self, is_general_text_entity: bool, s: usize, next: usize) -> u32 {
        if !self
            .accounting
            .diff_tolerated(XML_TOK_XML_DECL, s, next, XmlAccount::Direct)
        {
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        let mut version_bounds: Option<(usize, usize)> = None;
        let mut encoding_name_pos: Option<usize> = None;
        let mut new_encoding: Option<EncId> = None;
        let mut standalone: i32 = -1;

        if !do_parse_xml_decl(
            is_general_text_entity,
            self.encoding,
            self.input,
            s,
            next,
            &mut self.event_ptr,
            &mut version_bounds,
            &mut encoding_name_pos,
            &mut new_encoding,
            &mut standalone,
        ) {
            return if is_general_text_entity {
                XML_ERROR_TEXT_DECL
            } else {
                XML_ERROR_XML_DECL
            };
        }

        if !is_general_text_entity && standalone == 1 {
            self.dtd.standalone = true;
        }

        let stored_enc_name = encoding_name_pos.map(|enc_pos| {
            let len = name_length(self.encoding, self.input, enc_pos);
            store_string_to_vec(self.encoding, self.input, enc_pos, enc_pos + len)
        });
        let stored_version = version_bounds.map(|(v_start, v_end)| {
            store_string_to_vec(
                self.encoding,
                self.input,
                v_start,
                v_end - self.encoding.min_bpc(),
            )
        });

        self.events.push(Event::XmlDecl {
            version: stored_version,
            encoding: stored_enc_name,
            standalone,
        });

        if let Some(new_enc) = new_encoding {
            if new_enc.min_bpc() != self.encoding.min_bpc()
                || (new_enc.min_bpc() == 2 && new_enc != self.encoding)
            {
                self.event_ptr = encoding_name_pos;
                return XML_ERROR_INCORRECT_ENCODING;
            }
            self.encoding = new_enc;
        } else if let Some(enc_pos) = encoding_name_pos {
            self.event_ptr = Some(enc_pos);
            return XML_ERROR_UNKNOWN_ENCODING;
        }

        XML_ERROR_NONE
    }

    #[allow(clippy::too_many_arguments)]
    fn do_prolog(
        &mut self,
        mut enc: EncId,
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
        let is_main_enc = enc != EncId::InternalUtf8;

        loop {
            self.set_event_ptr(is_main_enc, s);
            self.set_event_end_ptr(is_main_enc, next);

            if tok <= 0 {
                if have_more && tok != XML_TOK_INVALID {
                    *next_ptr = s;
                    return XML_ERROR_NONE;
                }
                match tok {
                    XML_TOK_INVALID => {
                        self.set_event_ptr(is_main_enc, next);
                        return XML_ERROR_INVALID_TOKEN;
                    }
                    XML_TOK_PARTIAL => {
                        return XML_ERROR_UNCLOSED_TOKEN;
                    }
                    XML_TOK_PARTIAL_CHAR => {
                        return XML_ERROR_PARTIAL_CHAR;
                    }
                    x if x == -XML_TOK_PROLOG_S => {
                        tok = -tok;
                    }
                    XML_TOK_NONE => {
                        if !is_main_enc {
                            let between_decl = self
                                .open_internal_entities
                                .last()
                                .map(|e| e.between_decl)
                                .unwrap_or(false);
                            if !between_decl {
                                *next_ptr = s;
                                return XML_ERROR_NONE;
                            }
                            if self.prolog_state.token_role(XML_TOK_NONE, buf, end, end, enc)
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
                    let result = self.process_xml_decl(false, s, next);
                    if result != XML_ERROR_NONE {
                        return result;
                    }
                    enc = self.encoding;
                }
                XML_ROLE_DOCTYPE_NAME => {
                    self.doctype_name = Some(store_string_to_vec(enc, buf, s, next));
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
                XML_ROLE_TEXT_DECL => {
                    let result = self.process_xml_decl(true, s, next);
                    if result != XML_ERROR_NONE {
                        return result;
                    }
                    enc = self.encoding;
                }
                XML_ROLE_DOCTYPE_PUBLIC_ID => {
                    let (idx, _) = self.dtd.insert_param_entity(b"#".to_vec(), true);
                    self.decl_entity = Some(EntityRefId::Param(idx));
                    self.dtd.has_param_entity_refs = true;

                    let mut bad_pos = s;
                    if !is_public_id(enc, buf, s, next, &mut bad_pos) {
                        self.set_event_ptr(is_main_enc, bad_pos);
                        return XML_ERROR_PUBLICID;
                    }
                    let min_bpc = enc.min_bpc();
                    let mut pub_id =
                        store_string_to_vec(enc, buf, s + min_bpc, next - min_bpc);
                    normalize_public_id(&mut pub_id);
                    self.doctype_pubid = Some(pub_id.clone());
                    if self.dtd.keep_processing {
                        if let Some(decl_id) = self.decl_entity {
                            self.dtd.get_entity_mut(decl_id).public_id = Some(pub_id);
                        }
                    }
                }
                XML_ROLE_ENTITY_PUBLIC_ID => {
                    let mut bad_pos = s;
                    if !is_public_id(enc, buf, s, next, &mut bad_pos) {
                        self.set_event_ptr(is_main_enc, bad_pos);
                        return XML_ERROR_PUBLICID;
                    }
                    if self.dtd.keep_processing {
                        if let Some(decl_id) = self.decl_entity {
                            let min_bpc = enc.min_bpc();
                            let mut pub_id =
                                store_string_to_vec(enc, buf, s + min_bpc, next - min_bpc);
                            normalize_public_id(&mut pub_id);
                            self.dtd.get_entity_mut(decl_id).public_id = Some(pub_id);
                        }
                    }
                }
                XML_ROLE_DOCTYPE_CLOSE => {
                    if !allow_closing_doctype {
                        return XML_ERROR_INVALID_TOKEN;
                    }
                    if let Some(name) = self.doctype_name.take() {
                        let sysid = self.doctype_sysid.clone();
                        let pubid = self.doctype_pubid.take();
                        self.events.push(Event::DoctypeStart {
                            name,
                            sysid,
                            pubid,
                            has_internal_subset: false,
                        });
                    }
                    if self.doctype_sysid.is_some() {
                        self.dtd.has_param_entity_refs = true;
                    }
                    self.events.push(Event::DoctypeEnd);
                }
                XML_ROLE_INSTANCE_START => {
                    self.processor = ProcessorKind::Content;
                    return self.content_processor(s, end, next_ptr);
                }
                XML_ROLE_ATTLIST_ELEMENT_NAME => {
                    let name = store_string_to_vec(enc, buf, s, next);
                    let idx = self.dtd.get_or_insert_element_type(&name);
                    self.decl_element_type = Some(idx);
                }
                XML_ROLE_ATTRIBUTE_NAME => {
                    let name = store_string_to_vec(enc, buf, s, next);
                    let idx = self.dtd.get_or_insert_attribute_id(&name);
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
                XML_ROLE_ATTRIBUTE_TYPE_IDREF
                | XML_ROLE_ATTRIBUTE_TYPE_IDREFS
                | XML_ROLE_ATTRIBUTE_TYPE_ENTITY
                | XML_ROLE_ATTRIBUTE_TYPE_ENTITIES
                | XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN
                | XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS
                | XML_ROLE_ATTRIBUTE_ENUM_VALUE
                | XML_ROLE_ATTRIBUTE_NOTATION_VALUE => {}
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
                        let min_bpc = enc.min_bpc();
                        let mut att_val = Vec::new();
                        let result = self.store_attribute_value(
                            enc,
                            buf,
                            self.decl_attribute_is_cdata,
                            s + min_bpc,
                            next - min_bpc,
                            true,
                            &mut att_val,
                            XmlAccount::None,
                        );
                        if result != XML_ERROR_NONE {
                            return result;
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
                        let min_bpc = enc.min_bpc();
                        let mut ent_val = Vec::new();
                        let result = self.call_store_entity_value(
                            enc,
                            buf,
                            s + min_bpc,
                            next - min_bpc,
                            XmlAccount::None,
                            &mut ent_val,
                        );
                        if let Some(decl_id) = self.decl_entity {
                            let (name, is_param) = {
                                let ent = self.dtd.get_entity_mut(decl_id);
                                ent.text = Some(ent_val.clone());
                                (ent.name.clone(), ent.is_param)
                            };
                            self.set_event_end_ptr(is_main_enc, s);
                            self.events.push(Event::EntityDecl {
                                name,
                                is_pe: is_param,
                                value: Some(ent_val),
                                sysid: None,
                                pubid: None,
                                notation: None,
                            });
                        }
                        if result != XML_ERROR_NONE {
                            return result;
                        }
                    }
                }
                XML_ROLE_DOCTYPE_SYSTEM_ID => {
                    self.dtd.has_param_entity_refs = true;
                    let min_bpc = enc.min_bpc();
                    let sys_id = store_string_to_vec(enc, buf, s + min_bpc, next - min_bpc);
                    self.doctype_sysid = Some(sys_id.clone());
                    if self.decl_entity.is_none() {
                        let (idx, _) = self.dtd.insert_param_entity(b"#".to_vec(), true);
                        self.dtd.param_entities[idx].public_id = None;
                        self.decl_entity = Some(EntityRefId::Param(idx));
                    }
                    if self.dtd.keep_processing {
                        if let Some(decl_id) = self.decl_entity {
                            self.dtd.get_entity_mut(decl_id).system_id = Some(sys_id);
                        }
                    }
                }
                XML_ROLE_ENTITY_SYSTEM_ID => {
                    if self.dtd.keep_processing {
                        if let Some(decl_id) = self.decl_entity {
                            let min_bpc = enc.min_bpc();
                            let sys_id =
                                store_string_to_vec(enc, buf, s + min_bpc, next - min_bpc);
                            self.dtd.get_entity_mut(decl_id).system_id = Some(sys_id);
                        }
                    }
                }
                XML_ROLE_ENTITY_COMPLETE => {
                    if self.dtd.keep_processing {
                        if let Some(decl_id) = self.decl_entity {
                            self.set_event_end_ptr(is_main_enc, s);
                            let ent = self.dtd.get_entity(decl_id);
                            self.events.push(Event::EntityDecl {
                                name: ent.name.clone(),
                                is_pe: ent.is_param,
                                value: None,
                                sysid: ent.system_id.clone(),
                                pubid: ent.public_id.clone(),
                                notation: None,
                            });
                        }
                    }
                }
                XML_ROLE_ENTITY_NOTATION_NAME => {
                    if self.dtd.keep_processing {
                        if let Some(decl_id) = self.decl_entity {
                            let notation = store_string_to_vec(enc, buf, s, next);
                            self.dtd.get_entity_mut(decl_id).notation = Some(notation);
                            self.set_event_end_ptr(is_main_enc, s);
                            let ent = self.dtd.get_entity(decl_id);
                            self.events.push(Event::EntityDecl {
                                name: ent.name.clone(),
                                is_pe: false,
                                value: None,
                                sysid: ent.system_id.clone(),
                                pubid: ent.public_id.clone(),
                                notation: ent.notation.clone(),
                            });
                        }
                    }
                }
                XML_ROLE_GENERAL_ENTITY_NAME => {
                    if predefined_entity_name(enc, buf, s, next) != 0 {
                        self.decl_entity = None;
                    } else if self.dtd.keep_processing {
                        let name = store_string_to_vec(enc, buf, s, next);
                        let is_internal = self.open_internal_entities.is_empty();
                        let (idx, inserted) = self.dtd.insert_general_entity(name, is_internal);
                        if !inserted {
                            self.decl_entity = None;
                        } else {
                            self.dtd.general_entities[idx].public_id = None;
                            self.decl_entity = Some(EntityRefId::General(idx));
                        }
                    } else {
                        self.decl_entity = None;
                    }
                }
                XML_ROLE_PARAM_ENTITY_NAME => {
                    if self.dtd.keep_processing {
                        let name = store_string_to_vec(enc, buf, s, next);
                        let is_internal = self.open_internal_entities.is_empty();
                        let (idx, inserted) = self.dtd.insert_param_entity(name, is_internal);
                        if !inserted {
                            self.decl_entity = None;
                        } else {
                            self.dtd.param_entities[idx].public_id = None;
                            self.decl_entity = Some(EntityRefId::Param(idx));
                        }
                    } else {
                        self.decl_entity = None;
                    }
                }
                XML_ROLE_NOTATION_PUBLIC_ID => {
                    let mut bad_pos = s;
                    if !is_public_id(enc, buf, s, next, &mut bad_pos) {
                        self.set_event_ptr(is_main_enc, bad_pos);
                        return XML_ERROR_PUBLICID;
                    }
                }
                XML_ROLE_NOTATION_NAME
                | XML_ROLE_NOTATION_SYSTEM_ID
                | XML_ROLE_NOTATION_NO_SYSTEM_ID => {}
                XML_ROLE_ERROR => match tok {
                    XML_TOK_PARAM_ENTITY_REF => return XML_ERROR_PARAM_ENTITY_REF,
                    XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
                    _ => return XML_ERROR_SYNTAX,
                },
                XML_ROLE_IGNORE_SECT => {
                    let mut next_opt = Some(next);
                    let result =
                        self.do_ignore_section(enc, buf, &mut next_opt, end, next_ptr, have_more);
                    if result != XML_ERROR_NONE {
                        return result;
                    } else if let Some(new_next) = next_opt {
                        next = new_next;
                    } else {
                        self.processor = ProcessorKind::IgnoreSection;
                        return result;
                    }
                }
                XML_ROLE_GROUP_OPEN => {
                    let level = self.prolog_state.level;
                    if level >= self.group_connector.len() {
                        self.group_connector.resize(level + 1, 0);
                    }
                    self.group_connector[level] = 0;
                }
                XML_ROLE_GROUP_SEQUENCE => {
                    let level = self.prolog_state.level;
                    if self.group_connector[level] == ASCII_PIPE {
                        return XML_ERROR_SYNTAX;
                    }
                    self.group_connector[level] = ASCII_COMMA;
                }
                XML_ROLE_GROUP_CHOICE => {
                    let level = self.prolog_state.level;
                    if self.group_connector[level] == ASCII_COMMA {
                        return XML_ERROR_SYNTAX;
                    }
                    self.group_connector[level] = ASCII_PIPE;
                }
                XML_ROLE_PARAM_ENTITY_REF | XML_ROLE_INNER_PARAM_ENTITY_REF => {
                    self.dtd.has_param_entity_refs = true;
                    // m_paramEntityParsing is always XML_PARAM_ENTITY_PARSING_NEVER (0)
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
            tok = if is_main_enc {
                let t = xml_prolog_tok(&mut self.encoding, buf, s, end, &mut next);
                enc = self.encoding;
                t
            } else {
                prolog_tok(enc, buf, s, end, &mut next)
            };
        }
    }

    fn define_attribute(
        &mut self,
        el_type_idx: usize,
        att_id_idx: usize,
        is_cdata: bool,
        is_id: bool,
        value: Option<Vec<u8>>,
    ) {
        let att_name = self.dtd.attribute_ids[att_id_idx].name.clone();
        if value.is_some() || is_id {
            if self.dtd.element_types[el_type_idx]
                .default_att_for_name
                .contains_key(&att_name)
            {
                return;
            }
            if is_id && self.dtd.element_types[el_type_idx].id_att_idx.is_none() {
                self.dtd.element_types[el_type_idx].id_att_idx = Some(att_id_idx);
            }
        }
        let n_default_atts = self.dtd.element_types[el_type_idx].default_atts.len();
        self.dtd.element_types[el_type_idx]
            .default_atts
            .push(DefaultAttribute {
                att_id_idx,
                is_cdata,
                value,
            });
        if !is_cdata {
            self.dtd.attribute_ids[att_id_idx].maybe_tokenized = true;
        }
        self.dtd.element_types[el_type_idx]
            .default_att_for_name
            .entry(att_name)
            .or_insert(n_default_atts);
    }

    fn store_atts(
        &mut self,
        enc: EncId,
        buf: &[u8],
        att_str: usize,
        tag_name: &[u8],
        out_attrs: &mut Vec<(Vec<u8>, Vec<u8>)>,
        account: XmlAccount,
    ) -> u32 {
        let el_type_idx = self.dtd.get_or_insert_element_type(tag_name);
        let n_default_atts = self.dtd.element_types[el_type_idx].default_atts.len();

        let mut raw_atts = std::mem::take(&mut self.atts);
        get_atts(enc, buf, att_str, &mut raw_atts);

        let mut used_att_ids: Vec<usize> = Vec::with_capacity(raw_atts.len() + n_default_atts);

        for curr_att in &raw_atts {
            let att_name_len = name_length(enc, buf, curr_att.name);
            let att_name =
                store_string_to_vec(enc, buf, curr_att.name, curr_att.name + att_name_len);
            let att_id_idx = self.dtd.get_or_insert_attribute_id(&att_name);

            if self.dtd.attribute_ids[att_id_idx].in_use != 0 {
                if enc != EncId::InternalUtf8 {
                    self.event_ptr = Some(curr_att.name);
                }
                for &id in &used_att_ids {
                    self.dtd.attribute_ids[id].in_use = 0;
                }
                self.atts = raw_atts;
                return XML_ERROR_DUPLICATE_ATTRIBUTE;
            }
            self.dtd.attribute_ids[att_id_idx].in_use = 1;
            used_att_ids.push(att_id_idx);

            let val_vec = if !curr_att.normalized {
                let mut is_cdata = true;
                if self.dtd.attribute_ids[att_id_idx].maybe_tokenized {
                    if let Some(&def_idx) = self.dtd.element_types[el_type_idx]
                        .default_att_for_name
                        .get(&self.dtd.attribute_ids[att_id_idx].name)
                    {
                        is_cdata =
                            self.dtd.element_types[el_type_idx].default_atts[def_idx].is_cdata;
                    }
                }
                let mut pool = Vec::new();
                let result = self.store_attribute_value(
                    enc,
                    buf,
                    is_cdata,
                    curr_att.value_ptr,
                    curr_att.value_end,
                    false,
                    &mut pool,
                    account,
                );
                if result != XML_ERROR_NONE {
                    for &id in &used_att_ids {
                        self.dtd.attribute_ids[id].in_use = 0;
                    }
                    self.atts = raw_atts;
                    return result;
                }
                pool
            } else {
                store_string_to_vec(enc, buf, curr_att.value_ptr, curr_att.value_end)
            };

            out_attrs.push((self.dtd.attribute_ids[att_id_idx].name.clone(), val_vec));
        }

        for i in 0..n_default_atts {
            let da_id_idx = self.dtd.element_types[el_type_idx].default_atts[i].att_id_idx;
            if self.dtd.attribute_ids[da_id_idx].in_use == 0 {
                if let Some(ref val) = self.dtd.element_types[el_type_idx].default_atts[i].value {
                    self.dtd.attribute_ids[da_id_idx].in_use = 1;
                    used_att_ids.push(da_id_idx);
                    out_attrs.push((self.dtd.attribute_ids[da_id_idx].name.clone(), val.clone()));
                }
            }
        }

        for &id in &used_att_ids {
            self.dtd.attribute_ids[id].in_use = 0;
        }
        self.atts = raw_atts;
        XML_ERROR_NONE
    }

    #[allow(clippy::too_many_arguments)]
    fn store_attribute_value(
        &mut self,
        enc: EncId,
        buf: &[u8],
        is_cdata: bool,
        ptr: usize,
        end: usize,
        from_prolog: bool,
        pool: &mut Vec<u8>,
        account: XmlAccount,
    ) -> u32 {
        let mut next = ptr;
        let mut result;

        loop {
            if self.open_attribute_entities.is_empty() {
                result = self.append_attribute_value(
                    enc,
                    buf,
                    is_cdata,
                    next,
                    end,
                    from_prolog,
                    pool,
                    account,
                    &mut next,
                );
            } else {
                let Some(open_entity) = self.open_attribute_entities.last().cloned() else {
                    return XML_ERROR_UNEXPECTED_STATE;
                };
                let entity_id = open_entity.entity_id;
                let (has_more, processed, text_opt) = {
                    let ent = self.dtd.get_entity(entity_id);
                    (ent.has_more, ent.processed, ent.text.clone())
                };
                if has_more {
                    let text_bytes = text_opt.unwrap_or_default();
                    let text_start = processed;
                    let text_end = text_bytes.len();
                    let mut next_in_entity = text_start;
                    let internal_enc = self.internal_encoding;
                    result = self.append_attribute_value(
                        internal_enc,
                        &text_bytes,
                        is_cdata,
                        text_start,
                        text_end,
                        from_prolog,
                        pool,
                        XmlAccount::EntityExpansion,
                        &mut next_in_entity,
                    );
                    if result != XML_ERROR_NONE {
                        break;
                    }
                    if next_in_entity < text_end {
                        self.dtd.get_entity_mut(entity_id).processed = next_in_entity;
                        continue;
                    }
                    self.dtd.get_entity_mut(entity_id).has_more = false;
                    continue;
                }

                self.dtd.get_entity_mut(entity_id).open = false;
                self.open_attribute_entities.pop();
                result = XML_ERROR_NONE;
            }

            if result != XML_ERROR_NONE
                || (self.open_attribute_entities.is_empty() && end == next)
            {
                break;
            }
        }

        if result != XML_ERROR_NONE {
            return result;
        }
        if !is_cdata && pool.last() == Some(&0x20) {
            pool.pop();
        }
        XML_ERROR_NONE
    }

    #[allow(clippy::too_many_arguments)]
    fn append_attribute_value(
        &mut self,
        enc: EncId,
        buf: &[u8],
        is_cdata: bool,
        mut ptr: usize,
        end: usize,
        from_prolog: bool,
        pool: &mut Vec<u8>,
        account: XmlAccount,
        next_ptr: &mut usize,
    ) -> u32 {
        let is_main_enc = enc != EncId::InternalUtf8;
        let min_bpc = enc.min_bpc();

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
                    if is_main_enc {
                        self.event_ptr = Some(next);
                    }
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL => {
                    if is_main_enc {
                        self.event_ptr = Some(ptr);
                    }
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_CHAR_REF => {
                    let n = char_ref_number(enc, buf, ptr);
                    if n < 0 {
                        if is_main_enc {
                            self.event_ptr = Some(ptr);
                        }
                        return XML_ERROR_BAD_CHAR_REF;
                    }
                    if !is_cdata && n == 0x20 && (pool.is_empty() || pool.last() == Some(&0x20)) {
                        ptr = next;
                        continue;
                    }
                    let mut enc_buf = [0u8; 4];
                    let enc_len = xml_utf8_encode(n, &mut enc_buf);
                    pool.extend_from_slice(&enc_buf[..enc_len]);
                }
                XML_TOK_DATA_CHARS => {
                    append_converted_to_vec(enc, buf, ptr, next, pool);
                }
                XML_TOK_TRAILING_CR => {
                    next = ptr + min_bpc;
                    if !is_cdata && (pool.is_empty() || pool.last() == Some(&0x20)) {
                        ptr = next;
                        continue;
                    }
                    pool.push(0x20);
                }
                XML_TOK_ATTRIBUTE_VALUE_S | XML_TOK_DATA_NEWLINE => {
                    if !is_cdata && (pool.is_empty() || pool.last() == Some(&0x20)) {
                        ptr = next;
                        continue;
                    }
                    pool.push(0x20);
                }
                XML_TOK_ENTITY_REF => {
                    let ch = predefined_entity_name(enc, buf, ptr + min_bpc, next - min_bpc);
                    if ch != 0 {
                        self.accounting
                            .diff_tolerated(tok, 0, 1, XmlAccount::EntityExpansion);
                        pool.push(ch);
                    } else {
                        let name = store_string_to_vec(enc, buf, ptr + min_bpc, next - min_bpc);
                        let entity_idx_opt = self.dtd.lookup_general_entity(&name);
                        let check_entity_decl = if from_prolog {
                            self.prolog_state.document_entity
                                && (if self.dtd.standalone {
                                    self.open_internal_entities.is_empty()
                                } else {
                                    !self.dtd.has_param_entity_refs
                                })
                        } else {
                            !self.dtd.has_param_entity_refs || self.dtd.standalone
                        };
                        if check_entity_decl {
                            let Some(ent_idx) = entity_idx_opt else {
                                return XML_ERROR_UNDEFINED_ENTITY;
                            };
                            if !self.dtd.general_entities[ent_idx].is_internal {
                                return XML_ERROR_ENTITY_DECLARED_IN_PE;
                            }
                        } else if entity_idx_opt.is_none() {
                            ptr = next;
                            continue;
                        }
                        let ent_idx = entity_idx_opt.unwrap();
                        if self.dtd.general_entities[ent_idx].open {
                            if is_main_enc {
                                self.event_ptr = Some(ptr);
                            }
                            return XML_ERROR_RECURSIVE_ENTITY_REF;
                        }
                        if self.dtd.general_entities[ent_idx].notation.is_some() {
                            if is_main_enc {
                                self.event_ptr = Some(ptr);
                            }
                            return XML_ERROR_BINARY_ENTITY_REF;
                        }
                        if self.dtd.general_entities[ent_idx].text.is_none() {
                            if is_main_enc {
                                self.event_ptr = Some(ptr);
                            }
                            return XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                        } else {
                            let result = self.process_entity(
                                EntityRefId::General(ent_idx),
                                false,
                                EntityType::Attribute,
                            );
                            if result == XML_ERROR_NONE {
                                *next_ptr = next;
                            }
                            return result;
                        }
                    }
                }
                _ => {
                    if is_main_enc {
                        self.event_ptr = Some(ptr);
                    }
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            ptr = next;
        }
    }

    fn call_store_entity_value(
        &mut self,
        enc: EncId,
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
                let entity_id = open_entity.entity_id;
                let (has_more, processed, text_opt) = {
                    let ent = self.dtd.get_entity(entity_id);
                    (ent.has_more, ent.processed, ent.text.clone())
                };
                if has_more {
                    let text_bytes = text_opt.unwrap_or_default();
                    let text_start = processed;
                    let text_end = text_bytes.len();
                    let mut next_in_entity = text_start;
                    let internal_enc = self.internal_encoding;
                    result = self.store_entity_value(
                        internal_enc,
                        &text_bytes,
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
                        self.dtd.get_entity_mut(entity_id).processed = next_in_entity;
                        continue;
                    }
                    self.dtd.get_entity_mut(entity_id).has_more = false;
                    continue;
                }

                self.dtd.get_entity_mut(entity_id).open = false;
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
        enc: EncId,
        buf: &[u8],
        mut entity_text_ptr: usize,
        entity_text_end: usize,
        account: XmlAccount,
        pool: &mut Vec<u8>,
        next_ptr: &mut usize,
    ) -> u32 {
        let old_in_entity_value = self.prolog_state.in_entity_value;
        self.prolog_state.in_entity_value = true;
        let is_main_enc = enc != EncId::InternalUtf8;
        let min_bpc = enc.min_bpc();
        let mut next = entity_text_ptr;
        let result: u32;

        if entity_text_ptr >= entity_text_end {
            self.prolog_state.in_entity_value = old_in_entity_value;
            *next_ptr = next;
            return XML_ERROR_NONE;
        }

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
                    if is_main_enc {
                        self.event_ptr = Some(entity_text_ptr);
                    }
                    result = XML_ERROR_PARAM_ENTITY_REF;
                    break;
                }
                XML_TOK_NONE => {
                    result = XML_ERROR_NONE;
                    break;
                }
                XML_TOK_ENTITY_REF | XML_TOK_DATA_CHARS => {
                    append_converted_to_vec(enc, buf, entity_text_ptr, next, pool);
                }
                XML_TOK_TRAILING_CR => {
                    next = entity_text_ptr + min_bpc;
                    pool.push(0x0A);
                }
                XML_TOK_DATA_NEWLINE => {
                    pool.push(0x0A);
                }
                XML_TOK_CHAR_REF => {
                    let n = char_ref_number(enc, buf, entity_text_ptr);
                    if n < 0 {
                        if is_main_enc {
                            self.event_ptr = Some(entity_text_ptr);
                        }
                        result = XML_ERROR_BAD_CHAR_REF;
                        break;
                    }
                    let mut enc_buf = [0u8; 4];
                    let enc_len = xml_utf8_encode(n, &mut enc_buf);
                    pool.extend_from_slice(&enc_buf[..enc_len]);
                }
                XML_TOK_PARTIAL => {
                    if is_main_enc {
                        self.event_ptr = Some(entity_text_ptr);
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                XML_TOK_INVALID => {
                    if is_main_enc {
                        self.event_ptr = Some(next);
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                _ => {
                    if is_main_enc {
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

    fn report_processing_instruction(
        &mut self,
        enc: EncId,
        buf: &[u8],
        mut start: usize,
        end: usize,
    ) {
        let min_bpc = enc.min_bpc();
        start += min_bpc * 2;
        let tem = start + name_length(enc, buf, start);
        let target = store_string_to_vec(enc, buf, start, tem);
        let data_start = skip_s(enc, buf, tem);
        let mut data = store_string_to_vec(enc, buf, data_start, end - min_bpc * 2);
        normalize_lines(&mut data);
        self.events.push(Event::Pi { target, data });
    }

    fn report_comment(&mut self, enc: EncId, buf: &[u8], start: usize, end: usize) {
        let min_bpc = enc.min_bpc();
        let mut data = store_string_to_vec(enc, buf, start + min_bpc * 4, end - min_bpc * 3);
        normalize_lines(&mut data);
        self.events.push(Event::Comment(data));
    }
}

fn normalize_public_id(pub_id: &mut Vec<u8>) {
    let mut out = Vec::with_capacity(pub_id.len());
    for &b in pub_id.iter() {
        match b {
            0x20 | 0x0D | 0x0A => {
                if !out.is_empty() && out.last() != Some(&0x20) {
                    out.push(0x20);
                }
            }
            _ => out.push(b),
        }
    }
    if out.last() == Some(&0x20) {
        out.pop();
    }
    *pub_id = out;
}

fn normalize_lines(data: &mut Vec<u8>) {
    if !data.contains(&0x0D) {
        return;
    }
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        if data[i] == 0x0D {
            out.push(0x0A);
            i += 1;
            if i < data.len() && data[i] == 0x0A {
                i += 1;
            }
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    *data = out;
}

pub fn parse(input: &[u8]) -> Result<Vec<Event>, XmlError> {
    let mut parser = Parser::new(input);
    let error_code = parser.call_processor(0, input.len());
    if error_code != XML_ERROR_NONE {
        parser.error_code = error_code;
        parser.event_end_ptr = parser.event_ptr;
        parser.processor = ProcessorKind::Error;
        let (line, col) = parser.get_current_line_and_col();
        Err(XmlError {
            code: error_code,
            line,
            col,
        })
    } else {
        Ok(parser.events)
    }
}
