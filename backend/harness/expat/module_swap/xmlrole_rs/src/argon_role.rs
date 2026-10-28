// AUTO-EXTRACTED from Argon's port, expat.rs sha256
// 418570361191d0019294b981d80e3185c7a732e0d4c02a627107f60b5af72c0d
// by scripts/extract_argon_xmlrole.py. Do not edit by hand.

// --- Argon expat.rs XML_TOK_* / XML_ROLE_* constants (verbatim)
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

// --- Argon expat.rs lines 723-732 (verbatim)
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

// --- Argon expat.rs lines 734-743 (verbatim)
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

impl Encoding { // extractor: re-opened impl holding 3 of Argon's methods
// --- Argon expat.rs lines 806-812 (verbatim)
    #[inline]
    fn min_bytes_per_char(self) -> usize {
        match self {
            Encoding::Little2 | Encoding::Big2 => 2,
            _ => 1,
        }
    }

// --- Argon expat.rs lines 859-866 (verbatim)
    #[inline]
    fn char_matches(self, buf: &[u8], p: usize, c: u8) -> bool {
        match self {
            Encoding::Little2 => buf[p + 1] == 0 && buf[p] == c,
            Encoding::Big2 => buf[p] == 0 && buf[p + 1] == c,
            _ => buf[p] == c,
        }
    }

// --- Argon expat.rs lines 1039-1051 (verbatim)
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

} // extractor: end of re-opened impl

// --- Argon expat.rs lines 3515-4329 (verbatim)
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

