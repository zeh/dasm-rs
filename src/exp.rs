use libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut Mnext: libc::c_int;
    #[no_mangle]
    static mut Xdebug: bool;
    #[no_mangle]
    static mut Redo_why: libc::c_ulong;
    #[no_mangle]
    static mut Redo: libc::c_int;
    #[no_mangle]
    static mut Redo_eval: libc::c_int;
    #[no_mangle]
    static mut F_listfile: *mut libc::c_char;
    #[no_mangle]
    static mut FI_listfile: *mut FILE;
    #[no_mangle]
    fn asmerr(err: libc::c_int, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn ckmalloc(bytes: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn allocsymbol() -> *mut _SYMBOL;
    #[no_mangle]
    fn findsymbol(str: *const libc::c_char, len: libc::c_int) -> *mut _SYMBOL;
    #[no_mangle]
    fn CreateSymbol(str: *const libc::c_char, len: libc::c_int)
     -> *mut _SYMBOL;
    #[no_mangle]
    fn FreeSymbolList(sym: *mut _SYMBOL);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ASM_ERROR_EQUATES = libc::c_uint;
pub const ERROR_ILLEGAL_OPERAND_COMBINATION: ASM_ERROR_EQUATES = 37;
pub const ERROR_VALUE_MUST_BE_LT_10000: ASM_ERROR_EQUATES = 36;
pub const ERROR_VALUE_MUST_BE_LT_F: ASM_ERROR_EQUATES = 35;
pub const ERROR_VALUE_MUST_BE_LT_8: ASM_ERROR_EQUATES = 34;
pub const ERROR_VALUE_MUST_BE_LT_10: ASM_ERROR_EQUATES = 33;
pub const ERROR_VALUE_MUST_BE_1_OR_4: ASM_ERROR_EQUATES = 32;
pub const ERROR_BAD_FORMAT: ASM_ERROR_EQUATES = 31;
pub const ERROR_ONLY_ONE_PROCESSOR_SUPPORTED: ASM_ERROR_EQUATES = 30;
pub const ERROR_BADERROR: ASM_ERROR_EQUATES = 29;
pub const ERROR_REPEAT_NEGATIVE: ASM_ERROR_EQUATES = 28;
pub const ERROR_PROCESSOR_NOT_SUPPORTED: ASM_ERROR_EQUATES = 27;
pub const ERROR_VALUE_UNDEFINED: ASM_ERROR_EQUATES = 26;
pub const ERROR_MACRO_REPEATED: ASM_ERROR_EQUATES = 25;
pub const ERROR_LABEL_MISMATCH: ASM_ERROR_EQUATES = 24;
pub const ERROR_NOT_ENOUGH_ARGS: ASM_ERROR_EQUATES = 23;
pub const ERROR_ILLEGAL_BIT_SPECIFICATION: ASM_ERROR_EQUATES = 22;
pub const ERROR_ADDRESS_MUST_BE_LT_10000: ASM_ERROR_EQUATES = 21;
pub const ERROR_ADDRESS_MUST_BE_LT_100: ASM_ERROR_EQUATES = 20;
pub const ERROR_EQU_VALUE_MISMATCH: ASM_ERROR_EQUATES = 19;
pub const ERROR_ORIGIN_REVERSE_INDEXED: ASM_ERROR_EQUATES = 18;
pub const ERROR_ERR_PSEUDO_OP_ENCOUNTERED: ASM_ERROR_EQUATES = 17;
pub const ERROR_BRANCH_OUT_OF_RANGE: ASM_ERROR_EQUATES = 16;
pub const ERROR_ILLEGAL_CHARACTER: ASM_ERROR_EQUATES = 15;
pub const ERROR_PREMATURE_EOF: ASM_ERROR_EQUATES = 14;
pub const ERROR_NOT_ENOUGH_ARGUMENTS_PASSED_TO_MACRO: ASM_ERROR_EQUATES = 13;
pub const ERROR_ILLEGAL_FORCED_ADDRESSING_MODE: ASM_ERROR_EQUATES = 12;
pub const ERROR_ILLEGAL_ADDRESSING_MODE: ASM_ERROR_EQUATES = 11;
pub const ERROR_UNKNOWN_MNEMONIC: ASM_ERROR_EQUATES = 10;
pub const ERROR_DIVISION_BY_0: ASM_ERROR_EQUATES = 9;
pub const ERROR_UNBALANCED_BRACES: ASM_ERROR_EQUATES = 8;
pub const ERROR_EXPRESSION_TABLE_OVERFLOW: ASM_ERROR_EQUATES = 7;
pub const ERROR_SYNTAX_ERROR: ASM_ERROR_EQUATES = 6;
pub const ERROR_NON_ABORT: ASM_ERROR_EQUATES = 5;
pub const ERROR_TOO_MANY_PASSES: ASM_ERROR_EQUATES = 4;
pub const ERROR_NOT_RESOLVABLE: ASM_ERROR_EQUATES = 3;
pub const ERROR_FILE_ERROR: ASM_ERROR_EQUATES = 2;
pub const ERROR_COMMAND_LINE: ASM_ERROR_EQUATES = 1;
pub const ERROR_NONE: ASM_ERROR_EQUATES = 0;
pub type REASON_CODES = libc::c_uint;
pub const REASON_BRANCH_OUT_OF_RANGE: REASON_CODES = 32768;
pub const REASON_PHASE_ERROR: REASON_CODES = 16384;
pub const REASON_FORWARD_REFERENCE: REASON_CODES = 8192;
pub const REASON_REPEAT_NOT_RESOLVED: REASON_CODES = 4096;
pub const REASON_IF_NOT_RESOLVED: REASON_CODES = 2048;
pub const REASON_EQU_VALUE_MISMATCH: REASON_CODES = 1024;
pub const REASON_EQU_NOT_RESOLVED: REASON_CODES = 512;
pub const REASON_ALIGN_NORMAL_ORIGIN_NOT_KNOWN: REASON_CODES = 256;
pub const REASON_ALIGN_RELOCATABLE_ORIGIN_NOT_KNOWN: REASON_CODES = 128;
pub const REASON_ALIGN_NOT_RESOLVED: REASON_CODES = 64;
pub const REASON_DS_NOT_RESOLVED: REASON_CODES = 32;
pub const REASON_DV_NOT_RESOLVED_COULD: REASON_CODES = 16;
pub const REASON_DV_NOT_RESOLVED_PROBABLY: REASON_CODES = 8;
pub const REASON_DC_NOT_RESOVED: REASON_CODES = 4;
pub const REASON_OBSCURE: REASON_CODES = 2;
pub const REASON_MNEMONIC_NOT_RESOLVED: REASON_CODES = 1;
pub type ADDRESS_MODES = libc::c_uint;
pub const NUMOC: ADDRESS_MODES = 21;
pub const AM_BSS: ADDRESS_MODES = 20;
pub const AM_LONG: ADDRESS_MODES = 19;
pub const AM_EXPLIST: ADDRESS_MODES = 18;
pub const AM_SYMBOL: ADDRESS_MODES = 17;
pub const AM_BITBRAMOD: ADDRESS_MODES = 16;
pub const AM_BITMOD: ADDRESS_MODES = 15;
pub const AM_0Y: ADDRESS_MODES = 14;
pub const AM_0X: ADDRESS_MODES = 13;
pub const AM_INDWORD: ADDRESS_MODES = 12;
pub const AM_INDBYTEY: ADDRESS_MODES = 11;
pub const AM_INDBYTEX: ADDRESS_MODES = 10;
pub const AM_REL: ADDRESS_MODES = 9;
pub const AM_WORDADRY: ADDRESS_MODES = 8;
pub const AM_WORDADRX: ADDRESS_MODES = 7;
pub const AM_WORDADR: ADDRESS_MODES = 6;
pub const AM_BYTEADRY: ADDRESS_MODES = 5;
pub const AM_BYTEADRX: ADDRESS_MODES = 4;
pub const AM_BYTEADR: ADDRESS_MODES = 3;
pub const AM_IMM16: ADDRESS_MODES = 2;
pub const AM_IMM8: ADDRESS_MODES = 1;
pub const AM_IMP: ADDRESS_MODES = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SYMBOL {
    pub next: *mut _SYMBOL,
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_uchar,
    pub addrmode: libc::c_uchar,
    pub value: libc::c_long,
    pub namelen: libc::c_uint,
}
/*
 *  EXP.C
 *
 *  Handle expression evaluation and addressing mode decode.
 *
 *  NOTE! If you use the string field in an expression you must clear
 *  the SYM_MACRO and SYM_STRING bits in the flags before calling
 *  FreeSymbolList()!
 */
/* warning: ANSI disallows cast to union type */
/* warning: Calling functions without prototype */
pub type opfunc_t = Option<unsafe extern "C" fn() -> ()>;
#[no_mangle]
pub static mut Argflags: [libc::c_uchar; 64] = [0; 64];
#[no_mangle]
pub static mut Argstack: [libc::c_long; 64] = [0; 64];
#[no_mangle]
pub static mut Argstring: [*mut libc::c_char; 64] =
    [0 as *const libc::c_char as *mut libc::c_char; 64];
#[no_mangle]
pub static mut Oppri: [libc::c_int; 32] = [0; 32];
#[no_mangle]
pub static mut Opdis: [opfunc_t; 32] = [None; 32];
#[no_mangle]
pub static mut Argi: libc::c_int = 0;
#[no_mangle]
pub static mut Opi: libc::c_int = 0;
#[no_mangle]
pub static mut Lastwasop: libc::c_int = 0;
#[no_mangle]
pub static mut Argibase: libc::c_int = 0;
#[no_mangle]
pub static mut Opibase: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn eval(mut str: *const libc::c_char,
                              mut wantmode: libc::c_int) -> *mut _SYMBOL {
    let mut base: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut cur: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut oldargibase: libc::c_int = Argibase;
    let mut oldopibase: libc::c_int = Opibase;
    let mut scr: libc::c_int = 0;
    let mut pLine: *const libc::c_char = str;
    Argibase = Argi;
    Opibase = Opi;
    Lastwasop = 1 as libc::c_int;
    cur = allocsymbol();
    base = cur;
    while *str != 0 {
        if Xdebug {
            printf(b"char \'%c\'\n\x00" as *const u8 as *const libc::c_char,
                   *str as libc::c_int);
        }
        let mut current_block_184: u64;
        match *str as libc::c_int {
            32 | 10 => {
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            126 => {
                if Lastwasop != 0 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_invert as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         128 as libc::c_int);
                } else {
                    asmerr(ERROR_SYNTAX_ERROR as libc::c_int,
                           0 as libc::c_int != 0, pLine);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            42 => {
                if Lastwasop != 0 {
                    pushsymbol(b".\x00" as *const u8 as *const libc::c_char);
                } else {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_mult as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         20 as libc::c_int);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            47 => {
                doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
                                             opfunc_t>(Some(op_div as
                                                                unsafe extern "C" fn(_:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_int,
                                                                                     _:
                                                                                         libc::c_int)
                                                                    -> ())),
                     20 as libc::c_int);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            37 => {
                if Lastwasop != 0 {
                    str = pushbin(str.offset(1 as libc::c_int as isize))
                } else {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_mod as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         20 as libc::c_int);
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            63 => {
                /*  10      */
                doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
                                             opfunc_t>(Some(op_question as
                                                                unsafe extern "C" fn(_:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_int,
                                                                                     _:
                                                                                         libc::c_int)
                                                                    -> ())),
                     10 as libc::c_int);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            43 => {
                /*  19      */
                doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
                                             opfunc_t>(Some(op_add as
                                                                unsafe extern "C" fn(_:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_int,
                                                                                     _:
                                                                                         libc::c_int)
                                                                    -> ())),
                     19 as libc::c_int);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            45 => {
                /*  19: -   (or - unary)        */
                if Lastwasop != 0 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_negate as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         128 as libc::c_int);
                } else {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_sub as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         19 as libc::c_int);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            62 => {
                /*  18: >> <<  17: > >= <= <    */
                if Lastwasop != 0 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_takemsb as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         128 as libc::c_int);
                    str = str.offset(1)
                } else {
                    if *str.offset(1 as libc::c_int as isize) as libc::c_int
                           == '>' as i32 {
                        doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     libc::c_int)
                                                                -> ()>,
                                                     opfunc_t>(Some(op_shiftright
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                ())),
                             18 as libc::c_int);
                        str = str.offset(1)
                    } else if *str.offset(1 as libc::c_int as isize) as
                                  libc::c_int == '=' as i32 {
                        doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     libc::c_int)
                                                                -> ()>,
                                                     opfunc_t>(Some(op_greatereq
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                ())),
                             17 as libc::c_int);
                        str = str.offset(1)
                    } else {
                        doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     libc::c_int)
                                                                -> ()>,
                                                     opfunc_t>(Some(op_greater
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                ())),
                             17 as libc::c_int);
                    }
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            60 => {
                if Lastwasop != 0 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_takelsb as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         128 as libc::c_int);
                    str = str.offset(1)
                } else {
                    if *str.offset(1 as libc::c_int as isize) as libc::c_int
                           == '<' as i32 {
                        doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     libc::c_int)
                                                                -> ()>,
                                                     opfunc_t>(Some(op_shiftleft
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                ())),
                             18 as libc::c_int);
                        str = str.offset(1)
                    } else if *str.offset(1 as libc::c_int as isize) as
                                  libc::c_int == '=' as i32 {
                        doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     libc::c_int)
                                                                -> ()>,
                                                     opfunc_t>(Some(op_smallereq
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                ())),
                             17 as libc::c_int);
                        str = str.offset(1)
                    } else {
                        doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_long,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     libc::c_int)
                                                                -> ()>,
                                                     opfunc_t>(Some(op_smaller
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_long,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                ())),
                             17 as libc::c_int);
                    }
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            61 => {
                /*  16: ==  (= same as ==)      */
                if *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '=' as i32 {
                    str = str.offset(1)
                }
                doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
                                             opfunc_t>(Some(op_eqeq as
                                                                unsafe extern "C" fn(_:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_int,
                                                                                     _:
                                                                                         libc::c_int)
                                                                    -> ())),
                     16 as libc::c_int);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            33 => {
                /*  16: !=                      */
                if Lastwasop != 0 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_not as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         128 as libc::c_int);
                } else {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_noteq as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         16 as libc::c_int);
                    str = str.offset(1)
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            38 => {
                /*  15: &   12: &&              */
                if *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '&' as i32 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_andand as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         12 as libc::c_int);
                    str = str.offset(1)
                } else {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_and as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         15 as libc::c_int);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            94 => {
                /*  14: ^                       */
                doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_long,
                                                                         _:
                                                                             libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
                                             opfunc_t>(Some(op_xor as
                                                                unsafe extern "C" fn(_:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_int,
                                                                                     _:
                                                                                         libc::c_int)
                                                                    -> ())),
                     14 as libc::c_int);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            124 => {
                /*  13: |   11: ||              */
                if *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '|' as i32 {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_oror as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         11 as libc::c_int);
                    str = str.offset(1)
                } else {
                    doop(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_long,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>,
                                                 opfunc_t>(Some(op_or as
                                                                    unsafe extern "C" fn(_:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_long,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            ())),
                         13 as libc::c_int);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            40 => {
                if wantmode != 0 {
                    (*cur).addrmode =
                        AM_INDWORD as libc::c_int as libc::c_uchar;
                    str = str.offset(1);
                    current_block_184 = 3166194604430448652;
                } else { current_block_184 = 18384894229789369419; }
            }
            91 => { current_block_184 = 18384894229789369419; }
            41 => {
                if wantmode != 0 {
                    if (*cur).addrmode as libc::c_int ==
                           AM_INDWORD as libc::c_int &&
                           *str.offset(1 as libc::c_int as isize) as
                               libc::c_int == ',' as i32 &&
                           *str.offset(2 as libc::c_int as isize) as
                               libc::c_int | 0x20 as libc::c_int == 'y' as i32
                       {
                        (*cur).addrmode =
                            AM_INDBYTEY as libc::c_int as libc::c_uchar;
                        str = str.offset(2 as libc::c_int as isize)
                    }
                    //FIX: detect illegal opc (zp),x syntax...
                    if (*cur).addrmode as libc::c_int ==
                           AM_INDWORD as libc::c_int &&
                           *str.offset(1 as libc::c_int as isize) as
                               libc::c_int == ',' as i32 &&
                           *str.offset(2 as libc::c_int as isize) as
                               libc::c_int | 0x20 as libc::c_int == 'x' as i32
                       {
                        let mut sBuffer: [libc::c_char; 128] = [0; 128];
                        sprintf(sBuffer.as_mut_ptr(),
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                str);
                        asmerr(ERROR_ILLEGAL_ADDRESSING_MODE as libc::c_int,
                               0 as libc::c_int != 0, pLine);
                        Redo += 1;
                        Redo_why |=
                            REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as
                                libc::c_ulong
                        //we treat the opcode as valid to allow passes to continue, which should
                   //allow other errors (like phase errros) to resolve before our "++Redo"
                   //ultimately forces a failure.
                    }
                    str = str.offset(1);
                    current_block_184 = 3166194604430448652;
                } else { current_block_184 = 8741107198128373303; }
            }
            93 => { current_block_184 = 8741107198128373303; }
            35 => {
                (*cur).addrmode = AM_IMM8 as libc::c_int as libc::c_uchar;
                str = str.offset(1);
                /*
            * No other addressing mode is possible from now on
            * so we might as well allow () instead of [].
            */
                wantmode = 0 as libc::c_int; /* to lower case */
                current_block_184 = 3166194604430448652;
            }
            44 => {
                while Opi != Opibase { evaltop(); }
                Lastwasop = 1 as libc::c_int;
                scr =
                    *str.offset(1 as libc::c_int as isize) as libc::c_int |
                        0x20 as libc::c_int;
                if (*cur).addrmode as libc::c_int == AM_INDWORD as libc::c_int
                       && scr == 'x' as i32 &&
                       IsAlphaNum(*str.offset(2 as libc::c_int as isize) as
                                      libc::c_int) == 0 {
                    (*cur).addrmode =
                        AM_INDBYTEX as libc::c_int as libc::c_uchar;
                    str = str.offset(1)
                } else if (*cur).addrmode as libc::c_int ==
                              AM_INDWORD as libc::c_int && scr == 'y' as i32
                              &&
                              *str.offset(2 as libc::c_int as isize) as
                                  libc::c_int == ')' as i32 && wantmode != 0 {
                    let mut sBuffer_0: [libc::c_char; 128] = [0; 128];
                    sprintf(sBuffer_0.as_mut_ptr(),
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            str);
                    asmerr(ERROR_ILLEGAL_ADDRESSING_MODE as libc::c_int,
                           0 as libc::c_int != 0, pLine);
                    Redo += 1;
                    Redo_why |=
                        REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as
                            libc::c_ulong;
                    //FIX: detect illegal opc (zp,y) syntax...
                    //we treat the opcode as valid to allow passes to continue, which should
                   //allow other errors (like phase errros) to resolve before our "++Redo"
                   //ultimately forces a failure.
                    (*cur).addrmode = AM_0Y as libc::c_int as libc::c_uchar;
                    str = str.offset(1)
                } else if scr == 'x' as i32 &&
                              IsAlphaNum(*str.offset(2 as libc::c_int as
                                                         isize) as
                                             libc::c_int) == 0 {
                    (*cur).addrmode = AM_0X as libc::c_int as libc::c_uchar;
                    str = str.offset(1);
                    //FIX: OPCODE.FORCE needs to be adjusted for x indexing...
                    if Mnext == AM_WORDADR as libc::c_int {
                        Mnext = AM_WORDADRX as libc::c_int
                    }
                    if Mnext == AM_BYTEADR as libc::c_int {
                        Mnext = AM_BYTEADRX as libc::c_int
                    }
                    if Mnext == AM_INDWORD as libc::c_int {
                        Mnext = AM_0X as libc::c_int
                    }
                } else if scr == 'y' as i32 &&
                              IsAlphaNum(*str.offset(2 as libc::c_int as
                                                         isize) as
                                             libc::c_int) == 0 {
                    (*cur).addrmode = AM_0Y as libc::c_int as libc::c_uchar;
                    str = str.offset(1);
                    //FIX: OPCODE.FORCE needs to be adjusted for x indexing...
                    if Mnext == AM_WORDADR as libc::c_int {
                        Mnext = AM_WORDADRY as libc::c_int
                    }
                    if Mnext == AM_BYTEADR as libc::c_int {
                        Mnext = AM_BYTEADRY as libc::c_int
                    }
                    if Mnext == AM_INDWORD as libc::c_int {
                        Mnext = AM_0Y as libc::c_int
                    }
                } else {
                    let mut pNewSymbol: *mut _SYMBOL = allocsymbol();
                    (*cur).next = pNewSymbol;
                    Argi -= 1;
                    if Argi < Argibase {
                        asmerr(ERROR_SYNTAX_ERROR as libc::c_int,
                               0 as libc::c_int != 0, pLine);
                    }
                    if Argi > Argibase {
                        asmerr(ERROR_SYNTAX_ERROR as libc::c_int,
                               0 as libc::c_int != 0, pLine);
                    }
                    (*cur).value = Argstack[Argi as usize];
                    (*cur).flags = Argflags[Argi as usize];
                    (*cur).string =
                        Argstring[Argi as usize] as *mut libc::c_void as
                            *mut libc::c_char;
                    if !(*cur).string.is_null() {
                        (*cur).flags =
                            ((*cur).flags as libc::c_int | 0x8 as libc::c_int)
                                as libc::c_uchar;
                        if Xdebug {
                            printf(b"STRING: %s\n\x00" as *const u8 as
                                       *const libc::c_char, (*cur).string);
                        }
                    }
                    cur = pNewSymbol
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            36 => {
                str = pushhex(str.offset(1 as libc::c_int as isize));
                current_block_184 = 3166194604430448652;
            }
            39 => {
                str = pushchar(str.offset(1 as libc::c_int as isize));
                current_block_184 = 3166194604430448652;
            }
            34 => {
                str = pushstr(str.offset(1 as libc::c_int as isize));
                current_block_184 = 3166194604430448652;
            }
            _ => {
                let mut dol: *const libc::c_char = str;
                while *dol as libc::c_int >= '0' as i32 &&
                          *dol as libc::c_int <= '9' as i32 {
                    dol = dol.offset(1)
                }
                if *dol as libc::c_int == '$' as i32 {
                    str = pushsymbol(str)
                } else if *str as libc::c_int == '0' as i32 {
                    str = pushoct(str)
                } else if *str as libc::c_int > '0' as i32 &&
                              *str as libc::c_int <= '9' as i32 {
                    str = pushdec(str)
                } else { str = pushsymbol(str) }
                current_block_184 = 3166194604430448652;
            }
        }
        match current_block_184 {
            8741107198128373303 =>
            /* fall thru OK */
            {
                while Opi != Opibase &&
                          Oppri[(Opi - 1 as libc::c_int) as usize] != 0 {
                    evaltop();
                }
                if Opi != Opibase { Opi -= 1 }
                str = str.offset(1);
                if Argi == Argibase {
                    puts(b"\']\' error, no arg on stack\x00" as *const u8 as
                             *const libc::c_char);
                } else {
                    if *str as libc::c_int == 'd' as i32 {
                        /*  STRING CONVERSION   */
                        let mut buf: [libc::c_char; 32] = [0; 32];
                        str = str.offset(1);
                        if Argflags[(Argi - 1 as libc::c_int) as usize] as
                               libc::c_int == 0 as libc::c_int {
                            sprintf(buf.as_mut_ptr(),
                                    b"%ld\x00" as *const u8 as
                                        *const libc::c_char,
                                    Argstack[(Argi - 1 as libc::c_int) as
                                                 usize]);
                            Argstring[(Argi - 1 as libc::c_int) as usize] =
                                strcpy(ckmalloc(strlen(buf.as_mut_ptr()).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                                    as libc::c_int),
                                       buf.as_mut_ptr())
                        }
                    }
                    Lastwasop = 0 as libc::c_int
                }
            }
            18384894229789369419 =>
            /* fall thru OK */
            /*  eventually an argument      */
            {
                if Opi == 32 as libc::c_int {
                    puts(b"too many ops\x00" as *const u8 as
                             *const libc::c_char);
                } else {
                    let fresh0 = Opi;
                    Opi = Opi + 1;
                    Oppri[fresh0 as usize] = 0 as libc::c_int
                }
                str = str.offset(1)
            }
            _ => { }
        }
    }
    while Opi != Opibase { evaltop(); }
    if Argi != Argibase {
        Argi -= 1;
        (*cur).value = Argstack[Argi as usize];
        (*cur).flags = Argflags[Argi as usize];
        (*cur).string =
            Argstring[Argi as usize] as *mut libc::c_void as
                *mut libc::c_char;
        if !(*cur).string.is_null() {
            (*cur).flags =
                ((*cur).flags as libc::c_int | 0x8 as libc::c_int) as
                    libc::c_uchar;
            if Xdebug {
                printf(b"STRING: %s\n\x00" as *const u8 as
                           *const libc::c_char, (*cur).string);
            }
        }
        if (*base).addrmode as libc::c_int == 0 as libc::c_int {
            (*base).addrmode = AM_BYTEADR as libc::c_int as libc::c_uchar
        }
    }
    if Argi != Argibase || Opi != Opibase {
        asmerr(ERROR_SYNTAX_ERROR as libc::c_int, 0 as libc::c_int != 0,
               pLine);
    }
    Argi = Argibase;
    Opi = Opibase;
    Argibase = oldargibase;
    Opibase = oldopibase;
    return base;
}
#[no_mangle]
pub unsafe extern "C" fn IsAlphaNum(mut c: libc::c_int) -> libc::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32 ||
                c >= 'A' as i32 && c <= 'Z' as i32 ||
                c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn evaltop() {
    if Xdebug {
        printf(b"evaltop @(A,O) %d %d\n\x00" as *const u8 as
                   *const libc::c_char, Argi, Opi);
    }
    if Opi <= Opibase {
        asmerr(ERROR_SYNTAX_ERROR as libc::c_int, 0 as libc::c_int != 0,
               0 as *const libc::c_char);
        Opi = Opibase;
        return
    }
    Opi -= 1;
    if Oppri[Opi as usize] == 128 as libc::c_int {
        if Argi < Argibase + 1 as libc::c_int {
            asmerr(ERROR_SYNTAX_ERROR as libc::c_int, 0 as libc::c_int != 0,
                   0 as *const libc::c_char);
            Argi = Argibase;
            return
        }
        Argi -= 1;
        ::std::mem::transmute::<_,
                                fn(_: _,
                                   _:
                                       _)>(Some((*Opdis.as_mut_ptr().offset(Opi
                                                                                as
                                                                                isize)).expect("non-null function pointer")).expect("non-null function pointer"))(Argstack[Argi
                                                                                                                                                                               as
                                                                                                                                                                               usize],
                                                                                                                                                                  Argflags[Argi
                                                                                                                                                                               as
                                                                                                                                                                               usize]
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int);
    } else {
        if Argi < Argibase + 2 as libc::c_int {
            asmerr(ERROR_SYNTAX_ERROR as libc::c_int, 0 as libc::c_int != 0,
                   0 as *const libc::c_char);
            Argi = Argibase;
            return
        }
        Argi -= 2 as libc::c_int;
        ::std::mem::transmute::<_,
                                fn(_: _, _: _, _: _,
                                   _:
                                       _)>(Some((*Opdis.as_mut_ptr().offset(Opi
                                                                                as
                                                                                isize)).expect("non-null function pointer")).expect("non-null function pointer"))(Argstack[Argi
                                                                                                                                                                               as
                                                                                                                                                                               usize],
                                                                                                                                                                  Argstack[(Argi
                                                                                                                                                                                +
                                                                                                                                                                                1
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int)
                                                                                                                                                                               as
                                                                                                                                                                               usize],
                                                                                                                                                                  Argflags[Argi
                                                                                                                                                                               as
                                                                                                                                                                               usize]
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int,
                                                                                                                                                                  Argflags[(Argi
                                                                                                                                                                                +
                                                                                                                                                                                1
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int)
                                                                                                                                                                               as
                                                                                                                                                                               usize]
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int);
    };
}
unsafe extern "C" fn stackarg(mut val: libc::c_long, mut flags: libc::c_int,
                              mut ptr1: *const libc::c_char) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if Xdebug {
        printf(b"stackarg %ld (@%d)\n\x00" as *const u8 as
                   *const libc::c_char, val, Argi);
    }
    Lastwasop = 0 as libc::c_int;
    if flags & 0x8 as libc::c_int != 0 {
        /*
           Why unsigned char? Looks like we're converting to
           long in a very strange way... [phf]
        */
        let mut ptr: *const libc::c_uchar = ptr1 as *const libc::c_uchar;
        let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        len = 0 as libc::c_int;
        val = len as libc::c_long;
        while *ptr as libc::c_int != 0 && *ptr as libc::c_int != '\"' as i32 {
            val = val << 8 as libc::c_int | *ptr as libc::c_long;
            ptr = ptr.offset(1);
            len += 1
        }
        new = ckmalloc(len + 1 as libc::c_int);
        memcpy(new as *mut libc::c_void, ptr1 as *const libc::c_void,
               len as libc::c_ulong);
        *new.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        flags &= !(0x8 as libc::c_int);
        str = new
    }
    Argstack[Argi as usize] = val;
    Argstring[Argi as usize] = str;
    Argflags[Argi as usize] = flags as libc::c_uchar;
    Argi += 1;
    if Argi == 64 as libc::c_int {
        puts(b"stackarg: maxargs stacked\x00" as *const u8 as
                 *const libc::c_char);
        Argi = Argibase
    }
    while Opi != Opibase &&
              Oppri[(Opi - 1 as libc::c_int) as usize] == 128 as libc::c_int {
        evaltop();
    };
}
#[no_mangle]
pub unsafe extern "C" fn doop(mut func: opfunc_t, mut pri: libc::c_int) {
    if Xdebug { puts(b"doop\x00" as *const u8 as *const libc::c_char); }
    Lastwasop = 1 as libc::c_int;
    if Opi == Opibase || pri == 128 as libc::c_int {
        if Xdebug {
            printf(b"doop @ %d unary\n\x00" as *const u8 as
                       *const libc::c_char, Opi);
        }
        Opdis[Opi as usize] = func;
        Oppri[Opi as usize] = pri;
        Opi += 1;
        return
    }
    while Opi != Opibase && Oppri[(Opi - 1 as libc::c_int) as usize] != 0 &&
              pri <= Oppri[(Opi - 1 as libc::c_int) as usize] {
        evaltop();
    }
    if Xdebug {
        printf(b"doop @ %d\n\x00" as *const u8 as *const libc::c_char, Opi);
    }
    Opdis[Opi as usize] = func;
    Oppri[Opi as usize] = pri;
    Opi += 1;
    if Opi == 32 as libc::c_int {
        puts(b"doop: too many operators\x00" as *const u8 as
                 *const libc::c_char);
        Opi = Opibase
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_takelsb(mut v1: libc::c_long,
                                    mut f1: libc::c_int) {
    stackarg(v1 & 0xff as libc::c_long, f1, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_takemsb(mut v1: libc::c_long,
                                    mut f1: libc::c_int) {
    stackarg(v1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long, f1,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_negate(mut v1: libc::c_long,
                                   mut f1: libc::c_int) {
    stackarg(-v1, f1, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_invert(mut v1: libc::c_long,
                                   mut f1: libc::c_int) {
    stackarg(!v1, f1, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_not(mut v1: libc::c_long, mut f1: libc::c_int) {
    stackarg((v1 == 0) as libc::c_int as libc::c_long, f1,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_mult(mut v1: libc::c_long, mut v2: libc::c_long,
                                 mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 * v2, f1 | f2, 0 as *const libc::c_char);
    Lastwasop = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn op_div(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    Lastwasop = 1 as libc::c_int;
    if f1 | f2 != 0 {
        stackarg(0 as libc::c_long, f1 | f2, 0 as *const libc::c_char);
        return
    }
    if v2 == 0 as libc::c_int as libc::c_long {
        asmerr(ERROR_DIVISION_BY_0 as libc::c_int, 1 as libc::c_int != 0,
               0 as *const libc::c_char);
        stackarg(0 as libc::c_long, 0 as libc::c_int,
                 0 as *const libc::c_char);
    } else { stackarg(v1 / v2, 0 as libc::c_int, 0 as *const libc::c_char); };
}
#[no_mangle]
pub unsafe extern "C" fn op_mod(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    if f1 | f2 != 0 {
        stackarg(0 as libc::c_long, f1 | f2, 0 as *const libc::c_char);
        return
    }
    if v2 == 0 as libc::c_int as libc::c_long {
        stackarg(v1, 0 as libc::c_int, 0 as *const libc::c_char);
    } else { stackarg(v1 % v2, 0 as libc::c_int, 0 as *const libc::c_char); }
    Lastwasop = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn op_question(mut v1: libc::c_long,
                                     mut v2: libc::c_long,
                                     mut f1: libc::c_int,
                                     mut f2: libc::c_int) {
    if f1 != 0 {
        stackarg(0 as libc::c_long, f1, 0 as *const libc::c_char);
    } else {
        stackarg(if v1 != 0 { v2 } else { 0 as libc::c_int as libc::c_long },
                 if v1 != 0 { f2 } else { 0 as libc::c_int },
                 0 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_add(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 + v2, f1 | f2, 0 as *const libc::c_char);
    Lastwasop = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn op_sub(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 - v2, f1 | f2, 0 as *const libc::c_char);
    Lastwasop = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn op_shiftright(mut v1: libc::c_long,
                                       mut v2: libc::c_long,
                                       mut f1: libc::c_int,
                                       mut f2: libc::c_int) {
    if f1 | f2 != 0 {
        stackarg(0 as libc::c_long, f1 | f2, 0 as *const libc::c_char);
    } else {
        stackarg(v1 >> v2, 0 as libc::c_int, 0 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_shiftleft(mut v1: libc::c_long,
                                      mut v2: libc::c_long,
                                      mut f1: libc::c_int,
                                      mut f2: libc::c_int) {
    if f1 | f2 != 0 {
        stackarg(0 as libc::c_long, f1 | f2, 0 as *const libc::c_char);
    } else {
        stackarg(v1 << v2, 0 as libc::c_int, 0 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_greater(mut v1: libc::c_long,
                                    mut v2: libc::c_long, mut f1: libc::c_int,
                                    mut f2: libc::c_int) {
    stackarg((v1 > v2) as libc::c_int as libc::c_long, f1 | f2,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_greatereq(mut v1: libc::c_long,
                                      mut v2: libc::c_long,
                                      mut f1: libc::c_int,
                                      mut f2: libc::c_int) {
    stackarg((v1 >= v2) as libc::c_int as libc::c_long, f1 | f2,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_smaller(mut v1: libc::c_long,
                                    mut v2: libc::c_long, mut f1: libc::c_int,
                                    mut f2: libc::c_int) {
    stackarg((v1 < v2) as libc::c_int as libc::c_long, f1 | f2,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_smallereq(mut v1: libc::c_long,
                                      mut v2: libc::c_long,
                                      mut f1: libc::c_int,
                                      mut f2: libc::c_int) {
    stackarg((v1 <= v2) as libc::c_int as libc::c_long, f1 | f2,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_eqeq(mut v1: libc::c_long, mut v2: libc::c_long,
                                 mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg((v1 == v2) as libc::c_int as libc::c_long, f1 | f2,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_noteq(mut v1: libc::c_long, mut v2: libc::c_long,
                                  mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg((v1 != v2) as libc::c_int as libc::c_long, f1 | f2,
             0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_andand(mut v1: libc::c_long, mut v2: libc::c_long,
                                   mut f1: libc::c_int, mut f2: libc::c_int) {
    if f1 == 0 && v1 == 0 || f2 == 0 && v2 == 0 {
        stackarg(0 as libc::c_long, 0 as libc::c_int,
                 0 as *const libc::c_char);
        return
    }
    stackarg(1 as libc::c_long, f1 | f2, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_oror(mut v1: libc::c_long, mut v2: libc::c_long,
                                 mut f1: libc::c_int, mut f2: libc::c_int) {
    if f1 == 0 && v1 != 0 || f2 == 0 && v2 != 0 {
        stackarg(1 as libc::c_long, 0 as libc::c_int,
                 0 as *const libc::c_char);
        return
    }
    stackarg(0 as libc::c_long, f1 | f2, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_xor(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 ^ v2, f1 | f2, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_and(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 & v2, f1 | f2, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn op_or(mut v1: libc::c_long, mut v2: libc::c_long,
                               mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 | v2, f1 | f2, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn pushchar(mut str: *const libc::c_char)
 -> *const libc::c_char {
    if *str != 0 {
        stackarg(*str as libc::c_long, 0 as libc::c_int,
                 0 as *const libc::c_char);
        str = str.offset(1)
    } else {
        stackarg(' ' as i32 as libc::c_long, 0 as libc::c_int,
                 0 as *const libc::c_char);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushhex(mut str: *const libc::c_char)
 -> *const libc::c_char {
    let mut val: libc::c_long = 0 as libc::c_int as libc::c_long;
    loop  {
        if *str as libc::c_int >= '0' as i32 &&
               *str as libc::c_int <= '9' as i32 {
            val =
                (val << 4 as libc::c_int) +
                    (*str as libc::c_int - '0' as i32) as libc::c_long
        } else {
            if !(*str as libc::c_int >= 'a' as i32 &&
                     *str as libc::c_int <= 'f' as i32 ||
                     *str as libc::c_int >= 'A' as i32 &&
                         *str as libc::c_int <= 'F' as i32) {
                break ;
            }
            val =
                (val << 4 as libc::c_int) +
                    ((*str as libc::c_int & 0x1f as libc::c_int) +
                         9 as libc::c_int) as libc::c_long
        }
        str = str.offset(1)
    }
    stackarg(val, 0 as libc::c_int, 0 as *const libc::c_char);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushoct(mut str: *const libc::c_char)
 -> *const libc::c_char {
    let mut val: libc::c_long = 0 as libc::c_int as libc::c_long;
    while *str as libc::c_int >= '0' as i32 &&
              *str as libc::c_int <= '7' as i32 {
        val =
            (val << 3 as libc::c_int) +
                (*str as libc::c_int - '0' as i32) as libc::c_long;
        str = str.offset(1)
    }
    stackarg(val, 0 as libc::c_int, 0 as *const libc::c_char);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushdec(mut str: *const libc::c_char)
 -> *const libc::c_char {
    let mut val: libc::c_long = 0 as libc::c_int as libc::c_long;
    while *str as libc::c_int >= '0' as i32 &&
              *str as libc::c_int <= '9' as i32 {
        val =
            val * 10 as libc::c_int as libc::c_long +
                (*str as libc::c_int - '0' as i32) as libc::c_long;
        str = str.offset(1)
    }
    stackarg(val, 0 as libc::c_int, 0 as *const libc::c_char);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushbin(mut str: *const libc::c_char)
 -> *const libc::c_char {
    let mut val: libc::c_long = 0 as libc::c_int as libc::c_long;
    while *str as libc::c_int == '0' as i32 ||
              *str as libc::c_int == '1' as i32 {
        val =
            val << 1 as libc::c_int |
                (*str as libc::c_int - '0' as i32) as libc::c_long;
        str = str.offset(1)
    }
    stackarg(val, 0 as libc::c_int, 0 as *const libc::c_char);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushstr(mut str: *const libc::c_char)
 -> *const libc::c_char {
    stackarg(0 as libc::c_int as libc::c_long, 0x8 as libc::c_int, str);
    while *str as libc::c_int != 0 && *str as libc::c_int != '\"' as i32 {
        str = str.offset(1)
    }
    if *str as libc::c_int == '\"' as i32 { str = str.offset(1) }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushsymbol(mut str: *const libc::c_char)
 -> *const libc::c_char {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut macro_0: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    ptr = str;
    while *ptr as libc::c_int == '_' as i32 ||
              *ptr as libc::c_int == '.' as i32 ||
              *ptr as libc::c_int >= 'a' as i32 &&
                  *ptr as libc::c_int <= 'z' as i32 ||
              *ptr as libc::c_int >= 'A' as i32 &&
                  *ptr as libc::c_int <= 'Z' as i32 ||
              *ptr as libc::c_int >= '0' as i32 &&
                  *ptr as libc::c_int <= '9' as i32 {
        ptr = ptr.offset(1)
    }
    if ptr == str {
        asmerr(ERROR_ILLEGAL_CHARACTER as libc::c_int, 0 as libc::c_int != 0,
               str);
        printf(b"char = \'%c\' %d (-1: %d)\n\x00" as *const u8 as
                   *const libc::c_char, *str as libc::c_int,
               *str as libc::c_int,
               *str.offset(-(1 as libc::c_int as isize)) as libc::c_int);
        if !F_listfile.is_null() {
            fprintf(FI_listfile,
                    b"char = \'%c\' code %d\n\x00" as *const u8 as
                        *const libc::c_char, *str as libc::c_int,
                    *str as libc::c_int);
        }
        return str.offset(1 as libc::c_int as isize)
    }
    if *ptr as libc::c_int == '$' as i32 { ptr = ptr.offset(1) }
    sym =
        findsymbol(str,
                   ptr.wrapping_offset_from(str) as libc::c_long as
                       libc::c_int);
    if !sym.is_null() {
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            Redo_eval += 1
        }
        if (*sym).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            macro_0 = 1 as libc::c_int as libc::c_uchar;
            sym = eval((*sym).string, 0 as libc::c_int)
        }
        if (*sym).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            stackarg(0 as libc::c_int as libc::c_long, 0x8 as libc::c_int,
                     (*sym).string);
        } else {
            stackarg((*sym).value,
                     (*sym).flags as libc::c_int & 0x1 as libc::c_int,
                     0 as *const libc::c_char);
        }
        (*sym).flags =
            ((*sym).flags as libc::c_int |
                 (0x4 as libc::c_int | 0x40 as libc::c_int)) as libc::c_uchar;
        if macro_0 != 0 { FreeSymbolList(sym); }
    } else {
        stackarg(0 as libc::c_long, 0x1 as libc::c_int,
                 0 as *const libc::c_char);
        sym =
            CreateSymbol(str,
                         ptr.wrapping_offset_from(str) as libc::c_long as
                             libc::c_int);
        (*sym).flags =
            (0x4 as libc::c_int | 0x40 as libc::c_int | 0x1 as libc::c_int) as
                libc::c_uchar;
        Redo_eval += 1
    }
    return ptr;
}
