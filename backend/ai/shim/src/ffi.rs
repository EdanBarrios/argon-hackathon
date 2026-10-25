#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_void, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::slice;

use crate::expat::{self, Event};

pub type XML_Char = c_char;
pub type XML_LChar = c_char;
pub type XML_Index = c_long;
pub type XML_Size = c_ulong;
pub type XML_Bool = c_uchar;

pub const XML_TRUE: XML_Bool = 1;
pub const XML_FALSE: XML_Bool = 0;

pub type XML_Status = c_int;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;

pub type XML_Error = c_int;
pub const XML_ERROR_NONE: XML_Error = 0;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NOT_STARTED: XML_Error = 44;

pub type XML_Prop_Error = c_int;
pub const XML_PROP_ERROR_NONE: XML_Prop_Error = 0;
pub const XML_PROP_ERROR_INVALID_KEY: XML_Prop_Error = 1;
pub const XML_PROP_ERROR_INVALID_TYPE: XML_Prop_Error = 2;
pub const XML_PROP_ERROR_INVALID_VALUE: XML_Prop_Error = 3;
pub const XML_PROP_ERROR_PARSER_NULL: XML_Prop_Error = 4;
pub const XML_PROP_ERROR_PARSER_NOT_ROOT: XML_Prop_Error = 5;

pub type XML_Content_Type = c_int;
pub type XML_Content_Quant = c_int;
pub type XML_Parsing = c_int;
pub const XML_INITIALIZED: XML_Parsing = 0;
pub const XML_PARSING: XML_Parsing = 1;
pub const XML_FINISHED: XML_Parsing = 2;
pub const XML_SUSPENDED: XML_Parsing = 3;

pub type XML_ParamEntityParsing = c_int;
pub type XML_Parser_Property = c_int;
pub type XML_FeatureEnum = c_int;

pub const XML_FEATURE_END: XML_FeatureEnum = 0;
pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;
pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;
pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;
pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;
pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;
pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;
pub const XML_FEATURE_NS: XML_FeatureEnum = 8;
pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;
pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 11;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 12;
pub const XML_FEATURE_GE: XML_FeatureEnum = 13;
pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 14;
pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 15;

#[repr(C)]
pub struct XML_cp {
    pub type_: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *mut XML_Char,
    pub numchildren: c_uint,
    pub children: *mut XML_Content,
}
pub type XML_Content = XML_cp;

#[repr(C)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(usize) -> *mut c_void>,
    pub realloc_fcn: Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct XML_Encoding {
    pub map: [c_int; 256],
    pub data: *mut c_void,
    pub convert: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct XML_ParsingStatus {
    pub parsing: XML_Parsing,
    pub finalBuffer: XML_Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XML_Expat_Version {
    pub major: c_int,
    pub minor: c_int,
    pub micro: c_int,
}

#[repr(C)]
pub struct XML_Feature {
    pub feature: XML_FeatureEnum,
    pub name: *const XML_LChar,
    pub value: c_long,
}
unsafe impl Sync for XML_Feature {}

pub type XML_Parser = *mut XML_ParserStruct;

pub type XML_ElementDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Content)>;
pub type XML_AttlistDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        c_int,
    ),
>;
pub type XML_XmlDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, c_int)>;
pub type XML_StartElementHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const *const XML_Char)>;
pub type XML_EndElementHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
pub type XML_CharacterDataHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
pub type XML_CommentHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
pub type XML_StartCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void)>;
pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void)>;
pub type XML_DefaultHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, *const XML_Char, c_int),
>;
pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut c_void)>;
pub type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        c_int,
        *const XML_Char,
        c_int,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
pub type XML_UnparsedEntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
pub type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
pub type XML_StartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
pub type XML_EndNamespaceDeclHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
pub type XML_NotStandaloneHandler = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> c_int,
>;
pub type XML_SkippedEntityHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
pub type XML_UnknownEncodingHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Encoding) -> c_int>;

#[repr(C)]
pub struct XML_ParserStruct {
    // Must be first field so `#define XML_GetUserData(parser) (*(void **)(parser))` works!
    pub user_data: *mut c_void,
    pub use_parser_as_arg: bool,
    pub ns_sep: Option<XML_Char>,
    pub ns_triplets: bool,

    pub buffer: Vec<u8>,
    pub get_buffer: Vec<u8>,
    pub base: Option<Vec<u8>>,
    pub encoding: Option<Vec<u8>>,

    pub error_code: XML_Error,
    pub error_line: u64,
    pub error_col: u64,
    pub has_error: bool,

    pub stopped: bool,
    pub suspended: bool,
    pub parsing_state: XML_Parsing,
    pub final_buffer: XML_Bool,
    pub specified_attr_count: c_int,

