use libc;

use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
};
use crate::types::enums::{
    AddressModes,
    AsmErrorEquates,
};
use crate::utils::{
    transient,
};

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
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const libc::c_char)
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
        if state.debug {
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
                    asmerr(AsmErrorEquates::SyntaxError,
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
                        AddressModes::IndWord as u8;
                    str = str.offset(1);
                    current_block_184 = 3166194604430448652;
                } else { current_block_184 = 18384894229789369419; }
            }
            91 => { current_block_184 = 18384894229789369419; }
            41 => {
                if wantmode != 0 {
                    if (*cur).addrmode as libc::c_int ==
                        AddressModes::IndWord as i32 &&
                           *str.offset(1 as libc::c_int as isize) as
                               libc::c_int == ',' as i32 &&
                           *str.offset(2 as libc::c_int as isize) as
                               libc::c_int | 0x20 as libc::c_int == 'y' as i32
                       {
                        (*cur).addrmode =
                            AddressModes::IndByteY as u8;
                        str = str.offset(2 as libc::c_int as isize)
                    }
                    //FIX: detect illegal opc (zp),x syntax...
                    if (*cur).addrmode as libc::c_int ==
                            AddressModes::IndByteY as i32 &&
                           *str.offset(1 as libc::c_int as isize) as
                               libc::c_int == ',' as i32 &&
                           *str.offset(2 as libc::c_int as isize) as
                               libc::c_int | 0x20 as libc::c_int == 'x' as i32
                       {
                        let mut sBuffer: [libc::c_char; 128] = [0; 128];
                        sprintf(sBuffer.as_mut_ptr(),
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                str);
                        asmerr(AsmErrorEquates::IllegalAddressingMode,
                               0 as libc::c_int != 0, pLine);
                        Redo += 1;
                        Redo_why |=
                            ReasonCodes::MnemonicNotResolved
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
                (*cur).addrmode = AddressModes::Imm8 as u8;
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
                if (*cur).addrmode as libc::c_int == AddressModes::IndWord as i32
                       && scr == 'x' as i32 &&
                       IsAlphaNum(*str.offset(2 as libc::c_int as isize) as
                                      libc::c_int) == 0 {
                    (*cur).addrmode =
                        AddressModes::IndByteX as u8;
                    str = str.offset(1)
                } else if (*cur).addrmode as libc::c_int ==
                              AddressModes::IndWord as i32 && scr == 'y' as i32
                              &&
                              *str.offset(2 as libc::c_int as isize) as
                                  libc::c_int == ')' as i32 && wantmode != 0 {
                    let mut sBuffer_0: [libc::c_char; 128] = [0; 128];
                    sprintf(sBuffer_0.as_mut_ptr(),
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            str);
                    asmerr(AsmErrorEquates::IllegalAddressingMode,
                           0 as libc::c_int != 0, pLine);
                    Redo += 1;
                    Redo_why |=
                        ReasonCodes::MnemonicNotResolved;
                    //FIX: detect illegal opc (zp,y) syntax...
                    //we treat the opcode as valid to allow passes to continue, which should
                   //allow other errors (like phase errros) to resolve before our "++Redo"
                   //ultimately forces a failure.
                    (*cur).addrmode = AddressModes::ZeroY as u8;
                    str = str.offset(1)
                } else if scr == 'x' as i32 &&
                              IsAlphaNum(*str.offset(2 as libc::c_int as
                                                         isize) as
                                             libc::c_int) == 0 {
                    (*cur).addrmode = AddressModes::ZeroX as u8;
                    str = str.offset(1);
                    //FIX: OPCODE.FORCE needs to be adjusted for x indexing...
                    if Mnext == AddressModes::WordAdr as i32 {
                        Mnext = AddressModes::WordAdrX as i32
                    }
                    if Mnext == AddressModes::ByteAdr as i32 {
                        Mnext = AddressModes::ByteAdrX as i32
                    }
                    if Mnext == AddressModes::IndWord as i32 {
                        Mnext = AddressModes::ZeroX as i32
                    }
                } else if scr == 'y' as i32 &&
                              IsAlphaNum(*str.offset(2 as libc::c_int as
                                                         isize) as
                                             libc::c_int) == 0 {
                    (*cur).addrmode = AddressModes::ZeroY as u8;
                    str = str.offset(1);
                    //FIX: OPCODE.FORCE needs to be adjusted for x indexing...
                    if Mnext == AddressModes::WordAdr as i32 {
                        Mnext = AddressModes::WordAdrY as i32
                    }
                    if Mnext == AddressModes::ByteAdr as i32 {
                        Mnext = AddressModes::ByteAdrY as i32
                    }
                    if Mnext == AddressModes::IndWord as i32 {
                        Mnext = AddressModes::ZeroY as i32
                    }
                } else {
                    let mut pNewSymbol: *mut _SYMBOL = allocsymbol();
                    (*cur).next = pNewSymbol;
                    Argi -= 1;
                    if Argi < Argibase {
                        asmerr(AsmErrorEquates::SyntaxError,
                               0 as libc::c_int != 0, pLine);
                    }
                    if Argi > Argibase {
                        asmerr(AsmErrorEquates::SyntaxError,
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
                        if state.debug {
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
                    println!("\']\' error, no arg on stack");
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
                    println!("too many ops");
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
            if state.debug {
                println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
            }
        }
        if (*base).addrmode as libc::c_int == 0 as libc::c_int {
            (*base).addrmode = AddressModes::ByteAdr as libc::c_int as libc::c_uchar
        }
    }
    if Argi != Argibase || Opi != Opibase {
        asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
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
    if state.debug {
        printf(b"evaltop @(A,O) %d %d\n\x00" as *const u8 as
                   *const libc::c_char, Argi, Opi);
    }
    if Opi <= Opibase {
        asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
               0 as *const libc::c_char);
        Opi = Opibase;
        return
    }
    Opi -= 1;
    if Oppri[Opi as usize] == 128 as libc::c_int {
        if Argi < Argibase + 1 as libc::c_int {
            asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
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
            asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
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
    if state.debug {
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
        println!("stackarg: maxargs stacked");
        Argi = Argibase
    }
    while Opi != Opibase &&
              Oppri[(Opi - 1 as libc::c_int) as usize] == 128 as libc::c_int {
        evaltop();
    };
}
#[no_mangle]
pub unsafe extern "C" fn doop(mut func: opfunc_t, mut pri: libc::c_int) {
    if state.debug {
        println!("doop");
    }
    Lastwasop = 1 as libc::c_int;
    if Opi == Opibase || pri == 128 as libc::c_int {
        if state.debug {
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
    if state.debug {
        printf(b"doop @ %d\n\x00" as *const u8 as *const libc::c_char, Opi);
    }
    Opdis[Opi as usize] = func;
    Oppri[Opi as usize] = pri;
    Opi += 1;
    if Opi == 32 as libc::c_int {
        println!("doop: too many operators");
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
        asmerr(AsmErrorEquates::DivisionByZero, 1 as libc::c_int != 0,
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
        asmerr(AsmErrorEquates::IllegalCharacter, 0 as libc::c_int != 0,
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
