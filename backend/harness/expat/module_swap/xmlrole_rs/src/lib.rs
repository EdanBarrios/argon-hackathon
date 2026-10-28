// Rust replacement for libexpat 2.9.0 lib/xmlrole.c.
//
// argon_role.rs: Argon's safe-Rust prolog/role state machine, copied verbatim
//   by scripts/extract_argon_xmlrole.py (not edited by hand).
// ffi.rs: hand-written (Claude, systems side) unsafe C-ABI wrapper exposing
//   XmlPrologStateInit, XmlPrologStateInitExternalEntity and the
//   PROLOG_STATE.handler function pointers that xmlparse.c calls.
//
// Both files are textually included into one module because Argon's items are
// private (no `pub`), and the wrapper needs their fields.
#![allow(dead_code)]

include!("argon_role.rs");
include!("ffi.rs");