    pub start_element_handler: XML_StartElementHandler,
    pub end_element_handler: XML_EndElementHandler,
    pub char_data_handler: XML_CharacterDataHandler,
    pub pi_handler: XML_ProcessingInstructionHandler,
    pub comment_handler: XML_CommentHandler,
    pub start_cdata_handler: XML_StartCdataSectionHandler,
    pub end_cdata_handler: XML_EndCdataSectionHandler,
    pub default_handler: XML_DefaultHandler,
    pub start_doctype_handler: XML_StartDoctypeDeclHandler,
    pub end_doctype_handler: XML_EndDoctypeDeclHandler,
    pub entity_decl_handler: XML_EntityDeclHandler,
    pub unparsed_entity_decl_handler: XML_UnparsedEntityDeclHandler,
    pub notation_decl_handler: XML_NotationDeclHandler,
    pub start_ns_decl_handler: XML_StartNamespaceDeclHandler,
    pub end_ns_decl_handler: XML_EndNamespaceDeclHandler,
    pub not_standalone_handler: XML_NotStandaloneHandler,
    pub external_entity_ref_handler: XML_ExternalEntityRefHandler,
    pub external_entity_ref_handler_arg: *mut c_void,
    pub skipped_entity_handler: XML_SkippedEntityHandler,
    pub unknown_encoding_handler: XML_UnknownEncodingHandler,
    pub unknown_encoding_handler_data: *mut c_void,
    pub element_decl_handler: XML_ElementDeclHandler,
    pub attlist_decl_handler: XML_AttlistDeclHandler,
    pub xml_decl_handler: XML_XmlDeclHandler,

    pub param_entity_parsing: XML_ParamEntityParsing,
    pub use_foreign_dtd: XML_Bool,
    pub reparse_deferral: XML_Bool,
    pub hash_salt: c_ulong,
}

impl XML_ParserStruct {
    fn new(encoding: *const XML_Char, ns_sep: Option<XML_Char>) -> Self {
        let enc = unsafe { cstr_to_vec(encoding) };
        Self {
            user_data: ptr::null_mut(),
            use_parser_as_arg: false,
            ns_sep,
            ns_triplets: false,
            buffer: Vec::new(),
            get_buffer: Vec::new(),
            base: None,
            encoding: enc,
            error_code: XML_ERROR_NONE,
            error_line: 1,
            error_col: 0,
            has_error: false,
            stopped: false,
            suspended: false,
            parsing_state: XML_INITIALIZED,
            final_buffer: XML_FALSE,
            specified_attr_count: 0,
            start_element_handler: None,
            end_element_handler: None,
            char_data_handler: None,
            pi_handler: None,
            comment_handler: None,
            start_cdata_handler: None,
            end_cdata_handler: None,
            default_handler: None,
            start_doctype_handler: None,
            end_doctype_handler: None,
            entity_decl_handler: None,
            unparsed_entity_decl_handler: None,
            notation_decl_handler: None,
            start_ns_decl_handler: None,
            end_ns_decl_handler: None,
            not_standalone_handler: None,
            external_entity_ref_handler: None,
            external_entity_ref_handler_arg: ptr::null_mut(),
            skipped_entity_handler: None,
            unknown_encoding_handler: None,
            unknown_encoding_handler_data: ptr::null_mut(),
            element_decl_handler: None,
            attlist_decl_handler: None,
            xml_decl_handler: None,
            param_entity_parsing: 0,
            use_foreign_dtd: XML_FALSE,
            reparse_deferral: XML_TRUE,
            hash_salt: 0,
        }
    }

    #[inline]
    fn handler_arg(&mut self) -> *mut c_void {
        if self.use_parser_as_arg {
            self as *mut Self as *mut c_void
        } else {
            self.user_data
        }
    }

    #[inline]
    fn base_ptr(&self) -> *const XML_Char {
        match &self.base {
            Some(b) => b.as_ptr() as *const XML_Char,
            None => ptr::null(),
        }
    }
}

unsafe fn cstr_to_vec(p: *const XML_Char) -> Option<Vec<u8>> {
    if p.is_null() {
        None
    } else {
        let bytes = CStr::from_ptr(p).to_bytes_with_nul().to_vec();
        Some(bytes)
    }
}

