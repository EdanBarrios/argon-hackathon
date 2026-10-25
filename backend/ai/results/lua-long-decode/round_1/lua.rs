#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    clippy::needless_range_loop,
    clippy::manual_range_contains,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::if_same_then_else,
    clippy::match_same_arms,
    clippy::single_match,
    clippy::unnested_or_patterns,
    clippy::unnecessary_wraps,
    clippy::upper_case_acronyms,
    clippy::enum_variant_names,
    clippy::wrong_self_convention,
    clippy::needless_late_init,
    clippy::needless_return,
    clippy::let_and_return,
    clippy::manual_memcpy,
    clippy::manual_find,
    clippy::manual_strip,
    clippy::manual_map,
    clippy::manual_clamp,
    clippy::manual_is_ascii_check,
    clippy::char_lit_as_u8,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::ptr_arg,
    clippy::trivially_copy_pass_by_ref,
    clippy::redundant_field_names,
    clippy::len_without_is_empty,
    clippy::new_without_default,
    clippy::should_implement_trait,
    clippy::comparison_chain,
    clippy::neg_multiply,
    clippy::identity_op,
    clippy::erasing_op,
    clippy::eq_op,
    clippy::overly_complex_bool_expr,
    clippy::nonminimal_bool,
    clippy::blocks_in_conditions,
    clippy::assign_op_pattern,
    clippy::wildcard_in_or_patterns,
    clippy::large_enum_variant,
    clippy::result_unit_err,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::fn_params_excessive_bools,
    clippy::items_after_statements,
    clippy::similar_names,
    clippy::shadow_unrelated,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::used_underscore_binding,
    clippy::wildcard_imports,
    dead_code,
    unused_mut,
    unused_variables,
    unused_assignments,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

use std::collections::HashMap;

pub type LuaInteger = i64;
pub type LuaUnsigned = u64;
pub type LuaNumber = f64;
pub type LuaKContext = isize;
pub type Instruction = u32;

pub type StrId = usize;
pub type TableId = usize;
pub type ProtoId = usize;
pub type LClosureId = usize;
pub type CClosureId = usize;
pub type UpValId = usize;
pub type UdataId = usize;
pub type ThreadId = usize;

pub type LuaCFunction = fn(&mut LuaVM, ThreadId) -> LuaResult<i32>;
pub type LuaKFunction = fn(&mut LuaVM, ThreadId, i32, LuaKContext) -> LuaResult<i32>;

pub type LuaResult<T> = Result<T, i32>;

const LUA_VERSION_MAJOR: &str = "5";
const LUA_VERSION_MINOR: &str = "4";
const LUA_VERSION_RELEASE: &str = "7";
const LUA_VERSION_NUM: LuaNumber = 504.0;
const LUA_VERSION: &str = "Lua 5.4";

const LUA_SIGNATURE: &[u8] = b"\x1bLua";
const LUA_MULTRET: i32 = -1;

const LUAI_MAXSTACK: i32 = 1_000_000;
const ERRORSTACKSIZE: usize = (LUAI_MAXSTACK as usize) + 200;
const LUA_REGISTRYINDEX: i32 = -LUAI_MAXSTACK - 1000;

const fn lua_upvalueindex(i: i32) -> i32 {
    LUA_REGISTRYINDEX - i
}

const LUA_OK: i32 = 0;
const LUA_YIELD: i32 = 1;
const LUA_ERRRUN: i32 = 2;
const LUA_ERRSYNTAX: i32 = 3;
const LUA_ERRMEM: i32 = 4;
const LUA_ERRERR: i32 = 5;
const LUA_ERRFILE: i32 = LUA_ERRERR + 1;

const LUA_TNONE: i32 = -1;
const LUA_TNIL: i32 = 0;
const LUA_TBOOLEAN: i32 = 1;
const LUA_TLIGHTUSERDATA: i32 = 2;
const LUA_TNUMBER: i32 = 3;
const LUA_TSTRING: i32 = 4;
const LUA_TTABLE: i32 = 5;
const LUA_TFUNCTION: i32 = 6;
const LUA_TUSERDATA: i32 = 7;
const LUA_TTHREAD: i32 = 8;
const LUA_NUMTYPES: usize = 9;
const LUA_TUPVAL: i32 = 9;
const LUA_TPROTO: i32 = 10;
const LUA_TDEADKEY: u8 = 11;
const LUA_TOTALTYPES: usize = 12;

const LUA_MINSTACK: usize = 20;
const BASIC_STACK_SIZE: usize = 2 * LUA_MINSTACK;
const EXTRA_STACK: usize = 5;

const LUA_RIDX_MAINTHREAD: LuaInteger = 1;
const LUA_RIDX_GLOBALS: LuaInteger = 2;
const LUA_RIDX_LAST: LuaInteger = LUA_RIDX_GLOBALS;

const LUA_OPADD: i32 = 0;
const LUA_OPSUB: i32 = 1;
const LUA_OPMUL: i32 = 2;
const LUA_OPMOD: i32 = 3;
const LUA_OPPOW: i32 = 4;
const LUA_OPDIV: i32 = 5;
const LUA_OPIDIV: i32 = 6;
const LUA_OPBAND: i32 = 7;
const LUA_OPBOR: i32 = 8;
const LUA_OPBXOR: i32 = 9;
const LUA_OPSHL: i32 = 10;
const LUA_OPSHR: i32 = 11;
const LUA_OPUNM: i32 = 12;
const LUA_OPBNOT: i32 = 13;

const LUA_OPEQ: i32 = 0;
const LUA_OPLT: i32 = 1;
const LUA_OPLE: i32 = 2;

const LUA_GCSTOP: i32 = 0;
const LUA_GCRESTART: i32 = 1;
const LUA_GCCOLLECT: i32 = 2;
const LUA_GCCOUNT: i32 = 3;
const LUA_GCCOUNTB: i32 = 4;
const LUA_GCSTEP: i32 = 5;
const LUA_GCSETPAUSE: i32 = 6;
const LUA_GCSETSTEPMUL: i32 = 7;
const LUA_GCISRUNNING: i32 = 9;
const LUA_GCGEN: i32 = 10;
const LUA_GCINC: i32 = 11;

const LUA_IDSIZE: usize = 60;
const LUAI_MAXSHORTLEN: usize = 40;
const LUAI_MAXCCALLS: u32 = 200;
const MAXUPVAL: usize = 255;
const CLOSEKTOP: i32 = -1;
const ABSLINEINFO: i8 = -0x80;
const MAXIWTHABS: u8 = 128;
const LFIELDS_PER_FLUSH: i32 = 50;
const MAXVARS: usize = 200;
const MAXREGS: i32 = 255;
const NO_JUMP: i32 = -1;
const MAXTAGLOOP: usize = 2000;

const BIT_ISCOLLECTABLE: u8 = 1 << 6;
const fn makevariant(t: i32, v: u8) -> u8 {
    (t as u8) | (v << 4)
}
const fn ctb(t: u8) -> u8 {
    t | BIT_ISCOLLECTABLE
}

const LUA_VNIL: u8 = makevariant(LUA_TNIL, 0);
const LUA_VEMPTY: u8 = makevariant(LUA_TNIL, 1);
const LUA_VABSTKEY: u8 = makevariant(LUA_TNIL, 2);
const LUA_VFALSE: u8 = makevariant(LUA_TBOOLEAN, 0);
const LUA_VTRUE: u8 = makevariant(LUA_TBOOLEAN, 1);
const LUA_VLIGHTUSERDATA: u8 = makevariant(LUA_TLIGHTUSERDATA, 0);
const LUA_VNUMINT: u8 = makevariant(LUA_TNUMBER, 0);
const LUA_VNUMFLT: u8 = makevariant(LUA_TNUMBER, 1);
const LUA_VSHRSTR: u8 = makevariant(LUA_TSTRING, 0);
const LUA_VLNGSTR: u8 = makevariant(LUA_TSTRING, 1);
const LUA_VTABLE: u8 = makevariant(LUA_TTABLE, 0);
const LUA_VLCL: u8 = makevariant(LUA_TFUNCTION, 0);
const LUA_VLCF: u8 = makevariant(LUA_TFUNCTION, 1);
const LUA_VCCL: u8 = makevariant(LUA_TFUNCTION, 2);
const LUA_VUSERDATA: u8 = makevariant(LUA_TUSERDATA, 0);
const LUA_VTHREAD: u8 = makevariant(LUA_TTHREAD, 0);

const CIST_OAH: u16 = 1 << 0;
const CIST_C: u16 = 1 << 1;
const CIST_FRESH: u16 = 1 << 2;
const CIST_HOOKED: u16 = 1 << 3;
const CIST_YPCALL: u16 = 1 << 4;
const CIST_TAIL: u16 = 1 << 5;
const CIST_HOOKYIELD: u16 = 1 << 6;
const CIST_FIN: u16 = 1 << 7;
const CIST_TRAN: u16 = 1 << 8;
const CIST_CLSRET: u16 = 1 << 9;
const CIST_RECST: u16 = 10;
const CIST_LEQ: u16 = 1 << 13;

const NYCI: u32 = 0x10000 | 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TMS {
    TM_INDEX = 0,
    TM_NEWINDEX = 1,
    TM_GC = 2,
    TM_MODE = 3,
    TM_LEN = 4,
    TM_EQ = 5,
    TM_ADD = 6,
    TM_SUB = 7,
    TM_MUL = 8,
    TM_MOD = 9,
    TM_POW = 10,
    TM_DIV = 11,
    TM_IDIV = 12,
    TM_BAND = 13,
    TM_BOR = 14,
    TM_BXOR = 15,
    TM_SHL = 16,
    TM_SHR = 17,
    TM_UNM = 18,
    TM_BNOT = 19,
    TM_LT = 20,
    TM_LE = 21,
    TM_CONCAT = 22,
    TM_CALL = 23,
    TM_CLOSE = 24,
}
const TM_N: usize = 25;
const MASKFLAGS: u8 = !(!0u32 << (TMS::TM_EQ as u32 + 1)) as u8;
const BITRAS: u8 = 1 << 7;

impl TMS {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => TMS::TM_INDEX,
            1 => TMS::TM_NEWINDEX,
            2 => TMS::TM_GC,
            3 => TMS::TM_MODE,
            4 => TMS::TM_LEN,
            5 => TMS::TM_EQ,
            6 => TMS::TM_ADD,
            7 => TMS::TM_SUB,
            8 => TMS::TM_MUL,
            9 => TMS::TM_MOD,
            10 => TMS::TM_POW,
            11 => TMS::TM_DIV,
            12 => TMS::TM_IDIV,
            13 => TMS::TM_BAND,
            14 => TMS::TM_BOR,
            15 => TMS::TM_BXOR,
            16 => TMS::TM_SHL,
            17 => TMS::TM_SHR,
            18 => TMS::TM_UNM,
            19 => TMS::TM_BNOT,
            20 => TMS::TM_LT,
            21 => TMS::TM_LE,
            22 => TMS::TM_CONCAT,
            23 => TMS::TM_CALL,
            _ => TMS::TM_CLOSE,
        }
    }
}

static LUAT_TYPENAMES: [&str; LUA_TOTALTYPES] = [
    "no value", "nil", "boolean", "userdata", "number", "string", "table", "function", "userdata",
    "thread", "upvalue", "proto",
];

fn ttypename(t: i32) -> &'static str {
    let idx = (t + 1) as usize;
    LUAT_TYPENAMES.get(idx).copied().unwrap_or("no value")
}

#[derive(Clone, Copy)]
pub enum ValueData {
    Nil,
    Bool(bool),
    Int(LuaInteger),
    Flt(LuaNumber),
    Str(StrId),
    Table(TableId),
    LClosure(LClosureId),
    CClosure(CClosureId),
    Lcf(LuaCFunction),
    Udata(UdataId),
    LightUdata(usize),
    Thread(ThreadId),
}

impl std::fmt::Debug for ValueData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueData::Nil => write!(f, "Nil"),
            ValueData::Bool(b) => write!(f, "Bool({b})"),
            ValueData::Int(i) => write!(f, "Int({i})"),
            ValueData::Flt(n) => write!(f, "Flt({n})"),
            ValueData::Str(s) => write!(f, "Str({s})"),
            ValueData::Table(t) => write!(f, "Table({t})"),
            ValueData::LClosure(c) => write!(f, "LClosure({c})"),
            ValueData::CClosure(c) => write!(f, "CClosure({c})"),
            ValueData::Lcf(_) => write!(f, "Lcf"),
            ValueData::Udata(u) => write!(f, "Udata({u})"),
            ValueData::LightUdata(p) => write!(f, "LightUdata({p})"),
            ValueData::Thread(t) => write!(f, "Thread({t})"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TValue {
    pub tt_: u8,
    pub val: ValueData,
}

impl TValue {
    pub const fn nil() -> Self {
        TValue {
            tt_: LUA_VNIL,
            val: ValueData::Nil,
        }
    }
    pub const fn empty() -> Self {
        TValue {
            tt_: LUA_VEMPTY,
            val: ValueData::Nil,
        }
    }
    pub const fn abstkey() -> Self {
        TValue {
            tt_: LUA_VABSTKEY,
            val: ValueData::Nil,
        }
    }
    pub const fn bool_val(b: bool) -> Self {
        TValue {
            tt_: if b { LUA_VTRUE } else { LUA_VFALSE },
            val: ValueData::Bool(b),
        }
    }
    pub const fn int_val(i: LuaInteger) -> Self {
        TValue {
            tt_: LUA_VNUMINT,
            val: ValueData::Int(i),
        }
    }
    pub const fn flt_val(n: LuaNumber) -> Self {
        TValue {
            tt_: LUA_VNUMFLT,
            val: ValueData::Flt(n),
        }
    }
    pub const fn str_val(id: StrId, is_short: bool) -> Self {
        TValue {
            tt_: if is_short {
                ctb(LUA_VSHRSTR)
            } else {
                ctb(LUA_VLNGSTR)
            },
            val: ValueData::Str(id),
        }
    }
    pub const fn table_val(id: TableId) -> Self {
        TValue {
            tt_: ctb(LUA_VTABLE),
            val: ValueData::Table(id),
        }
    }
    pub const fn lcl_val(id: LClosureId) -> Self {
        TValue {
            tt_: ctb(LUA_VLCL),
            val: ValueData::LClosure(id),
        }
    }
    pub const fn ccl_val(id: CClosureId) -> Self {
        TValue {
            tt_: ctb(LUA_VCCL),
            val: ValueData::CClosure(id),
        }
    }
    pub const fn lcf_val(f: LuaCFunction) -> Self {
        TValue {
            tt_: LUA_VLCF,
            val: ValueData::Lcf(f),
        }
    }
    pub const fn udata_val(id: UdataId) -> Self {
        TValue {
            tt_: ctb(LUA_VUSERDATA),
            val: ValueData::Udata(id),
        }
    }
    pub const fn lightudata_val(p: usize) -> Self {
        TValue {
            tt_: LUA_VLIGHTUSERDATA,
            val: ValueData::LightUdata(p),
        }
    }
    pub const fn thread_val(id: ThreadId) -> Self {
        TValue {
            tt_: ctb(LUA_VTHREAD),
            val: ValueData::Thread(id),
        }
    }

    pub fn rawtt(&self) -> u8 {
        self.tt_
    }
    pub fn ttypetag(&self) -> u8 {
        self.tt_ & 0x3F
    }
    pub fn ttype(&self) -> i32 {
        (self.tt_ & 0x0F) as i32
    }
    pub fn is_collectable(&self) -> bool {
        (self.tt_ & BIT_ISCOLLECTABLE) != 0
    }
    pub fn is_nil(&self) -> bool {
        self.ttype() == LUA_TNIL
    }
    pub fn is_strict_nil(&self) -> bool {
        self.tt_ == LUA_VNIL
    }
    pub fn is_empty(&self) -> bool {
        self.is_nil()
    }
    pub fn is_abstkey(&self) -> bool {
        self.tt_ == LUA_VABSTKEY
    }
    pub fn is_boolean(&self) -> bool {
        self.ttype() == LUA_TBOOLEAN
    }
    pub fn is_false(&self) -> bool {
        self.tt_ == LUA_VFALSE
    }
    pub fn is_true(&self) -> bool {
        self.tt_ == LUA_VTRUE
    }
    pub fn l_isfalse(&self) -> bool {
        self.is_false() || self.is_nil()
    }
    pub fn is_number(&self) -> bool {
        self.ttype() == LUA_TNUMBER
    }
    pub fn is_integer(&self) -> bool {
        self.tt_ == LUA_VNUMINT
    }
    pub fn is_float(&self) -> bool {
        self.tt_ == LUA_VNUMFLT
    }
    pub fn is_string(&self) -> bool {
        self.ttype() == LUA_TSTRING
    }
    pub fn is_shrstring(&self) -> bool {
        self.tt_ == ctb(LUA_VSHRSTR)
    }
    pub fn is_lngstring(&self) -> bool {
        self.tt_ == ctb(LUA_VLNGSTR)
    }
    pub fn is_table(&self) -> bool {
        self.tt_ == ctb(LUA_VTABLE)
    }
    pub fn is_function(&self) -> bool {
        self.ttype() == LUA_TFUNCTION
    }
    pub fn is_Lclosure(&self) -> bool {
        self.tt_ == ctb(LUA_VLCL)
    }
    pub fn is_lcf(&self) -> bool {
        self.tt_ == LUA_VLCF
    }
    pub fn is_Cclosure(&self) -> bool {
        self.tt_ == ctb(LUA_VCCL)
    }
    pub fn is_closure(&self) -> bool {
        self.is_Lclosure() || self.is_Cclosure()
    }
    pub fn is_fulluserdata(&self) -> bool {
        self.tt_ == ctb(LUA_VUSERDATA)
    }
    pub fn is_lightuserdata(&self) -> bool {
        self.tt_ == LUA_VLIGHTUSERDATA
    }
    pub fn is_thread(&self) -> bool {
        self.tt_ == ctb(LUA_VTHREAD)
    }
    pub fn ivalue(&self) -> LuaInteger {
        match self.val {
            ValueData::Int(i) => i,
            _ => 0,
        }
    }
    pub fn fltvalue(&self) -> LuaNumber {
        match self.val {
            ValueData::Flt(n) => n,
            _ => 0.0,
        }
    }
    pub fn nvalue(&self) -> LuaNumber {
        match self.val {
            ValueData::Int(i) => i as LuaNumber,
            ValueData::Flt(n) => n,
            _ => 0.0,
        }
    }
    pub fn tsvalue(&self) -> StrId {
        match self.val {
            ValueData::Str(id) => id,
            _ => 0,
        }
    }
    pub fn hvalue(&self) -> TableId {
        match self.val {
            ValueData::Table(id) => id,
            _ => 0,
        }
    }
    pub fn clLvalue(&self) -> LClosureId {
        match self.val {
            ValueData::LClosure(id) => id,
            _ => 0,
        }
    }
    pub fn clCvalue(&self) -> CClosureId {
        match self.val {
            ValueData::CClosure(id) => id,
            _ => 0,
        }
    }
    pub fn fvalue(&self) -> Option<LuaCFunction> {
        match self.val {
            ValueData::Lcf(f) => Some(f),
            _ => None,
        }
    }
    pub fn uvalue(&self) -> UdataId {
        match self.val {
            ValueData::Udata(id) => id,
            _ => 0,
        }
    }
    pub fn pvalue(&self) -> usize {
        match self.val {
            ValueData::LightUdata(p) => p,
            _ => 0,
        }
    }
    pub fn thvalue(&self) -> ThreadId {
        match self.val {
            ValueData::Thread(id) => id,
            _ => 0,
        }
    }
    pub fn point2uint(&self) -> u32 {
        match self.val {
            ValueData::LightUdata(p) => (p as u32).wrapping_mul(16).wrapping_add(0x10000),
            ValueData::Lcf(f) => (f as usize) as u32,
            ValueData::Table(id) => (id as u32).wrapping_mul(32).wrapping_add(0x200000),
            ValueData::LClosure(id) => (id as u32).wrapping_mul(32).wrapping_add(0x300000),
            ValueData::CClosure(id) => (id as u32).wrapping_mul(32).wrapping_add(0x400000),
            ValueData::Udata(id) => (id as u32).wrapping_mul(32).wrapping_add(0x500000),
            ValueData::Thread(id) => (id as u32).wrapping_mul(32).wrapping_add(0x600000),
            ValueData::Str(id) => (id as u32).wrapping_mul(32).wrapping_add(0x700000),
            _ => 0,
        }
    }
    pub fn topointer(&self) -> usize {
        match self.val {
            ValueData::Lcf(f) => f as usize,
            ValueData::LightUdata(p) => p,
            ValueData::Udata(id) => id.wrapping_mul(32).wrapping_add(0x500000),
            ValueData::Table(id) => id.wrapping_mul(32).wrapping_add(0x200000),
            ValueData::LClosure(id) => id.wrapping_mul(32).wrapping_add(0x300000),
            ValueData::CClosure(id) => id.wrapping_mul(32).wrapping_add(0x400000),
            ValueData::Thread(id) => id.wrapping_mul(32).wrapping_add(0x600000),
            ValueData::Str(id) => id.wrapping_mul(32).wrapping_add(0x700000),
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValOrigin {
    None,
    Stack(usize),
    Upval(UpValId),
}

#[derive(Clone, Copy, Debug)]
pub struct StackValue {
    pub val: TValue,
    pub delta: u16,
}

impl StackValue {
    pub const fn nil() -> Self {
        StackValue {
            val: TValue::nil(),
            delta: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TString {
    pub tt: u8,
    pub extra: u8,
    pub is_short: bool,
    pub hash: u32,
    pub contents: Vec<u8>,
}

impl TString {
    pub fn len(&self) -> usize {
        self.contents.len()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub i_val: TValue,
    pub key_tt: u8,
    pub next: i32,
    pub key_val: ValueData,
}

impl Node {
    pub const fn dummy() -> Self {
        Node {
            i_val: TValue::empty(),
            key_tt: LUA_VNIL,
            next: 0,
            key_val: ValueData::Nil,
        }
    }
    pub const fn empty_node() -> Self {
        Node {
            i_val: TValue::empty(),
            key_tt: LUA_TNIL as u8,
            next: 0,
            key_val: ValueData::Nil,
        }
    }
    pub fn key_is_nil(&self) -> bool {
        self.key_tt == (LUA_TNIL as u8)
    }
    pub fn key_is_integer(&self) -> bool {
        self.key_tt == LUA_VNUMINT
    }
    pub fn key_ival(&self) -> LuaInteger {
        match self.key_val {
            ValueData::Int(i) => i,
            _ => 0,
        }
    }
    pub fn key_is_shrstr(&self) -> bool {
        self.key_tt == ctb(LUA_VSHRSTR)
    }
    pub fn key_strval(&self) -> StrId {
        match self.key_val {
            ValueData::Str(id) => id,
            _ => 0,
        }
    }
    pub fn key_is_dead(&self) -> bool {
        self.key_tt == LUA_TDEADKEY
    }
    pub fn get_key(&self) -> TValue {
        TValue {
            tt_: self.key_tt,
            val: self.key_val,
        }
    }
    pub fn set_key(&mut self, k: TValue) {
        self.key_tt = k.tt_;
        self.key_val = k.val;
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    pub flags: u8,
    pub lsizenode: u8,
    pub alimit: u32,
    pub array: Vec<TValue>,
    pub node: Vec<Node>,
    pub lastfree: usize,
    pub is_dummy: bool,
    pub metatable: Option<TableId>,
}

#[derive(Clone, Debug)]
pub enum UdataPayload {
    Bytes(Vec<u8>),
    RanState([u64; 4]),
    GMatch(GMatchStateData),
}

#[derive(Clone, Debug)]
pub struct GMatchStateData {
    pub src_pos: usize,
    pub lastmatch: Option<usize>,
    pub s_id: StrId,
    pub p_id: StrId,
}

#[derive(Clone, Debug)]
pub struct Udata {
    pub nuvalue: u16,
    pub len: usize,
    pub metatable: Option<TableId>,
    pub uv: Vec<TValue>,
    pub payload: UdataPayload,
}

#[derive(Clone, Copy, Debug)]
pub struct Upvaldesc {
    pub name: Option<StrId>,
    pub instack: u8,
    pub idx: u8,
    pub kind: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct LocVar {
    pub varname: Option<StrId>,
    pub startpc: i32,
    pub endpc: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct AbsLineInfo {
    pub pc: i32,
    pub line: i32,
}

#[derive(Clone, Debug)]
pub struct Proto {
    pub numparams: u8,
    pub is_vararg: u8,
    pub maxstacksize: u8,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub k: Vec<TValue>,
    pub code: Vec<Instruction>,
    pub p: Vec<ProtoId>,
    pub upvalues: Vec<Upvaldesc>,
    pub lineinfo: Vec<i8>,
    pub abslineinfo: Vec<AbsLineInfo>,
    pub locvars: Vec<LocVar>,
    pub source: Option<StrId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            numparams: 0,
            is_vararg: 0,
            maxstacksize: 0,
            linedefined: 0,
            lastlinedefined: 0,
            k: Vec::new(),
            code: Vec::new(),
            p: Vec::new(),
            upvalues: Vec::new(),
            lineinfo: Vec::new(),
            abslineinfo: Vec::new(),
            locvars: Vec::new(),
            source: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UpValLoc {
    Open { thread: ThreadId, stack_idx: usize },
    Closed(TValue),
}

#[derive(Clone, Debug)]
pub struct UpVal {
    pub loc: UpValLoc,
}

#[derive(Clone, Debug)]
pub struct LClosure {
    pub nupvalues: u8,
    pub p: ProtoId,
    pub upvals: Vec<UpValId>,
}

#[derive(Clone, Debug)]
pub struct CClosure {
    pub nupvalues: u8,
    pub f: LuaCFunction,
    pub upvalue: Vec<TValue>,
}

#[derive(Clone, Copy, Debug)]
pub struct CallInfo {
    pub func: usize,
    pub top: usize,
    pub savedpc: usize,
    pub trap: i32,
    pub nextraargs: i32,
    pub k: Option<LuaKFunction>,
    pub old_errfunc: isize,
    pub ctx: LuaKContext,
    pub u2_val: i32,
    pub ftransfer: u16,
    pub ntransfer: u16,
    pub nresults: i16,
    pub callstatus: u16,
}

impl CallInfo {
    pub fn new_base() -> Self {
        CallInfo {
            func: 0,
            top: 1 + LUA_MINSTACK,
            savedpc: 0,
            trap: 0,
            nextraargs: 0,
            k: None,
            old_errfunc: 0,
            ctx: 0,
            u2_val: 0,
            ftransfer: 0,
            ntransfer: 0,
            nresults: 0,
            callstatus: CIST_C,
        }
    }
    pub fn is_lua(&self) -> bool {
        (self.callstatus & CIST_C) == 0
    }
    pub fn is_luacode(&self) -> bool {
        (self.callstatus & (CIST_C | CIST_HOOKED)) == 0
    }
    pub fn get_cistrecst(&self) -> i32 {
        ((self.callstatus >> CIST_RECST) & 7) as i32
    }
    pub fn set_cistrecst(&mut self, st: i32) {
        self.callstatus =
            (self.callstatus & !(7 << CIST_RECST)) | (((st as u16) & 7) << CIST_RECST);
    }
}

pub struct LuaThread {
    pub status: i32,
    pub allowhook: u8,
    pub top: usize,
    pub stack_last: usize,
    pub stack: Vec<StackValue>,
    pub openupval: Vec<UpValId>,
    pub tbclist: usize,
    pub error_jmp_depth: usize,
    pub ci_vec: Vec<CallInfo>,
    pub ci_idx: usize,
    pub errfunc: isize,
    pub nCcalls: u32,
    pub oldpc: i32,
    pub hookmask: u8,
}

impl LuaThread {
    pub fn new() -> Self {
        let mut stack = vec![StackValue::nil(); BASIC_STACK_SIZE + EXTRA_STACK];
        if let Some(slot) = stack.get_mut(0) {
            slot.val = TValue::nil();
        }
        let base_ci = CallInfo::new_base();
        LuaThread {
            status: LUA_OK,
            allowhook: 1,
            top: 1,
            stack_last: BASIC_STACK_SIZE,
            stack,
            openupval: Vec::new(),
            tbclist: 0,
            error_jmp_depth: 0,
            ci_vec: vec![base_ci],
            ci_idx: 0,
            errfunc: 0,
            nCcalls: 0,
            oldpc: 0,
            hookmask: 0,
        }
    }
    pub fn ci(&self) -> CallInfo {
        self.ci_vec
            .get(self.ci_idx)
            .copied()
            .unwrap_or_else(CallInfo::new_base)
    }
    pub fn ci_mut(&mut self) -> &mut CallInfo {
        let idx = self.ci_idx;
        if idx >= self.ci_vec.len() {
            self.ci_vec.resize(idx + 1, CallInfo::new_base());
        }
        match self.ci_vec.get_mut(idx) {
            Some(c) => c,
            None => unreachable_fallback(&mut self.ci_vec),
        }
    }
    pub fn yieldable(&self) -> bool {
        (self.nCcalls & 0xffff0000) == 0
    }
    pub fn get_ccalls(&self) -> u32 {
        self.nCcalls & 0xffff
    }
    pub fn inc_nny(&mut self) {
        self.nCcalls = self.nCcalls.wrapping_add(0x10000);
    }
    pub fn dec_nny(&mut self) {
        self.nCcalls = self.nCcalls.wrapping_sub(0x10000);
    }
}

fn unreachable_fallback<T>(v: &mut Vec<T>) -> &mut T {
    match v.first_mut() {
        Some(x) => x,
        None => loop {},
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WarnState {
    Off,
    On,
    Cont,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SlotLoc {
    Array(TableId, usize),
    NodeVal(TableId, usize),
    AbsentKey,
    Null,
}

pub struct LuaVM {
    pub strings: Vec<TString>,
    pub short_str_map: HashMap<Vec<u8>, StrId>,
    pub tables: Vec<Table>,
    pub protos: Vec<Proto>,
    pub lclosures: Vec<LClosure>,
    pub cclosures: Vec<CClosure>,
    pub upvals: Vec<UpVal>,
    pub udatas: Vec<Udata>,
    pub threads: Vec<LuaThread>,
    pub output: Vec<u8>,
    pub err_output: Vec<u8>,
    pub seed: u32,
    pub l_registry: TValue,
    pub nilvalue: TValue,
    pub memerrmsg: StrId,
    pub envn: StrId,
    pub name_str: StrId,
    pub brk_str: StrId,
    pub tmname: [StrId; TM_N],
    pub mt: [Option<TableId>; LUA_NUMTYPES],
    pub mainthread: ThreadId,
    pub gcstp: u8,
    pub gckind: i32,
    pub gcpause: i32,
    pub gcstepmul: i32,
    pub gcstepsize: i32,
    pub genminormul: i32,
    pub genmajormul: i32,
    pub warn_state: WarnState,
}

// ============================================================================
// OpCodes and Instruction encoding
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum OpMode {
    iABC = 0,
    iABx = 1,
    iAsBx = 2,
    iAx = 3,
    isJ = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum OpCode {
    OP_MOVE = 0,
    OP_LOADI = 1,
    OP_LOADF = 2,
    OP_LOADK = 3,
    OP_LOADKX = 4,
    OP_LOADFALSE = 5,
    OP_LFALSESKIP = 6,
    OP_LOADTRUE = 7,
    OP_LOADNIL = 8,
    OP_GETUPVAL = 9,
    OP_SETUPVAL = 10,
    OP_GETTABUP = 11,
    OP_GETTABLE = 12,
    OP_GETI = 13,
    OP_GETFIELD = 14,
    OP_SETTABUP = 15,
    OP_SETTABLE = 16,
    OP_SETI = 17,
    OP_SETFIELD = 18,
    OP_NEWTABLE = 19,
    OP_SELF = 20,
    OP_ADDI = 21,
    OP_ADDK = 22,
    OP_SUBK = 23,
    OP_MULK = 24,
    OP_MODK = 25,
    OP_POWK = 26,
    OP_DIVK = 27,
    OP_IDIVK = 28,
    OP_BANDK = 29,
    OP_BORK = 30,
    OP_BXORK = 31,
    OP_SHRI = 32,
    OP_SHLI = 33,
    OP_ADD = 34,
    OP_SUB = 35,
    OP_MUL = 36,
    OP_MOD = 37,
    OP_POW = 38,
    OP_DIV = 39,
    OP_IDIV = 40,
    OP_BAND = 41,
    OP_BOR = 42,
    OP_BXOR = 43,
    OP_SHL = 44,
    OP_SHR = 45,
    OP_MMBIN = 46,
    OP_MMBINI = 47,
    OP_MMBINK = 48,
    OP_UNM = 49,
    OP_BNOT = 50,
    OP_NOT = 51,
    OP_LEN = 52,
    OP_CONCAT = 53,
    OP_CLOSE = 54,
    OP_TBC = 55,
    OP_JMP = 56,
    OP_EQ = 57,
    OP_LT = 58,
    OP_LE = 59,
    OP_EQK = 60,
    OP_EQI = 61,
    OP_LTI = 62,
    OP_LEI = 63,
    OP_GTI = 64,
    OP_GEI = 65,
    OP_TEST = 66,
    OP_TESTSET = 67,
    OP_CALL = 68,
    OP_TAILCALL = 69,
    OP_RETURN = 70,
    OP_RETURN0 = 71,
    OP_RETURN1 = 72,
    OP_FORLOOP = 73,
    OP_FORPREP = 74,
    OP_TFORPREP = 75,
    OP_TFORCALL = 76,
    OP_TFORLOOP = 77,
    OP_SETLIST = 78,
    OP_CLOSURE = 79,
    OP_VARARG = 80,
    OP_VARARGPREP = 81,
    OP_EXTRAARG = 82,
}

const NUM_OPCODES: usize = 83;

impl OpCode {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => OpCode::OP_MOVE,
            1 => OpCode::OP_LOADI,
            2 => OpCode::OP_LOADF,
            3 => OpCode::OP_LOADK,
            4 => OpCode::OP_LOADKX,
            5 => OpCode::OP_LOADFALSE,
            6 => OpCode::OP_LFALSESKIP,
            7 => OpCode::OP_LOADTRUE,
            8 => OpCode::OP_LOADNIL,
            9 => OpCode::OP_GETUPVAL,
            10 => OpCode::OP_SETUPVAL,
            11 => OpCode::OP_GETTABUP,
            12 => OpCode::OP_GETTABLE,
            13 => OpCode::OP_GETI,
            14 => OpCode::OP_GETFIELD,
            15 => OpCode::OP_SETTABUP,
            16 => OpCode::OP_SETTABLE,
            17 => OpCode::OP_SETI,
            18 => OpCode::OP_SETFIELD,
            19 => OpCode::OP_NEWTABLE,
            20 => OpCode::OP_SELF,
            21 => OpCode::OP_ADDI,
            22 => OpCode::OP_ADDK,
            23 => OpCode::OP_SUBK,
            24 => OpCode::OP_MULK,
            25 => OpCode::OP_MODK,
            26 => OpCode::OP_POWK,
            27 => OpCode::OP_DIVK,
            28 => OpCode::OP_IDIVK,
            29 => OpCode::OP_BANDK,
            30 => OpCode::OP_BORK,
            31 => OpCode::OP_BXORK,
            32 => OpCode::OP_SHRI,
            33 => OpCode::OP_SHLI,
            34 => OpCode::OP_ADD,
            35 => OpCode::OP_SUB,
            36 => OpCode::OP_MUL,
            37 => OpCode::OP_MOD,
            38 => OpCode::OP_POW,
            39 => OpCode::OP_DIV,
            40 => OpCode::OP_IDIV,
            41 => OpCode::OP_BAND,
            42 => OpCode::OP_BOR,
            43 => OpCode::OP_BXOR,
            44 => OpCode::OP_SHL,
            45 => OpCode::OP_SHR,
            46 => OpCode::OP_MMBIN,
            47 => OpCode::OP_MMBINI,
            48 => OpCode::OP_MMBINK,
            49 => OpCode::OP_UNM,
            50 => OpCode::OP_BNOT,
            51 => OpCode::OP_NOT,
            52 => OpCode::OP_LEN,
            53 => OpCode::OP_CONCAT,
            54 => OpCode::OP_CLOSE,
            55 => OpCode::OP_TBC,
            56 => OpCode::OP_JMP,
            57 => OpCode::OP_EQ,
            58 => OpCode::OP_LT,
            59 => OpCode::OP_LE,
            60 => OpCode::OP_EQK,
            61 => OpCode::OP_EQI,
            62 => OpCode::OP_LTI,
            63 => OpCode::OP_LEI,
            64 => OpCode::OP_GTI,
            65 => OpCode::OP_GEI,
            66 => OpCode::OP_TEST,
            67 => OpCode::OP_TESTSET,
            68 => OpCode::OP_CALL,
            69 => OpCode::OP_TAILCALL,
            70 => OpCode::OP_RETURN,
            71 => OpCode::OP_RETURN0,
            72 => OpCode::OP_RETURN1,
            73 => OpCode::OP_FORLOOP,
            74 => OpCode::OP_FORPREP,
            75 => OpCode::OP_TFORPREP,
            76 => OpCode::OP_TFORCALL,
            77 => OpCode::OP_TFORLOOP,
            78 => OpCode::OP_SETLIST,
            79 => OpCode::OP_CLOSURE,
            80 => OpCode::OP_VARARG,
            81 => OpCode::OP_VARARGPREP,
            _ => OpCode::OP_EXTRAARG,
        }
    }
}

const SIZE_C: u32 = 8;
const SIZE_B: u32 = 8;
const SIZE_BX: u32 = SIZE_C + SIZE_B + 1;
const SIZE_A: u32 = 8;
const SIZE_AX: u32 = SIZE_BX + SIZE_A;
const SIZE_SJ: u32 = SIZE_BX + SIZE_A;
const SIZE_OP: u32 = 7;

const POS_OP: u32 = 0;
const POS_A: u32 = POS_OP + SIZE_OP;
const POS_K: u32 = POS_A + SIZE_A;
const POS_B: u32 = POS_K + 1;
const POS_C: u32 = POS_B + SIZE_B;
const POS_BX: u32 = POS_K;
const POS_AX: u32 = POS_A;
const POS_SJ: u32 = POS_A;

const MAXARG_BX: i32 = (1 << SIZE_BX) - 1;
const OFFSET_SBX: i32 = MAXARG_BX >> 1;
const MAXARG_AX: i32 = (1 << SIZE_AX) - 1;
const MAXARG_SJ: i32 = (1 << SIZE_SJ) - 1;
const OFFSET_SJ: i32 = MAXARG_SJ >> 1;
const MAXARG_A: i32 = (1 << SIZE_A) - 1;
const MAXARG_B: i32 = (1 << SIZE_B) - 1;
const MAXARG_C: i32 = (1 << SIZE_C) - 1;
const OFFSET_SC: i32 = MAXARG_C >> 1;
const MAXINDEXRK: i32 = MAXARG_B;
const NO_REG: i32 = MAXARG_A;

const fn int2sC(i: i32) -> i32 {
    i + OFFSET_SC
}
const fn sC2int(i: i32) -> i32 {
    i - OFFSET_SC
}
const fn mask1(n: u32, p: u32) -> u32 {
    (!((!0u32) << n)) << p
}
const fn mask0(n: u32, p: u32) -> u32 {
    !mask1(n, p)
}

fn GET_OPCODE(i: Instruction) -> OpCode {
    OpCode::from_u8(((i >> POS_OP) & mask1(SIZE_OP, 0)) as u8)
}
fn SET_OPCODE(i: &mut Instruction, o: OpCode) {
    *i = (*i & mask0(SIZE_OP, POS_OP)) | (((o as u32) << POS_OP) & mask1(SIZE_OP, POS_OP));
}
fn getarg(i: Instruction, pos: u32, size: u32) -> i32 {
    ((i >> pos) & mask1(size, 0)) as i32
}
fn setarg(i: &mut Instruction, v: i32, pos: u32, size: u32) {
    *i = (*i & mask0(size, pos)) | (((v as u32) << pos) & mask1(size, pos));
}
fn GETARG_A(i: Instruction) -> i32 {
    getarg(i, POS_A, SIZE_A)
}
fn SETARG_A(i: &mut Instruction, v: i32) {
    setarg(i, v, POS_A, SIZE_A);
}
fn GETARG_B(i: Instruction) -> i32 {
    getarg(i, POS_B, SIZE_B)
}
fn GETARG_sB(i: Instruction) -> i32 {
    sC2int(GETARG_B(i))
}
fn SETARG_B(i: &mut Instruction, v: i32) {
    setarg(i, v, POS_B, SIZE_B);
}
fn GETARG_C(i: Instruction) -> i32 {
    getarg(i, POS_C, SIZE_C)
}
fn GETARG_sC(i: Instruction) -> i32 {
    sC2int(GETARG_C(i))
}
fn SETARG_C(i: &mut Instruction, v: i32) {
    setarg(i, v, POS_C, SIZE_C);
}
fn TESTARG_k(i: Instruction) -> bool {
    (i & (1u32 << POS_K)) != 0
}
fn GETARG_k(i: Instruction) -> i32 {
    getarg(i, POS_K, 1)
}
fn SETARG_k(i: &mut Instruction, v: i32) {
    setarg(i, v, POS_K, 1);
}
fn GETARG_Bx(i: Instruction) -> i32 {
    getarg(i, POS_BX, SIZE_BX)
}
fn SETARG_Bx(i: &mut Instruction, v: i32) {
    setarg(i, v, POS_BX, SIZE_BX);
}
fn GETARG_Ax(i: Instruction) -> i32 {
    getarg(i, POS_AX, SIZE_AX)
}
fn GETARG_sBx(i: Instruction) -> i32 {
    GETARG_Bx(i) - OFFSET_SBX
}
fn GETARG_sJ(i: Instruction) -> i32 {
    getarg(i, POS_SJ, SIZE_SJ) - OFFSET_SJ
}
fn SETARG_sJ(i: &mut Instruction, j: i32) {
    setarg(i, j + OFFSET_SJ, POS_SJ, SIZE_SJ);
}
fn CREATE_ABCk(o: OpCode, a: i32, b: i32, c: i32, k: i32) -> Instruction {
    ((o as u32) << POS_OP)
        | ((a as u32) << POS_A)
        | ((b as u32) << POS_B)
        | ((c as u32) << POS_C)
        | ((k as u32) << POS_K)
}
fn CREATE_ABx(o: OpCode, a: i32, bc: u32) -> Instruction {
    ((o as u32) << POS_OP) | ((a as u32) << POS_A) | (bc << POS_BX)
}
fn CREATE_Ax(o: OpCode, a: i32) -> Instruction {
    ((o as u32) << POS_OP) | ((a as u32) << POS_AX)
}
fn CREATE_sJ(o: OpCode, j: u32, k: i32) -> Instruction {
    ((o as u32) << POS_OP) | (j << POS_SJ) | ((k as u32) << POS_K)
}

const fn opmode(mm: u8, ot: u8, it: u8, t: u8, a: u8, m: OpMode) -> u8 {
    (mm << 7) | (ot << 6) | (it << 5) | (t << 4) | (a << 3) | (m as u8)
}

static LUAP_OPMODES: [u8; NUM_OPCODES] = [
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_MOVE
    opmode(0, 0, 0, 0, 1, OpMode::iAsBx), // OP_LOADI
    opmode(0, 0, 0, 0, 1, OpMode::iAsBx), // OP_LOADF
    opmode(0, 0, 0, 0, 1, OpMode::iABx),  // OP_LOADK
    opmode(0, 0, 0, 0, 1, OpMode::iABx),  // OP_LOADKX
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_LOADFALSE
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_LFALSESKIP
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_LOADTRUE
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_LOADNIL
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_GETUPVAL
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_SETUPVAL
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_GETTABUP
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_GETTABLE
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_GETI
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_GETFIELD
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_SETTABUP
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_SETTABLE
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_SETI
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_SETFIELD
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_NEWTABLE
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SELF
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_ADDI
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_ADDK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SUBK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_MULK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_MODK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_POWK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_DIVK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_IDIVK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BANDK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BORK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BXORK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SHRI
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SHLI
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_ADD
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SUB
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_MUL
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_MOD
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_POW
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_DIV
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_IDIV
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BAND
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BOR
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BXOR
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SHL
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_SHR
    opmode(1, 0, 0, 0, 0, OpMode::iABC),  // OP_MMBIN
    opmode(1, 0, 0, 0, 0, OpMode::iABC),  // OP_MMBINI
    opmode(1, 0, 0, 0, 0, OpMode::iABC),  // OP_MMBINK
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_UNM
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_BNOT
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_NOT
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_LEN
    opmode(0, 0, 0, 0, 1, OpMode::iABC),  // OP_CONCAT
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_CLOSE
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_TBC
    opmode(0, 0, 0, 0, 0, OpMode::isJ),   // OP_JMP
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_EQ
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_LT
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_LE
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_EQK
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_EQI
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_LTI
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_LEI
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_GTI
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_GEI
    opmode(0, 0, 0, 1, 0, OpMode::iABC),  // OP_TEST
    opmode(0, 0, 0, 1, 1, OpMode::iABC),  // OP_TESTSET
    opmode(0, 1, 1, 0, 1, OpMode::iABC),  // OP_CALL
    opmode(0, 1, 1, 0, 1, OpMode::iABC),  // OP_TAILCALL
    opmode(0, 0, 1, 0, 0, OpMode::iABC),  // OP_RETURN
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_RETURN0
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_RETURN1
    opmode(0, 0, 0, 0, 1, OpMode::iABx),  // OP_FORLOOP
    opmode(0, 0, 0, 0, 1, OpMode::iABx),  // OP_FORPREP
    opmode(0, 0, 0, 0, 0, OpMode::iABx),  // OP_TFORPREP
    opmode(0, 0, 0, 0, 0, OpMode::iABC),  // OP_TFORCALL
    opmode(0, 0, 0, 0, 1, OpMode::iABx),  // OP_TFORLOOP
    opmode(0, 0, 1, 0, 0, OpMode::iABC),  // OP_SETLIST
    opmode(0, 0, 0, 0, 1, OpMode::iABx),  // OP_CLOSURE
    opmode(0, 1, 0, 0, 1, OpMode::iABC),  // OP_VARARG
    opmode(0, 0, 1, 0, 1, OpMode::iABC),  // OP_VARARGPREP
    opmode(0, 0, 0, 0, 0, OpMode::iAx),   // OP_EXTRAARG
];

fn get_opmode_bits(m: OpCode) -> u8 {
    LUAP_OPMODES.get(m as usize).copied().unwrap_or(0)
}
fn testAMode(m: OpCode) -> bool {
    (get_opmode_bits(m) & (1 << 3)) != 0
}
fn testTMode(m: OpCode) -> bool {
    (get_opmode_bits(m) & (1 << 4)) != 0
}
fn testITMode(m: OpCode) -> bool {
    (get_opmode_bits(m) & (1 << 5)) != 0
}
fn testOTMode(m: OpCode) -> bool {
    (get_opmode_bits(m) & (1 << 6)) != 0
}
fn testMMMode(m: OpCode) -> bool {
    (get_opmode_bits(m) & (1 << 7)) != 0
}
fn isIT(i: Instruction) -> bool {
    testITMode(GET_OPCODE(i)) && GETARG_B(i) == 0
}

// ============================================================================
// Ctype helpers (lctype.c)
// ============================================================================

const ALPHABIT: u8 = 0;
const DIGITBIT: u8 = 1;
const PRINTBIT: u8 = 2;
const SPACEBIT: u8 = 3;
const XDIGITBIT: u8 = 4;

static LUAI_CTYPE: [u8; 257] = [
    0x00, // EOZ
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0c, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x16, 0x16, 0x16, 0x16, 0x16, 0x16, 0x16, 0x16, 0x16, 0x16, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x05,
    0x04, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

fn testprop(c: i32, p: u8) -> bool {
    if c < -1 || c > 255 {
        return false;
    }
    let idx = (c + 1) as usize;
    (LUAI_CTYPE.get(idx).copied().unwrap_or(0) & p) != 0
}
fn lislalpha(c: i32) -> bool {
    testprop(c, 1 << ALPHABIT)
}
fn lislalnum(c: i32) -> bool {
    testprop(c, (1 << ALPHABIT) | (1 << DIGITBIT))
}
fn lisdigit(c: i32) -> bool {
    testprop(c, 1 << DIGITBIT)
}
fn lisspace(c: i32) -> bool {
    testprop(c, 1 << SPACEBIT)
}
fn lisprint(c: i32) -> bool {
    testprop(c, 1 << PRINTBIT)
}
fn lisxdigit(c: i32) -> bool {
    testprop(c, 1 << XDIGITBIT)
}
fn ltolower(c: u8) -> u8 {
    c | (b'A' ^ b'a')
}

fn c_isalpha(c: u8) -> bool {
    c.is_ascii_alphabetic()
}
fn c_iscntrl(c: u8) -> bool {
    c < 0x20 || c == 0x7f
}
fn c_isdigit(c: u8) -> bool {
    c.is_ascii_digit()
}
fn c_isgraph(c: u8) -> bool {
    (0x21..=0x7e).contains(&c)
}
fn c_islower(c: u8) -> bool {
    c.is_ascii_lowercase()
}
fn c_ispunct(c: u8) -> bool {
    c_isgraph(c) && !c_isalnum(c)
}
fn c_isspace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}
fn c_isupper(c: u8) -> bool {
    c.is_ascii_uppercase()
}
fn c_isalnum(c: u8) -> bool {
    c.is_ascii_alphanumeric()
}
fn c_isxdigit(c: u8) -> bool {
    c.is_ascii_hexdigit()
}
fn c_tolower(c: u8) -> u8 {
    c.to_ascii_lowercase()
}
fn c_toupper(c: u8) -> u8 {
    c.to_ascii_uppercase()
}

// ============================================================================
// Object utilities & Number formatting / parsing (lobject.c)
// ============================================================================

static LOG_2: [u8; 256] = [
    0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
];

pub fn luaO_ceillog2(mut x: u32) -> i32 {
    let mut l: i32 = 0;
    x = x.wrapping_sub(1);
    while x >= 256 {
        l += 8;
        x >>= 8;
    }
    l + (LOG_2.get(x as usize).copied().unwrap_or(0) as i32)
}

pub fn luaO_hexavalue(c: u8) -> u8 {
    if lisdigit(c as i32) {
        c.wrapping_sub(b'0')
    } else {
        ltolower(c).wrapping_sub(b'a').wrapping_add(10)
    }
}

pub fn luaO_utf8esc(x_in: u64) -> Vec<u8> {
    let mut x = x_in;
    if x < 0x80 {
        vec![x as u8]
    } else {
        let mut bytes = Vec::with_capacity(8);
        let mut mfb: u64 = 0x3f;
        loop {
            bytes.push(0x80 | ((x & 0x3f) as u8));
            x >>= 6;
            mfb >>= 1;
            if x <= mfb {
                break;
            }
        }
        bytes.push(((!mfb << 1) as u8) | (x as u8));
        bytes.reverse();
        bytes
    }
}

pub fn c_frexp(x: f64) -> (f64, i32) {
    if x == 0.0 || x.is_nan() || x.is_infinite() {
        return (x, 0);
    }
    let bits = x.to_bits();
    let sign = bits & (1u64 << 63);
    let mut exp = ((bits >> 52) & 0x7ff) as i32;
    let mut mant = bits & 0x000f_ffff_ffff_ffff;
    if exp == 0 {
        let shift = mant.leading_zeros() - 11;
        mant <<= shift;
        mant &= 0x000f_ffff_ffff_ffff;
        exp = 1 - (shift as i32);
    }
    let real_exp = exp - 1022;
    let new_bits = sign | (0x3fe_u64 << 52) | mant;
    (f64::from_bits(new_bits), real_exp)
}

pub fn c_ldexp(x: f64, exp: i32) -> f64 {
    if x == 0.0 || x.is_nan() || x.is_infinite() || exp == 0 {
        return x;
    }
    let (m, e) = c_frexp(x);
    let total_e = (e as i64) + (exp as i64);
    if total_e > 1024 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    } else if total_e < -1074 {
        if x.is_sign_negative() {
            -0.0
        } else {
            0.0
        }
    } else {
        // Multiply m by 2^total_e in steps
        let mut res = m;
        let mut rem = total_e;
        while rem > 0 {
            let step = rem.min(1023);
            let factor = f64::from_bits(((step + 1023) as u64) << 52);
            res *= factor;
            rem -= step;
        }
        while rem < 0 {
            let step = rem.max(-1022);
            let factor = f64::from_bits(((step + 1023) as u64) << 52);
            res *= factor;
            rem -= step;
        }
        res
    }
}

pub fn lua_numbertointeger(n: LuaNumber) -> Option<LuaInteger> {
    let min_flt = LuaInteger::MIN as LuaNumber;
    if n >= min_flt && n < -min_flt {
        Some(n as LuaInteger)
    } else {
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum F2Imod {
    F2Ieq,
    F2Ifloor,
    F2Iceil,
}

pub fn luaV_flttointeger(n: LuaNumber, mode: F2Imod) -> Option<LuaInteger> {
    let mut f = n.floor();
    if n != f {
        if mode == F2Imod::F2Ieq {
            return None;
        } else if mode == F2Imod::F2Iceil {
            f += 1.0;
        }
    }
    lua_numbertointeger(f)
}

fn parse_hex_float(s: &[u8]) -> Option<(LuaNumber, usize)> {
    let mut idx = 0;
    while let Some(&b) = s.get(idx) {
        if lisspace(b as i32) {
            idx += 1;
        } else {
            break;
        }
    }
    let mut neg = false;
    if let Some(&b) = s.get(idx) {
        if b == b'-' {
            neg = true;
            idx += 1;
        } else if b == b'+' {
            idx += 1;
        }
    }
    if s.get(idx).copied() != Some(b'0') {
        return None;
    }
    match s.get(idx + 1).copied() {
        Some(b'x') | Some(b'X') => idx += 2,
        _ => return None,
    }

    let mut mant: u64 = 0;
    let mut sigdig: usize = 0;
    let mut nosigdig: usize = 0;
    let mut exp_hex: i64 = 0;
    let mut hasdot = false;
    let mut sticky = false;

    while let Some(&b) = s.get(idx) {
        if b == b'.' {
            if hasdot {
                break;
            }
            hasdot = true;
            idx += 1;
        } else if lisxdigit(b as i32) {
            let d = luaO_hexavalue(b) as u64;
            if sigdig == 0 && d == 0 {
                nosigdig += 1;
            } else {
                sigdig += 1;
                if sigdig <= 15 {
                    mant = (mant << 4) | d;
                } else {
                    if d != 0 {
                        sticky = true;
                    }
                    exp_hex += 1;
                }
            }
            if hasdot {
                exp_hex -= 1;
            }
            idx += 1;
        } else {
            break;
        }
    }
    if nosigdig + sigdig == 0 {
        return None;
    }
    let mut valid_end = idx;
    let mut exp2: i64 = exp_hex * 4;

    if let Some(&b) = s.get(idx) {
        if b == b'p' || b == b'P' {
            idx += 1;
            let mut exp_neg = false;
            if let Some(&sb) = s.get(idx) {
                if sb == b'-' {
                    exp_neg = true;
                    idx += 1;
                } else if sb == b'+' {
                    idx += 1;
                }
            }
            if !s.get(idx).map_or(false, |&c| lisdigit(c as i32)) {
                return None;
            }
            let mut exp1: i64 = 0;
            while let Some(&c) = s.get(idx) {
                if lisdigit(c as i32) {
                    exp1 = exp1.saturating_mul(10).saturating_add((c - b'0') as i64);
                    idx += 1;
                } else {
                    break;
                }
            }
            if exp_neg {
                exp1 = -exp1;
            }
            exp2 = exp2.saturating_add(exp1);
            valid_end = idx;
        }
    }

    if mant == 0 {
        let zero = if neg { -0.0 } else { 0.0 };
        return Some((zero, valid_end));
    }

    let bits = 64 - mant.leading_zeros();
    let unbiased_exp = exp2.saturating_add((bits as i64) - 1);
    if unbiased_exp > 1023 {
        return Some((
            if neg {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            },
            valid_end,
        ));
    }
    if unbiased_exp < -1075 {
        return Some((if neg { -0.0 } else { 0.0 }, valid_end));
    }

    let target_bits: i32 = if unbiased_exp >= -1022 {
        53
    } else {
        (unbiased_exp + 1074) as i32
    };

    let mut final_mant: u64;
    let mut final_exp: i64 = if unbiased_exp >= -1022 {
        unbiased_exp + 1023
    } else {
        0
    };

    let shift = (bits as i32) - target_bits;
    if shift <= 0 {
        final_mant = mant << (-shift as u32);
    } else {
        let sh = shift as u32;
        let round_bit = if sh <= 64 {
            (mant >> (sh - 1)) & 1
        } else {
            0
        };
        let low_mask = if sh - 1 == 0 {
            0
        } else if sh - 1 >= 64 {
            !0u64
        } else {
            (1u64 << (sh - 1)) - 1
        };
        let rem_sticky = ((mant & low_mask) != 0) || sticky;
        final_mant = if sh >= 64 { 0 } else { mant >> sh };
        if round_bit != 0 && (rem_sticky || (final_mant & 1) != 0) {
            final_mant += 1;
            if target_bits == 53 && final_mant >= (1u64 << 53) {
                final_mant >>= 1;
                final_exp += 1;
                if final_exp >= 2047 {
                    return Some((
                        if neg {
                            f64::NEG_INFINITY
                        } else {
                            f64::INFINITY
                        },
                        valid_end,
                    ));
                }
            } else if target_bits < 53 && final_mant >= (1u64 << 52) {
                final_exp = 1;
            }
        }
    }

    let raw_bits = ((if neg { 1u64 } else { 0u64 }) << 63)
        | ((final_exp as u64) << 52)
        | (final_mant & 0x000f_ffff_ffff_ffff);
    Some((f64::from_bits(raw_bits), valid_end))
}

fn l_str2int(s: &[u8]) -> Option<(LuaInteger, usize)> {
    let mut idx = 0;
    while let Some(&b) = s.get(idx) {
        if lisspace(b as i32) {
            idx += 1;
        } else {
            break;
        }
    }
    let mut neg = 0u64;
    if let Some(&b) = s.get(idx) {
        if b == b'-' {
            neg = 1;
            idx += 1;
        } else if b == b'+' {
            idx += 1;
        }
    }
    let mut a: LuaUnsigned = 0;
    let mut empty = true;
    if s.get(idx).copied() == Some(b'0')
        && matches!(s.get(idx + 1).copied(), Some(b'x') | Some(b'X'))
    {
        idx += 2;
        while let Some(&b) = s.get(idx) {
            if lisxdigit(b as i32) {
                a = a.wrapping_mul(16).wrapping_add(luaO_hexavalue(b) as u64);
                empty = false;
                idx += 1;
            } else {
                break;
            }
        }
    } else {
        const MAXBY10: u64 = (LuaInteger::MAX as u64) / 10;
        const MAXLASTD: u64 = (LuaInteger::MAX as u64) % 10;
        while let Some(&b) = s.get(idx) {
            if lisdigit(b as i32) {
                let d = (b - b'0') as u64;
                if a >= MAXBY10 && (a > MAXBY10 || d > MAXLASTD + neg) {
                    return None;
                }
                a = a * 10 + d;
                empty = false;
                idx += 1;
            } else {
                break;
            }
        }
    }
    while let Some(&b) = s.get(idx) {
        if lisspace(b as i32) {
            idx += 1;
        } else {
            break;
        }
    }
    if empty || idx != s.len() {
        None
    } else {
        let res = if neg != 0 {
            0u64.wrapping_sub(a) as LuaInteger
        } else {
            a as LuaInteger
        };
        Some((res, idx))
    }
}

fn l_str2d(s: &[u8]) -> Option<(LuaNumber, usize)> {
    // C stops at first NUL byte
    let s_nul_len = s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let s_c = s.get(..s_nul_len)?;
    let mut mode: u8 = 0;
    for &b in s_c {
        if matches!(b, b'.' | b'x' | b'X' | b'n' | b'N') {
            mode = ltolower(b);
            break;
        }
    }
    if mode == b'n' {
        return None;
    }
    if mode == b'x' {
        let (n, mut end_idx) = parse_hex_float(s_c)?;
        while let Some(&b) = s_c.get(end_idx) {
            if lisspace(b as i32) {
                end_idx += 1;
            } else {
                break;
            }
        }
        if end_idx == s_c.len() {
            Some((n, end_idx))
        } else {
            None
        }
    } else {
        let mut idx = 0;
        while let Some(&b) = s_c.get(idx) {
            if lisspace(b as i32) {
                idx += 1;
            } else {
                break;
            }
        }
        let start = idx;
        if let Some(&b) = s_c.get(idx) {
            if b == b'-' || b == b'+' {
                idx += 1;
            }
        }
        let mut has_digits = false;
        while let Some(&b) = s_c.get(idx) {
            if lisdigit(b as i32) {
                has_digits = true;
                idx += 1;
            } else {
                break;
            }
        }
        if s_c.get(idx).copied() == Some(b'.') {
            idx += 1;
            while let Some(&b) = s_c.get(idx) {
                if lisdigit(b as i32) {
                    has_digits = true;
                    idx += 1;
                } else {
                    break;
                }
            }
        }
        if !has_digits {
            return None;
        }
        if matches!(s_c.get(idx).copied(), Some(b'e') | Some(b'E')) {
            idx += 1;
            if matches!(s_c.get(idx).copied(), Some(b'-') | Some(b'+')) {
                idx += 1;
            }
            let mut exp_digits = false;
            while let Some(&b) = s_c.get(idx) {
                if lisdigit(b as i32) {
                    exp_digits = true;
                    idx += 1;
                } else {
                    break;
                }
            }
            if !exp_digits {
                return None;
            }
        }
        let num_slice = s_c.get(start..idx)?;
        let num_str = std::str::from_utf8(num_slice).ok()?;
        let n: f64 = num_str.parse().ok()?;
        while let Some(&b) = s_c.get(idx) {
            if lisspace(b as i32) {
                idx += 1;
            } else {
                break;
            }
        }
        if idx == s_c.len() {
            Some((n, idx))
        } else {
            None
        }
    }
}

pub fn luaO_str2num(s: &[u8]) -> Option<(TValue, usize)> {
    let s_nul_len = s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let s_c = s.get(..s_nul_len)?;
    if let Some((i, end)) = l_str2int(s_c) {
        Some((TValue::int_val(i), end + 1))
    } else if let Some((n, end)) = l_str2d(s_c) {
        Some((TValue::flt_val(n), end + 1))
    } else {
        None
    }
}

// ============================================================================
// C printf / snprintf emulation for numbers and string.format
// ============================================================================

pub fn format_float_g14(n: f64) -> String {
    let mut s = c_format_float(n, false, false, false, false, false, None, Some(14), b'g');
    let looks_like_int = s
        .bytes()
        .all(|b| b == b'-' || b.is_ascii_digit());
    if looks_like_int {
        s.push_str(".0");
    }
    s
}

pub fn tostringbuff(obj: &TValue) -> String {
    if obj.is_integer() {
        obj.ivalue().to_string()
    } else {
        format_float_g14(obj.fltvalue())
    }
}

fn c_format_float(
    n: f64,
    flag_minus: bool,
    flag_plus: bool,
    flag_space: bool,
    flag_hash: bool,
    flag_zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
    spec: u8,
) -> String {
    let is_upper = spec.is_ascii_uppercase();
    let is_neg = n.to_bits() >> 63 != 0;
    let sign_str = if is_neg {
        "-"
    } else if flag_plus {
        "+"
    } else if flag_space {
        " "
    } else {
        ""
    };

    if n.is_nan() || n.is_infinite() {
        let body = if n.is_nan() {
            if is_upper {
                "NAN"
            } else {
                "nan"
            }
        } else if is_upper {
            "INF"
        } else {
            "inf"
        };
        let full = format!("{sign_str}{body}");
        let w = width.unwrap_or(0);
        if full.len() >= w {
            return full;
        }
        let pad = " ".repeat(w - full.len());
        return if flag_minus {
            format!("{full}{pad}")
        } else {
            format!("{pad}{full}")
        };
    }

    let abs_n = n.abs();
    let mut prefix = String::from(sign_str);
    let body = match spec {
        b'f' | b'F' => {
            let prec = precision.unwrap_or(6);
            let mut s = format!("{abs_n:.prec$}");
            if prec == 0 && flag_hash {
                s.push('.');
            }
            s
        }
        b'e' | b'E' => {
            let prec = precision.unwrap_or(6);
            format_exp_style(abs_n, prec, flag_hash, false, is_upper)
        }
        b'g' | b'G' => {
            let prec = match precision {
                None => 6,
                Some(0) => 1,
                Some(p) => p,
            };
            format_g_style(abs_n, prec, flag_hash, is_upper)
        }
        b'a' | b'A' => {
            prefix.push_str(if is_upper { "0X" } else { "0x" });
            format_hex_style(abs_n, precision, flag_hash, is_upper)
        }
        _ => String::new(),
    };

    let total_len = prefix.len() + body.len();
    let w = width.unwrap_or(0);
    if total_len >= w {
        format!("{prefix}{body}")
    } else {
        let pad_len = w - total_len;
        if flag_minus {
            format!("{prefix}{body}{}", " ".repeat(pad_len))
        } else if flag_zero {
            format!("{prefix}{}{body}", "0".repeat(pad_len))
        } else {
            format!("{}{prefix}{body}", " ".repeat(pad_len))
        }
    }
}

fn format_exp_style(
    abs_n: f64,
    prec: usize,
    flag_hash: bool,
    strip_zeros: bool,
    is_upper: bool,
) -> String {
    let raw = format!("{abs_n:.prec$e}");
    let (mant_part, exp_part) = match raw.find('e') {
        Some(pos) => (
            raw.get(..pos).unwrap_or(""),
            raw.get(pos + 1..).unwrap_or("0"),
        ),
        None => (raw.as_str(), "0"),
    };
    let exp_val: i32 = exp_part.parse().unwrap_or(0);
    let mut mant = String::from(mant_part);
    if strip_zeros && !flag_hash {
        if mant.contains('.') {
            while mant.ends_with('0') {
                mant.pop();
            }
            if mant.ends_with('.') {
                mant.pop();
            }
        }
    } else if prec == 0 && flag_hash && !mant.contains('.') {
        mant.push('.');
    }
    let exp_ch = if is_upper { 'E' } else { 'e' };
    format!("{mant}{exp_ch}{exp_val:+03}")
}

fn format_g_style(abs_n: f64, prec: usize, flag_hash: bool, is_upper: bool) -> String {
    let exp_prec = prec.saturating_sub(1);
    let raw = format!("{abs_n:.exp_prec$e}");
    let exp_val: i32 = match raw.find('e') {
        Some(pos) => raw.get(pos + 1..).unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    };
    if exp_val >= -4 && exp_val < (prec as i32) {
        let frac_digits = ((prec as i32) - 1 - exp_val).max(0) as usize;
        let mut s = format!("{abs_n:.frac_digits$}");
        if !flag_hash {
            if s.contains('.') {
                while s.ends_with('0') {
                    s.pop();
                }
                if s.ends_with('.') {
                    s.pop();
                }
            }
        } else if frac_digits == 0 && !s.contains('.') {
            s.push('.');
        }
        s
    } else {
        format_exp_style(abs_n, exp_prec, flag_hash, true, is_upper)
    }
}

fn format_hex_style(
    abs_n: f64,
    precision: Option<usize>,
    flag_hash: bool,
    is_upper: bool,
) -> String {
    let p_ch = if is_upper { 'P' } else { 'p' };
    if abs_n == 0.0 {
        let mut s = String::from("0");
        match precision {
            None => {
                if flag_hash {
                    s.push('.');
                }
            }
            Some(0) => {
                if flag_hash {
                    s.push('.');
                }
            }
            Some(p) => {
                s.push('.');
                s.push_str(&"0".repeat(p));
            }
        }
        s.push(p_ch);
        s.push_str("+0");
        return s;
    }

    let bits = abs_n.to_bits();
    let biased_exp = ((bits >> 52) & 0x7ff) as i32;
    let mut mant = bits & 0x000f_ffff_ffff_ffff;
    let mut leading: u64 = if biased_exp == 0 { 0 } else { 1 };
    let exp2: i32 = if biased_exp == 0 {
        -1022
    } else {
        biased_exp - 1023
    };

    let mut s = String::new();
    match precision {
        None => {
            let lead_ch = std::char::from_digit(leading as u32, 16).unwrap_or('1');
            s.push(if is_upper {
                lead_ch.to_ascii_uppercase()
            } else {
                lead_ch
            });
            let mut nibbles = if is_upper {
                format!("{mant:013X}")
            } else {
                format!("{mant:013x}")
            };
            while nibbles.ends_with('0') {
                nibbles.pop();
            }
            if !nibbles.is_empty() || flag_hash {
                s.push('.');
                s.push_str(&nibbles);
            }
        }
        Some(p) => {
            if p < 13 {
                let shift = (13 - p) * 4;
                let half = 1u64 << (shift - 1);
                let mask = (1u64 << shift) - 1;
                let rem = mant & mask;
                let mut rounded = if shift >= 64 { 0 } else { mant >> shift };
                let odd = if p == 0 {
                    (leading & 1) != 0
                } else {
                    (rounded & 1) != 0
                };
                if rem > half || (rem == half && odd) {
                    if p == 0 {
                        leading += 1;
                    } else {
                        rounded += 1;
                        if rounded >= (1u64 << (p * 4)) {
                            leading += 1;
                            rounded = 0;
                        }
                    }
                }
                mant = rounded;
            }
            let lead_ch = std::char::from_digit(leading as u32, 16).unwrap_or('1');
            s.push(if is_upper {
                lead_ch.to_ascii_uppercase()
            } else {
                lead_ch
            });
            if p == 0 {
                if flag_hash {
                    s.push('.');
                }
            } else {
                s.push('.');
                if p <= 13 {
                    let hex_s = if is_upper {
                        format!("{mant:0p$X}", p = p)
                    } else {
                        format!("{mant:0p$x}", p = p)
                    };
                    s.push_str(&hex_s);
                } else {
                    let hex_s = if is_upper {
                        format!("{mant:013X}")
                    } else {
                        format!("{mant:013x}")
                    };
                    s.push_str(&hex_s);
                    s.push_str(&"0".repeat(p - 13));
                }
            }
        }
    }
    s.push(p_ch);
    s.push_str(&format!("{exp2:+}"));
    s
}

fn c_format_int(
    n: LuaInteger,
    flag_minus: bool,
    flag_plus: bool,
    flag_space: bool,
    flag_hash: bool,
    flag_zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
    spec: u8,
) -> String {
    let is_signed = matches!(spec, b'd' | b'i');
    let mut prefix = String::new();
    let uval: u64 = if is_signed {
        if n < 0 {
            prefix.push('-');
            0u64.wrapping_sub(n as u64)
        } else {
            if flag_plus {
                prefix.push('+');
            } else if flag_space {
                prefix.push(' ');
            }
            n as u64
        }
    } else {
        n as u64
    };

    let mut digits = if uval == 0 && precision == Some(0) {
        String::new()
    } else {
        match spec {
            b'd' | b'i' | b'u' => uval.to_string(),
            b'o' => format!("{uval:o}"),
            b'x' => format!("{uval:x}"),
            b'X' => format!("{uval:X}"),
            _ => String::new(),
        }
    };

    if flag_hash {
        match spec {
            b'o' => {
                if !digits.starts_with('0') {
                    digits.insert(0, '0');
                }
            }
            b'x' => {
                if uval != 0 {
                    prefix.push_str("0x");
                }
            }
            b'X' => {
                if uval != 0 {
                    prefix.push_str("0X");
                }
            }
            _ => {}
        }
    }

    if let Some(p) = precision {
        if digits.len() < p {
            digits = format!("{}{digits}", "0".repeat(p - digits.len()));
        }
    }

    let total_len = prefix.len() + digits.len();
    let w = width.unwrap_or(0);
    if total_len >= w {
        format!("{prefix}{digits}")
    } else {
        let pad_len = w - total_len;
        if flag_minus {
            format!("{prefix}{digits}{}", " ".repeat(pad_len))
        } else if flag_zero && precision.is_none() {
            format!("{prefix}{}{digits}", "0".repeat(pad_len))
        } else {
            format!("{}{prefix}{digits}", " ".repeat(pad_len))
        }
    }
}

pub fn luaO_chunkid(source: &[u8]) -> String {
    const RETS: &str = "...";
    const PRE: &str = "[string \"";
    const POS: &str = "\"]";
    let bufflen = LUA_IDSIZE;
    let srclen = source.len() + 1; // +1 for terminating '\0' as in C
    let first = source.first().copied().unwrap_or(0);
    if first == b'=' {
        let rest = source.get(1..).unwrap_or(&[]);
        if srclen <= bufflen {
            String::from_utf8_lossy(rest).into_owned()
        } else {
            let take = (bufflen - 1).min(rest.len());
            String::from_utf8_lossy(rest.get(..take).unwrap_or(&[])).into_owned()
        }
    } else if first == b'@' {
        let rest = source.get(1..).unwrap_or(&[]);
        if srclen <= bufflen {
            String::from_utf8_lossy(rest).into_owned()
        } else {
            let rem = bufflen - RETS.len();
            let take_bytes = rem - 1;
            let start = rest.len().saturating_sub(take_bytes);
            format!(
                "{RETS}{}",
                String::from_utf8_lossy(rest.get(start..).unwrap_or(&[]))
            )
        }
    } else {
        let nl_pos = source.iter().position(|&b| b == b'\n');
        let max_src = bufflen - (PRE.len() + RETS.len() + POS.len() + 1);
        if srclen < max_src && nl_pos.is_none() {
            format!("{PRE}{}{POS}", String::from_utf8_lossy(source))
        } else {
            let mut len = nl_pos.unwrap_or(source.len());
            if len > max_src {
                len = max_src;
            }
            format!(
                "{PRE}{}{RETS}{POS}",
                String::from_utf8_lossy(source.get(..len).unwrap_or(&[]))
            )
        }
    }
}

// ============================================================================
// String table & Hashing (lstring.c)
// ============================================================================

pub fn luaS_hash(str_bytes: &[u8], seed: u32) -> u32 {
    let mut l = str_bytes.len();
    let mut h: u32 = seed ^ (l as u32);
    while l > 0 {
        let b = str_bytes.get(l - 1).copied().unwrap_or(0) as u32;
        h ^= (h << 5).wrapping_add(h >> 2).wrapping_add(b);
        l -= 1;
    }
    h
}

impl LuaVM {
    pub fn luaS_newlstr(&mut self, bytes: &[u8]) -> StrId {
        if bytes.len() <= LUAI_MAXSHORTLEN {
            if let Some(&id) = self.short_str_map.get(bytes) {
                return id;
            }
            let h = luaS_hash(bytes, self.seed);
            let id = self.strings.len();
            self.strings.push(TString {
                tt: LUA_VSHRSTR,
                extra: 0,
                is_short: true,
                hash: h,
                contents: bytes.to_vec(),
            });
            self.short_str_map.insert(bytes.to_vec(), id);
            id
        } else {
            let id = self.strings.len();
            self.strings.push(TString {
                tt: LUA_VLNGSTR,
                extra: 0,
                is_short: false,
                hash: self.seed,
                contents: bytes.to_vec(),
            });
            id
        }
    }

    pub fn luaS_new(&mut self, s: &str) -> StrId {
        self.luaS_newlstr(s.as_bytes())
    }

    pub fn luaS_newlstr_val(&mut self, bytes: &[u8]) -> TValue {
        let is_short = bytes.len() <= LUAI_MAXSHORTLEN;
        let id = self.luaS_newlstr(bytes);
        TValue::str_val(id, is_short)
    }

    pub fn luaS_hashlongstr(&mut self, id: StrId) -> u32 {
        if let Some(ts) = self.strings.get_mut(id) {
            if !ts.is_short && ts.extra == 0 {
                ts.hash = luaS_hash(&ts.contents, ts.hash);
                ts.extra = 1;
            }
            ts.hash
        } else {
            0
        }
    }

    pub fn luaS_eqlngstr(&self, a: StrId, b: StrId) -> bool {
        if a == b {
            return true;
        }
        match (self.strings.get(a), self.strings.get(b)) {
            (Some(sa), Some(sb)) => sa.contents == sb.contents,
            _ => false,
        }
    }

    pub fn get_str_bytes(&self, id: StrId) -> &[u8] {
        match self.strings.get(id) {
            Some(ts) => &ts.contents,
            None => &[],
        }
    }

    pub fn get_str_lossy(&self, id: StrId) -> String {
        String::from_utf8_lossy(self.get_str_bytes(id)).into_owned()
    }
}

// ============================================================================
// Tables (ltable.c)
// ============================================================================

const MAXABITS: usize = 31;
const MAXASIZE: u32 = 1u32 << MAXABITS;
const MAXHBITS: i32 = (MAXABITS as i32) - 1;
const MAXHSIZE: u32 = 1u32 << MAXHBITS;

const fn ispow2(x: u32) -> bool {
    (x & x.wrapping_sub(1)) == 0
}

fn l_hashfloat(n: LuaNumber) -> i32 {
    let (m, i) = c_frexp(n);
    let scaled = m * -(i32::MIN as LuaNumber);
    match lua_numbertointeger(scaled) {
        None => 0,
        Some(ni) => {
            let u = (i as u32).wrapping_add(ni as u32);
            (if u <= (i32::MAX as u32) { u } else { !u }) as i32
        }
    }
}

fn arrayindex(k: LuaInteger) -> u32 {
    if (k as LuaUnsigned).wrapping_sub(1) < (MAXASIZE as LuaUnsigned) {
        k as u32
    } else {
        0
    }
}

fn computesizes(nums: &[u32; MAXABITS + 1], pna: &mut u32) -> u32 {
    let mut twotoi: u32 = 1;
    let mut a: u32 = 0;
    let mut na: u32 = 0;
    let mut optimal: u32 = 0;
    let mut i: usize = 0;
    while twotoi > 0 && *pna > twotoi / 2 {
        a += nums.get(i).copied().unwrap_or(0);
        if a > twotoi / 2 {
            optimal = twotoi;
            na = a;
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2);
    }
    *pna = na;
    optimal
}

fn countint(key: LuaInteger, nums: &mut [u32; MAXABITS + 1]) -> u32 {
    let k = arrayindex(key);
    if k != 0 {
        let idx = luaO_ceillog2(k) as usize;
        if let Some(slot) = nums.get_mut(idx) {
            *slot += 1;
        }
        1
    } else {
        0
    }
}

impl Table {
    pub fn new_empty() -> Self {
        Table {
            flags: MASKFLAGS,
            lsizenode: 0,
            alimit: 0,
            array: Vec::new(),
            node: vec![Node::dummy()],
            lastfree: 0,
            is_dummy: true,
            metatable: None,
        }
    }
    pub fn isrealasize(&self) -> bool {
        (self.flags & BITRAS) == 0
    }
    pub fn setrealasize(&mut self) {
        self.flags &= !BITRAS;
    }
    pub fn setnorealasize(&mut self) {
        self.flags |= BITRAS;
    }
    pub fn limitequalsasize(&self) -> bool {
        self.isrealasize() || ispow2(self.alimit)
    }
    pub fn realasize(&self) -> u32 {
        if self.limitequalsasize() {
            self.alimit
        } else {
            let mut size = self.alimit;
            size |= size >> 1;
            size |= size >> 2;
            size |= size >> 4;
            size |= size >> 8;
            size |= size >> 16;
            size += 1;
            size
        }
    }
    pub fn ispow2realasize(&self) -> bool {
        !self.isrealasize() || ispow2(self.alimit)
    }
    pub fn setlimittosize(&mut self) -> u32 {
        self.alimit = self.realasize();
        self.setrealasize();
        self.alimit
    }
    pub fn sizenode(&self) -> usize {
        1usize << (self.lsizenode as usize)
    }
    pub fn allocsizenode(&self) -> u32 {
        if self.is_dummy {
            0
        } else {
            self.sizenode() as u32
        }
    }
    pub fn invalidate_tm_cache(&mut self) {
        self.flags &= !MASKFLAGS;
    }
}

impl LuaVM {
    pub fn luaH_new(&mut self) -> TableId {
        let id = self.tables.len();
        self.tables.push(Table::new_empty());
        id
    }

    pub fn mainpositionTV(&mut self, t_id: TableId, key: &TValue) -> usize {
        let sizenode = self.tables.get(t_id).map_or(1, |t| t.sizenode());
        let mod_size = ((sizenode - 1) | 1) as u64;
        match key.ttypetag() {
            LUA_VNUMINT => {
                let ui = key.ivalue() as u64;
                if ui <= (i32::MAX as u64) {
                    ((ui as u32 as u64) % mod_size) as usize
                } else {
                    (ui % mod_size) as usize
                }
            }
            LUA_VNUMFLT => {
                let h = l_hashfloat(key.fltvalue()) as u32 as u64;
                (h % mod_size) as usize
            }
            LUA_VSHRSTR => {
                let h = self
                    .strings
                    .get(key.tsvalue())
                    .map_or(0, |s| s.hash) as usize;
                h & (sizenode - 1)
            }
            LUA_VLNGSTR => {
                let h = self.luaS_hashlongstr(key.tsvalue()) as usize;
                h & (sizenode - 1)
            }
            LUA_VFALSE => 0,
            LUA_VTRUE => 1 & (sizenode - 1),
            _ => {
                let h = key.point2uint() as u64;
                (h % mod_size) as usize
            }
        }
    }

    pub fn equalkey(&self, k1: &TValue, n2: &Node, deadok: bool) -> bool {
        if k1.rawtt() != n2.key_tt && !(deadok && n2.key_is_dead() && k1.is_collectable()) {
            return false;
        }
        match n2.key_tt {
            LUA_VNIL | LUA_VFALSE | LUA_VTRUE => true,
            LUA_VNUMINT => k1.ivalue() == n2.key_ival(),
            LUA_VNUMFLT => match n2.key_val {
                ValueData::Flt(f2) => k1.fltvalue() == f2,
                _ => false,
            },
            LUA_VLIGHTUSERDATA => match n2.key_val {
                ValueData::LightUdata(p2) => k1.pvalue() == p2,
                _ => false,
            },
            LUA_VLCF => match (k1.val, n2.key_val) {
                (ValueData::Lcf(f1), ValueData::Lcf(f2)) => std::ptr::fn_addr_eq(f1, f2),
                _ => false,
            },
            x if x == ctb(LUA_VLNGSTR) => self.luaS_eqlngstr(k1.tsvalue(), n2.key_strval()),
            _ => match (k1.val, n2.key_val) {
                (ValueData::Str(a), ValueData::Str(b)) => a == b,
                (ValueData::Table(a), ValueData::Table(b)) => a == b,
                (ValueData::LClosure(a), ValueData::LClosure(b)) => a == b,
                (ValueData::CClosure(a), ValueData::CClosure(b)) => a == b,
                (ValueData::Udata(a), ValueData::Udata(b)) => a == b,
                (ValueData::Thread(a), ValueData::Thread(b)) => a == b,
                _ => false,
            },
        }
    }

    pub fn getgeneric(&mut self, t_id: TableId, key: &TValue, deadok: bool) -> (TValue, SlotLoc) {
        let mut n_idx = self.mainpositionTV(t_id, key);
        loop {
            let node = match self.tables.get(t_id).and_then(|t| t.node.get(n_idx)).copied() {
                Some(nd) => nd,
                None => return (TValue::abstkey(), SlotLoc::AbsentKey),
            };
            if self.equalkey(key, &node, deadok) {
                return (node.i_val, SlotLoc::NodeVal(t_id, n_idx));
            } else {
                let nx = node.next;
                if nx == 0 {
                    return (TValue::abstkey(), SlotLoc::AbsentKey);
                }
                n_idx = ((n_idx as i32) + nx) as usize;
            }
        }
    }

    pub fn luaH_getint(&mut self, t_id: TableId, key: LuaInteger) -> (TValue, SlotLoc) {
        let (alimit, is_real) = match self.tables.get(t_id) {
            Some(t) => (t.alimit as u64, t.isrealasize()),
            None => return (TValue::abstkey(), SlotLoc::AbsentKey),
        };
        let ukey = key as u64;
        if ukey.wrapping_sub(1) < alimit {
            let idx = (ukey - 1) as usize;
            let v = self
                .tables
                .get(t_id)
                .and_then(|t| t.array.get(idx))
                .copied()
                .unwrap_or_else(TValue::empty);
            return (v, SlotLoc::Array(t_id, idx));
        } else if !is_real && ((ukey.wrapping_sub(1) & !(alimit.wrapping_sub(1))) < alimit) {
            if let Some(t) = self.tables.get_mut(t_id) {
                t.alimit = key as u32;
            }
            let idx = (ukey - 1) as usize;
            let v = self
                .tables
                .get(t_id)
                .and_then(|t| t.array.get(idx))
                .copied()
                .unwrap_or_else(TValue::empty);
            return (v, SlotLoc::Array(t_id, idx));
        } else {
            let sizenode = self.tables.get(t_id).map_or(1, |t| t.sizenode());
            let mod_size = ((sizenode - 1) | 1) as u64;
            let mut n_idx = if ukey <= (i32::MAX as u64) {
                ((ukey as u32 as u64) % mod_size) as usize
            } else {
                (ukey % mod_size) as usize
            };
            loop {
                let node = match self.tables.get(t_id).and_then(|t| t.node.get(n_idx)).copied() {
                    Some(nd) => nd,
                    None => break,
                };
                if node.key_is_integer() && node.key_ival() == key {
                    return (node.i_val, SlotLoc::NodeVal(t_id, n_idx));
                } else {
                    let nx = node.next;
                    if nx == 0 {
                        break;
                    }
                    n_idx = ((n_idx as i32) + nx) as usize;
                }
            }
            (TValue::abstkey(), SlotLoc::AbsentKey)
        }
    }

    pub fn luaH_getshortstr(&self, t_id: TableId, key: StrId) -> (TValue, SlotLoc) {
        let hash = self.strings.get(key).map_or(0, |s| s.hash) as usize;
        let t = match self.tables.get(t_id) {
            Some(tb) => tb,
            None => return (TValue::abstkey(), SlotLoc::AbsentKey),
        };
        let mut n_idx = hash & (t.sizenode() - 1);
        loop {
            let node = match t.node.get(n_idx) {
                Some(nd) => nd,
                None => return (TValue::abstkey(), SlotLoc::AbsentKey),
            };
            if node.key_is_shrstr() && node.key_strval() == key {
                return (node.i_val, SlotLoc::NodeVal(t_id, n_idx));
            } else {
                let nx = node.next;
                if nx == 0 {
                    return (TValue::abstkey(), SlotLoc::AbsentKey);
                }
                n_idx = ((n_idx as i32) + nx) as usize;
            }
        }
    }

    pub fn luaH_getstr(&mut self, t_id: TableId, key: StrId) -> (TValue, SlotLoc) {
        let is_short = self.strings.get(key).map_or(true, |s| s.is_short);
        if is_short {
            self.luaH_getshortstr(t_id, key)
        } else {
            let ko = TValue::str_val(key, false);
            self.getgeneric(t_id, &ko, false)
        }
    }

    pub fn luaH_get(&mut self, t_id: TableId, key: &TValue) -> (TValue, SlotLoc) {
        match key.ttypetag() {
            LUA_VSHRSTR => self.luaH_getshortstr(t_id, key.tsvalue()),
            LUA_VNUMINT => self.luaH_getint(t_id, key.ivalue()),
            LUA_VNIL => (TValue::abstkey(), SlotLoc::AbsentKey),
            LUA_VNUMFLT => {
                if let Some(k) = luaV_flttointeger(key.fltvalue(), F2Imod::F2Ieq) {
                    self.luaH_getint(t_id, k)
                } else {
                    self.getgeneric(t_id, key, false)
                }
            }
            _ => self.getgeneric(t_id, key, false),
        }
    }

    pub fn write_slot(&mut self, slot: SlotLoc, val: TValue) {
        match slot {
            SlotLoc::Array(t_id, idx) => {
                if let Some(t) = self.tables.get_mut(t_id) {
                    if let Some(elem) = t.array.get_mut(idx) {
                        *elem = val;
                    }
                }
            }
            SlotLoc::NodeVal(t_id, idx) => {
                if let Some(t) = self.tables.get_mut(t_id) {
                    if let Some(nd) = t.node.get_mut(idx) {
                        nd.i_val = val;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn luaH_resize(
        &mut self,
        l: ThreadId,
        t_id: TableId,
        newasize: u32,
        nhsize: u32,
    ) -> LuaResult<()> {
        let oldasize = match self.tables.get_mut(t_id) {
            Some(t) => t.setlimittosize(),
            None => return Ok(()),
        };

        let (new_nodes, new_lsize, new_lastfree, new_is_dummy) = if nhsize == 0 {
            (vec![Node::dummy()], 0u8, 0usize, true)
        } else {
            let lsize = luaO_ceillog2(nhsize);
            if lsize > MAXHBITS || (1u32 << lsize) > MAXHSIZE {
                return self.luaG_runerror(l, "table overflow");
            }
            let size = 1usize << (lsize as usize);
            (vec![Node::empty_node(); size], lsize as u8, size, false)
        };

        let (old_nodes, old_is_dummy) = match self.tables.get_mut(t_id) {
            Some(t) => {
                let on = std::mem::replace(&mut t.node, new_nodes);
                let oid = t.is_dummy;
                t.lsizenode = new_lsize;
                t.lastfree = new_lastfree;
                t.is_dummy = new_is_dummy;
                (on, oid)
            }
            None => return Ok(()),
        };

        if newasize < oldasize {
            if let Some(t) = self.tables.get_mut(t_id) {
                t.alimit = newasize;
            }
            for i in newasize..oldasize {
                let val = self
                    .tables
                    .get(t_id)
                    .and_then(|t| t.array.get(i as usize))
                    .copied()
                    .unwrap_or_else(TValue::empty);
                if !val.is_empty() {
                    self.luaH_setint(l, t_id, (i as LuaInteger) + 1, val)?;
                }
            }
        }

        if let Some(t) = self.tables.get_mut(t_id) {
            t.array.resize(newasize as usize, TValue::empty());
            t.alimit = newasize;
        }

        if !old_is_dummy {
            for old_node in old_nodes {
                if !old_node.i_val.is_empty() {
                    let k = old_node.get_key();
                    let v = old_node.i_val;
                    self.luaH_set(l, t_id, &k, v)?;
                }
            }
        }
        Ok(())
    }

    pub fn luaH_resizearray(&mut self, l: ThreadId, t_id: TableId, nasize: u32) -> LuaResult<()> {
        let nsize = self.tables.get(t_id).map_or(0, |t| t.allocsizenode());
        self.luaH_resize(l, t_id, nasize, nsize)
    }

    fn rehash(&mut self, l: ThreadId, t_id: TableId, ek: &TValue) -> LuaResult<()> {
        let mut nums = [0u32; MAXABITS + 1];
        let asize_limit = match self.tables.get_mut(t_id) {
            Some(t) => t.setlimittosize(),
            None => 0,
        };
        let mut na: u32 = 0;
        let mut i: u32 = 1;
        let mut lg: usize = 0;
        let mut ttlg: u32 = 1;
        while lg <= MAXABITS {
            let mut lc: u32 = 0;
            let mut lim = ttlg;
            if lim > asize_limit {
                lim = asize_limit;
                if i > lim {
                    break;
                }
            }
            while i <= lim {
                let is_emp = self
                    .tables
                    .get(t_id)
                    .and_then(|t| t.array.get((i - 1) as usize))
                    .map_or(true, |v| v.is_empty());
                if !is_emp {
                    lc += 1;
                }
                i += 1;
            }
            if let Some(slot) = nums.get_mut(lg) {
                *slot += lc;
            }
            na += lc;
            lg += 1;
            ttlg = ttlg.wrapping_mul(2);
        }

        let mut totaluse: u32 = na;
        let sizenode = self.tables.get(t_id).map_or(0, |t| t.sizenode());
        let mut j = sizenode;
        while j > 0 {
            j -= 1;
            if let Some(nd) = self.tables.get(t_id).and_then(|t| t.node.get(j)).copied() {
                if !nd.i_val.is_empty() {
                    if nd.key_is_integer() {
                        na += countint(nd.key_ival(), &mut nums);
                    }
                    totaluse += 1;
                }
            }
        }
        if ek.is_integer() {
            na += countint(ek.ivalue(), &mut nums);
        }
        totaluse += 1;
        let asize = computesizes(&nums, &mut na);
        self.luaH_resize(l, t_id, asize, totaluse - na)
    }

    fn getfreepos(&mut self, t_id: TableId) -> Option<usize> {
        let t = self.tables.get_mut(t_id)?;
        if !t.is_dummy {
            while t.lastfree > 0 {
                t.lastfree -= 1;
                let lf = t.lastfree;
                if t.node.get(lf).map_or(false, |nd| nd.key_is_nil()) {
                    return Some(lf);
                }
            }
        }
        None
    }

    pub fn luaH_newkey(
        &mut self,
        l: ThreadId,
        t_id: TableId,
        key_in: &TValue,
        value: TValue,
    ) -> LuaResult<()> {
        let mut key = *key_in;
        if key.is_nil() {
            return self.luaG_runerror(l, "table index is nil");
        } else if key.is_float() {
            let f = key.fltvalue();
            if let Some(k) = luaV_flttointeger(f, F2Imod::F2Ieq) {
                key = TValue::int_val(k);
            } else if f.is_nan() {
                return self.luaG_runerror(l, "table index is NaN");
            }
        }
        if value.is_nil() {
            return Ok(());
        }
        let mut mp = self.mainpositionTV(t_id, &key);
        let mp_occupied = self
            .tables
            .get(t_id)
            .map_or(true, |t| t.is_dummy || !t.node.get(mp).map_or(true, |nd| nd.i_val.is_empty()));
        if mp_occupied {
            let f = match self.getfreepos(t_id) {
                Some(pos) => pos,
                None => {
                    self.rehash(l, t_id, &key)?;
                    return self.luaH_set(l, t_id, &key, value);
                }
            };
            let mp_node = self
                .tables
                .get(t_id)
                .and_then(|t| t.node.get(mp))
                .copied()
                .unwrap_or_else(Node::empty_node);
            let mp_key = mp_node.get_key();
            let mut othern = self.mainpositionTV(t_id, &mp_key);
            if othern != mp {
                while ((othern as i32)
                    + self
                        .tables
                        .get(t_id)
                        .and_then(|t| t.node.get(othern))
                        .map_or(0, |nd| nd.next)) as usize
                    != mp
                {
                    let nx = self
                        .tables
                        .get(t_id)
                        .and_then(|t| t.node.get(othern))
                        .map_or(0, |nd| nd.next);
                    othern = ((othern as i32) + nx) as usize;
                }
                if let Some(t) = self.tables.get_mut(t_id) {
                    if let Some(ond) = t.node.get_mut(othern) {
                        ond.next = (f as i32) - (othern as i32);
                    }
                    if let Some(fnd) = t.node.get_mut(f) {
                        *fnd = mp_node;
                        if mp_node.next != 0 {
                            fnd.next += (mp as i32) - (f as i32);
                        }
                    }
                    if let Some(mnd) = t.node.get_mut(mp) {
                        if mp_node.next != 0 {
                            mnd.next = 0;
                        }
                        mnd.i_val = TValue::empty();
                    }
                }
            } else {
                if let Some(t) = self.tables.get_mut(t_id) {
                    if mp_node.next != 0 {
                        if let Some(fnd) = t.node.get_mut(f) {
                            fnd.next = ((mp as i32) + mp_node.next) - (f as i32);
                        }
                    }
                    if let Some(mnd) = t.node.get_mut(mp) {
                        mnd.next = (f as i32) - (mp as i32);
                    }
                }
                mp = f;
            }
        }
        if let Some(t) = self.tables.get_mut(t_id) {
            if let Some(mnd) = t.node.get_mut(mp) {
                mnd.set_key(key);
                mnd.i_val = value;
            }
        }
        Ok(())
    }

    pub fn luaH_finishset(
        &mut self,
        l: ThreadId,
        t_id: TableId,
        key: &TValue,
        slot_val: &TValue,
        slot_loc: SlotLoc,
        value: TValue,
    ) -> LuaResult<()> {
        if slot_val.is_abstkey() || slot_loc == SlotLoc::AbsentKey {
            self.luaH_newkey(l, t_id, key, value)
        } else {
            self.write_slot(slot_loc, value);
            Ok(())
        }
    }

    pub fn luaH_set(
        &mut self,
        l: ThreadId,
        t_id: TableId,
        key: &TValue,
        value: TValue,
    ) -> LuaResult<()> {
        let (slot_val, slot_loc) = self.luaH_get(t_id, key);
        self.luaH_finishset(l, t_id, key, &slot_val, slot_loc, value)
    }

    pub fn luaH_setint(
        &mut self,
        l: ThreadId,
        t_id: TableId,
        key: LuaInteger,
        value: TValue,
    ) -> LuaResult<()> {
        let (slot_val, slot_loc) = self.luaH_getint(t_id, key);
        if slot_val.is_abstkey() {
            let k = TValue::int_val(key);
            self.luaH_newkey(l, t_id, &k, value)
        } else {
            self.write_slot(slot_loc, value);
            Ok(())
        }
    }

    fn findindex(&mut self, l: ThreadId, t_id: TableId, key: &TValue, asize: u32) -> LuaResult<u32> {
        if key.is_nil() {
            return Ok(0);
        }
        let i = if key.is_integer() {
            arrayindex(key.ivalue())
        } else {
            0
        };
        if i.wrapping_sub(1) < asize {
            Ok(i)
        } else {
            let (val, loc) = self.getgeneric(t_id, key, true);
            if val.is_abstkey() {
                return self.luaG_runerror(l, "invalid key to 'next'");
            }
            match loc {
                SlotLoc::NodeVal(_, idx) => Ok((idx as u32) + 1 + asize),
                _ => self.luaG_runerror(l, "invalid key to 'next'"),
            }
        }
    }

    pub fn luaH_next(&mut self, l: ThreadId, t_id: TableId, key_stk: usize) -> LuaResult<bool> {
        let asize = self.tables.get(t_id).map_or(0, |t| t.realasize());
        let key_val = self.stack_get(l, key_stk);
        let mut i = self.findindex(l, t_id, &key_val, asize)?;
        while i < asize {
            let elem = self
                .tables
                .get(t_id)
                .and_then(|t| t.array.get(i as usize))
                .copied()
                .unwrap_or_else(TValue::empty);
            if !elem.is_empty() {
                self.stack_set(l, key_stk, TValue::int_val((i as LuaInteger) + 1));
                self.stack_set(l, key_stk + 1, elem);
                return Ok(true);
            }
            i += 1;
        }
        let sizenode = self.tables.get(t_id).map_or(0, |t| t.sizenode()) as u32;
        i -= asize;
        while i < sizenode {
            if let Some(nd) = self
                .tables
                .get(t_id)
                .and_then(|t| t.node.get(i as usize))
                .copied()
            {
                if !nd.i_val.is_empty() {
                    self.stack_set(l, key_stk, nd.get_key());
                    self.stack_set(l, key_stk + 1, nd.i_val);
                    return Ok(true);
                }
            }
            i += 1;
        }
        Ok(false)
    }

    fn hash_search(&mut self, t_id: TableId, mut j: LuaUnsigned) -> LuaUnsigned {
        let mut i: LuaUnsigned;
        if j == 0 {
            j += 1;
        }
        loop {
            i = j;
            if j <= (LuaInteger::MAX as LuaUnsigned) / 2 {
                j *= 2;
            } else {
                j = LuaInteger::MAX as LuaUnsigned;
                if self.luaH_getint(t_id, j as LuaInteger).0.is_empty() {
                    break;
                } else {
                    return j;
                }
            }
            if self.luaH_getint(t_id, j as LuaInteger).0.is_empty() {
                break;
            }
        }
        while j - i > 1 {
            let m = (i + j) / 2;
            if self.luaH_getint(t_id, m as LuaInteger).0.is_empty() {
                j = m;
            } else {
                i = m;
            }
        }
        i
    }

    pub fn luaH_getn(&mut self, t_id: TableId) -> LuaUnsigned {
        let limit = match self.tables.get(t_id) {
            Some(t) => t.alimit,
            None => return 0,
        };
        if limit > 0 {
            let lim_empty = self
                .tables
                .get(t_id)
                .and_then(|t| t.array.get((limit - 1) as usize))
                .map_or(true, |v| v.is_empty());
            if lim_empty {
                if limit >= 2 {
                    let prev_empty = self
                        .tables
                        .get(t_id)
                        .and_then(|t| t.array.get((limit - 2) as usize))
                        .map_or(true, |v| v.is_empty());
                    if !prev_empty {
                        if let Some(t) = self.tables.get_mut(t_id) {
                            if t.ispow2realasize() && !ispow2(limit - 1) {
                                t.alimit = limit - 1;
                                t.setnorealasize();
                            }
                        }
                        return (limit - 1) as LuaUnsigned;
                    }
                }
                let mut i: u32 = 0;
                let mut j: u32 = limit;
                while j - i > 1 {
                    let m = (i + j) / 2;
                    let m_empty = self
                        .tables
                        .get(t_id)
                        .and_then(|t| t.array.get((m - 1) as usize))
                        .map_or(true, |v| v.is_empty());
                    if m_empty {
                        j = m;
                    } else {
                        i = m;
                    }
                }
                let boundary = i;
                if let Some(t) = self.tables.get_mut(t_id) {
                    if t.ispow2realasize() && boundary > t.realasize() / 2 {
                        t.alimit = boundary;
                        t.setnorealasize();
                    }
                }
                return boundary as LuaUnsigned;
            }
        }

        let lim_eq_asize = self
            .tables
            .get(t_id)
            .map_or(true, |t| t.limitequalsasize());
        let mut cur_limit = limit;
        if !lim_eq_asize {
            let next_empty = self
                .tables
                .get(t_id)
                .and_then(|t| t.array.get(cur_limit as usize))
                .map_or(true, |v| v.is_empty());
            if next_empty {
                return cur_limit as LuaUnsigned;
            }
            cur_limit = self.tables.get(t_id).map_or(0, |t| t.realasize());
            let last_empty = self
                .tables
                .get(t_id)
                .and_then(|t| t.array.get((cur_limit - 1) as usize))
                .map_or(true, |v| v.is_empty());
            if last_empty {
                let mut i = self.tables.get(t_id).map_or(0, |t| t.alimit);
                let mut j = cur_limit;
                while j - i > 1 {
                    let m = (i + j) / 2;
                    let m_empty = self
                        .tables
                        .get(t_id)
                        .and_then(|t| t.array.get((m - 1) as usize))
                        .map_or(true, |v| v.is_empty());
                    if m_empty {
                        j = m;
                    } else {
                        i = m;
                    }
                }
                if let Some(t) = self.tables.get_mut(t_id) {
                    t.alimit = i;
                }
                return i as LuaUnsigned;
            }
        }

        let is_dummy = self.tables.get(t_id).map_or(true, |t| t.is_dummy);
        if is_dummy
            || self
                .luaH_getint(t_id, (cur_limit as LuaInteger) + 1)
                .0
                .is_empty()
        {
            cur_limit as LuaUnsigned
        } else {
            self.hash_search(t_id, cur_limit as LuaUnsigned)
        }
    }
}

// ============================================================================
// Stack & Thread helpers
// ============================================================================

impl LuaVM {
    pub fn stack_get(&self, l: ThreadId, idx: usize) -> TValue {
        self.threads
            .get(l)
            .and_then(|th| th.stack.get(idx))
            .map_or_else(TValue::nil, |sv| sv.val)
    }

    pub fn stack_set(&mut self, l: ThreadId, idx: usize, val: TValue) {
        if let Some(th) = self.threads.get_mut(l) {
            if idx >= th.stack.len() {
                th.stack.resize(idx + EXTRA_STACK + 1, StackValue::nil());
            }
            if let Some(sv) = th.stack.get_mut(idx) {
                sv.val = val;
            }
        }
    }

    pub fn get_top(&self, l: ThreadId) -> usize {
        self.threads.get(l).map_or(1, |th| th.top)
    }

    pub fn set_top_raw(&mut self, l: ThreadId, new_top: usize) {
        if let Some(th) = self.threads.get_mut(l) {
            if new_top >= th.stack.len() {
                th.stack.resize(new_top + EXTRA_STACK + 1, StackValue::nil());
            }
            th.top = new_top;
        }
    }

    pub fn get_upval(&self, uv_id: UpValId) -> TValue {
        match self.upvals.get(uv_id).map(|u| u.loc) {
            Some(UpValLoc::Open { thread, stack_idx }) => self.stack_get(thread, stack_idx),
            Some(UpValLoc::Closed(val)) => val,
            None => TValue::nil(),
        }
    }

    pub fn set_upval(&mut self, uv_id: UpValId, val: TValue) {
        match self.upvals.get(uv_id).map(|u| u.loc) {
            Some(UpValLoc::Open { thread, stack_idx }) => {
                self.stack_set(thread, stack_idx, val);
            }
            Some(UpValLoc::Closed(_)) => {
                if let Some(u) = self.upvals.get_mut(uv_id) {
                    u.loc = UpValLoc::Closed(val);
                }
            }
            None => {}
        }
    }

    pub fn luaD_reallocstack(
        &mut self,
        l: ThreadId,
        newsize: usize,
        raiseerror: bool,
    ) -> LuaResult<bool> {
        if let Some(th) = self.threads.get_mut(l) {
            let oldsize = th.stack_last;
            th.stack.resize(newsize + EXTRA_STACK, StackValue::nil());
            th.stack_last = newsize;
            for i in (oldsize + EXTRA_STACK)..(newsize + EXTRA_STACK) {
                if let Some(sv) = th.stack.get_mut(i) {
                    sv.val = TValue::nil();
                }
            }
            for ci in &mut th.ci_vec {
                if ci.is_lua() {
                    ci.trap = 1;
                }
            }
            let _ = raiseerror;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn luaD_growstack(
        &mut self,
        l: ThreadId,
        n: usize,
        raiseerror: bool,
    ) -> LuaResult<bool> {
        let (size, top) = match self.threads.get(l) {
            Some(th) => (th.stack_last, th.top),
            None => return Ok(false),
        };
        if size > (LUAI_MAXSTACK as usize) {
            if raiseerror {
                return Err(LUA_ERRERR);
            }
            return Ok(false);
        } else if n < (LUAI_MAXSTACK as usize) {
            let mut newsize = 2 * size;
            let needed = top + n;
            if newsize > (LUAI_MAXSTACK as usize) {
                newsize = LUAI_MAXSTACK as usize;
            }
            if newsize < needed {
                newsize = needed;
            }
            if newsize <= (LUAI_MAXSTACK as usize) {
                return self.luaD_reallocstack(l, newsize, raiseerror);
            }
        }
        self.luaD_reallocstack(l, ERRORSTACKSIZE, raiseerror)?;
        if raiseerror {
            self.luaG_runerror(l, "stack overflow")?;
        }
        Ok(false)
    }

    pub fn luaD_checkstack(&mut self, l: ThreadId, n: usize) -> LuaResult<()> {
        let need_grow = self
            .threads
            .get(l)
            .map_or(false, |th| th.stack_last.saturating_sub(th.top) <= n);
        if need_grow {
            self.luaD_growstack(l, n, true)?;
        }
        Ok(())
    }

    pub fn luaD_shrinkstack(&mut self, l: ThreadId) {
        let (inuse, stack_last) = match self.threads.get(l) {
            Some(th) => {
                let mut lim = th.top;
                for idx in 0..=th.ci_idx {
                    if let Some(ci) = th.ci_vec.get(idx) {
                        if lim < ci.top {
                            lim = ci.top;
                        }
                    }
                }
                let mut res = lim + 1;
                if res < LUA_MINSTACK {
                    res = LUA_MINSTACK;
                }
                (res, th.stack_last)
            }
            None => return,
        };
        let max_sz = if inuse > (LUAI_MAXSTACK as usize) / 3 {
            LUAI_MAXSTACK as usize
        } else {
            inuse * 3
        };
        if inuse <= (LUAI_MAXSTACK as usize) && stack_last > max_sz {
            let nsize = if inuse > (LUAI_MAXSTACK as usize) / 2 {
                LUAI_MAXSTACK as usize
            } else {
                inuse * 2
            };
            let _ = self.luaD_reallocstack(l, nsize, false);
        }
    }

    pub fn luaD_inctop(&mut self, l: ThreadId) -> LuaResult<()> {
        self.luaD_checkstack(l, 1)?;
        if let Some(th) = self.threads.get_mut(l) {
            th.top += 1;
        }
        Ok(())
    }

    pub fn luaE_checkcstack(&mut self, l: ThreadId) -> LuaResult<()> {
        let ccalls = self.threads.get(l).map_or(0, |th| th.get_ccalls());
        if ccalls == LUAI_MAXCCALLS {
            self.luaG_runerror(l, "C stack overflow")
        } else if ccalls >= (LUAI_MAXCCALLS / 10 * 11) {
            Err(LUA_ERRERR)
        } else {
            Ok(())
        }
    }

    pub fn luaE_incCstack(&mut self, l: ThreadId) -> LuaResult<()> {
        if let Some(th) = self.threads.get_mut(l) {
            th.nCcalls = th.nCcalls.wrapping_add(1);
        }
        let ccalls = self.threads.get(l).map_or(0, |th| th.get_ccalls());
        if ccalls >= LUAI_MAXCCALLS {
            self.luaE_checkcstack(l)?;
        }
        Ok(())
    }
}

// ============================================================================
// Closures, Upvalues & To-Be-Closed variables (lfunc.c)
// ============================================================================

const MAXDELTA: usize = 0xFFFF;

impl LuaVM {
    pub fn luaF_newLclosure(&mut self, nupvals: usize, p: ProtoId) -> LClosureId {
        let id = self.lclosures.len();
        self.lclosures.push(LClosure {
            nupvalues: nupvals as u8,
            p,
            upvals: vec![0; nupvals],
        });
        id
    }

    pub fn luaF_newCclosure(&mut self, f: LuaCFunction, upvals: Vec<TValue>) -> CClosureId {
        let id = self.cclosures.len();
        self.cclosures.push(CClosure {
            nupvalues: upvals.len() as u8,
            f,
            upvalue: upvals,
        });
        id
    }

    pub fn luaF_initupvals(&mut self, cl_id: LClosureId) {
        let nup = self.lclosures.get(cl_id).map_or(0, |c| c.nupvalues as usize);
        let mut uv_ids = Vec::with_capacity(nup);
        for _ in 0..nup {
            let uv_id = self.upvals.len();
            self.upvals.push(UpVal {
                loc: UpValLoc::Closed(TValue::nil()),
            });
            uv_ids.push(uv_id);
        }
        if let Some(cl) = self.lclosures.get_mut(cl_id) {
            cl.upvals = uv_ids;
        }
    }

    pub fn luaF_findupval(&mut self, l: ThreadId, level: usize) -> UpValId {
        let open_list = self
            .threads
            .get(l)
            .map_or_else(Vec::new, |th| th.openupval.clone());
        let mut insert_pos = open_list.len();
        for (idx, &uv_id) in open_list.iter().enumerate() {
            if let Some(UpValLoc::Open { stack_idx, .. }) = self.upvals.get(uv_id).map(|u| u.loc) {
                if stack_idx == level {
                    return uv_id;
                }
                if stack_idx < level {
                    insert_pos = idx;
                    break;
                }
            }
        }
        let uv_id = self.upvals.len();
        self.upvals.push(UpVal {
            loc: UpValLoc::Open {
                thread: l,
                stack_idx: level,
            },
        });
        if let Some(th) = self.threads.get_mut(l) {
            th.openupval.insert(insert_pos, uv_id);
        }
        uv_id
    }

    pub fn luaF_closeupval(&mut self, l: ThreadId, level: usize) {
        loop {
            let first_uv = self
                .threads
                .get(l)
                .and_then(|th| th.openupval.first().copied());
            match first_uv {
                Some(uv_id) => {
                    let uplevel = match self.upvals.get(uv_id).map(|u| u.loc) {
                        Some(UpValLoc::Open { stack_idx, .. }) => stack_idx,
                        _ => break,
                    };
                    if uplevel >= level {
                        let val = self.stack_get(l, uplevel);
                        if let Some(u) = self.upvals.get_mut(uv_id) {
                            u.loc = UpValLoc::Closed(val);
                        }
                        if let Some(th) = self.threads.get_mut(l) {
                            th.openupval.remove(0);
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    fn callclosemethod(
        &mut self,
        l: ThreadId,
        obj: TValue,
        err: TValue,
        yy: bool,
    ) -> LuaResult<()> {
        let top = self.get_top(l);
        let tm = self.luaT_gettmbyobj(&obj, TMS::TM_CLOSE);
        self.stack_set(l, top, tm);
        self.stack_set(l, top + 1, obj);
        self.stack_set(l, top + 2, err);
        self.set_top_raw(l, top + 3);
        if yy {
            self.luaD_call(l, top, 0)
        } else {
            self.luaD_callnoyield(l, top, 0)
        }
    }

    fn checkclosemth(&mut self, l: ThreadId, level: usize) -> LuaResult<()> {
        let v = self.stack_get(l, level);
        let tm = self.luaT_gettmbyobj(&v, TMS::TM_CLOSE);
        if tm.is_nil() {
            let ci_idx = self.threads.get(l).map_or(0, |th| th.ci_idx);
            let func = self.threads.get(l).map_or(0, |th| th.ci().func);
            let idx = (level as i32) - (func as i32);
            let vname = self
                .luaG_findlocal(l, ci_idx, idx)
                .map(|(n, _)| n)
                .unwrap_or_else(|| "?".to_string());
            return self.luaG_runerror(l, &format!("variable '{vname}' got a non-closable value"));
        }
        Ok(())
    }

    fn prepcallclosemth(
        &mut self,
        l: ThreadId,
        level: usize,
        status: i32,
        yy: bool,
    ) -> LuaResult<()> {
        let uv = self.stack_get(l, level);
        let errobj = if status == CLOSEKTOP {
            TValue::nil()
        } else {
            self.luaD_seterrorobj(l, status, level + 1);
            self.stack_get(l, level + 1)
        };
        self.callclosemethod(l, uv, errobj, yy)
    }

    pub fn luaF_newtbcupval(&mut self, l: ThreadId, level: usize) -> LuaResult<()> {
        let v = self.stack_get(l, level);
        if v.l_isfalse() {
            return Ok(());
        }
        self.checkclosemth(l, level)?;
        if let Some(th) = self.threads.get_mut(l) {
            while level.saturating_sub(th.tbclist) > MAXDELTA {
                th.tbclist += MAXDELTA;
                let tbc = th.tbclist;
                if let Some(sv) = th.stack.get_mut(tbc) {
                    sv.delta = 0;
                }
            }
            let diff = (level - th.tbclist) as u16;
            if let Some(sv) = th.stack.get_mut(level) {
                sv.delta = diff;
            }
            th.tbclist = level;
        }
        Ok(())
    }

    fn poptbclist(&mut self, l: ThreadId) {
        if let Some(th) = self.threads.get_mut(l) {
            let mut tbc = th.tbclist;
            let delta = th.stack.get(tbc).map_or(0, |sv| sv.delta) as usize;
            tbc = tbc.saturating_sub(delta);
            while tbc > 0 && th.stack.get(tbc).map_or(1, |sv| sv.delta) == 0 {
                tbc = tbc.saturating_sub(MAXDELTA);
            }
            th.tbclist = tbc;
        }
    }

    pub fn luaF_close(
        &mut self,
        l: ThreadId,
        level: usize,
        status: i32,
        yy: bool,
    ) -> LuaResult<usize> {
        self.luaF_closeupval(l, level);
        while self.threads.get(l).map_or(0, |th| th.tbclist) >= level {
            let tbc = self.threads.get(l).map_or(0, |th| th.tbclist);
            self.poptbclist(l);
            self.prepcallclosemth(l, tbc, status, yy)?;
        }
        Ok(level)
    }

    pub fn luaF_getlocalname(&self, p_id: ProtoId, mut local_number: i32, pc: i32) -> Option<StrId> {
        let p = self.protos.get(p_id)?;
        for lv in &p.locvars {
            if lv.startpc > pc {
                break;
            }
            if pc < lv.endpc {
                local_number -= 1;
                if local_number == 0 {
                    return lv.varname;
                }
            }
        }
        None
    }
}

// ============================================================================
// Tag Methods, Arithmetic & Varargs (ltm.c & lobject.c)
// ============================================================================

impl LuaVM {
    pub fn fasttm(&mut self, et: Option<TableId>, event: TMS) -> Option<TValue> {
        let t_id = et?;
        let flags = self.tables.get(t_id).map_or(0, |t| t.flags);
        if (flags & (1u8 << (event as u8))) != 0 {
            return None;
        }
        let ename = self.tmname.get(event as usize).copied().unwrap_or(0);
        let (tm, _) = self.luaH_getshortstr(t_id, ename);
        if tm.is_nil() {
            if let Some(t) = self.tables.get_mut(t_id) {
                t.flags |= 1u8 << (event as u8);
            }
            None
        } else {
            Some(tm)
        }
    }

    pub fn luaT_gettmbyobj(&self, o: &TValue, event: TMS) -> TValue {
        let mt = match o.ttype() {
            LUA_TTABLE => self.tables.get(o.hvalue()).and_then(|t| t.metatable),
            LUA_TUSERDATA => self.udatas.get(o.uvalue()).and_then(|u| u.metatable),
            t => {
                if t >= 0 && (t as usize) < LUA_NUMTYPES {
                    self.mt.get(t as usize).copied().flatten()
                } else {
                    None
                }
            }
        };
        match mt {
            Some(m_id) => {
                let ename = self.tmname.get(event as usize).copied().unwrap_or(0);
                self.luaH_getshortstr(m_id, ename).0
            }
            None => TValue::nil(),
        }
    }

    pub fn luaT_objtypename(&self, o: &TValue) -> String {
        let mt = if o.is_table() {
            self.tables.get(o.hvalue()).and_then(|t| t.metatable)
        } else if o.is_fulluserdata() {
            self.udatas.get(o.uvalue()).and_then(|u| u.metatable)
        } else {
            None
        };
        if let Some(m_id) = mt {
            let (name_val, _) = self.luaH_getshortstr(m_id, self.name_str);
            if name_val.is_string() {
                return self.get_str_lossy(name_val.tsvalue());
            }
        }
        ttypename(o.ttype()).to_string()
    }

    pub fn luaT_callTM(
        &mut self,
        l: ThreadId,
        f: &TValue,
        p1: &TValue,
        p2: &TValue,
        p3: &TValue,
    ) -> LuaResult<()> {
        let func = self.get_top(l);
        self.stack_set(l, func, *f);
        self.stack_set(l, func + 1, *p1);
        self.stack_set(l, func + 2, *p2);
        self.stack_set(l, func + 3, *p3);
        self.set_top_raw(l, func + 4);
        let is_lc = self.threads.get(l).map_or(false, |th| th.ci().is_luacode());
        if is_lc {
            self.luaD_call(l, func, 0)
        } else {
            self.luaD_callnoyield(l, func, 0)
        }
    }

    pub fn luaT_callTMres(
        &mut self,
        l: ThreadId,
        f: &TValue,
        p1: &TValue,
        p2: &TValue,
        res: usize,
    ) -> LuaResult<()> {
        let func = self.get_top(l);
        self.stack_set(l, func, *f);
        self.stack_set(l, func + 1, *p1);
        self.stack_set(l, func + 2, *p2);
        self.set_top_raw(l, func + 3);
        let is_lc = self.threads.get(l).map_or(false, |th| th.ci().is_luacode());
        if is_lc {
            self.luaD_call(l, func, 1)?;
        } else {
            self.luaD_callnoyield(l, func, 1)?;
        }
        let new_top = self.get_top(l) - 1;
        self.set_top_raw(l, new_top);
        let rval = self.stack_get(l, new_top);
        self.stack_set(l, res, rval);
        Ok(())
    }

    pub fn callbinTM(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        p2: &TValue,
        res: usize,
        event: TMS,
    ) -> LuaResult<bool> {
        let mut tm = self.luaT_gettmbyobj(p1, event);
        if tm.is_nil() {
            tm = self.luaT_gettmbyobj(p2, event);
        }
        if tm.is_nil() {
            return Ok(false);
        }
        self.luaT_callTMres(l, &tm, p1, p2, res)?;
        Ok(true)
    }

    pub fn luaT_trybinTM(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        o1: ValOrigin,
        p2: &TValue,
        o2: ValOrigin,
        res: usize,
        event: TMS,
    ) -> LuaResult<()> {
        if !self.callbinTM(l, p1, p2, res, event)? {
            match event {
                TMS::TM_BAND
                | TMS::TM_BOR
                | TMS::TM_BXOR
                | TMS::TM_SHL
                | TMS::TM_SHR
                | TMS::TM_BNOT => {
                    if self.tonumber(p1).is_some() && self.tonumber(p2).is_some() {
                        return self.luaG_tointerror(l, p1, o1, p2, o2);
                    } else {
                        return self.luaG_opinterror(
                            l,
                            p1,
                            o1,
                            p2,
                            o2,
                            "perform bitwise operation on",
                        );
                    }
                }
                _ => {
                    return self.luaG_opinterror(l, p1, o1, p2, o2, "perform arithmetic on");
                }
            }
        }
        Ok(())
    }

    pub fn luaT_tryconcatTM(&mut self, l: ThreadId) -> LuaResult<()> {
        let top = self.get_top(l);
        let p1 = self.stack_get(l, top - 2);
        let p2 = self.stack_get(l, top - 1);
        if !self.callbinTM(l, &p1, &p2, top - 2, TMS::TM_CONCAT)? {
            return self.luaG_concaterror(
                l,
                &p1,
                ValOrigin::Stack(top - 2),
                &p2,
                ValOrigin::Stack(top - 1),
            );
        }
        Ok(())
    }

    pub fn luaT_trybinassocTM(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        o1: ValOrigin,
        p2: &TValue,
        o2: ValOrigin,
        flip: bool,
        res: usize,
        event: TMS,
    ) -> LuaResult<()> {
        if flip {
            self.luaT_trybinTM(l, p2, o2, p1, o1, res, event)
        } else {
            self.luaT_trybinTM(l, p1, o1, p2, o2, res, event)
        }
    }

    pub fn luaT_trybiniTM(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        o1: ValOrigin,
        i2: LuaInteger,
        flip: bool,
        res: usize,
        event: TMS,
    ) -> LuaResult<()> {
        let aux = TValue::int_val(i2);
        self.luaT_trybinassocTM(l, p1, o1, &aux, ValOrigin::None, flip, res, event)
    }

    pub fn luaT_callorderTM(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        p2: &TValue,
        event: TMS,
    ) -> LuaResult<bool> {
        let top = self.get_top(l);
        if self.callbinTM(l, p1, p2, top, event)? {
            return Ok(!self.stack_get(l, top).l_isfalse());
        }
        self.luaG_ordererror(l, p1, p2)
    }

    pub fn luaT_callorderiTM(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        v2: i32,
        flip: bool,
        isfloat: bool,
        event: TMS,
    ) -> LuaResult<bool> {
        let aux = if isfloat {
            TValue::flt_val(v2 as LuaNumber)
        } else {
            TValue::int_val(v2 as LuaInteger)
        };
        if flip {
            self.luaT_callorderTM(l, &aux, p1, event)
        } else {
            self.luaT_callorderTM(l, p1, &aux, event)
        }
    }

    pub fn luaT_adjustvarargs(
        &mut self,
        l: ThreadId,
        nfixparams: i32,
        p_id: ProtoId,
    ) -> LuaResult<()> {
        let maxstack = self.protos.get(p_id).map_or(2, |p| p.maxstacksize as usize);
        let (func, top) = match self.threads.get(l) {
            Some(th) => (th.ci().func, th.top),
            None => return Ok(()),
        };
        let actual = (top as i32) - (func as i32) - 1;
        let nextra = actual - nfixparams;
        if let Some(th) = self.threads.get_mut(l) {
            th.ci_mut().nextraargs = nextra;
        }
        self.luaD_checkstack(l, maxstack + 1)?;
        let mut cur_top = self.get_top(l);
        let fval = self.stack_get(l, func);
        self.stack_set(l, cur_top, fval);
        cur_top += 1;
        for i in 1..=(nfixparams as usize) {
            let pval = self.stack_get(l, func + i);
            self.stack_set(l, cur_top, pval);
            cur_top += 1;
            self.stack_set(l, func + i, TValue::nil());
        }
        self.set_top_raw(l, cur_top);
        let shift = (actual + 1) as usize;
        if let Some(th) = self.threads.get_mut(l) {
            let ci = th.ci_mut();
            ci.func += shift;
            ci.top += shift;
        }
        Ok(())
    }

    pub fn luaT_getvarargs(
        &mut self,
        l: ThreadId,
        where_stk: usize,
        mut wanted: i32,
    ) -> LuaResult<()> {
        let (func, nextra) = match self.threads.get(l) {
            Some(th) => {
                let ci = th.ci();
                (ci.func, ci.nextraargs)
            }
            None => return Ok(()),
        };
        if wanted < 0 {
            wanted = nextra;
            self.luaD_checkstack(l, nextra.max(0) as usize)?;
            self.set_top_raw(l, where_stk + (nextra.max(0) as usize));
        }
        let mut i: i32 = 0;
        while i < wanted && i < nextra {
            let src = ((func as i32) - nextra + i) as usize;
            let v = self.stack_get(l, src);
            self.stack_set(l, where_stk + (i as usize), v);
            i += 1;
        }
        while i < wanted {
            self.stack_set(l, where_stk + (i as usize), TValue::nil());
            i += 1;
        }
        Ok(())
    }
}

// ============================================================================
// Arithmetic & Coercion (lvm.c & lobject.c)
// ============================================================================

pub fn luaV_shiftl(x: LuaInteger, y: LuaInteger) -> LuaInteger {
    const NBITS: LuaInteger = 64;
    if y < 0 {
        if y <= -NBITS {
            0
        } else {
            ((x as LuaUnsigned) >> ((-y) as u32)) as LuaInteger
        }
    } else if y >= NBITS {
        0
    } else {
        ((x as LuaUnsigned) << (y as u32)) as LuaInteger
    }
}

pub fn luaV_shiftr(x: LuaInteger, y: LuaInteger) -> LuaInteger {
    luaV_shiftl(x, 0i64.wrapping_sub(y))
}

pub fn luaV_modf(m: LuaNumber, n: LuaNumber) -> LuaNumber {
    let mut r = m % n;
    if if r > 0.0 {
        n < 0.0
    } else {
        r < 0.0 && n > 0.0
    } {
        r += n;
    }
    r
}

pub fn numarith(op: i32, v1: LuaNumber, v2: LuaNumber) -> LuaNumber {
    match op {
        LUA_OPADD => v1 + v2,
        LUA_OPSUB => v1 - v2,
        LUA_OPMUL => v1 * v2,
        LUA_OPDIV => v1 / v2,
        LUA_OPPOW => {
            if v2 == 2.0 {
                v1 * v1
            } else {
                v1.powf(v2)
            }
        }
        LUA_OPIDIV => (v1 / v2).floor(),
        LUA_OPUNM => -v1,
        LUA_OPMOD => luaV_modf(v1, v2),
        _ => 0.0,
    }
}

pub fn tonumberns(o: &TValue) -> Option<LuaNumber> {
    if o.is_float() {
        Some(o.fltvalue())
    } else if o.is_integer() {
        Some(o.ivalue() as LuaNumber)
    } else {
        None
    }
}

pub fn tointegerns(o: &TValue, mode: F2Imod) -> Option<LuaInteger> {
    if o.is_integer() {
        Some(o.ivalue())
    } else if o.is_float() {
        luaV_flttointeger(o.fltvalue(), mode)
    } else {
        None
    }
}

impl LuaVM {
    pub fn luaV_idiv(&mut self, l: ThreadId, m: LuaInteger, n: LuaInteger) -> LuaResult<LuaInteger> {
        if (n as LuaUnsigned).wrapping_add(1) <= 1 {
            if n == 0 {
                return self.luaG_runerror(l, "attempt to divide by zero");
            }
            return Ok(0i64.wrapping_sub(m));
        } else {
            let mut q = m / n;
            if (m ^ n) < 0 && m % n != 0 {
                q -= 1;
            }
            Ok(q)
        }
    }

    pub fn luaV_mod(&mut self, l: ThreadId, m: LuaInteger, n: LuaInteger) -> LuaResult<LuaInteger> {
        if (n as LuaUnsigned).wrapping_add(1) <= 1 {
            if n == 0 {
                return self.luaG_runerror(l, "attempt to perform 'n%0'");
            }
            return Ok(0);
        } else {
            let mut r = m % n;
            if r != 0 && (r ^ n) < 0 {
                r += n;
            }
            Ok(r)
        }
    }

    pub fn intarith(
        &mut self,
        l: ThreadId,
        op: i32,
        v1: LuaInteger,
        v2: LuaInteger,
    ) -> LuaResult<LuaInteger> {
        match op {
            LUA_OPADD => Ok(v1.wrapping_add(v2)),
            LUA_OPSUB => Ok(v1.wrapping_sub(v2)),
            LUA_OPMUL => Ok(v1.wrapping_mul(v2)),
            LUA_OPMOD => self.luaV_mod(l, v1, v2),
            LUA_OPIDIV => self.luaV_idiv(l, v1, v2),
            LUA_OPBAND => Ok(v1 & v2),
            LUA_OPBOR => Ok(v1 | v2),
            LUA_OPBXOR => Ok(v1 ^ v2),
            LUA_OPSHL => Ok(luaV_shiftl(v1, v2)),
            LUA_OPSHR => Ok(luaV_shiftr(v1, v2)),
            LUA_OPUNM => Ok(0i64.wrapping_sub(v1)),
            LUA_OPBNOT => Ok(!v1),
            _ => Ok(0),
        }
    }

    pub fn luaO_rawarith(
        &mut self,
        l: ThreadId,
        op: i32,
        p1: &TValue,
        p2: &TValue,
    ) -> LuaResult<Option<TValue>> {
        match op {
            LUA_OPBAND | LUA_OPBOR | LUA_OPBXOR | LUA_OPSHL | LUA_OPSHR | LUA_OPBNOT => {
                if let (Some(i1), Some(i2)) = (
                    tointegerns(p1, F2Imod::F2Ieq),
                    tointegerns(p2, F2Imod::F2Ieq),
                ) {
                    let r = self.intarith(l, op, i1, i2)?;
                    Ok(Some(TValue::int_val(r)))
                } else {
                    Ok(None)
                }
            }
            LUA_OPDIV | LUA_OPPOW => {
                if let (Some(n1), Some(n2)) = (tonumberns(p1), tonumberns(p2)) {
                    Ok(Some(TValue::flt_val(numarith(op, n1, n2))))
                } else {
                    Ok(None)
                }
            }
            _ => {
                if p1.is_integer() && p2.is_integer() {
                    let r = self.intarith(l, op, p1.ivalue(), p2.ivalue())?;
                    Ok(Some(TValue::int_val(r)))
                } else if let (Some(n1), Some(n2)) = (tonumberns(p1), tonumberns(p2)) {
                    Ok(Some(TValue::flt_val(numarith(op, n1, n2))))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub fn luaO_arith(
        &mut self,
        l: ThreadId,
        op: i32,
        p1: &TValue,
        p2: &TValue,
        res: usize,
    ) -> LuaResult<()> {
        if let Some(v) = self.luaO_rawarith(l, op, p1, p2)? {
            self.stack_set(l, res, v);
            Ok(())
        } else {
            let tm = TMS::from_u8(((op - LUA_OPADD) + (TMS::TM_ADD as i32)) as u8);
            self.luaT_trybinTM(l, p1, ValOrigin::None, p2, ValOrigin::None, res, tm)
        }
    }

    pub fn l_strton(&self, obj: &TValue) -> Option<TValue> {
        if !obj.is_string() {
            return None;
        }
        let bytes = self.get_str_bytes(obj.tsvalue());
        let (v, sz) = luaO_str2num(bytes)?;
        if sz == bytes.len() + 1 {
            Some(v)
        } else {
            None
        }
    }

    pub fn tonumber(&self, obj: &TValue) -> Option<LuaNumber> {
        if obj.is_float() {
            Some(obj.fltvalue())
        } else if obj.is_integer() {
            Some(obj.ivalue() as LuaNumber)
        } else {
            let v = self.l_strton(obj)?;
            Some(v.nvalue())
        }
    }

    pub fn tointeger(&self, obj: &TValue, mode: F2Imod) -> Option<LuaInteger> {
        if obj.is_integer() {
            Some(obj.ivalue())
        } else if let Some(v) = self.l_strton(obj) {
            tointegerns(&v, mode)
        } else {
            tointegerns(obj, mode)
        }
    }

    pub fn luaO_tostring(&mut self, obj: &mut TValue) {
        let s = tostringbuff(obj);
        *obj = self.luaS_newlstr_val(s.as_bytes());
    }
}

// ============================================================================
// Debug & Symbolic Execution (ldebug.c)
// ============================================================================

#[derive(Clone)]
pub struct LuaDebug {
    pub event: i32,
    pub name: Option<String>,
    pub namewhat: String,
    pub what: String,
    pub source: Vec<u8>,
    pub currentline: i32,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub nups: u8,
    pub nparams: u8,
    pub isvararg: bool,
    pub istailcall: bool,
    pub ftransfer: u16,
    pub ntransfer: u16,
    pub short_src: String,
    pub i_ci: usize,
}

impl LuaDebug {
    pub fn new() -> Self {
        LuaDebug {
            event: 0,
            name: None,
            namewhat: String::new(),
            what: String::new(),
            source: Vec::new(),
            currentline: -1,
            linedefined: -1,
            lastlinedefined: -1,
            nups: 0,
            nparams: 0,
            isvararg: false,
            istailcall: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: String::new(),
            i_ci: 0,
        }
    }
}

impl LuaVM {
    pub fn currentpc(&self, l: ThreadId, ci_idx: usize) -> i32 {
        let savedpc = self
            .threads
            .get(l)
            .and_then(|th| th.ci_vec.get(ci_idx))
            .map_or(0, |ci| ci.savedpc);
        (savedpc as i32) - 1
    }

    fn getbaseline(&self, p_id: ProtoId, pc: i32) -> (i32, i32) {
        let f = match self.protos.get(p_id) {
            Some(pr) => pr,
            None => return (-1, -1),
        };
        if f.abslineinfo.is_empty() || pc < f.abslineinfo.first().map_or(0, |a| a.pc) {
            (-1, f.linedefined)
        } else {
            let mut i = ((pc as u32) / (MAXIWTHABS as u32)) as isize - 1;
            while (i + 1) < (f.abslineinfo.len() as isize)
                && pc >= f.abslineinfo.get((i + 1) as usize).map_or(0, |a| a.pc)
            {
                i += 1;
            }
            let entry = f
                .abslineinfo
                .get(i.max(0) as usize)
                .copied()
                .unwrap_or(AbsLineInfo {
                    pc: -1,
                    line: f.linedefined,
                });
            (entry.pc, entry.line)
        }
    }

    pub fn luaG_getfuncline(&self, p_id: ProtoId, pc: i32) -> i32 {
        let has_lineinfo = self
            .protos
            .get(p_id)
            .map_or(false, |f| !f.lineinfo.is_empty());
        if !has_lineinfo {
            -1
        } else {
            let (mut basepc, mut baseline) = self.getbaseline(p_id, pc);
            while {
                basepc += 1;
                basepc <= pc
            } {
                let diff = self
                    .protos
                    .get(p_id)
                    .and_then(|f| f.lineinfo.get(basepc as usize))
                    .copied()
                    .unwrap_or(0);
                baseline += diff as i32;
            }
            baseline
        }
    }

    pub fn getcurrentline(&self, l: ThreadId, ci_idx: usize) -> i32 {
        let func = self
            .threads
            .get(l)
            .and_then(|th| th.ci_vec.get(ci_idx))
            .map_or(0, |ci| ci.func);
        let fval = self.stack_get(l, func);
        let p_id = self
            .lclosures
            .get(fval.clLvalue())
            .map_or(0, |cl| cl.p);
        let pc = self.currentpc(l, ci_idx);
        self.luaG_getfuncline(p_id, pc)
    }

    pub fn upvalname(&self, p_id: ProtoId, uv: usize) -> String {
        self.protos
            .get(p_id)
            .and_then(|p| p.upvalues.get(uv))
            .and_then(|u| u.name)
            .map_or_else(|| "?".to_string(), |s| self.get_str_lossy(s))
    }

    pub fn luaG_findlocal(
        &self,
        l: ThreadId,
        ci_idx: usize,
        n: i32,
    ) -> Option<(String, usize)> {
        let th = self.threads.get(l)?;
        let ci = th.ci_vec.get(ci_idx)?;
        let base = ci.func + 1;
        let mut name: Option<String> = None;
        if ci.is_lua() {
            let fval = self.stack_get(l, ci.func);
            let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
            if n < 0 {
                let is_va = self.protos.get(p_id).map_or(0, |p| p.is_vararg) != 0;
                if is_va {
                    let nextra = ci.nextraargs;
                    if n >= -nextra {
                        let pos = ((ci.func as i32) - nextra - (n + 1)) as usize;
                        return Some(("(vararg)".to_string(), pos));
                    }
                }
                return None;
            } else {
                let pc = self.currentpc(l, ci_idx);
                name = self
                    .luaF_getlocalname(p_id, n, pc)
                    .map(|s| self.get_str_lossy(s));
            }
        }
        if name.is_none() {
            let limit = if ci_idx == th.ci_idx {
                th.top
            } else {
                th.ci_vec.get(ci_idx + 1).map_or(th.top, |nci| nci.func)
            };
            if (limit as i32) - (base as i32) >= n && n > 0 {
                name = Some(if ci.is_lua() {
                    "(temporary)".to_string()
                } else {
                    "(C temporary)".to_string()
                });
            } else {
                return None;
            }
        }
        let pos = base + ((n - 1) as usize);
        name.map(|nm| (nm, pos))
    }

    fn filterpc(pc: i32, jmptarget: i32) -> i32 {
        if pc < jmptarget {
            -1
        } else {
            pc
        }
    }

    fn findsetreg(&self, p_id: ProtoId, mut lastpc: i32, reg: i32) -> i32 {
        let p = match self.protos.get(p_id) {
            Some(pr) => pr,
            None => return -1,
        };
        let mut setreg: i32 = -1;
        let mut jmptarget: i32 = 0;
        if lastpc >= 0 {
            if let Some(&inst) = p.code.get(lastpc as usize) {
                if testMMMode(GET_OPCODE(inst)) {
                    lastpc -= 1;
                }
            }
        }
        let mut pc: i32 = 0;
        while pc < lastpc {
            let i = p.code.get(pc as usize).copied().unwrap_or(0);
            let op = GET_OPCODE(i);
            let a = GETARG_A(i);
            let change = match op {
                OpCode::OP_LOADNIL => {
                    let b = GETARG_B(i);
                    a <= reg && reg <= a + b
                }
                OpCode::OP_TFORCALL => reg >= a + 2,
                OpCode::OP_CALL | OpCode::OP_TAILCALL => reg >= a,
                OpCode::OP_JMP => {
                    let b = GETARG_sJ(i);
                    let dest = pc + 1 + b;
                    if dest <= lastpc && dest > jmptarget {
                        jmptarget = dest;
                    }
                    false
                }
                _ => testAMode(op) && reg == a,
            };
            if change {
                setreg = Self::filterpc(pc, jmptarget);
            }
            pc += 1;
        }
        setreg
    }

    fn kname(&self, p_id: ProtoId, index: i32) -> (Option<&'static str>, String) {
        if let Some(kvalue) = self
            .protos
            .get(p_id)
            .and_then(|p| p.k.get(index as usize))
        {
            if kvalue.is_string() {
                return (Some("constant"), self.get_str_lossy(kvalue.tsvalue()));
            }
        }
        (None, "?".to_string())
    }

    fn basicgetobjname(
        &self,
        p_id: ProtoId,
        ppc: &mut i32,
        reg: i32,
    ) -> (Option<&'static str>, String) {
        let mut pc = *ppc;
        if let Some(s_id) = self.luaF_getlocalname(p_id, reg + 1, pc) {
            return (Some("local"), self.get_str_lossy(s_id));
        }
        pc = self.findsetreg(p_id, pc, reg);
        *ppc = pc;
        if pc != -1 {
            let i = self
                .protos
                .get(p_id)
                .and_then(|p| p.code.get(pc as usize))
                .copied()
                .unwrap_or(0);
            let op = GET_OPCODE(i);
            match op {
                OpCode::OP_MOVE => {
                    let b = GETARG_B(i);
                    if b < GETARG_A(i) {
                        return self.basicgetobjname(p_id, ppc, b);
                    }
                }
                OpCode::OP_GETUPVAL => {
                    return (
                        Some("upvalue"),
                        self.upvalname(p_id, GETARG_B(i) as usize),
                    );
                }
                OpCode::OP_LOADK => return self.kname(p_id, GETARG_Bx(i)),
                OpCode::OP_LOADKX => {
                    let next_i = self
                        .protos
                        .get(p_id)
                        .and_then(|p| p.code.get((pc + 1) as usize))
                        .copied()
                        .unwrap_or(0);
                    return self.kname(p_id, GETARG_Ax(next_i));
                }
                _ => {}
            }
        }
        (None, String::new())
    }

    fn rname(&self, p_id: ProtoId, mut pc: i32, c: i32) -> String {
        let (what, name) = self.basicgetobjname(p_id, &mut pc, c);
        if what == Some("constant") {
            name
        } else {
            "?".to_string()
        }
    }

    fn rkname(&self, p_id: ProtoId, pc: i32, i: Instruction) -> String {
        let c = GETARG_C(i);
        if GETARG_k(i) != 0 {
            self.kname(p_id, c).1
        } else {
            self.rname(p_id, pc, c)
        }
    }

    fn is_env(&self, p_id: ProtoId, mut pc: i32, i: Instruction, isup: bool) -> &'static str {
        let t = GETARG_B(i);
        let name = if isup {
            Some(self.upvalname(p_id, t as usize))
        } else {
            let (w, nm) = self.basicgetobjname(p_id, &mut pc, t);
            if w.is_some() {
                Some(nm)
            } else {
                None
            }
        };
        if name.as_deref() == Some("_ENV") {
            "global"
        } else {
            "field"
        }
    }

    pub fn getobjname(
        &self,
        p_id: ProtoId,
        mut lastpc: i32,
        reg: i32,
    ) -> (Option<&'static str>, String) {
        let (kind, name) = self.basicgetobjname(p_id, &mut lastpc, reg);
        if kind.is_some() {
            return (kind, name);
        } else if lastpc != -1 {
            let i = self
                .protos
                .get(p_id)
                .and_then(|p| p.code.get(lastpc as usize))
                .copied()
                .unwrap_or(0);
            let op = GET_OPCODE(i);
            match op {
                OpCode::OP_GETTABUP => {
                    let k = GETARG_C(i);
                    let (_, knm) = self.kname(p_id, k);
                    return (Some(self.is_env(p_id, lastpc, i, true)), knm);
                }
                OpCode::OP_GETTABLE => {
                    let k = GETARG_C(i);
                    let rnm = self.rname(p_id, lastpc, k);
                    return (Some(self.is_env(p_id, lastpc, i, false)), rnm);
                }
                OpCode::OP_GETI => {
                    return (Some("field"), "integer index".to_string());
                }
                OpCode::OP_GETFIELD => {
                    let k = GETARG_C(i);
                    let (_, knm) = self.kname(p_id, k);
                    return (Some(self.is_env(p_id, lastpc, i, false)), knm);
                }
                OpCode::OP_SELF => {
                    let rknm = self.rkname(p_id, lastpc, i);
                    return (Some("method"), rknm);
                }
                _ => {}
            }
        }
        (None, String::new())
    }

    fn funcnamefromcode(&self, p_id: ProtoId, pc: i32) -> (Option<&'static str>, String) {
        let i = match self.protos.get(p_id).and_then(|p| p.code.get(pc as usize)) {
            Some(&inst) => inst,
            None => return (None, String::new()),
        };
        let tm = match GET_OPCODE(i) {
            OpCode::OP_CALL | OpCode::OP_TAILCALL => {
                return self.getobjname(p_id, pc, GETARG_A(i));
            }
            OpCode::OP_TFORCALL => {
                return (Some("for iterator"), "for iterator".to_string());
            }
            OpCode::OP_SELF
            | OpCode::OP_GETTABUP
            | OpCode::OP_GETTABLE
            | OpCode::OP_GETI
            | OpCode::OP_GETFIELD => TMS::TM_INDEX,
            OpCode::OP_SETTABUP
            | OpCode::OP_SETTABLE
            | OpCode::OP_SETI
            | OpCode::OP_SETFIELD => TMS::TM_NEWINDEX,
            OpCode::OP_MMBIN | OpCode::OP_MMBINI | OpCode::OP_MMBINK => {
                TMS::from_u8(GETARG_C(i) as u8)
            }
            OpCode::OP_UNM => TMS::TM_UNM,
            OpCode::OP_BNOT => TMS::TM_BNOT,
            OpCode::OP_LEN => TMS::TM_LEN,
            OpCode::OP_CONCAT => TMS::TM_CONCAT,
            OpCode::OP_EQ => TMS::TM_EQ,
            OpCode::OP_LT | OpCode::OP_LTI | OpCode::OP_GTI => TMS::TM_LT,
            OpCode::OP_LE | OpCode::OP_LEI | OpCode::OP_GEI => TMS::TM_LE,
            OpCode::OP_CLOSE | OpCode::OP_RETURN => TMS::TM_CLOSE,
            _ => return (None, String::new()),
        };
        let tm_str_id = self.tmname.get(tm as usize).copied().unwrap_or(0);
        let full = self.get_str_lossy(tm_str_id);
        let sub = full.get(2..).unwrap_or(&full).to_string();
        (Some("metamethod"), sub)
    }

    pub fn funcnamefromcall(
        &self,
        l: ThreadId,
        ci_idx: usize,
    ) -> (Option<&'static str>, String) {
        let ci = match self.threads.get(l).and_then(|th| th.ci_vec.get(ci_idx)) {
            Some(c) => *c,
            None => return (None, String::new()),
        };
        if (ci.callstatus & CIST_HOOKED) != 0 {
            (Some("hook"), "?".to_string())
        } else if (ci.callstatus & CIST_FIN) != 0 {
            (Some("metamethod"), "__gc".to_string())
        } else if ci.is_lua() {
            let fval = self.stack_get(l, ci.func);
            let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
            let pc = self.currentpc(l, ci_idx);
            self.funcnamefromcode(p_id, pc)
        } else {
            (None, String::new())
        }
    }

    fn getupvalname(&self, l: ThreadId, ci_idx: usize, origin: ValOrigin) -> Option<(&'static str, String)> {
        let ci = self.threads.get(l)?.ci_vec.get(ci_idx)?;
        let fval = self.stack_get(l, ci.func);
        let cl = self.lclosures.get(fval.clLvalue())?;
        let p_id = cl.p;
        for (i, &uv_id) in cl.upvals.iter().enumerate() {
            let matches = match origin {
                ValOrigin::Upval(orig_uv) => {
                    if orig_uv == uv_id {
                        true
                    } else {
                        match (
                            self.upvals.get(orig_uv).map(|u| u.loc),
                            self.upvals.get(uv_id).map(|u| u.loc),
                        ) {
                            (
                                Some(UpValLoc::Open {
                                    thread: t1,
                                    stack_idx: s1,
                                }),
                                Some(UpValLoc::Open {
                                    thread: t2,
                                    stack_idx: s2,
                                }),
                            ) => t1 == t2 && s1 == s2,
                            _ => false,
                        }
                    }
                }
                ValOrigin::Stack(stk_idx) => match self.upvals.get(uv_id).map(|u| u.loc) {
                    Some(UpValLoc::Open { thread, stack_idx }) => {
                        thread == l && stack_idx == stk_idx
                    }
                    _ => false,
                },
                ValOrigin::None => false,
            };
            if matches {
                return Some(("upvalue", self.upvalname(p_id, i)));
            }
        }
        None
    }

    fn instack(&self, l: ThreadId, ci_idx: usize, origin: ValOrigin) -> i32 {
        let ci = match self.threads.get(l).and_then(|th| th.ci_vec.get(ci_idx)) {
            Some(c) => *c,
            None => return -1,
        };
        let stk_idx = match origin {
            ValOrigin::Stack(idx) => Some(idx),
            ValOrigin::Upval(uv_id) => match self.upvals.get(uv_id).map(|u| u.loc) {
                Some(UpValLoc::Open { thread, stack_idx }) if thread == l => Some(stack_idx),
                _ => None,
            },
            ValOrigin::None => None,
        };
        if let Some(pos_idx) = stk_idx {
            let base = ci.func + 1;
            if pos_idx >= base && pos_idx < ci.top {
                return (pos_idx - base) as i32;
            }
        }
        -1
    }

    pub fn varinfo(&self, l: ThreadId, origin: ValOrigin) -> String {
        let ci_idx = self.threads.get(l).map_or(0, |th| th.ci_idx);
        let is_lua = self
            .threads
            .get(l)
            .and_then(|th| th.ci_vec.get(ci_idx))
            .map_or(false, |ci| ci.is_lua());
        let mut kind_name: Option<(&'static str, String)> = None;
        if is_lua {
            kind_name = self.getupvalname(l, ci_idx, origin);
            if kind_name.is_none() {
                let reg = self.instack(l, ci_idx, origin);
                if reg >= 0 {
                    let func = self
                        .threads
                        .get(l)
                        .and_then(|th| th.ci_vec.get(ci_idx))
                        .map_or(0, |ci| ci.func);
                    let fval = self.stack_get(l, func);
                    let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
                    let pc = self.currentpc(l, ci_idx);
                    let (k, nm) = self.getobjname(p_id, pc, reg);
                    if let Some(kstr) = k {
                        kind_name = Some((kstr, nm));
                    }
                }
            }
        }
        match kind_name {
            Some((k, nm)) => format!(" ({k} '{nm}')"),
            None => String::new(),
        }
    }

    pub fn luaG_typeerror<T>(
        &mut self,
        l: ThreadId,
        o: &TValue,
        origin: ValOrigin,
        op: &str,
    ) -> LuaResult<T> {
        let t = self.luaT_objtypename(o);
        let extra = self.varinfo(l, origin);
        self.luaG_runerror(l, &format!("attempt to {op} a {t} value{extra}"))
    }

    pub fn luaG_callerror<T>(
        &mut self,
        l: ThreadId,
        o: &TValue,
        origin: ValOrigin,
    ) -> LuaResult<T> {
        let ci_idx = self.threads.get(l).map_or(0, |th| th.ci_idx);
        let (kind, name) = self.funcnamefromcall(l, ci_idx);
        let extra = match kind {
            Some(k) => format!(" ({k} '{name}')"),
            None => self.varinfo(l, origin),
        };
        let t = self.luaT_objtypename(o);
        self.luaG_runerror(l, &format!("attempt to call a {t} value{extra}"))
    }

    pub fn luaG_forerror<T>(&mut self, l: ThreadId, o: &TValue, what: &str) -> LuaResult<T> {
        let t = self.luaT_objtypename(o);
        self.luaG_runerror(l, &format!("bad 'for' {what} (number expected, got {t})"))
    }

    pub fn luaG_concaterror<T>(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        o1: ValOrigin,
        p2: &TValue,
        o2: ValOrigin,
    ) -> LuaResult<T> {
        if p1.is_string() || p1.is_number() {
            self.luaG_typeerror(l, p2, o2, "concatenate")
        } else {
            self.luaG_typeerror(l, p1, o1, "concatenate")
        }
    }

    pub fn luaG_opinterror<T>(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        o1: ValOrigin,
        p2: &TValue,
        o2: ValOrigin,
        msg: &str,
    ) -> LuaResult<T> {
        if !p1.is_number() {
            self.luaG_typeerror(l, p1, o1, msg)
        } else {
            self.luaG_typeerror(l, p2, o2, msg)
        }
    }

    pub fn luaG_tointerror<T>(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        o1: ValOrigin,
        _p2: &TValue,
        o2: ValOrigin,
    ) -> LuaResult<T> {
        let orig = if tointegerns(p1, F2Imod::F2Ieq).is_none() {
            o1
        } else {
            o2
        };
        let extra = self.varinfo(l, orig);
        self.luaG_runerror(l, &format!("number{extra} has no integer representation"))
    }

    pub fn luaG_ordererror<T>(
        &mut self,
        l: ThreadId,
        p1: &TValue,
        p2: &TValue,
    ) -> LuaResult<T> {
        let t1 = self.luaT_objtypename(p1);
        let t2 = self.luaT_objtypename(p2);
        if t1 == t2 {
            self.luaG_runerror(l, &format!("attempt to compare two {t1} values"))
        } else {
            self.luaG_runerror(l, &format!("attempt to compare {t1} with {t2}"))
        }
    }

    pub fn luaG_addinfo(&self, msg: &str, src: Option<StrId>, line: i32) -> String {
        let chunk = match src {
            Some(s_id) => luaO_chunkid(self.get_str_bytes(s_id)),
            None => "?".to_string(),
        };
        format!("{chunk}:{line}: {msg}")
    }

    pub fn luaG_errormsg<T>(&mut self, l: ThreadId) -> LuaResult<T> {
        let errfunc = self.threads.get(l).map_or(0, |th| th.errfunc);
        if errfunc != 0 {
            let ef_idx = errfunc as usize;
            let top = self.get_top(l);
            let arg = self.stack_get(l, top - 1);
            let ef_val = self.stack_get(l, ef_idx);
            self.stack_set(l, top, arg);
            self.stack_set(l, top - 1, ef_val);
            self.set_top_raw(l, top + 1);
            self.luaD_callnoyield(l, top - 1, 1)?;
        }
        Err(LUA_ERRRUN)
    }

    pub fn luaG_runerror<T>(&mut self, l: ThreadId, msg: &str) -> LuaResult<T> {
        let ci_idx = self.threads.get(l).map_or(0, |th| th.ci_idx);
        let is_lua = self
            .threads
            .get(l)
            .and_then(|th| th.ci_vec.get(ci_idx))
            .map_or(false, |ci| ci.is_lua());
        let final_msg = if is_lua {
            let func = self
                .threads
                .get(l)
                .and_then(|th| th.ci_vec.get(ci_idx))
                .map_or(0, |ci| ci.func);
            let fval = self.stack_get(l, func);
            let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
            let src = self.protos.get(p_id).and_then(|p| p.source);
            let line = self.getcurrentline(l, ci_idx);
            self.luaG_addinfo(msg, src, line)
        } else {
            msg.to_string()
        };
        let str_val = self.luaS_newlstr_val(final_msg.as_bytes());
        let top = self.get_top(l);
        self.stack_set(l, top, str_val);
        self.set_top_raw(l, top + 1);
        self.luaG_errormsg(l)
    }

    pub fn lua_getstack(&self, l: ThreadId, mut level: i32, ar: &mut LuaDebug) -> bool {
        if level < 0 {
            return false;
        }
        let th = match self.threads.get(l) {
            Some(t) => t,
            None => return false,
        };
        let mut ci_idx = th.ci_idx;
        while level > 0 && ci_idx > 0 {
            level -= 1;
            ci_idx -= 1;
        }
        if level == 0 && ci_idx > 0 {
            ar.i_ci = ci_idx;
            true
        } else {
            false
        }
    }

    fn funcinfo(&self, ar: &mut LuaDebug, fval: &TValue) {
        if !fval.is_Lclosure() {
            ar.source = b"=[C]".to_vec();
            ar.linedefined = -1;
            ar.lastlinedefined = -1;
            ar.what = "C".to_string();
        } else {
            let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
            if let Some(p) = self.protos.get(p_id) {
                if let Some(s_id) = p.source {
                    ar.source = self.get_str_bytes(s_id).to_vec();
                } else {
                    ar.source = b"=?".to_vec();
                }
                ar.linedefined = p.linedefined;
                ar.lastlinedefined = p.lastlinedefined;
                ar.what = if ar.linedefined == 0 {
                    "main".to_string()
                } else {
                    "Lua".to_string()
                };
            }
        }
        ar.short_src = luaO_chunkid(&ar.source);
    }

    fn getfuncname(&self, l: ThreadId, ci_idx: usize) -> (Option<&'static str>, String) {
        if ci_idx > 0 {
            let is_tail = self
                .threads
                .get(l)
                .and_then(|th| th.ci_vec.get(ci_idx))
                .map_or(false, |ci| (ci.callstatus & CIST_TAIL) != 0);
            if !is_tail {
                return self.funcnamefromcall(l, ci_idx - 1);
            }
        }
        (None, String::new())
    }

    pub fn lua_getinfo(&mut self, l: ThreadId, what: &str, ar: &mut LuaDebug) -> LuaResult<bool> {
        let mut chars = what.chars().peekable();
        let (ci_idx_opt, func_val) = if chars.peek() == Some(&'>') {
            chars.next();
            let top = self.get_top(l);
            let f = self.stack_get(l, top - 1);
            self.set_top_raw(l, top - 1);
            (None, f)
        } else {
            let ci_idx = ar.i_ci;
            let func = self
                .threads
                .get(l)
                .and_then(|th| th.ci_vec.get(ci_idx))
                .map_or(0, |ci| ci.func);
            (Some(ci_idx), self.stack_get(l, func))
        };

        let mut status = true;
        let mut push_f = false;
        for ch in chars {
            match ch {
                'S' => {
                    self.funcinfo(ar, &func_val);
                }
                'l' => {
                    ar.currentline = match ci_idx_opt {
                        Some(c_idx) => {
                            let is_lua = self
                                .threads
                                .get(l)
                                .and_then(|th| th.ci_vec.get(c_idx))
                                .map_or(false, |ci| ci.is_lua());
                            if is_lua {
                                self.getcurrentline(l, c_idx)
                            } else {
                                -1
                            }
                        }
                        None => -1,
                    };
                }
                'u' => {
                    if func_val.is_Lclosure() {
                        let (nups, is_va, nparams) = self
                            .lclosures
                            .get(func_val.clLvalue())
                            .and_then(|cl| {
                                let p = self.protos.get(cl.p)?;
                                Some((cl.nupvalues, p.is_vararg != 0, p.numparams))
                            })
                            .unwrap_or((0, true, 0));
                        ar.nups = nups;
                        ar.isvararg = is_va;
                        ar.nparams = nparams;
                    } else if func_val.is_Cclosure() {
                        ar.nups = self
                            .cclosures
                            .get(func_val.clCvalue())
                            .map_or(0, |c| c.nupvalues);
                        ar.isvararg = true;
                        ar.nparams = 0;
                    } else {
                        ar.nups = 0;
                        ar.isvararg = true;
                        ar.nparams = 0;
                    }
                }
                't' => {
                    ar.istailcall = match ci_idx_opt {
                        Some(c_idx) => self
                            .threads
                            .get(l)
                            .and_then(|th| th.ci_vec.get(c_idx))
                            .map_or(false, |ci| (ci.callstatus & CIST_TAIL) != 0),
                        None => false,
                    };
                }
                'n' => {
                    if let Some(c_idx) = ci_idx_opt {
                        let (nw, nm) = self.getfuncname(l, c_idx);
                        if let Some(nw_str) = nw {
                            ar.namewhat = nw_str.to_string();
                            ar.name = Some(nm);
                        } else {
                            ar.namewhat = String::new();
                            ar.name = None;
                        }
                    } else {
                        ar.namewhat = String::new();
                        ar.name = None;
                    }
                }
                'f' => {
                    push_f = true;
                }
                'r' | 'L' => {}
                _ => status = false,
            }
        }
        if push_f {
            let top = self.get_top(l);
            self.stack_set(l, top, func_val);
            self.set_top_raw(l, top + 1);
        }
        Ok(status)
    }
}

// ============================================================================
// Calls, Protected Calls, Coroutines (ldo.c)
// ============================================================================

const fn hastocloseCfunc(n: i16) -> bool {
    (n as i32) < LUA_MULTRET
}
const fn codeNresults(n: i32) -> i16 {
    (-n - 3) as i16
}
const fn decodeNresults(n: i16) -> i32 {
    -(n as i32) - 3
}

const fn errorstatus(s: i32) -> bool {
    s > LUA_YIELD
}

impl LuaVM {
    pub fn luaD_seterrorobj(&mut self, l: ThreadId, errcode: i32, oldtop: usize) {
        match errcode {
            LUA_ERRMEM => {
                let v = TValue::str_val(self.memerrmsg, true);
                self.stack_set(l, oldtop, v);
            }
            LUA_ERRERR => {
                let v = self.luaS_newlstr_val(b"error in error handling");
                self.stack_set(l, oldtop, v);
            }
            LUA_OK => {
                self.stack_set(l, oldtop, TValue::nil());
            }
            _ => {
                let top = self.get_top(l);
                let v = self.stack_get(l, top.saturating_sub(1));
                self.stack_set(l, oldtop, v);
            }
        }
        self.set_top_raw(l, oldtop + 1);
    }

    pub fn luaD_rawrunprotected<F>(&mut self, l: ThreadId, f: F) -> i32
    where
        F: FnOnce(&mut LuaVM, ThreadId) -> LuaResult<()>,
    {
        let old_nccalls = self.threads.get(l).map_or(0, |th| th.nCcalls);
        if let Some(th) = self.threads.get_mut(l) {
            th.error_jmp_depth += 1;
        }
        let res = f(self, l);
        if let Some(th) = self.threads.get_mut(l) {
            th.error_jmp_depth = th.error_jmp_depth.saturating_sub(1);
            th.nCcalls = old_nccalls;
        }
        match res {
            Ok(()) => LUA_OK,
            Err(status) => status,
        }
    }

    fn tryfuncTM(&mut self, l: ThreadId, func: usize) -> LuaResult<usize> {
        self.luaD_checkstack(l, 1)?;
        let fval = self.stack_get(l, func);
        let tm = self.luaT_gettmbyobj(&fval, TMS::TM_CALL);
        if tm.is_nil() {
            return self.luaG_callerror(l, &fval, ValOrigin::Stack(func));
        }
        let top = self.get_top(l);
        let mut p = top;
        while p > func {
            let prev = self.stack_get(l, p - 1);
            self.stack_set(l, p, prev);
            p -= 1;
        }
        self.set_top_raw(l, top + 1);
        self.stack_set(l, func, tm);
        Ok(func)
    }

    fn moveresults(
        &mut self,
        l: ThreadId,
        mut res: usize,
        mut nres: i32,
        wanted_in: i16,
    ) -> LuaResult<()> {
        let mut wanted = wanted_in as i32;
        match wanted {
            0 => {
                self.set_top_raw(l, res);
                return Ok(());
            }
            1 => {
                if nres == 0 {
                    self.stack_set(l, res, TValue::nil());
                } else {
                    let top = self.get_top(l);
                    let v = self.stack_get(l, top - (nres as usize));
                    self.stack_set(l, res, v);
                }
                self.set_top_raw(l, res + 1);
                return Ok(());
            }
            LUA_MULTRET => {
                wanted = nres;
            }
            _ => {
                if hastocloseCfunc(wanted_in) {
                    if let Some(th) = self.threads.get_mut(l) {
                        let ci = th.ci_mut();
                        ci.callstatus |= CIST_CLSRET;
                        ci.u2_val = nres;
                    }
                    res = self.luaF_close(l, res, CLOSEKTOP, true)?;
                    if let Some(th) = self.threads.get_mut(l) {
                        th.ci_mut().callstatus &= !CIST_CLSRET;
                    }
                    wanted = decodeNresults(wanted_in);
                    if wanted == LUA_MULTRET {
                        wanted = nres;
                    }
                }
            }
        }
        let top = self.get_top(l);
        let firstresult = top - (nres as usize);
        if nres > wanted {
            nres = wanted;
        }
        let mut i: i32 = 0;
        while i < nres {
            let v = self.stack_get(l, firstresult + (i as usize));
            self.stack_set(l, res + (i as usize), v);
            i += 1;
        }
        while i < wanted {
            self.stack_set(l, res + (i as usize), TValue::nil());
            i += 1;
        }
        self.set_top_raw(l, res + (wanted as usize));
        Ok(())
    }

    pub fn luaD_poscall(&mut self, l: ThreadId, ci_idx: usize, nres: i32) -> LuaResult<()> {
        let (func, wanted) = match self.threads.get(l).and_then(|th| th.ci_vec.get(ci_idx)) {
            Some(ci) => (ci.func, ci.nresults),
            None => return Ok(()),
        };
        self.moveresults(l, func, nres, wanted)?;
        if let Some(th) = self.threads.get_mut(l) {
            th.ci_idx = ci_idx.saturating_sub(1);
        }
        Ok(())
    }

    fn prep_call_info(
        &mut self,
        l: ThreadId,
        func: usize,
        nret: i32,
        mask: u16,
        top: usize,
    ) -> usize {
        if let Some(th) = self.threads.get_mut(l) {
            let next_idx = th.ci_idx + 1;
            if next_idx >= th.ci_vec.len() {
                th.ci_vec.push(CallInfo::new_base());
            }
            th.ci_idx = next_idx;
            if let Some(ci) = th.ci_vec.get_mut(next_idx) {
                ci.func = func;
                ci.nresults = nret as i16;
                ci.callstatus = mask;
                ci.top = top;
                ci.savedpc = 0;
                ci.trap = 0;
                ci.nextraargs = 0;
                ci.k = None;
                ci.old_errfunc = 0;
                ci.ctx = 0;
                ci.u2_val = 0;
            }
            next_idx
        } else {
            0
        }
    }

    fn precallC(
        &mut self,
        l: ThreadId,
        func: usize,
        nresults: i32,
        f: LuaCFunction,
    ) -> LuaResult<i32> {
        self.luaD_checkstack(l, LUA_MINSTACK)?;
        let top = self.get_top(l);
        let ci_idx = self.prep_call_info(l, func, nresults, CIST_C, top + LUA_MINSTACK);
        let n = f(self, l)?;
        self.luaD_poscall(l, ci_idx, n)?;
        Ok(n)
    }

    pub fn luaD_pretailcall(
        &mut self,
        l: ThreadId,
        ci_idx: usize,
        mut func: usize,
        mut narg1: i32,
        delta: i32,
    ) -> LuaResult<i32> {
        loop {
            let fval = self.stack_get(l, func);
            match fval.ttypetag() {
                LUA_VCCL => {
                    let f = match self.cclosures.get(fval.clCvalue()) {
                        Some(cc) => cc.f,
                        None => return Ok(0),
                    };
                    return self.precallC(l, func, LUA_MULTRET, f);
                }
                LUA_VLCF => {
                    let f = match fval.fvalue() {
                        Some(fn_ptr) => fn_ptr,
                        None => return Ok(0),
                    };
                    return self.precallC(l, func, LUA_MULTRET, f);
                }
                LUA_VLCL => {
                    let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
                    let (fsize, nfixparams) = self
                        .protos
                        .get(p_id)
                        .map_or((2, 0), |p| (p.maxstacksize as i32, p.numparams as i32));
                    let need = (fsize - delta).max(0) as usize;
                    self.luaD_checkstack(l, need)?;
                    let ci_func = self
                        .threads
                        .get(l)
                        .and_then(|th| th.ci_vec.get(ci_idx))
                        .map_or(0, |ci| ((ci.func as i32) - delta) as usize);
                    if let Some(th) = self.threads.get_mut(l) {
                        if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                            ci.func = ci_func;
                        }
                    }
                    for i in 0..(narg1 as usize) {
                        let v = self.stack_get(l, func + i);
                        self.stack_set(l, ci_func + i, v);
                    }
                    func = ci_func;
                    while narg1 <= nfixparams {
                        self.stack_set(l, func + (narg1 as usize), TValue::nil());
                        narg1 += 1;
                    }
                    if let Some(th) = self.threads.get_mut(l) {
                        if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                            ci.top = func + 1 + (fsize as usize);
                            ci.savedpc = 0;
                            ci.callstatus |= CIST_TAIL;
                        }
                    }
                    self.set_top_raw(l, func + (narg1 as usize));
                    return Ok(-1);
                }
                _ => {
                    func = self.tryfuncTM(l, func)?;
                    narg1 += 1;
                }
            }
        }
    }

    pub fn luaD_precall(
        &mut self,
        l: ThreadId,
        mut func: usize,
        nresults: i32,
    ) -> LuaResult<Option<usize>> {
        loop {
            let fval = self.stack_get(l, func);
            match fval.ttypetag() {
                LUA_VCCL => {
                    let f = match self.cclosures.get(fval.clCvalue()) {
                        Some(cc) => cc.f,
                        None => return Ok(None),
                    };
                    self.precallC(l, func, nresults, f)?;
                    return Ok(None);
                }
                LUA_VLCF => {
                    let f = match fval.fvalue() {
                        Some(fn_ptr) => fn_ptr,
                        None => return Ok(None),
                    };
                    self.precallC(l, func, nresults, f)?;
                    return Ok(None);
                }
                LUA_VLCL => {
                    let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
                    let (fsize, nfixparams) = self
                        .protos
                        .get(p_id)
                        .map_or((2, 0), |p| (p.maxstacksize as usize, p.numparams as i32));
                    let top = self.get_top(l);
                    let mut narg = (top as i32) - (func as i32) - 1;
                    self.luaD_checkstack(l, fsize)?;
                    let ci_idx = self.prep_call_info(l, func, nresults, 0, func + 1 + fsize);
                    let mut cur_top = self.get_top(l);
                    while narg < nfixparams {
                        self.stack_set(l, cur_top, TValue::nil());
                        cur_top += 1;
                        narg += 1;
                    }
                    self.set_top_raw(l, cur_top);
                    return Ok(Some(ci_idx));
                }
                _ => {
                    func = self.tryfuncTM(l, func)?;
                }
            }
        }
    }

    fn ccall(&mut self, l: ThreadId, func: usize, nresults: i32, inc: u32) -> LuaResult<()> {
        if let Some(th) = self.threads.get_mut(l) {
            th.nCcalls = th.nCcalls.wrapping_add(inc);
        }
        let ccalls = self.threads.get(l).map_or(0, |th| th.get_ccalls());
        if ccalls >= LUAI_MAXCCALLS {
            self.luaD_checkstack(l, 0)?;
            self.luaE_checkcstack(l)?;
        }
        if let Some(ci_idx) = self.luaD_precall(l, func, nresults)? {
            if let Some(th) = self.threads.get_mut(l) {
                if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                    ci.callstatus = CIST_FRESH;
                }
            }
            self.luaV_execute(l, ci_idx)?;
        }
        if let Some(th) = self.threads.get_mut(l) {
            th.nCcalls = th.nCcalls.wrapping_sub(inc);
        }
        Ok(())
    }

    pub fn luaD_call(&mut self, l: ThreadId, func: usize, nresults: i32) -> LuaResult<()> {
        self.ccall(l, func, nresults, 1)
    }

    pub fn luaD_callnoyield(&mut self, l: ThreadId, func: usize, nresults: i32) -> LuaResult<()> {
        self.ccall(l, func, nresults, NYCI)
    }

    fn finishpcallk(&mut self, l: ThreadId, ci_idx: usize) -> LuaResult<i32> {
        let (mut status, funcidx, callstatus, old_errfunc) =
            match self.threads.get(l).and_then(|th| th.ci_vec.get(ci_idx)) {
                Some(ci) => (
                    ci.get_cistrecst(),
                    ci.u2_val as usize,
                    ci.callstatus,
                    ci.old_errfunc,
                ),
                None => return Ok(LUA_OK),
            };
        if status == LUA_OK {
            status = LUA_YIELD;
        } else {
            let mut func = funcidx;
            if let Some(th) = self.threads.get_mut(l) {
                th.allowhook = (callstatus & CIST_OAH) as u8;
            }
            func = self.luaF_close(l, func, status, true)?;
            self.luaD_seterrorobj(l, status, func);
            self.luaD_shrinkstack(l);
            if let Some(th) = self.threads.get_mut(l) {
                if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                    ci.set_cistrecst(LUA_OK);
                }
            }
        }
        if let Some(th) = self.threads.get_mut(l) {
            if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                ci.callstatus &= !CIST_YPCALL;
            }
            th.errfunc = old_errfunc;
        }
        Ok(status)
    }

    fn finishCcall(&mut self, l: ThreadId, ci_idx: usize) -> LuaResult<()> {
        let ci = match self.threads.get(l).and_then(|th| th.ci_vec.get(ci_idx)) {
            Some(c) => *c,
            None => return Ok(()),
        };
        let n = if (ci.callstatus & CIST_CLSRET) != 0 {
            ci.u2_val
        } else {
            let mut status = LUA_YIELD;
            if (ci.callstatus & CIST_YPCALL) != 0 {
                status = self.finishpcallk(l, ci_idx)?;
            }
            self.adjustresults(l, LUA_MULTRET);
            match ci.k {
                Some(k_fn) => k_fn(self, l, status, ci.ctx)?,
                None => 0,
            }
        };
        self.luaD_poscall(l, ci_idx, n)
    }

    pub fn unroll(&mut self, l: ThreadId) -> LuaResult<()> {
        loop {
            let ci_idx = self.threads.get(l).map_or(0, |th| th.ci_idx);
            if ci_idx == 0 {
                break;
            }
            let is_lua = self
                .threads
                .get(l)
                .and_then(|th| th.ci_vec.get(ci_idx))
                .map_or(false, |ci| ci.is_lua());
            if !is_lua {
                self.finishCcall(l, ci_idx)?;
            } else {
                self.luaV_finishOp(l)?;
                self.luaV_execute(l, ci_idx)?;
            }
        }
        Ok(())
    }

    fn findpcall(&self, l: ThreadId) -> Option<usize> {
        let th = self.threads.get(l)?;
        let mut idx = th.ci_idx;
        while idx > 0 {
            if let Some(ci) = th.ci_vec.get(idx) {
                if (ci.callstatus & CIST_YPCALL) != 0 {
                    return Some(idx);
                }
            }
            idx -= 1;
        }
        None
    }

    fn precover(&mut self, l: ThreadId, mut status: i32) -> i32 {
        while errorstatus(status) {
            if let Some(ci_idx) = self.findpcall(l) {
                if let Some(th) = self.threads.get_mut(l) {
                    th.ci_idx = ci_idx;
                    if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                        ci.set_cistrecst(status);
                    }
                }
                status = self.luaD_rawrunprotected(l, |vm, th_id| vm.unroll(th_id));
            } else {
                break;
            }
        }
        status
    }

    fn resume_error(&mut self, l: ThreadId, msg: &str, narg: i32) -> (i32, i32) {
        let top = self.get_top(l).saturating_sub(narg as usize);
        let s = self.luaS_newlstr_val(msg.as_bytes());
        self.stack_set(l, top, s);
        self.set_top_raw(l, top + 1);
        (LUA_ERRRUN, 0)
    }

    fn resume_inner(&mut self, l: ThreadId, mut n: i32) -> LuaResult<()> {
        let top = self.get_top(l);
        let first_arg = top - (n as usize);
        let (status, ci_idx, ci) = match self.threads.get(l) {
            Some(th) => (th.status, th.ci_idx, th.ci()),
            None => return Ok(()),
        };
        if status == LUA_OK {
            self.ccall(l, first_arg - 1, LUA_MULTRET, 0)
        } else {
            if let Some(th) = self.threads.get_mut(l) {
                th.status = LUA_OK;
            }
            if ci.is_lua() {
                if let Some(th) = self.threads.get_mut(l) {
                    if let Some(c) = th.ci_vec.get_mut(ci_idx) {
                        c.savedpc = c.savedpc.saturating_sub(1);
                    }
                }
                self.set_top_raw(l, first_arg);
                self.luaV_execute(l, ci_idx)?;
            } else {
                if let Some(k_fn) = ci.k {
                    n = k_fn(self, l, LUA_YIELD, ci.ctx)?;
                }
                self.luaD_poscall(l, ci_idx, n)?;
            }
            self.unroll(l)
        }
    }

    pub fn lua_resume(
        &mut self,
        l: ThreadId,
        from: Option<ThreadId>,
        nargs: i32,
    ) -> (i32, i32) {
        let (th_status, ci_idx, top, func) = match self.threads.get(l) {
            Some(th) => (th.status, th.ci_idx, th.top, th.ci().func),
            None => return (LUA_ERRRUN, 0),
        };
        if th_status == LUA_OK {
            if ci_idx != 0 {
                return self.resume_error(l, "cannot resume non-suspended coroutine", nargs);
            } else if (top as i32) - ((func + 1) as i32) == nargs {
                return self.resume_error(l, "cannot resume dead coroutine", nargs);
            }
        } else if th_status != LUA_YIELD {
            return self.resume_error(l, "cannot resume dead coroutine", nargs);
        }

        let from_ccalls = from
            .and_then(|f| self.threads.get(f))
            .map_or(0, |th| th.get_ccalls());
        if from_ccalls >= LUAI_MAXCCALLS {
            return self.resume_error(l, "C stack overflow", nargs);
        }
        if let Some(th) = self.threads.get_mut(l) {
            th.nCcalls = from_ccalls + 1;
        }
        let mut status = self.luaD_rawrunprotected(l, |vm, th_id| vm.resume_inner(th_id, nargs));
        status = self.precover(l, status);
        if errorstatus(status) {
            if let Some(th) = self.threads.get_mut(l) {
                th.status = status;
            }
            let cur_top = self.get_top(l);
            self.luaD_seterrorobj(l, status, cur_top);
            let new_top = self.get_top(l);
            if let Some(th) = self.threads.get_mut(l) {
                th.ci_mut().top = new_top;
            }
        }
        let nresults = if status == LUA_YIELD {
            self.threads.get(l).map_or(0, |th| th.ci().u2_val)
        } else {
            let (t, f) = self
                .threads
                .get(l)
                .map_or((1, 0), |th| (th.top, th.ci().func));
            (t as i32) - ((f + 1) as i32)
        };
        (status, nresults)
    }

    pub fn lua_yieldk(
        &mut self,
        l: ThreadId,
        nresults: i32,
        ctx: LuaKContext,
        k: Option<LuaKFunction>,
    ) -> LuaResult<i32> {
        let is_yieldable = self.threads.get(l).map_or(false, |th| th.yieldable());
        if !is_yieldable {
            if l != self.mainthread {
                return self.luaG_runerror(l, "attempt to yield across a C-call boundary");
            } else {
                return self.luaG_runerror(l, "attempt to yield from outside a coroutine");
            }
        }
        if let Some(th) = self.threads.get_mut(l) {
            th.status = LUA_YIELD;
            let ci = th.ci_mut();
            ci.u2_val = nresults;
            if !ci.is_lua() {
                ci.k = k;
                if k.is_some() {
                    ci.ctx = ctx;
                }
            }
        }
        Err(LUA_YIELD)
    }

    pub fn luaD_closeprotected(
        &mut self,
        l: ThreadId,
        level: usize,
        mut status: i32,
    ) -> i32 {
        let (old_ci, old_allowhooks) = self
            .threads
            .get(l)
            .map_or((0, 1), |th| (th.ci_idx, th.allowhook));
        loop {
            let pcl_status = status;
            let st = self.luaD_rawrunprotected(l, |vm, th_id| {
                vm.luaF_close(th_id, level, pcl_status, false)?;
                Ok(())
            });
            if st == LUA_OK {
                return pcl_status;
            } else {
                status = st;
                if let Some(th) = self.threads.get_mut(l) {
                    th.ci_idx = old_ci;
                    th.allowhook = old_allowhooks;
                }
            }
        }
    }

    pub fn luaD_pcall<F>(
        &mut self,
        l: ThreadId,
        func: F,
        old_top: usize,
        ef: isize,
    ) -> i32
    where
        F: FnOnce(&mut LuaVM, ThreadId) -> LuaResult<()>,
    {
        let (old_ci, old_allowhooks, old_errfunc) = self
            .threads
            .get(l)
            .map_or((0, 1, 0), |th| (th.ci_idx, th.allowhook, th.errfunc));
        if let Some(th) = self.threads.get_mut(l) {
            th.errfunc = ef;
        }
        let mut status = self.luaD_rawrunprotected(l, func);
        if status != LUA_OK {
            if let Some(th) = self.threads.get_mut(l) {
                th.ci_idx = old_ci;
                th.allowhook = old_allowhooks;
            }
            status = self.luaD_closeprotected(l, old_top, status);
            self.luaD_seterrorobj(l, status, old_top);
            self.luaD_shrinkstack(l);
        }
        if let Some(th) = self.threads.get_mut(l) {
            th.errfunc = old_errfunc;
        }
        status
    }

    pub fn luaE_resetthread(&mut self, l: ThreadId, mut status: i32) -> i32 {
        if let Some(th) = self.threads.get_mut(l) {
            th.ci_idx = 0;
            if let Some(sv) = th.stack.get_mut(0) {
                sv.val = TValue::nil();
            }
            if let Some(ci) = th.ci_vec.get_mut(0) {
                ci.func = 0;
                ci.callstatus = CIST_C;
            }
            if status == LUA_YIELD {
                status = LUA_OK;
            }
            th.status = LUA_OK;
        }
        status = self.luaD_closeprotected(l, 1, status);
        if status != LUA_OK {
            self.luaD_seterrorobj(l, status, 1);
        } else {
            self.set_top_raw(l, 1);
        }
        let top = self.get_top(l);
        if let Some(th) = self.threads.get_mut(l) {
            if let Some(ci) = th.ci_vec.get_mut(0) {
                ci.top = top + LUA_MINSTACK;
            }
        }
        let _ = self.luaD_reallocstack(l, top + LUA_MINSTACK, false);
        status
    }

    pub fn lua_closethread(&mut self, l: ThreadId, from: Option<ThreadId>) -> i32 {
        let from_ccalls = from
            .and_then(|f| self.threads.get(f))
            .map_or(0, |th| th.get_ccalls());
        let st = self.threads.get(l).map_or(LUA_OK, |th| th.status);
        if let Some(th) = self.threads.get_mut(l) {
            th.nCcalls = from_ccalls;
        }
        self.luaE_resetthread(l, st)
    }
}

// ============================================================================
// Virtual Machine Operations & Main Loop (lvm.c)
// ============================================================================

const MAXINTFITSF: LuaUnsigned = 1u64 << 53;

fn l_intfitsf(i: LuaInteger) -> bool {
    MAXINTFITSF.wrapping_add(i as LuaUnsigned) <= (2 * MAXINTFITSF)
}

fn LTintfloat(i: LuaInteger, f: LuaNumber) -> bool {
    if l_intfitsf(i) {
        (i as LuaNumber) < f
    } else if let Some(fi) = luaV_flttointeger(f, F2Imod::F2Iceil) {
        i < fi
    } else {
        f > 0.0
    }
}

fn LEintfloat(i: LuaInteger, f: LuaNumber) -> bool {
    if l_intfitsf(i) {
        (i as LuaNumber) <= f
    } else if let Some(fi) = luaV_flttointeger(f, F2Imod::F2Ifloor) {
        i <= fi
    } else {
        f > 0.0
    }
}

fn LTfloatint(f: LuaNumber, i: LuaInteger) -> bool {
    if l_intfitsf(i) {
        f < (i as LuaNumber)
    } else if let Some(fi) = luaV_flttointeger(f, F2Imod::F2Ifloor) {
        fi < i
    } else {
        f < 0.0
    }
}

fn LEfloatint(f: LuaNumber, i: LuaInteger) -> bool {
    if l_intfitsf(i) {
        f <= (i as LuaNumber)
    } else if let Some(fi) = luaV_flttointeger(f, F2Imod::F2Iceil) {
        fi <= i
    } else {
        f < 0.0
    }
}

pub fn LTnum(l: &TValue, r: &TValue) -> bool {
    if l.is_integer() {
        let li = l.ivalue();
        if r.is_integer() {
            li < r.ivalue()
        } else {
            LTintfloat(li, r.fltvalue())
        }
    } else {
        let lf = l.fltvalue();
        if r.is_float() {
            lf < r.fltvalue()
        } else {
            LTfloatint(lf, r.ivalue())
        }
    }
}

pub fn LEnum(l: &TValue, r: &TValue) -> bool {
    if l.is_integer() {
        let li = l.ivalue();
        if r.is_integer() {
            li <= r.ivalue()
        } else {
            LEintfloat(li, r.fltvalue())
        }
    } else {
        let lf = l.fltvalue();
        if r.is_float() {
            lf <= r.fltvalue()
        } else {
            LEfloatint(lf, r.ivalue())
        }
    }
}

impl LuaVM {
    fn forlimit(
        &mut self,
        l: ThreadId,
        init: LuaInteger,
        lim: &TValue,
        step: LuaInteger,
    ) -> LuaResult<(bool, LuaInteger)> {
        let mode = if step < 0 {
            F2Imod::F2Iceil
        } else {
            F2Imod::F2Ifloor
        };
        let p = match self.tointeger(lim, mode) {
            Some(v) => v,
            None => {
                let flim = match self.tonumber(lim) {
                    Some(f) => f,
                    None => return self.luaG_forerror(l, lim, "limit"),
                };
                if 0.0 < flim {
                    if step < 0 {
                        return Ok((true, 0));
                    }
                    LuaInteger::MAX
                } else {
                    if step > 0 {
                        return Ok((true, 0));
                    }
                    LuaInteger::MIN
                }
            }
        };
        let skip = if step > 0 { init > p } else { init < p };
        Ok((skip, p))
    }

    fn forprep(&mut self, l: ThreadId, ra: usize) -> LuaResult<bool> {
        let pinit = self.stack_get(l, ra);
        let plimit = self.stack_get(l, ra + 1);
        let pstep = self.stack_get(l, ra + 2);
        if pinit.is_integer() && pstep.is_integer() {
            let init = pinit.ivalue();
            let step = pstep.ivalue();
            if step == 0 {
                return self.luaG_runerror(l, "'for' step is zero");
            }
            self.stack_set(l, ra + 3, TValue::int_val(init));
            let (skip, limit) = self.forlimit(l, init, &plimit, step)?;
            if skip {
                return Ok(true);
            } else {
                let count: LuaUnsigned = if step > 0 {
                    let mut c = (limit as LuaUnsigned).wrapping_sub(init as LuaUnsigned);
                    if step != 1 {
                        c /= step as LuaUnsigned;
                    }
                    c
                } else {
                    let c = (init as LuaUnsigned).wrapping_sub(limit as LuaUnsigned);
                    let d = ((-(step + 1)) as LuaUnsigned).wrapping_add(1);
                    c / d
                };
                self.stack_set(l, ra + 1, TValue::int_val(count as LuaInteger));
            }
        } else {
            let limit = match self.tonumber(&plimit) {
                Some(n) => n,
                None => return self.luaG_forerror(l, &plimit, "limit"),
            };
            let step = match self.tonumber(&pstep) {
                Some(n) => n,
                None => return self.luaG_forerror(l, &pstep, "step"),
            };
            let init = match self.tonumber(&pinit) {
                Some(n) => n,
                None => return self.luaG_forerror(l, &pinit, "initial value"),
            };
            if step == 0.0 {
                return self.luaG_runerror(l, "'for' step is zero");
            }
            if if 0.0 < step {
                limit < init
            } else {
                init < limit
            } {
                return Ok(true);
            } else {
                self.stack_set(l, ra + 1, TValue::flt_val(limit));
                self.stack_set(l, ra + 2, TValue::flt_val(step));
                self.stack_set(l, ra, TValue::flt_val(init));
                self.stack_set(l, ra + 3, TValue::flt_val(init));
            }
        }
        Ok(false)
    }

    fn floatforloop(&mut self, l: ThreadId, ra: usize) -> bool {
        let step = self.stack_get(l, ra + 2).fltvalue();
        let limit = self.stack_get(l, ra + 1).fltvalue();
        let mut idx = self.stack_get(l, ra).fltvalue();
        idx += step;
        if if 0.0 < step {
            idx <= limit
        } else {
            limit <= idx
        } {
            self.stack_set(l, ra, TValue::flt_val(idx));
            self.stack_set(l, ra + 3, TValue::flt_val(idx));
            true
        } else {
            false
        }
    }

    pub fn luaV_finishget(
        &mut self,
        l: ThreadId,
        mut t: TValue,
        mut t_origin: ValOrigin,
        key: TValue,
        val: usize,
        mut has_slot: bool,
    ) -> LuaResult<()> {
        for _ in 0..MAXTAGLOOP {
            let tm = if !has_slot {
                let m = self.luaT_gettmbyobj(&t, TMS::TM_INDEX);
                if m.is_nil() {
                    return self.luaG_typeerror(l, &t, t_origin, "index");
                }
                m
            } else {
                let mt = self.tables.get(t.hvalue()).and_then(|tb| tb.metatable);
                match self.fasttm(mt, TMS::TM_INDEX) {
                    Some(m) => m,
                    None => {
                        self.stack_set(l, val, TValue::nil());
                        return Ok(());
                    }
                }
            };
            if tm.is_function() {
                return self.luaT_callTMres(l, &tm, &t, &key, val);
            }
            t = tm;
            t_origin = ValOrigin::None;
            if t.is_table() {
                let (sval, _) = self.luaH_get(t.hvalue(), &key);
                if !sval.is_empty() {
                    self.stack_set(l, val, sval);
                    return Ok(());
                }
                has_slot = true;
            } else {
                has_slot = false;
            }
        }
        self.luaG_runerror(l, "'__index' chain too long; possible loop")
    }

    pub fn luaV_finishset(
        &mut self,
        l: ThreadId,
        mut t: TValue,
        mut t_origin: ValOrigin,
        key: TValue,
        val: TValue,
        mut slot_opt: Option<(TValue, SlotLoc)>,
    ) -> LuaResult<()> {
        for _ in 0..MAXTAGLOOP {
            let tm = if let Some((slot_val, slot_loc)) = slot_opt {
                let h_id = t.hvalue();
                let mt = self.tables.get(h_id).and_then(|tb| tb.metatable);
                match self.fasttm(mt, TMS::TM_NEWINDEX) {
                    Some(m) => m,
                    None => {
                        self.luaH_finishset(l, h_id, &key, &slot_val, slot_loc, val)?;
                        if let Some(tb) = self.tables.get_mut(h_id) {
                            tb.invalidate_tm_cache();
                        }
                        return Ok(());
                    }
                }
            } else {
                let m = self.luaT_gettmbyobj(&t, TMS::TM_NEWINDEX);
                if m.is_nil() {
                    return self.luaG_typeerror(l, &t, t_origin, "index");
                }
                m
            };
            if tm.is_function() {
                return self.luaT_callTM(l, &tm, &t, &key, &val);
            }
            t = tm;
            t_origin = ValOrigin::None;
            if t.is_table() {
                let (sval, sloc) = self.luaH_get(t.hvalue(), &key);
                if !sval.is_empty() {
                    self.write_slot(sloc, val);
                    return Ok(());
                }
                slot_opt = Some((sval, sloc));
            } else {
                slot_opt = None;
            }
        }
        self.luaG_runerror(l, "'__newindex' chain too long; possible loop")
    }

    pub fn l_strcmp(&self, ts1: StrId, ts2: StrId) -> std::cmp::Ordering {
        let s1 = self.get_str_bytes(ts1);
        let s2 = self.get_str_bytes(ts2);
        s1.cmp(s2)
    }

    pub fn lessthanothers(&mut self, l: ThreadId, left: &TValue, right: &TValue) -> LuaResult<bool> {
        if left.is_string() && right.is_string() {
            Ok(self.l_strcmp(left.tsvalue(), right.tsvalue()) == std::cmp::Ordering::Less)
        } else {
            self.luaT_callorderTM(l, left, right, TMS::TM_LT)
        }
    }

    pub fn luaV_lessthan(&mut self, l: ThreadId, left: &TValue, right: &TValue) -> LuaResult<bool> {
        if left.is_number() && right.is_number() {
            Ok(LTnum(left, right))
        } else {
            self.lessthanothers(l, left, right)
        }
    }

    pub fn lessequalothers(&mut self, l: ThreadId, left: &TValue, right: &TValue) -> LuaResult<bool> {
        if left.is_string() && right.is_string() {
            Ok(self.l_strcmp(left.tsvalue(), right.tsvalue()) != std::cmp::Ordering::Greater)
        } else {
            self.luaT_callorderTM(l, left, right, TMS::TM_LE)
        }
    }

    pub fn luaV_lessequal(&mut self, l: ThreadId, left: &TValue, right: &TValue) -> LuaResult<bool> {
        if left.is_number() && right.is_number() {
            Ok(LEnum(left, right))
        } else {
            self.lessequalothers(l, left, right)
        }
    }

    pub fn luaV_rawequalobj(&self, t1: &TValue, t2: &TValue) -> bool {
        if t1.ttypetag() != t2.ttypetag() {
            if t1.ttype() != t2.ttype() || t1.ttype() != LUA_TNUMBER {
                return false;
            } else {
                match (
                    tointegerns(t1, F2Imod::F2Ieq),
                    tointegerns(t2, F2Imod::F2Ieq),
                ) {
                    (Some(i1), Some(i2)) => i1 == i2,
                    _ => false,
                }
            }
        } else {
            match t1.ttypetag() {
                LUA_VNIL | LUA_VFALSE | LUA_VTRUE => true,
                LUA_VNUMINT => t1.ivalue() == t2.ivalue(),
                LUA_VNUMFLT => t1.fltvalue() == t2.fltvalue(),
                LUA_VLIGHTUSERDATA => t1.pvalue() == t2.pvalue(),
                LUA_VLCF => match (t1.val, t2.val) {
                    (ValueData::Lcf(f1), ValueData::Lcf(f2)) => std::ptr::fn_addr_eq(f1, f2),
                    _ => false,
                },
                LUA_VSHRSTR => t1.tsvalue() == t2.tsvalue(),
                LUA_VLNGSTR => self.luaS_eqlngstr(t1.tsvalue(), t2.tsvalue()),
                LUA_VUSERDATA => t1.uvalue() == t2.uvalue(),
                LUA_VTABLE => t1.hvalue() == t2.hvalue(),
                _ => match (t1.val, t2.val) {
                    (ValueData::LClosure(a), ValueData::LClosure(b)) => a == b,
                    (ValueData::CClosure(a), ValueData::CClosure(b)) => a == b,
                    (ValueData::Thread(a), ValueData::Thread(b)) => a == b,
                    _ => false,
                },
            }
        }
    }

    pub fn luaV_equalobj(
        &mut self,
        l_opt: Option<ThreadId>,
        t1: &TValue,
        t2: &TValue,
    ) -> LuaResult<bool> {
        if t1.ttypetag() != t2.ttypetag() {
            return Ok(self.luaV_rawequalobj(t1, t2));
        }
        let tm_opt = match t1.ttypetag() {
            LUA_VUSERDATA => {
                if t1.uvalue() == t2.uvalue() {
                    return Ok(true);
                }
                if l_opt.is_none() {
                    return Ok(false);
                }
                let mt1 = self.udatas.get(t1.uvalue()).and_then(|u| u.metatable);
                let mut tm = self.fasttm(mt1, TMS::TM_EQ);
                if tm.is_none() {
                    let mt2 = self.udatas.get(t2.uvalue()).and_then(|u| u.metatable);
                    tm = self.fasttm(mt2, TMS::TM_EQ);
                }
                tm
            }
            LUA_VTABLE => {
                if t1.hvalue() == t2.hvalue() {
                    return Ok(true);
                }
                if l_opt.is_none() {
                    return Ok(false);
                }
                let mt1 = self.tables.get(t1.hvalue()).and_then(|t| t.metatable);
                let mut tm = self.fasttm(mt1, TMS::TM_EQ);
                if tm.is_none() {
                    let mt2 = self.tables.get(t2.hvalue()).and_then(|t| t.metatable);
                    tm = self.fasttm(mt2, TMS::TM_EQ);
                }
                tm
            }
            _ => return Ok(self.luaV_rawequalobj(t1, t2)),
        };
        match (tm_opt, l_opt) {
            (Some(tm), Some(l)) => {
                let top = self.get_top(l);
                self.luaT_callTMres(l, &tm, t1, t2, top)?;
                Ok(!self.stack_get(l, top).l_isfalse())
            }
            _ => Ok(false),
        }
    }

    fn ensure_tostring(&mut self, l: ThreadId, stk_idx: usize) -> bool {
        let mut v = self.stack_get(l, stk_idx);
        if v.is_string() {
            true
        } else if v.is_number() {
            self.luaO_tostring(&mut v);
            self.stack_set(l, stk_idx, v);
            true
        } else {
            false
        }
    }

    fn is_emptystr(&self, v: &TValue) -> bool {
        v.is_shrstring() && self.get_str_bytes(v.tsvalue()).is_empty()
    }

    pub fn luaV_concat(&mut self, l: ThreadId, mut total: i32) -> LuaResult<()> {
        if total == 1 {
            return Ok(());
        }
        loop {
            let top = self.get_top(l);
            let mut n: i32 = 2;
            let v_top2 = self.stack_get(l, top - 2);
            if !(v_top2.is_string() || v_top2.is_number()) || !self.ensure_tostring(l, top - 1) {
                self.luaT_tryconcatTM(l)?;
            } else if self.is_emptystr(&self.stack_get(l, top - 1)) {
                let _ = self.ensure_tostring(l, top - 2);
            } else if self.is_emptystr(&self.stack_get(l, top - 2)) {
                let v1 = self.stack_get(l, top - 1);
                self.stack_set(l, top - 2, v1);
            } else {
                let mut tl = self.get_str_bytes(self.stack_get(l, top - 1).tsvalue()).len();
                n = 1;
                while n < total && self.ensure_tostring(l, top - (n as usize) - 1) {
                    let s_id = self.stack_get(l, top - (n as usize) - 1).tsvalue();
                    let len = self.get_str_bytes(s_id).len();
                    if len >= (usize::MAX / 2) - tl {
                        self.set_top_raw(l, top - (total as usize));
                        return self.luaG_runerror(l, "string length overflow");
                    }
                    tl += len;
                    n += 1;
                }
                let mut buff = Vec::with_capacity(tl);
                let mut rem = n as usize;
                while rem > 0 {
                    let s_id = self.stack_get(l, top - rem).tsvalue();
                    buff.extend_from_slice(self.get_str_bytes(s_id));
                    rem -= 1;
                }
                let ts_val = self.luaS_newlstr_val(&buff);
                self.stack_set(l, top - (n as usize), ts_val);
            }
            total -= n - 1;
            let cur_top = self.get_top(l);
            self.set_top_raw(l, cur_top - ((n - 1) as usize));
            if total <= 1 {
                break;
            }
        }
        Ok(())
    }

    pub fn luaV_objlen(
        &mut self,
        l: ThreadId,
        ra: usize,
        rb: &TValue,
        rb_origin: ValOrigin,
    ) -> LuaResult<()> {
        let tm = match rb.ttypetag() {
            LUA_VTABLE => {
                let h = rb.hvalue();
                let mt = self.tables.get(h).and_then(|t| t.metatable);
                match self.fasttm(mt, TMS::TM_LEN) {
                    Some(m) => m,
                    None => {
                        let len = self.luaH_getn(h) as LuaInteger;
                        self.stack_set(l, ra, TValue::int_val(len));
                        return Ok(());
                    }
                }
            }
            LUA_VSHRSTR | LUA_VLNGSTR => {
                let len = self.get_str_bytes(rb.tsvalue()).len() as LuaInteger;
                self.stack_set(l, ra, TValue::int_val(len));
                return Ok(());
            }
            _ => {
                let m = self.luaT_gettmbyobj(rb, TMS::TM_LEN);
                if m.is_nil() {
                    return self.luaG_typeerror(l, rb, rb_origin, "get length of");
                }
                m
            }
        };
        self.luaT_callTMres(l, &tm, rb, rb, ra)
    }

    fn pushclosure(
        &mut self,
        l: ThreadId,
        p_id: ProtoId,
        encup: &[UpValId],
        base: usize,
        ra: usize,
    ) {
        let updescs = self
            .protos
            .get(p_id)
            .map_or_else(Vec::new, |p| p.upvalues.clone());
        let nup = updescs.len();
        let ncl = self.luaF_newLclosure(nup, p_id);
        self.stack_set(l, ra, TValue::lcl_val(ncl));
        let mut uvs = Vec::with_capacity(nup);
        for ud in &updescs {
            if ud.instack != 0 {
                uvs.push(self.luaF_findupval(l, base + (ud.idx as usize)));
            } else {
                uvs.push(encup.get(ud.idx as usize).copied().unwrap_or(0));
            }
        }
        if let Some(cl) = self.lclosures.get_mut(ncl) {
            cl.upvals = uvs;
        }
    }

    pub fn luaV_finishOp(&mut self, l: ThreadId) -> LuaResult<()> {
        let (ci_idx, base, savedpc) = match self.threads.get(l) {
            Some(th) => {
                let ci = th.ci();
                (th.ci_idx, ci.func + 1, ci.savedpc)
            }
            None => return Ok(()),
        };
        let fval = self.stack_get(l, base - 1);
        let p_id = self.lclosures.get(fval.clLvalue()).map_or(0, |c| c.p);
        let inst = self
            .protos
            .get(p_id)
            .and_then(|p| p.code.get(savedpc.saturating_sub(1)))
            .copied()
            .unwrap_or(0);
        let op = GET_OPCODE(inst);
        match op {
            OpCode::OP_MMBIN | OpCode::OP_MMBINI | OpCode::OP_MMBINK => {
                let pi = self
                    .protos
                    .get(p_id)
                    .and_then(|p| p.code.get(savedpc.saturating_sub(2)))
                    .copied()
                    .unwrap_or(0);
                let top = self.get_top(l) - 1;
                self.set_top_raw(l, top);
                let res_val = self.stack_get(l, top);
                self.stack_set(l, base + (GETARG_A(pi) as usize), res_val);
            }
            OpCode::OP_UNM
            | OpCode::OP_BNOT
            | OpCode::OP_LEN
            | OpCode::OP_GETTABUP
            | OpCode::OP_GETTABLE
            | OpCode::OP_GETI
            | OpCode::OP_GETFIELD
            | OpCode::OP_SELF => {
                let top = self.get_top(l) - 1;
                self.set_top_raw(l, top);
                let res_val = self.stack_get(l, top);
                self.stack_set(l, base + (GETARG_A(inst) as usize), res_val);
            }
            OpCode::OP_LT
            | OpCode::OP_LE
            | OpCode::OP_LTI
            | OpCode::OP_LEI
            | OpCode::OP_GTI
            | OpCode::OP_GEI
            | OpCode::OP_EQ => {
                let top = self.get_top(l) - 1;
                let mut res = !self.stack_get(l, top).l_isfalse();
                self.set_top_raw(l, top);
                if let Some(th) = self.threads.get_mut(l) {
                    if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                        if (ci.callstatus & CIST_LEQ) != 0 {
                            ci.callstatus ^= CIST_LEQ;
                            res = !res;
                        }
                        if (res as i32) != GETARG_k(inst) {
                            ci.savedpc += 1;
                        }
                    }
                }
            }
            OpCode::OP_CONCAT => {
                let top = self.get_top(l) - 1;
                let a = GETARG_A(inst) as usize;
                let total = (top as i32) - 1 - ((base + a) as i32);
                let v = self.stack_get(l, top);
                self.stack_set(l, top - 2, v);
                self.set_top_raw(l, top - 1);
                self.luaV_concat(l, total)?;
            }
            OpCode::OP_CLOSE => {
                if let Some(th) = self.threads.get_mut(l) {
                    if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                        ci.savedpc = ci.savedpc.saturating_sub(1);
                    }
                }
            }
            OpCode::OP_RETURN => {
                let ra = base + (GETARG_A(inst) as usize);
                let nres = self
                    .threads
                    .get(l)
                    .and_then(|th| th.ci_vec.get(ci_idx))
                    .map_or(0, |ci| ci.u2_val);
                self.set_top_raw(l, ra + (nres.max(0) as usize));
                if let Some(th) = self.threads.get_mut(l) {
                    if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                        ci.savedpc = ci.savedpc.saturating_sub(1);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn savepc(&mut self, l: ThreadId, ci_idx: usize, pc: usize) {
        if let Some(th) = self.threads.get_mut(l) {
            if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                ci.savedpc = pc;
            }
        }
    }

    fn savestate(&mut self, l: ThreadId, ci_idx: usize, pc: usize) {
        if let Some(th) = self.threads.get_mut(l) {
            if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                ci.savedpc = pc;
                let ci_top = ci.top;
                th.top = ci_top;
            }
        }
    }

    pub fn luaV_execute(&mut self, l: ThreadId, mut ci_idx: usize) -> LuaResult<()> {
        'startfunc: loop {
            'returning: loop {
                if let Some(th) = self.threads.get_mut(l) {
                    th.ci_idx = ci_idx;
                }
                let (mut base, mut pc) = match self.threads.get(l).and_then(|th| th.ci_vec.get(ci_idx)) {
                    Some(ci) => (ci.func + 1, ci.savedpc),
                    None => return Ok(()),
                };
                let cl_id = self.stack_get(l, base - 1).clLvalue();
                let p_id = self.lclosures.get(cl_id).map_or(0, |c| c.p);

                'vmloop: loop {
                    let mut i = self
                        .protos
                        .get(p_id)
                        .and_then(|p| p.code.get(pc))
                        .copied()
                        .unwrap_or(0);
                    pc += 1;

                    let op = GET_OPCODE(i);
                    match op {
                        OpCode::OP_MOVE => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = base + (GETARG_B(i) as usize);
                            let v = self.stack_get(l, rb);
                            self.stack_set(l, ra, v);
                        }
                        OpCode::OP_LOADI => {
                            let ra = base + (GETARG_A(i) as usize);
                            let b = GETARG_sBx(i) as LuaInteger;
                            self.stack_set(l, ra, TValue::int_val(b));
                        }
                        OpCode::OP_LOADF => {
                            let ra = base + (GETARG_A(i) as usize);
                            let b = GETARG_sBx(i) as LuaNumber;
                            self.stack_set(l, ra, TValue::flt_val(b));
                        }
                        OpCode::OP_LOADK => {
                            let ra = base + (GETARG_A(i) as usize);
                            let bx = GETARG_Bx(i) as usize;
                            let v = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(bx))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            self.stack_set(l, ra, v);
                        }
                        OpCode::OP_LOADKX => {
                            let ra = base + (GETARG_A(i) as usize);
                            let extra = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc))
                                .copied()
                                .unwrap_or(0);
                            pc += 1;
                            let ax = GETARG_Ax(extra) as usize;
                            let v = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(ax))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            self.stack_set(l, ra, v);
                        }
                        OpCode::OP_LOADFALSE => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.stack_set(l, ra, TValue::bool_val(false));
                        }
                        OpCode::OP_LFALSESKIP => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.stack_set(l, ra, TValue::bool_val(false));
                            pc += 1;
                        }
                        OpCode::OP_LOADTRUE => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.stack_set(l, ra, TValue::bool_val(true));
                        }
                        OpCode::OP_LOADNIL => {
                            let mut ra = base + (GETARG_A(i) as usize);
                            let mut b = GETARG_B(i);
                            loop {
                                self.stack_set(l, ra, TValue::nil());
                                ra += 1;
                                if b == 0 {
                                    break;
                                }
                                b -= 1;
                            }
                        }
                        OpCode::OP_GETUPVAL => {
                            let ra = base + (GETARG_A(i) as usize);
                            let b = GETARG_B(i) as usize;
                            let uv_id = self
                                .lclosures
                                .get(cl_id)
                                .and_then(|c| c.upvals.get(b))
                                .copied()
                                .unwrap_or(0);
                            let v = self.get_upval(uv_id);
                            self.stack_set(l, ra, v);
                        }
                        OpCode::OP_SETUPVAL => {
                            let ra = base + (GETARG_A(i) as usize);
                            let b = GETARG_B(i) as usize;
                            let uv_id = self
                                .lclosures
                                .get(cl_id)
                                .and_then(|c| c.upvals.get(b))
                                .copied()
                                .unwrap_or(0);
                            let v = self.stack_get(l, ra);
                            self.set_upval(uv_id, v);
                        }
                        OpCode::OP_GETTABUP => {
                            let ra = base + (GETARG_A(i) as usize);
                            let b = GETARG_B(i) as usize;
                            let uv_id = self
                                .lclosures
                                .get(cl_id)
                                .and_then(|c| c.upvals.get(b))
                                .copied()
                                .unwrap_or(0);
                            let upval = self.get_upval(uv_id);
                            let rc = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let key = rc.tsvalue();
                            if upval.is_table() {
                                let (sval, _) = self.luaH_getshortstr(upval.hvalue(), key);
                                if !sval.is_empty() {
                                    self.stack_set(l, ra, sval);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    upval,
                                    ValOrigin::Upval(uv_id),
                                    rc,
                                    ra,
                                    true,
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    upval,
                                    ValOrigin::Upval(uv_id),
                                    rc,
                                    ra,
                                    false,
                                )?;
                            }
                        }
                        OpCode::OP_GETTABLE => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rc_idx = base + (GETARG_C(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            let rc = self.stack_get(l, rc_idx);
                            if rb.is_table() {
                                let (sval, _) = if rc.is_integer() {
                                    self.luaH_getint(rb.hvalue(), rc.ivalue())
                                } else {
                                    self.luaH_get(rb.hvalue(), &rc)
                                };
                                if !sval.is_empty() {
                                    self.stack_set(l, ra, sval);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    rc,
                                    ra,
                                    true,
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    rc,
                                    ra,
                                    false,
                                )?;
                            }
                        }
                        OpCode::OP_GETI => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            let c = GETARG_C(i) as LuaInteger;
                            if rb.is_table() {
                                let (sval, _) = self.luaH_getint(rb.hvalue(), c);
                                if !sval.is_empty() {
                                    self.stack_set(l, ra, sval);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    TValue::int_val(c),
                                    ra,
                                    true,
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    TValue::int_val(c),
                                    ra,
                                    false,
                                )?;
                            }
                        }
                        OpCode::OP_GETFIELD => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            let rc = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let key = rc.tsvalue();
                            if rb.is_table() {
                                let (sval, _) = self.luaH_getshortstr(rb.hvalue(), key);
                                if !sval.is_empty() {
                                    self.stack_set(l, ra, sval);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    rc,
                                    ra,
                                    true,
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    rc,
                                    ra,
                                    false,
                                )?;
                            }
                        }
                        OpCode::OP_SETTABUP => {
                            let a = GETARG_A(i) as usize;
                            let uv_id = self
                                .lclosures
                                .get(cl_id)
                                .and_then(|c| c.upvals.get(a))
                                .copied()
                                .unwrap_or(0);
                            let upval = self.get_upval(uv_id);
                            let rb = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_B(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let rc = if TESTARG_k(i) {
                                self.protos
                                    .get(p_id)
                                    .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                    .copied()
                                    .unwrap_or_else(TValue::nil)
                            } else {
                                self.stack_get(l, base + (GETARG_C(i) as usize))
                            };
                            if upval.is_table() {
                                let (sval, sloc) =
                                    self.luaH_getshortstr(upval.hvalue(), rb.tsvalue());
                                if !sval.is_empty() {
                                    self.write_slot(sloc, rc);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    upval,
                                    ValOrigin::Upval(uv_id),
                                    rb,
                                    rc,
                                    Some((sval, sloc)),
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    upval,
                                    ValOrigin::Upval(uv_id),
                                    rb,
                                    rc,
                                    None,
                                )?;
                            }
                        }
                        OpCode::OP_SETTABLE => {
                            let ra_idx = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let tab = self.stack_get(l, ra_idx);
                            let rb = self.stack_get(l, rb_idx);
                            let rc = if TESTARG_k(i) {
                                self.protos
                                    .get(p_id)
                                    .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                    .copied()
                                    .unwrap_or_else(TValue::nil)
                            } else {
                                self.stack_get(l, base + (GETARG_C(i) as usize))
                            };
                            if tab.is_table() {
                                let (sval, sloc) = if rb.is_integer() {
                                    self.luaH_getint(tab.hvalue(), rb.ivalue())
                                } else {
                                    self.luaH_get(tab.hvalue(), &rb)
                                };
                                if !sval.is_empty() {
                                    self.write_slot(sloc, rc);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    tab,
                                    ValOrigin::Stack(ra_idx),
                                    rb,
                                    rc,
                                    Some((sval, sloc)),
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    tab,
                                    ValOrigin::Stack(ra_idx),
                                    rb,
                                    rc,
                                    None,
                                )?;
                            }
                        }
                        OpCode::OP_SETI => {
                            let ra_idx = base + (GETARG_A(i) as usize);
                            let tab = self.stack_get(l, ra_idx);
                            let b = GETARG_B(i) as LuaInteger;
                            let rc = if TESTARG_k(i) {
                                self.protos
                                    .get(p_id)
                                    .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                    .copied()
                                    .unwrap_or_else(TValue::nil)
                            } else {
                                self.stack_get(l, base + (GETARG_C(i) as usize))
                            };
                            if tab.is_table() {
                                let (sval, sloc) = self.luaH_getint(tab.hvalue(), b);
                                if !sval.is_empty() {
                                    self.write_slot(sloc, rc);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    tab,
                                    ValOrigin::Stack(ra_idx),
                                    TValue::int_val(b),
                                    rc,
                                    Some((sval, sloc)),
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    tab,
                                    ValOrigin::Stack(ra_idx),
                                    TValue::int_val(b),
                                    rc,
                                    None,
                                )?;
                            }
                        }
                        OpCode::OP_SETFIELD => {
                            let ra_idx = base + (GETARG_A(i) as usize);
                            let tab = self.stack_get(l, ra_idx);
                            let rb = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_B(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let rc = if TESTARG_k(i) {
                                self.protos
                                    .get(p_id)
                                    .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                    .copied()
                                    .unwrap_or_else(TValue::nil)
                            } else {
                                self.stack_get(l, base + (GETARG_C(i) as usize))
                            };
                            if tab.is_table() {
                                let (sval, sloc) =
                                    self.luaH_getshortstr(tab.hvalue(), rb.tsvalue());
                                if !sval.is_empty() {
                                    self.write_slot(sloc, rc);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    tab,
                                    ValOrigin::Stack(ra_idx),
                                    rb,
                                    rc,
                                    Some((sval, sloc)),
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishset(
                                    l,
                                    tab,
                                    ValOrigin::Stack(ra_idx),
                                    rb,
                                    rc,
                                    None,
                                )?;
                            }
                        }
                        OpCode::OP_NEWTABLE => {
                            let ra = base + (GETARG_A(i) as usize);
                            let mut b = GETARG_B(i) as u32;
                            let mut c = GETARG_C(i) as u32;
                            if b > 0 {
                                b = 1u32 << (b - 1);
                            }
                            let extra = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc))
                                .copied()
                                .unwrap_or(0);
                            if TESTARG_k(i) {
                                c += (GETARG_Ax(extra) as u32) * ((MAXARG_C as u32) + 1);
                            }
                            pc += 1;
                            self.set_top_raw(l, ra + 1);
                            let t_id = self.luaH_new();
                            self.stack_set(l, ra, TValue::table_val(t_id));
                            if b != 0 || c != 0 {
                                self.luaH_resize(l, t_id, c, b)?;
                            }
                        }
                        OpCode::OP_SELF => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            let rc = if TESTARG_k(i) {
                                self.protos
                                    .get(p_id)
                                    .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                    .copied()
                                    .unwrap_or_else(TValue::nil)
                            } else {
                                self.stack_get(l, base + (GETARG_C(i) as usize))
                            };
                            self.stack_set(l, ra + 1, rb);
                            if rb.is_table() {
                                let (sval, _) = self.luaH_getstr(rb.hvalue(), rc.tsvalue());
                                if !sval.is_empty() {
                                    self.stack_set(l, ra, sval);
                                    continue 'vmloop;
                                }
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    rc,
                                    ra,
                                    true,
                                )?;
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaV_finishget(
                                    l,
                                    rb,
                                    ValOrigin::Stack(rb_idx),
                                    rc,
                                    ra,
                                    false,
                                )?;
                            }
                        }
                        OpCode::OP_ADDI => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let imm = GETARG_sC(i);
                            if v1.is_integer() {
                                pc += 1;
                                self.stack_set(
                                    l,
                                    ra,
                                    TValue::int_val(v1.ivalue().wrapping_add(imm as LuaInteger)),
                                );
                            } else if v1.is_float() {
                                pc += 1;
                                self.stack_set(
                                    l,
                                    ra,
                                    TValue::flt_val(v1.fltvalue() + (imm as LuaNumber)),
                                );
                            }
                        }
                        OpCode::OP_ADDK
                        | OpCode::OP_SUBK
                        | OpCode::OP_MULK
                        | OpCode::OP_MODK
                        | OpCode::OP_IDIVK => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let v2 = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            if op == OpCode::OP_MODK || op == OpCode::OP_IDIVK {
                                self.savestate(l, ci_idx, pc);
                            }
                            if v1.is_integer() && v2.is_integer() {
                                let i1 = v1.ivalue();
                                let i2 = v2.ivalue();
                                let r = match op {
                                    OpCode::OP_ADDK => i1.wrapping_add(i2),
                                    OpCode::OP_SUBK => i1.wrapping_sub(i2),
                                    OpCode::OP_MULK => i1.wrapping_mul(i2),
                                    OpCode::OP_MODK => self.luaV_mod(l, i1, i2)?,
                                    _ => self.luaV_idiv(l, i1, i2)?,
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::int_val(r));
                            } else if let (Some(n1), Some(n2)) = (tonumberns(&v1), tonumberns(&v2))
                            {
                                let r = match op {
                                    OpCode::OP_ADDK => n1 + n2,
                                    OpCode::OP_SUBK => n1 - n2,
                                    OpCode::OP_MULK => n1 * n2,
                                    OpCode::OP_MODK => luaV_modf(n1, n2),
                                    _ => (n1 / n2).floor(),
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::flt_val(r));
                            }
                        }
                        OpCode::OP_POWK | OpCode::OP_DIVK => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let v2 = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            if let (Some(n1), Some(n2)) = (tonumberns(&v1), tonumberns(&v2)) {
                                let r = if op == OpCode::OP_POWK {
                                    if n2 == 2.0 {
                                        n1 * n1
                                    } else {
                                        n1.powf(n2)
                                    }
                                } else {
                                    n1 / n2
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::flt_val(r));
                            }
                        }
                        OpCode::OP_BANDK | OpCode::OP_BORK | OpCode::OP_BXORK => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let v2 = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_C(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let i2 = v2.ivalue();
                            if let Some(i1) = tointegerns(&v1, F2Imod::F2Ieq) {
                                let r = match op {
                                    OpCode::OP_BANDK => i1 & i2,
                                    OpCode::OP_BORK => i1 | i2,
                                    _ => i1 ^ i2,
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::int_val(r));
                            }
                        }
                        OpCode::OP_SHRI => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let ic = GETARG_sC(i) as LuaInteger;
                            if let Some(ib) = tointegerns(&rb, F2Imod::F2Ieq) {
                                pc += 1;
                                self.stack_set(l, ra, TValue::int_val(luaV_shiftl(ib, -ic)));
                            }
                        }
                        OpCode::OP_SHLI => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let ic = GETARG_sC(i) as LuaInteger;
                            if let Some(ib) = tointegerns(&rb, F2Imod::F2Ieq) {
                                pc += 1;
                                self.stack_set(l, ra, TValue::int_val(luaV_shiftl(ic, ib)));
                            }
                        }
                        OpCode::OP_ADD
                        | OpCode::OP_SUB
                        | OpCode::OP_MUL
                        | OpCode::OP_MOD
                        | OpCode::OP_IDIV => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let v2 = self.stack_get(l, base + (GETARG_C(i) as usize));
                            if op == OpCode::OP_MOD || op == OpCode::OP_IDIV {
                                self.savestate(l, ci_idx, pc);
                            }
                            if v1.is_integer() && v2.is_integer() {
                                let i1 = v1.ivalue();
                                let i2 = v2.ivalue();
                                let r = match op {
                                    OpCode::OP_ADD => i1.wrapping_add(i2),
                                    OpCode::OP_SUB => i1.wrapping_sub(i2),
                                    OpCode::OP_MUL => i1.wrapping_mul(i2),
                                    OpCode::OP_MOD => self.luaV_mod(l, i1, i2)?,
                                    _ => self.luaV_idiv(l, i1, i2)?,
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::int_val(r));
                            } else if let (Some(n1), Some(n2)) = (tonumberns(&v1), tonumberns(&v2))
                            {
                                let r = match op {
                                    OpCode::OP_ADD => n1 + n2,
                                    OpCode::OP_SUB => n1 - n2,
                                    OpCode::OP_MUL => n1 * n2,
                                    OpCode::OP_MOD => luaV_modf(n1, n2),
                                    _ => (n1 / n2).floor(),
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::flt_val(r));
                            }
                        }
                        OpCode::OP_POW | OpCode::OP_DIV => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let v2 = self.stack_get(l, base + (GETARG_C(i) as usize));
                            if let (Some(n1), Some(n2)) = (tonumberns(&v1), tonumberns(&v2)) {
                                let r = if op == OpCode::OP_POW {
                                    if n2 == 2.0 {
                                        n1 * n1
                                    } else {
                                        n1.powf(n2)
                                    }
                                } else {
                                    n1 / n2
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::flt_val(r));
                            }
                        }
                        OpCode::OP_BAND
                        | OpCode::OP_BOR
                        | OpCode::OP_BXOR
                        | OpCode::OP_SHR
                        | OpCode::OP_SHL => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v1 = self.stack_get(l, base + (GETARG_B(i) as usize));
                            let v2 = self.stack_get(l, base + (GETARG_C(i) as usize));
                            if let (Some(i1), Some(i2)) = (
                                tointegerns(&v1, F2Imod::F2Ieq),
                                tointegerns(&v2, F2Imod::F2Ieq),
                            ) {
                                let r = match op {
                                    OpCode::OP_BAND => i1 & i2,
                                    OpCode::OP_BOR => i1 | i2,
                                    OpCode::OP_BXOR => i1 ^ i2,
                                    OpCode::OP_SHR => luaV_shiftr(i1, i2),
                                    _ => luaV_shiftl(i1, i2),
                                };
                                pc += 1;
                                self.stack_set(l, ra, TValue::int_val(r));
                            }
                        }
                        OpCode::OP_MMBIN => {
                            let ra_idx = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let pi = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc - 2))
                                .copied()
                                .unwrap_or(0);
                            let va = self.stack_get(l, ra_idx);
                            let vb = self.stack_get(l, rb_idx);
                            let tm = TMS::from_u8(GETARG_C(i) as u8);
                            let result = base + (GETARG_A(pi) as usize);
                            self.savestate(l, ci_idx, pc);
                            self.luaT_trybinTM(
                                l,
                                &va,
                                ValOrigin::Stack(ra_idx),
                                &vb,
                                ValOrigin::Stack(rb_idx),
                                result,
                                tm,
                            )?;
                        }
                        OpCode::OP_MMBINI => {
                            let ra_idx = base + (GETARG_A(i) as usize);
                            let pi = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc - 2))
                                .copied()
                                .unwrap_or(0);
                            let imm = GETARG_sB(i) as LuaInteger;
                            let tm = TMS::from_u8(GETARG_C(i) as u8);
                            let flip = GETARG_k(i) != 0;
                            let result = base + (GETARG_A(pi) as usize);
                            let va = self.stack_get(l, ra_idx);
                            self.savestate(l, ci_idx, pc);
                            self.luaT_trybiniTM(
                                l,
                                &va,
                                ValOrigin::Stack(ra_idx),
                                imm,
                                flip,
                                result,
                                tm,
                            )?;
                        }
                        OpCode::OP_MMBINK => {
                            let ra_idx = base + (GETARG_A(i) as usize);
                            let pi = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc - 2))
                                .copied()
                                .unwrap_or(0);
                            let imm = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_B(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let tm = TMS::from_u8(GETARG_C(i) as u8);
                            let flip = GETARG_k(i) != 0;
                            let result = base + (GETARG_A(pi) as usize);
                            let va = self.stack_get(l, ra_idx);
                            self.savestate(l, ci_idx, pc);
                            self.luaT_trybinassocTM(
                                l,
                                &va,
                                ValOrigin::Stack(ra_idx),
                                &imm,
                                ValOrigin::None,
                                flip,
                                result,
                                tm,
                            )?;
                        }
                        OpCode::OP_UNM => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            if rb.is_integer() {
                                self.stack_set(
                                    l,
                                    ra,
                                    TValue::int_val(0i64.wrapping_sub(rb.ivalue())),
                                );
                            } else if let Some(nb) = tonumberns(&rb) {
                                self.stack_set(l, ra, TValue::flt_val(-nb));
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaT_trybinTM(
                                    l,
                                    &rb,
                                    ValOrigin::Stack(rb_idx),
                                    &rb,
                                    ValOrigin::Stack(rb_idx),
                                    ra,
                                    TMS::TM_UNM,
                                )?;
                            }
                        }
                        OpCode::OP_BNOT => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            if let Some(ib) = tointegerns(&rb, F2Imod::F2Ieq) {
                                self.stack_set(l, ra, TValue::int_val(!ib));
                            } else {
                                self.savestate(l, ci_idx, pc);
                                self.luaT_trybinTM(
                                    l,
                                    &rb,
                                    ValOrigin::Stack(rb_idx),
                                    &rb,
                                    ValOrigin::Stack(rb_idx),
                                    ra,
                                    TMS::TM_BNOT,
                                )?;
                            }
                        }
                        OpCode::OP_NOT => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = self.stack_get(l, base + (GETARG_B(i) as usize));
                            self.stack_set(l, ra, TValue::bool_val(rb.l_isfalse()));
                        }
                        OpCode::OP_LEN => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb_idx = base + (GETARG_B(i) as usize);
                            let rb = self.stack_get(l, rb_idx);
                            self.savestate(l, ci_idx, pc);
                            self.luaV_objlen(l, ra, &rb, ValOrigin::Stack(rb_idx))?;
                        }
                        OpCode::OP_CONCAT => {
                            let ra = base + (GETARG_A(i) as usize);
                            let n = GETARG_B(i);
                            self.set_top_raw(l, ra + (n as usize));
                            self.savepc(l, ci_idx, pc);
                            self.luaV_concat(l, n)?;
                        }
                        OpCode::OP_CLOSE => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.savestate(l, ci_idx, pc);
                            self.luaF_close(l, ra, LUA_OK, true)?;
                        }
                        OpCode::OP_TBC => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.savestate(l, ci_idx, pc);
                            self.luaF_newtbcupval(l, ra)?;
                        }
                        OpCode::OP_JMP => {
                            pc = ((pc as i32) + GETARG_sJ(i)) as usize;
                        }
                        OpCode::OP_EQ => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = base + (GETARG_B(i) as usize);
                            let va = self.stack_get(l, ra);
                            let vb = self.stack_get(l, rb);
                            self.savestate(l, ci_idx, pc);
                            let cond = self.luaV_equalobj(Some(l), &va, &vb)?;
                            if (cond as i32) != GETARG_k(i) {
                                pc += 1;
                            } else {
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_LT | OpCode::OP_LE => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = base + (GETARG_B(i) as usize);
                            let va = self.stack_get(l, ra);
                            let vb = self.stack_get(l, rb);
                            let cond = if va.is_integer() && vb.is_integer() {
                                if op == OpCode::OP_LT {
                                    va.ivalue() < vb.ivalue()
                                } else {
                                    va.ivalue() <= vb.ivalue()
                                }
                            } else if va.is_number() && vb.is_number() {
                                if op == OpCode::OP_LT {
                                    LTnum(&va, &vb)
                                } else {
                                    LEnum(&va, &vb)
                                }
                            } else {
                                self.savestate(l, ci_idx, pc);
                                if op == OpCode::OP_LT {
                                    self.lessthanothers(l, &va, &vb)?
                                } else {
                                    self.lessequalothers(l, &va, &vb)?
                                }
                            };
                            if (cond as i32) != GETARG_k(i) {
                                pc += 1;
                            } else {
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_EQK => {
                            let va = self.stack_get(l, base + (GETARG_A(i) as usize));
                            let rb = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.k.get(GETARG_B(i) as usize))
                                .copied()
                                .unwrap_or_else(TValue::nil);
                            let cond = self.luaV_rawequalobj(&va, &rb);
                            if (cond as i32) != GETARG_k(i) {
                                pc += 1;
                            } else {
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_EQI => {
                            let va = self.stack_get(l, base + (GETARG_A(i) as usize));
                            let im = GETARG_sB(i);
                            let cond = if va.is_integer() {
                                va.ivalue() == (im as LuaInteger)
                            } else if va.is_float() {
                                va.fltvalue() == (im as LuaNumber)
                            } else {
                                false
                            };
                            if (cond as i32) != GETARG_k(i) {
                                pc += 1;
                            } else {
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_LTI | OpCode::OP_LEI | OpCode::OP_GTI | OpCode::OP_GEI => {
                            let va = self.stack_get(l, base + (GETARG_A(i) as usize));
                            let im = GETARG_sB(i);
                            let cond = if va.is_integer() {
                                let iv = va.ivalue();
                                let im_i = im as LuaInteger;
                                match op {
                                    OpCode::OP_LTI => iv < im_i,
                                    OpCode::OP_LEI => iv <= im_i,
                                    OpCode::OP_GTI => iv > im_i,
                                    _ => iv >= im_i,
                                }
                            } else if va.is_float() {
                                let fv = va.fltvalue();
                                let fim = im as LuaNumber;
                                match op {
                                    OpCode::OP_LTI => fv < fim,
                                    OpCode::OP_LEI => fv <= fim,
                                    OpCode::OP_GTI => fv > fim,
                                    _ => fv >= fim,
                                }
                            } else {
                                let isf = GETARG_C(i) != 0;
                                let (inv, tm) = match op {
                                    OpCode::OP_LTI => (false, TMS::TM_LT),
                                    OpCode::OP_LEI => (false, TMS::TM_LE),
                                    OpCode::OP_GTI => (true, TMS::TM_LT),
                                    _ => (true, TMS::TM_LE),
                                };
                                self.savestate(l, ci_idx, pc);
                                self.luaT_callorderiTM(l, &va, im, inv, isf, tm)?
                            };
                            if (cond as i32) != GETARG_k(i) {
                                pc += 1;
                            } else {
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_TEST => {
                            let va = self.stack_get(l, base + (GETARG_A(i) as usize));
                            let cond = !va.l_isfalse();
                            if (cond as i32) != GETARG_k(i) {
                                pc += 1;
                            } else {
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_TESTSET => {
                            let ra = base + (GETARG_A(i) as usize);
                            let rb = self.stack_get(l, base + (GETARG_B(i) as usize));
                            if (rb.l_isfalse() as i32) == GETARG_k(i) {
                                pc += 1;
                            } else {
                                self.stack_set(l, ra, rb);
                                let ni = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                pc = ((pc as i32) + GETARG_sJ(ni) + 1) as usize;
                            }
                        }
                        OpCode::OP_CALL => {
                            let ra = base + (GETARG_A(i) as usize);
                            let b = GETARG_B(i);
                            let nresults = GETARG_C(i) - 1;
                            if b != 0 {
                                self.set_top_raw(l, ra + (b as usize));
                            }
                            self.savepc(l, ci_idx, pc);
                            if let Some(new_ci) = self.luaD_precall(l, ra, nresults)? {
                                ci_idx = new_ci;
                                continue 'startfunc;
                            }
                        }
                        OpCode::OP_TAILCALL => {
                            let ra = base + (GETARG_A(i) as usize);
                            let mut b = GETARG_B(i);
                            let nparams1 = GETARG_C(i);
                            let delta = if nparams1 != 0 {
                                let nextra = self
                                    .threads
                                    .get(l)
                                    .and_then(|th| th.ci_vec.get(ci_idx))
                                    .map_or(0, |ci| ci.nextraargs);
                                nextra + nparams1
                            } else {
                                0
                            };
                            if b != 0 {
                                self.set_top_raw(l, ra + (b as usize));
                            } else {
                                b = (self.get_top(l) as i32) - (ra as i32);
                            }
                            self.savepc(l, ci_idx, pc);
                            if TESTARG_k(i) {
                                self.luaF_closeupval(l, base);
                            }
                            let n = self.luaD_pretailcall(l, ci_idx, ra, b, delta)?;
                            if n < 0 {
                                continue 'startfunc;
                            } else {
                                if let Some(th) = self.threads.get_mut(l) {
                                    if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                                        ci.func = ((ci.func as i32) - delta) as usize;
                                    }
                                }
                                let callstatus = self
                                    .threads
                                    .get(l)
                                    .and_then(|th| th.ci_vec.get(ci_idx))
                                    .map_or(0, |ci| ci.callstatus);
                                self.luaD_poscall(l, ci_idx, n)?;
                                if (callstatus & CIST_FRESH) != 0 {
                                    return Ok(());
                                } else {
                                    ci_idx = ci_idx.saturating_sub(1);
                                    continue 'returning;
                                }
                            }
                        }
                        OpCode::OP_RETURN => {
                            let ra = base + (GETARG_A(i) as usize);
                            let mut n = GETARG_B(i) - 1;
                            let nparams1 = GETARG_C(i);
                            if n < 0 {
                                n = (self.get_top(l) as i32) - (ra as i32);
                            }
                            self.savepc(l, ci_idx, pc);
                            if TESTARG_k(i) {
                                if let Some(th) = self.threads.get_mut(l) {
                                    let ci_top = th.ci().top;
                                    th.ci_mut().u2_val = n;
                                    if th.top < ci_top {
                                        th.top = ci_top;
                                    }
                                }
                                self.luaF_close(l, base, CLOSEKTOP, true)?;
                            }
                            if nparams1 != 0 {
                                if let Some(th) = self.threads.get_mut(l) {
                                    if let Some(ci) = th.ci_vec.get_mut(ci_idx) {
                                        let shift = ci.nextraargs + nparams1;
                                        ci.func = ((ci.func as i32) - shift) as usize;
                                    }
                                }
                            }
                            self.set_top_raw(l, ra + (n as usize));
                            let callstatus = self
                                .threads
                                .get(l)
                                .and_then(|th| th.ci_vec.get(ci_idx))
                                .map_or(0, |ci| ci.callstatus);
                            self.luaD_poscall(l, ci_idx, n)?;
                            if (callstatus & CIST_FRESH) != 0 {
                                return Ok(());
                            } else {
                                ci_idx = ci_idx.saturating_sub(1);
                                continue 'returning;
                            }
                        }
                        OpCode::OP_RETURN0 => {
                            let (nres, callstatus) = self
                                .threads
                                .get(l)
                                .and_then(|th| th.ci_vec.get(ci_idx))
                                .map_or((0, 0), |ci| (ci.nresults as i32, ci.callstatus));
                            if let Some(th) = self.threads.get_mut(l) {
                                th.ci_idx = ci_idx.saturating_sub(1);
                            }
                            let mut cur_top = base - 1;
                            let mut rem = nres;
                            while rem > 0 {
                                self.stack_set(l, cur_top, TValue::nil());
                                cur_top += 1;
                                rem -= 1;
                            }
                            self.set_top_raw(l, cur_top);
                            if (callstatus & CIST_FRESH) != 0 {
                                return Ok(());
                            } else {
                                ci_idx = ci_idx.saturating_sub(1);
                                continue 'returning;
                            }
                        }
                        OpCode::OP_RETURN1 => {
                            let (mut nres, callstatus) = self
                                .threads
                                .get(l)
                                .and_then(|th| th.ci_vec.get(ci_idx))
                                .map_or((0, 0), |ci| (ci.nresults as i32, ci.callstatus));
                            if let Some(th) = self.threads.get_mut(l) {
                                th.ci_idx = ci_idx.saturating_sub(1);
                            }
                            if nres == 0 {
                                self.set_top_raw(l, base - 1);
                            } else {
                                let ra = base + (GETARG_A(i) as usize);
                                let v = self.stack_get(l, ra);
                                self.stack_set(l, base - 1, v);
                                let mut cur_top = base;
                                while nres > 1 {
                                    self.stack_set(l, cur_top, TValue::nil());
                                    cur_top += 1;
                                    nres -= 1;
                                }
                                self.set_top_raw(l, cur_top);
                            }
                            if (callstatus & CIST_FRESH) != 0 {
                                return Ok(());
                            } else {
                                ci_idx = ci_idx.saturating_sub(1);
                                continue 'returning;
                            }
                        }
                        OpCode::OP_FORLOOP => {
                            let ra = base + (GETARG_A(i) as usize);
                            if self.stack_get(l, ra + 2).is_integer() {
                                let count = self.stack_get(l, ra + 1).ivalue() as LuaUnsigned;
                                if count > 0 {
                                    let step = self.stack_get(l, ra + 2).ivalue();
                                    let mut idx = self.stack_get(l, ra).ivalue();
                                    self.stack_set(
                                        l,
                                        ra + 1,
                                        TValue::int_val((count - 1) as LuaInteger),
                                    );
                                    idx = idx.wrapping_add(step);
                                    self.stack_set(l, ra, TValue::int_val(idx));
                                    self.stack_set(l, ra + 3, TValue::int_val(idx));
                                    pc = pc.saturating_sub(GETARG_Bx(i) as usize);
                                }
                            } else if self.floatforloop(l, ra) {
                                pc = pc.saturating_sub(GETARG_Bx(i) as usize);
                            }
                        }
                        OpCode::OP_FORPREP => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.savestate(l, ci_idx, pc);
                            if self.forprep(l, ra)? {
                                pc += (GETARG_Bx(i) as usize) + 1;
                            }
                        }
                        OpCode::OP_TFORPREP => {
                            let ra = base + (GETARG_A(i) as usize);
                            self.savestate(l, ci_idx, pc);
                            self.luaF_newtbcupval(l, ra + 3)?;
                            pc += GETARG_Bx(i) as usize;
                            i = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc))
                                .copied()
                                .unwrap_or(0);
                            pc += 1;
                            // Fall through to TFORCALL
                            let ra_c = base + (GETARG_A(i) as usize);
                            let v0 = self.stack_get(l, ra_c);
                            let v1 = self.stack_get(l, ra_c + 1);
                            let v2 = self.stack_get(l, ra_c + 2);
                            self.stack_set(l, ra_c + 4, v0);
                            self.stack_set(l, ra_c + 5, v1);
                            self.stack_set(l, ra_c + 6, v2);
                            self.set_top_raw(l, ra_c + 7);
                            self.savepc(l, ci_idx, pc);
                            self.luaD_call(l, ra_c + 4, GETARG_C(i))?;
                            i = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc))
                                .copied()
                                .unwrap_or(0);
                            pc += 1;
                            let ra_l = base + (GETARG_A(i) as usize);
                            let ctrl = self.stack_get(l, ra_l + 4);
                            if !ctrl.is_nil() {
                                self.stack_set(l, ra_l + 2, ctrl);
                                pc = pc.saturating_sub(GETARG_Bx(i) as usize);
                            }
                        }
                        OpCode::OP_TFORCALL => {
                            let ra = base + (GETARG_A(i) as usize);
                            let v0 = self.stack_get(l, ra);
                            let v1 = self.stack_get(l, ra + 1);
                            let v2 = self.stack_get(l, ra + 2);
                            self.stack_set(l, ra + 4, v0);
                            self.stack_set(l, ra + 5, v1);
                            self.stack_set(l, ra + 6, v2);
                            self.set_top_raw(l, ra + 7);
                            self.savepc(l, ci_idx, pc);
                            self.luaD_call(l, ra + 4, GETARG_C(i))?;
                            i = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.code.get(pc))
                                .copied()
                                .unwrap_or(0);
                            pc += 1;
                            let ra_l = base + (GETARG_A(i) as usize);
                            let ctrl = self.stack_get(l, ra_l + 4);
                            if !ctrl.is_nil() {
                                self.stack_set(l, ra_l + 2, ctrl);
                                pc = pc.saturating_sub(GETARG_Bx(i) as usize);
                            }
                        }
                        OpCode::OP_TFORLOOP => {
                            let ra = base + (GETARG_A(i) as usize);
                            let ctrl = self.stack_get(l, ra + 4);
                            if !ctrl.is_nil() {
                                self.stack_set(l, ra + 2, ctrl);
                                pc = pc.saturating_sub(GETARG_Bx(i) as usize);
                            }
                        }
                        OpCode::OP_SETLIST => {
                            let ra = base + (GETARG_A(i) as usize);
                            let mut n = GETARG_B(i);
                            let mut last = GETARG_C(i) as u32;
                            let h_id = self.stack_get(l, ra).hvalue();
                            if n == 0 {
                                n = (self.get_top(l) as i32) - (ra as i32) - 1;
                            } else {
                                let ci_top = self
                                    .threads
                                    .get(l)
                                    .and_then(|th| th.ci_vec.get(ci_idx))
                                    .map_or(0, |ci| ci.top);
                                self.set_top_raw(l, ci_top);
                            }
                            last += n as u32;
                            if TESTARG_k(i) {
                                let extra = self
                                    .protos
                                    .get(p_id)
                                    .and_then(|p| p.code.get(pc))
                                    .copied()
                                    .unwrap_or(0);
                                last += (GETARG_Ax(extra) as u32) * ((MAXARG_C as u32) + 1);
                                pc += 1;
                            }
                            let cur_asize = self.tables.get(h_id).map_or(0, |t| t.realasize());
                            if last > cur_asize {
                                self.luaH_resizearray(l, h_id, last)?;
                            }
                            while n > 0 {
                                let val = self.stack_get(l, ra + (n as usize));
                                if let Some(t) = self.tables.get_mut(h_id) {
                                    if let Some(elem) = t.array.get_mut((last - 1) as usize) {
                                        *elem = val;
                                    }
                                }
                                last -= 1;
                                n -= 1;
                            }
                        }
                        OpCode::OP_CLOSURE => {
                            let ra = base + (GETARG_A(i) as usize);
                            let sub_p = self
                                .protos
                                .get(p_id)
                                .and_then(|p| p.p.get(GETARG_Bx(i) as usize))
                                .copied()
                                .unwrap_or(0);
                            let encup = self
                                .lclosures
                                .get(cl_id)
                                .map_or_else(Vec::new, |c| c.upvals.clone());
                            self.savestate(l, ci_idx, pc);
                            self.pushclosure(l, sub_p, &encup, base, ra);
                        }
                        OpCode::OP_VARARG => {
                            let ra = base + (GETARG_A(i) as usize);
                            let n = GETARG_C(i) - 1;
                            self.savestate(l, ci_idx, pc);
                            self.luaT_getvarargs(l, ra, n)?;
                        }
                        OpCode::OP_VARARGPREP => {
                            self.savepc(l, ci_idx, pc);
                            self.luaT_adjustvarargs(l, GETARG_A(i), p_id)?;
                            base = self
                                .threads
                                .get(l)
                                .and_then(|th| th.ci_vec.get(ci_idx))
                                .map_or(1, |ci| ci.func + 1);
                        }
                        OpCode::OP_EXTRAARG => {}
                    }
                }
            }
        }
    }
}

// ============================================================================
// Lexer (llex.c)
// ============================================================================

const FIRST_RESERVED: i32 = 256;
const TK_AND: i32 = FIRST_RESERVED;
const TK_BREAK: i32 = FIRST_RESERVED + 1;
const TK_DO: i32 = FIRST_RESERVED + 2;
const TK_ELSE: i32 = FIRST_RESERVED + 3;
const TK_ELSEIF: i32 = FIRST_RESERVED + 4;
const TK_END: i32 = FIRST_RESERVED + 5;
const TK_FALSE: i32 = FIRST_RESERVED + 6;
const TK_FOR: i32 = FIRST_RESERVED + 7;
const TK_FUNCTION: i32 = FIRST_RESERVED + 8;
const TK_GOTO: i32 = FIRST_RESERVED + 9;
const TK_IF: i32 = FIRST_RESERVED + 10;
const TK_IN: i32 = FIRST_RESERVED + 11;
const TK_LOCAL: i32 = FIRST_RESERVED + 12;
const TK_NIL: i32 = FIRST_RESERVED + 13;
const TK_NOT: i32 = FIRST_RESERVED + 14;
const TK_OR: i32 = FIRST_RESERVED + 15;
const TK_REPEAT: i32 = FIRST_RESERVED + 16;
const TK_RETURN: i32 = FIRST_RESERVED + 17;
const TK_THEN: i32 = FIRST_RESERVED + 18;
const TK_TRUE: i32 = FIRST_RESERVED + 19;
const TK_UNTIL: i32 = FIRST_RESERVED + 20;
const TK_WHILE: i32 = FIRST_RESERVED + 21;
const TK_IDIV: i32 = FIRST_RESERVED + 22;
const TK_CONCAT: i32 = FIRST_RESERVED + 23;
const TK_DOTS: i32 = FIRST_RESERVED + 24;
const TK_EQ: i32 = FIRST_RESERVED + 25;
const TK_GE: i32 = FIRST_RESERVED + 26;
const TK_LE: i32 = FIRST_RESERVED + 27;
const TK_NE: i32 = FIRST_RESERVED + 28;
const TK_SHL: i32 = FIRST_RESERVED + 29;
const TK_SHR: i32 = FIRST_RESERVED + 30;
const TK_DBCOLON: i32 = FIRST_RESERVED + 31;
const TK_EOS: i32 = FIRST_RESERVED + 32;
const TK_FLT: i32 = FIRST_RESERVED + 33;
const TK_INT: i32 = FIRST_RESERVED + 34;
const TK_NAME: i32 = FIRST_RESERVED + 35;
const TK_STRING: i32 = FIRST_RESERVED + 36;

const NUM_RESERVED: usize = (TK_WHILE - FIRST_RESERVED + 1) as usize;
const EOZ: i32 = -1;

static LUAX_TOKENS: [&str; 37] = [
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "goto", "if", "in",
    "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while", "//", "..",
    "...", "==", ">=", "<=", "~=", "<<", ">>", "::", "<eof>", "<number>", "<integer>", "<name>",
    "<string>",
];

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub token: i32,
    pub r: LuaNumber,
    pub i: LuaInteger,
    pub ts: StrId,
}

impl Token {
    pub const fn empty() -> Self {
        Token {
            token: TK_EOS,
            r: 0.0,
            i: 0,
            ts: 0,
        }
    }
}

pub enum InputSource {
    Buffer { bytes: Vec<u8>, pos: usize },
    Reader { slot: usize, buf: Vec<u8>, pos: usize, eof: bool },
}

pub struct LexState {
    pub current: i32,
    pub linenumber: i32,
    pub lastline: i32,
    pub t: Token,
    pub lookahead: Token,
    pub fs_stack: Vec<FuncState>,
    pub l: ThreadId,
    pub input: InputSource,
    pub buff: Vec<u8>,
    pub h: TableId,
    pub dyd: Dyndata,
    pub source: StrId,
    pub envn: StrId,
    pub brkn: StrId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum ExpKind {
    VVOID = 0,
    VNIL = 1,
    VTRUE = 2,
    VFALSE = 3,
    VK = 4,
    VKFLT = 5,
    VKINT = 6,
    VKSTR = 7,
    VNONRELOC = 8,
    VLOCAL = 9,
    VUPVAL = 10,
    VCONST = 11,
    VINDEXED = 12,
    VINDEXUP = 13,
    VINDEXI = 14,
    VINDEXSTR = 15,
    VJMP = 16,
    VRELOC = 17,
    VCALL = 18,
    VVARARG = 19,
}

pub fn vkisvar(k: ExpKind) -> bool {
    (ExpKind::VLOCAL as u8) <= (k as u8) && (k as u8) <= (ExpKind::VINDEXSTR as u8)
}

pub fn vkisindexed(k: ExpKind) -> bool {
    (ExpKind::VINDEXED as u8) <= (k as u8) && (k as u8) <= (ExpKind::VINDEXSTR as u8)
}

#[derive(Clone, Copy, Debug)]
pub struct ExpDesc {
    pub k: ExpKind,
    pub ival: LuaInteger,
    pub nval: LuaNumber,
    pub strval: StrId,
    pub info: i32,
    pub ind_idx: i16,
    pub ind_t: u8,
    pub var_ridx: u8,
    pub var_vidx: u16,
    pub t: i32,
    pub f: i32,
}

impl ExpDesc {
    pub fn new(k: ExpKind, info: i32) -> Self {
        ExpDesc {
            k,
            ival: 0,
            nval: 0.0,
            strval: 0,
            info,
            ind_idx: 0,
            ind_t: 0,
            var_ridx: 0,
            var_vidx: 0,
            t: NO_JUMP,
            f: NO_JUMP,
        }
    }
    pub fn hasjumps(&self) -> bool {
        self.t != self.f
    }
}

const VDKREG: u8 = 0;
const RDKCONST: u8 = 1;
const RDKTOCLOSE: u8 = 2;
const RDKCTC: u8 = 3;

#[derive(Clone, Copy, Debug)]
pub struct Vardesc {
    pub kind: u8,
    pub ridx: u8,
    pub pidx: i16,
    pub name: Option<StrId>,
    pub k: TValue,
}

#[derive(Clone, Copy, Debug)]
pub struct Labeldesc {
    pub name: StrId,
    pub pc: i32,
    pub line: i32,
    pub nactvar: u8,
    pub close: u8,
}

#[derive(Clone, Debug)]
pub struct Dyndata {
    pub actvar: Vec<Vardesc>,
    pub gt: Vec<Labeldesc>,
    pub label: Vec<Labeldesc>,
}

impl Dyndata {
    pub fn new() -> Self {
        Dyndata {
            actvar: Vec::new(),
            gt: Vec::new(),
            label: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BlockCnt {
    pub firstlabel: usize,
    pub firstgoto: usize,
    pub nactvar: u8,
    pub upval: u8,
    pub isloop: u8,
    pub insidetbc: u8,
}

#[derive(Clone, Debug)]
pub struct FuncState {
    pub f: ProtoId,
    pub bl_stack: Vec<BlockCnt>,
    pub pc: i32,
    pub lasttarget: i32,
    pub previousline: i32,
    pub nk: i32,
    pub np: i32,
    pub nabslineinfo: i32,
    pub firstlocal: usize,
    pub firstlabel: usize,
    pub ndebugvars: i16,
    pub nactvar: u8,
    pub nups: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needclose: u8,
}

impl FuncState {
    pub fn new(f: ProtoId, linedefined: i32, firstlocal: usize, firstlabel: usize) -> Self {
        FuncState {
            f,
            bl_stack: Vec::new(),
            pc: 0,
            lasttarget: 0,
            previousline: linedefined,
            nk: 0,
            np: 0,
            nabslineinfo: 0,
            firstlocal,
            firstlabel,
            ndebugvars: 0,
            nactvar: 0,
            nups: 0,
            freereg: 0,
            iwthabs: 0,
            needclose: 0,
        }
    }
}

impl LexState {
    pub fn fs(&self) -> &FuncState {
        match self.fs_stack.last() {
            Some(f) => f,
            None => loop {},
        }
    }
    pub fn fs_mut(&mut self) -> &mut FuncState {
        match self.fs_stack.last_mut() {
            Some(f) => f,
            None => loop {},
        }
    }
}

impl LuaVM {
    pub fn zgetc(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        match &mut ls.input {
            InputSource::Buffer { bytes, pos } => {
                if let Some(&b) = bytes.get(*pos) {
                    *pos += 1;
                    Ok(b as i32)
                } else {
                    Ok(EOZ)
                }
            }
            InputSource::Reader {
                slot,
                buf,
                pos,
                eof,
            } => {
                if let Some(&b) = buf.get(*pos) {
                    *pos += 1;
                    return Ok(b as i32);
                }
                if *eof {
                    return Ok(EOZ);
                }
                let l = ls.l;
                if !self.lua_checkstack(l, 2)? {
                    return self.luaL_error_str(l, "too many nested functions");
                }
                self.lua_pushvalue(l, 1);
                self.lua_call(l, 0, 1)?;
                if self.lua_isnil(l, -1) {
                    self.lua_pop(l, 1);
                    *eof = true;
                    return Ok(EOZ);
                } else if !self.lua_isstring(l, -1) {
                    return self.luaL_error_str(l, "reader function must return a string");
                }
                self.lua_replace(l, *slot as i32)?;
                let bytes = self
                    .lua_tolstring(l, *slot as i32)?
                    .unwrap_or_default();
                if bytes.is_empty() {
                    *eof = true;
                    return Ok(EOZ);
                }
                let first = bytes.first().copied().unwrap_or(0) as i32;
                *buf = bytes;
                *pos = 1;
                Ok(first)
            }
        }
    }

    fn lex_next(&mut self, ls: &mut LexState) -> LuaResult<()> {
        ls.current = self.zgetc(ls)?;
        Ok(())
    }

    fn lex_save(&self, ls: &mut LexState, c: i32) {
        ls.buff.push(c as u8);
    }

    fn lex_save_and_next(&mut self, ls: &mut LexState) -> LuaResult<()> {
        self.lex_save(ls, ls.current);
        self.lex_next(ls)
    }

    pub fn luaX_token2str(&self, token: i32) -> String {
        if token < FIRST_RESERVED {
            if lisprint(token) {
                format!("'{}'", (token as u8) as char)
            } else {
                format!("'<\\{token}>'")
            }
        } else {
            let s = LUAX_TOKENS
                .get((token - FIRST_RESERVED) as usize)
                .copied()
                .unwrap_or("?");
            if token < TK_EOS {
                format!("'{s}'")
            } else {
                s.to_string()
            }
        }
    }

    fn txtToken(&self, ls: &LexState, token: i32) -> String {
        match token {
            TK_NAME | TK_STRING | TK_FLT | TK_INT => {
                let s_nul_len = ls
                    .buff
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(ls.buff.len());
                let slice = ls.buff.get(..s_nul_len).unwrap_or(&[]);
                format!("'{}'", String::from_utf8_lossy(slice))
            }
            _ => self.luaX_token2str(token),
        }
    }

    pub fn lexerror<T>(&mut self, ls: &LexState, msg: &str, token: i32) -> LuaResult<T> {
        let info_msg = self.luaG_addinfo(msg, Some(ls.source), ls.linenumber);
        let full_msg = if token != 0 {
            format!("{info_msg} near {}", self.txtToken(ls, token))
        } else {
            info_msg
        };
        let str_val = self.luaS_newlstr_val(full_msg.as_bytes());
        let top = self.get_top(ls.l);
        self.stack_set(ls.l, top, str_val);
        self.set_top_raw(ls.l, top + 1);
        Err(LUA_ERRSYNTAX)
    }

    pub fn luaX_syntaxerror<T>(&mut self, ls: &LexState, msg: &str) -> LuaResult<T> {
        self.lexerror(ls, msg, ls.t.token)
    }

    pub fn luaX_newstring(&mut self, ls: &mut LexState, bytes: &[u8]) -> LuaResult<StrId> {
        let ts = self.luaS_newlstr(bytes);
        let (o, sloc) = self.luaH_getstr(ls.h, ts);
        if !o.is_nil() {
            if let SlotLoc::NodeVal(t_id, n_idx) = sloc {
                if let Some(nd) = self.tables.get(t_id).and_then(|t| t.node.get(n_idx)) {
                    return Ok(nd.key_strval());
                }
            }
            Ok(ts)
        } else {
            let is_short = bytes.len() <= LUAI_MAXSHORTLEN;
            let stv = TValue::str_val(ts, is_short);
            self.luaH_finishset(ls.l, ls.h, &stv, &o, sloc, stv)?;
            Ok(ts)
        }
    }

    fn currIsNewline(ls: &LexState) -> bool {
        ls.current == (b'\n' as i32) || ls.current == (b'\r' as i32)
    }

    fn inclinenumber(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let old = ls.current;
        self.lex_next(ls)?;
        if Self::currIsNewline(ls) && ls.current != old {
            self.lex_next(ls)?;
        }
        ls.linenumber = ls.linenumber.saturating_add(1);
        if ls.linenumber >= i32::MAX {
            return self.lexerror(ls, "chunk has too many lines", 0);
        }
        Ok(())
    }

    fn check_next1(&mut self, ls: &mut LexState, c: u8) -> LuaResult<bool> {
        if ls.current == (c as i32) {
            self.lex_next(ls)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check_next2(&mut self, ls: &mut LexState, c1: u8, c2: u8) -> LuaResult<bool> {
        if ls.current == (c1 as i32) || ls.current == (c2 as i32) {
            self.lex_save_and_next(ls)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn read_numeral(&mut self, ls: &mut LexState, tok: &mut Token) -> LuaResult<i32> {
        let mut expo = (b'E', b'e');
        let first = ls.current;
        self.lex_save_and_next(ls)?;
        if first == (b'0' as i32) && self.check_next2(ls, b'x', b'X')? {
            expo = (b'P', b'p');
        }
        loop {
            if self.check_next2(ls, expo.0, expo.1)? {
                self.check_next2(ls, b'-', b'+')?;
            } else if lisxdigit(ls.current) || ls.current == (b'.' as i32) {
                self.lex_save_and_next(ls)?;
            } else {
                break;
            }
        }
        if lislalpha(ls.current) {
            self.lex_save_and_next(ls)?;
        }
        let Some((obj, _)) = luaO_str2num(&ls.buff) else {
            return self.lexerror(ls, "malformed number", TK_FLT);
        };
        if obj.is_integer() {
            tok.i = obj.ivalue();
            Ok(TK_INT)
        } else {
            tok.r = obj.fltvalue();
            Ok(TK_FLT)
        }
    }

    fn skip_sep(&mut self, ls: &mut LexState) -> LuaResult<usize> {
        let mut count: usize = 0;
        let s = ls.current;
        self.lex_save_and_next(ls)?;
        while ls.current == (b'=' as i32) {
            self.lex_save_and_next(ls)?;
            count += 1;
        }
        if ls.current == s {
            Ok(count + 2)
        } else if count == 0 {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn read_long_string(
        &mut self,
        ls: &mut LexState,
        is_str: bool,
        sep: usize,
    ) -> LuaResult<Option<StrId>> {
        let line = ls.linenumber;
        self.lex_save_and_next(ls)?;
        if Self::currIsNewline(ls) {
            self.inclinenumber(ls)?;
        }
        loop {
            match ls.current {
                EOZ => {
                    let what = if is_str { "string" } else { "comment" };
                    let msg = format!("unfinished long {what} (starting at line {line})");
                    return self.lexerror(ls, &msg, TK_EOS);
                }
                c if c == (b']' as i32) => {
                    if self.skip_sep(ls)? == sep {
                        self.lex_save_and_next(ls)?;
                        break;
                    }
                }
                c if c == (b'\n' as i32) || c == (b'\r' as i32) => {
                    self.lex_save(ls, b'\n' as i32);
                    self.inclinenumber(ls)?;
                    if !is_str {
                        ls.buff.clear();
                    }
                }
                _ => {
                    if is_str {
                        self.lex_save_and_next(ls)?;
                    } else {
                        self.lex_next(ls)?;
                    }
                }
            }
        }
        if is_str {
            let len = ls.buff.len();
            let content = ls
                .buff
                .get(sep..len.saturating_sub(sep))
                .unwrap_or(&[])
                .to_vec();
            let ts = self.luaX_newstring(ls, &content)?;
            Ok(Some(ts))
        } else {
            Ok(None)
        }
    }

    fn esccheck(&mut self, ls: &mut LexState, cond: bool, msg: &str) -> LuaResult<()> {
        if !cond {
            if ls.current != EOZ {
                self.lex_save_and_next(ls)?;
            }
            return self.lexerror(ls, msg, TK_STRING);
        }
        Ok(())
    }

    fn gethexa(&mut self, ls: &mut LexState) -> LuaResult<u8> {
        self.lex_save_and_next(ls)?;
        let is_hex = lisxdigit(ls.current);
        self.esccheck(ls, is_hex, "hexadecimal digit expected")?;
        Ok(luaO_hexavalue(ls.current as u8))
    }

    fn readhexaesc(&mut self, ls: &mut LexState) -> LuaResult<u8> {
        let mut r = self.gethexa(ls)?;
        r = (r << 4) + self.gethexa(ls)?;
        let new_len = ls.buff.len().saturating_sub(2);
        ls.buff.truncate(new_len);
        Ok(r)
    }

    fn readutf8esc(&mut self, ls: &mut LexState) -> LuaResult<u64> {
        let mut i: usize = 4;
        self.lex_save_and_next(ls)?;
        let has_brace = ls.current == (b'{' as i32);
        self.esccheck(ls, has_brace, "missing '{'")?;
        let mut r = self.gethexa(ls)? as u64;
        loop {
            self.lex_save_and_next(ls)?;
            if !lisxdigit(ls.current) {
                break;
            }
            i += 1;
            self.esccheck(ls, r <= (0x7FFFFFFFu64 >> 4), "UTF-8 value too large")?;
            r = (r << 4) + (luaO_hexavalue(ls.current as u8) as u64);
        }
        let has_close = ls.current == (b'}' as i32);
        self.esccheck(ls, has_close, "missing '}'")?;
        self.lex_next(ls)?;
        let new_len = ls.buff.len().saturating_sub(i);
        ls.buff.truncate(new_len);
        Ok(r)
    }

    fn utf8esc(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let code = self.readutf8esc(ls)?;
        let bytes = luaO_utf8esc(code);
        for b in bytes {
            self.lex_save(ls, b as i32);
        }
        Ok(())
    }

    fn readdecesc(&mut self, ls: &mut LexState) -> LuaResult<u8> {
        let mut i: usize = 0;
        let mut r: i32 = 0;
        while i < 3 && lisdigit(ls.current) {
            r = 10 * r + (ls.current - (b'0' as i32));
            self.lex_save_and_next(ls)?;
            i += 1;
        }
        self.esccheck(ls, r <= 255, "decimal escape too large")?;
        let new_len = ls.buff.len().saturating_sub(i);
        ls.buff.truncate(new_len);
        Ok(r as u8)
    }

    fn read_string(&mut self, ls: &mut LexState, del: i32) -> LuaResult<StrId> {
        self.lex_save_and_next(ls)?;
        while ls.current != del {
            match ls.current {
                EOZ => return self.lexerror(ls, "unfinished string", TK_EOS),
                c if c == (b'\n' as i32) || c == (b'\r' as i32) => {
                    return self.lexerror(ls, "unfinished string", TK_STRING);
                }
                c if c == (b'\\' as i32) => {
                    self.lex_save_and_next(ls)?;
                    match ls.current {
                        x if x == (b'a' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, 0x07);
                        }
                        x if x == (b'b' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, 0x08);
                        }
                        x if x == (b'f' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, 0x0c);
                        }
                        x if x == (b'n' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, b'\n' as i32);
                        }
                        x if x == (b'r' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, b'\r' as i32);
                        }
                        x if x == (b't' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, b'\t' as i32);
                        }
                        x if x == (b'v' as i32) => {
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, 0x0b);
                        }
                        x if x == (b'x' as i32) => {
                            let ch = self.readhexaesc(ls)?;
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, ch as i32);
                        }
                        x if x == (b'u' as i32) => {
                            self.utf8esc(ls)?;
                        }
                        x if x == (b'\n' as i32) || x == (b'\r' as i32) => {
                            self.inclinenumber(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, b'\n' as i32);
                        }
                        x if x == (b'\\' as i32)
                            || x == (b'"' as i32)
                            || x == (b'\'' as i32) =>
                        {
                            let ch = ls.current;
                            self.lex_next(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, ch);
                        }
                        EOZ => {}
                        x if x == (b'z' as i32) => {
                            ls.buff.pop();
                            self.lex_next(ls)?;
                            while lisspace(ls.current) {
                                if Self::currIsNewline(ls) {
                                    self.inclinenumber(ls)?;
                                } else {
                                    self.lex_next(ls)?;
                                }
                            }
                        }
                        _ => {
                            let is_d = lisdigit(ls.current);
                            self.esccheck(ls, is_d, "invalid escape sequence")?;
                            let ch = self.readdecesc(ls)?;
                            ls.buff.pop();
                            self.lex_save(ls, ch as i32);
                        }
                    }
                }
                _ => {
                    self.lex_save_and_next(ls)?;
                }
            }
        }
        self.lex_save_and_next(ls)?;
        let len = ls.buff.len();
        let content = ls
            .buff
            .get(1..len.saturating_sub(1))
            .unwrap_or(&[])
            .to_vec();
        self.luaX_newstring(ls, &content)
    }

    fn llex(&mut self, ls: &mut LexState) -> LuaResult<Token> {
        let mut tok = Token::empty();
        ls.buff.clear();
        loop {
            match ls.current {
                c if c == (b'\n' as i32) || c == (b'\r' as i32) => {
                    self.inclinenumber(ls)?;
                }
                c if c == (b' ' as i32)
                    || c == 0x0c
                    || c == (b'\t' as i32)
                    || c == 0x0b =>
                {
                    self.lex_next(ls)?;
                }
                c if c == (b'-' as i32) => {
                    self.lex_next(ls)?;
                    if ls.current != (b'-' as i32) {
                        tok.token = b'-' as i32;
                        return Ok(tok);
                    }
                    self.lex_next(ls)?;
                    if ls.current == (b'[' as i32) {
                        let sep = self.skip_sep(ls)?;
                        ls.buff.clear();
                        if sep >= 2 {
                            self.read_long_string(ls, false, sep)?;
                            ls.buff.clear();
                            continue;
                        }
                    }
                    while !Self::currIsNewline(ls) && ls.current != EOZ {
                        self.lex_next(ls)?;
                    }
                }
                c if c == (b'[' as i32) => {
                    let sep = self.skip_sep(ls)?;
                    if sep >= 2 {
                        if let Some(ts) = self.read_long_string(ls, true, sep)? {
                            tok.ts = ts;
                        }
                        tok.token = TK_STRING;
                        return Ok(tok);
                    } else if sep == 0 {
                        return self.lexerror(ls, "invalid long string delimiter", TK_STRING);
                    }
                    tok.token = b'[' as i32;
                    return Ok(tok);
                }
                c if c == (b'=' as i32) => {
                    self.lex_next(ls)?;
                    if self.check_next1(ls, b'=')? {
                        tok.token = TK_EQ;
                    } else {
                        tok.token = b'=' as i32;
                    }
                    return Ok(tok);
                }
                c if c == (b'<' as i32) => {
                    self.lex_next(ls)?;
                    if self.check_next1(ls, b'=')? {
                        tok.token = TK_LE;
                    } else if self.check_next1(ls, b'<')? {
                        tok.token = TK_SHL;
                    } else {
                        tok.token = b'<' as i32;
                    }
                    return Ok(tok);
                }
                c if c == (b'>' as i32) => {
                    self.lex_next(ls)?;
                    if self.check_next1(ls, b'=')? {
                        tok.token = TK_GE;
                    } else if self.check_next1(ls, b'>')? {
                        tok.token = TK_SHR;
                    } else {
                        tok.token = b'>' as i32;
                    }
                    return Ok(tok);
                }
                c if c == (b'/' as i32) => {
                    self.lex_next(ls)?;
                    if self.check_next1(ls, b'/')? {
                        tok.token = TK_IDIV;
                    } else {
                        tok.token = b'/' as i32;
                    }
                    return Ok(tok);
                }
                c if c == (b'~' as i32) => {
                    self.lex_next(ls)?;
                    if self.check_next1(ls, b'=')? {
                        tok.token = TK_NE;
                    } else {
                        tok.token = b'~' as i32;
                    }
                    return Ok(tok);
                }
                c if c == (b':' as i32) => {
                    self.lex_next(ls)?;
                    if self.check_next1(ls, b':')? {
                        tok.token = TK_DBCOLON;
                    } else {
                        tok.token = b':' as i32;
                    }
                    return Ok(tok);
                }
                c if c == (b'"' as i32) || c == (b'\'' as i32) => {
                    tok.ts = self.read_string(ls, c)?;
                    tok.token = TK_STRING;
                    return Ok(tok);
                }
                c if c == (b'.' as i32) => {
                    self.lex_save_and_next(ls)?;
                    if self.check_next1(ls, b'.')? {
                        if self.check_next1(ls, b'.')? {
                            tok.token = TK_DOTS;
                        } else {
                            tok.token = TK_CONCAT;
                        }
                        return Ok(tok);
                    } else if !lisdigit(ls.current) {
                        tok.token = b'.' as i32;
                        return Ok(tok);
                    } else {
                        tok.token = self.read_numeral(ls, &mut tok)?;
                        return Ok(tok);
                    }
                }
                c if (b'0' as i32..=b'9' as i32).contains(&c) => {
                    tok.token = self.read_numeral(ls, &mut tok)?;
                    return Ok(tok);
                }
                EOZ => {
                    tok.token = TK_EOS;
                    return Ok(tok);
                }
                _ => {
                    if lislalpha(ls.current) {
                        loop {
                            self.lex_save_and_next(ls)?;
                            if !lislalnum(ls.current) {
                                break;
                            }
                        }
                        let content = ls.buff.clone();
                        let ts = self.luaX_newstring(ls, &content)?;
                        tok.ts = ts;
                        let extra = self.strings.get(ts).map_or(0, |s| s.extra);
                        if extra > 0 {
                            tok.token = (extra as i32) - 1 + FIRST_RESERVED;
                        } else {
                            tok.token = TK_NAME;
                        }
                        return Ok(tok);
                    } else {
                        let c = ls.current;
                        self.lex_next(ls)?;
                        tok.token = c;
                        return Ok(tok);
                    }
                }
            }
        }
    }

    pub fn luaX_next(&mut self, ls: &mut LexState) -> LuaResult<()> {
        ls.lastline = ls.linenumber;
        if ls.lookahead.token != TK_EOS {
            ls.t = ls.lookahead;
            ls.lookahead.token = TK_EOS;
        } else {
            ls.t = self.llex(ls)?;
        }
        Ok(())
    }

    pub fn luaX_lookahead(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        ls.lookahead = self.llex(ls)?;
        Ok(ls.lookahead.token)
    }
}

// ============================================================================
// Code Generator (lcode.c)
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum BinOpr {
    OPR_ADD = 0,
    OPR_SUB = 1,
    OPR_MUL = 2,
    OPR_MOD = 3,
    OPR_POW = 4,
    OPR_DIV = 5,
    OPR_IDIV = 6,
    OPR_BAND = 7,
    OPR_BOR = 8,
    OPR_BXOR = 9,
    OPR_SHL = 10,
    OPR_SHR = 11,
    OPR_CONCAT = 12,
    OPR_EQ = 13,
    OPR_LT = 14,
    OPR_LE = 15,
    OPR_NE = 16,
    OPR_GT = 17,
    OPR_GE = 18,
    OPR_AND = 19,
    OPR_OR = 20,
    OPR_NOBINOPR = 21,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum UnOpr {
    OPR_MINUS = 0,
    OPR_BNOT = 1,
    OPR_NOT = 2,
    OPR_LEN = 3,
    OPR_NOUNOPR = 4,
}

fn tonumeral(e: &ExpDesc) -> Option<TValue> {
    if e.hasjumps() {
        return None;
    }
    match e.k {
        ExpKind::VKINT => Some(TValue::int_val(e.ival)),
        ExpKind::VKFLT => Some(TValue::flt_val(e.nval)),
        _ => None,
    }
}

const fn fitsC(i: LuaInteger) -> bool {
    (i as LuaUnsigned).wrapping_add(OFFSET_SC as LuaUnsigned) <= (MAXARG_C as LuaUnsigned)
}

const fn fitsBx(i: LuaInteger) -> bool {
    (-OFFSET_SBX as LuaInteger) <= i && i <= ((MAXARG_BX - OFFSET_SBX) as LuaInteger)
}

impl LuaVM {
    pub fn luaK_semerror<T>(&mut self, ls: &mut LexState, msg: &str) -> LuaResult<T> {
        ls.t.token = 0;
        self.luaX_syntaxerror(ls, msg)
    }

    fn const2val(&self, ls: &LexState, e: &ExpDesc) -> TValue {
        ls.dyd
            .actvar
            .get(e.info as usize)
            .map_or_else(TValue::nil, |vd| vd.k)
    }

    pub fn luaK_exp2const(&self, ls: &LexState, e: &ExpDesc) -> Option<TValue> {
        if e.hasjumps() {
            return None;
        }
        match e.k {
            ExpKind::VFALSE => Some(TValue::bool_val(false)),
            ExpKind::VTRUE => Some(TValue::bool_val(true)),
            ExpKind::VNIL => Some(TValue::nil()),
            ExpKind::VKSTR => {
                let is_short = self.strings.get(e.strval).map_or(true, |s| s.is_short);
                Some(TValue::str_val(e.strval, is_short))
            }
            ExpKind::VCONST => Some(self.const2val(ls, e)),
            _ => tonumeral(e),
        }
    }

    fn get_inst(&self, ls: &LexState, pc: i32) -> Instruction {
        let f_id = ls.fs().f;
        self.protos
            .get(f_id)
            .and_then(|p| p.code.get(pc as usize))
            .copied()
            .unwrap_or(0)
    }

    fn set_inst(&mut self, ls: &LexState, pc: i32, inst: Instruction) {
        let f_id = ls.fs().f;
        if let Some(p) = self.protos.get_mut(f_id) {
            if let Some(slot) = p.code.get_mut(pc as usize) {
                *slot = inst;
            }
        }
    }

    fn previousinstruction(&self, ls: &LexState) -> Option<Instruction> {
        let fs = ls.fs();
        if fs.pc > fs.lasttarget {
            Some(self.get_inst(ls, fs.pc - 1))
        } else {
            None
        }
    }

    pub fn luaK_nil(&mut self, ls: &mut LexState, mut from: i32, n: i32) -> LuaResult<()> {
        let mut l = from + n - 1;
        if let Some(mut previous) = self.previousinstruction(ls) {
            if GET_OPCODE(previous) == OpCode::OP_LOADNIL {
                let pfrom = GETARG_A(previous);
                let pl = pfrom + GETARG_B(previous);
                if (pfrom <= from && from <= pl + 1) || (from <= pfrom && pfrom <= l + 1) {
                    if pfrom < from {
                        from = pfrom;
                    }
                    if pl > l {
                        l = pl;
                    }
                    SETARG_A(&mut previous, from);
                    SETARG_B(&mut previous, l - from);
                    let prev_pc = ls.fs().pc - 1;
                    self.set_inst(ls, prev_pc, previous);
                    return Ok(());
                }
            }
        }
        self.luaK_codeABCk(ls, OpCode::OP_LOADNIL, from, n - 1, 0, 0)?;
        Ok(())
    }

    fn getjump(&self, ls: &LexState, pc: i32) -> i32 {
        let offset = GETARG_sJ(self.get_inst(ls, pc));
        if offset == NO_JUMP {
            NO_JUMP
        } else {
            (pc + 1) + offset
        }
    }

    fn fixjump(&mut self, ls: &LexState, pc: i32, dest: i32) -> LuaResult<()> {
        let mut jmp = self.get_inst(ls, pc);
        let offset = dest - (pc + 1);
        if !(-OFFSET_SJ <= offset && offset <= MAXARG_SJ - OFFSET_SJ) {
            return self.luaX_syntaxerror(ls, "control structure too long");
        }
        SETARG_sJ(&mut jmp, offset);
        self.set_inst(ls, pc, jmp);
        Ok(())
    }

    pub fn luaK_concat(&mut self, ls: &LexState, l1: &mut i32, l2: i32) -> LuaResult<()> {
        if l2 == NO_JUMP {
            return Ok(());
        } else if *l1 == NO_JUMP {
            *l1 = l2;
        } else {
            let mut list = *l1;
            loop {
                let next = self.getjump(ls, list);
                if next == NO_JUMP {
                    break;
                }
                list = next;
            }
            self.fixjump(ls, list, l2)?;
        }
        Ok(())
    }

    pub fn luaK_jump(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        self.codesJ(ls, OpCode::OP_JMP, NO_JUMP, 0)
    }

    pub fn luaK_ret(&mut self, ls: &mut LexState, first: i32, nret: i32) -> LuaResult<()> {
        let op = match nret {
            0 => OpCode::OP_RETURN0,
            1 => OpCode::OP_RETURN1,
            _ => OpCode::OP_RETURN,
        };
        self.luaK_codeABCk(ls, op, first, nret + 1, 0, 0)?;
        Ok(())
    }

    fn condjump(
        &mut self,
        ls: &mut LexState,
        op: OpCode,
        a: i32,
        b: i32,
        c: i32,
        k: i32,
    ) -> LuaResult<i32> {
        self.luaK_codeABCk(ls, op, a, b, c, k)?;
        self.luaK_jump(ls)
    }

    pub fn luaK_getlabel(&mut self, ls: &mut LexState) -> i32 {
        let pc = ls.fs().pc;
        ls.fs_mut().lasttarget = pc;
        pc
    }

    fn getjumpcontrol_pc(&self, ls: &LexState, pc: i32) -> i32 {
        if pc >= 1 && testTMode(GET_OPCODE(self.get_inst(ls, pc - 1))) {
            pc - 1
        } else {
            pc
        }
    }

    fn patchtestreg(&mut self, ls: &LexState, node: i32, reg: i32) -> bool {
        let ctrl_pc = self.getjumpcontrol_pc(ls, node);
        let mut i = self.get_inst(ls, ctrl_pc);
        if GET_OPCODE(i) != OpCode::OP_TESTSET {
            return false;
        }
        if reg != NO_REG && reg != GETARG_B(i) {
            SETARG_A(&mut i, reg);
        } else {
            i = CREATE_ABCk(OpCode::OP_TEST, GETARG_B(i), 0, 0, GETARG_k(i));
        }
        self.set_inst(ls, ctrl_pc, i);
        true
    }

    fn removevalues(&mut self, ls: &LexState, mut list: i32) {
        while list != NO_JUMP {
            let next = self.getjump(ls, list);
            self.patchtestreg(ls, list, NO_REG);
            list = next;
        }
    }

    fn patchlistaux(
        &mut self,
        ls: &LexState,
        mut list: i32,
        vtarget: i32,
        reg: i32,
        dtarget: i32,
    ) -> LuaResult<()> {
        while list != NO_JUMP {
            let next = self.getjump(ls, list);
            if self.patchtestreg(ls, list, reg) {
                self.fixjump(ls, list, vtarget)?;
            } else {
                self.fixjump(ls, list, dtarget)?;
            }
            list = next;
        }
        Ok(())
    }

    pub fn luaK_patchlist(&mut self, ls: &LexState, list: i32, target: i32) -> LuaResult<()> {
        self.patchlistaux(ls, list, target, NO_REG, target)
    }

    pub fn luaK_patchtohere(&mut self, ls: &mut LexState, list: i32) -> LuaResult<()> {
        let hr = self.luaK_getlabel(ls);
        self.luaK_patchlist(ls, list, hr)
    }

    fn savelineinfo(&mut self, ls: &mut LexState, line: i32) {
        let f_id = ls.fs().f;
        let fs = ls.fs_mut();
        let mut linedif = line - fs.previousline;
        let pc = fs.pc - 1;
        let old_iwthabs = fs.iwthabs;
        fs.iwthabs = fs.iwthabs.saturating_add(1);
        if linedif.abs() >= 0x80 || old_iwthabs >= MAXIWTHABS {
            if let Some(p) = self.protos.get_mut(f_id) {
                p.abslineinfo.push(AbsLineInfo { pc, line });
            }
            fs.nabslineinfo += 1;
            linedif = ABSLINEINFO as i32;
            fs.iwthabs = 1;
        }
        if let Some(p) = self.protos.get_mut(f_id) {
            if (pc as usize) < p.lineinfo.len() {
                if let Some(slot) = p.lineinfo.get_mut(pc as usize) {
                    *slot = linedif as i8;
                }
            } else {
                p.lineinfo.push(linedif as i8);
            }
        }
        ls.fs_mut().previousline = line;
    }

    fn removelastlineinfo(&mut self, ls: &mut LexState) {
        let f_id = ls.fs().f;
        let pc = (ls.fs().pc - 1) as usize;
        let linfo = self
            .protos
            .get(f_id)
            .and_then(|p| p.lineinfo.get(pc))
            .copied()
            .unwrap_or(0);
        if linfo != ABSLINEINFO {
            let fs = ls.fs_mut();
            fs.previousline -= linfo as i32;
            fs.iwthabs = fs.iwthabs.saturating_sub(1);
        } else {
            if let Some(p) = self.protos.get_mut(f_id) {
                p.abslineinfo.pop();
            }
            let fs = ls.fs_mut();
            fs.nabslineinfo -= 1;
            fs.iwthabs = MAXIWTHABS + 1;
        }
    }

    fn removelastinstruction(&mut self, ls: &mut LexState) {
        self.removelastlineinfo(ls);
        ls.fs_mut().pc -= 1;
    }

    pub fn luaK_code(&mut self, ls: &mut LexState, i: Instruction) -> LuaResult<i32> {
        let f_id = ls.fs().f;
        let pc = ls.fs().pc;
        if let Some(p) = self.protos.get_mut(f_id) {
            if (pc as usize) < p.code.len() {
                if let Some(slot) = p.code.get_mut(pc as usize) {
                    *slot = i;
                }
            } else {
                p.code.push(i);
            }
        }
        ls.fs_mut().pc += 1;
        let lastline = ls.lastline;
        self.savelineinfo(ls, lastline);
        Ok(pc)
    }

    pub fn luaK_codeABCk(
        &mut self,
        ls: &mut LexState,
        o: OpCode,
        a: i32,
        b: i32,
        c: i32,
        k: i32,
    ) -> LuaResult<i32> {
        self.luaK_code(ls, CREATE_ABCk(o, a, b, c, k))
    }

    pub fn luaK_codeABx(
        &mut self,
        ls: &mut LexState,
        o: OpCode,
        a: i32,
        bc: u32,
    ) -> LuaResult<i32> {
        self.luaK_code(ls, CREATE_ABx(o, a, bc))
    }

    fn codeAsBx(&mut self, ls: &mut LexState, o: OpCode, a: i32, bc: i32) -> LuaResult<i32> {
        let b = (bc + OFFSET_SBX) as u32;
        self.luaK_code(ls, CREATE_ABx(o, a, b))
    }

    fn codesJ(&mut self, ls: &mut LexState, o: OpCode, sj: i32, k: i32) -> LuaResult<i32> {
        let j = (sj + OFFSET_SJ) as u32;
        self.luaK_code(ls, CREATE_sJ(o, j, k))
    }

    fn codeextraarg(&mut self, ls: &mut LexState, a: i32) -> LuaResult<i32> {
        self.luaK_code(ls, CREATE_Ax(OpCode::OP_EXTRAARG, a))
    }

    fn luaK_codek(&mut self, ls: &mut LexState, reg: i32, k: i32) -> LuaResult<i32> {
        if k <= MAXARG_BX {
            self.luaK_codeABx(ls, OpCode::OP_LOADK, reg, k as u32)
        } else {
            let p = self.luaK_codeABx(ls, OpCode::OP_LOADKX, reg, 0)?;
            self.codeextraarg(ls, k)?;
            Ok(p)
        }
    }

    pub fn luaK_checkstack(&mut self, ls: &mut LexState, n: i32) -> LuaResult<()> {
        let newstack = (ls.fs().freereg as i32) + n;
        let f_id = ls.fs().f;
        let cur_max = self.protos.get(f_id).map_or(2, |p| p.maxstacksize as i32);
        if newstack > cur_max {
            if newstack >= MAXREGS {
                return self
                    .luaX_syntaxerror(ls, "function or expression needs too many registers");
            }
            if let Some(p) = self.protos.get_mut(f_id) {
                p.maxstacksize = newstack as u8;
            }
        }
        Ok(())
    }

    pub fn luaK_reserveregs(&mut self, ls: &mut LexState, n: i32) -> LuaResult<()> {
        self.luaK_checkstack(ls, n)?;
        let fr = ls.fs().freereg;
        ls.fs_mut().freereg = ((fr as i32) + n) as u8;
        Ok(())
    }

    fn freereg(&mut self, ls: &mut LexState, reg: i32) {
        if reg >= Self::luaY_nvarstack(ls) {
            ls.fs_mut().freereg = ls.fs().freereg.saturating_sub(1);
        }
    }

    fn freeregs(&mut self, ls: &mut LexState, r1: i32, r2: i32) {
        if r1 > r2 {
            self.freereg(ls, r1);
            self.freereg(ls, r2);
        } else {
            self.freereg(ls, r2);
            self.freereg(ls, r1);
        }
    }

    fn freeexp(&mut self, ls: &mut LexState, e: &ExpDesc) {
        if e.k == ExpKind::VNONRELOC {
            self.freereg(ls, e.info);
        }
    }

    fn freeexps(&mut self, ls: &mut LexState, e1: &ExpDesc, e2: &ExpDesc) {
        let r1 = if e1.k == ExpKind::VNONRELOC {
            e1.info
        } else {
            -1
        };
        let r2 = if e2.k == ExpKind::VNONRELOC {
            e2.info
        } else {
            -1
        };
        self.freeregs(ls, r1, r2);
    }

    fn addk(&mut self, ls: &mut LexState, key: &TValue, v: &TValue) -> LuaResult<i32> {
        let f_id = ls.fs().f;
        let nk = ls.fs().nk;
        let (idx_val, sloc) = self.luaH_get(ls.h, key);
        if idx_val.is_integer() {
            let k = idx_val.ivalue() as i32;
            if k >= 0 && k < nk {
                let existing = self
                    .protos
                    .get(f_id)
                    .and_then(|p| p.k.get(k as usize))
                    .copied()
                    .unwrap_or_else(TValue::nil);
                if existing.ttypetag() == v.ttypetag() && self.luaV_rawequalobj(&existing, v) {
                    return Ok(k);
                }
            }
        }
        let k = nk;
        self.luaH_finishset(
            ls.l,
            ls.h,
            key,
            &idx_val,
            sloc,
            TValue::int_val(k as LuaInteger),
        )?;
        if let Some(p) = self.protos.get_mut(f_id) {
            if (k as usize) < p.k.len() {
                if let Some(slot) = p.k.get_mut(k as usize) {
                    *slot = *v;
                }
            } else {
                p.k.push(*v);
            }
        }
        ls.fs_mut().nk += 1;
        Ok(k)
    }

    fn stringK(&mut self, ls: &mut LexState, s: StrId) -> LuaResult<i32> {
        let is_short = self.strings.get(s).map_or(true, |ts| ts.is_short);
        let o = TValue::str_val(s, is_short);
        self.addk(ls, &o, &o)
    }

    fn luaK_intK(&mut self, ls: &mut LexState, n: LuaInteger) -> LuaResult<i32> {
        let o = TValue::int_val(n);
        self.addk(ls, &o, &o)
    }

    fn luaK_numberK(&mut self, ls: &mut LexState, r: LuaNumber) -> LuaResult<i32> {
        let o = TValue::flt_val(r);
        match luaV_flttointeger(r, F2Imod::F2Ieq) {
            None => self.addk(ls, &o, &o),
            Some(ik) => {
                let q = c_ldexp(1.0, -53 + 1);
                let k = if ik == 0 { q } else { r + r * q };
                let kv = TValue::flt_val(k);
                self.addk(ls, &kv, &o)
            }
        }
    }

    fn boolF(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        let o = TValue::bool_val(false);
        self.addk(ls, &o, &o)
    }

    fn boolT(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        let o = TValue::bool_val(true);
        self.addk(ls, &o, &o)
    }

    fn nilK(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        let k = TValue::table_val(ls.h);
        let v = TValue::nil();
        self.addk(ls, &k, &v)
    }

    pub fn luaK_int(&mut self, ls: &mut LexState, reg: i32, i: LuaInteger) -> LuaResult<()> {
        if fitsBx(i) {
            self.codeAsBx(ls, OpCode::OP_LOADI, reg, i as i32)?;
        } else {
            let k = self.luaK_intK(ls, i)?;
            self.luaK_codek(ls, reg, k)?;
        }
        Ok(())
    }

    fn luaK_float(&mut self, ls: &mut LexState, reg: i32, f: LuaNumber) -> LuaResult<()> {
        if let Some(fi) = luaV_flttointeger(f, F2Imod::F2Ieq) {
            if fitsBx(fi) {
                self.codeAsBx(ls, OpCode::OP_LOADF, reg, fi as i32)?;
                return Ok(());
            }
        }
        let k = self.luaK_numberK(ls, f)?;
        self.luaK_codek(ls, reg, k)?;
        Ok(())
    }

    fn const2exp(v: &TValue, e: &mut ExpDesc) {
        match v.ttypetag() {
            LUA_VNUMINT => {
                e.k = ExpKind::VKINT;
                e.ival = v.ivalue();
            }
            LUA_VNUMFLT => {
                e.k = ExpKind::VKFLT;
                e.nval = v.fltvalue();
            }
            LUA_VFALSE => {
                e.k = ExpKind::VFALSE;
            }
            LUA_VTRUE => {
                e.k = ExpKind::VTRUE;
            }
            LUA_VNIL => {
                e.k = ExpKind::VNIL;
            }
            LUA_VSHRSTR | LUA_VLNGSTR => {
                e.k = ExpKind::VKSTR;
                e.strval = v.tsvalue();
            }
            _ => {}
        }
    }

    pub fn luaK_setreturns(
        &mut self,
        ls: &mut LexState,
        e: &ExpDesc,
        nresults: i32,
    ) -> LuaResult<()> {
        let mut inst = self.get_inst(ls, e.info);
        if e.k == ExpKind::VCALL {
            SETARG_C(&mut inst, nresults + 1);
            self.set_inst(ls, e.info, inst);
        } else {
            let fr = ls.fs().freereg as i32;
            SETARG_C(&mut inst, nresults + 1);
            SETARG_A(&mut inst, fr);
            self.set_inst(ls, e.info, inst);
            self.luaK_reserveregs(ls, 1)?;
        }
        Ok(())
    }

    fn str2K(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        e.info = self.stringK(ls, e.strval)?;
        e.k = ExpKind::VK;
        Ok(())
    }

    pub fn luaK_setoneret(&mut self, ls: &LexState, e: &mut ExpDesc) {
        if e.k == ExpKind::VCALL {
            let inst = self.get_inst(ls, e.info);
            e.k = ExpKind::VNONRELOC;
            e.info = GETARG_A(inst);
        } else if e.k == ExpKind::VVARARG {
            let mut inst = self.get_inst(ls, e.info);
            SETARG_C(&mut inst, 2);
            self.set_inst(ls, e.info, inst);
            e.k = ExpKind::VRELOC;
        }
    }

    pub fn luaK_dischargevars(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        match e.k {
            ExpKind::VCONST => {
                let v = self.const2val(ls, e);
                Self::const2exp(&v, e);
            }
            ExpKind::VLOCAL => {
                e.info = e.var_ridx as i32;
                e.k = ExpKind::VNONRELOC;
            }
            ExpKind::VUPVAL => {
                e.info = self.luaK_codeABCk(ls, OpCode::OP_GETUPVAL, 0, e.info, 0, 0)?;
                e.k = ExpKind::VRELOC;
            }
            ExpKind::VINDEXUP => {
                e.info = self.luaK_codeABCk(
                    ls,
                    OpCode::OP_GETTABUP,
                    0,
                    e.ind_t as i32,
                    e.ind_idx as i32,
                    0,
                )?;
                e.k = ExpKind::VRELOC;
            }
            ExpKind::VINDEXI => {
                self.freereg(ls, e.ind_t as i32);
                e.info = self.luaK_codeABCk(
                    ls,
                    OpCode::OP_GETI,
                    0,
                    e.ind_t as i32,
                    e.ind_idx as i32,
                    0,
                )?;
                e.k = ExpKind::VRELOC;
            }
            ExpKind::VINDEXSTR => {
                self.freereg(ls, e.ind_t as i32);
                e.info = self.luaK_codeABCk(
                    ls,
                    OpCode::OP_GETFIELD,
                    0,
                    e.ind_t as i32,
                    e.ind_idx as i32,
                    0,
                )?;
                e.k = ExpKind::VRELOC;
            }
            ExpKind::VINDEXED => {
                self.freeregs(ls, e.ind_t as i32, e.ind_idx as i32);
                e.info = self.luaK_codeABCk(
                    ls,
                    OpCode::OP_GETTABLE,
                    0,
                    e.ind_t as i32,
                    e.ind_idx as i32,
                    0,
                )?;
                e.k = ExpKind::VRELOC;
            }
            ExpKind::VVARARG | ExpKind::VCALL => {
                self.luaK_setoneret(ls, e);
            }
            _ => {}
        }
        Ok(())
    }

    fn discharge2reg(&mut self, ls: &mut LexState, e: &mut ExpDesc, reg: i32) -> LuaResult<()> {
        self.luaK_dischargevars(ls, e)?;
        match e.k {
            ExpKind::VNIL => {
                self.luaK_nil(ls, reg, 1)?;
            }
            ExpKind::VFALSE => {
                self.luaK_codeABCk(ls, OpCode::OP_LOADFALSE, reg, 0, 0, 0)?;
            }
            ExpKind::VTRUE => {
                self.luaK_codeABCk(ls, OpCode::OP_LOADTRUE, reg, 0, 0, 0)?;
            }
            ExpKind::VKSTR => {
                self.str2K(ls, e)?;
                self.luaK_codek(ls, reg, e.info)?;
            }
            ExpKind::VK => {
                self.luaK_codek(ls, reg, e.info)?;
            }
            ExpKind::VKFLT => {
                self.luaK_float(ls, reg, e.nval)?;
            }
            ExpKind::VKINT => {
                self.luaK_int(ls, reg, e.ival)?;
            }
            ExpKind::VRELOC => {
                let mut inst = self.get_inst(ls, e.info);
                SETARG_A(&mut inst, reg);
                self.set_inst(ls, e.info, inst);
            }
            ExpKind::VNONRELOC => {
                if reg != e.info {
                    self.luaK_codeABCk(ls, OpCode::OP_MOVE, reg, e.info, 0, 0)?;
                }
            }
            _ => {
                return Ok(());
            }
        }
        e.info = reg;
        e.k = ExpKind::VNONRELOC;
        Ok(())
    }

    fn discharge2anyreg(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        if e.k != ExpKind::VNONRELOC {
            self.luaK_reserveregs(ls, 1)?;
            let reg = (ls.fs().freereg as i32) - 1;
            self.discharge2reg(ls, e, reg)?;
        }
        Ok(())
    }

    fn code_loadbool(&mut self, ls: &mut LexState, a: i32, op: OpCode) -> LuaResult<i32> {
        self.luaK_getlabel(ls);
        self.luaK_codeABCk(ls, op, a, 0, 0, 0)
    }

    fn need_value(&self, ls: &LexState, mut list: i32) -> bool {
        while list != NO_JUMP {
            let ctrl_pc = self.getjumpcontrol_pc(ls, list);
            let i = self.get_inst(ls, ctrl_pc);
            if GET_OPCODE(i) != OpCode::OP_TESTSET {
                return true;
            }
            list = self.getjump(ls, list);
        }
        false
    }

    fn exp2reg(&mut self, ls: &mut LexState, e: &mut ExpDesc, reg: i32) -> LuaResult<()> {
        self.discharge2reg(ls, e, reg)?;
        if e.k == ExpKind::VJMP {
            let info = e.info;
            self.luaK_concat(ls, &mut e.t, info)?;
        }
        if e.hasjumps() {
            let mut p_f = NO_JUMP;
            let mut p_t = NO_JUMP;
            if self.need_value(ls, e.t) || self.need_value(ls, e.f) {
                let fj = if e.k == ExpKind::VJMP {
                    NO_JUMP
                } else {
                    self.luaK_jump(ls)?
                };
                p_f = self.code_loadbool(ls, reg, OpCode::OP_LFALSESKIP)?;
                p_t = self.code_loadbool(ls, reg, OpCode::OP_LOADTRUE)?;
                self.luaK_patchtohere(ls, fj)?;
            }
            let final_lbl = self.luaK_getlabel(ls);
            self.patchlistaux(ls, e.f, final_lbl, reg, p_f)?;
            self.patchlistaux(ls, e.t, final_lbl, reg, p_t)?;
        }
        e.f = NO_JUMP;
        e.t = NO_JUMP;
        e.info = reg;
        e.k = ExpKind::VNONRELOC;
        Ok(())
    }

    pub fn luaK_exp2nextreg(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        self.luaK_dischargevars(ls, e)?;
        self.freeexp(ls, e);
        self.luaK_reserveregs(ls, 1)?;
        let reg = (ls.fs().freereg as i32) - 1;
        self.exp2reg(ls, e, reg)
    }

    pub fn luaK_exp2anyreg(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<i32> {
        self.luaK_dischargevars(ls, e)?;
        if e.k == ExpKind::VNONRELOC {
            if !e.hasjumps() {
                return Ok(e.info);
            }
            if e.info >= Self::luaY_nvarstack(ls) {
                let r = e.info;
                self.exp2reg(ls, e, r)?;
                return Ok(e.info);
            }
        }
        self.luaK_exp2nextreg(ls, e)?;
        Ok(e.info)
    }

    pub fn luaK_exp2anyregup(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        if e.k != ExpKind::VUPVAL || e.hasjumps() {
            self.luaK_exp2anyreg(ls, e)?;
        }
        Ok(())
    }

    pub fn luaK_exp2val(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        if e.hasjumps() {
            self.luaK_exp2anyreg(ls, e)?;
        } else {
            self.luaK_dischargevars(ls, e)?;
        }
        Ok(())
    }

    fn luaK_exp2K(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<bool> {
        if !e.hasjumps() {
            let info = match e.k {
                ExpKind::VTRUE => self.boolT(ls)?,
                ExpKind::VFALSE => self.boolF(ls)?,
                ExpKind::VNIL => self.nilK(ls)?,
                ExpKind::VKINT => self.luaK_intK(ls, e.ival)?,
                ExpKind::VKFLT => self.luaK_numberK(ls, e.nval)?,
                ExpKind::VKSTR => self.stringK(ls, e.strval)?,
                ExpKind::VK => e.info,
                _ => return Ok(false),
            };
            if info <= MAXINDEXRK {
                e.k = ExpKind::VK;
                e.info = info;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn exp2RK(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<i32> {
        if self.luaK_exp2K(ls, e)? {
            Ok(1)
        } else {
            self.luaK_exp2anyreg(ls, e)?;
            Ok(0)
        }
    }

    fn codeABRK(
        &mut self,
        ls: &mut LexState,
        o: OpCode,
        a: i32,
        b: i32,
        ec: &mut ExpDesc,
    ) -> LuaResult<()> {
        let k = self.exp2RK(ls, ec)?;
        self.luaK_codeABCk(ls, o, a, b, ec.info, k)?;
        Ok(())
    }

    pub fn luaK_storevar(
        &mut self,
        ls: &mut LexState,
        var: &ExpDesc,
        ex: &mut ExpDesc,
    ) -> LuaResult<()> {
        match var.k {
            ExpKind::VLOCAL => {
                self.freeexp(ls, ex);
                self.exp2reg(ls, ex, var.var_ridx as i32)?;
                return Ok(());
            }
            ExpKind::VUPVAL => {
                let e = self.luaK_exp2anyreg(ls, ex)?;
                self.luaK_codeABCk(ls, OpCode::OP_SETUPVAL, e, var.info, 0, 0)?;
            }
            ExpKind::VINDEXUP => {
                self.codeABRK(
                    ls,
                    OpCode::OP_SETTABUP,
                    var.ind_t as i32,
                    var.ind_idx as i32,
                    ex,
                )?;
            }
            ExpKind::VINDEXI => {
                self.codeABRK(
                    ls,
                    OpCode::OP_SETI,
                    var.ind_t as i32,
                    var.ind_idx as i32,
                    ex,
                )?;
            }
            ExpKind::VINDEXSTR => {
                self.codeABRK(
                    ls,
                    OpCode::OP_SETFIELD,
                    var.ind_t as i32,
                    var.ind_idx as i32,
                    ex,
                )?;
            }
            ExpKind::VINDEXED => {
                self.codeABRK(
                    ls,
                    OpCode::OP_SETTABLE,
                    var.ind_t as i32,
                    var.ind_idx as i32,
                    ex,
                )?;
            }
            _ => {}
        }
        self.freeexp(ls, ex);
        Ok(())
    }

    pub fn luaK_self(
        &mut self,
        ls: &mut LexState,
        e: &mut ExpDesc,
        key: &mut ExpDesc,
    ) -> LuaResult<()> {
        self.luaK_exp2anyreg(ls, e)?;
        let ereg = e.info;
        self.freeexp(ls, e);
        e.info = ls.fs().freereg as i32;
        e.k = ExpKind::VNONRELOC;
        self.luaK_reserveregs(ls, 2)?;
        self.codeABRK(ls, OpCode::OP_SELF, e.info, ereg, key)?;
        self.freeexp(ls, key);
        Ok(())
    }

    fn negatecondition(&mut self, ls: &LexState, e: &ExpDesc) {
        let ctrl_pc = self.getjumpcontrol_pc(ls, e.info);
        let mut pc_inst = self.get_inst(ls, ctrl_pc);
        SETARG_k(&mut pc_inst, GETARG_k(pc_inst) ^ 1);
        self.set_inst(ls, ctrl_pc, pc_inst);
    }

    fn jumponcond(&mut self, ls: &mut LexState, e: &mut ExpDesc, cond: i32) -> LuaResult<i32> {
        if e.k == ExpKind::VRELOC {
            let ie = self.get_inst(ls, e.info);
            if GET_OPCODE(ie) == OpCode::OP_NOT {
                self.removelastinstruction(ls);
                return self.condjump(
                    ls,
                    OpCode::OP_TEST,
                    GETARG_B(ie),
                    0,
                    0,
                    if cond == 0 { 1 } else { 0 },
                );
            }
        }
        self.discharge2anyreg(ls, e)?;
        self.freeexp(ls, e);
        self.condjump(ls, OpCode::OP_TESTSET, NO_REG, e.info, 0, cond)
    }

    pub fn luaK_goiftrue(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        self.luaK_dischargevars(ls, e)?;
        let pc = match e.k {
            ExpKind::VJMP => {
                self.negatecondition(ls, e);
                e.info
            }
            ExpKind::VK
            | ExpKind::VKFLT
            | ExpKind::VKINT
            | ExpKind::VKSTR
            | ExpKind::VTRUE => NO_JUMP,
            _ => self.jumponcond(ls, e, 0)?,
        };
        self.luaK_concat(ls, &mut e.f, pc)?;
        self.luaK_patchtohere(ls, e.t)?;
        e.t = NO_JUMP;
        Ok(())
    }

    pub fn luaK_goiffalse(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        self.luaK_dischargevars(ls, e)?;
        let pc = match e.k {
            ExpKind::VJMP => e.info,
            ExpKind::VNIL | ExpKind::VFALSE => NO_JUMP,
            _ => self.jumponcond(ls, e, 1)?,
        };
        self.luaK_concat(ls, &mut e.t, pc)?;
        self.luaK_patchtohere(ls, e.f)?;
        e.f = NO_JUMP;
        Ok(())
    }

    fn codenot(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        match e.k {
            ExpKind::VNIL | ExpKind::VFALSE => {
                e.k = ExpKind::VTRUE;
            }
            ExpKind::VK
            | ExpKind::VKFLT
            | ExpKind::VKINT
            | ExpKind::VKSTR
            | ExpKind::VTRUE => {
                e.k = ExpKind::VFALSE;
            }
            ExpKind::VJMP => {
                self.negatecondition(ls, e);
            }
            ExpKind::VRELOC | ExpKind::VNONRELOC => {
                self.discharge2anyreg(ls, e)?;
                self.freeexp(ls, e);
                e.info = self.luaK_codeABCk(ls, OpCode::OP_NOT, 0, e.info, 0, 0)?;
                e.k = ExpKind::VRELOC;
            }
            _ => {}
        }
        std::mem::swap(&mut e.f, &mut e.t);
        self.removevalues(ls, e.f);
        self.removevalues(ls, e.t);
        Ok(())
    }

    fn isKstr(&self, ls: &LexState, e: &ExpDesc) -> bool {
        if e.k == ExpKind::VK && !e.hasjumps() && e.info <= MAXARG_B {
            let f_id = ls.fs().f;
            self.protos
                .get(f_id)
                .and_then(|p| p.k.get(e.info as usize))
                .map_or(false, |kv| kv.is_shrstring())
        } else {
            false
        }
    }

    fn isKint(e: &ExpDesc) -> bool {
        e.k == ExpKind::VKINT && !e.hasjumps()
    }

    fn isCint(e: &ExpDesc) -> bool {
        Self::isKint(e) && (e.ival as LuaUnsigned) <= (MAXARG_C as LuaUnsigned)
    }

    fn isSCint(e: &ExpDesc) -> bool {
        Self::isKint(e) && fitsC(e.ival)
    }

    fn isSCnumber(e: &ExpDesc, pi: &mut i32, isfloat: &mut i32) -> bool {
        let i = if e.k == ExpKind::VKINT {
            e.ival
        } else if e.k == ExpKind::VKFLT {
            match luaV_flttointeger(e.nval, F2Imod::F2Ieq) {
                Some(iv) => {
                    *isfloat = 1;
                    iv
                }
                None => return false,
            }
        } else {
            return false;
        };
        if !e.hasjumps() && fitsC(i) {
            *pi = int2sC(i as i32);
            true
        } else {
            false
        }
    }

    pub fn luaK_indexed(
        &mut self,
        ls: &mut LexState,
        t: &mut ExpDesc,
        k: &mut ExpDesc,
    ) -> LuaResult<()> {
        if k.k == ExpKind::VKSTR {
            self.str2K(ls, k)?;
        }
        if t.k == ExpKind::VUPVAL && !self.isKstr(ls, k) {
            self.luaK_exp2anyreg(ls, t)?;
        }
        if t.k == ExpKind::VUPVAL {
            let temp = t.info as u8;
            t.ind_t = temp;
            t.ind_idx = k.info as i16;
            t.k = ExpKind::VINDEXUP;
        } else {
            t.ind_t = if t.k == ExpKind::VLOCAL {
                t.var_ridx
            } else {
                t.info as u8
            };
            if self.isKstr(ls, k) {
                t.ind_idx = k.info as i16;
                t.k = ExpKind::VINDEXSTR;
            } else if Self::isCint(k) {
                t.ind_idx = k.ival as i16;
                t.k = ExpKind::VINDEXI;
            } else {
                t.ind_idx = self.luaK_exp2anyreg(ls, k)? as i16;
                t.k = ExpKind::VINDEXED;
            }
        }
        Ok(())
    }

    fn validop(op: i32, v1: &TValue, v2: &TValue) -> bool {
        match op {
            LUA_OPBAND | LUA_OPBOR | LUA_OPBXOR | LUA_OPSHL | LUA_OPSHR | LUA_OPBNOT => {
                tointegerns(v1, F2Imod::F2Ieq).is_some()
                    && tointegerns(v2, F2Imod::F2Ieq).is_some()
            }
            LUA_OPDIV | LUA_OPIDIV | LUA_OPMOD => v2.nvalue() != 0.0,
            _ => true,
        }
    }

    fn constfolding(
        &mut self,
        ls: &LexState,
        op: i32,
        e1: &mut ExpDesc,
        e2: &ExpDesc,
    ) -> LuaResult<bool> {
        let (Some(v1), Some(v2)) = (tonumeral(e1), tonumeral(e2)) else {
            return Ok(false);
        };
        if !Self::validop(op, &v1, &v2) {
            return Ok(false);
        }
        let Some(res) = self.luaO_rawarith(ls.l, op, &v1, &v2)? else {
            return Ok(false);
        };
        if res.is_integer() {
            e1.k = ExpKind::VKINT;
            e1.ival = res.ivalue();
        } else {
            let n = res.fltvalue();
            if n.is_nan() || n == 0.0 {
                return Ok(false);
            }
            e1.k = ExpKind::VKFLT;
            e1.nval = n;
        }
        Ok(true)
    }

    fn binopr2op(opr: BinOpr, baser: BinOpr, base: OpCode) -> OpCode {
        OpCode::from_u8(((opr as i32) - (baser as i32) + (base as i32)) as u8)
    }

    fn unopr2op(opr: UnOpr) -> OpCode {
        OpCode::from_u8(((opr as i32) - (UnOpr::OPR_MINUS as i32) + (OpCode::OP_UNM as i32)) as u8)
    }

    fn binopr2TM(opr: BinOpr) -> TMS {
        TMS::from_u8(((opr as i32) - (BinOpr::OPR_ADD as i32) + (TMS::TM_ADD as i32)) as u8)
    }

    pub fn luaK_fixline(&mut self, ls: &mut LexState, line: i32) {
        self.removelastlineinfo(ls);
        self.savelineinfo(ls, line);
    }

    fn codeunexpval(
        &mut self,
        ls: &mut LexState,
        op: OpCode,
        e: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        let r = self.luaK_exp2anyreg(ls, e)?;
        self.freeexp(ls, e);
        e.info = self.luaK_codeABCk(ls, op, 0, r, 0, 0)?;
        e.k = ExpKind::VRELOC;
        self.luaK_fixline(ls, line);
        Ok(())
    }

    fn finishbinexpval(
        &mut self,
        ls: &mut LexState,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        op: OpCode,
        v2: i32,
        flip: i32,
        line: i32,
        mmop: OpCode,
        event: TMS,
    ) -> LuaResult<()> {
        let v1 = self.luaK_exp2anyreg(ls, e1)?;
        let pc = self.luaK_codeABCk(ls, op, 0, v1, v2, 0)?;
        self.freeexps(ls, e1, e2);
        e1.info = pc;
        e1.k = ExpKind::VRELOC;
        self.luaK_fixline(ls, line);
        self.luaK_codeABCk(ls, mmop, v1, v2, event as i32, flip)?;
        self.luaK_fixline(ls, line);
        Ok(())
    }

    fn codebinexpval(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        let op = Self::binopr2op(opr, BinOpr::OPR_ADD, OpCode::OP_ADD);
        let v2 = self.luaK_exp2anyreg(ls, e2)?;
        self.finishbinexpval(
            ls,
            e1,
            e2,
            op,
            v2,
            0,
            line,
            OpCode::OP_MMBIN,
            Self::binopr2TM(opr),
        )
    }

    fn codebini(
        &mut self,
        ls: &mut LexState,
        op: OpCode,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        flip: i32,
        line: i32,
        event: TMS,
    ) -> LuaResult<()> {
        let v2 = int2sC(e2.ival as i32);
        self.finishbinexpval(ls, e1, e2, op, v2, flip, line, OpCode::OP_MMBINI, event)
    }

    fn codebinK(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        flip: i32,
        line: i32,
    ) -> LuaResult<()> {
        let event = Self::binopr2TM(opr);
        let v2 = e2.info;
        let op = Self::binopr2op(opr, BinOpr::OPR_ADD, OpCode::OP_ADDK);
        self.finishbinexpval(ls, e1, e2, op, v2, flip, line, OpCode::OP_MMBINK, event)
    }

    fn finishbinexpneg(
        &mut self,
        ls: &mut LexState,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        op: OpCode,
        line: i32,
        event: TMS,
    ) -> LuaResult<bool> {
        if !Self::isKint(e2) {
            return Ok(false);
        }
        let i2 = e2.ival;
        if !(fitsC(i2) && fitsC(-i2)) {
            return Ok(false);
        }
        let v2 = i2 as i32;
        self.finishbinexpval(
            ls,
            e1,
            e2,
            op,
            int2sC(-v2),
            0,
            line,
            OpCode::OP_MMBINI,
            event,
        )?;
        let last_pc = ls.fs().pc - 1;
        let mut last_inst = self.get_inst(ls, last_pc);
        SETARG_B(&mut last_inst, int2sC(v2));
        self.set_inst(ls, last_pc, last_inst);
        Ok(true)
    }

    fn codebinNoK(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        flip: i32,
        line: i32,
    ) -> LuaResult<()> {
        if flip != 0 {
            std::mem::swap(e1, e2);
        }
        self.codebinexpval(ls, opr, e1, e2, line)
    }

    fn codearith(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        flip: i32,
        line: i32,
    ) -> LuaResult<()> {
        if tonumeral(e2).is_some() && self.luaK_exp2K(ls, e2)? {
            self.codebinK(ls, opr, e1, e2, flip, line)
        } else {
            self.codebinNoK(ls, opr, e1, e2, flip, line)
        }
    }

    fn codecommutative(
        &mut self,
        ls: &mut LexState,
        op: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        let mut flip = 0;
        if tonumeral(e1).is_some() {
            std::mem::swap(e1, e2);
            flip = 1;
        }
        if op == BinOpr::OPR_ADD && Self::isSCint(e2) {
            self.codebini(ls, OpCode::OP_ADDI, e1, e2, flip, line, TMS::TM_ADD)
        } else {
            self.codearith(ls, op, e1, e2, flip, line)
        }
    }

    fn codebitwise(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        let mut flip = 0;
        if e1.k == ExpKind::VKINT {
            std::mem::swap(e1, e2);
            flip = 1;
        }
        if e2.k == ExpKind::VKINT && self.luaK_exp2K(ls, e2)? {
            self.codebinK(ls, opr, e1, e2, flip, line)
        } else {
            self.codebinNoK(ls, opr, e1, e2, flip, line)
        }
    }

    fn codeorder(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
    ) -> LuaResult<()> {
        let mut im = 0;
        let mut isfloat = 0;
        let (r1, r2, op) = if Self::isSCnumber(e2, &mut im, &mut isfloat) {
            let r1 = self.luaK_exp2anyreg(ls, e1)?;
            (r1, im, Self::binopr2op(opr, BinOpr::OPR_LT, OpCode::OP_LTI))
        } else if Self::isSCnumber(e1, &mut im, &mut isfloat) {
            let r1 = self.luaK_exp2anyreg(ls, e2)?;
            (r1, im, Self::binopr2op(opr, BinOpr::OPR_LT, OpCode::OP_GTI))
        } else {
            let r1 = self.luaK_exp2anyreg(ls, e1)?;
            let r2 = self.luaK_exp2anyreg(ls, e2)?;
            (r1, r2, Self::binopr2op(opr, BinOpr::OPR_LT, OpCode::OP_LT))
        };
        self.freeexps(ls, e1, e2);
        e1.info = self.condjump(ls, op, r1, r2, isfloat, 1)?;
        e1.k = ExpKind::VJMP;
        Ok(())
    }

    fn codeeq(
        &mut self,
        ls: &mut LexState,
        opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
    ) -> LuaResult<()> {
        if e1.k != ExpKind::VNONRELOC {
            std::mem::swap(e1, e2);
        }
        let r1 = self.luaK_exp2anyreg(ls, e1)?;
        let mut im = 0;
        let mut isfloat = 0;
        let (op, r2) = if Self::isSCnumber(e2, &mut im, &mut isfloat) {
            (OpCode::OP_EQI, im)
        } else if self.exp2RK(ls, e2)? != 0 {
            (OpCode::OP_EQK, e2.info)
        } else {
            (OpCode::OP_EQ, self.luaK_exp2anyreg(ls, e2)?)
        };
        self.freeexps(ls, e1, e2);
        e1.info = self.condjump(
            ls,
            op,
            r1,
            r2,
            isfloat,
            if opr == BinOpr::OPR_EQ { 1 } else { 0 },
        )?;
        e1.k = ExpKind::VJMP;
        Ok(())
    }

    pub fn luaK_prefix(
        &mut self,
        ls: &mut LexState,
        opr: UnOpr,
        e: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        let mut ef = ExpDesc::new(ExpKind::VKINT, 0);
        ef.ival = 0;
        self.luaK_dischargevars(ls, e)?;
        match opr {
            UnOpr::OPR_MINUS | UnOpr::OPR_BNOT => {
                if self.constfolding(ls, (opr as i32) + LUA_OPUNM, e, &ef)? {
                    return Ok(());
                }
                self.codeunexpval(ls, Self::unopr2op(opr), e, line)
            }
            UnOpr::OPR_LEN => self.codeunexpval(ls, Self::unopr2op(opr), e, line),
            UnOpr::OPR_NOT => self.codenot(ls, e),
            _ => Ok(()),
        }
    }

    pub fn luaK_infix(
        &mut self,
        ls: &mut LexState,
        op: BinOpr,
        v: &mut ExpDesc,
    ) -> LuaResult<()> {
        self.luaK_dischargevars(ls, v)?;
        match op {
            BinOpr::OPR_AND => {
                self.luaK_goiftrue(ls, v)?;
            }
            BinOpr::OPR_OR => {
                self.luaK_goiffalse(ls, v)?;
            }
            BinOpr::OPR_CONCAT => {
                self.luaK_exp2nextreg(ls, v)?;
            }
            BinOpr::OPR_ADD
            | BinOpr::OPR_SUB
            | BinOpr::OPR_MUL
            | BinOpr::OPR_DIV
            | BinOpr::OPR_IDIV
            | BinOpr::OPR_MOD
            | BinOpr::OPR_POW
            | BinOpr::OPR_BAND
            | BinOpr::OPR_BOR
            | BinOpr::OPR_BXOR
            | BinOpr::OPR_SHL
            | BinOpr::OPR_SHR => {
                if tonumeral(v).is_none() {
                    self.luaK_exp2anyreg(ls, v)?;
                }
            }
            BinOpr::OPR_EQ | BinOpr::OPR_NE => {
                if tonumeral(v).is_none() {
                    self.exp2RK(ls, v)?;
                }
            }
            BinOpr::OPR_LT | BinOpr::OPR_LE | BinOpr::OPR_GT | BinOpr::OPR_GE => {
                let mut d1 = 0;
                let mut d2 = 0;
                if !Self::isSCnumber(v, &mut d1, &mut d2) {
                    self.luaK_exp2anyreg(ls, v)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn codeconcat(
        &mut self,
        ls: &mut LexState,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        if let Some(mut ie2) = self.previousinstruction(ls) {
            if GET_OPCODE(ie2) == OpCode::OP_CONCAT {
                let n = GETARG_B(ie2);
                self.freeexp(ls, e2);
                SETARG_A(&mut ie2, e1.info);
                SETARG_B(&mut ie2, n + 1);
                let prev_pc = ls.fs().pc - 1;
                self.set_inst(ls, prev_pc, ie2);
                return Ok(());
            }
        }
        self.luaK_codeABCk(ls, OpCode::OP_CONCAT, e1.info, 2, 0, 0)?;
        self.freeexp(ls, e2);
        self.luaK_fixline(ls, line);
        Ok(())
    }

    pub fn luaK_posfix(
        &mut self,
        ls: &mut LexState,
        mut opr: BinOpr,
        e1: &mut ExpDesc,
        e2: &mut ExpDesc,
        line: i32,
    ) -> LuaResult<()> {
        self.luaK_dischargevars(ls, e2)?;
        if (opr as u8) <= (BinOpr::OPR_SHR as u8)
            && self.constfolding(ls, (opr as i32) + LUA_OPADD, e1, e2)?
        {
            return Ok(());
        }
        match opr {
            BinOpr::OPR_AND => {
                let e1_f = e1.f;
                self.luaK_concat(ls, &mut e2.f, e1_f)?;
                *e1 = *e2;
            }
            BinOpr::OPR_OR => {
                let e1_t = e1.t;
                self.luaK_concat(ls, &mut e2.t, e1_t)?;
                *e1 = *e2;
            }
            BinOpr::OPR_CONCAT => {
                self.luaK_exp2nextreg(ls, e2)?;
                self.codeconcat(ls, e1, e2, line)?;
            }
            BinOpr::OPR_ADD | BinOpr::OPR_MUL => {
                self.codecommutative(ls, opr, e1, e2, line)?;
            }
            BinOpr::OPR_SUB => {
                if !self.finishbinexpneg(ls, e1, e2, OpCode::OP_ADDI, line, TMS::TM_SUB)? {
                    self.codearith(ls, opr, e1, e2, 0, line)?;
                }
            }
            BinOpr::OPR_DIV | BinOpr::OPR_IDIV | BinOpr::OPR_MOD | BinOpr::OPR_POW => {
                self.codearith(ls, opr, e1, e2, 0, line)?;
            }
            BinOpr::OPR_BAND | BinOpr::OPR_BOR | BinOpr::OPR_BXOR => {
                self.codebitwise(ls, opr, e1, e2, line)?;
            }
            BinOpr::OPR_SHL => {
                if Self::isSCint(e1) {
                    std::mem::swap(e1, e2);
                    self.codebini(ls, OpCode::OP_SHLI, e1, e2, 1, line, TMS::TM_SHL)?;
                } else if self.finishbinexpneg(ls, e1, e2, OpCode::OP_SHRI, line, TMS::TM_SHL)? {
                } else {
                    self.codebinexpval(ls, opr, e1, e2, line)?;
                }
            }
            BinOpr::OPR_SHR => {
                if Self::isSCint(e2) {
                    self.codebini(ls, OpCode::OP_SHRI, e1, e2, 0, line, TMS::TM_SHR)?;
                } else {
                    self.codebinexpval(ls, opr, e1, e2, line)?;
                }
            }
            BinOpr::OPR_EQ | BinOpr::OPR_NE => {
                self.codeeq(ls, opr, e1, e2)?;
            }
            BinOpr::OPR_GT | BinOpr::OPR_GE => {
                std::mem::swap(e1, e2);
                opr = if opr == BinOpr::OPR_GT {
                    BinOpr::OPR_LT
                } else {
                    BinOpr::OPR_LE
                };
                self.codeorder(ls, opr, e1, e2)?;
            }
            BinOpr::OPR_LT | BinOpr::OPR_LE => {
                self.codeorder(ls, opr, e1, e2)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn luaK_settablesize(
        &mut self,
        ls: &LexState,
        pc: i32,
        ra: i32,
        asize: i32,
        hsize: i32,
    ) {
        let rb = if hsize != 0 {
            luaO_ceillog2(hsize as u32) + 1
        } else {
            0
        };
        let extra = asize / (MAXARG_C + 1);
        let rc = asize % (MAXARG_C + 1);
        let k = if extra > 0 { 1 } else { 0 };
        self.set_inst(ls, pc, CREATE_ABCk(OpCode::OP_NEWTABLE, ra, rb, rc, k));
        self.set_inst(ls, pc + 1, CREATE_Ax(OpCode::OP_EXTRAARG, extra));
    }

    pub fn luaK_setlist(
        &mut self,
        ls: &mut LexState,
        base: i32,
        mut nelems: i32,
        mut tostore: i32,
    ) -> LuaResult<()> {
        if tostore == LUA_MULTRET {
            tostore = 0;
        }
        if nelems <= MAXARG_C {
            self.luaK_codeABCk(ls, OpCode::OP_SETLIST, base, tostore, nelems, 0)?;
        } else {
            let extra = nelems / (MAXARG_C + 1);
            nelems %= MAXARG_C + 1;
            self.luaK_codeABCk(ls, OpCode::OP_SETLIST, base, tostore, nelems, 1)?;
            self.codeextraarg(ls, extra)?;
        }
        ls.fs_mut().freereg = (base + 1) as u8;
        Ok(())
    }

    fn finaltarget(&self, ls: &LexState, mut i: i32) -> i32 {
        for _ in 0..100 {
            let pc = self.get_inst(ls, i);
            if GET_OPCODE(pc) != OpCode::OP_JMP {
                break;
            } else {
                i += GETARG_sJ(pc) + 1;
            }
        }
        i
    }

    pub fn luaK_finish(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let f_id = ls.fs().f;
        let pc_len = ls.fs().pc;
        let needclose = ls.fs().needclose != 0;
        let (is_vararg, numparams) = self
            .protos
            .get(f_id)
            .map_or((false, 0), |p| (p.is_vararg != 0, p.numparams as i32));
        for i in 0..pc_len {
            let mut inst = self.get_inst(ls, i);
            match GET_OPCODE(inst) {
                OpCode::OP_RETURN0 | OpCode::OP_RETURN1 => {
                    if !(needclose || is_vararg) {
                        continue;
                    }
                    SET_OPCODE(&mut inst, OpCode::OP_RETURN);
                    if needclose {
                        SETARG_k(&mut inst, 1);
                    }
                    if is_vararg {
                        SETARG_C(&mut inst, numparams + 1);
                    }
                    self.set_inst(ls, i, inst);
                }
                OpCode::OP_RETURN | OpCode::OP_TAILCALL => {
                    if needclose {
                        SETARG_k(&mut inst, 1);
                    }
                    if is_vararg {
                        SETARG_C(&mut inst, numparams + 1);
                    }
                    self.set_inst(ls, i, inst);
                }
                OpCode::OP_JMP => {
                    let target = self.finaltarget(ls, i);
                    self.fixjump(ls, i, target)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Parser (lparser.c)
// ============================================================================

const UNARY_PRIORITY: u8 = 12;
static PRIORITY: [(u8, u8); 21] = [
    (10, 10),
    (10, 10), // + -
    (11, 11),
    (11, 11), // * %
    (14, 13), // ^
    (11, 11),
    (11, 11), // / //
    (6, 6),
    (4, 4),
    (5, 5), // & | ~
    (7, 7),
    (7, 7),   // << >>
    (9, 8),   // ..
    (3, 3),
    (3, 3),
    (3, 3), // == < <=
    (3, 3),
    (3, 3),
    (3, 3), // ~= > >=
    (2, 2),
    (1, 1), // and or
];

pub struct ConsControl {
    pub v: ExpDesc,
    pub t_info: i32,
    pub nh: i32,
    pub na: i32,
    pub tostore: i32,
}

impl LuaVM {
    fn error_expected<T>(&mut self, ls: &LexState, token: i32) -> LuaResult<T> {
        let tstr = self.luaX_token2str(token);
        self.luaX_syntaxerror(ls, &format!("{tstr} expected"))
    }

    fn errorlimit<T>(&mut self, ls: &LexState, limit: i32, what: &str) -> LuaResult<T> {
        let f_id = ls.fs().f;
        let line = self.protos.get(f_id).map_or(0, |p| p.linedefined);
        let where_str = if line == 0 {
            "main function".to_string()
        } else {
            format!("function at line {line}")
        };
        let msg = format!("too many {what} (limit is {limit}) in {where_str}");
        self.luaX_syntaxerror(ls, &msg)
    }

    fn checklimit(&mut self, ls: &LexState, v: i32, l: i32, what: &str) -> LuaResult<()> {
        if v > l {
            return self.errorlimit(ls, l, what);
        }
        Ok(())
    }

    fn testnext(&mut self, ls: &mut LexState, c: i32) -> LuaResult<bool> {
        if ls.t.token == c {
            self.luaX_next(ls)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check(&mut self, ls: &LexState, c: i32) -> LuaResult<()> {
        if ls.t.token != c {
            return self.error_expected(ls, c);
        }
        Ok(())
    }

    fn checknext(&mut self, ls: &mut LexState, c: i32) -> LuaResult<()> {
        self.check(ls, c)?;
        self.luaX_next(ls)
    }

    fn check_match(
        &mut self,
        ls: &mut LexState,
        what: i32,
        who: i32,
        where_line: i32,
    ) -> LuaResult<()> {
        if !self.testnext(ls, what)? {
            if where_line == ls.linenumber {
                return self.error_expected(ls, what);
            } else {
                let what_s = self.luaX_token2str(what);
                let who_s = self.luaX_token2str(who);
                return self.luaX_syntaxerror(
                    ls,
                    &format!("{what_s} expected (to close {who_s} at line {where_line})"),
                );
            }
        }
        Ok(())
    }

    fn str_checkname(&mut self, ls: &mut LexState) -> LuaResult<StrId> {
        self.check(ls, TK_NAME)?;
        let ts = ls.t.ts;
        self.luaX_next(ls)?;
        Ok(ts)
    }

    fn codestring(e: &mut ExpDesc, s: StrId) {
        e.f = NO_JUMP;
        e.t = NO_JUMP;
        e.k = ExpKind::VKSTR;
        e.strval = s;
    }

    fn codename(&mut self, ls: &mut LexState, e: &mut ExpDesc) -> LuaResult<()> {
        let s = self.str_checkname(ls)?;
        Self::codestring(e, s);
        Ok(())
    }

    fn registerlocalvar(&mut self, ls: &mut LexState, varname: Option<StrId>) -> i16 {
        let f_id = ls.fs().f;
        let pc = ls.fs().pc;
        let ndebug = ls.fs().ndebugvars;
        if let Some(p) = self.protos.get_mut(f_id) {
            if (ndebug as usize) < p.locvars.len() {
                if let Some(slot) = p.locvars.get_mut(ndebug as usize) {
                    slot.varname = varname;
                    slot.startpc = pc;
                    slot.endpc = 0;
                }
            } else {
                p.locvars.push(LocVar {
                    varname,
                    startpc: pc,
                    endpc: 0,
                });
            }
        }
        ls.fs_mut().ndebugvars += 1;
        ndebug
    }

    fn new_localvar(&mut self, ls: &mut LexState, name: StrId) -> LuaResult<usize> {
        let firstlocal = ls.fs().firstlocal;
        let cur_n = ls.dyd.actvar.len();
        self.checklimit(
            ls,
            (cur_n + 1 - firstlocal) as i32,
            MAXVARS as i32,
            "local variables",
        )?;
        ls.dyd.actvar.push(Vardesc {
            kind: VDKREG,
            ridx: 0,
            pidx: -1,
            name: Some(name),
            k: TValue::nil(),
        });
        Ok(cur_n - firstlocal)
    }

    fn new_localvarliteral(&mut self, ls: &mut LexState, lit: &[u8]) -> LuaResult<usize> {
        let ts = self.luaX_newstring(ls, lit)?;
        self.new_localvar(ls, ts)
    }

    fn getlocalvardesc(ls: &LexState, fs_idx: usize, vidx: usize) -> Vardesc {
        let firstlocal = ls.fs_stack.get(fs_idx).map_or(0, |fs| fs.firstlocal);
        ls.dyd
            .actvar
            .get(firstlocal + vidx)
            .copied()
            .unwrap_or(Vardesc {
                kind: VDKREG,
                ridx: 0,
                pidx: -1,
                name: None,
                k: TValue::nil(),
            })
    }

    fn getlocalvardesc_mut<'a>(ls: &'a mut LexState, vidx: usize) -> Option<&'a mut Vardesc> {
        let firstlocal = ls.fs().firstlocal;
        ls.dyd.actvar.get_mut(firstlocal + vidx)
    }

    fn reglevel(ls: &LexState, mut nvar: i32) -> i32 {
        let fs_idx = ls.fs_stack.len() - 1;
        while nvar > 0 {
            nvar -= 1;
            let vd = Self::getlocalvardesc(ls, fs_idx, nvar as usize);
            if vd.kind != RDKCTC {
                return (vd.ridx as i32) + 1;
            }
        }
        0
    }

    pub fn luaY_nvarstack(ls: &LexState) -> i32 {
        Self::reglevel(ls, ls.fs().nactvar as i32)
    }

    fn init_var(ls: &LexState, fs_idx: usize, e: &mut ExpDesc, vidx: usize) {
        e.f = NO_JUMP;
        e.t = NO_JUMP;
        e.k = ExpKind::VLOCAL;
        e.var_vidx = vidx as u16;
        e.var_ridx = Self::getlocalvardesc(ls, fs_idx, vidx).ridx;
    }

    fn check_readonly(&mut self, ls: &mut LexState, e: &ExpDesc) -> LuaResult<()> {
        let fs_idx = ls.fs_stack.len() - 1;
        let varname: Option<StrId> = match e.k {
            ExpKind::VCONST => ls.dyd.actvar.get(e.info as usize).and_then(|v| v.name),
            ExpKind::VLOCAL => {
                let vd = Self::getlocalvardesc(ls, fs_idx, e.var_vidx as usize);
                if vd.kind != VDKREG {
                    vd.name
                } else {
                    None
                }
            }
            ExpKind::VUPVAL => {
                let f_id = ls.fs().f;
                self.protos
                    .get(f_id)
                    .and_then(|p| p.upvalues.get(e.info as usize))
                    .and_then(|up| if up.kind != VDKREG { up.name } else { None })
            }
            _ => None,
        };
        if let Some(vn) = varname {
            let msg = format!(
                "attempt to assign to const variable '{}'",
                self.get_str_lossy(vn)
            );
            return self.luaK_semerror(ls, &msg);
        }
        Ok(())
    }

    fn adjustlocalvars(&mut self, ls: &mut LexState, nvars: i32) {
        let mut reglevel = Self::luaY_nvarstack(ls) as u8;
        for _ in 0..nvars {
            let vidx = ls.fs().nactvar as usize;
            ls.fs_mut().nactvar += 1;
            let name = Self::getlocalvardesc(ls, ls.fs_stack.len() - 1, vidx).name;
            let pidx = self.registerlocalvar(ls, name);
            if let Some(vd) = Self::getlocalvardesc_mut(ls, vidx) {
                vd.ridx = reglevel;
                vd.pidx = pidx;
            }
            reglevel += 1;
        }
    }

    fn removevars(&mut self, ls: &mut LexState, tolevel: u8) {
        let nactvar = ls.fs().nactvar;
        let f_id = ls.fs().f;
        let pc = ls.fs().pc;
        let diff = (nactvar - tolevel) as usize;
        let new_len = ls.dyd.actvar.len().saturating_sub(diff);
        while ls.fs().nactvar > tolevel {
            ls.fs_mut().nactvar -= 1;
            let vidx = ls.fs().nactvar as usize;
            let vd = Self::getlocalvardesc(ls, ls.fs_stack.len() - 1, vidx);
            if vd.kind != RDKCTC && vd.pidx >= 0 {
                if let Some(p) = self.protos.get_mut(f_id) {
                    if let Some(lv) = p.locvars.get_mut(vd.pidx as usize) {
                        lv.endpc = pc;
                    }
                }
            }
        }
        ls.dyd.actvar.truncate(new_len);
    }

    fn searchupvalue(&self, ls: &LexState, fs_idx: usize, name: StrId) -> i32 {
        let f_id = ls.fs_stack.get(fs_idx).map_or(0, |fs| fs.f);
        let nups = ls.fs_stack.get(fs_idx).map_or(0, |fs| fs.nups as usize);
        if let Some(p) = self.protos.get(f_id) {
            for i in 0..nups {
                if let Some(up) = p.upvalues.get(i) {
                    if up.name == Some(name) {
                        return i as i32;
                    }
                }
            }
        }
        -1
    }

    fn allocupvalue(&mut self, ls: &mut LexState, fs_idx: usize) -> LuaResult<usize> {
        let (f_id, nups) = ls
            .fs_stack
            .get(fs_idx)
            .map_or((0, 0), |fs| (fs.f, fs.nups as i32));
        if nups + 1 > (MAXUPVAL as i32) {
            return self.errorlimit(ls, MAXUPVAL as i32, "upvalues");
        }
        if let Some(p) = self.protos.get_mut(f_id) {
            p.upvalues.push(Upvaldesc {
                name: None,
                instack: 0,
                idx: 0,
                kind: VDKREG,
            });
        }
        if let Some(fs) = ls.fs_stack.get_mut(fs_idx) {
            fs.nups += 1;
        }
        Ok(nups as usize)
    }

    fn newupvalue(
        &mut self,
        ls: &mut LexState,
        fs_idx: usize,
        name: StrId,
        v: &ExpDesc,
    ) -> LuaResult<i32> {
        let prev_idx = fs_idx - 1;
        let (instack, idx, kind) = if v.k == ExpKind::VLOCAL {
            let vd = Self::getlocalvardesc(ls, prev_idx, v.var_vidx as usize);
            (1u8, v.var_ridx, vd.kind)
        } else {
            let prev_f = ls.fs_stack.get(prev_idx).map_or(0, |fs| fs.f);
            let k = self
                .protos
                .get(prev_f)
                .and_then(|p| p.upvalues.get(v.info as usize))
                .map_or(VDKREG, |u| u.kind);
            (0u8, v.info as u8, k)
        };
        let up_idx = self.allocupvalue(ls, fs_idx)?;
        let f_id = ls.fs_stack.get(fs_idx).map_or(0, |fs| fs.f);
        if let Some(p) = self.protos.get_mut(f_id) {
            if let Some(up) = p.upvalues.get_mut(up_idx) {
                up.instack = instack;
                up.idx = idx;
                up.kind = kind;
                up.name = Some(name);
            }
        }
        Ok(up_idx as i32)
    }

    fn searchvar(ls: &LexState, fs_idx: usize, n: StrId, var: &mut ExpDesc) -> i32 {
        let (nactvar, firstlocal) = ls
            .fs_stack
            .get(fs_idx)
            .map_or((0, 0), |fs| (fs.nactvar as isize, fs.firstlocal));
        let mut i = nactvar - 1;
        while i >= 0 {
            let vd = Self::getlocalvardesc(ls, fs_idx, i as usize);
            if vd.name == Some(n) {
                if vd.kind == RDKCTC {
                    *var = ExpDesc::new(ExpKind::VCONST, (firstlocal + (i as usize)) as i32);
                } else {
                    Self::init_var(ls, fs_idx, var, i as usize);
                }
                return var.k as i32;
            }
            i -= 1;
        }
        -1
    }

    fn markupval(ls: &mut LexState, fs_idx: usize, level: u8) {
        if let Some(fs) = ls.fs_stack.get_mut(fs_idx) {
            for bl in fs.bl_stack.iter_mut().rev() {
                if bl.nactvar <= level {
                    bl.upval = 1;
                    break;
                }
            }
            fs.needclose = 1;
        }
    }

    fn marktobeclosed(ls: &mut LexState) {
        let fs = ls.fs_mut();
        if let Some(bl) = fs.bl_stack.last_mut() {
            bl.upval = 1;
            bl.insidetbc = 1;
        }
        fs.needclose = 1;
    }

    fn singlevaraux(
        &mut self,
        ls: &mut LexState,
        fs_idx_opt: Option<usize>,
        n: StrId,
        var: &mut ExpDesc,
        base: bool,
    ) -> LuaResult<()> {
        let Some(fs_idx) = fs_idx_opt else {
            *var = ExpDesc::new(ExpKind::VVOID, 0);
            return Ok(());
        };
        let v = Self::searchvar(ls, fs_idx, n, var);
        if v >= 0 {
            if v == (ExpKind::VLOCAL as i32) && !base {
                Self::markupval(ls, fs_idx, var.var_vidx as u8);
            }
        } else {
            let mut idx = self.searchupvalue(ls, fs_idx, n);
            if idx < 0 {
                let prev_opt = if fs_idx > 0 { Some(fs_idx - 1) } else { None };
                self.singlevaraux(ls, prev_opt, n, var, false)?;
                if var.k == ExpKind::VLOCAL || var.k == ExpKind::VUPVAL {
                    idx = self.newupvalue(ls, fs_idx, n, var)?;
                } else {
                    return Ok(());
                }
            }
            *var = ExpDesc::new(ExpKind::VUPVAL, idx);
        }
        Ok(())
    }

    fn singlevar(&mut self, ls: &mut LexState, var: &mut ExpDesc) -> LuaResult<()> {
        let varname = self.str_checkname(ls)?;
        let top_fs = ls.fs_stack.len() - 1;
        self.singlevaraux(ls, Some(top_fs), varname, var, true)?;
        if var.k == ExpKind::VVOID {
            let envn = ls.envn;
            self.singlevaraux(ls, Some(top_fs), envn, var, true)?;
            self.luaK_exp2anyregup(ls, var)?;
            let mut key = ExpDesc::new(ExpKind::VVOID, 0);
            Self::codestring(&mut key, varname);
            self.luaK_indexed(ls, var, &mut key)?;
        }
        Ok(())
    }

    fn adjust_assign(
        &mut self,
        ls: &mut LexState,
        nvars: i32,
        nexps: i32,
        e: &mut ExpDesc,
    ) -> LuaResult<()> {
        let needed = nvars - nexps;
        if e.k == ExpKind::VCALL || e.k == ExpKind::VVARARG {
            let mut extra = needed + 1;
            if extra < 0 {
                extra = 0;
            }
            self.luaK_setreturns(ls, e, extra)?;
        } else {
            if e.k != ExpKind::VVOID {
                self.luaK_exp2nextreg(ls, e)?;
            }
            if needed > 0 {
                let fr = ls.fs().freereg as i32;
                self.luaK_nil(ls, fr, needed)?;
            }
        }
        if needed > 0 {
            self.luaK_reserveregs(ls, needed)?;
        } else {
            let fr = ls.fs().freereg as i32;
            ls.fs_mut().freereg = (fr + needed) as u8;
        }
        Ok(())
    }

    fn jumpscopeerror<T>(&mut self, ls: &mut LexState, gt: &Labeldesc) -> LuaResult<T> {
        let fs_idx = ls.fs_stack.len() - 1;
        let vd = Self::getlocalvardesc(ls, fs_idx, gt.nactvar as usize);
        let varname = vd
            .name
            .map_or_else(|| "?".to_string(), |s| self.get_str_lossy(s));
        let gt_name = self.get_str_lossy(gt.name);
        let line = gt.line;
        let msg = format!("<goto {gt_name}> at line {line} jumps into the scope of local '{varname}'");
        self.luaK_semerror(ls, &msg)
    }

    fn solvegoto(&mut self, ls: &mut LexState, g: usize, label: &Labeldesc) -> LuaResult<()> {
        let gt = match ls.dyd.gt.get(g).copied() {
            Some(x) => x,
            None => return Ok(()),
        };
        if gt.nactvar < label.nactvar {
            return self.jumpscopeerror(ls, &gt);
        }
        self.luaK_patchlist(ls, gt.pc, label.pc)?;
        ls.dyd.gt.remove(g);
        Ok(())
    }

    fn findlabel(ls: &LexState, name: StrId) -> Option<Labeldesc> {
        let firstlabel = ls.fs().firstlabel;
        for i in firstlabel..ls.dyd.label.len() {
            if let Some(lb) = ls.dyd.label.get(i) {
                if lb.name == name {
                    return Some(*lb);
                }
            }
        }
        None
    }

    fn newgotoentry(&mut self, ls: &mut LexState, name: StrId, line: i32, pc: i32) -> usize {
        let nactvar = ls.fs().nactvar;
        let n = ls.dyd.gt.len();
        ls.dyd.gt.push(Labeldesc {
            name,
            pc,
            line,
            nactvar,
            close: 0,
        });
        n
    }

    fn solvegotos(&mut self, ls: &mut LexState, lb: &Labeldesc) -> LuaResult<bool> {
        let mut i = ls
            .fs()
            .bl_stack
            .last()
            .map_or(0, |bl| bl.firstgoto);
        let mut needsclose = false;
        while i < ls.dyd.gt.len() {
            let gt = match ls.dyd.gt.get(i).copied() {
                Some(x) => x,
                None => break,
            };
            if gt.name == lb.name {
                needsclose |= gt.close != 0;
                self.solvegoto(ls, i, lb)?;
            } else {
                i += 1;
            }
        }
        Ok(needsclose)
    }

    fn createlabel(
        &mut self,
        ls: &mut LexState,
        name: StrId,
        line: i32,
        last: bool,
    ) -> LuaResult<bool> {
        let pc = self.luaK_getlabel(ls);
        let mut nactvar = ls.fs().nactvar;
        if last {
            if let Some(bl) = ls.fs().bl_stack.last() {
                nactvar = bl.nactvar;
            }
        }
        let lb = Labeldesc {
            name,
            pc,
            line,
            nactvar,
            close: 0,
        };
        ls.dyd.label.push(lb);
        if self.solvegotos(ls, &lb)? {
            let nvs = Self::luaY_nvarstack(ls);
            self.luaK_codeABCk(ls, OpCode::OP_CLOSE, nvs, 0, 0, 0)?;
            return Ok(true);
        }
        Ok(false)
    }

    fn movegotosout(ls: &mut LexState, bl: &BlockCnt) {
        let bl_reglevel = Self::reglevel(ls, bl.nactvar as i32);
        for i in bl.firstgoto..ls.dyd.gt.len() {
            let gt_nactvar = ls.dyd.gt.get(i).map_or(0, |g| g.nactvar);
            let gt_reglevel = Self::reglevel(ls, gt_nactvar as i32);
            if let Some(gt) = ls.dyd.gt.get_mut(i) {
                if gt_reglevel > bl_reglevel {
                    gt.close |= bl.upval;
                }
                gt.nactvar = bl.nactvar;
            }
        }
    }

    fn enterblock(ls: &mut LexState, isloop: u8) {
        let nactvar = ls.fs().nactvar;
        let firstlabel = ls.dyd.label.len();
        let firstgoto = ls.dyd.gt.len();
        let insidetbc = ls
            .fs()
            .bl_stack
            .last()
            .map_or(0, |b| b.insidetbc);
        ls.fs_mut().bl_stack.push(BlockCnt {
            firstlabel,
            firstgoto,
            nactvar,
            upval: 0,
            isloop,
            insidetbc,
        });
    }

    fn undefgoto<T>(&mut self, ls: &mut LexState, gt: &Labeldesc) -> LuaResult<T> {
        let msg = if gt.name == ls.brkn {
            format!("break outside loop at line {}", gt.line)
        } else {
            format!(
                "no visible label '{}' for <goto> at line {}",
                self.get_str_lossy(gt.name),
                gt.line
            )
        };
        self.luaK_semerror(ls, &msg)
    }

    fn leaveblock(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let bl = match ls.fs().bl_stack.last().copied() {
            Some(b) => b,
            None => return Ok(()),
        };
        let has_prev = ls.fs().bl_stack.len() > 1;
        let stklevel = Self::reglevel(ls, bl.nactvar as i32);
        self.removevars(ls, bl.nactvar);
        let mut hasclose = false;
        if bl.isloop != 0 {
            let brkn = ls.brkn;
            hasclose = self.createlabel(ls, brkn, 0, false)?;
        }
        if !hasclose && has_prev && bl.upval != 0 {
            self.luaK_codeABCk(ls, OpCode::OP_CLOSE, stklevel, 0, 0, 0)?;
        }
        ls.fs_mut().freereg = stklevel as u8;
        ls.dyd.label.truncate(bl.firstlabel);
        ls.fs_mut().bl_stack.pop();
        if has_prev {
            Self::movegotosout(ls, &bl);
        } else if bl.firstgoto < ls.dyd.gt.len() {
            if let Some(gt) = ls.dyd.gt.get(bl.firstgoto).copied() {
                return self.undefgoto(ls, &gt);
            }
        }
        Ok(())
    }

    fn addprototype(&mut self, ls: &mut LexState) -> ProtoId {
        let f_id = ls.fs().f;
        let clp = self.protos.len();
        self.protos.push(Proto::new());
        if let Some(p) = self.protos.get_mut(f_id) {
            p.p.push(clp);
        }
        ls.fs_mut().np += 1;
        clp
    }

    fn codeclosure(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        let bx = (ls.fs().np - 1) as u32;
        let pc = self.luaK_codeABx(ls, OpCode::OP_CLOSURE, 0, bx)?;
        *v = ExpDesc::new(ExpKind::VRELOC, pc);
        self.luaK_exp2nextreg(ls, v)
    }

    fn open_func(&mut self, ls: &mut LexState, f_id: ProtoId, linedefined: i32) {
        let src = ls.source;
        if let Some(p) = self.protos.get_mut(f_id) {
            p.source = Some(src);
            p.maxstacksize = 2;
            p.linedefined = linedefined;
        }
        let firstlocal = ls.dyd.actvar.len();
        let firstlabel = ls.dyd.label.len();
        let fs = FuncState::new(f_id, linedefined, firstlocal, firstlabel);
        ls.fs_stack.push(fs);
        Self::enterblock(ls, 0);
    }

    fn close_func(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let nvs = Self::luaY_nvarstack(ls);
        self.luaK_ret(ls, nvs, 0)?;
        self.leaveblock(ls)?;
        self.luaK_finish(ls)?;
        ls.fs_stack.pop();
        Ok(())
    }

    fn block_follow(ls: &LexState, withuntil: bool) -> bool {
        match ls.t.token {
            TK_ELSE | TK_ELSEIF | TK_END | TK_EOS => true,
            TK_UNTIL => withuntil,
            _ => false,
        }
    }

    fn statlist(&mut self, ls: &mut LexState) -> LuaResult<()> {
        while !Self::block_follow(ls, true) {
            if ls.t.token == TK_RETURN {
                self.statement(ls)?;
                return Ok(());
            }
            self.statement(ls)?;
        }
        Ok(())
    }

    fn fieldsel(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        self.luaK_exp2anyregup(ls, v)?;
        self.luaX_next(ls)?;
        let mut key = ExpDesc::new(ExpKind::VVOID, 0);
        self.codename(ls, &mut key)?;
        self.luaK_indexed(ls, v, &mut key)
    }

    fn yindex(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        self.luaX_next(ls)?;
        self.expr(ls, v)?;
        self.luaK_exp2val(ls, v)?;
        self.checknext(ls, b']' as i32)
    }

    fn recfield(&mut self, ls: &mut LexState, cc: &mut ConsControl) -> LuaResult<()> {
        let reg = ls.fs().freereg;
        let mut key = ExpDesc::new(ExpKind::VVOID, 0);
        let mut val = ExpDesc::new(ExpKind::VVOID, 0);
        if ls.t.token == TK_NAME {
            self.checklimit(ls, cc.nh, i32::MAX, "items in a constructor")?;
            self.codename(ls, &mut key)?;
        } else {
            self.yindex(ls, &mut key)?;
        }
        cc.nh += 1;
        self.checknext(ls, b'=' as i32)?;
        let mut tab = ExpDesc::new(ExpKind::VNONRELOC, cc.t_info);
        self.luaK_indexed(ls, &mut tab, &mut key)?;
        self.expr(ls, &mut val)?;
        self.luaK_storevar(ls, &tab, &mut val)?;
        ls.fs_mut().freereg = reg;
        Ok(())
    }

    fn closelistfield(&mut self, ls: &mut LexState, cc: &mut ConsControl) -> LuaResult<()> {
        if cc.v.k == ExpKind::VVOID {
            return Ok(());
        }
        self.luaK_exp2nextreg(ls, &mut cc.v)?;
        cc.v.k = ExpKind::VVOID;
        if cc.tostore == LFIELDS_PER_FLUSH {
            self.luaK_setlist(ls, cc.t_info, cc.na, cc.tostore)?;
            cc.na += cc.tostore;
            cc.tostore = 0;
        }
        Ok(())
    }

    fn lastlistfield(&mut self, ls: &mut LexState, cc: &mut ConsControl) -> LuaResult<()> {
        if cc.tostore == 0 {
            return Ok(());
        }
        if cc.v.k == ExpKind::VCALL || cc.v.k == ExpKind::VVARARG {
            self.luaK_setreturns(ls, &cc.v, LUA_MULTRET)?;
            self.luaK_setlist(ls, cc.t_info, cc.na, LUA_MULTRET)?;
            cc.na -= 1;
        } else {
            if cc.v.k != ExpKind::VVOID {
                self.luaK_exp2nextreg(ls, &mut cc.v)?;
            }
            self.luaK_setlist(ls, cc.t_info, cc.na, cc.tostore)?;
        }
        cc.na += cc.tostore;
        Ok(())
    }

    fn listfield(&mut self, ls: &mut LexState, cc: &mut ConsControl) -> LuaResult<()> {
        self.expr(ls, &mut cc.v)?;
        cc.tostore += 1;
        Ok(())
    }

    fn field(&mut self, ls: &mut LexState, cc: &mut ConsControl) -> LuaResult<()> {
        match ls.t.token {
            TK_NAME => {
                if self.luaX_lookahead(ls)? != (b'=' as i32) {
                    self.listfield(ls, cc)
                } else {
                    self.recfield(ls, cc)
                }
            }
            c if c == (b'[' as i32) => self.recfield(ls, cc),
            _ => self.listfield(ls, cc),
        }
    }

    fn constructor(&mut self, ls: &mut LexState, t: &mut ExpDesc) -> LuaResult<()> {
        let line = ls.linenumber;
        let pc = self.luaK_codeABCk(ls, OpCode::OP_NEWTABLE, 0, 0, 0, 0)?;
        self.luaK_code(ls, 0)?;
        let freereg = ls.fs().freereg as i32;
        *t = ExpDesc::new(ExpKind::VNONRELOC, freereg);
        self.luaK_reserveregs(ls, 1)?;
        let mut cc = ConsControl {
            v: ExpDesc::new(ExpKind::VVOID, 0),
            t_info: t.info,
            nh: 0,
            na: 0,
            tostore: 0,
        };
        self.checknext(ls, b'{' as i32)?;
        loop {
            if ls.t.token == (b'}' as i32) {
                break;
            }
            self.closelistfield(ls, &mut cc)?;
            self.field(ls, &mut cc)?;
            if !(self.testnext(ls, b',' as i32)? || self.testnext(ls, b';' as i32)?) {
                break;
            }
        }
        self.check_match(ls, b'}' as i32, b'{' as i32, line)?;
        self.lastlistfield(ls, &mut cc)?;
        self.luaK_settablesize(ls, pc, t.info, cc.na, cc.nh);
        Ok(())
    }

    fn setvararg(&mut self, ls: &mut LexState, nparams: i32) -> LuaResult<()> {
        let f_id = ls.fs().f;
        if let Some(p) = self.protos.get_mut(f_id) {
            p.is_vararg = 1;
        }
        self.luaK_codeABCk(ls, OpCode::OP_VARARGPREP, nparams, 0, 0, 0)?;
        Ok(())
    }

    fn parlist(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let mut nparams: i32 = 0;
        let mut isvararg = false;
        if ls.t.token != (b')' as i32) {
            loop {
                match ls.t.token {
                    TK_NAME => {
                        let nm = self.str_checkname(ls)?;
                        self.new_localvar(ls, nm)?;
                        nparams += 1;
                    }
                    TK_DOTS => {
                        self.luaX_next(ls)?;
                        isvararg = true;
                    }
                    _ => return self.luaX_syntaxerror(ls, "<name> or '...' expected"),
                }
                if isvararg || !self.testnext(ls, b',' as i32)? {
                    break;
                }
            }
        }
        self.adjustlocalvars(ls, nparams);
        let nactvar = ls.fs().nactvar;
        let f_id = ls.fs().f;
        if let Some(p) = self.protos.get_mut(f_id) {
            p.numparams = nactvar;
        }
        if isvararg {
            self.setvararg(ls, nactvar as i32)?;
        }
        self.luaK_reserveregs(ls, nactvar as i32)
    }

    fn body(
        &mut self,
        ls: &mut LexState,
        e: &mut ExpDesc,
        ismethod: bool,
        line: i32,
    ) -> LuaResult<()> {
        let new_f = self.addprototype(ls);
        self.open_func(ls, new_f, line);
        self.checknext(ls, b'(' as i32)?;
        if ismethod {
            self.new_localvarliteral(ls, b"self")?;
            self.adjustlocalvars(ls, 1);
        }
        self.parlist(ls)?;
        self.checknext(ls, b')' as i32)?;
        self.statlist(ls)?;
        let lastline = ls.linenumber;
        if let Some(p) = self.protos.get_mut(new_f) {
            p.lastlinedefined = lastline;
        }
        self.check_match(ls, TK_END, TK_FUNCTION, line)?;
        self.close_func(ls)?;
        self.codeclosure(ls, e)
    }

    fn explist(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<i32> {
        let mut n: i32 = 1;
        self.expr(ls, v)?;
        while self.testnext(ls, b',' as i32)? {
            self.luaK_exp2nextreg(ls, v)?;
            self.expr(ls, v)?;
            n += 1;
        }
        Ok(n)
    }

    fn funcargs(&mut self, ls: &mut LexState, f: &mut ExpDesc) -> LuaResult<()> {
        let mut args = ExpDesc::new(ExpKind::VVOID, 0);
        let line = ls.linenumber;
        match ls.t.token {
            c if c == (b'(' as i32) => {
                self.luaX_next(ls)?;
                if ls.t.token == (b')' as i32) {
                    args.k = ExpKind::VVOID;
                } else {
                    self.explist(ls, &mut args)?;
                    if args.k == ExpKind::VCALL || args.k == ExpKind::VVARARG {
                        self.luaK_setreturns(ls, &args, LUA_MULTRET)?;
                    }
                }
                self.check_match(ls, b')' as i32, b'(' as i32, line)?;
            }
            c if c == (b'{' as i32) => {
                self.constructor(ls, &mut args)?;
            }
            TK_STRING => {
                Self::codestring(&mut args, ls.t.ts);
                self.luaX_next(ls)?;
            }
            _ => {
                return self.luaX_syntaxerror(ls, "function arguments expected");
            }
        }
        let base = f.info;
        let nparams = if args.k == ExpKind::VCALL || args.k == ExpKind::VVARARG {
            LUA_MULTRET
        } else {
            if args.k != ExpKind::VVOID {
                self.luaK_exp2nextreg(ls, &mut args)?;
            }
            (ls.fs().freereg as i32) - (base + 1)
        };
        let pc = self.luaK_codeABCk(ls, OpCode::OP_CALL, base, nparams + 1, 2, 0)?;
        *f = ExpDesc::new(ExpKind::VCALL, pc);
        self.luaK_fixline(ls, line);
        ls.fs_mut().freereg = (base + 1) as u8;
        Ok(())
    }

    fn primaryexp(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        match ls.t.token {
            c if c == (b'(' as i32) => {
                let line = ls.linenumber;
                self.luaX_next(ls)?;
                self.expr(ls, v)?;
                self.check_match(ls, b')' as i32, b'(' as i32, line)?;
                self.luaK_dischargevars(ls, v)
            }
            TK_NAME => self.singlevar(ls, v),
            _ => self.luaX_syntaxerror(ls, "unexpected symbol"),
        }
    }

    fn suffixedexp(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        self.primaryexp(ls, v)?;
        loop {
            match ls.t.token {
                c if c == (b'.' as i32) => {
                    self.fieldsel(ls, v)?;
                }
                c if c == (b'[' as i32) => {
                    let mut key = ExpDesc::new(ExpKind::VVOID, 0);
                    self.luaK_exp2anyregup(ls, v)?;
                    self.yindex(ls, &mut key)?;
                    self.luaK_indexed(ls, v, &mut key)?;
                }
                c if c == (b':' as i32) => {
                    let mut key = ExpDesc::new(ExpKind::VVOID, 0);
                    self.luaX_next(ls)?;
                    self.codename(ls, &mut key)?;
                    self.luaK_self(ls, v, &mut key)?;
                    self.funcargs(ls, v)?;
                }
                c if c == (b'(' as i32) || c == TK_STRING || c == (b'{' as i32) => {
                    self.luaK_exp2nextreg(ls, v)?;
                    self.funcargs(ls, v)?;
                }
                _ => return Ok(()),
            }
        }
    }

    fn simpleexp(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        match ls.t.token {
            TK_FLT => {
                *v = ExpDesc::new(ExpKind::VKFLT, 0);
                v.nval = ls.t.r;
            }
            TK_INT => {
                *v = ExpDesc::new(ExpKind::VKINT, 0);
                v.ival = ls.t.i;
            }
            TK_STRING => {
                Self::codestring(v, ls.t.ts);
            }
            TK_NIL => {
                *v = ExpDesc::new(ExpKind::VNIL, 0);
            }
            TK_TRUE => {
                *v = ExpDesc::new(ExpKind::VTRUE, 0);
            }
            TK_FALSE => {
                *v = ExpDesc::new(ExpKind::VFALSE, 0);
            }
            TK_DOTS => {
                let f_id = ls.fs().f;
                let is_va = self.protos.get(f_id).map_or(false, |p| p.is_vararg != 0);
                if !is_va {
                    return self.luaX_syntaxerror(ls, "cannot use '...' outside a vararg function");
                }
                let pc = self.luaK_codeABCk(ls, OpCode::OP_VARARG, 0, 0, 1, 0)?;
                *v = ExpDesc::new(ExpKind::VVARARG, pc);
            }
            c if c == (b'{' as i32) => {
                return self.constructor(ls, v);
            }
            TK_FUNCTION => {
                self.luaX_next(ls)?;
                let line = ls.linenumber;
                return self.body(ls, v, false, line);
            }
            _ => {
                return self.suffixedexp(ls, v);
            }
        }
        self.luaX_next(ls)
    }

    fn getunopr(op: i32) -> UnOpr {
        match op {
            TK_NOT => UnOpr::OPR_NOT,
            c if c == (b'-' as i32) => UnOpr::OPR_MINUS,
            c if c == (b'~' as i32) => UnOpr::OPR_BNOT,
            c if c == (b'#' as i32) => UnOpr::OPR_LEN,
            _ => UnOpr::OPR_NOUNOPR,
        }
    }

    fn getbinopr(op: i32) -> BinOpr {
        match op {
            c if c == (b'+' as i32) => BinOpr::OPR_ADD,
            c if c == (b'-' as i32) => BinOpr::OPR_SUB,
            c if c == (b'*' as i32) => BinOpr::OPR_MUL,
            c if c == (b'%' as i32) => BinOpr::OPR_MOD,
            c if c == (b'^' as i32) => BinOpr::OPR_POW,
            c if c == (b'/' as i32) => BinOpr::OPR_DIV,
            TK_IDIV => BinOpr::OPR_IDIV,
            c if c == (b'&' as i32) => BinOpr::OPR_BAND,
            c if c == (b'|' as i32) => BinOpr::OPR_BOR,
            c if c == (b'~' as i32) => BinOpr::OPR_BXOR,
            TK_SHL => BinOpr::OPR_SHL,
            TK_SHR => BinOpr::OPR_SHR,
            TK_CONCAT => BinOpr::OPR_CONCAT,
            TK_NE => BinOpr::OPR_NE,
            TK_EQ => BinOpr::OPR_EQ,
            c if c == (b'<' as i32) => BinOpr::OPR_LT,
            TK_LE => BinOpr::OPR_LE,
            c if c == (b'>' as i32) => BinOpr::OPR_GT,
            TK_GE => BinOpr::OPR_GE,
            TK_AND => BinOpr::OPR_AND,
            TK_OR => BinOpr::OPR_OR,
            _ => BinOpr::OPR_NOBINOPR,
        }
    }

    fn subexpr(&mut self, ls: &mut LexState, v: &mut ExpDesc, limit: u8) -> LuaResult<BinOpr> {
        self.luaE_incCstack(ls.l)?;
        let uop = Self::getunopr(ls.t.token);
        if uop != UnOpr::OPR_NOUNOPR {
            let line = ls.linenumber;
            self.luaX_next(ls)?;
            self.subexpr(ls, v, UNARY_PRIORITY)?;
            self.luaK_prefix(ls, uop, v, line)?;
        } else {
            self.simpleexp(ls, v)?;
        }
        let mut op = Self::getbinopr(ls.t.token);
        while op != BinOpr::OPR_NOBINOPR
            && PRIORITY.get(op as usize).map_or(0, |p| p.0) > limit
        {
            let mut v2 = ExpDesc::new(ExpKind::VVOID, 0);
            let line = ls.linenumber;
            self.luaX_next(ls)?;
            self.luaK_infix(ls, op, v)?;
            let right_prio = PRIORITY.get(op as usize).map_or(0, |p| p.1);
            let nextop = self.subexpr(ls, &mut v2, right_prio)?;
            self.luaK_posfix(ls, op, v, &mut v2, line)?;
            op = nextop;
        }
        if let Some(th) = self.threads.get_mut(ls.l) {
            th.nCcalls = th.nCcalls.wrapping_sub(1);
        }
        Ok(op)
    }

    fn expr(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<()> {
        self.subexpr(ls, v, 0)?;
        Ok(())
    }

    fn block(&mut self, ls: &mut LexState) -> LuaResult<()> {
        Self::enterblock(ls, 0);
        self.statlist(ls)?;
        self.leaveblock(ls)
    }

    fn check_conflict(
        &mut self,
        ls: &mut LexState,
        lh_list: &mut [ExpDesc],
        v: &ExpDesc,
    ) -> LuaResult<()> {
        let extra = ls.fs().freereg;
        let mut conflict = false;
        for lh in lh_list.iter_mut() {
            if vkisindexed(lh.k) {
                if lh.k == ExpKind::VINDEXUP {
                    if v.k == ExpKind::VUPVAL && (lh.ind_t as i32) == v.info {
                        conflict = true;
                        lh.k = ExpKind::VINDEXSTR;
                        lh.ind_t = extra;
                    }
                } else {
                    if v.k == ExpKind::VLOCAL && lh.ind_t == v.var_ridx {
                        conflict = true;
                        lh.ind_t = extra;
                    }
                    if lh.k == ExpKind::VINDEXED
                        && v.k == ExpKind::VLOCAL
                        && lh.ind_idx == (v.var_ridx as i16)
                    {
                        conflict = true;
                        lh.ind_idx = extra as i16;
                    }
                }
            }
        }
        if conflict {
            if v.k == ExpKind::VLOCAL {
                self.luaK_codeABCk(
                    ls,
                    OpCode::OP_MOVE,
                    extra as i32,
                    v.var_ridx as i32,
                    0,
                    0,
                )?;
            } else {
                self.luaK_codeABCk(ls, OpCode::OP_GETUPVAL, extra as i32, v.info, 0, 0)?;
            }
            self.luaK_reserveregs(ls, 1)?;
        }
        Ok(())
    }

    fn restassign(
        &mut self,
        ls: &mut LexState,
        lh_list: &mut Vec<ExpDesc>,
        nvars: i32,
    ) -> LuaResult<()> {
        let cur_v = match lh_list.last().copied() {
            Some(x) => x,
            None => return Ok(()),
        };
        if !vkisvar(cur_v.k) {
            return self.luaX_syntaxerror(ls, "syntax error");
        }
        self.check_readonly(ls, &cur_v)?;
        if self.testnext(ls, b',' as i32)? {
            let mut nv = ExpDesc::new(ExpKind::VVOID, 0);
            self.suffixedexp(ls, &mut nv)?;
            if !vkisindexed(nv.k) {
                self.check_conflict(ls, lh_list, &nv)?;
            }
            lh_list.push(nv);
            self.luaE_incCstack(ls.l)?;
            self.restassign(ls, lh_list, nvars + 1)?;
            if let Some(th) = self.threads.get_mut(ls.l) {
                th.nCcalls = th.nCcalls.wrapping_sub(1);
            }
        } else {
            self.checknext(ls, b'=' as i32)?;
            let mut e = ExpDesc::new(ExpKind::VVOID, 0);
            let nexps = self.explist(ls, &mut e)?;
            if nexps != nvars {
                self.adjust_assign(ls, nvars, nexps, &mut e)?;
            } else {
                self.luaK_setoneret(ls, &mut e);
                if let Some(last_lh) = lh_list.pop() {
                    self.luaK_storevar(ls, &last_lh, &mut e)?;
                }
                return Ok(());
            }
        }
        if let Some(last_lh) = lh_list.pop() {
            let reg = (ls.fs().freereg as i32) - 1;
            let mut e = ExpDesc::new(ExpKind::VNONRELOC, reg);
            self.luaK_storevar(ls, &last_lh, &mut e)?;
        }
        Ok(())
    }

    fn cond(&mut self, ls: &mut LexState) -> LuaResult<i32> {
        let mut v = ExpDesc::new(ExpKind::VVOID, 0);
        self.expr(ls, &mut v)?;
        if v.k == ExpKind::VNIL {
            v.k = ExpKind::VFALSE;
        }
        self.luaK_goiftrue(ls, &mut v)?;
        Ok(v.f)
    }

    fn gotostat(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let line = ls.linenumber;
        let name = self.str_checkname(ls)?;
        if let Some(lb) = Self::findlabel(ls, name) {
            let lblevel = Self::reglevel(ls, lb.nactvar as i32);
            if Self::luaY_nvarstack(ls) > lblevel {
                self.luaK_codeABCk(ls, OpCode::OP_CLOSE, lblevel, 0, 0, 0)?;
            }
            let jmp = self.luaK_jump(ls)?;
            self.luaK_patchlist(ls, jmp, lb.pc)?;
        } else {
            let jmp = self.luaK_jump(ls)?;
            self.newgotoentry(ls, name, line, jmp);
        }
        Ok(())
    }

    fn breakstat(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let line = ls.linenumber;
        self.luaX_next(ls)?;
        let jmp = self.luaK_jump(ls)?;
        let brkn = ls.brkn;
        self.newgotoentry(ls, brkn, line, jmp);
        Ok(())
    }

    fn checkrepeated(&mut self, ls: &mut LexState, name: StrId) -> LuaResult<()> {
        if let Some(lb) = Self::findlabel(ls, name) {
            let msg = format!(
                "label '{}' already defined on line {}",
                self.get_str_lossy(name),
                lb.line
            );
            return self.luaK_semerror(ls, &msg);
        }
        Ok(())
    }

    fn labelstat(&mut self, ls: &mut LexState, name: StrId, line: i32) -> LuaResult<()> {
        self.checknext(ls, TK_DBCOLON)?;
        while ls.t.token == (b';' as i32) || ls.t.token == TK_DBCOLON {
            self.statement(ls)?;
        }
        self.checkrepeated(ls, name)?;
        let is_last = Self::block_follow(ls, false);
        self.createlabel(ls, name, line, is_last)?;
        Ok(())
    }

    fn whilestat(&mut self, ls: &mut LexState, line: i32) -> LuaResult<()> {
        self.luaX_next(ls)?;
        let whileinit = self.luaK_getlabel(ls);
        let condexit = self.cond(ls)?;
        Self::enterblock(ls, 1);
        self.checknext(ls, TK_DO)?;
        self.block(ls)?;
        let jmp = self.luaK_jump(ls)?;
        self.luaK_patchlist(ls, jmp, whileinit)?;
        self.check_match(ls, TK_END, TK_WHILE, line)?;
        self.leaveblock(ls)?;
        self.luaK_patchtohere(ls, condexit)
    }

    fn repeatstat(&mut self, ls: &mut LexState, line: i32) -> LuaResult<()> {
        let repeat_init = self.luaK_getlabel(ls);
        Self::enterblock(ls, 1);
        Self::enterblock(ls, 0);
        self.luaX_next(ls)?;
        self.statlist(ls)?;
        self.check_match(ls, TK_UNTIL, TK_REPEAT, line)?;
        let mut condexit = self.cond(ls)?;
        let bl2 = ls.fs().bl_stack.last().copied().unwrap_or(BlockCnt {
            firstlabel: 0,
            firstgoto: 0,
            nactvar: 0,
            upval: 0,
            isloop: 0,
            insidetbc: 0,
        });
        self.leaveblock(ls)?;
        if bl2.upval != 0 {
            let exit = self.luaK_jump(ls)?;
            self.luaK_patchtohere(ls, condexit)?;
            let lvl = Self::reglevel(ls, bl2.nactvar as i32);
            self.luaK_codeABCk(ls, OpCode::OP_CLOSE, lvl, 0, 0, 0)?;
            condexit = self.luaK_jump(ls)?;
            self.luaK_patchtohere(ls, exit)?;
        }
        self.luaK_patchlist(ls, condexit, repeat_init)?;
        self.leaveblock(ls)
    }

    fn exp1(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let mut e = ExpDesc::new(ExpKind::VVOID, 0);
        self.expr(ls, &mut e)?;
        self.luaK_exp2nextreg(ls, &mut e)
    }

    fn fixforjump(&mut self, ls: &LexState, pc: i32, dest: i32, back: bool) -> LuaResult<()> {
        let mut jmp = self.get_inst(ls, pc);
        let mut offset = dest - (pc + 1);
        if back {
            offset = -offset;
        }
        if offset > MAXARG_BX {
            return self.luaX_syntaxerror(ls, "control structure too long");
        }
        SETARG_Bx(&mut jmp, offset);
        self.set_inst(ls, pc, jmp);
        Ok(())
    }

    fn forbody(
        &mut self,
        ls: &mut LexState,
        base: i32,
        line: i32,
        nvars: i32,
        isgen: bool,
    ) -> LuaResult<()> {
        self.checknext(ls, TK_DO)?;
        let prep_op = if isgen {
            OpCode::OP_TFORPREP
        } else {
            OpCode::OP_FORPREP
        };
        let loop_op = if isgen {
            OpCode::OP_TFORLOOP
        } else {
            OpCode::OP_FORLOOP
        };
        let prep = self.luaK_codeABx(ls, prep_op, base, 0)?;
        Self::enterblock(ls, 0);
        self.adjustlocalvars(ls, nvars);
        self.luaK_reserveregs(ls, nvars)?;
        self.block(ls)?;
        self.leaveblock(ls)?;
        let lbl = self.luaK_getlabel(ls);
        self.fixforjump(ls, prep, lbl, false)?;
        if isgen {
            self.luaK_codeABCk(ls, OpCode::OP_TFORCALL, base, 0, nvars, 0)?;
            self.luaK_fixline(ls, line);
        }
        let endfor = self.luaK_codeABx(ls, loop_op, base, 0)?;
        self.fixforjump(ls, endfor, prep + 1, true)?;
        self.luaK_fixline(ls, line);
        Ok(())
    }

    fn fornum(&mut self, ls: &mut LexState, varname: StrId, line: i32) -> LuaResult<()> {
        let base = ls.fs().freereg as i32;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvar(ls, varname)?;
        self.checknext(ls, b'=' as i32)?;
        self.exp1(ls)?;
        self.checknext(ls, b',' as i32)?;
        self.exp1(ls)?;
        if self.testnext(ls, b',' as i32)? {
            self.exp1(ls)?;
        } else {
            let fr = ls.fs().freereg as i32;
            self.luaK_int(ls, fr, 1)?;
            self.luaK_reserveregs(ls, 1)?;
        }
        self.adjustlocalvars(ls, 3);
        self.forbody(ls, base, line, 1, false)
    }

    fn forlist(&mut self, ls: &mut LexState, indexname: StrId) -> LuaResult<()> {
        let mut nvars: i32 = 5;
        let base = ls.fs().freereg as i32;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvarliteral(ls, b"(for state)")?;
        self.new_localvar(ls, indexname)?;
        while self.testnext(ls, b',' as i32)? {
            let nm = self.str_checkname(ls)?;
            self.new_localvar(ls, nm)?;
            nvars += 1;
        }
        self.checknext(ls, TK_IN)?;
        let line = ls.linenumber;
        let mut e = ExpDesc::new(ExpKind::VVOID, 0);
        let nexps = self.explist(ls, &mut e)?;
        self.adjust_assign(ls, 4, nexps, &mut e)?;
        self.adjustlocalvars(ls, 4);
        Self::marktobeclosed(ls);
        self.luaK_checkstack(ls, 3)?;
        self.forbody(ls, base, line, nvars - 4, true)
    }

    fn forstat(&mut self, ls: &mut LexState, line: i32) -> LuaResult<()> {
        Self::enterblock(ls, 1);
        self.luaX_next(ls)?;
        let varname = self.str_checkname(ls)?;
        match ls.t.token {
            c if c == (b'=' as i32) => self.fornum(ls, varname, line)?,
            c if c == (b',' as i32) || c == TK_IN => self.forlist(ls, varname)?,
            _ => return self.luaX_syntaxerror(ls, "'=' or 'in' expected"),
        }
        self.check_match(ls, TK_END, TK_FOR, line)?;
        self.leaveblock(ls)
    }

    fn test_then_block(&mut self, ls: &mut LexState, escapelist: &mut i32) -> LuaResult<()> {
        let mut v = ExpDesc::new(ExpKind::VVOID, 0);
        self.luaX_next(ls)?;
        self.expr(ls, &mut v)?;
        self.checknext(ls, TK_THEN)?;
        let jf = if ls.t.token == TK_BREAK {
            let line = ls.linenumber;
            self.luaK_goiffalse(ls, &mut v)?;
            self.luaX_next(ls)?;
            Self::enterblock(ls, 0);
            let brkn = ls.brkn;
            self.newgotoentry(ls, brkn, line, v.t);
            while self.testnext(ls, b';' as i32)? {}
            if Self::block_follow(ls, false) {
                self.leaveblock(ls)?;
                return Ok(());
            } else {
                self.luaK_jump(ls)?
            }
        } else {
            self.luaK_goiftrue(ls, &mut v)?;
            Self::enterblock(ls, 0);
            v.f
        };
        self.statlist(ls)?;
        self.leaveblock(ls)?;
        if ls.t.token == TK_ELSE || ls.t.token == TK_ELSEIF {
            let jmp = self.luaK_jump(ls)?;
            self.luaK_concat(ls, escapelist, jmp)?;
        }
        self.luaK_patchtohere(ls, jf)
    }

    fn ifstat(&mut self, ls: &mut LexState, line: i32) -> LuaResult<()> {
        let mut escapelist = NO_JUMP;
        self.test_then_block(ls, &mut escapelist)?;
        while ls.t.token == TK_ELSEIF {
            self.test_then_block(ls, &mut escapelist)?;
        }
        if self.testnext(ls, TK_ELSE)? {
            self.block(ls)?;
        }
        self.check_match(ls, TK_END, TK_IF, line)?;
        self.luaK_patchtohere(ls, escapelist)
    }

    fn localfunc(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let mut b = ExpDesc::new(ExpKind::VVOID, 0);
        let fvar = ls.fs().nactvar as usize;
        let nm = self.str_checkname(ls)?;
        self.new_localvar(ls, nm)?;
        self.adjustlocalvars(ls, 1);
        let line = ls.linenumber;
        self.body(ls, &mut b, false, line)?;
        let vd = Self::getlocalvardesc(ls, ls.fs_stack.len() - 1, fvar);
        let f_id = ls.fs().f;
        let pc = ls.fs().pc;
        if vd.pidx >= 0 {
            if let Some(p) = self.protos.get_mut(f_id) {
                if let Some(lv) = p.locvars.get_mut(vd.pidx as usize) {
                    lv.startpc = pc;
                }
            }
        }
        Ok(())
    }

    fn getlocalattribute(&mut self, ls: &mut LexState) -> LuaResult<u8> {
        if self.testnext(ls, b'<' as i32)? {
            let attr_id = self.str_checkname(ls)?;
            self.checknext(ls, b'>' as i32)?;
            let attr = self.get_str_bytes(attr_id);
            if attr == b"const" {
                return Ok(RDKCONST);
            } else if attr == b"close" {
                return Ok(RDKTOCLOSE);
            } else {
                let s = self.get_str_lossy(attr_id);
                return self.luaK_semerror(ls, &format!("unknown attribute '{s}'"));
            }
        }
        Ok(VDKREG)
    }

    fn checktoclose(&mut self, ls: &mut LexState, level: i32) -> LuaResult<()> {
        if level != -1 {
            Self::marktobeclosed(ls);
            let rl = Self::reglevel(ls, level);
            self.luaK_codeABCk(ls, OpCode::OP_TBC, rl, 0, 0, 0)?;
        }
        Ok(())
    }

    fn localstat(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let mut toclose: i32 = -1;
        let mut vidx: usize = 0;
        let mut nvars: i32 = 0;
        loop {
            let nm = self.str_checkname(ls)?;
            vidx = self.new_localvar(ls, nm)?;
            let kind = self.getlocalattribute(ls)?;
            if let Some(vd) = Self::getlocalvardesc_mut(ls, vidx) {
                vd.kind = kind;
            }
            if kind == RDKTOCLOSE {
                if toclose != -1 {
                    return self
                        .luaK_semerror(ls, "multiple to-be-closed variables in local list");
                }
                toclose = (ls.fs().nactvar as i32) + nvars;
            }
            nvars += 1;
            if !self.testnext(ls, b',' as i32)? {
                break;
            }
        }
        let mut e = ExpDesc::new(ExpKind::VVOID, 0);
        let nexps = if self.testnext(ls, b'=' as i32)? {
            self.explist(ls, &mut e)?
        } else {
            e.k = ExpKind::VVOID;
            0
        };
        let last_vd = Self::getlocalvardesc(ls, ls.fs_stack.len() - 1, vidx);
        if nvars == nexps && last_vd.kind == RDKCONST {
            if let Some(kv) = self.luaK_exp2const(ls, &e) {
                if let Some(vd) = Self::getlocalvardesc_mut(ls, vidx) {
                    vd.kind = RDKCTC;
                    vd.k = kv;
                }
                self.adjustlocalvars(ls, nvars - 1);
                ls.fs_mut().nactvar += 1;
                return self.checktoclose(ls, toclose);
            }
        }
        self.adjust_assign(ls, nvars, nexps, &mut e)?;
        self.adjustlocalvars(ls, nvars);
        self.checktoclose(ls, toclose)
    }

    fn funcname(&mut self, ls: &mut LexState, v: &mut ExpDesc) -> LuaResult<bool> {
        let mut ismethod = false;
        self.singlevar(ls, v)?;
        while ls.t.token == (b'.' as i32) {
            self.fieldsel(ls, v)?;
        }
        if ls.t.token == (b':' as i32) {
            ismethod = true;
            self.fieldsel(ls, v)?;
        }
        Ok(ismethod)
    }

    fn funcstat(&mut self, ls: &mut LexState, line: i32) -> LuaResult<()> {
        let mut v = ExpDesc::new(ExpKind::VVOID, 0);
        let mut b = ExpDesc::new(ExpKind::VVOID, 0);
        self.luaX_next(ls)?;
        let ismethod = self.funcname(ls, &mut v)?;
        self.body(ls, &mut b, ismethod, line)?;
        self.check_readonly(ls, &v)?;
        self.luaK_storevar(ls, &v, &mut b)?;
        self.luaK_fixline(ls, line);
        Ok(())
    }

    fn exprstat(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let mut v = ExpDesc::new(ExpKind::VVOID, 0);
        self.suffixedexp(ls, &mut v)?;
        if ls.t.token == (b'=' as i32) || ls.t.token == (b',' as i32) {
            let mut lh_list = vec![v];
            self.restassign(ls, &mut lh_list, 1)
        } else {
            if v.k != ExpKind::VCALL {
                return self.luaX_syntaxerror(ls, "syntax error");
            }
            let mut inst = self.get_inst(ls, v.info);
            SETARG_C(&mut inst, 1);
            self.set_inst(ls, v.info, inst);
            Ok(())
        }
    }

    fn retstat(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let mut e = ExpDesc::new(ExpKind::VVOID, 0);
        let mut first = Self::luaY_nvarstack(ls);
        let nret = if Self::block_follow(ls, true) || ls.t.token == (b';' as i32) {
            0
        } else {
            let mut n = self.explist(ls, &mut e)?;
            if e.k == ExpKind::VCALL || e.k == ExpKind::VVARARG {
                self.luaK_setreturns(ls, &e, LUA_MULTRET)?;
                let insidetbc = ls
                    .fs()
                    .bl_stack
                    .last()
                    .map_or(false, |b| b.insidetbc != 0);
                if e.k == ExpKind::VCALL && n == 1 && !insidetbc {
                    let mut inst = self.get_inst(ls, e.info);
                    SET_OPCODE(&mut inst, OpCode::OP_TAILCALL);
                    self.set_inst(ls, e.info, inst);
                }
                n = LUA_MULTRET;
            } else if n == 1 {
                first = self.luaK_exp2anyreg(ls, &mut e)?;
            } else {
                self.luaK_exp2nextreg(ls, &mut e)?;
            }
            n
        };
        self.luaK_ret(ls, first, nret)?;
        self.testnext(ls, b';' as i32)?;
        Ok(())
    }

    fn statement(&mut self, ls: &mut LexState) -> LuaResult<()> {
        let line = ls.linenumber;
        self.luaE_incCstack(ls.l)?;
        match ls.t.token {
            c if c == (b';' as i32) => {
                self.luaX_next(ls)?;
            }
            TK_IF => self.ifstat(ls, line)?,
            TK_WHILE => self.whilestat(ls, line)?,
            TK_DO => {
                self.luaX_next(ls)?;
                self.block(ls)?;
                self.check_match(ls, TK_END, TK_DO, line)?;
            }
            TK_FOR => self.forstat(ls, line)?,
            TK_REPEAT => self.repeatstat(ls, line)?,
            TK_FUNCTION => self.funcstat(ls, line)?,
            TK_LOCAL => {
                self.luaX_next(ls)?;
                if self.testnext(ls, TK_FUNCTION)? {
                    self.localfunc(ls)?;
                } else {
                    self.localstat(ls)?;
                }
            }
            TK_DBCOLON => {
                self.luaX_next(ls)?;
                let nm = self.str_checkname(ls)?;
                self.labelstat(ls, nm, line)?;
            }
            TK_RETURN => {
                self.luaX_next(ls)?;
                self.retstat(ls)?;
            }
            TK_BREAK => {
                self.breakstat(ls)?;
            }
            TK_GOTO => {
                self.luaX_next(ls)?;
                self.gotostat(ls)?;
            }
            _ => {
                self.exprstat(ls)?;
            }
        }
        ls.fs_mut().freereg = Self::luaY_nvarstack(ls) as u8;
        if let Some(th) = self.threads.get_mut(ls.l) {
            th.nCcalls = th.nCcalls.wrapping_sub(1);
        }
        Ok(())
    }

    fn mainfunc(&mut self, ls: &mut LexState, main_f: ProtoId) -> LuaResult<()> {
        self.open_func(ls, main_f, 0);
        self.setvararg(ls, 0)?;
        let up_idx = self.allocupvalue(ls, 0)?;
        let envn = ls.envn;
        if let Some(p) = self.protos.get_mut(main_f) {
            if let Some(up) = p.upvalues.get_mut(up_idx) {
                up.instack = 1;
                up.idx = 0;
                up.kind = VDKREG;
                up.name = Some(envn);
            }
        }
        self.luaX_next(ls)?;
        self.statlist(ls)?;
        self.check(ls, TK_EOS)?;
        self.close_func(ls)
    }

    pub fn luaY_parser(
        &mut self,
        l: ThreadId,
        input: InputSource,
        name: &[u8],
        firstchar: i32,
    ) -> LuaResult<LClosureId> {
        let main_f = self.protos.len();
        self.protos.push(Proto::new());
        let cl_id = self.luaF_newLclosure(1, main_f);
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::lcl_val(cl_id));
        self.luaD_inctop(l)?;
        let h_id = self.luaH_new();
        let top2 = self.get_top(l);
        self.stack_set(l, top2, TValue::table_val(h_id));
        self.luaD_inctop(l)?;
        let source_id = self.luaS_newlstr(name);
        if let Some(p) = self.protos.get_mut(main_f) {
            p.source = Some(source_id);
        }
        let mut ls = LexState {
            current: firstchar,
            linenumber: 1,
            lastline: 1,
            t: Token::empty(),
            lookahead: Token::empty(),
            fs_stack: Vec::new(),
            l,
            input,
            buff: Vec::with_capacity(32),
            h: h_id,
            dyd: Dyndata::new(),
            source: source_id,
            envn: self.envn,
            brkn: self.brk_str,
        };
        self.mainfunc(&mut ls, main_f)?;
        let cur_top = self.get_top(l);
        self.set_top_raw(l, cur_top - 1);
        Ok(cl_id)
    }

    fn checkmode(&mut self, l: ThreadId, mode: Option<&[u8]>, x: &str) -> LuaResult<()> {
        if let Some(m) = mode {
            let x_ch = x.as_bytes().first().copied().unwrap_or(0);
            if !m.contains(&x_ch) {
                let m_str = String::from_utf8_lossy(m);
                let msg = format!("attempt to load a {x} chunk (mode is '{m_str}')");
                let str_val = self.luaS_newlstr_val(msg.as_bytes());
                let top = self.get_top(l);
                self.stack_set(l, top, str_val);
                self.set_top_raw(l, top + 1);
                return Err(LUA_ERRSYNTAX);
            }
        }
        Ok(())
    }

    pub fn luaD_protectedparser(
        &mut self,
        l: ThreadId,
        mut input: InputSource,
        name: &[u8],
        mode: Option<&[u8]>,
    ) -> i32 {
        if let Some(th) = self.threads.get_mut(l) {
            th.inc_nny();
        }
        let old_top = self.get_top(l);
        let ef = self.threads.get(l).map_or(0, |th| th.errfunc);
        let mode_vec = mode.map(|m| m.to_vec());
        let name_vec = name.to_vec();
        let status = self.luaD_pcall(
            l,
            move |vm, th_id| {
                let mut dummy_ls = LexState {
                    current: EOZ,
                    linenumber: 1,
                    lastline: 1,
                    t: Token::empty(),
                    lookahead: Token::empty(),
                    fs_stack: Vec::new(),
                    l: th_id,
                    input: InputSource::Buffer {
                        bytes: Vec::new(),
                        pos: 0,
                    },
                    buff: Vec::new(),
                    h: 0,
                    dyd: Dyndata::new(),
                    source: 0,
                    envn: vm.envn,
                    brkn: vm.brk_str,
                };
                std::mem::swap(&mut dummy_ls.input, &mut input);
                let c = vm.zgetc(&mut dummy_ls)?;
                if c == (LUA_SIGNATURE.first().copied().unwrap_or(0x1b) as i32) {
                    vm.checkmode(th_id, mode_vec.as_deref(), "binary")?;
                    let msg = format!(
                        "{}: bad binary format (binary chunks not supported)",
                        String::from_utf8_lossy(&name_vec)
                    );
                    let sv = vm.luaS_newlstr_val(msg.as_bytes());
                    let t = vm.get_top(th_id);
                    vm.stack_set(th_id, t, sv);
                    vm.set_top_raw(th_id, t + 1);
                    return Err(LUA_ERRSYNTAX);
                } else {
                    vm.checkmode(th_id, mode_vec.as_deref(), "text")?;
                    let mut real_input = InputSource::Buffer {
                        bytes: Vec::new(),
                        pos: 0,
                    };
                    std::mem::swap(&mut dummy_ls.input, &mut real_input);
                    let cl_id = vm.luaY_parser(th_id, real_input, &name_vec, c)?;
                    vm.luaF_initupvals(cl_id);
                }
                Ok(())
            },
            old_top,
            ef,
        );
        if let Some(th) = self.threads.get_mut(l) {
            th.dec_nny();
        }
        status
    }
}

// ============================================================================
// Lua C API & Auxiliary Library (lapi.c & lauxlib.c)
// ============================================================================

const fn ispseudo(i: i32) -> bool {
    i <= LUA_REGISTRYINDEX
}
const fn isupvalue(i: i32) -> bool {
    i < LUA_REGISTRYINDEX
}

impl LuaVM {
    pub fn index2value(&self, l: ThreadId, idx: i32) -> (TValue, bool) {
        let th = match self.threads.get(l) {
            Some(t) => t,
            None => return (TValue::nil(), false),
        };
        let ci = th.ci();
        if idx > 0 {
            let o = ci.func + (idx as usize);
            if o >= th.top {
                (TValue::nil(), false)
            } else {
                (self.stack_get(l, o), true)
            }
        } else if !ispseudo(idx) {
            let o = ((th.top as isize) + (idx as isize)) as usize;
            (self.stack_get(l, o), true)
        } else if idx == LUA_REGISTRYINDEX {
            (self.l_registry, true)
        } else {
            let up_idx = (LUA_REGISTRYINDEX - idx) as usize;
            let fval = self.stack_get(l, ci.func);
            if fval.is_Cclosure() {
                if let Some(cc) = self.cclosures.get(fval.clCvalue()) {
                    if let Some(&uv) = cc.upvalue.get(up_idx - 1) {
                        return (uv, true);
                    }
                }
            }
            (TValue::nil(), false)
        }
    }

    pub fn index2stack(&self, l: ThreadId, idx: i32) -> usize {
        let th = match self.threads.get(l) {
            Some(t) => t,
            None => return 0,
        };
        let ci = th.ci();
        if idx > 0 {
            ci.func + (idx as usize)
        } else {
            ((th.top as isize) + (idx as isize)) as usize
        }
    }

    pub fn adjustresults(&mut self, l: ThreadId, nres: i32) {
        if nres <= LUA_MULTRET {
            if let Some(th) = self.threads.get_mut(l) {
                let top = th.top;
                if th.ci().top < top {
                    th.ci_mut().top = top;
                }
            }
        }
    }

    pub fn lua_checkstack(&mut self, l: ThreadId, n: usize) -> LuaResult<bool> {
        let (stack_last, top) = match self.threads.get(l) {
            Some(th) => (th.stack_last, th.top),
            None => return Ok(false),
        };
        let res = if stack_last.saturating_sub(top) > n {
            true
        } else {
            self.luaD_growstack(l, n, false)?
        };
        if res {
            if let Some(th) = self.threads.get_mut(l) {
                let needed = th.top + n;
                if th.ci().top < needed {
                    th.ci_mut().top = needed;
                }
            }
        }
        Ok(res)
    }

    pub fn lua_xmove(&mut self, from: ThreadId, to: ThreadId, n: usize) {
        if from == to {
            return;
        }
        let from_top = self.get_top(from).saturating_sub(n);
        self.set_top_raw(from, from_top);
        let mut to_top = self.get_top(to);
        for i in 0..n {
            let v = self.stack_get(from, from_top + i);
            self.stack_set(to, to_top, v);
            to_top += 1;
        }
        self.set_top_raw(to, to_top);
    }

    pub fn lua_absindex(&self, l: ThreadId, idx: i32) -> i32 {
        if idx > 0 || ispseudo(idx) {
            idx
        } else {
            let (top, func) = self
                .threads
                .get(l)
                .map_or((1, 0), |th| (th.top, th.ci().func));
            ((top - func) as i32) + idx
        }
    }

    pub fn lua_gettop(&self, l: ThreadId) -> i32 {
        let (top, func) = self
            .threads
            .get(l)
            .map_or((1, 0), |th| (th.top, th.ci().func));
        (top as i32) - ((func + 1) as i32)
    }

    pub fn lua_settop(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        let (func, mut top, tbclist) = match self.threads.get(l) {
            Some(th) => (th.ci().func, th.top, th.tbclist),
            None => return Ok(()),
        };
        let mut newtop = if idx >= 0 {
            let target = func + 1 + (idx as usize);
            while top < target {
                self.stack_set(l, top, TValue::nil());
                top += 1;
            }
            target
        } else {
            ((top as isize) + (idx as isize) + 1) as usize
        };
        if newtop < top && tbclist >= newtop {
            newtop = self.luaF_close(l, newtop, CLOSEKTOP, false)?;
        }
        self.set_top_raw(l, newtop);
        Ok(())
    }

    pub fn lua_pop(&mut self, l: ThreadId, n: i32) {
        let _ = self.lua_settop(l, -n - 1);
    }

    pub fn lua_closeslot(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        let mut level = self.index2stack(l, idx);
        level = self.luaF_close(l, level, CLOSEKTOP, false)?;
        self.stack_set(l, level, TValue::nil());
        Ok(())
    }

    fn reverse_stack(&mut self, l: ThreadId, mut from: usize, mut to: usize) {
        while from < to {
            let vf = self.stack_get(l, from);
            let vt = self.stack_get(l, to);
            self.stack_set(l, from, vt);
            self.stack_set(l, to, vf);
            from += 1;
            to -= 1;
        }
    }

    pub fn lua_rotate(&mut self, l: ThreadId, idx: i32, n: i32) {
        let t = self.get_top(l) - 1;
        let p = self.index2stack(l, idx);
        let m = if n >= 0 {
            t - (n as usize)
        } else {
            p + ((-n) as usize) - 1
        };
        self.reverse_stack(l, p, m);
        self.reverse_stack(l, m + 1, t);
        self.reverse_stack(l, p, t);
    }

    pub fn lua_insert(&mut self, l: ThreadId, idx: i32) {
        self.lua_rotate(l, idx, 1);
    }

    pub fn lua_remove(&mut self, l: ThreadId, idx: i32) {
        self.lua_rotate(l, idx, -1);
        self.lua_pop(l, 1);
    }

    pub fn lua_replace(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        self.lua_copy(l, -1, idx);
        self.lua_settop(l, -2)
    }

    pub fn lua_copy(&mut self, l: ThreadId, fromidx: i32, toidx: i32) {
        let (fr, _) = self.index2value(l, fromidx);
        if toidx > 0 || !ispseudo(toidx) {
            let stk = self.index2stack(l, toidx);
            self.stack_set(l, stk, fr);
        } else if toidx == LUA_REGISTRYINDEX {
            self.l_registry = fr;
        } else if isupvalue(toidx) {
            let up_idx = (LUA_REGISTRYINDEX - toidx) as usize;
            let func = self.threads.get(l).map_or(0, |th| th.ci().func);
            let fval = self.stack_get(l, func);
            if fval.is_Cclosure() {
                if let Some(cc) = self.cclosures.get_mut(fval.clCvalue()) {
                    if let Some(slot) = cc.upvalue.get_mut(up_idx - 1) {
                        *slot = fr;
                    }
                }
            }
        }
    }

    pub fn lua_pushvalue(&mut self, l: ThreadId, idx: i32) {
        let (v, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        self.stack_set(l, top, v);
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_type(&self, l: ThreadId, idx: i32) -> i32 {
        let (o, valid) = self.index2value(l, idx);
        if valid {
            o.ttype()
        } else {
            LUA_TNONE
        }
    }

    pub fn lua_typename(&self, t: i32) -> &'static str {
        ttypename(t)
    }

    pub fn luaL_typename(&self, l: ThreadId, idx: i32) -> &'static str {
        self.lua_typename(self.lua_type(l, idx))
    }

    pub fn lua_isnil(&self, l: ThreadId, idx: i32) -> bool {
        self.lua_type(l, idx) == LUA_TNIL
    }

    pub fn lua_isnone(&self, l: ThreadId, idx: i32) -> bool {
        self.lua_type(l, idx) == LUA_TNONE
    }

    pub fn lua_isnoneornil(&self, l: ThreadId, idx: i32) -> bool {
        self.lua_type(l, idx) <= 0
    }

    pub fn lua_isboolean(&self, l: ThreadId, idx: i32) -> bool {
        self.lua_type(l, idx) == LUA_TBOOLEAN
    }

    pub fn lua_istable(&self, l: ThreadId, idx: i32) -> bool {
        self.lua_type(l, idx) == LUA_TTABLE
    }

    pub fn lua_isfunction(&self, l: ThreadId, idx: i32) -> bool {
        self.lua_type(l, idx) == LUA_TFUNCTION
    }

    pub fn lua_isinteger(&self, l: ThreadId, idx: i32) -> bool {
        let (o, _) = self.index2value(l, idx);
        o.is_integer()
    }

    pub fn lua_isnumber(&self, l: ThreadId, idx: i32) -> bool {
        let (o, _) = self.index2value(l, idx);
        self.tonumber(&o).is_some()
    }

    pub fn lua_isstring(&self, l: ThreadId, idx: i32) -> bool {
        let (o, _) = self.index2value(l, idx);
        o.is_string() || o.is_number()
    }

    pub fn lua_rawequal(&self, l: ThreadId, idx1: i32, idx2: i32) -> bool {
        let (o1, v1) = self.index2value(l, idx1);
        let (o2, v2) = self.index2value(l, idx2);
        if v1 && v2 {
            self.luaV_rawequalobj(&o1, &o2)
        } else {
            false
        }
    }

    pub fn lua_arith(&mut self, l: ThreadId, op: i32) -> LuaResult<()> {
        if op == LUA_OPUNM || op == LUA_OPBNOT {
            let top = self.get_top(l);
            let v = self.stack_get(l, top - 1);
            self.stack_set(l, top, v);
            self.set_top_raw(l, top + 1);
        }
        let top = self.get_top(l);
        let p1 = self.stack_get(l, top - 2);
        let p2 = self.stack_get(l, top - 1);
        self.luaO_arith(l, op, &p1, &p2, top - 2)?;
        let new_top = self.get_top(l) - 1;
        self.set_top_raw(l, new_top);
        Ok(())
    }

    pub fn lua_compare(&mut self, l: ThreadId, idx1: i32, idx2: i32, op: i32) -> LuaResult<bool> {
        let (o1, v1) = self.index2value(l, idx1);
        let (o2, v2) = self.index2value(l, idx2);
        if v1 && v2 {
            match op {
                LUA_OPEQ => self.luaV_equalobj(Some(l), &o1, &o2),
                LUA_OPLT => self.luaV_lessthan(l, &o1, &o2),
                LUA_OPLE => self.luaV_lessequal(l, &o1, &o2),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub fn lua_stringtonumber(&mut self, l: ThreadId, s: &[u8]) -> usize {
        if let Some((v, sz)) = luaO_str2num(s) {
            let top = self.get_top(l);
            self.stack_set(l, top, v);
            self.set_top_raw(l, top + 1);
            sz
        } else {
            0
        }
    }

    pub fn lua_tonumberx(&self, l: ThreadId, idx: i32) -> Option<LuaNumber> {
        let (o, _) = self.index2value(l, idx);
        self.tonumber(&o)
    }

    pub fn lua_tointegerx(&self, l: ThreadId, idx: i32) -> Option<LuaInteger> {
        let (o, _) = self.index2value(l, idx);
        self.tointeger(&o, F2Imod::F2Ieq)
    }

    pub fn lua_toboolean(&self, l: ThreadId, idx: i32) -> bool {
        let (o, _) = self.index2value(l, idx);
        !o.l_isfalse()
    }

    pub fn lua_tolstring(&mut self, l: ThreadId, idx: i32) -> LuaResult<Option<Vec<u8>>> {
        let (mut o, valid) = self.index2value(l, idx);
        if !valid {
            return Ok(None);
        }
        if !o.is_string() {
            if !o.is_number() {
                return Ok(None);
            }
            self.luaO_tostring(&mut o);
            if idx > 0 || !ispseudo(idx) {
                let stk = self.index2stack(l, idx);
                self.stack_set(l, stk, o);
            }
        }
        Ok(Some(self.get_str_bytes(o.tsvalue()).to_vec()))
    }

    pub fn lua_rawlen(&mut self, l: ThreadId, idx: i32) -> LuaUnsigned {
        let (o, _) = self.index2value(l, idx);
        match o.ttypetag() {
            LUA_VSHRSTR | LUA_VLNGSTR => self.get_str_bytes(o.tsvalue()).len() as LuaUnsigned,
            LUA_VUSERDATA => self.udatas.get(o.uvalue()).map_or(0, |u| u.len as LuaUnsigned),
            LUA_VTABLE => self.luaH_getn(o.hvalue()),
            _ => 0,
        }
    }

    pub fn lua_tothread(&self, l: ThreadId, idx: i32) -> Option<ThreadId> {
        let (o, _) = self.index2value(l, idx);
        if o.is_thread() {
            Some(o.thvalue())
        } else {
            None
        }
    }

    pub fn lua_topointer(&self, l: ThreadId, idx: i32) -> usize {
        let (o, _) = self.index2value(l, idx);
        o.topointer()
    }

    pub fn lua_pushnil(&mut self, l: ThreadId) {
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::nil());
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_pushnumber(&mut self, l: ThreadId, n: LuaNumber) {
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::flt_val(n));
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_pushinteger(&mut self, l: ThreadId, n: LuaInteger) {
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::int_val(n));
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_pushlstring(&mut self, l: ThreadId, s: &[u8]) {
        let v = self.luaS_newlstr_val(s);
        let top = self.get_top(l);
        self.stack_set(l, top, v);
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_pushstring(&mut self, l: ThreadId, s: &str) {
        self.lua_pushlstring(l, s.as_bytes());
    }

    pub fn lua_pushcclosure(&mut self, l: ThreadId, f: LuaCFunction, n: usize) {
        if n == 0 {
            let top = self.get_top(l);
            self.stack_set(l, top, TValue::lcf_val(f));
            self.set_top_raw(l, top + 1);
        } else {
            let top = self.get_top(l) - n;
            let mut upvals = Vec::with_capacity(n);
            for i in 0..n {
                upvals.push(self.stack_get(l, top + i));
            }
            let cc_id = self.luaF_newCclosure(f, upvals);
            self.stack_set(l, top, TValue::ccl_val(cc_id));
            self.set_top_raw(l, top + 1);
        }
    }

    pub fn lua_pushcfunction(&mut self, l: ThreadId, f: LuaCFunction) {
        self.lua_pushcclosure(l, f, 0);
    }

    pub fn lua_pushboolean(&mut self, l: ThreadId, b: bool) {
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::bool_val(b));
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_pushlightuserdata(&mut self, l: ThreadId, p: usize) {
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::lightudata_val(p));
        self.set_top_raw(l, top + 1);
    }

    pub fn lua_pushthread(&mut self, l: ThreadId) -> bool {
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::thread_val(l));
        self.set_top_raw(l, top + 1);
        self.mainthread == l
    }

    pub fn getGtable(&self) -> TValue {
        let reg_id = self.l_registry.hvalue();
        self.tables
            .get(reg_id)
            .and_then(|t| t.array.get((LUA_RIDX_GLOBALS - 1) as usize))
            .copied()
            .unwrap_or_else(TValue::nil)
    }

    fn auxgetstr(&mut self, l: ThreadId, t: TValue, k: &[u8]) -> LuaResult<i32> {
        let str_id = self.luaS_newlstr(k);
        let is_short = k.len() <= LUAI_MAXSHORTLEN;
        if t.is_table() {
            let (sval, _) = self.luaH_getstr(t.hvalue(), str_id);
            if !sval.is_empty() {
                let top = self.get_top(l);
                self.stack_set(l, top, sval);
                self.set_top_raw(l, top + 1);
                return Ok(sval.ttype());
            }
            let top = self.get_top(l);
            let k_val = TValue::str_val(str_id, is_short);
            self.stack_set(l, top, k_val);
            self.set_top_raw(l, top + 1);
            self.luaV_finishget(l, t, ValOrigin::None, k_val, top, true)?;
        } else {
            let top = self.get_top(l);
            let k_val = TValue::str_val(str_id, is_short);
            self.stack_set(l, top, k_val);
            self.set_top_raw(l, top + 1);
            self.luaV_finishget(l, t, ValOrigin::None, k_val, top, false)?;
        }
        let top = self.get_top(l);
        Ok(self.stack_get(l, top - 1).ttype())
    }

    pub fn lua_getglobal(&mut self, l: ThreadId, name: &str) -> LuaResult<i32> {
        let g = self.getGtable();
        self.auxgetstr(l, g, name.as_bytes())
    }

    pub fn lua_gettable(&mut self, l: ThreadId, idx: i32) -> LuaResult<i32> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let key = self.stack_get(l, top - 1);
        if t.is_table() {
            let (sval, _) = self.luaH_get(t.hvalue(), &key);
            if !sval.is_empty() {
                self.stack_set(l, top - 1, sval);
                return Ok(sval.ttype());
            }
            self.luaV_finishget(l, t, ValOrigin::None, key, top - 1, true)?;
        } else {
            self.luaV_finishget(l, t, ValOrigin::None, key, top - 1, false)?;
        }
        let cur_top = self.get_top(l);
        Ok(self.stack_get(l, cur_top - 1).ttype())
    }

    pub fn lua_getfield(&mut self, l: ThreadId, idx: i32, k: &str) -> LuaResult<i32> {
        let (t, _) = self.index2value(l, idx);
        self.auxgetstr(l, t, k.as_bytes())
    }

    pub fn lua_geti(&mut self, l: ThreadId, idx: i32, n: LuaInteger) -> LuaResult<i32> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        if t.is_table() {
            let (sval, _) = self.luaH_getint(t.hvalue(), n);
            if !sval.is_empty() {
                self.stack_set(l, top, sval);
                self.set_top_raw(l, top + 1);
                return Ok(sval.ttype());
            }
            self.luaV_finishget(l, t, ValOrigin::None, TValue::int_val(n), top, true)?;
        } else {
            self.luaV_finishget(l, t, ValOrigin::None, TValue::int_val(n), top, false)?;
        }
        let cur_top = self.get_top(l);
        self.set_top_raw(l, cur_top + 1);
        Ok(self.stack_get(l, cur_top).ttype())
    }

    fn finishrawget(&mut self, l: ThreadId, val: TValue) -> i32 {
        let v = if val.is_empty() { TValue::nil() } else { val };
        let top = self.get_top(l);
        self.stack_set(l, top, v);
        self.set_top_raw(l, top + 1);
        v.ttype()
    }

    pub fn lua_rawget(&mut self, l: ThreadId, idx: i32) -> i32 {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let key = self.stack_get(l, top - 1);
        self.set_top_raw(l, top - 1);
        let (val, _) = self.luaH_get(t.hvalue(), &key);
        self.finishrawget(l, val)
    }

    pub fn lua_rawgeti(&mut self, l: ThreadId, idx: i32, n: LuaInteger) -> i32 {
        let (t, _) = self.index2value(l, idx);
        let (val, _) = self.luaH_getint(t.hvalue(), n);
        self.finishrawget(l, val)
    }

    pub fn lua_createtable(&mut self, l: ThreadId, narray: u32, nrec: u32) -> LuaResult<()> {
        let t_id = self.luaH_new();
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::table_val(t_id));
        self.set_top_raw(l, top + 1);
        if narray > 0 || nrec > 0 {
            self.luaH_resize(l, t_id, narray, nrec)?;
        }
        Ok(())
    }

    pub fn lua_getmetatable(&mut self, l: ThreadId, objindex: i32) -> bool {
        let (obj, _) = self.index2value(l, objindex);
        let mt = match obj.ttype() {
            LUA_TTABLE => self.tables.get(obj.hvalue()).and_then(|t| t.metatable),
            LUA_TUSERDATA => self.udatas.get(obj.uvalue()).and_then(|u| u.metatable),
            t => {
                if t >= 0 && (t as usize) < LUA_NUMTYPES {
                    self.mt.get(t as usize).copied().flatten()
                } else {
                    None
                }
            }
        };
        if let Some(m_id) = mt {
            let top = self.get_top(l);
            self.stack_set(l, top, TValue::table_val(m_id));
            self.set_top_raw(l, top + 1);
            true
        } else {
            false
        }
    }

    fn auxsetstr(&mut self, l: ThreadId, t: TValue, k: &[u8]) -> LuaResult<()> {
        let str_id = self.luaS_newlstr(k);
        let is_short = k.len() <= LUAI_MAXSHORTLEN;
        let top = self.get_top(l);
        let val = self.stack_get(l, top - 1);
        if t.is_table() {
            let (sval, sloc) = self.luaH_getstr(t.hvalue(), str_id);
            if !sval.is_empty() {
                self.write_slot(sloc, val);
                self.set_top_raw(l, top - 1);
                return Ok(());
            }
            let k_val = TValue::str_val(str_id, is_short);
            self.stack_set(l, top, k_val);
            self.set_top_raw(l, top + 1);
            self.luaV_finishset(l, t, ValOrigin::None, k_val, val, Some((sval, sloc)))?;
            let new_top = self.get_top(l) - 2;
            self.set_top_raw(l, new_top);
        } else {
            let k_val = TValue::str_val(str_id, is_short);
            self.stack_set(l, top, k_val);
            self.set_top_raw(l, top + 1);
            self.luaV_finishset(l, t, ValOrigin::None, k_val, val, None)?;
            let new_top = self.get_top(l) - 2;
            self.set_top_raw(l, new_top);
        }
        Ok(())
    }

    pub fn lua_setglobal(&mut self, l: ThreadId, name: &str) -> LuaResult<()> {
        let g = self.getGtable();
        self.auxsetstr(l, g, name.as_bytes())
    }

    pub fn lua_settable(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let key = self.stack_get(l, top - 2);
        let val = self.stack_get(l, top - 1);
        if t.is_table() {
            let (sval, sloc) = self.luaH_get(t.hvalue(), &key);
            if !sval.is_empty() {
                self.write_slot(sloc, val);
            } else {
                self.luaV_finishset(l, t, ValOrigin::None, key, val, Some((sval, sloc)))?;
            }
        } else {
            self.luaV_finishset(l, t, ValOrigin::None, key, val, None)?;
        }
        let new_top = self.get_top(l) - 2;
        self.set_top_raw(l, new_top);
        Ok(())
    }

    pub fn lua_setfield(&mut self, l: ThreadId, idx: i32, k: &str) -> LuaResult<()> {
        let (t, _) = self.index2value(l, idx);
        self.auxsetstr(l, t, k.as_bytes())
    }

    pub fn lua_seti(&mut self, l: ThreadId, idx: i32, n: LuaInteger) -> LuaResult<()> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let val = self.stack_get(l, top - 1);
        if t.is_table() {
            let (sval, sloc) = self.luaH_getint(t.hvalue(), n);
            if !sval.is_empty() {
                self.write_slot(sloc, val);
            } else {
                self.luaV_finishset(
                    l,
                    t,
                    ValOrigin::None,
                    TValue::int_val(n),
                    val,
                    Some((sval, sloc)),
                )?;
            }
        } else {
            self.luaV_finishset(l, t, ValOrigin::None, TValue::int_val(n), val, None)?;
        }
        let new_top = self.get_top(l) - 1;
        self.set_top_raw(l, new_top);
        Ok(())
    }

    pub fn lua_rawset(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let key = self.stack_get(l, top - 2);
        let val = self.stack_get(l, top - 1);
        let t_id = t.hvalue();
        self.luaH_set(l, t_id, &key, val)?;
        if let Some(tb) = self.tables.get_mut(t_id) {
            tb.invalidate_tm_cache();
        }
        self.set_top_raw(l, top - 2);
        Ok(())
    }

    pub fn lua_rawseti(&mut self, l: ThreadId, idx: i32, n: LuaInteger) -> LuaResult<()> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let val = self.stack_get(l, top - 1);
        self.luaH_setint(l, t.hvalue(), n, val)?;
        self.set_top_raw(l, top - 1);
        Ok(())
    }

    pub fn lua_setmetatable(&mut self, l: ThreadId, objindex: i32) {
        let (obj, _) = self.index2value(l, objindex);
        let top = self.get_top(l);
        let mt_val = self.stack_get(l, top - 1);
        let mt = if mt_val.is_nil() {
            None
        } else {
            Some(mt_val.hvalue())
        };
        match obj.ttype() {
            LUA_TTABLE => {
                if let Some(t) = self.tables.get_mut(obj.hvalue()) {
                    t.metatable = mt;
                }
            }
            LUA_TUSERDATA => {
                if let Some(u) = self.udatas.get_mut(obj.uvalue()) {
                    u.metatable = mt;
                }
            }
            t => {
                if t >= 0 && (t as usize) < LUA_NUMTYPES {
                    if let Some(slot) = self.mt.get_mut(t as usize) {
                        *slot = mt;
                    }
                }
            }
        }
        self.set_top_raw(l, top - 1);
    }

    pub fn lua_callk(
        &mut self,
        l: ThreadId,
        nargs: i32,
        nresults: i32,
        ctx: LuaKContext,
        k: Option<LuaKFunction>,
    ) -> LuaResult<()> {
        let top = self.get_top(l);
        let func = top - ((nargs + 1) as usize);
        let is_yieldable = self.threads.get(l).map_or(false, |th| th.yieldable());
        if k.is_some() && is_yieldable {
            if let Some(th) = self.threads.get_mut(l) {
                let ci = th.ci_mut();
                ci.k = k;
                ci.ctx = ctx;
            }
            self.luaD_call(l, func, nresults)?;
        } else {
            self.luaD_callnoyield(l, func, nresults)?;
        }
        self.adjustresults(l, nresults);
        Ok(())
    }

    pub fn lua_call(&mut self, l: ThreadId, nargs: i32, nresults: i32) -> LuaResult<()> {
        self.lua_callk(l, nargs, nresults, 0, None)
    }

    pub fn lua_pcallk(
        &mut self,
        l: ThreadId,
        nargs: i32,
        nresults: i32,
        errfunc: i32,
        ctx: LuaKContext,
        k: Option<LuaKFunction>,
    ) -> LuaResult<i32> {
        let ef_stk: isize = if errfunc == 0 {
            0
        } else {
            self.index2stack(l, errfunc) as isize
        };
        let top = self.get_top(l);
        let c_func = top - ((nargs + 1) as usize);
        let is_yieldable = self.threads.get(l).map_or(false, |th| th.yieldable());
        let status = if k.is_none() || !is_yieldable {
            self.luaD_pcall(
                l,
                |vm, th_id| vm.luaD_callnoyield(th_id, c_func, nresults),
                c_func,
                ef_stk,
            )
        } else {
            if let Some(th) = self.threads.get_mut(l) {
                let old_ef = th.errfunc;
                let allowhook = th.allowhook as u16;
                th.errfunc = ef_stk;
                let ci = th.ci_mut();
                ci.k = k;
                ci.ctx = ctx;
                ci.u2_val = c_func as i32;
                ci.old_errfunc = old_ef;
                ci.callstatus = (ci.callstatus & !CIST_OAH) | (allowhook & CIST_OAH) | CIST_YPCALL;
            }
            self.luaD_call(l, c_func, nresults)?;
            if let Some(th) = self.threads.get_mut(l) {
                let old_ef = th.ci().old_errfunc;
                th.ci_mut().callstatus &= !CIST_YPCALL;
                th.errfunc = old_ef;
            }
            LUA_OK
        };
        self.adjustresults(l, nresults);
        Ok(status)
    }

    pub fn lua_pcall(
        &mut self,
        l: ThreadId,
        nargs: i32,
        nresults: i32,
        errfunc: i32,
    ) -> LuaResult<i32> {
        self.lua_pcallk(l, nargs, nresults, errfunc, 0, None)
    }

    pub fn lua_load(
        &mut self,
        l: ThreadId,
        input: InputSource,
        chunkname: &[u8],
        mode: Option<&[u8]>,
    ) -> i32 {
        let cname = if chunkname.is_empty() {
            b"?".as_slice()
        } else {
            chunkname
        };
        let status = self.luaD_protectedparser(l, input, cname, mode);
        if status == LUA_OK {
            let top = self.get_top(l);
            let fval = self.stack_get(l, top - 1);
            if fval.is_Lclosure() {
                let cl_id = fval.clLvalue();
                let uv0_opt = self
                    .lclosures
                    .get(cl_id)
                    .and_then(|c| c.upvals.first().copied());
                if let Some(uv0) = uv0_opt {
                    let gt = self.getGtable();
                    self.set_upval(uv0, gt);
                }
            }
        }
        status
    }

    pub fn lua_error<T>(&mut self, l: ThreadId) -> LuaResult<T> {
        let top = self.get_top(l);
        let errobj = self.stack_get(l, top - 1);
        if errobj.is_shrstring() && errobj.tsvalue() == self.memerrmsg {
            Err(LUA_ERRMEM)
        } else {
            self.luaG_errormsg(l)
        }
    }

    pub fn lua_next(&mut self, l: ThreadId, idx: i32) -> LuaResult<bool> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        let more = self.luaH_next(l, t.hvalue(), top - 1)?;
        if more {
            self.set_top_raw(l, top + 1);
        } else {
            self.set_top_raw(l, top - 1);
        }
        Ok(more)
    }

    pub fn lua_toclose(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        let o = self.index2stack(l, idx);
        let nresults = self.threads.get(l).map_or(0, |th| th.ci().nresults);
        self.luaF_newtbcupval(l, o)?;
        if !hastocloseCfunc(nresults) {
            if let Some(th) = self.threads.get_mut(l) {
                th.ci_mut().nresults = codeNresults(nresults as i32);
            }
        }
        Ok(())
    }

    pub fn lua_concat(&mut self, l: ThreadId, n: i32) -> LuaResult<()> {
        if n > 0 {
            self.luaV_concat(l, n)?;
        } else {
            self.lua_pushlstring(l, b"");
        }
        Ok(())
    }

    pub fn lua_len(&mut self, l: ThreadId, idx: i32) -> LuaResult<()> {
        let (t, _) = self.index2value(l, idx);
        let top = self.get_top(l);
        self.luaV_objlen(l, top, &t, ValOrigin::None)?;
        self.set_top_raw(l, top + 1);
        Ok(())
    }

    pub fn lua_setupvalue(&mut self, l: ThreadId, funcindex: i32, n: i32) -> Option<String> {
        let (fi, _) = self.index2value(l, funcindex);
        let top = self.get_top(l);
        let val = self.stack_get(l, top - 1);
        match fi.ttypetag() {
            LUA_VCCL => {
                let cc_id = fi.clCvalue();
                let nup = self.cclosures.get(cc_id).map_or(0, |c| c.nupvalues as i32);
                if !(1 <= n && n <= nup) {
                    return None;
                }
                if let Some(cc) = self.cclosures.get_mut(cc_id) {
                    if let Some(slot) = cc.upvalue.get_mut((n - 1) as usize) {
                        *slot = val;
                    }
                }
                self.set_top_raw(l, top - 1);
                Some(String::new())
            }
            LUA_VLCL => {
                let cl_id = fi.clLvalue();
                let (p_id, uv_id) = self
                    .lclosures
                    .get(cl_id)
                    .and_then(|c| {
                        let u = c.upvals.get((n - 1) as usize).copied()?;
                        Some((c.p, u))
                    })?;
                let name_opt = self
                    .protos
                    .get(p_id)
                    .and_then(|p| p.upvalues.get((n - 1) as usize))
                    .and_then(|u| u.name);
                self.set_upval(uv_id, val);
                self.set_top_raw(l, top - 1);
                Some(match name_opt {
                    Some(s) => self.get_str_lossy(s),
                    None => "(no name)".to_string(),
                })
            }
            _ => None,
        }
    }

    pub fn lua_newthread(&mut self, l: ThreadId) -> ThreadId {
        let mut new_th = LuaThread::new();
        if let Some(th) = self.threads.get(l) {
            new_th.hookmask = th.hookmask;
        }
        let new_id = self.threads.len();
        self.threads.push(new_th);
        let top = self.get_top(l);
        self.stack_set(l, top, TValue::thread_val(new_id));
        self.set_top_raw(l, top + 1);
        new_id
    }

    // ========================================================================
    // lauxlib.c functions
    // ========================================================================

    fn findfield(&mut self, l: ThreadId, objidx: i32, level: i32) -> LuaResult<bool> {
        if level == 0 || !self.lua_istable(l, -1) {
            return Ok(false);
        }
        self.lua_pushnil(l);
        while self.lua_next(l, -2)? {
            if self.lua_type(l, -2) == LUA_TSTRING {
                if self.lua_rawequal(l, objidx, -1) {
                    self.lua_pop(l, 1);
                    return Ok(true);
                } else if self.findfield(l, objidx, level - 1)? {
                    self.lua_pushstring(l, ".");
                    self.lua_replace(l, -3)?;
                    self.lua_concat(l, 3)?;
                    return Ok(true);
                }
            }
            self.lua_pop(l, 1);
        }
        Ok(false)
    }

    fn pushglobalfuncname(&mut self, l: ThreadId, ar: &mut LuaDebug) -> LuaResult<bool> {
        let top = self.lua_gettop(l);
        self.lua_getinfo(l, "f", ar)?;
        self.lua_getfield(l, LUA_REGISTRYINDEX, "_LOADED")?;
        self.luaL_checkstack(l, 6, Some("not enough stack"))?;
        if self.findfield(l, top + 1, 2)? {
            let name_bytes = self.lua_tolstring(l, -1)?.unwrap_or_default();
            if name_bytes.starts_with(b"_G.") {
                let sub = name_bytes.get(3..).unwrap_or(&[]).to_vec();
                self.lua_pushlstring(l, &sub);
                self.lua_remove(l, -2);
            }
            self.lua_copy(l, -1, top + 1);
            self.lua_settop(l, top + 1)?;
            Ok(true)
        } else {
            self.lua_settop(l, top)?;
            Ok(false)
        }
    }

    pub fn luaL_argerror<T>(&mut self, l: ThreadId, mut arg: i32, extramsg: &str) -> LuaResult<T> {
        let mut ar = LuaDebug::new();
        if !self.lua_getstack(l, 0, &mut ar) {
            return self.luaL_error_str(l, &format!("bad argument #{arg} ({extramsg})"));
        }
        self.lua_getinfo(l, "n", &mut ar)?;
        if ar.namewhat == "method" {
            arg -= 1;
            if arg == 0 {
                let nm = ar.name.as_deref().unwrap_or("?");
                return self.luaL_error_str(l, &format!("calling '{nm}' on bad self ({extramsg})"));
            }
        }
        if ar.name.is_none() {
            if self.pushglobalfuncname(l, &mut ar)? {
                let nm_bytes = self.lua_tolstring(l, -1)?.unwrap_or_default();
                ar.name = Some(String::from_utf8_lossy(&nm_bytes).into_owned());
            } else {
                ar.name = Some("?".to_string());
            }
        }
        let nm = ar.name.as_deref().unwrap_or("?");
        self.luaL_error_str(l, &format!("bad argument #{arg} to '{nm}' ({extramsg})"))
    }

    pub fn luaL_typeerror<T>(&mut self, l: ThreadId, arg: i32, tname: &str) -> LuaResult<T> {
        let typearg = if self.luaL_getmetafield(l, arg, "__name")? == LUA_TSTRING {
            let b = self.lua_tolstring(l, -1)?.unwrap_or_default();
            String::from_utf8_lossy(&b).into_owned()
        } else if self.lua_type(l, arg) == LUA_TLIGHTUSERDATA {
            "light userdata".to_string()
        } else {
            self.luaL_typename(l, arg).to_string()
        };
        let msg = format!("{tname} expected, got {typearg}");
        self.luaL_argerror(l, arg, &msg)
    }

    pub fn tag_error<T>(&mut self, l: ThreadId, arg: i32, tag: i32) -> LuaResult<T> {
        let tname = self.lua_typename(tag);
        self.luaL_typeerror(l, arg, tname)
    }

    pub fn luaL_where(&mut self, l: ThreadId, level: i32) -> LuaResult<()> {
        let mut ar = LuaDebug::new();
        if self.lua_getstack(l, level, &mut ar) {
            self.lua_getinfo(l, "Sl", &mut ar)?;
            if ar.currentline > 0 {
                let s = format!("{}:{}: ", ar.short_src, ar.currentline);
                self.lua_pushstring(l, &s);
                return Ok(());
            }
        }
        self.lua_pushstring(l, "");
        Ok(())
    }

    pub fn luaL_error_str<T>(&mut self, l: ThreadId, msg: &str) -> LuaResult<T> {
        self.luaL_where(l, 1)?;
        self.lua_pushstring(l, msg);
        self.lua_concat(l, 2)?;
        self.lua_error(l)
    }

    pub fn luaL_checkstack(
        &mut self,
        l: ThreadId,
        space: usize,
        msg: Option<&str>,
    ) -> LuaResult<()> {
        if !self.lua_checkstack(l, space)? {
            if let Some(m) = msg {
                return self.luaL_error_str(l, &format!("stack overflow ({m})"));
            } else {
                return self.luaL_error_str(l, "stack overflow");
            }
        }
        Ok(())
    }

    pub fn luaL_checktype(&mut self, l: ThreadId, arg: i32, t: i32) -> LuaResult<()> {
        if self.lua_type(l, arg) != t {
            return self.tag_error(l, arg, t);
        }
        Ok(())
    }

    pub fn luaL_checkany(&mut self, l: ThreadId, arg: i32) -> LuaResult<()> {
        if self.lua_type(l, arg) == LUA_TNONE {
            return self.luaL_argerror(l, arg, "value expected");
        }
        Ok(())
    }

    pub fn luaL_checklstring(&mut self, l: ThreadId, arg: i32) -> LuaResult<Vec<u8>> {
        match self.lua_tolstring(l, arg)? {
            Some(s) => Ok(s),
            None => self.tag_error(l, arg, LUA_TSTRING),
        }
    }

    pub fn luaL_optlstring(
        &mut self,
        l: ThreadId,
        arg: i32,
        def: &[u8],
    ) -> LuaResult<Vec<u8>> {
        if self.lua_isnoneornil(l, arg) {
            Ok(def.to_vec())
        } else {
            self.luaL_checklstring(l, arg)
        }
    }

    pub fn luaL_checknumber(&mut self, l: ThreadId, arg: i32) -> LuaResult<LuaNumber> {
        match self.lua_tonumberx(l, arg) {
            Some(d) => Ok(d),
            None => self.tag_error(l, arg, LUA_TNUMBER),
        }
    }

    pub fn luaL_optnumber(
        &mut self,
        l: ThreadId,
        arg: i32,
        def: LuaNumber,
    ) -> LuaResult<LuaNumber> {
        if self.lua_isnoneornil(l, arg) {
            Ok(def)
        } else {
            self.luaL_checknumber(l, arg)
        }
    }

    pub fn luaL_checkinteger(&mut self, l: ThreadId, arg: i32) -> LuaResult<LuaInteger> {
        match self.lua_tointegerx(l, arg) {
            Some(d) => Ok(d),
            None => {
                if self.lua_isnumber(l, arg) {
                    self.luaL_argerror(l, arg, "number has no integer representation")
                } else {
                    self.tag_error(l, arg, LUA_TNUMBER)
                }
            }
        }
    }

    pub fn luaL_optinteger(
        &mut self,
        l: ThreadId,
        arg: i32,
        def: LuaInteger,
    ) -> LuaResult<LuaInteger> {
        if self.lua_isnoneornil(l, arg) {
            Ok(def)
        } else {
            self.luaL_checkinteger(l, arg)
        }
    }

    pub fn luaL_getmetafield(&mut self, l: ThreadId, obj: i32, event: &str) -> LuaResult<i32> {
        if !self.lua_getmetatable(l, obj) {
            Ok(LUA_TNIL)
        } else {
            self.lua_pushstring(l, event);
            let tt = self.lua_rawget(l, -2);
            if tt == LUA_TNIL {
                self.lua_pop(l, 2);
            } else {
                self.lua_remove(l, -2);
            }
            Ok(tt)
        }
    }

    pub fn luaL_callmeta(&mut self, l: ThreadId, mut obj: i32, event: &str) -> LuaResult<bool> {
        obj = self.lua_absindex(l, obj);
        if self.luaL_getmetafield(l, obj, event)? == LUA_TNIL {
            return Ok(false);
        }
        self.lua_pushvalue(l, obj);
        self.lua_call(l, 1, 1)?;
        Ok(true)
    }

    pub fn luaL_len(&mut self, l: ThreadId, idx: i32) -> LuaResult<LuaInteger> {
        self.lua_len(l, idx)?;
        let res = self.lua_tointegerx(l, -1);
        if let Some(len) = res {
            self.lua_pop(l, 1);
            Ok(len)
        } else {
            self.luaL_error_str(l, "object length is not an integer")
        }
    }

    pub fn luaL_tolstring(&mut self, l: ThreadId, mut idx: i32) -> LuaResult<Vec<u8>> {
        idx = self.lua_absindex(l, idx);
        if self.luaL_callmeta(l, idx, "__tostring")? {
            if !self.lua_isstring(l, -1) {
                return self.luaL_error_str(l, "'__tostring' must return a string");
            }
        } else {
            match self.lua_type(l, idx) {
                LUA_TNUMBER => {
                    let (o, _) = self.index2value(l, idx);
                    let s = tostringbuff(&o);
                    self.lua_pushlstring(l, s.as_bytes());
                }
                LUA_TSTRING => {
                    self.lua_pushvalue(l, idx);
                }
                LUA_TBOOLEAN => {
                    let b = self.lua_toboolean(l, idx);
                    self.lua_pushstring(l, if b { "true" } else { "false" });
                }
                LUA_TNIL => {
                    self.lua_pushstring(l, "nil");
                }
                _ => {
                    let tt = self.luaL_getmetafield(l, idx, "__name")?;
                    let kind = if tt == LUA_TSTRING {
                        let b = self.lua_tolstring(l, -1)?.unwrap_or_default();
                        String::from_utf8_lossy(&b).into_owned()
                    } else {
                        self.luaL_typename(l, idx).to_string()
                    };
                    let ptr = self.lua_topointer(l, idx);
                    let s = format!("{kind}: 0x{ptr:x}");
                    self.lua_pushstring(l, &s);
                    if tt != LUA_TNIL {
                        self.lua_remove(l, -2);
                    }
                }
            }
        }
        Ok(self.lua_tolstring(l, -1)?.unwrap_or_default())
    }

    pub fn luaL_setfuncs(
        &mut self,
        l: ThreadId,
        funcs: &[(&str, Option<LuaCFunction>)],
        nup: usize,
    ) -> LuaResult<()> {
        self.luaL_checkstack(l, nup, Some("too many upvalues"))?;
        for &(name, func_opt) in funcs {
            match func_opt {
                None => self.lua_pushboolean(l, false),
                Some(f) => {
                    for _ in 0..nup {
                        self.lua_pushvalue(l, -(nup as i32));
                    }
                    self.lua_pushcclosure(l, f, nup);
                }
            }
            self.lua_setfield(l, -((nup as i32) + 2), name)?;
        }
        self.lua_pop(l, nup as i32);
        Ok(())
    }

    pub fn luaL_getsubtable(
        &mut self,
        l: ThreadId,
        mut idx: i32,
        fname: &str,
    ) -> LuaResult<bool> {
        if self.lua_getfield(l, idx, fname)? == LUA_TTABLE {
            Ok(true)
        } else {
            self.lua_pop(l, 1);
            idx = self.lua_absindex(l, idx);
            self.lua_createtable(l, 0, 0)?;
            self.lua_pushvalue(l, -1);
            self.lua_setfield(l, idx, fname)?;
            Ok(false)
        }
    }

    pub fn luaL_requiref(
        &mut self,
        l: ThreadId,
        modname: &str,
        openf: LuaCFunction,
        glb: bool,
    ) -> LuaResult<()> {
        self.luaL_getsubtable(l, LUA_REGISTRYINDEX, "_LOADED")?;
        self.lua_getfield(l, -1, modname)?;
        if !self.lua_toboolean(l, -1) {
            self.lua_pop(l, 1);
            self.lua_pushcfunction(l, openf);
            self.lua_pushstring(l, modname);
            self.lua_call(l, 1, 1)?;
            self.lua_pushvalue(l, -1);
            self.lua_setfield(l, -3, modname)?;
        }
        self.lua_remove(l, -2);
        if glb {
            self.lua_pushvalue(l, -1);
            self.lua_setglobal(l, modname)?;
        }
        Ok(())
    }

    pub fn luaL_checkoption(
        &mut self,
        l: ThreadId,
        arg: i32,
        def: Option<&[u8]>,
        lst: &[&[u8]],
    ) -> LuaResult<usize> {
        let name = match def {
            Some(d) => self.luaL_optlstring(l, arg, d)?,
            None => self.luaL_checklstring(l, arg)?,
        };
        let name_nul_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let name_c = name.get(..name_nul_len).unwrap_or(&[]);
        for (i, &opt) in lst.iter().enumerate() {
            if opt == name_c {
                return Ok(i);
            }
        }
        let s = String::from_utf8_lossy(name_c);
        self.luaL_argerror(l, arg, &format!("invalid option '{s}'"))
    }
}

// ============================================================================
// Base Library (lbaselib.c)
// ============================================================================

fn b_str2int(s: &[u8], base: u32) -> Option<LuaInteger> {
    if s.contains(&0) {
        return None;
    }
    let mut idx = 0;
    while let Some(&b) = s.get(idx) {
        if c_isspace(b) {
            idx += 1;
        } else {
            break;
        }
    }
    let mut neg = false;
    if let Some(&b) = s.get(idx) {
        if b == b'-' {
            neg = true;
            idx += 1;
        } else if b == b'+' {
            idx += 1;
        }
    }
    if !s.get(idx).map_or(false, |&b| c_isalnum(b)) {
        return None;
    }
    let mut n: LuaUnsigned = 0;
    while let Some(&b) = s.get(idx) {
        if !c_isalnum(b) {
            break;
        }
        let digit = if c_isdigit(b) {
            (b - b'0') as u32
        } else {
            (c_toupper(b) - b'A' + 10) as u32
        };
        if digit >= base {
            return None;
        }
        n = n.wrapping_mul(base as u64).wrapping_add(digit as u64);
        idx += 1;
    }
    while let Some(&b) = s.get(idx) {
        if c_isspace(b) {
            idx += 1;
        } else {
            break;
        }
    }
    if idx == s.len() {
        Some(if neg {
            0u64.wrapping_sub(n) as LuaInteger
        } else {
            n as LuaInteger
        })
    } else {
        None
    }
}

fn luaB_print(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    for i in 1..=n {
        let s = vm.luaL_tolstring(l, i)?;
        if i > 1 {
            vm.output.push(b'\t');
        }
        vm.output.extend_from_slice(&s);
        vm.lua_pop(l, 1);
    }
    vm.output.push(b'\n');
    Ok(0)
}

fn luaB_warn(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    vm.luaL_checklstring(l, 1)?;
    for i in 2..=n {
        vm.luaL_checklstring(l, i)?;
    }
    for i in 1..n {
        let s = vm.lua_tolstring(l, i)?.unwrap_or_default();
        vm.do_warning(&s, true);
    }
    let s = vm.lua_tolstring(l, n)?.unwrap_or_default();
    vm.do_warning(&s, false);
    Ok(0)
}

impl LuaVM {
    fn do_warning(&mut self, msg: &[u8], tocont: bool) {
        let msg_nul = msg.iter().position(|&b| b == 0).unwrap_or(msg.len());
        let msg_c = msg.get(..msg_nul).unwrap_or(&[]);
        if self.warn_state != WarnState::Cont && !tocont && msg_c.first().copied() == Some(b'@') {
            let ctrl = msg_c.get(1..).unwrap_or(&[]);
            if ctrl == b"off" {
                self.warn_state = WarnState::Off;
            } else if ctrl == b"on" {
                self.warn_state = WarnState::On;
            }
            return;
        }
        match self.warn_state {
            WarnState::Off => {}
            WarnState::On => {
                self.err_output.extend_from_slice(b"Lua warning: ");
                self.err_output.extend_from_slice(msg_c);
                if tocont {
                    self.warn_state = WarnState::Cont;
                } else {
                    self.err_output.push(b'\n');
                }
            }
            WarnState::Cont => {
                self.err_output.extend_from_slice(msg_c);
                if !tocont {
                    self.err_output.push(b'\n');
                    self.warn_state = WarnState::On;
                }
            }
        }
    }
}

fn luaB_tonumber(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_isnoneornil(l, 2) {
        if vm.lua_type(l, 1) == LUA_TNUMBER {
            vm.lua_settop(l, 1)?;
            return Ok(1);
        } else {
            if let Some(s) = vm.lua_tolstring(l, 1)? {
                let len = s.len();
                if vm.lua_stringtonumber(l, &s) == len + 1 {
                    return Ok(1);
                }
            }
            vm.luaL_checkany(l, 1)?;
        }
    } else {
        let base = vm.luaL_checkinteger(l, 2)?;
        vm.luaL_checktype(l, 1, LUA_TSTRING)?;
        let s = vm.lua_tolstring(l, 1)?.unwrap_or_default();
        if !(2..=36).contains(&base) {
            return vm.luaL_argerror(l, 2, "base out of range");
        }
        if let Some(n) = b_str2int(&s, base as u32) {
            vm.lua_pushinteger(l, n);
            return Ok(1);
        }
    }
    vm.lua_pushnil(l);
    Ok(1)
}

fn luaB_error(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let level = vm.luaL_optinteger(l, 2, 1)? as i32;
    vm.lua_settop(l, 1)?;
    if vm.lua_type(l, 1) == LUA_TSTRING && level > 0 {
        vm.luaL_where(l, level)?;
        vm.lua_pushvalue(l, 1);
        vm.lua_concat(l, 2)?;
    }
    vm.lua_error(l)
}

fn luaB_getmetatable(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checkany(l, 1)?;
    if !vm.lua_getmetatable(l, 1) {
        vm.lua_pushnil(l);
        return Ok(1);
    }
    vm.luaL_getmetafield(l, 1, "__metatable")?;
    Ok(1)
}

fn luaB_setmetatable(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let t = vm.lua_type(l, 2);
    vm.luaL_checktype(l, 1, LUA_TTABLE)?;
    if !(t == LUA_TNIL || t == LUA_TTABLE) {
        return vm.luaL_typeerror(l, 2, "nil or table");
    }
    if vm.luaL_getmetafield(l, 1, "__metatable")? != LUA_TNIL {
        return vm.luaL_error_str(l, "cannot change a protected metatable");
    }
    vm.lua_settop(l, 2)?;
    vm.lua_setmetatable(l, 1);
    Ok(1)
}

fn luaB_rawequal(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checkany(l, 1)?;
    vm.luaL_checkany(l, 2)?;
    let eq = vm.lua_rawequal(l, 1, 2);
    vm.lua_pushboolean(l, eq);
    Ok(1)
}

fn luaB_rawlen(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let t = vm.lua_type(l, 1);
    if !(t == LUA_TTABLE || t == LUA_TSTRING) {
        return vm.luaL_typeerror(l, 1, "table or string");
    }
    let len = vm.lua_rawlen(l, 1);
    vm.lua_pushinteger(l, len as LuaInteger);
    Ok(1)
}

fn luaB_rawget(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checktype(l, 1, LUA_TTABLE)?;
    vm.luaL_checkany(l, 2)?;
    vm.lua_settop(l, 2)?;
    vm.lua_rawget(l, 1);
    Ok(1)
}

fn luaB_rawset(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checktype(l, 1, LUA_TTABLE)?;
    vm.luaL_checkany(l, 2)?;
    vm.luaL_checkany(l, 3)?;
    vm.lua_settop(l, 3)?;
    vm.lua_rawset(l, 1)?;
    Ok(1)
}

fn luaB_collectgarbage(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static OPTS: [&[u8]; 10] = [
        b"stop",
        b"restart",
        b"collect",
        b"count",
        b"step",
        b"setpause",
        b"setstepmul",
        b"isrunning",
        b"generational",
        b"incremental",
    ];
    static OPTSNUM: [i32; 10] = [
        LUA_GCSTOP,
        LUA_GCRESTART,
        LUA_GCCOLLECT,
        LUA_GCCOUNT,
        LUA_GCSTEP,
        LUA_GCSETPAUSE,
        LUA_GCSETSTEPMUL,
        LUA_GCISRUNNING,
        LUA_GCGEN,
        LUA_GCINC,
    ];
    let opt_idx = vm.luaL_checkoption(l, 1, Some(b"collect"), &OPTS)?;
    let o = OPTSNUM.get(opt_idx).copied().unwrap_or(LUA_GCCOLLECT);
    match o {
        LUA_GCCOUNT => {
            vm.lua_pushnumber(l, 25.0);
            Ok(1)
        }
        LUA_GCSTEP => {
            let _step = vm.luaL_optinteger(l, 2, 0)?;
            vm.lua_pushboolean(l, true);
            Ok(1)
        }
        LUA_GCSETPAUSE => {
            let p = vm.luaL_optinteger(l, 2, 0)? as i32;
            let prev = vm.gcpause;
            vm.gcpause = (p / 4) * 4;
            vm.lua_pushinteger(l, prev as LuaInteger);
            Ok(1)
        }
        LUA_GCSETSTEPMUL => {
            let p = vm.luaL_optinteger(l, 2, 0)? as i32;
            let prev = vm.gcstepmul;
            vm.gcstepmul = (p / 4) * 4;
            vm.lua_pushinteger(l, prev as LuaInteger);
            Ok(1)
        }
        LUA_GCISRUNNING => {
            vm.lua_pushboolean(l, vm.gcstp == 0);
            Ok(1)
        }
        LUA_GCGEN => {
            let minormul = vm.luaL_optinteger(l, 2, 0)? as i32;
            let majormul = vm.luaL_optinteger(l, 3, 0)? as i32;
            let prev = vm.gckind;
            if minormul != 0 {
                vm.genminormul = minormul;
            }
            if majormul != 0 {
                vm.genmajormul = (majormul / 4) * 4;
            }
            vm.gckind = LUA_GCGEN;
            vm.lua_pushstring(
                l,
                if prev == LUA_GCINC {
                    "incremental"
                } else {
                    "generational"
                },
            );
            Ok(1)
        }
        LUA_GCINC => {
            let pause = vm.luaL_optinteger(l, 2, 0)? as i32;
            let stepmul = vm.luaL_optinteger(l, 3, 0)? as i32;
            let stepsize = vm.luaL_optinteger(l, 4, 0)? as i32;
            let prev = vm.gckind;
            if pause != 0 {
                vm.gcpause = (pause / 4) * 4;
            }
            if stepmul != 0 {
                vm.gcstepmul = (stepmul / 4) * 4;
            }
            if stepsize != 0 {
                vm.gcstepsize = stepsize;
            }
            vm.gckind = LUA_GCINC;
            vm.lua_pushstring(
                l,
                if prev == LUA_GCINC {
                    "incremental"
                } else {
                    "generational"
                },
            );
            Ok(1)
        }
        LUA_GCSTOP => {
            vm.gcstp = 1;
            vm.lua_pushinteger(l, 0);
            Ok(1)
        }
        LUA_GCRESTART => {
            vm.gcstp = 0;
            vm.lua_pushinteger(l, 0);
            Ok(1)
        }
        _ => {
            vm.lua_pushinteger(l, 0);
            Ok(1)
        }
    }
}

fn luaB_type(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let t = vm.lua_type(l, 1);
    if t == LUA_TNONE {
        return vm.luaL_argerror(l, 1, "value expected");
    }
    let tname = vm.lua_typename(t);
    vm.lua_pushstring(l, tname);
    Ok(1)
}

fn luaB_next(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checktype(l, 1, LUA_TTABLE)?;
    vm.lua_settop(l, 2)?;
    if vm.lua_next(l, 1)? {
        Ok(2)
    } else {
        vm.lua_pushnil(l);
        Ok(1)
    }
}

fn pairscont(_vm: &mut LuaVM, _l: ThreadId, _status: i32, _k: LuaKContext) -> LuaResult<i32> {
    Ok(3)
}

fn luaB_pairs(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checkany(l, 1)?;
    if vm.luaL_getmetafield(l, 1, "__pairs")? == LUA_TNIL {
        vm.lua_pushcfunction(l, luaB_next);
        vm.lua_pushvalue(l, 1);
        vm.lua_pushnil(l);
    } else {
        vm.lua_pushvalue(l, 1);
        vm.lua_callk(l, 1, 3, 0, Some(pairscont))?;
    }
    Ok(3)
}

fn ipairsaux(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut i = vm.luaL_checkinteger(l, 2)?;
    i = i.wrapping_add(1);
    vm.lua_pushinteger(l, i);
    if vm.lua_geti(l, 1, i)? == LUA_TNIL {
        Ok(1)
    } else {
        Ok(2)
    }
}

fn luaB_ipairs(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checkany(l, 1)?;
    vm.lua_pushcfunction(l, ipairsaux);
    vm.lua_pushvalue(l, 1);
    vm.lua_pushinteger(l, 0);
    Ok(3)
}

fn load_aux(vm: &mut LuaVM, l: ThreadId, status: i32, envidx: i32) -> LuaResult<i32> {
    if status == LUA_OK {
        if envidx != 0 {
            vm.lua_pushvalue(l, envidx);
            if vm.lua_setupvalue(l, -2, 1).is_none() {
                vm.lua_pop(l, 1);
            }
        }
        Ok(1)
    } else {
        vm.lua_pushnil(l);
        vm.lua_insert(l, -2);
        Ok(2)
    }
}

const RESERVEDSLOT: i32 = 5;

fn luaB_load(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s_opt = vm.lua_tolstring(l, 1)?;
    let mode = vm.luaL_optlstring(l, 3, b"bt")?;
    let env = if !vm.lua_isnone(l, 4) { 4 } else { 0 };
    let status = if let Some(s) = s_opt {
        let chunkname = vm.luaL_optlstring(l, 2, &s)?;
        vm.lua_load(
            l,
            InputSource::Buffer { bytes: s, pos: 0 },
            &chunkname,
            Some(&mode),
        )
    } else {
        let chunkname = vm.luaL_optlstring(l, 2, b"=(load)")?;
        vm.luaL_checktype(l, 1, LUA_TFUNCTION)?;
        vm.lua_settop(l, RESERVEDSLOT)?;
        vm.lua_load(
            l,
            InputSource::Reader {
                slot: RESERVEDSLOT as usize,
                buf: Vec::new(),
                pos: 0,
                eof: false,
            },
            &chunkname,
            Some(&mode),
        )
    };
    load_aux(vm, l, status, env)
}

fn luaB_assert(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_toboolean(l, 1) {
        Ok(vm.lua_gettop(l))
    } else {
        vm.luaL_checkany(l, 1)?;
        vm.lua_remove(l, 1);
        vm.lua_pushstring(l, "assertion failed!");
        vm.lua_settop(l, 1)?;
        luaB_error(vm, l)
    }
}

fn luaB_select(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    if vm.lua_type(l, 1) == LUA_TSTRING {
        let s = vm.lua_tolstring(l, 1)?.unwrap_or_default();
        if s.first().copied() == Some(b'#') {
            vm.lua_pushinteger(l, (n - 1) as LuaInteger);
            return Ok(1);
        }
    }
    let mut i = vm.luaL_checkinteger(l, 1)?;
    if i < 0 {
        i = (n as LuaInteger) + i;
    } else if i > (n as LuaInteger) {
        i = n as LuaInteger;
    }
    if i < 1 {
        return vm.luaL_argerror(l, 1, "index out of range");
    }
    Ok(n - (i as i32))
}

fn finishpcall(vm: &mut LuaVM, l: ThreadId, status: i32, extra: LuaKContext) -> LuaResult<i32> {
    if status != LUA_OK && status != LUA_YIELD {
        vm.lua_pushboolean(l, false);
        vm.lua_pushvalue(l, -2);
        Ok(2)
    } else {
        Ok(vm.lua_gettop(l) - (extra as i32))
    }
}

fn luaB_pcall(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checkany(l, 1)?;
    vm.lua_pushboolean(l, true);
    vm.lua_insert(l, 1);
    let nargs = vm.lua_gettop(l) - 2;
    let status = vm.lua_pcallk(l, nargs, LUA_MULTRET, 0, 0, Some(finishpcall))?;
    finishpcall(vm, l, status, 0)
}

fn luaB_xpcall(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    vm.luaL_checktype(l, 2, LUA_TFUNCTION)?;
    vm.lua_pushboolean(l, true);
    vm.lua_pushvalue(l, 1);
    vm.lua_rotate(l, 3, 2);
    let status = vm.lua_pcallk(l, n - 2, LUA_MULTRET, 2, 2, Some(finishpcall))?;
    finishpcall(vm, l, status, 2)
}

fn luaB_tostring(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checkany(l, 1)?;
    vm.luaL_tolstring(l, 1)?;
    Ok(1)
}

pub fn luaopen_base(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static BASE_FUNCS: [(&str, Option<LuaCFunction>); 23] = [
        ("assert", Some(luaB_assert)),
        ("collectgarbage", Some(luaB_collectgarbage)),
        ("error", Some(luaB_error)),
        ("getmetatable", Some(luaB_getmetatable)),
        ("ipairs", Some(luaB_ipairs)),
        ("load", Some(luaB_load)),
        ("next", Some(luaB_next)),
        ("pairs", Some(luaB_pairs)),
        ("pcall", Some(luaB_pcall)),
        ("print", Some(luaB_print)),
        ("warn", Some(luaB_warn)),
        ("rawequal", Some(luaB_rawequal)),
        ("rawlen", Some(luaB_rawlen)),
        ("rawget", Some(luaB_rawget)),
        ("rawset", Some(luaB_rawset)),
        ("select", Some(luaB_select)),
        ("setmetatable", Some(luaB_setmetatable)),
        ("tonumber", Some(luaB_tonumber)),
        ("tostring", Some(luaB_tostring)),
        ("type", Some(luaB_type)),
        ("xpcall", Some(luaB_xpcall)),
        ("_G", None),
        ("_VERSION", None),
    ];
    let gt = vm.getGtable();
    let top = vm.get_top(l);
    vm.stack_set(l, top, gt);
    vm.set_top_raw(l, top + 1);
    vm.luaL_setfuncs(l, &BASE_FUNCS, 0)?;
    vm.lua_pushvalue(l, -1);
    vm.lua_setfield(l, -2, "_G")?;
    vm.lua_pushstring(l, LUA_VERSION);
    vm.lua_setfield(l, -2, "_VERSION")?;
    Ok(1)
}

// ============================================================================
// Coroutine Library (lcorolib.c)
// ============================================================================

const COS_RUN: usize = 0;
const COS_DEAD: usize = 1;
const COS_YIELD: usize = 2;
const COS_NORM: usize = 3;
static STATNAME: [&str; 4] = ["running", "dead", "suspended", "normal"];

fn getco(vm: &mut LuaVM, l: ThreadId) -> LuaResult<ThreadId> {
    match vm.lua_tothread(l, 1) {
        Some(co) => Ok(co),
        None => vm.luaL_typeerror(l, 1, "thread"),
    }
}

fn auxresume(vm: &mut LuaVM, l: ThreadId, co: ThreadId, narg: i32) -> LuaResult<i32> {
    if !vm.lua_checkstack(co, narg.max(0) as usize)? {
        vm.lua_pushstring(l, "too many arguments to resume");
        return Ok(-1);
    }
    vm.lua_xmove(l, co, narg.max(0) as usize);
    let (status, nres) = vm.lua_resume(co, Some(l), narg);
    if status == LUA_OK || status == LUA_YIELD {
        if !vm.lua_checkstack(l, (nres + 1) as usize)? {
            vm.lua_pop(co, nres);
            vm.lua_pushstring(l, "too many results to resume");
            return Ok(-1);
        }
        vm.lua_xmove(co, l, nres as usize);
        Ok(nres)
    } else {
        vm.lua_xmove(co, l, 1);
        Ok(-1)
    }
}

fn luaB_coresume(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let co = getco(vm, l)?;
    let narg = vm.lua_gettop(l) - 1;
    let r = auxresume(vm, l, co, narg)?;
    if r < 0 {
        vm.lua_pushboolean(l, false);
        vm.lua_insert(l, -2);
        Ok(2)
    } else {
        vm.lua_pushboolean(l, true);
        vm.lua_insert(l, -(r + 1));
        Ok(r + 1)
    }
}

fn luaB_auxwrap(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let co = vm.lua_tothread(l, lua_upvalueindex(1)).unwrap_or(0);
    let narg = vm.lua_gettop(l);
    let r = auxresume(vm, l, co, narg)?;
    if r < 0 {
        let mut stat = vm.threads.get(co).map_or(LUA_OK, |th| th.status);
        if stat != LUA_OK && stat != LUA_YIELD {
            stat = vm.lua_closethread(co, Some(l));
            vm.lua_xmove(co, l, 1);
        }
        if stat != LUA_ERRMEM && vm.lua_type(l, -1) == LUA_TSTRING {
            vm.luaL_where(l, 1)?;
            vm.lua_insert(l, -2);
            vm.lua_concat(l, 2)?;
        }
        return vm.lua_error(l);
    }
    Ok(r)
}

fn luaB_cocreate(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    vm.luaL_checktype(l, 1, LUA_TFUNCTION)?;
    let nl = vm.lua_newthread(l);
    vm.lua_pushvalue(l, 1);
    vm.lua_xmove(l, nl, 1);
    Ok(1)
}

fn luaB_cowrap(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    luaB_cocreate(vm, l)?;
    vm.lua_pushcclosure(l, luaB_auxwrap, 1);
    Ok(1)
}

fn luaB_yield(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    vm.lua_yieldk(l, n, 0, None)
}

fn auxstatus(vm: &LuaVM, l: ThreadId, co: ThreadId) -> usize {
    if l == co {
        COS_RUN
    } else {
        let st = vm.threads.get(co).map_or(LUA_OK, |th| th.status);
        match st {
            LUA_YIELD => COS_YIELD,
            LUA_OK => {
                let mut ar = LuaDebug::new();
                if vm.lua_getstack(co, 0, &mut ar) {
                    COS_NORM
                } else if vm.lua_gettop(co) == 0 {
                    COS_DEAD
                } else {
                    COS_YIELD
                }
            }
            _ => COS_DEAD,
        }
    }
}

fn luaB_costatus(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let co = getco(vm, l)?;
    let st = auxstatus(vm, l, co);
    let s = STATNAME.get(st).copied().unwrap_or("dead");
    vm.lua_pushstring(l, s);
    Ok(1)
}

fn luaB_yieldable(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let co = if vm.lua_isnone(l, 1) {
        l
    } else {
        getco(vm, l)?
    };
    let y = vm.threads.get(co).map_or(false, |th| th.yieldable());
    vm.lua_pushboolean(l, y);
    Ok(1)
}

fn luaB_corunning(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let ismain = vm.lua_pushthread(l);
    vm.lua_pushboolean(l, ismain);
    Ok(2)
}

fn luaB_close(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let co = getco(vm, l)?;
    let status = auxstatus(vm, l, co);
    match status {
        COS_DEAD | COS_YIELD => {
            let st = vm.lua_closethread(co, Some(l));
            if st == LUA_OK {
                vm.lua_pushboolean(l, true);
                Ok(1)
            } else {
                vm.lua_pushboolean(l, false);
                vm.lua_xmove(co, l, 1);
                Ok(2)
            }
        }
        _ => {
            let sname = STATNAME.get(status).copied().unwrap_or("running");
            vm.luaL_error_str(l, &format!("cannot close a {sname} coroutine"))
        }
    }
}

pub fn luaopen_coroutine(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static CO_FUNCS: [(&str, Option<LuaCFunction>); 8] = [
        ("create", Some(luaB_cocreate)),
        ("resume", Some(luaB_coresume)),
        ("running", Some(luaB_corunning)),
        ("status", Some(luaB_costatus)),
        ("wrap", Some(luaB_cowrap)),
        ("yield", Some(luaB_yield)),
        ("isyieldable", Some(luaB_yieldable)),
        ("close", Some(luaB_close)),
    ];
    vm.lua_createtable(l, 0, CO_FUNCS.len() as u32)?;
    vm.luaL_setfuncs(l, &CO_FUNCS, 0)?;
    Ok(1)
}

// ============================================================================
// Table Library (ltablib.c)
// ============================================================================

const TAB_R: u8 = 1;
const TAB_W: u8 = 2;
const TAB_L: u8 = 4;
const TAB_RW: u8 = TAB_R | TAB_W;

fn checkfield(vm: &mut LuaVM, l: ThreadId, key: &str, n: i32) -> bool {
    vm.lua_pushstring(l, key);
    vm.lua_rawget(l, -n) != LUA_TNIL
}

fn checktab(vm: &mut LuaVM, l: ThreadId, arg: i32, what: u8) -> LuaResult<()> {
    if vm.lua_type(l, arg) != LUA_TTABLE {
        let mut n = 1;
        let ok = vm.lua_getmetatable(l, arg)
            && ((what & TAB_R) == 0 || {
                n += 1;
                checkfield(vm, l, "__index", n)
            })
            && ((what & TAB_W) == 0 || {
                n += 1;
                checkfield(vm, l, "__newindex", n)
            })
            && ((what & TAB_L) == 0 || {
                n += 1;
                checkfield(vm, l, "__len", n)
            });
        if ok {
            vm.lua_pop(l, n);
        } else {
            vm.luaL_checktype(l, arg, LUA_TTABLE)?;
        }
    }
    Ok(())
}

fn aux_getn(vm: &mut LuaVM, l: ThreadId, n: i32, w: u8) -> LuaResult<LuaInteger> {
    checktab(vm, l, n, w | TAB_L)?;
    vm.luaL_len(l, n)
}

fn tinsert(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut e = aux_getn(vm, l, 1, TAB_RW)?;
    e = e.wrapping_add(1);
    let pos = match vm.lua_gettop(l) {
        2 => e,
        3 => {
            let p = vm.luaL_checkinteger(l, 2)?;
            if (p as LuaUnsigned).wrapping_sub(1) >= (e as LuaUnsigned) {
                return vm.luaL_argerror(l, 2, "position out of bounds");
            }
            let mut i = e;
            while i > p {
                vm.lua_geti(l, 1, i - 1)?;
                vm.lua_seti(l, 1, i)?;
                i -= 1;
            }
            p
        }
        _ => return vm.luaL_error_str(l, "wrong number of arguments to 'insert'"),
    };
    vm.lua_seti(l, 1, pos)?;
    Ok(0)
}

fn tremove(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let size = aux_getn(vm, l, 1, TAB_RW)?;
    let mut pos = vm.luaL_optinteger(l, 2, size)?;
    if pos != size && (pos as LuaUnsigned).wrapping_sub(1) > (size as LuaUnsigned) {
        return vm.luaL_argerror(l, 2, "position out of bounds");
    }
    vm.lua_geti(l, 1, pos)?;
    while pos < size {
        vm.lua_geti(l, 1, pos + 1)?;
        vm.lua_seti(l, 1, pos)?;
        pos += 1;
    }
    vm.lua_pushnil(l);
    vm.lua_seti(l, 1, pos)?;
    Ok(1)
}

fn tmove(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let f = vm.luaL_checkinteger(l, 2)?;
    let e = vm.luaL_checkinteger(l, 3)?;
    let t = vm.luaL_checkinteger(l, 4)?;
    let tt = if !vm.lua_isnoneornil(l, 5) { 5 } else { 1 };
    checktab(vm, l, 1, TAB_R)?;
    checktab(vm, l, tt, TAB_W)?;
    if e >= f {
        if !(f > 0 || e < LuaInteger::MAX + f) {
            return vm.luaL_argerror(l, 3, "too many elements to move");
        }
        let n = e - f + 1;
        if !(t <= LuaInteger::MAX - n + 1) {
            return vm.luaL_argerror(l, 4, "destination wrap around");
        }
        if t > e || t <= f || (tt != 1 && !vm.lua_compare(l, 1, tt, LUA_OPEQ)?) {
            for i in 0..n {
                vm.lua_geti(l, 1, f + i)?;
                vm.lua_seti(l, tt, t + i)?;
            }
        } else {
            let mut i = n - 1;
            while i >= 0 {
                vm.lua_geti(l, 1, f + i)?;
                vm.lua_seti(l, tt, t + i)?;
                i -= 1;
            }
        }
    }
    vm.lua_pushvalue(l, tt);
    Ok(1)
}

fn addfield(vm: &mut LuaVM, l: ThreadId, b: &mut Vec<u8>, i: LuaInteger) -> LuaResult<()> {
    vm.lua_geti(l, 1, i)?;
    if !vm.lua_isstring(l, -1) {
        let tname = vm.luaL_typename(l, -1);
        return vm.luaL_error_str(
            l,
            &format!("invalid value ({tname}) at index {i} in table for 'concat'"),
        );
    }
    let s = vm.lua_tolstring(l, -1)?.unwrap_or_default();
    b.extend_from_slice(&s);
    vm.lua_pop(l, 1);
    Ok(())
}

fn tconcat(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut last = aux_getn(vm, l, 1, TAB_R)?;
    let sep = vm.luaL_optlstring(l, 2, b"")?;
    let mut i = vm.luaL_optinteger(l, 3, 1)?;
    last = vm.luaL_optinteger(l, 4, last)?;
    let mut b = Vec::new();
    while i < last {
        addfield(vm, l, &mut b, i)?;
        b.extend_from_slice(&sep);
        i += 1;
    }
    if i == last {
        addfield(vm, l, &mut b, i)?;
    }
    vm.lua_pushlstring(l, &b);
    Ok(1)
}

fn tpack(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    vm.lua_createtable(l, n as u32, 1)?;
    vm.lua_insert(l, 1);
    let mut i = n;
    while i >= 1 {
        vm.lua_seti(l, 1, i as LuaInteger)?;
        i -= 1;
    }
    vm.lua_pushinteger(l, n as LuaInteger);
    vm.lua_setfield(l, 1, "n")?;
    Ok(1)
}

fn tunpack(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut i = vm.luaL_optinteger(l, 2, 1)?;
    let e = if vm.lua_isnoneornil(l, 3) {
        vm.luaL_len(l, 1)?
    } else {
        vm.luaL_checkinteger(l, 3)?
    };
    if i > e {
        return Ok(0);
    }
    let n = (e as LuaUnsigned).wrapping_sub(i as LuaUnsigned);
    if n >= (i32::MAX as LuaUnsigned) || !vm.lua_checkstack(l, (n + 1) as usize)? {
        return vm.luaL_error_str(l, "too many results to unpack");
    }
    while i < e {
        vm.lua_geti(l, 1, i)?;
        i += 1;
    }
    vm.lua_geti(l, 1, e)?;
    Ok((n + 1) as i32)
}

type IdxT = u32;
const RANLIMIT: IdxT = 100;

fn set2(vm: &mut LuaVM, l: ThreadId, i: IdxT, j: IdxT) -> LuaResult<()> {
    vm.lua_seti(l, 1, i as LuaInteger)?;
    vm.lua_seti(l, 1, j as LuaInteger)
}

fn sort_comp(vm: &mut LuaVM, l: ThreadId, a: i32, b: i32) -> LuaResult<bool> {
    if vm.lua_isnil(l, 2) {
        vm.lua_compare(l, a, b, LUA_OPLT)
    } else {
        vm.lua_pushvalue(l, 2);
        vm.lua_pushvalue(l, a - 1);
        vm.lua_pushvalue(l, b - 2);
        vm.lua_call(l, 2, 1)?;
        let res = vm.lua_toboolean(l, -1);
        vm.lua_pop(l, 1);
        Ok(res)
    }
}

fn partition(vm: &mut LuaVM, l: ThreadId, lo: IdxT, up: IdxT) -> LuaResult<IdxT> {
    let mut i = lo;
    let mut j = up - 1;
    loop {
        loop {
            i += 1;
            vm.lua_geti(l, 1, i as LuaInteger)?;
            if !sort_comp(vm, l, -1, -2)? {
                break;
            }
            if i == up - 1 {
                return vm.luaL_error_str(l, "invalid order function for sorting");
            }
            vm.lua_pop(l, 1);
        }
        loop {
            j -= 1;
            vm.lua_geti(l, 1, j as LuaInteger)?;
            if !sort_comp(vm, l, -3, -1)? {
                break;
            }
            if j < i {
                return vm.luaL_error_str(l, "invalid order function for sorting");
            }
            vm.lua_pop(l, 1);
        }
        if j < i {
            vm.lua_pop(l, 1);
            set2(vm, l, up - 1, i)?;
            return Ok(i);
        }
        set2(vm, l, i, j)?;
    }
}

fn choosePivot(lo: IdxT, up: IdxT, rnd: u32) -> IdxT {
    let r4 = (up - lo) / 4;
    (rnd % (r4 * 2)) + (lo + r4)
}

fn auxsort(
    vm: &mut LuaVM,
    l: ThreadId,
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: u32,
) -> LuaResult<()> {
    while lo < up {
        vm.lua_geti(l, 1, lo as LuaInteger)?;
        vm.lua_geti(l, 1, up as LuaInteger)?;
        if sort_comp(vm, l, -1, -2)? {
            set2(vm, l, lo, up)?;
        } else {
            vm.lua_pop(l, 2);
        }
        if up - lo == 1 {
            return Ok(());
        }
        let mut p = if up - lo < RANLIMIT || rnd == 0 {
            (lo + up) / 2
        } else {
            choosePivot(lo, up, rnd)
        };
        vm.lua_geti(l, 1, p as LuaInteger)?;
        vm.lua_geti(l, 1, lo as LuaInteger)?;
        if sort_comp(vm, l, -2, -1)? {
            set2(vm, l, p, lo)?;
        } else {
            vm.lua_pop(l, 1);
            vm.lua_geti(l, 1, up as LuaInteger)?;
            if sort_comp(vm, l, -1, -2)? {
                set2(vm, l, p, up)?;
            } else {
                vm.lua_pop(l, 2);
            }
        }
        if up - lo == 2 {
            return Ok(());
        }
        vm.lua_geti(l, 1, p as LuaInteger)?;
        vm.lua_pushvalue(l, -1);
        vm.lua_geti(l, 1, (up - 1) as LuaInteger)?;
        set2(vm, l, p, up - 1)?;
        p = partition(vm, l, lo, up)?;
        let n = if p - lo < up - p {
            auxsort(vm, l, lo, p - 1, rnd)?;
            let sz = p - lo;
            lo = p + 1;
            sz
        } else {
            auxsort(vm, l, p + 1, up, rnd)?;
            let sz = up - p;
            up = p - 1;
            sz
        };
        if (up - lo) / 128 > n {
            rnd = !0u32;
        }
    }
    Ok(())
}

fn tsort(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = aux_getn(vm, l, 1, TAB_RW)?;
    if n > 1 {
        if n >= (i32::MAX as LuaInteger) {
            return vm.luaL_argerror(l, 1, "array too big");
        }
        if !vm.lua_isnoneornil(l, 2) {
            vm.luaL_checktype(l, 2, LUA_TFUNCTION)?;
        }
        vm.lua_settop(l, 2)?;
        auxsort(vm, l, 1, n as IdxT, 0)?;
    }
    Ok(0)
}

pub fn luaopen_table(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static TAB_FUNCS: [(&str, Option<LuaCFunction>); 7] = [
        ("concat", Some(tconcat)),
        ("insert", Some(tinsert)),
        ("pack", Some(tpack)),
        ("unpack", Some(tunpack)),
        ("remove", Some(tremove)),
        ("move", Some(tmove)),
        ("sort", Some(tsort)),
    ];
    vm.lua_createtable(l, 0, TAB_FUNCS.len() as u32)?;
    vm.luaL_setfuncs(l, &TAB_FUNCS, 0)?;
    Ok(1)
}

// ============================================================================
// String Library (lstrlib.c)
// ============================================================================

const MAXSIZE: usize = i32::MAX as usize;
const LUA_MAXCAPTURES: usize = 32;
const MAXCCALLS_MATCH: i32 = 200;
const L_ESC: u8 = b'%';

fn posrelatI(pos: LuaInteger, len: usize) -> usize {
    if pos > 0 {
        pos as usize
    } else if pos == 0 {
        1
    } else if pos < -(len as LuaInteger) {
        1
    } else {
        ((len as LuaInteger) + pos + 1) as usize
    }
}

fn getendpos(vm: &mut LuaVM, l: ThreadId, arg: i32, def: LuaInteger, len: usize) -> LuaResult<usize> {
    let pos = vm.luaL_optinteger(l, arg, def)?;
    if pos > (len as LuaInteger) {
        Ok(len)
    } else if pos >= 0 {
        Ok(pos as usize)
    } else if pos < -(len as LuaInteger) {
        Ok(0)
    } else {
        Ok(((len as LuaInteger) + pos + 1) as usize)
    }
}

fn str_len(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    vm.lua_pushinteger(l, s.len() as LuaInteger);
    Ok(1)
}

fn str_sub(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let start_arg = vm.luaL_checkinteger(l, 2)?;
    let start = posrelatI(start_arg, s.len());
    let end = getendpos(vm, l, 3, -1, s.len())?;
    if start <= end {
        let sub = s.get((start - 1)..end).unwrap_or(&[]);
        vm.lua_pushlstring(l, sub);
    } else {
        vm.lua_pushlstring(l, b"");
    }
    Ok(1)
}

fn str_reverse(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut s = vm.luaL_checklstring(l, 1)?;
    s.reverse();
    vm.lua_pushlstring(l, &s);
    Ok(1)
}

fn str_lower(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut s = vm.luaL_checklstring(l, 1)?;
    for b in &mut s {
        *b = c_tolower(*b);
    }
    vm.lua_pushlstring(l, &s);
    Ok(1)
}

fn str_upper(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let mut s = vm.luaL_checklstring(l, 1)?;
    for b in &mut s {
        *b = c_toupper(*b);
    }
    vm.lua_pushlstring(l, &s);
    Ok(1)
}

fn str_rep(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let n = vm.luaL_checkinteger(l, 2)?;
    let sep = vm.luaL_optlstring(l, 3, b"")?;
    let len = s.len();
    let lsep = sep.len();
    if n <= 0 {
        vm.lua_pushlstring(l, b"");
    } else if len.checked_add(lsep).map_or(true, |sum| sum > MAXSIZE / (n as usize)) {
        return vm.luaL_error_str(l, "resulting string too large");
    } else {
        let nu = n as usize;
        let totallen = nu * len + (nu - 1) * lsep;
        let mut out = Vec::with_capacity(totallen);
        for i in 0..nu {
            if i > 0 && lsep > 0 {
                out.extend_from_slice(&sep);
            }
            out.extend_from_slice(&s);
        }
        vm.lua_pushlstring(l, &out);
    }
    Ok(1)
}

fn str_byte(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let pi = vm.luaL_optinteger(l, 2, 1)?;
    let posi = posrelatI(pi, s.len());
    let pose = getendpos(vm, l, 3, pi, s.len())?;
    if posi > pose {
        return Ok(0);
    }
    if pose - posi >= (i32::MAX as usize) {
        return vm.luaL_error_str(l, "string slice too long");
    }
    let n = (pose - posi) + 1;
    vm.luaL_checkstack(l, n, Some("string slice too long"))?;
    for i in 0..n {
        let b = s.get(posi + i - 1).copied().unwrap_or(0);
        vm.lua_pushinteger(l, b as LuaInteger);
    }
    Ok(n as i32)
}

fn str_char(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    let mut out = Vec::with_capacity(n.max(0) as usize);
    for i in 1..=n {
        let c = vm.luaL_checkinteger(l, i)? as LuaUnsigned;
        if c > 255 {
            return vm.luaL_argerror(l, i, "value out of range");
        }
        out.push(c as u8);
    }
    vm.lua_pushlstring(l, &out);
    Ok(1)
}

fn str_tonum(vm: &mut LuaVM, l: ThreadId, arg: i32) -> LuaResult<bool> {
    if vm.lua_type(l, arg) == LUA_TNUMBER {
        vm.lua_pushvalue(l, arg);
        Ok(true)
    } else if let Some(s) = vm.lua_tolstring(l, arg)? {
        let len = s.len();
        Ok(vm.lua_stringtonumber(l, &s) == len + 1)
    } else {
        Ok(false)
    }
}

fn str_trymt(vm: &mut LuaVM, l: ThreadId, mtname: &str) -> LuaResult<()> {
    vm.lua_settop(l, 2)?;
    if vm.lua_type(l, 2) == LUA_TSTRING || vm.luaL_getmetafield(l, 2, mtname)? == LUA_TNIL {
        let op = mtname.get(2..).unwrap_or(mtname);
        let t1 = vm.luaL_typename(l, -2);
        let t2 = vm.luaL_typename(l, -1);
        return vm.luaL_error_str(l, &format!("attempt to {op} a '{t1}' with a '{t2}'"));
    }
    vm.lua_insert(l, -3);
    vm.lua_call(l, 2, 1)
}

fn str_arith(vm: &mut LuaVM, l: ThreadId, op: i32, mtname: &str) -> LuaResult<i32> {
    if str_tonum(vm, l, 1)? && str_tonum(vm, l, 2)? {
        vm.lua_arith(l, op)?;
    } else {
        str_trymt(vm, l, mtname)?;
    }
    Ok(1)
}

fn arith_add(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPADD, "__add")
}
fn arith_sub(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPSUB, "__sub")
}
fn arith_mul(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPMUL, "__mul")
}
fn arith_mod(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPMOD, "__mod")
}
fn arith_pow(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPPOW, "__pow")
}
fn arith_div(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPDIV, "__div")
}
fn arith_idiv(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPIDIV, "__idiv")
}
fn arith_unm(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_arith(vm, l, LUA_OPUNM, "__unm")
}

// Pattern matching
const CAP_UNFINISHED: isize = -1;
const CAP_POSITION: isize = -2;

#[derive(Clone, Copy)]
struct Capture {
    init: usize,
    len: isize,
}

struct MatchState<'a> {
    src: &'a [u8],
    p: &'a [u8],
    l: ThreadId,
    matchdepth: i32,
    level: usize,
    capture: [Capture; LUA_MAXCAPTURES],
}

impl<'a> MatchState<'a> {
    fn new(src: &'a [u8], p: &'a [u8], l: ThreadId) -> Self {
        MatchState {
            src,
            p,
            l,
            matchdepth: MAXCCALLS_MATCH,
            level: 0,
            capture: [Capture { init: 0, len: 0 }; LUA_MAXCAPTURES],
        }
    }
    fn reprep(&mut self) {
        self.level = 0;
        self.matchdepth = MAXCCALLS_MATCH;
    }
}

fn check_capture(vm: &mut LuaVM, ms: &MatchState, l_ch: u8) -> LuaResult<usize> {
    let idx = (l_ch as isize) - (b'1' as isize);
    if idx < 0
        || (idx as usize) >= ms.level
        || ms
            .capture
            .get(idx as usize)
            .map_or(true, |c| c.len == CAP_UNFINISHED)
    {
        return vm.luaL_error_str(ms.l, &format!("invalid capture index %{}", idx + 1));
    }
    Ok(idx as usize)
}

fn capture_to_close(vm: &mut LuaVM, ms: &MatchState) -> LuaResult<usize> {
    let mut level = ms.level;
    while level > 0 {
        level -= 1;
        if ms
            .capture
            .get(level)
            .map_or(false, |c| c.len == CAP_UNFINISHED)
        {
            return Ok(level);
        }
    }
    vm.luaL_error_str(ms.l, "invalid pattern capture")
}

fn classend(vm: &mut LuaVM, ms: &MatchState, mut p_idx: usize) -> LuaResult<usize> {
    let ch = ms.p.get(p_idx).copied().unwrap_or(0);
    p_idx += 1;
    match ch {
        L_ESC => {
            if p_idx == ms.p.len() {
                return vm.luaL_error_str(ms.l, "malformed pattern (ends with '%')");
            }
            Ok(p_idx + 1)
        }
        b'[' => {
            if ms.p.get(p_idx).copied() == Some(b'^') {
                p_idx += 1;
            }
            loop {
                if p_idx == ms.p.len() {
                    return vm.luaL_error_str(ms.l, "malformed pattern (missing ']')");
                }
                let c = ms.p.get(p_idx).copied().unwrap_or(0);
                p_idx += 1;
                if c == L_ESC && p_idx < ms.p.len() {
                    p_idx += 1;
                }
                if ms.p.get(p_idx).copied() == Some(b']') {
                    break;
                }
            }
            Ok(p_idx + 1)
        }
        _ => Ok(p_idx),
    }
}

fn match_class(c: u8, cl: u8) -> bool {
    let res = match c_tolower(cl) {
        b'a' => c_isalpha(c),
        b'c' => c_iscntrl(c),
        b'd' => c_isdigit(c),
        b'g' => c_isgraph(c),
        b'l' => c_islower(c),
        b'p' => c_ispunct(c),
        b's' => c_isspace(c),
        b'u' => c_isupper(c),
        b'w' => c_isalnum(c),
        b'x' => c_isxdigit(c),
        b'z' => c == 0,
        _ => return cl == c,
    };
    if c_islower(cl) {
        res
    } else {
        !res
    }
}

fn matchbracketclass(c: u8, p: &[u8], mut p_idx: usize, ec: usize) -> bool {
    let mut sig = true;
    if p.get(p_idx + 1).copied() == Some(b'^') {
        sig = false;
        p_idx += 1;
    }
    while {
        p_idx += 1;
        p_idx < ec
    } {
        let ch = p.get(p_idx).copied().unwrap_or(0);
        if ch == L_ESC {
            p_idx += 1;
            let esc = p.get(p_idx).copied().unwrap_or(0);
            if match_class(c, esc) {
                return sig;
            }
        } else if p.get(p_idx + 1).copied() == Some(b'-') && p_idx + 2 < ec {
            p_idx += 2;
            let hi = p.get(p_idx).copied().unwrap_or(0);
            if ch <= c && c <= hi {
                return sig;
            }
        } else if ch == c {
            return sig;
        }
    }
    !sig
}

fn singlematch(ms: &MatchState, s_idx: usize, p_idx: usize, ep: usize) -> bool {
    if s_idx >= ms.src.len() {
        return false;
    }
    let c = ms.src.get(s_idx).copied().unwrap_or(0);
    let pch = ms.p.get(p_idx).copied().unwrap_or(0);
    match pch {
        b'.' => true,
        L_ESC => match_class(c, ms.p.get(p_idx + 1).copied().unwrap_or(0)),
        b'[' => matchbracketclass(c, ms.p, p_idx, ep - 1),
        _ => pch == c,
    }
}

fn matchbalance(
    vm: &mut LuaVM,
    ms: &MatchState,
    mut s_idx: usize,
    p_idx: usize,
) -> LuaResult<Option<usize>> {
    if p_idx + 1 >= ms.p.len() {
        return vm.luaL_error_str(ms.l, "malformed pattern (missing arguments to '%b')");
    }
    let s_ch = ms.src.get(s_idx).copied();
    let b = ms.p.get(p_idx).copied().unwrap_or(0);
    if s_ch != Some(b) {
        return Ok(None);
    }
    let e = ms.p.get(p_idx + 1).copied().unwrap_or(0);
    let mut cont: i32 = 1;
    while {
        s_idx += 1;
        s_idx < ms.src.len()
    } {
        let cur = ms.src.get(s_idx).copied().unwrap_or(0);
        if cur == e {
            cont -= 1;
            if cont == 0 {
                return Ok(Some(s_idx + 1));
            }
        } else if cur == b {
            cont += 1;
        }
    }
    Ok(None)
}

fn max_expand(
    vm: &mut LuaVM,
    ms: &mut MatchState,
    s_idx: usize,
    p_idx: usize,
    ep: usize,
) -> LuaResult<Option<usize>> {
    let mut i: isize = 0;
    while singlematch(ms, s_idx + (i as usize), p_idx, ep) {
        i += 1;
    }
    while i >= 0 {
        if let Some(res) = pat_match(vm, ms, s_idx + (i as usize), ep + 1)? {
            return Ok(Some(res));
        }
        i -= 1;
    }
    Ok(None)
}

fn min_expand(
    vm: &mut LuaVM,
    ms: &mut MatchState,
    mut s_idx: usize,
    p_idx: usize,
    ep: usize,
) -> LuaResult<Option<usize>> {
    loop {
        if let Some(res) = pat_match(vm, ms, s_idx, ep + 1)? {
            return Ok(Some(res));
        } else if singlematch(ms, s_idx, p_idx, ep) {
            s_idx += 1;
        } else {
            return Ok(None);
        }
    }
}

fn start_capture(
    vm: &mut LuaVM,
    ms: &mut MatchState,
    s_idx: usize,
    p_idx: usize,
    what: isize,
) -> LuaResult<Option<usize>> {
    let level = ms.level;
    if level >= LUA_MAXCAPTURES {
        return vm.luaL_error_str(ms.l, "too many captures");
    }
    if let Some(cap) = ms.capture.get_mut(level) {
        cap.init = s_idx;
        cap.len = what;
    }
    ms.level = level + 1;
    let res = pat_match(vm, ms, s_idx, p_idx)?;
    if res.is_none() {
        ms.level -= 1;
    }
    Ok(res)
}

fn end_capture(
    vm: &mut LuaVM,
    ms: &mut MatchState,
    s_idx: usize,
    p_idx: usize,
) -> LuaResult<Option<usize>> {
    let l = capture_to_close(vm, ms)?;
    let init = ms.capture.get(l).map_or(0, |c| c.init);
    if let Some(cap) = ms.capture.get_mut(l) {
        cap.len = (s_idx as isize) - (init as isize);
    }
    let res = pat_match(vm, ms, s_idx, p_idx)?;
    if res.is_none() {
        if let Some(cap) = ms.capture.get_mut(l) {
            cap.len = CAP_UNFINISHED;
        }
    }
    Ok(res)
}

fn match_capture(
    vm: &mut LuaVM,
    ms: &MatchState,
    s_idx: usize,
    l_ch: u8,
) -> LuaResult<Option<usize>> {
    let l = check_capture(vm, ms, l_ch)?;
    let cap = ms
        .capture
        .get(l)
        .copied()
        .unwrap_or(Capture { init: 0, len: 0 });
    let len = cap.len as usize;
    if ms.src.len().saturating_sub(s_idx) >= len {
        let cap_slice = ms.src.get(cap.init..(cap.init + len)).unwrap_or(&[]);
        let s_slice = ms.src.get(s_idx..(s_idx + len)).unwrap_or(&[]);
        if cap_slice == s_slice {
            return Ok(Some(s_idx + len));
        }
    }
    Ok(None)
}

fn pat_match(
    vm: &mut LuaVM,
    ms: &mut MatchState,
    mut s_idx: usize,
    mut p_idx: usize,
) -> LuaResult<Option<usize>> {
    if ms.matchdepth == 0 {
        return vm.luaL_error_str(ms.l, "pattern too complex");
    }
    ms.matchdepth -= 1;
    let res: LuaResult<Option<usize>> = 'init: loop {
        if p_idx == ms.p.len() {
            break 'init Ok(Some(s_idx));
        }
        let pch = ms.p.get(p_idx).copied().unwrap_or(0);
        match pch {
            b'(' => {
                if ms.p.get(p_idx + 1).copied() == Some(b')') {
                    break 'init start_capture(vm, ms, s_idx, p_idx + 2, CAP_POSITION);
                } else {
                    break 'init start_capture(vm, ms, s_idx, p_idx + 1, CAP_UNFINISHED);
                }
            }
            b')' => {
                break 'init end_capture(vm, ms, s_idx, p_idx + 1);
            }
            b'$' if p_idx + 1 == ms.p.len() => {
                break 'init Ok(if s_idx == ms.src.len() {
                    Some(s_idx)
                } else {
                    None
                });
            }
            L_ESC => {
                let next_p = ms.p.get(p_idx + 1).copied().unwrap_or(0);
                match next_p {
                    b'b' => match matchbalance(vm, ms, s_idx, p_idx + 2)? {
                        Some(ns) => {
                            s_idx = ns;
                            p_idx += 4;
                            continue 'init;
                        }
                        None => break 'init Ok(None),
                    },
                    b'f' => {
                        p_idx += 2;
                        if ms.p.get(p_idx).copied() != Some(b'[') {
                            break 'init vm.luaL_error_str(
                                ms.l,
                                "missing '[' after '%f' in pattern",
                            );
                        }
                        let ep = classend(vm, ms, p_idx)?;
                        let previous = if s_idx == 0 {
                            0
                        } else {
                            ms.src.get(s_idx - 1).copied().unwrap_or(0)
                        };
                        let current_c = ms.src.get(s_idx).copied().unwrap_or(0);
                        if !matchbracketclass(previous, ms.p, p_idx, ep - 1)
                            && matchbracketclass(current_c, ms.p, p_idx, ep - 1)
                        {
                            p_idx = ep;
                            continue 'init;
                        }
                        break 'init Ok(None);
                    }
                    b'0'..=b'9' => match match_capture(vm, ms, s_idx, next_p)? {
                        Some(ns) => {
                            s_idx = ns;
                            p_idx += 2;
                            continue 'init;
                        }
                        None => break 'init Ok(None),
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        let ep = classend(vm, ms, p_idx)?;
        let ep_ch = ms.p.get(ep).copied().unwrap_or(0);
        if !singlematch(ms, s_idx, p_idx, ep) {
            if ep_ch == b'*' || ep_ch == b'?' || ep_ch == b'-' {
                p_idx = ep + 1;
                continue 'init;
            } else {
                break 'init Ok(None);
            }
        } else {
            match ep_ch {
                b'?' => {
                    if let Some(r) = pat_match(vm, ms, s_idx + 1, ep + 1)? {
                        break 'init Ok(Some(r));
                    } else {
                        p_idx = ep + 1;
                        continue 'init;
                    }
                }
                b'+' => {
                    s_idx += 1;
                    break 'init max_expand(vm, ms, s_idx, p_idx, ep);
                }
                b'*' => {
                    break 'init max_expand(vm, ms, s_idx, p_idx, ep);
                }
                b'-' => {
                    break 'init min_expand(vm, ms, s_idx, p_idx, ep);
                }
                _ => {
                    s_idx += 1;
                    p_idx = ep;
                    continue 'init;
                }
            }
        }
    };
    ms.matchdepth += 1;
    res
}

fn lmemfind(s1: &[u8], s2: &[u8]) -> Option<usize> {
    if s2.is_empty() {
        return Some(0);
    }
    if s2.len() > s1.len() {
        return None;
    }
    s1.windows(s2.len()).position(|w| w == s2)
}

fn push_onecapture(
    vm: &mut LuaVM,
    ms: &MatchState,
    i: usize,
    s_start: Option<usize>,
    s_end: usize,
) -> LuaResult<()> {
    if i >= ms.level {
        if i != 0 {
            return vm.luaL_error_str(ms.l, &format!("invalid capture index %{}", i + 1));
        }
        let st = s_start.unwrap_or(0);
        let slice = ms.src.get(st..s_end).unwrap_or(&[]);
        vm.lua_pushlstring(ms.l, slice);
    } else {
        let cap = ms
            .capture
            .get(i)
            .copied()
            .unwrap_or(Capture { init: 0, len: 0 });
        if cap.len == CAP_UNFINISHED {
            return vm.luaL_error_str(ms.l, "unfinished capture");
        } else if cap.len == CAP_POSITION {
            vm.lua_pushinteger(ms.l, (cap.init as LuaInteger) + 1);
        } else {
            let slice = ms
                .src
                .get(cap.init..(cap.init + (cap.len as usize)))
                .unwrap_or(&[]);
            vm.lua_pushlstring(ms.l, slice);
        }
    }
    Ok(())
}

fn push_captures(
    vm: &mut LuaVM,
    ms: &MatchState,
    s_start: Option<usize>,
    s_end: usize,
) -> LuaResult<i32> {
    let nlevels = if ms.level == 0 && s_start.is_some() {
        1
    } else {
        ms.level
    };
    vm.luaL_checkstack(ms.l, nlevels, Some("too many captures"))?;
    for i in 0..nlevels {
        push_onecapture(vm, ms, i, s_start, s_end)?;
    }
    Ok(nlevels as i32)
}

fn nospecials(p: &[u8]) -> bool {
    !p.iter()
        .any(|&b| matches!(b, b'^' | b'$' | b'*' | b'+' | b'?' | b'.' | b'(' | b'[' | b'%' | b'-'))
}

fn str_find_aux(vm: &mut LuaVM, l: ThreadId, find: bool) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let p = vm.luaL_checklstring(l, 2)?;
    let init_arg = vm.luaL_optinteger(l, 3, 1)?;
    let init = posrelatI(init_arg, s.len()) - 1;
    if init > s.len() {
        vm.lua_pushnil(l);
        return Ok(1);
    }
    if find && (vm.lua_toboolean(l, 4) || nospecials(&p)) {
        if let Some(off) = lmemfind(s.get(init..).unwrap_or(&[]), &p) {
            let pos = init + off;
            vm.lua_pushinteger(l, (pos + 1) as LuaInteger);
            vm.lua_pushinteger(l, (pos + p.len()) as LuaInteger);
            return Ok(2);
        }
    } else {
        let anchor = p.first().copied() == Some(b'^');
        let p_slice = if anchor {
            p.get(1..).unwrap_or(&[])
        } else {
            p.as_slice()
        };
        let mut ms = MatchState::new(&s, p_slice, l);
        let mut s1 = init;
        loop {
            ms.reprep();
            if let Some(res) = pat_match(vm, &mut ms, s1, 0)? {
                if find {
                    vm.lua_pushinteger(l, (s1 + 1) as LuaInteger);
                    vm.lua_pushinteger(l, res as LuaInteger);
                    let ncaps = push_captures(vm, &ms, None, 0)?;
                    return Ok(ncaps + 2);
                } else {
                    return push_captures(vm, &ms, Some(s1), res);
                }
            }
            if s1 >= s.len() || anchor {
                break;
            }
            s1 += 1;
        }
    }
    vm.lua_pushnil(l);
    Ok(1)
}

fn str_find(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_find_aux(vm, l, true)
}

fn str_match(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    str_find_aux(vm, l, false)
}

fn gmatch_aux(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let (u_val, _) = vm.index2value(l, lua_upvalueindex(3));
    let u_id = u_val.uvalue();
    let gm = match vm.udatas.get(u_id).and_then(|u| match &u.payload {
        UdataPayload::GMatch(g) => Some(g.clone()),
        _ => None,
    }) {
        Some(g) => g,
        None => return Ok(0),
    };
    let s = vm.get_str_bytes(gm.s_id).to_vec();
    let p = vm.get_str_bytes(gm.p_id).to_vec();
    let mut ms = MatchState::new(&s, &p, l);
    let mut src = gm.src_pos;
    while src <= s.len() {
        ms.reprep();
        if let Some(e) = pat_match(vm, &mut ms, src, 0)? {
            if Some(e) != gm.lastmatch {
                if let Some(u) = vm.udatas.get_mut(u_id) {
                    if let UdataPayload::GMatch(g) = &mut u.payload {
                        g.src_pos = e;
                        g.lastmatch = Some(e);
                    }
                }
                return push_captures(vm, &ms, Some(src), e);
            }
        }
        src += 1;
    }
    Ok(0)
}

fn str_gmatch(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let _p = vm.luaL_checklstring(l, 2)?;
    let init_arg = vm.luaL_optinteger(l, 3, 1)?;
    let mut init = posrelatI(init_arg, s.len()) - 1;
    vm.lua_settop(l, 2)?;
    if init > s.len() {
        init = s.len() + 1;
    }
    let s_id = vm.index2value(l, 1).0.tsvalue();
    let p_id = vm.index2value(l, 2).0.tsvalue();
    let u_id = vm.udatas.len();
    vm.udatas.push(Udata {
        nuvalue: 0,
        len: 0,
        metatable: None,
        uv: Vec::new(),
        payload: UdataPayload::GMatch(GMatchStateData {
            src_pos: init,
            lastmatch: None,
            s_id,
            p_id,
        }),
    });
    let top = vm.get_top(l);
    vm.stack_set(l, top, TValue::udata_val(u_id));
    vm.set_top_raw(l, top + 1);
    vm.lua_pushcclosure(l, gmatch_aux, 3);
    Ok(1)
}

fn add_s(
    vm: &mut LuaVM,
    ms: &MatchState,
    b: &mut Vec<u8>,
    s_idx: usize,
    e_idx: usize,
) -> LuaResult<()> {
    let news = vm.lua_tolstring(ms.l, 3)?.unwrap_or_default();
    let mut i = 0;
    while i < news.len() {
        let ch = news.get(i).copied().unwrap_or(0);
        if ch != L_ESC {
            b.push(ch);
            i += 1;
        } else {
            i += 1;
            let esc = news.get(i).copied().unwrap_or(0);
            if esc == L_ESC {
                b.push(esc);
            } else if esc == b'0' {
                b.extend_from_slice(ms.src.get(s_idx..e_idx).unwrap_or(&[]));
            } else if c_isdigit(esc) {
                let cap_idx = (esc - b'1') as usize;
                push_onecapture(vm, ms, cap_idx, Some(s_idx), e_idx)?;
                let val = vm.lua_tolstring(ms.l, -1)?.unwrap_or_default();
                b.extend_from_slice(&val);
                vm.lua_pop(ms.l, 1);
            } else {
                return vm.luaL_error_str(ms.l, "invalid use of '%' in replacement string");
            }
            i += 1;
        }
    }
    Ok(())
}

fn add_value(
    vm: &mut LuaVM,
    ms: &MatchState,
    b: &mut Vec<u8>,
    s_idx: usize,
    e_idx: usize,
    tr: i32,
) -> LuaResult<bool> {
    match tr {
        LUA_TFUNCTION => {
            vm.lua_pushvalue(ms.l, 3);
            let n = push_captures(vm, ms, Some(s_idx), e_idx)?;
            vm.lua_call(ms.l, n, 1)?;
        }
        LUA_TTABLE => {
            push_onecapture(vm, ms, 0, Some(s_idx), e_idx)?;
            vm.lua_gettable(ms.l, 3)?;
        }
        _ => {
            add_s(vm, ms, b, s_idx, e_idx)?;
            return Ok(true);
        }
    }
    if !vm.lua_toboolean(ms.l, -1) {
        vm.lua_pop(ms.l, 1);
        b.extend_from_slice(ms.src.get(s_idx..e_idx).unwrap_or(&[]));
        Ok(false)
    } else if !vm.lua_isstring(ms.l, -1) {
        let tname = vm.luaL_typename(ms.l, -1);
        vm.luaL_error_str(ms.l, &format!("invalid replacement value (a {tname})"))
    } else {
        let r = vm.lua_tolstring(ms.l, -1)?.unwrap_or_default();
        b.extend_from_slice(&r);
        vm.lua_pop(ms.l, 1);
        Ok(true)
    }
}

fn str_gsub(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let src = vm.luaL_checklstring(l, 1)?;
    let p = vm.luaL_checklstring(l, 2)?;
    let tr = vm.lua_type(l, 3);
    let max_s = vm.luaL_optinteger(l, 4, (src.len() as LuaInteger) + 1)?;
    if !matches!(
        tr,
        LUA_TNUMBER | LUA_TSTRING | LUA_TFUNCTION | LUA_TTABLE
    ) {
        return vm.luaL_typeerror(l, 3, "string/function/table");
    }
    let anchor = p.first().copied() == Some(b'^');
    let p_slice = if anchor {
        p.get(1..).unwrap_or(&[])
    } else {
        p.as_slice()
    };
    let mut ms = MatchState::new(&src, p_slice, l);
    let mut n: LuaInteger = 0;
    let mut changed = false;
    let mut b = Vec::new();
    let mut src_pos: usize = 0;
    let mut lastmatch: Option<usize> = None;

    while n < max_s {
        ms.reprep();
        if let Some(e) = pat_match(vm, &mut ms, src_pos, 0)? {
            if Some(e) != lastmatch {
                n += 1;
                if add_value(vm, &ms, &mut b, src_pos, e, tr)? {
                    changed = true;
                }
                src_pos = e;
                lastmatch = Some(e);
                if anchor {
                    break;
                }
                continue;
            }
        }
        if src_pos < src.len() {
            b.push(src.get(src_pos).copied().unwrap_or(0));
            src_pos += 1;
        } else {
            break;
        }
        if anchor {
            break;
        }
    }
    if !changed {
        vm.lua_pushvalue(l, 1);
    } else {
        b.extend_from_slice(src.get(src_pos..).unwrap_or(&[]));
        vm.lua_pushlstring(l, &b);
    }
    vm.lua_pushinteger(l, n);
    Ok(2)
}

// string.format
fn addquoted(b: &mut Vec<u8>, s: &[u8]) {
    b.push(b'"');
    for i in 0..s.len() {
        let ch = s.get(i).copied().unwrap_or(0);
        if ch == b'"' || ch == b'\\' || ch == b'\n' {
            b.push(b'\\');
            b.push(ch);
        } else if c_iscntrl(ch) {
            let next_ch = s.get(i + 1).copied().unwrap_or(0);
            if !c_isdigit(next_ch) {
                b.extend_from_slice(format!("\\{ch}").as_bytes());
            } else {
                b.extend_from_slice(format!("\\{ch:03}").as_bytes());
            }
        } else {
            b.push(ch);
        }
    }
    b.push(b'"');
}

fn quotefloat(n: LuaNumber) -> String {
    if n == f64::INFINITY {
        "1e9999".to_string()
    } else if n == f64::NEG_INFINITY {
        "-1e9999".to_string()
    } else if n.is_nan() {
        "(0/0)".to_string()
    } else {
        c_format_float(n, false, false, false, false, false, None, None, b'a')
    }
}

fn addliteral(vm: &mut LuaVM, l: ThreadId, b: &mut Vec<u8>, arg: i32) -> LuaResult<()> {
    match vm.lua_type(l, arg) {
        LUA_TSTRING => {
            let s = vm.lua_tolstring(l, arg)?.unwrap_or_default();
            addquoted(b, &s);
        }
        LUA_TNUMBER => {
            if !vm.lua_isinteger(l, arg) {
                let n = vm.lua_tonumberx(l, arg).unwrap_or(0.0);
                b.extend_from_slice(quotefloat(n).as_bytes());
            } else {
                let n = vm.lua_tointegerx(l, arg).unwrap_or(0);
                let s = if n == LuaInteger::MIN {
                    format!("0x{:x}", n as LuaUnsigned)
                } else {
                    n.to_string()
                };
                b.extend_from_slice(s.as_bytes());
            }
        }
        LUA_TNIL | LUA_TBOOLEAN => {
            let s = vm.luaL_tolstring(l, arg)?;
            b.extend_from_slice(&s);
            vm.lua_pop(l, 1);
        }
        _ => {
            return vm.luaL_argerror(l, arg, "value has no literal form");
        }
    }
    Ok(())
}

struct ParsedFormat {
    flag_minus: bool,
    flag_plus: bool,
    flag_space: bool,
    flag_hash: bool,
    flag_zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
}

fn checkformat(
    vm: &mut LuaVM,
    l: ThreadId,
    form: &[u8],
    flags: &[u8],
    allow_precision: bool,
) -> LuaResult<ParsedFormat> {
    let mut pf = ParsedFormat {
        flag_minus: false,
        flag_plus: false,
        flag_space: false,
        flag_hash: false,
        flag_zero: false,
        width: None,
        precision: None,
    };
    let mut idx = 1; // skip '%'
    while let Some(&b) = form.get(idx) {
        if flags.contains(&b) {
            match b {
                b'-' => pf.flag_minus = true,
                b'+' => pf.flag_plus = true,
                b' ' => pf.flag_space = true,
                b'#' => pf.flag_hash = true,
                b'0' => pf.flag_zero = true,
                _ => {}
            }
            idx += 1;
        } else {
            break;
        }
    }
    if form.get(idx).copied() != Some(b'0') {
        let mut w: usize = 0;
        let mut has_w = false;
        if let Some(&d1) = form.get(idx) {
            if c_isdigit(d1) {
                w = (d1 - b'0') as usize;
                has_w = true;
                idx += 1;
                if let Some(&d2) = form.get(idx) {
                    if c_isdigit(d2) {
                        w = w * 10 + ((d2 - b'0') as usize);
                        idx += 1;
                    }
                }
            }
        }
        if has_w {
            pf.width = Some(w);
        }
        if form.get(idx).copied() == Some(b'.') && allow_precision {
            idx += 1;
            let mut p: usize = 0;
            if let Some(&d1) = form.get(idx) {
                if c_isdigit(d1) {
                    p = (d1 - b'0') as usize;
                    idx += 1;
                    if let Some(&d2) = form.get(idx) {
                        if c_isdigit(d2) {
                            p = p * 10 + ((d2 - b'0') as usize);
                            idx += 1;
                        }
                    }
                }
            }
            pf.precision = Some(p);
        }
    }
    let spec_ok = form.get(idx).map_or(false, |&b| c_isalpha(b)) && idx + 1 == form.len();
    if !spec_ok {
        let f_str = String::from_utf8_lossy(form);
        return vm.luaL_error_str(l, &format!("invalid conversion specification: '{f_str}'"));
    }
    Ok(pf)
}

fn str_format(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let top = vm.lua_gettop(l);
    let mut arg = 1;
    let strfrmt = vm.luaL_checklstring(l, arg)?;
    let mut out = Vec::new();
    let mut idx = 0;
    const L_FMTFLAGSF: &[u8] = b"-+#0 ";
    const L_FMTFLAGSX: &[u8] = b"-#0";
    const L_FMTFLAGSI: &[u8] = b"-+0 ";
    const L_FMTFLAGSU: &[u8] = b"-0";
    const L_FMTFLAGSC: &[u8] = b"-";

    while idx < strfrmt.len() {
        let ch = strfrmt.get(idx).copied().unwrap_or(0);
        if ch != L_ESC {
            out.push(ch);
            idx += 1;
        } else {
            idx += 1;
            if strfrmt.get(idx).copied() == Some(L_ESC) {
                out.push(L_ESC);
                idx += 1;
            } else {
                arg += 1;
                if arg > top {
                    return vm.luaL_argerror(l, arg, "no value");
                }
                let start_idx = idx;
                while let Some(&b) = strfrmt.get(idx) {
                    if L_FMTFLAGSF.contains(&b) || (b'1'..=b'9').contains(&b) || b == b'.' {
                        idx += 1;
                    } else {
                        break;
                    }
                }
                if idx < strfrmt.len() {
                    idx += 1;
                }
                let span_len = idx - start_idx;
                if span_len >= 32 - 10 {
                    return vm.luaL_error_str(l, "invalid format (too long)");
                }
                let mut form = Vec::with_capacity(span_len + 1);
                form.push(b'%');
                let slice = strfrmt.get(start_idx..idx).unwrap_or(&[]);
                let nul_pos = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
                form.extend_from_slice(slice.get(..nul_pos).unwrap_or(&[]));
                let spec = slice.last().copied().unwrap_or(0);

                match spec {
                    b'c' => {
                        let pf = checkformat(vm, l, &form, L_FMTFLAGSC, false)?;
                        let c_int = vm.luaL_checkinteger(l, arg)? as i32;
                        let byte_val = (c_int as u8) as u8;
                        let w = pf.width.unwrap_or(0);
                        if w <= 1 {
                            out.push(byte_val);
                        } else if pf.flag_minus {
                            out.push(byte_val);
                            out.extend(std::iter::repeat(b' ').take(w - 1));
                        } else {
                            out.extend(std::iter::repeat(b' ').take(w - 1));
                            out.push(byte_val);
                        }
                    }
                    b'd' | b'i' | b'u' | b'o' | b'x' | b'X' => {
                        let flags = match spec {
                            b'd' | b'i' => L_FMTFLAGSI,
                            b'u' => L_FMTFLAGSU,
                            _ => L_FMTFLAGSX,
                        };
                        let n = vm.luaL_checkinteger(l, arg)?;
                        let pf = checkformat(vm, l, &form, flags, true)?;
                        let s = c_format_int(
                            n,
                            pf.flag_minus,
                            pf.flag_plus,
                            pf.flag_space,
                            pf.flag_hash,
                            pf.flag_zero,
                            pf.width,
                            pf.precision,
                            spec,
                        );
                        out.extend_from_slice(s.as_bytes());
                    }
                    b'a' | b'A' | b'f' | b'e' | b'E' | b'g' | b'G' => {
                        let n = vm.luaL_checknumber(l, arg)?;
                        let pf = checkformat(vm, l, &form, L_FMTFLAGSF, true)?;
                        let s = c_format_float(
                            n,
                            pf.flag_minus,
                            pf.flag_plus,
                            pf.flag_space,
                            pf.flag_hash,
                            pf.flag_zero,
                            pf.width,
                            pf.precision,
                            spec,
                        );
                        out.extend_from_slice(s.as_bytes());
                    }
                    b'p' => {
                        let p = vm.lua_topointer(l, arg);
                        let pf = checkformat(vm, l, &form, L_FMTFLAGSC, false)?;
                        let s = if p == 0 {
                            "(null)".to_string()
                        } else {
                            format!("0x{p:x}")
                        };
                        let w = pf.width.unwrap_or(0);
                        if s.len() >= w {
                            out.extend_from_slice(s.as_bytes());
                        } else if pf.flag_minus {
                            out.extend_from_slice(s.as_bytes());
                            out.extend(std::iter::repeat(b' ').take(w - s.len()));
                        } else {
                            out.extend(std::iter::repeat(b' ').take(w - s.len()));
                            out.extend_from_slice(s.as_bytes());
                        }
                    }
                    b'q' => {
                        if form.len() != 2 {
                            return vm
                                .luaL_error_str(l, "specifier '%q' cannot have modifiers");
                        }
                        addliteral(vm, l, &mut out, arg)?;
                    }
                    b's' => {
                        let s = vm.luaL_tolstring(l, arg)?;
                        if form.len() == 2 {
                            out.extend_from_slice(&s);
                            vm.lua_pop(l, 1);
                        } else {
                            if s.contains(&0) {
                                return vm.luaL_argerror(l, arg, "string contains zeros");
                            }
                            let pf = checkformat(vm, l, &form, L_FMTFLAGSC, true)?;
                            if pf.precision.is_none() && s.len() >= 100 {
                                out.extend_from_slice(&s);
                                vm.lua_pop(l, 1);
                            } else {
                                let slice = match pf.precision {
                                    Some(p) if p < s.len() => s.get(..p).unwrap_or(&[]),
                                    _ => s.as_slice(),
                                };
                                let w = pf.width.unwrap_or(0);
                                if slice.len() >= w {
                                    out.extend_from_slice(slice);
                                } else if pf.flag_minus {
                                    out.extend_from_slice(slice);
                                    out.extend(std::iter::repeat(b' ').take(w - slice.len()));
                                } else {
                                    out.extend(std::iter::repeat(b' ').take(w - slice.len()));
                                    out.extend_from_slice(slice);
                                }
                                vm.lua_pop(l, 1);
                            }
                        }
                    }
                    _ => {
                        let f_str = String::from_utf8_lossy(&form);
                        return vm.luaL_error_str(
                            l,
                            &format!("invalid conversion '{f_str}' to 'format'"),
                        );
                    }
                }
            }
        }
    }
    vm.lua_pushlstring(l, &out);
    Ok(1)
}

// string.pack / packsize / unpack
const MAXINTSIZE: usize = 16;
const SZINT: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
enum KOption {
    Kint,
    Kuint,
    Kfloat,
    Knumber,
    Kdouble,
    Kchar,
    Kstring,
    Kzstr,
    Kpadding,
    Kpaddalign,
    Knop,
}

struct PackHeader {
    islittle: bool,
    maxalign: usize,
}

fn getnum(fmt: &[u8], idx: &mut usize, df: isize) -> isize {
    if !fmt.get(*idx).map_or(false, |&b| c_isdigit(b)) {
        df
    } else {
        let mut a: isize = 0;
        while let Some(&b) = fmt.get(*idx) {
            if c_isdigit(b) && a <= ((MAXSIZE as isize) - 9) / 10 {
                a = a * 10 + ((b - b'0') as isize);
                *idx += 1;
            } else {
                break;
            }
        }
        a
    }
}

fn getnumlimit(
    vm: &mut LuaVM,
    l: ThreadId,
    fmt: &[u8],
    idx: &mut usize,
    df: usize,
) -> LuaResult<usize> {
    let sz = getnum(fmt, idx, df as isize);
    if sz <= 0 || (sz as usize) > MAXINTSIZE {
        return vm.luaL_error_str(
            l,
            &format!("integral size ({sz}) out of limits [1,{MAXINTSIZE}]"),
        );
    }
    Ok(sz as usize)
}

fn getoption(
    vm: &mut LuaVM,
    l: ThreadId,
    h: &mut PackHeader,
    fmt: &[u8],
    idx: &mut usize,
) -> LuaResult<(KOption, usize)> {
    let opt = fmt.get(*idx).copied().unwrap_or(0);
    *idx += 1;
    match opt {
        b'b' => Ok((KOption::Kint, 1)),
        b'B' => Ok((KOption::Kuint, 1)),
        b'h' => Ok((KOption::Kint, 2)),
        b'H' => Ok((KOption::Kuint, 2)),
        b'l' => Ok((KOption::Kint, 8)),
        b'L' => Ok((KOption::Kuint, 8)),
        b'j' => Ok((KOption::Kint, 8)),
        b'J' => Ok((KOption::Kuint, 8)),
        b'T' => Ok((KOption::Kuint, 8)),
        b'f' => Ok((KOption::Kfloat, 4)),
        b'n' => Ok((KOption::Knumber, 8)),
        b'd' => Ok((KOption::Kdouble, 8)),
        b'i' => {
            let sz = getnumlimit(vm, l, fmt, idx, 4)?;
            Ok((KOption::Kint, sz))
        }
        b'I' => {
            let sz = getnumlimit(vm, l, fmt, idx, 4)?;
            Ok((KOption::Kuint, sz))
        }
        b's' => {
            let sz = getnumlimit(vm, l, fmt, idx, 8)?;
            Ok((KOption::Kstring, sz))
        }
        b'c' => {
            let sz = getnum(fmt, idx, -1);
            if sz == -1 {
                return vm.luaL_error_str(l, "missing size for format option 'c'");
            }
            Ok((KOption::Kchar, sz as usize))
        }
        b'z' => Ok((KOption::Kzstr, 0)),
        b'x' => Ok((KOption::Kpadding, 1)),
        b'X' => Ok((KOption::Kpaddalign, 0)),
        b' ' => Ok((KOption::Knop, 0)),
        b'<' => {
            h.islittle = true;
            Ok((KOption::Knop, 0))
        }
        b'>' => {
            h.islittle = false;
            Ok((KOption::Knop, 0))
        }
        b'=' => {
            h.islittle = true;
            Ok((KOption::Knop, 0))
        }
        b'!' => {
            h.maxalign = getnumlimit(vm, l, fmt, idx, 8)?;
            Ok((KOption::Knop, 0))
        }
        _ => vm.luaL_error_str(l, &format!("invalid format option '{}'", opt as char)),
    }
}

fn getdetails(
    vm: &mut LuaVM,
    l: ThreadId,
    h: &mut PackHeader,
    totalsize: usize,
    fmt: &[u8],
    idx: &mut usize,
) -> LuaResult<(KOption, usize, usize)> {
    let (opt, size) = getoption(vm, l, h, fmt, idx)?;
    let mut align = size;
    if opt == KOption::Kpaddalign {
        if *idx >= fmt.len() {
            return vm.luaL_argerror(l, 1, "invalid next option for option 'X'");
        }
        let (next_opt, next_align) = getoption(vm, l, h, fmt, idx)?;
        if next_opt == KOption::Kchar || next_align == 0 {
            return vm.luaL_argerror(l, 1, "invalid next option for option 'X'");
        }
        align = next_align;
    }
    let ntoalign = if align <= 1 || opt == KOption::Kchar {
        0
    } else {
        if align > h.maxalign {
            align = h.maxalign;
        }
        if (align & (align - 1)) != 0 {
            return vm.luaL_argerror(l, 1, "format asks for alignment not power of 2");
        }
        (align - (totalsize & (align - 1))) & (align - 1)
    };
    Ok((opt, size, ntoalign))
}

fn packint(b: &mut Vec<u8>, mut n: LuaUnsigned, islittle: bool, size: usize, neg: bool) {
    let start = b.len();
    b.resize(start + size, 0);
    if let Some(slot) = b.get_mut(if islittle { start } else { start + size - 1 }) {
        *slot = (n & 0xFF) as u8;
    }
    for i in 1..size {
        n >>= 8;
        let pos = if islittle {
            start + i
        } else {
            start + size - 1 - i
        };
        if let Some(slot) = b.get_mut(pos) {
            *slot = (n & 0xFF) as u8;
        }
    }
    if neg && size > SZINT {
        for i in SZINT..size {
            let pos = if islittle {
                start + i
            } else {
                start + size - 1 - i
            };
            if let Some(slot) = b.get_mut(pos) {
                *slot = 0xFF;
            }
        }
    }
}

fn str_pack(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let fmt_raw = vm.luaL_checklstring(l, 1)?;
    let fmt_nul = fmt_raw
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(fmt_raw.len());
    let fmt = fmt_raw.get(..fmt_nul).unwrap_or(&[]).to_vec();
    let mut h = PackHeader {
        islittle: true,
        maxalign: 1,
    };
    let mut arg = 1;
    let mut totalsize: usize = 0;
    let mut out = Vec::new();
    let mut idx = 0;
    while idx < fmt.len() {
        let (opt, size, ntoalign) = getdetails(vm, l, &mut h, totalsize, &fmt, &mut idx)?;
        totalsize += ntoalign + size;
        out.extend(std::iter::repeat(0u8).take(ntoalign));
        arg += 1;
        match opt {
            KOption::Kint => {
                let n = vm.luaL_checkinteger(l, arg)?;
                if size < SZINT {
                    let lim = 1i64 << (size * 8 - 1);
                    if !(-lim <= n && n < lim) {
                        return vm.luaL_argerror(l, arg, "integer overflow");
                    }
                }
                packint(&mut out, n as LuaUnsigned, h.islittle, size, n < 0);
            }
            KOption::Kuint => {
                let n = vm.luaL_checkinteger(l, arg)?;
                if size < SZINT && (n as LuaUnsigned) >= (1u64 << (size * 8)) {
                    return vm.luaL_argerror(l, arg, "unsigned overflow");
                }
                packint(&mut out, n as LuaUnsigned, h.islittle, size, false);
            }
            KOption::Kfloat => {
                let f = vm.luaL_checknumber(l, arg)? as f32;
                let bytes = if h.islittle {
                    f.to_le_bytes()
                } else {
                    f.to_be_bytes()
                };
                out.extend_from_slice(&bytes);
            }
            KOption::Knumber | KOption::Kdouble => {
                let f = vm.luaL_checknumber(l, arg)?;
                let bytes = if h.islittle {
                    f.to_le_bytes()
                } else {
                    f.to_be_bytes()
                };
                out.extend_from_slice(&bytes);
            }
            KOption::Kchar => {
                let s = vm.luaL_checklstring(l, arg)?;
                if s.len() > size {
                    return vm.luaL_argerror(l, arg, "string longer than given size");
                }
                out.extend_from_slice(&s);
                if s.len() < size {
                    out.extend(std::iter::repeat(0u8).take(size - s.len()));
                }
            }
            KOption::Kstring => {
                let s = vm.luaL_checklstring(l, arg)?;
                if size < 8 && s.len() >= (1usize << (size * 8)) {
                    return vm.luaL_argerror(l, arg, "string length does not fit in given size");
                }
                packint(&mut out, s.len() as LuaUnsigned, h.islittle, size, false);
                out.extend_from_slice(&s);
                totalsize += s.len();
            }
            KOption::Kzstr => {
                let s = vm.luaL_checklstring(l, arg)?;
                if s.contains(&0) {
                    return vm.luaL_argerror(l, arg, "string contains zeros");
                }
                out.extend_from_slice(&s);
                out.push(0);
                totalsize += s.len() + 1;
            }
            KOption::Kpadding => {
                out.push(0);
                arg -= 1;
            }
            KOption::Kpaddalign | KOption::Knop => {
                arg -= 1;
            }
        }
    }
    vm.lua_pushlstring(l, &out);
    Ok(1)
}

fn str_packsize(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let fmt_raw = vm.luaL_checklstring(l, 1)?;
    let fmt_nul = fmt_raw
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(fmt_raw.len());
    let fmt = fmt_raw.get(..fmt_nul).unwrap_or(&[]).to_vec();
    let mut h = PackHeader {
        islittle: true,
        maxalign: 1,
    };
    let mut totalsize: usize = 0;
    let mut idx = 0;
    while idx < fmt.len() {
        let (opt, mut size, ntoalign) = getdetails(vm, l, &mut h, totalsize, &fmt, &mut idx)?;
        if opt == KOption::Kstring || opt == KOption::Kzstr {
            return vm.luaL_argerror(l, 1, "variable-length format");
        }
        size += ntoalign;
        if totalsize > MAXSIZE - size {
            return vm.luaL_argerror(l, 1, "format result too large");
        }
        totalsize += size;
    }
    vm.lua_pushinteger(l, totalsize as LuaInteger);
    Ok(1)
}

fn unpackint(
    vm: &mut LuaVM,
    l: ThreadId,
    str_slice: &[u8],
    islittle: bool,
    size: usize,
    issigned: bool,
) -> LuaResult<LuaInteger> {
    let mut res: LuaUnsigned = 0;
    let limit = size.min(SZINT);
    let mut i = limit;
    while i > 0 {
        i -= 1;
        res <<= 8;
        let idx = if islittle { i } else { size - 1 - i };
        res |= str_slice.get(idx).copied().unwrap_or(0) as LuaUnsigned;
    }
    if size < SZINT {
        if issigned {
            let mask: LuaUnsigned = 1u64 << (size * 8 - 1);
            res = (res ^ mask).wrapping_sub(mask);
        }
    } else if size > SZINT {
        let mask: u8 = if !issigned || (res as LuaInteger) >= 0 {
            0
        } else {
            0xFF
        };
        for j in limit..size {
            let idx = if islittle { j } else { size - 1 - j };
            if str_slice.get(idx).copied().unwrap_or(0) != mask {
                return vm.luaL_error_str(
                    l,
                    &format!("{size}-byte integer does not fit into Lua Integer"),
                );
            }
        }
    }
    Ok(res as LuaInteger)
}

fn str_unpack(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let fmt_raw = vm.luaL_checklstring(l, 1)?;
    let fmt_nul = fmt_raw
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(fmt_raw.len());
    let fmt = fmt_raw.get(..fmt_nul).unwrap_or(&[]).to_vec();
    let data = vm.luaL_checklstring(l, 2)?;
    let pos_arg = vm.luaL_optinteger(l, 3, 1)?;
    let mut pos = posrelatI(pos_arg, data.len()) - 1;
    if pos > data.len() {
        return vm.luaL_argerror(l, 3, "initial position out of string");
    }
    let mut h = PackHeader {
        islittle: true,
        maxalign: 1,
    };
    let mut n: i32 = 0;
    let mut idx = 0;
    while idx < fmt.len() {
        let (opt, size, ntoalign) = getdetails(vm, l, &mut h, pos, &fmt, &mut idx)?;
        if ntoalign + size > data.len() - pos {
            return vm.luaL_argerror(l, 2, "data string too short");
        }
        pos += ntoalign;
        vm.luaL_checkstack(l, 2, Some("too many results"))?;
        n += 1;
        match opt {
            KOption::Kint | KOption::Kuint => {
                let slice = data.get(pos..(pos + size)).unwrap_or(&[]);
                let val = unpackint(vm, l, slice, h.islittle, size, opt == KOption::Kint)?;
                vm.lua_pushinteger(l, val);
            }
            KOption::Kfloat => {
                let mut arr = [0u8; 4];
                if let Some(s) = data.get(pos..(pos + 4)) {
                    arr.copy_from_slice(s);
                }
                let f = if h.islittle {
                    f32::from_le_bytes(arr)
                } else {
                    f32::from_be_bytes(arr)
                };
                vm.lua_pushnumber(l, f as LuaNumber);
            }
            KOption::Knumber | KOption::Kdouble => {
                let mut arr = [0u8; 8];
                if let Some(s) = data.get(pos..(pos + 8)) {
                    arr.copy_from_slice(s);
                }
                let f = if h.islittle {
                    f64::from_le_bytes(arr)
                } else {
                    f64::from_be_bytes(arr)
                };
                vm.lua_pushnumber(l, f);
            }
            KOption::Kchar => {
                let slice = data.get(pos..(pos + size)).unwrap_or(&[]);
                vm.lua_pushlstring(l, slice);
            }
            KOption::Kstring => {
                let slice = data.get(pos..(pos + size)).unwrap_or(&[]);
                let len = unpackint(vm, l, slice, h.islittle, size, false)? as usize;
                if len > data.len() - pos - size {
                    return vm.luaL_argerror(l, 2, "data string too short");
                }
                let str_slice = data.get((pos + size)..(pos + size + len)).unwrap_or(&[]);
                vm.lua_pushlstring(l, str_slice);
                pos += len;
            }
            KOption::Kzstr => {
                let rest = data.get(pos..).unwrap_or(&[]);
                let Some(zlen) = rest.iter().position(|&b| b == 0) else {
                    return vm.luaL_argerror(l, 2, "unfinished string for format 'z'");
                };
                let str_slice = rest.get(..zlen).unwrap_or(&[]);
                vm.lua_pushlstring(l, str_slice);
                pos += zlen + 1;
            }
            KOption::Kpaddalign | KOption::Kpadding | KOption::Knop => {
                n -= 1;
            }
        }
        pos += size;
    }
    vm.lua_pushinteger(l, (pos + 1) as LuaInteger);
    Ok(n + 1)
}

pub fn luaopen_string(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static STRLIB: [(&str, Option<LuaCFunction>); 16] = [
        ("byte", Some(str_byte)),
        ("char", Some(str_char)),
        ("find", Some(str_find)),
        ("format", Some(str_format)),
        ("gmatch", Some(str_gmatch)),
        ("gsub", Some(str_gsub)),
        ("len", Some(str_len)),
        ("lower", Some(str_lower)),
        ("match", Some(str_match)),
        ("rep", Some(str_rep)),
        ("reverse", Some(str_reverse)),
        ("sub", Some(str_sub)),
        ("upper", Some(str_upper)),
        ("pack", Some(str_pack)),
        ("packsize", Some(str_packsize)),
        ("unpack", Some(str_unpack)),
    ];
    static STRING_METAMETHODS: [(&str, Option<LuaCFunction>); 9] = [
        ("__add", Some(arith_add)),
        ("__sub", Some(arith_sub)),
        ("__mul", Some(arith_mul)),
        ("__mod", Some(arith_mod)),
        ("__pow", Some(arith_pow)),
        ("__div", Some(arith_div)),
        ("__idiv", Some(arith_idiv)),
        ("__unm", Some(arith_unm)),
        ("__index", None),
    ];
    vm.lua_createtable(l, 0, STRLIB.len() as u32)?;
    vm.luaL_setfuncs(l, &STRLIB, 0)?;

    vm.lua_createtable(l, 0, STRING_METAMETHODS.len() as u32)?;
    vm.luaL_setfuncs(l, &STRING_METAMETHODS, 0)?;
    vm.lua_pushlstring(l, b"");
    vm.lua_pushvalue(l, -2);
    vm.lua_setmetatable(l, -2);
    vm.lua_pop(l, 1);
    vm.lua_pushvalue(l, -2);
    vm.lua_setfield(l, -2, "__index")?;
    vm.lua_pop(l, 1);
    Ok(1)
}

// ============================================================================
// Math Library (lmathlib.c)
// ============================================================================

const PI: LuaNumber = std::f64::consts::PI;

fn math_abs(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_isinteger(l, 1) {
        let mut n = vm.lua_tointegerx(l, 1).unwrap_or(0);
        if n < 0 {
            n = 0u64.wrapping_sub(n as u64) as LuaInteger;
        }
        vm.lua_pushinteger(l, n);
    } else {
        let d = vm.luaL_checknumber(l, 1)?;
        vm.lua_pushnumber(l, d.abs());
    }
    Ok(1)
}

fn math_sin(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.sin());
    Ok(1)
}

fn math_cos(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.cos());
    Ok(1)
}

fn math_tan(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.tan());
    Ok(1)
}

fn math_asin(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.asin());
    Ok(1)
}

fn math_acos(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.acos());
    Ok(1)
}

fn math_atan(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let y = vm.luaL_checknumber(l, 1)?;
    let x = vm.luaL_optnumber(l, 2, 1.0)?;
    vm.lua_pushnumber(l, y.atan2(x));
    Ok(1)
}

fn math_toint(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if let Some(n) = vm.lua_tointegerx(l, 1) {
        vm.lua_pushinteger(l, n);
    } else {
        vm.luaL_checkany(l, 1)?;
        vm.lua_pushnil(l);
    }
    Ok(1)
}

fn pushnumint(vm: &mut LuaVM, l: ThreadId, d: LuaNumber) {
    if let Some(n) = lua_numbertointeger(d) {
        vm.lua_pushinteger(l, n);
    } else {
        vm.lua_pushnumber(l, d);
    }
}

fn math_floor(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_isinteger(l, 1) {
        vm.lua_settop(l, 1)?;
    } else {
        let d = vm.luaL_checknumber(l, 1)?.floor();
        pushnumint(vm, l, d);
    }
    Ok(1)
}

fn math_ceil(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_isinteger(l, 1) {
        vm.lua_settop(l, 1)?;
    } else {
        let d = vm.luaL_checknumber(l, 1)?.ceil();
        pushnumint(vm, l, d);
    }
    Ok(1)
}

fn math_fmod(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_isinteger(l, 1) && vm.lua_isinteger(l, 2) {
        let d = vm.lua_tointegerx(l, 2).unwrap_or(0);
        if (d as LuaUnsigned).wrapping_add(1) <= 1 {
            if d == 0 {
                return vm.luaL_argerror(l, 2, "zero");
            }
            vm.lua_pushinteger(l, 0);
        } else {
            let n = vm.lua_tointegerx(l, 1).unwrap_or(0);
            vm.lua_pushinteger(l, n % d);
        }
    } else {
        let x = vm.luaL_checknumber(l, 1)?;
        let y = vm.luaL_checknumber(l, 2)?;
        vm.lua_pushnumber(l, x % y);
    }
    Ok(1)
}

fn math_modf(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_isinteger(l, 1) {
        vm.lua_settop(l, 1)?;
        vm.lua_pushnumber(l, 0.0);
    } else {
        let n = vm.luaL_checknumber(l, 1)?;
        let ip = if n < 0.0 { n.ceil() } else { n.floor() };
        pushnumint(vm, l, ip);
        vm.lua_pushnumber(l, if n == ip { 0.0 } else { n - ip });
    }
    Ok(2)
}

fn math_sqrt(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.sqrt());
    Ok(1)
}

fn math_ult(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let a = vm.luaL_checkinteger(l, 1)?;
    let b = vm.luaL_checkinteger(l, 2)?;
    vm.lua_pushboolean(l, (a as LuaUnsigned) < (b as LuaUnsigned));
    Ok(1)
}

fn math_log(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let x = vm.luaL_checknumber(l, 1)?;
    let res = if vm.lua_isnoneornil(l, 2) {
        x.ln()
    } else {
        let base = vm.luaL_checknumber(l, 2)?;
        if base == 2.0 {
            x.log2()
        } else if base == 10.0 {
            x.log10()
        } else {
            x.ln() / base.ln()
        }
    };
    vm.lua_pushnumber(l, res);
    Ok(1)
}

fn math_exp(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d.exp());
    Ok(1)
}

fn math_deg(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d * (180.0 / PI));
    Ok(1)
}

fn math_rad(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let d = vm.luaL_checknumber(l, 1)?;
    vm.lua_pushnumber(l, d * (PI / 180.0));
    Ok(1)
}

fn math_min(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    if n < 1 {
        return vm.luaL_argerror(l, 1, "value expected");
    }
    let mut imin = 1;
    for i in 2..=n {
        if vm.lua_compare(l, i, imin, LUA_OPLT)? {
            imin = i;
        }
    }
    vm.lua_pushvalue(l, imin);
    Ok(1)
}

fn math_max(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    if n < 1 {
        return vm.luaL_argerror(l, 1, "value expected");
    }
    let mut imax = 1;
    for i in 2..=n {
        if vm.lua_compare(l, imax, i, LUA_OPLT)? {
            imax = i;
        }
    }
    vm.lua_pushvalue(l, imax);
    Ok(1)
}

fn math_type(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    if vm.lua_type(l, 1) == LUA_TNUMBER {
        if vm.lua_isinteger(l, 1) {
            vm.lua_pushstring(l, "integer");
        } else {
            vm.lua_pushstring(l, "float");
        }
    } else {
        vm.luaL_checkany(l, 1)?;
        vm.lua_pushnil(l);
    }
    Ok(1)
}

// xoshiro256** PRNG
fn nextrand(state: &mut [u64; 4]) -> u64 {
    let state0 = state.first().copied().unwrap_or(0);
    let state1 = state.get(1).copied().unwrap_or(0);
    let state2 = state.get(2).copied().unwrap_or(0) ^ state0;
    let state3 = state.get(3).copied().unwrap_or(0) ^ state1;
    let res = state1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
    if let Some(s0) = state.get_mut(0) {
        *s0 = state0 ^ state3;
    }
    if let Some(s1) = state.get_mut(1) {
        *s1 = state1 ^ state2;
    }
    if let Some(s2) = state.get_mut(2) {
        *s2 = state2 ^ (state1 << 17);
    }
    if let Some(s3) = state.get_mut(3) {
        *s3 = state3.rotate_left(45);
    }
    res
}

fn i2d(x: u64) -> LuaNumber {
    const FIGS: u32 = 53;
    const SHIFT: u32 = 64 - FIGS;
    let scale: LuaNumber = 0.5 / ((1u64 << (FIGS - 1)) as LuaNumber);
    let sx = (x >> SHIFT) as i64;
    (sx as LuaNumber) * scale
}

fn project(mut ran: u64, n: u64, state: &mut [u64; 4]) -> u64 {
    if (n & n.wrapping_add(1)) == 0 {
        ran & n
    } else {
        let mut lim = n;
        lim |= lim >> 1;
        lim |= lim >> 2;
        lim |= lim >> 4;
        lim |= lim >> 8;
        lim |= lim >> 16;
        lim |= lim >> 32;
        loop {
            ran &= lim;
            if ran <= n {
                return ran;
            }
            ran = nextrand(state);
        }
    }
}

fn setseed_state(state: &mut [u64; 4], n1: u64, n2: u64) {
    if let Some(s0) = state.get_mut(0) {
        *s0 = n1;
    }
    if let Some(s1) = state.get_mut(1) {
        *s1 = 0xff;
    }
    if let Some(s2) = state.get_mut(2) {
        *s2 = n2;
    }
    if let Some(s3) = state.get_mut(3) {
        *s3 = 0;
    }
    for _ in 0..16 {
        nextrand(state);
    }
}

fn math_random(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let (u_val, _) = vm.index2value(l, lua_upvalueindex(1));
    let u_id = u_val.uvalue();
    let mut state = match vm.udatas.get(u_id).and_then(|u| match u.payload {
        UdataPayload::RanState(s) => Some(s),
        _ => None,
    }) {
        Some(s) => s,
        None => [0; 4],
    };
    let rv = nextrand(&mut state);
    if let Some(u) = vm.udatas.get_mut(u_id) {
        u.payload = UdataPayload::RanState(state);
    }
    let (low, up) = match vm.lua_gettop(l) {
        0 => {
            vm.lua_pushnumber(l, i2d(rv));
            return Ok(1);
        }
        1 => {
            let u = vm.luaL_checkinteger(l, 1)?;
            if u == 0 {
                vm.lua_pushinteger(l, rv as LuaInteger);
                return Ok(1);
            }
            (1, u)
        }
        2 => {
            let lo = vm.luaL_checkinteger(l, 1)?;
            let u = vm.luaL_checkinteger(l, 2)?;
            (lo, u)
        }
        _ => return vm.luaL_error_str(l, "wrong number of arguments"),
    };
    if low > up {
        return vm.luaL_argerror(l, 1, "interval is empty");
    }
    let diff = (up as u64).wrapping_sub(low as u64);
    let p = project(rv, diff, &mut state);
    if let Some(u) = vm.udatas.get_mut(u_id) {
        u.payload = UdataPayload::RanState(state);
    }
    vm.lua_pushinteger(l, p.wrapping_add(low as u64) as LuaInteger);
    Ok(1)
}

fn math_randomseed(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let (u_val, _) = vm.index2value(l, lua_upvalueindex(1));
    let u_id = u_val.uvalue();
    let (n1, n2) = if vm.lua_isnone(l, 1) {
        (42u64, 0u64)
    } else {
        let a = vm.luaL_checkinteger(l, 1)? as u64;
        let b = vm.luaL_optinteger(l, 2, 0)? as u64;
        (a, b)
    };
    let mut state = [0u64; 4];
    setseed_state(&mut state, n1, n2);
    if let Some(u) = vm.udatas.get_mut(u_id) {
        u.payload = UdataPayload::RanState(state);
    }
    vm.lua_pushinteger(l, n1 as LuaInteger);
    vm.lua_pushinteger(l, n2 as LuaInteger);
    Ok(2)
}

pub fn luaopen_math(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static MATHLIB: [(&str, Option<LuaCFunction>); 27] = [
        ("abs", Some(math_abs)),
        ("acos", Some(math_acos)),
        ("asin", Some(math_asin)),
        ("atan", Some(math_atan)),
        ("ceil", Some(math_ceil)),
        ("cos", Some(math_cos)),
        ("deg", Some(math_deg)),
        ("exp", Some(math_exp)),
        ("tointeger", Some(math_toint)),
        ("floor", Some(math_floor)),
        ("fmod", Some(math_fmod)),
        ("ult", Some(math_ult)),
        ("log", Some(math_log)),
        ("max", Some(math_max)),
        ("min", Some(math_min)),
        ("modf", Some(math_modf)),
        ("rad", Some(math_rad)),
        ("sin", Some(math_sin)),
        ("sqrt", Some(math_sqrt)),
        ("tan", Some(math_tan)),
        ("type", Some(math_type)),
        ("random", None),
        ("randomseed", None),
        ("pi", None),
        ("huge", None),
        ("maxinteger", None),
        ("mininteger", None),
    ];
    static RANDFUNCS: [(&str, Option<LuaCFunction>); 2] = [
        ("random", Some(math_random)),
        ("randomseed", Some(math_randomseed)),
    ];
    vm.lua_createtable(l, 0, MATHLIB.len() as u32)?;
    vm.luaL_setfuncs(l, &MATHLIB, 0)?;
    vm.lua_pushnumber(l, PI);
    vm.lua_setfield(l, -2, "pi")?;
    vm.lua_pushnumber(l, f64::INFINITY);
    vm.lua_setfield(l, -2, "huge")?;
    vm.lua_pushinteger(l, LuaInteger::MAX);
    vm.lua_setfield(l, -2, "maxinteger")?;
    vm.lua_pushinteger(l, LuaInteger::MIN);
    vm.lua_setfield(l, -2, "mininteger")?;

    let mut init_state = [0u64; 4];
    setseed_state(&mut init_state, 42, 0);
    let u_id = vm.udatas.len();
    vm.udatas.push(Udata {
        nuvalue: 0,
        len: 32,
        metatable: None,
        uv: Vec::new(),
        payload: UdataPayload::RanState(init_state),
    });
    let top = vm.get_top(l);
    vm.stack_set(l, top, TValue::udata_val(u_id));
    vm.set_top_raw(l, top + 1);
    vm.luaL_setfuncs(l, &RANDFUNCS, 1)?;
    Ok(1)
}

// ============================================================================
// UTF-8 Library (lutf8lib.c)
// ============================================================================

const MAXUNICODE: u32 = 0x10FFFF;
const MAXUTF: u32 = 0x7FFFFFFF;
const MSGINVALID: &str = "invalid UTF-8 code";

const fn iscont(c: u8) -> bool {
    (c & 0xC0) == 0x80
}

fn s_byte_at(s: &[u8], idx: usize) -> u8 {
    s.get(idx).copied().unwrap_or(0)
}

fn u_posrelat(pos: LuaInteger, len: usize) -> LuaInteger {
    if pos >= 0 {
        pos
    } else if 0u64.wrapping_sub(pos as u64) > (len as u64) {
        0
    } else {
        (len as LuaInteger) + pos + 1
    }
}

fn utf8_decode(s: &[u8], start: usize, strict: bool) -> Option<(usize, u32)> {
    static LIMITS: [u32; 6] = [!0u32, 0x80, 0x800, 0x10000, 0x200000, 0x4000000];
    let mut c = s_byte_at(s, start) as u32;
    let mut res: u32 = 0;
    let mut end_pos = start;
    if c < 0x80 {
        res = c;
    } else {
        let mut count: usize = 0;
        while (c & 0x40) != 0 {
            count += 1;
            let cc = s_byte_at(s, start + count);
            if !iscont(cc) {
                return None;
            }
            res = (res << 6) | ((cc & 0x3F) as u32);
            c <<= 1;
        }
        res |= (c & 0x7F) << (count * 5);
        if count > 5
            || res > MAXUTF
            || res < LIMITS.get(count).copied().unwrap_or(!0u32)
        {
            return None;
        }
        end_pos += count;
    }
    if strict && (res > MAXUNICODE || (0xD800..=0xDFFF).contains(&res)) {
        return None;
    }
    Some((end_pos + 1, res))
}

fn utflen(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let len = s.len();
    let posi_arg = vm.luaL_optinteger(l, 2, 1)?;
    let mut posi = u_posrelat(posi_arg, len);
    let posj_arg = vm.luaL_optinteger(l, 3, -1)?;
    let mut posj = u_posrelat(posj_arg, len);
    let lax = vm.lua_toboolean(l, 4);
    if !(1 <= posi && {
        posi -= 1;
        posi <= (len as LuaInteger)
    }) {
        return vm.luaL_argerror(l, 2, "initial position out of bounds");
    }
    posj -= 1;
    if posj >= (len as LuaInteger) {
        return vm.luaL_argerror(l, 3, "final position out of bounds");
    }
    let mut n: LuaInteger = 0;
    while posi <= posj {
        match utf8_decode(&s, posi as usize, !lax) {
            Some((next_pos, _)) => {
                posi = next_pos as LuaInteger;
                n += 1;
            }
            None => {
                vm.lua_pushnil(l);
                vm.lua_pushinteger(l, posi + 1);
                return Ok(2);
            }
        }
    }
    vm.lua_pushinteger(l, n);
    Ok(1)
}

fn codepoint(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let len = s.len();
    let posi_arg = vm.luaL_optinteger(l, 2, 1)?;
    let posi = u_posrelat(posi_arg, len);
    let pose_arg = vm.luaL_optinteger(l, 3, posi)?;
    let pose = u_posrelat(pose_arg, len);
    let lax = vm.lua_toboolean(l, 4);
    if posi < 1 {
        return vm.luaL_argerror(l, 2, "out of bounds");
    }
    if pose > (len as LuaInteger) {
        return vm.luaL_argerror(l, 3, "out of bounds");
    }
    if posi > pose {
        return Ok(0);
    }
    if pose - posi >= (i32::MAX as LuaInteger) {
        return vm.luaL_error_str(l, "string slice too long");
    }
    let n_est = ((pose - posi) + 1) as usize;
    vm.luaL_checkstack(l, n_est, Some("string slice too long"))?;
    let mut n: i32 = 0;
    let se = pose as usize;
    let mut cur = (posi - 1) as usize;
    while cur < se {
        match utf8_decode(&s, cur, !lax) {
            Some((next_pos, code)) => {
                cur = next_pos;
                vm.lua_pushinteger(l, code as LuaInteger);
                n += 1;
            }
            None => return vm.luaL_error_str(l, MSGINVALID),
        }
    }
    Ok(n)
}

fn utfchar(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let n = vm.lua_gettop(l);
    let mut out = Vec::new();
    for i in 1..=n {
        let code = vm.luaL_checkinteger(l, i)? as LuaUnsigned;
        if code > (MAXUTF as LuaUnsigned) {
            return vm.luaL_argerror(l, i, "value out of range");
        }
        out.extend_from_slice(&luaO_utf8esc(code));
    }
    vm.lua_pushlstring(l, &out);
    Ok(1)
}

fn byteoffset(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let len = s.len();
    let mut n = vm.luaL_checkinteger(l, 2)?;
    let def_posi = if n >= 0 {
        1
    } else {
        (len as LuaInteger) + 1
    };
    let posi_arg = vm.luaL_optinteger(l, 3, def_posi)?;
    let mut posi = u_posrelat(posi_arg, len);
    if !(1 <= posi && {
        posi -= 1;
        posi <= (len as LuaInteger)
    }) {
        return vm.luaL_argerror(l, 3, "position out of bounds");
    }
    if n == 0 {
        while posi > 0 && iscont(s_byte_at(&s, posi as usize)) {
            posi -= 1;
        }
    } else {
        if iscont(s_byte_at(&s, posi as usize)) {
            return vm.luaL_error_str(l, "initial position is a continuation byte");
        }
        if n < 0 {
            while n < 0 && posi > 0 {
                loop {
                    posi -= 1;
                    if !(posi > 0 && iscont(s_byte_at(&s, posi as usize))) {
                        break;
                    }
                }
                n += 1;
            }
        } else {
            n -= 1;
            while n > 0 && posi < (len as LuaInteger) {
                loop {
                    posi += 1;
                    if !iscont(s_byte_at(&s, posi as usize)) {
                        break;
                    }
                }
                n -= 1;
            }
        }
    }
    if n == 0 {
        vm.lua_pushinteger(l, posi + 1);
    } else {
        vm.lua_pushnil(l);
    }
    Ok(1)
}

fn iter_aux(vm: &mut LuaVM, l: ThreadId, strict: bool) -> LuaResult<i32> {
    let s = vm.luaL_checklstring(l, 1)?;
    let len = s.len();
    let mut n = vm.lua_tointegerx(l, 2).unwrap_or(0) as LuaUnsigned;
    if n < (len as LuaUnsigned) {
        while iscont(s_byte_at(&s, n as usize)) {
            n += 1;
        }
    }
    if n >= (len as LuaUnsigned) {
        Ok(0)
    } else {
        match utf8_decode(&s, n as usize, strict) {
            Some((next_pos, code)) if !iscont(s_byte_at(&s, next_pos)) => {
                vm.lua_pushinteger(l, (n + 1) as LuaInteger);
                vm.lua_pushinteger(l, code as LuaInteger);
                Ok(2)
            }
            _ => vm.luaL_error_str(l, MSGINVALID),
        }
    }
}

fn iter_auxstrict(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    iter_aux(vm, l, true)
}

fn iter_auxlax(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    iter_aux(vm, l, false)
}

fn iter_codes(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    let lax = vm.lua_toboolean(l, 2);
    let s = vm.luaL_checklstring(l, 1)?;
    if iscont(s_byte_at(&s, 0)) {
        return vm.luaL_argerror(l, 1, MSGINVALID);
    }
    vm.lua_pushcfunction(l, if lax { iter_auxlax } else { iter_auxstrict });
    vm.lua_pushvalue(l, 1);
    vm.lua_pushinteger(l, 0);
    Ok(3)
}

pub fn luaopen_utf8(vm: &mut LuaVM, l: ThreadId) -> LuaResult<i32> {
    static UTF8_FUNCS: [(&str, Option<LuaCFunction>); 6] = [
        ("offset", Some(byteoffset)),
        ("codepoint", Some(codepoint)),
        ("char", Some(utfchar)),
        ("len", Some(utflen)),
        ("codes", Some(iter_codes)),
        ("charpattern", None),
    ];
    const UTF8PATT: &[u8] = b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*";
    vm.lua_createtable(l, 0, UTF8_FUNCS.len() as u32)?;
    vm.luaL_setfuncs(l, &UTF8_FUNCS, 0)?;
    vm.lua_pushlstring(l, UTF8PATT);
    vm.lua_setfield(l, -2, "charpattern")?;
    Ok(1)
}

// ============================================================================
// VM Initialization & Entrypoint (lstate.c & lua_driver.c)
// ============================================================================

impl LuaVM {
    pub fn new() -> LuaResult<Self> {
        let mut main_th = LuaThread::new();
        main_th.inc_nny();

        let mut vm = LuaVM {
            strings: Vec::new(),
            short_str_map: HashMap::new(),
            tables: Vec::new(),
            protos: Vec::new(),
            lclosures: Vec::new(),
            cclosures: Vec::new(),
            upvals: Vec::new(),
            udatas: Vec::new(),
            threads: vec![main_th],
            output: Vec::new(),
            err_output: Vec::new(),
            seed: 0,
            l_registry: TValue::nil(),
            nilvalue: TValue::nil(),
            memerrmsg: 0,
            envn: 0,
            name_str: 0,
            brk_str: 0,
            tmname: [0; TM_N],
            mt: [None; LUA_NUMTYPES],
            mainthread: 0,
            gcstp: 0,
            gckind: LUA_GCINC,
            gcpause: 200,
            gcstepmul: 100,
            gcstepsize: 13,
            genminormul: 20,
            genmajormul: 100,
            warn_state: WarnState::Off,
        };

        // Initialize registry
        let reg_id = vm.luaH_new();
        vm.l_registry = TValue::table_val(reg_id);
        vm.luaH_resize(0, reg_id, LUA_RIDX_LAST as u32, 0)?;
        let g_id = vm.luaH_new();
        if let Some(reg) = vm.tables.get_mut(reg_id) {
            if let Some(s0) = reg.array.get_mut((LUA_RIDX_MAINTHREAD - 1) as usize) {
                *s0 = TValue::thread_val(0);
            }
            if let Some(s1) = reg.array.get_mut((LUA_RIDX_GLOBALS - 1) as usize) {
                *s1 = TValue::table_val(g_id);
            }
        }

        // luaS_init
        vm.memerrmsg = vm.luaS_new("not enough memory");
        vm.name_str = vm.luaS_new("__name");
        vm.brk_str = vm.luaS_new("break");

        // luaT_init
        static EVENT_NAMES: [&str; TM_N] = [
            "__index",
            "__newindex",
            "__gc",
            "__mode",
            "__len",
            "__eq",
            "__add",
            "__sub",
            "__mul",
            "__mod",
            "__pow",
            "__div",
            "__idiv",
            "__band",
            "__bor",
            "__bxor",
            "__shl",
            "__shr",
            "__unm",
            "__bnot",
            "__lt",
            "__le",
            "__concat",
            "__call",
            "__close",
        ];
        for (i, &ename) in EVENT_NAMES.iter().enumerate() {
            let s_id = vm.luaS_new(ename);
            if let Some(slot) = vm.tmname.get_mut(i) {
                *slot = s_id;
            }
        }

        // luaX_init
        vm.envn = vm.luaS_new("_ENV");
        for i in 0..NUM_RESERVED {
            if let Some(&tok_s) = LUAX_TOKENS.get(i) {
                let ts_id = vm.luaS_new(tok_s);
                if let Some(ts) = vm.strings.get_mut(ts_id) {
                    ts.extra = (i + 1) as u8;
                }
            }
        }

        Ok(vm)
    }

    pub fn open_driver_libs(&mut self) -> LuaResult<()> {
        let l = self.mainthread;
        static LIBS: [(&str, LuaCFunction); 6] = [
            ("_G", luaopen_base),
            ("coroutine", luaopen_coroutine),
            ("table", luaopen_table),
            ("string", luaopen_string),
            ("math", luaopen_math),
            ("utf8", luaopen_utf8),
        ];
        for &(name, func) in &LIBS {
            self.luaL_requiref(l, name, func, true)?;
            self.lua_pop(l, 1);
        }
        self.lua_pushnil(l);
        self.lua_setglobal(l, "dofile")?;
        self.lua_pushnil(l);
        self.lua_setglobal(l, "loadfile")?;
        self.lua_pushnil(l);
        self.lua_setglobal(l, "require")?;

        // math.randomseed(42)
        self.lua_getglobal(l, "math")?;
        self.lua_getfield(l, -1, "randomseed")?;
        self.lua_pushinteger(l, 42);
        self.lua_call(l, 1, 0)?;
        self.lua_pop(l, 1);
        Ok(())
    }
}

pub fn run(script: &[u8]) -> Vec<u8> {
    let mut vm = match LuaVM::new() {
        Ok(v) => v,
        Err(_) => return b"ERROR failed to initialize state\n".to_vec(),
    };
    let l = vm.mainthread;
    if vm.open_driver_libs().is_err() {
        return b"ERROR failed to open libraries\n".to_vec();
    }

    let mut st = vm.lua_load(
        l,
        InputSource::Buffer {
            bytes: script.to_vec(),
            pos: 0,
        },
        b"=input",
        Some(b"t"),
    );
    if st == LUA_OK {
        st = vm.lua_pcall(l, 0, 0, 0).unwrap_or(LUA_ERRRUN);
    }
    if st != LUA_OK {
        let t = vm.lua_type(l, -1);
        if t == LUA_TSTRING || t == LUA_TNUMBER {
            let msg = vm.lua_tolstring(l, -1).ok().flatten().unwrap_or_default();
            let nul_pos = msg.iter().position(|&b| b == 0).unwrap_or(msg.len());
            vm.output.extend_from_slice(b"ERROR ");
            vm.output
                .extend_from_slice(msg.get(..nul_pos).unwrap_or(&[]));
            vm.output.push(b'\n');
        } else {
            let tname = vm.luaL_typename(l, -1);
            vm.output
                .extend_from_slice(format!("ERROR (error object is a {tname} value)\n").as_bytes());
        }
    }
    let _ = vm.luaD_closeprotected(l, 1, LUA_OK);
    vm.output
}
