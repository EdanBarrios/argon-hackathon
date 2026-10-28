// HAND-WRITTEN C-ABI wrapper (not Argon's code). Unsafe by necessity: it
// dereferences raw pointers handed over by xmlparse.c.
//
// C side (vendor xmlrole.h, XML_DTD on):
//   typedef struct prolog_state {
//     int (*handler)(struct prolog_state *, int tok, const char *ptr,
//                    const char *end, const ENCODING *enc);
//     unsigned level; int role_none;
//     unsigned includeLevel; int documentEntity; int inEntityValue;
//   } PROLOG_STATE;
//
// Argon's state is an enum (PrologHandler); C needs a function pointer it can
// call. So there is one extern "C" trampoline per enum variant; each copies the
// C struct into Argon's PrologState, runs Argon's token_role, and copies the
// result back, storing the trampoline for the new variant in `handler`.

use std::os::raw::{c_char, c_int, c_uint};

#[cfg(not(target_endian = "little"))]
compile_error!("encoding mapping below assumes a little-endian host");

type CHandler = unsafe extern "C" fn(
    *mut CPrologState,
    c_int,
    *const c_char,
    *const c_char,
    *const CEncoding,
) -> c_int;

#[repr(C)]
pub struct CPrologState {
    handler: Option<CHandler>,
    level: c_uint,
    role_none: c_int,
    include_level: c_uint,
    document_entity: c_int,
    in_entity_value: c_int,
}

// Prefix of struct encoding (xmltok.h, XML_DTD on): 4 scanners + 2 literal
// scanners + 10 function pointers, then minBytesPerChar, isUtf8, isUtf16.
#[repr(C)]
pub struct CEncoding {
    _vtable: [*const (); 16],
    min_bytes_per_char: c_int,
    is_utf8: c_char,
    is_utf16: c_char,
}

// Argon's name_matches_ascii only depends on the code unit width and byte
// order. 1-byte encodings (UTF-8, Latin-1, ASCII, the init encoding) compare
// identically. For 2-byte ones, xmltok.c sets isUtf16 = 1 exactly on the
// native-order (little-endian here) encodings.
unsafe fn map_encoding(enc: *const CEncoding) -> Encoding {
    let e = &*enc;
    if e.min_bytes_per_char == 2 {
        if e.is_utf16 != 0 {
            Encoding::Little2
        } else {
            Encoding::Big2
        }
    } else {
        Encoding::Utf8
    }
}

unsafe fn dispatch(
    h: PrologHandler,
    s: *mut CPrologState,
    tok: c_int,
    ptr: *const c_char,
    end: *const c_char,
    enc: *const CEncoding,
) -> c_int {
    let st = &mut *s;
    let mut ps = PrologState {
        handler: h,
        level: st.level,
        role_none: st.role_none,
        include_level: st.include_level,
        document_entity: st.document_entity != 0,
        in_entity_value: st.in_entity_value != 0,
    };
    let len = (end as usize).wrapping_sub(ptr as usize);
    let buf: &[u8] = if ptr.is_null() || len == 0 || end < ptr {
        &[]
    } else {
        std::slice::from_raw_parts(ptr as *const u8, len)
    };
    let role = ps.token_role(tok, buf, 0, buf.len(), map_encoding(enc));
    st.handler = Some(trampoline(ps.handler));
    st.level = ps.level;
    st.role_none = ps.role_none;
    st.include_level = ps.include_level;
    st.document_entity = ps.document_entity as c_int;
    st.in_entity_value = ps.in_entity_value as c_int;
    role
}

macro_rules! trampolines {
    ($($v:ident => $f:ident),* $(,)?) => {
        $(
            unsafe extern "C" fn $f(
                s: *mut CPrologState,
                tok: c_int,
                ptr: *const c_char,
                end: *const c_char,
                enc: *const CEncoding,
            ) -> c_int {
                dispatch(PrologHandler::$v, s, tok, ptr, end, enc)
            }
        )*
        fn trampoline(h: PrologHandler) -> CHandler {
            match h { $(PrologHandler::$v => $f,)* }
        }
    };
}

trampolines! {
    Prolog0 => t_prolog0, Prolog1 => t_prolog1, Prolog2 => t_prolog2,
    Doctype0 => t_doctype0, Doctype1 => t_doctype1, Doctype2 => t_doctype2,
    Doctype3 => t_doctype3, Doctype4 => t_doctype4, Doctype5 => t_doctype5,
    InternalSubset => t_internal_subset, ExternalSubset0 => t_external_subset0,
    ExternalSubset1 => t_external_subset1,
    CondSect0 => t_cond_sect0, CondSect1 => t_cond_sect1, CondSect2 => t_cond_sect2,
    Entity0 => t_entity0, Entity1 => t_entity1, Entity2 => t_entity2,
    Entity3 => t_entity3, Entity4 => t_entity4, Entity5 => t_entity5,
    Entity6 => t_entity6, Entity7 => t_entity7, Entity8 => t_entity8,
    Entity9 => t_entity9, Entity10 => t_entity10,
    Notation0 => t_notation0, Notation1 => t_notation1, Notation2 => t_notation2,
    Notation3 => t_notation3, Notation4 => t_notation4,
    Attlist0 => t_attlist0, Attlist1 => t_attlist1, Attlist2 => t_attlist2,
    Attlist3 => t_attlist3, Attlist4 => t_attlist4, Attlist5 => t_attlist5,
    Attlist6 => t_attlist6, Attlist7 => t_attlist7, Attlist8 => t_attlist8,
    Attlist9 => t_attlist9,
    Element0 => t_element0, Element1 => t_element1, Element2 => t_element2,
    Element3 => t_element3, Element4 => t_element4, Element5 => t_element5,
    Element6 => t_element6, Element7 => t_element7,
    DeclClose => t_decl_close, Error => t_error,
}

// Same fields as C's XmlPrologStateInit (level and role_none left as is).
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInit(state: *mut CPrologState) {
    let st = &mut *state;
    st.handler = Some(trampoline(PrologState::new().handler));
    st.document_entity = 1;
    st.include_level = 0;
    st.in_entity_value = 0;
}

// Same fields as C's XmlPrologStateInitExternalEntity. Argon's port has no
// external-entity init (out of its scope), so this one is written by hand.
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(state: *mut CPrologState) {
    let st = &mut *state;
    st.handler = Some(trampoline(PrologHandler::ExternalSubset0));
    st.document_entity = 0;
    st.include_level = 0;
}