#[inline]
fn to_c_buf(bytes: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(bytes.len() + 1);
    v.extend_from_slice(bytes);
    v.push(0);
    v
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser {
    Box::into_raw(Box::new(XML_ParserStruct::new(encoding, None)))
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreateNS(
    encoding: *const XML_Char,
    namespaceSeparator: XML_Char,
) -> XML_Parser {
    Box::into_raw(Box::new(XML_ParserStruct::new(
        encoding,
        Some(namespaceSeparator),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate_MM(
    encoding: *const XML_Char,
    _memsuite: *const XML_Memory_Handling_Suite,
    namespaceSeparator: *const XML_Char,
) -> XML_Parser {
    let ns_sep = if namespaceSeparator.is_null() {
        None
    } else {
        Some(*namespaceSeparator)
    };
    Box::into_raw(Box::new(XML_ParserStruct::new(encoding, ns_sep)))
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserReset(
    parser: XML_Parser,
    encoding: *const XML_Char,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    let p = &mut *parser;
    let ns_sep = p.ns_sep;
    let ns_triplets = p.ns_triplets;
    let unk_handler = p.unknown_encoding_handler;
    let unk_data = p.unknown_encoding_handler_data;
    *p = XML_ParserStruct::new(encoding, ns_sep);
    p.ns_triplets = ns_triplets;
    p.unknown_encoding_handler = unk_handler;
    p.unknown_encoding_handler_data = unk_data;
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserFree(parser: XML_Parser) {
    if !parser.is_null() {
        drop(Box::from_raw(parser));
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetElementDeclHandler(
    parser: XML_Parser,
    eldecl: XML_ElementDeclHandler,
) {
    if !parser.is_null() {
        (*parser).element_decl_handler = eldecl;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    parser: XML_Parser,
    attdecl: XML_AttlistDeclHandler,
) {
    if !parser.is_null() {
        (*parser).attlist_decl_handler = attdecl;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetXmlDeclHandler(parser: XML_Parser, xmldecl: XML_XmlDeclHandler) {
    if !parser.is_null() {
        (*parser).xml_decl_handler = xmldecl;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    parser: XML_Parser,
    handler: XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).entity_decl_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetElementHandler(
    parser: XML_Parser,
    start: XML_StartElementHandler,
    end: XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).start_element_handler = start;
        (*parser).end_element_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartElementHandler(
    parser: XML_Parser,
    handler: XML_StartElementHandler,
) {
    if !parser.is_null() {
        (*parser).start_element_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndElementHandler(
    parser: XML_Parser,
    handler: XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).end_element_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    parser: XML_Parser,
    handler: XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        (*parser).char_data_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    parser: XML_Parser,
    handler: XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        (*parser).pi_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(parser: XML_Parser, handler: XML_CommentHandler) {
    if !parser.is_null() {
        (*parser).comment_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    parser: XML_Parser,
    start: XML_StartCdataSectionHandler,
    end: XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).start_cdata_handler = start;
        (*parser).end_cdata_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    parser: XML_Parser,
    start: XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).start_cdata_handler = start;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    parser: XML_Parser,
    end: XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).end_cdata_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler) {
    if !parser.is_null() {
        (*parser).default_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    parser: XML_Parser,
    handler: XML_DefaultHandler,
) {
    if !parser.is_null() {
        (*parser).default_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    parser: XML_Parser,
    start: XML_StartDoctypeDeclHandler,
    end: XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).start_doctype_handler = start;
        (*parser).end_doctype_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    parser: XML_Parser,
    start: XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).start_doctype_handler = start;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    parser: XML_Parser,
    end: XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).end_doctype_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    parser: XML_Parser,
    handler: XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).unparsed_entity_decl_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    parser: XML_Parser,
    handler: XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        (*parser).notation_decl_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    parser: XML_Parser,
    start: XML_StartNamespaceDeclHandler,
    end: XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).start_ns_decl_handler = start;
        (*parser).end_ns_decl_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    parser: XML_Parser,
    start: XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).start_ns_decl_handler = start;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    parser: XML_Parser,
    end: XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).end_ns_decl_handler = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    parser: XML_Parser,
    handler: XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        (*parser).not_standalone_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    parser: XML_Parser,
    handler: XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        (*parser).external_entity_ref_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(parser: XML_Parser, arg: *mut c_void) {
    if !parser.is_null() {
        (*parser).external_entity_ref_handler_arg = arg;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    parser: XML_Parser,
    handler: XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        (*parser).skipped_entity_handler = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    parser: XML_Parser,
    handler: XML_UnknownEncodingHandler,
    encodingHandlerData: *mut c_void,
) {
    if !parser.is_null() {
        (*parser).unknown_encoding_handler = handler;
        (*parser).unknown_encoding_handler_data = encodingHandlerData;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_DefaultCurrent(_parser: XML_Parser) {}

#[no_mangle]
pub unsafe extern "C" fn XML_SetReturnNSTriplet(parser: XML_Parser, do_nst: c_int) {
    if !parser.is_null() {
        (*parser).ns_triplets = do_nst != 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUserData(parser: XML_Parser, userData: *mut c_void) {
    if !parser.is_null() {
        (*parser).user_data = userData;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEncoding(
    parser: XML_Parser,
    encoding: *const XML_Char,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    (*parser).encoding = cstr_to_vec(encoding);
    XML_STATUS_OK
}

#[no_mangle]
pub unsafe extern "C" fn XML_UseParserAsHandlerArg(parser: XML_Parser) {
    if !parser.is_null() {
        (*parser).use_parser_as_arg = true;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_UseForeignDTD(parser: XML_Parser, useDTD: XML_Bool) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    (*parser).use_foreign_dtd = useDTD;
    XML_ERROR_NONE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    (*parser).base = cstr_to_vec(base);
    XML_STATUS_OK
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBase(parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return ptr::null();
    }
    (*parser).base_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -1;
    }
    (*parser).specified_attr_count
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetIdAttributeIndex(_parser: XML_Parser) -> c_int {
    -1
}

fn has_ns_markup(events: &[Event]) -> bool {
    for ev in events {
        if let Event::Start { name, attrs } = ev {
            if name.contains(&b':') {
                return true;
            }
            for (k, _) in attrs {
                if k.as_slice() == b"xmlns"
                    || k.starts_with(b"xmlns:")
                    || k.contains(&b':')
                {
                    return true;
                }
            }
        }
    }
    false
}

unsafe fn run_final_parse(p: &mut XML_ParserStruct) -> XML_Status {
    p.parsing_state = XML_PARSING;
    p.final_buffer = XML_TRUE;

    let input_slice = p.buffer.as_slice();
    let parse_res = catch_unwind(AssertUnwindSafe(|| expat::parse(input_slice)));

    let events = match parse_res {
        Ok(Ok(evs)) => evs,
        Ok(Err(err)) => {
            p.error_code = err.code as XML_Error;
            p.error_line = err.line;
            p.error_col = err.col;
            p.has_error = true;
            p.parsing_state = XML_FINISHED;
            return XML_STATUS_ERROR;
        }
        Err(_) => {
            p.error_code = XML_ERROR_UNEXPECTED_STATE;
            p.error_line = 1;
            p.error_col = 0;
            p.has_error = true;
            p.parsing_state = XML_FINISHED;
            return XML_STATUS_ERROR;
        }
    };

    if p.ns_sep.is_some() && has_ns_markup(&events) {
        p.error_code = XML_ERROR_UNBOUND_PREFIX;
        p.error_line = 1;
        p.error_col = 0;
        p.has_error = true;
        p.parsing_state = XML_FINISHED;
        return XML_STATUS_ERROR;
    }

    for ev in events {
        if p.stopped {
            break;
        }
        match ev {
            Event::XmlDecl {
                version,
                encoding,
                standalone,
            } => {
                if let Some(cb) = p.xml_decl_handler {
                    let v_c = version.as_ref().map(|b| to_c_buf(b));
                    let e_c = encoding.as_ref().map(|b| to_c_buf(b));
                    let v_ptr = v_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let e_ptr = e_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let arg = p.handler_arg();
                    cb(arg, v_ptr, e_ptr, standalone as c_int);
                }
            }
            Event::DoctypeStart {
                name,
                sysid,
                pubid,
                has_internal_subset,
            } => {
                if let Some(cb) = p.start_doctype_handler {
                    let name_c = to_c_buf(&name);
                    let sys_c = sysid.as_ref().map(|b| to_c_buf(b));
                    let pub_c = pubid.as_ref().map(|b| to_c_buf(b));
                    let sys_ptr = sys_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let pub_ptr = pub_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let arg = p.handler_arg();
                    cb(
                        arg,
                        name_c.as_ptr() as *const XML_Char,
                        sys_ptr,
                        pub_ptr,
                        if has_internal_subset { 1 } else { 0 },
                    );
                }
            }
            Event::DoctypeEnd => {
                if let Some(cb) = p.end_doctype_handler {
                    let arg = p.handler_arg();
                    cb(arg);
                }
            }
            Event::EntityDecl {
                name,
                is_pe,
                value,
                sysid,
                pubid,
                notation,
            } => {
                if let Some(cb) = p.entity_decl_handler {
                    let name_c = to_c_buf(&name);
                    let sys_c = sysid.as_ref().map(|b| to_c_buf(b));
                    let pub_c = pubid.as_ref().map(|b| to_c_buf(b));
                    let not_c = notation.as_ref().map(|b| to_c_buf(b));
                    let (val_ptr, val_len) = match &value {
                        Some(v) => (v.as_ptr() as *const XML_Char, v.len() as c_int),
                        None => (ptr::null(), 0),
                    };
                    let sys_ptr = sys_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let pub_ptr = pub_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let not_ptr = not_c
                        .as_ref()
                        .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                    let base_ptr = p.base_ptr();
                    let arg = p.handler_arg();
                    cb(
                        arg,
                        name_c.as_ptr() as *const XML_Char,
                        if is_pe { 1 } else { 0 },
                        val_ptr,
                        val_len,
                        base_ptr,
                        sys_ptr,
                        pub_ptr,
                        not_ptr,
                    );
                }
            }
            Event::Start { name, attrs } => {
                p.specified_attr_count = (attrs.len() * 2) as c_int;
                if let Some(cb) = p.start_element_handler {
                    let name_c = to_c_buf(&name);
                    let mut attr_bufs: Vec<Vec<u8>> = Vec::with_capacity(attrs.len() * 2);
                    for (k, v) in &attrs {
                        attr_bufs.push(to_c_buf(k));
                        attr_bufs.push(to_c_buf(v));
                    }
                    let mut attr_ptrs: Vec<*const XML_Char> =
                        Vec::with_capacity(attr_bufs.len() + 1);
                    for b in &attr_bufs {
                        attr_ptrs.push(b.as_ptr() as *const XML_Char);
                    }
                    attr_ptrs.push(ptr::null());
                    let arg = p.handler_arg();
                    cb(
                        arg,
                        name_c.as_ptr() as *const XML_Char,
                        attr_ptrs.as_ptr(),
                    );
                }
            }
            Event::End { name } => {
                if let Some(cb) = p.end_element_handler {
                    let name_c = to_c_buf(&name);
                    let arg = p.handler_arg();
                    cb(arg, name_c.as_ptr() as *const XML_Char);
                }
            }
            Event::Text(data) => {
                if let Some(cb) = p.char_data_handler {
                    let arg = p.handler_arg();
                    cb(arg, data.as_ptr() as *const XML_Char, data.len() as c_int);
                }
            }
            Event::Pi { target, data } => {
                if let Some(cb) = p.pi_handler {
                    let t_c = to_c_buf(&target);
                    let d_c = to_c_buf(&data);
                    let arg = p.handler_arg();
                    cb(
                        arg,
                        t_c.as_ptr() as *const XML_Char,
                        d_c.as_ptr() as *const XML_Char,
                    );
                }
            }
            Event::Comment(data) => {
                if let Some(cb) = p.comment_handler {
                    let d_c = to_c_buf(&data);
                    let arg = p.handler_arg();
                    cb(arg, d_c.as_ptr() as *const XML_Char);
                }
            }
            Event::CdataStart => {
                if let Some(cb) = p.start_cdata_handler {
                    let arg = p.handler_arg();
                    cb(arg);
                }
            }
            Event::CdataEnd => {
                if let Some(cb) = p.end_cdata_handler {
                    let arg = p.handler_arg();
                    cb(arg);
                }
            }
            Event::Skipped { name, is_pe } => {
                if let Some(cb) = p.skipped_entity_handler {
                    let name_c = to_c_buf(&name);
                    let arg = p.handler_arg();
                    cb(
                        arg,
                        name_c.as_ptr() as *const XML_Char,
                        if is_pe { 1 } else { 0 },
                    );
                }
            }
        }
        if p.stopped {
            break;
        }
    }

    if p.stopped {
        if p.suspended {
            p.parsing_state = XML_SUSPENDED;
            return XML_STATUS_SUSPENDED;
        } else {
            p.error_code = XML_ERROR_ABORTED;
            p.has_error = true;
            p.parsing_state = XML_FINISHED;
            return XML_STATUS_ERROR;
        }
    }

    p.parsing_state = XML_FINISHED;
    XML_STATUS_OK
}

#[no_mangle]
pub unsafe extern "C" fn XML_Parse(
    parser: XML_Parser,
    s: *const c_char,
    len: c_int,
    isFinal: c_int,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let p = &mut *parser;
    if len < 0 {
        p.error_code = XML_ERROR_INVALID_ARGUMENT;
        p.has_error = true;
        return XML_STATUS_ERROR;
    }
    if len > 0 {
        if s.is_null() {
            p.error_code = XML_ERROR_INVALID_ARGUMENT;
            p.has_error = true;
            return XML_STATUS_ERROR;
        }
        let slice = slice::from_raw_parts(s as *const u8, len as usize);
        p.buffer.extend_from_slice(slice);
    }
    if isFinal == 0 {
        p.parsing_state = XML_PARSING;
        p.final_buffer = XML_FALSE;
        return XML_STATUS_OK;
    }
    run_final_parse(p)
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(parser: XML_Parser, len: c_int) -> *mut c_void {
    if parser.is_null() || len < 0 {
        return ptr::null_mut();
    }
    let p = &mut *parser;
    let needed = (len as usize).max(1);
    p.get_buffer.resize(needed, 0);
    p.get_buffer.as_mut_ptr() as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParseBuffer(
    parser: XML_Parser,
    len: c_int,
    isFinal: c_int,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let p = &mut *parser;
    if len < 0 || (len as usize) > p.get_buffer.len() {
        p.error_code = XML_ERROR_INVALID_ARGUMENT;
        p.has_error = true;
        return XML_STATUS_ERROR;
    }
    if len > 0 {
        p.buffer.extend_from_slice(&p.get_buffer[..len as usize]);
    }
    if isFinal == 0 {
        p.parsing_state = XML_PARSING;
        p.final_buffer = XML_FALSE;
        return XML_STATUS_OK;
    }
    run_final_parse(p)
}

#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let p = &mut *parser;
    p.stopped = true;
    if resumable != 0 {
        p.suspended = true;
    } else {
        p.suspended = false;
        p.error_code = XML_ERROR_ABORTED;
        p.has_error = true;
    }
    XML_STATUS_OK
}

#[no_mangle]
pub unsafe extern "C" fn XML_ResumeParser(parser: XML_Parser) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let p = &mut *parser;
    if !p.suspended {
        p.error_code = XML_ERROR_NOT_SUSPENDED;
        p.has_error = true;
        return XML_STATUS_ERROR;
    }
    p.suspended = false;
    p.stopped = false;
    XML_STATUS_OK
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetParsingStatus(
    parser: XML_Parser,
    status: *mut XML_ParsingStatus,
) {
    if parser.is_null() || status.is_null() {
        return;
    }
    let p = &*parser;
    (*status).parsing = p.parsing_state;
    (*status).finalBuffer = p.final_buffer;
}

#[no_mangle]
pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    _parser: XML_Parser,
    _context: *const XML_Char,
    _encoding: *const XML_Char,
) -> XML_Parser {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetParamEntityParsing(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> c_int {
    if parser.is_null() {
        return 0;
    }
    (*parser).param_entity_parsing = parsing;
    1
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetHashSalt(parser: XML_Parser, hash_salt: c_ulong) -> c_int {
    if parser.is_null() {
        return 0;
    }
    (*parser).hash_salt = hash_salt;
    1
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetHashSalt16Bytes(
    parser: XML_Parser,
    _entropy: *const u8,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    (*parser).error_code
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0;
    }
    (*parser).error_line as XML_Size
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentLineNumber64(parser: XML_Parser) -> u64 {
    if parser.is_null() {
        return 0;
    }
    (*parser).error_line
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0;
    }
    (*parser).error_col as XML_Size
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber64(parser: XML_Parser) -> u64 {
    if parser.is_null() {
        return 0;
    }
    (*parser).error_col
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(parser: XML_Parser) -> XML_Index {
    let _ = parser;
    -1
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex64(parser: XML_Parser) -> i64 {
    let _ = parser;
    -1
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount(parser: XML_Parser) -> c_int {
    let _ = parser;
    0
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount64(parser: XML_Parser) -> u64 {
    let _ = parser;
    0
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetInputContext(
    _parser: XML_Parser,
    _offset: *mut c_int,
    _size: *mut c_int,
) -> *const c_char {
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetInputContext64(
    _parser: XML_Parser,
    _offset: *mut i64,
    _size: *mut u64,
) -> *const c_char {
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorLineNumber(parser: XML_Parser) -> XML_Size {
    XML_GetCurrentLineNumber(parser)
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorColumnNumber(parser: XML_Parser) -> XML_Size {
    XML_GetCurrentColumnNumber(parser)
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorByteIndex(parser: XML_Parser) -> XML_Index {
    XML_GetCurrentByteIndex(parser)
}

#[no_mangle]
pub unsafe extern "C" fn XML_FreeContentModel(_parser: XML_Parser, _model: *mut XML_Content) {}

#[no_mangle]
pub unsafe extern "C" fn XML_MemMalloc(_parser: XML_Parser, size: usize) -> *mut c_void {
    if size == 0 {
        return ptr::null_mut();
    }
    let layout = match std::alloc::Layout::from_size_align(size + 16, 16) {
        Ok(l) => l,
        Err(_) => return ptr::null_mut(),
    };
    let raw = std::alloc::alloc(layout);
    if raw.is_null() {
        return ptr::null_mut();
    }
    *(raw as *mut usize) = size;
    raw.add(16) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemRealloc(
    parser: XML_Parser,
    ptr: *mut c_void,
    size: usize,
) -> *mut c_void {
    if ptr.is_null() {
        return XML_MemMalloc(parser, size);
    }
    if size == 0 {
        XML_MemFree(parser, ptr);
        return ptr::null_mut();
    }
    let base = (ptr as *mut u8).sub(16);
    let old_size = *(base as *mut usize);
    let old_layout = match std::alloc::Layout::from_size_align(old_size + 16, 16) {
        Ok(l) => l,
        Err(_) => return ptr::null_mut(),
    };
    let new_raw = std::alloc::realloc(base, old_layout, size + 16);
    if new_raw.is_null() {
        return ptr::null_mut();
    }
    *(new_raw as *mut usize) = size;
    new_raw.add(16) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemFree(_parser: XML_Parser, ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let base = (ptr as *mut u8).sub(16);
    let size = *(base as *mut usize);
    if let Ok(layout) = std::alloc::Layout::from_size_align(size + 16, 16) {
        std::alloc::dealloc(base, layout);
    }
}

#[no_mangle]
pub extern "C" fn XML_ErrorString(code: XML_Error) -> *const XML_LChar {
    let s: &'static [u8] = match code {
        XML_ERROR_NONE => return ptr::null(),
        XML_ERROR_NO_MEMORY => b"out of memory\0",
        XML_ERROR_SYNTAX => b"syntax error\0",
        XML_ERROR_NO_ELEMENTS => b"no element found\0",
        XML_ERROR_INVALID_TOKEN => b"not well-formed (invalid token)\0",
        XML_ERROR_UNCLOSED_TOKEN => b"unclosed token\0",
        XML_ERROR_PARTIAL_CHAR => b"partial character\0",
        XML_ERROR_TAG_MISMATCH => b"mismatched tag\0",
        XML_ERROR_DUPLICATE_ATTRIBUTE => b"duplicate attribute\0",
        XML_ERROR_JUNK_AFTER_DOC_ELEMENT => b"junk after document element\0",
        XML_ERROR_PARAM_ENTITY_REF => b"illegal parameter entity reference\0",
        XML_ERROR_UNDEFINED_ENTITY => b"undefined entity\0",
        XML_ERROR_RECURSIVE_ENTITY_REF => b"recursive entity reference\0",
        XML_ERROR_ASYNC_ENTITY => b"asynchronous entity\0",
        XML_ERROR_BAD_CHAR_REF => b"reference to invalid character number\0",
        XML_ERROR_BINARY_ENTITY_REF => b"reference to binary entity\0",
        XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF => b"reference to external entity in attribute\0",
        XML_ERROR_MISPLACED_XML_PI => b"XML or text declaration not at start of entity\0",
        XML_ERROR_UNKNOWN_ENCODING => b"unknown encoding\0",
        XML_ERROR_INCORRECT_ENCODING => b"encoding specified in XML declaration is incorrect\0",
        XML_ERROR_UNCLOSED_CDATA_SECTION => b"unclosed CDATA section\0",
        XML_ERROR_EXTERNAL_ENTITY_HANDLING => b"error in processing external entity reference\0",
        XML_ERROR_NOT_STANDALONE => b"document is not standalone\0",
        XML_ERROR_UNEXPECTED_STATE => b"unexpected parser state - please send a bug report\0",
        XML_ERROR_ENTITY_DECLARED_IN_PE => b"entity declared in parameter entity\0",
        XML_ERROR_FEATURE_REQUIRES_XML_DTD => b"requested feature requires XML_DTD support in Expat\0",
        XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING => b"cannot change setting once parsing has begun\0",
        XML_ERROR_UNBOUND_PREFIX => b"unbound prefix\0",
        XML_ERROR_UNDECLARING_PREFIX => b"must not undeclare prefix\0",
        XML_ERROR_INCOMPLETE_PE => b"incomplete markup in parameter entity\0",
        XML_ERROR_XML_DECL => b"XML declaration not well-formed\0",
        XML_ERROR_TEXT_DECL => b"text declaration not well-formed\0",
        XML_ERROR_PUBLICID => b"illegal character(s) in public id\0",
        XML_ERROR_SUSPENDED => b"parser suspended\0",
        XML_ERROR_NOT_SUSPENDED => b"parser not suspended\0",
        XML_ERROR_ABORTED => b"parsing aborted\0",
        XML_ERROR_FINISHED => b"parsing finished\0",
        XML_ERROR_SUSPEND_PE => b"cannot suspend in external parameter entity\0",
        XML_ERROR_RESERVED_PREFIX_XML => b"reserved prefix (xml) must not be undeclared or bound to another namespace name\0",
        XML_ERROR_RESERVED_PREFIX_XMLNS => b"reserved prefix (xmlns) must not be declared or undeclared\0",
        XML_ERROR_RESERVED_NAMESPACE_URI => b"prefix must not be bound to one of the reserved namespace names\0",
        XML_ERROR_INVALID_ARGUMENT => b"invalid argument\0",
        XML_ERROR_NO_BUFFER => b"a successful prior call to function XML_GetBuffer is required\0",
        XML_ERROR_AMPLIFICATION_LIMIT_BREACH => b"limit on input amplification factor (from DTD and entities) breached\0",
        XML_ERROR_NOT_STARTED => b"parser not started\0",
        _ => return ptr::null(),
    };
    s.as_ptr() as *const XML_LChar
}

#[no_mangle]
pub extern "C" fn XML_ExpatVersion() -> *const XML_LChar {
    b"expat_2.9.0-theseus-argon\0".as_ptr() as *const XML_LChar
}

#[no_mangle]
pub extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    XML_Expat_Version {
        major: 2,
        minor: 9,
        micro: 0,
    }
}

static FEATURE_LIST: [XML_Feature; 5] = [
    XML_Feature {
        feature: XML_FEATURE_SIZEOF_XML_CHAR,
        name: b"XML_CHAR\0".as_ptr() as *const XML_LChar,
        value: 1,
    },
    XML_Feature {
        feature: XML_FEATURE_SIZEOF_XML_LCHAR,
        name: b"XML_LCHAR\0".as_ptr() as *const XML_LChar,
        value: 1,
    },
    XML_Feature {
        feature: XML_FEATURE_DTD,
        name: b"XML_DTD\0".as_ptr() as *const XML_LChar,
        value: 0,
    },
    XML_Feature {
        feature: XML_FEATURE_NS,
        name: b"XML_NS\0".as_ptr() as *const XML_LChar,
        value: 0,
    },
    XML_Feature {
        feature: XML_FEATURE_END,
        name: ptr::null(),
        value: 0,
    },
];

#[no_mangle]
pub extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    FEATURE_LIST.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetReparseDeferralEnabled(
    parser: XML_Parser,
    enabled: XML_Bool,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    (*parser).reparse_deferral = enabled;
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    parser: XML_Parser,
    _maximumAmplificationFactor: f32,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    parser: XML_Parser,
    _activationThresholdBytes: c_ulonglong,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    parser: XML_Parser,
    _maximumAmplificationFactor: f32,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold(
    parser: XML_Parser,
    _activationThresholdBytes: c_ulonglong,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    XML_TRUE
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetPropertyBool(
    parser: XML_Parser,
    _property: XML_Parser_Property,
    value: *mut XML_Bool,
) -> XML_Prop_Error {
    if parser.is_null() {
        return XML_PROP_ERROR_PARSER_NULL;
    }
    if value.is_null() {
        return XML_PROP_ERROR_INVALID_VALUE;
    }
    *value = (*parser).reparse_deferral;
    XML_PROP_ERROR_NONE
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetPropertyDouble(
    parser: XML_Parser,
    _property: XML_Parser_Property,
    value: *mut f64,
) -> XML_Prop_Error {
    if parser.is_null() {
        return XML_PROP_ERROR_PARSER_NULL;
    }
    if value.is_null() {
        return XML_PROP_ERROR_INVALID_VALUE;
    }
    *value = 0.0;
    XML_PROP_ERROR_NONE
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetPropertyUInt64(
    parser: XML_Parser,
    _property: XML_Parser_Property,
    value: *mut u64,
) -> XML_Prop_Error {
    if parser.is_null() {
        return XML_PROP_ERROR_PARSER_NULL;
    }
    if value.is_null() {
        return XML_PROP_ERROR_INVALID_VALUE;
    }
    *value = 0;
    XML_PROP_ERROR_NONE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetPropertyBool(
    parser: XML_Parser,
    _property: XML_Parser_Property,
    value: XML_Bool,
) -> XML_Prop_Error {
    if parser.is_null() {
        return XML_PROP_ERROR_PARSER_NULL;
    }
    (*parser).reparse_deferral = value;
    XML_PROP_ERROR_NONE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetPropertyDouble(
    parser: XML_Parser,
    _property: XML_Parser_Property,
    _value: f64,
) -> XML_Prop_Error {
    if parser.is_null() {
        return XML_PROP_ERROR_PARSER_NULL;
    }
    XML_PROP_ERROR_NONE
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetPropertyUInt64(
    parser: XML_Parser,
    _property: XML_Parser_Property,
    _value: u64,
) -> XML_Prop_Error {
    if parser.is_null() {
        return XML_PROP_ERROR_PARSER_NULL;
    }
    XML_PROP_ERROR_NONE
}
