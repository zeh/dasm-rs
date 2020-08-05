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

use crate::expressions::{
    is_alpha_num,
};

pub const MAX_OPS: usize = 32;
pub const MAX_ARGS: usize = 64;

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
pub static mut Argstring: [*mut libc::c_char; MAX_ARGS] =
    [0 as *const libc::c_char as *mut libc::c_char; MAX_ARGS];
#[no_mangle]
pub static mut Opdis: [opfunc_t; MAX_OPS] = [None; MAX_OPS];
#[no_mangle]
pub unsafe extern "C" fn eval(mut str: *const libc::c_char,
                              mut wantmode: libc::c_int) -> *mut _SYMBOL {
    let mut base: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut cur: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut oldArgIndexBase = state.expressions.argIndexBase;
    let mut oldOpIndexBase = state.expressions.opIndexBase;
    let mut scr: libc::c_int = 0;
    let mut pLine: *const libc::c_char = str;
    state.expressions.argIndexBase = state.expressions.argIndex;
    state.expressions.opIndexBase = state.expressions.opIndex;
    state.expressions.lastWasOp = true;
    cur = allocsymbol();
    base = cur;
    while *str != 0 {
        if state.parameters.debug {
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
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_int) -> ()>, opfunc_t>(
                            Some(op_invert as unsafe extern "C" fn(_: libc::c_long, _: libc::c_int) -> ())
                        ),
                        128
                    );
                } else {
                    asmerr(AsmErrorEquates::SyntaxError,
                           0 as libc::c_int != 0, pLine);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            42 => {
                if state.expressions.lastWasOp {
                    pushsymbol(b".\x00" as *const u8 as *const libc::c_char);
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ()>, opfunc_t>(
                            Some(op_mult as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        20
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            47 => {
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ()>, opfunc_t>(
                        Some(op_div as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                    ),
                    20
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            37 => {
                if state.expressions.lastWasOp {
                    str = pushbin(str.offset(1 as libc::c_int as isize))
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_mod as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        20
                    );
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            63 => {
                /*  10      */
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                        Some(op_question as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                    ),
                    10
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            43 => {
                /*  19      */
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                        Some(op_add as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                    ),
                    19
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            45 => {
                /*  19: -   (or - unary)        */
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_negate as unsafe extern "C" fn(_: libc::c_long, _: libc::c_int) -> ())
                        ),
                        128
                    );
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_sub as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        19
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            62 => {
                /*  18: >> <<  17: > >= <= <    */
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_takemsb as unsafe extern "C" fn(_: libc::c_long, _: libc::c_int) -> ())
                        ),
                        128
                    );
                    str = str.offset(1)
                } else {
                    if *str.offset(1 as libc::c_int as isize) as libc::c_int == '>' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                                Some(op_shiftright as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                            ),
                            18
                        );
                        str = str.offset(1)
                    } else if *str.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                                Some(op_greatereq as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                            ),
                            17
                        );
                        str = str.offset(1)
                    } else {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                                Some(op_greater as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                            ),
                            17
                        );
                    }
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            60 => {
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_takelsb as unsafe extern "C" fn(_: libc::c_long, _: libc::c_int) -> ())
                        ),
                        128
                    );
                    str = str.offset(1)
                } else {
                    if *str.offset(1 as libc::c_int as isize) as libc::c_int == '<' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                                Some(op_shiftleft as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())),
                            18
                        );
                        str = str.offset(1)
                    } else if *str.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                                Some(op_smallereq as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                            ),
                            17
                        );
                        str = str.offset(1)
                    } else {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                                Some(op_smaller as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                            ),
                            17
                        );
                    }
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            61 => {
                /*  16: ==  (= same as ==)      */
                if *str.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                    str = str.offset(1)
                }
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                        Some(op_eqeq as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                    ),
                    16
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            33 => {
                /*  16: !=                      */
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_not as unsafe extern "C" fn(_: libc::c_long, _: libc::c_int) -> ())
                        ),
                        128
                    );
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_noteq as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        16
                    );
                    str = str.offset(1)
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            38 => {
                /*  15: &   12: &&              */
                if *str.offset(1 as libc::c_int as isize) as libc::c_int == '&' as i32 {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_andand as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        12
                    );
                    str = str.offset(1)
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_and as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        15
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            94 => {
                /*  14: ^                       */
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                        Some(op_xor as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                    ),
                    14
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            124 => {
                /*  13: |   11: ||              */
                if *str.offset(1 as libc::c_int as isize) as libc::c_int == '|' as i32 {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_oror as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        11
                    );
                    str = str.offset(1)
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int)-> ()>, opfunc_t>(
                            Some(op_or as unsafe extern "C" fn(_: libc::c_long, _: libc::c_long, _: libc::c_int, _: libc::c_int) -> ())
                        ),
                        13
                    );
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
                        state.execution.redoIndex += 1;
                        state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved
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
            44 => { // ',' - FIXME: convert back to original char
                while state.expressions.opIndex != state.expressions.opIndexBase { evaltop(); }
                state.expressions.lastWasOp = true;
                scr =
                    *str.offset(1 as libc::c_int as isize) as libc::c_int |
                        0x20 as libc::c_int;
                if (*cur).addrmode as libc::c_int == AddressModes::IndWord as i32
                       && scr == 'x' as i32 && !is_alpha_num(*str.offset(2) as u8 as char) {
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
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved;
                    //FIX: detect illegal opc (zp,y) syntax...
                    //we treat the opcode as valid to allow passes to continue, which should
                   //allow other errors (like phase errros) to resolve before our "++Redo"
                   //ultimately forces a failure.
                    (*cur).addrmode = AddressModes::ZeroY as u8;
                    str = str.offset(1)
                } else if scr == 'x' as i32 && !is_alpha_num(*str.offset(2) as u8 as char) {
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
                } else if scr == 'y' as i32 && !is_alpha_num(*str.offset(2) as u8 as char) {
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
                    state.expressions.argIndex -= 1;
                    if state.expressions.argIndex < state.expressions.argIndexBase {
                        asmerr(AsmErrorEquates::SyntaxError,
                               0 as libc::c_int != 0, pLine);
                    }
                    if state.expressions.argIndex > state.expressions.argIndexBase {
                        asmerr(AsmErrorEquates::SyntaxError,
                               0 as libc::c_int != 0, pLine);
                    }
                    (*cur).value = state.expressions.argStack[state.expressions.argIndex];
                    (*cur).flags = state.expressions.argFlags[state.expressions.argIndex];
                    (*cur).string =
                        Argstring[state.expressions.argIndex] as *mut libc::c_void as
                            *mut libc::c_char;
                    if !(*cur).string.is_null() {
                        (*cur).flags =
                            ((*cur).flags as libc::c_int | 0x8 as libc::c_int)
                                as libc::c_uchar;
                        if state.parameters.debug {
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
                while state.expressions.opIndex != state.expressions.opIndexBase && state.expressions.opPri[state.expressions.opIndex - 1] != 0 {
                    evaltop();
                }
                if state.expressions.opIndex != state.expressions.opIndexBase { state.expressions.opIndex -= 1 }
                str = str.offset(1);
                if state.expressions.argIndex == state.expressions.argIndexBase {
                    println!("\']\' error, no arg on stack");
                } else {
                    if *str as libc::c_int == 'd' as i32 {
                        /*  STRING CONVERSION   */
                        let mut buf: [libc::c_char; 32] = [0; 32];
                        str = str.offset(1);
                        if state.expressions.argFlags[state.expressions.argIndex - 1] == 0 {
                            sprintf(buf.as_mut_ptr(),
                                b"%ld\x00" as *const u8 as *const libc::c_char,
                                state.expressions.argStack[state.expressions.argIndex - 1]
                            );
                            Argstring[state.expressions.argIndex - 1] =
                                strcpy(ckmalloc(strlen(buf.as_mut_ptr()).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                                    as libc::c_int),
                                       buf.as_mut_ptr())
                        }
                    }
                    state.expressions.lastWasOp = false
                }
            }
            18384894229789369419 =>
            /* fall thru OK */
            /*  eventually an argument      */
            {
                if state.expressions.opIndex == 32 {
                    println!("too many ops");
                } else {
                    let fresh0 = state.expressions.opIndex;
                    state.expressions.opIndex = state.expressions.opIndex + 1;
                    state.expressions.opPri[fresh0] = 0
                }
                str = str.offset(1)
            }
            _ => { }
        }
    }
    while state.expressions.opIndex != state.expressions.opIndexBase { evaltop(); }
    if state.expressions.argIndex != state.expressions.argIndexBase {
        state.expressions.argIndex -= 1;
        (*cur).value = state.expressions.argStack[state.expressions.argIndex];
        (*cur).flags = state.expressions.argFlags[state.expressions.argIndex];
        (*cur).string =
            Argstring[state.expressions.argIndex] as *mut libc::c_void as
                *mut libc::c_char;
        if !(*cur).string.is_null() {
            (*cur).flags =
                ((*cur).flags as libc::c_int | 0x8 as libc::c_int) as
                    libc::c_uchar;
            if state.parameters.debug {
                println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
            }
        }
        if (*base).addrmode as libc::c_int == 0 as libc::c_int {
            (*base).addrmode = AddressModes::ByteAdr as libc::c_int as libc::c_uchar
        }
    }
    if state.expressions.argIndex != state.expressions.argIndexBase || state.expressions.opIndex != state.expressions.opIndexBase {
        asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
               pLine);
    }
    state.expressions.argIndex = state.expressions.argIndexBase;
    state.expressions.opIndex = state.expressions.opIndexBase;
    state.expressions.argIndexBase = oldArgIndexBase;
    state.expressions.opIndexBase = oldOpIndexBase;
    return base;
}
#[no_mangle]
pub unsafe extern "C" fn evaltop() {
    if state.parameters.debug {
        printf(b"evaltop @(A,O) %d %d\n\x00" as *const u8 as
                   *const libc::c_char, state.expressions.argIndex, state.expressions.opIndex);
    }
    if state.expressions.opIndex <= state.expressions.opIndexBase {
        asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
               0 as *const libc::c_char);
        state.expressions.opIndex = state.expressions.opIndexBase;
        return
    }
    state.expressions.opIndex -= 1;
    if state.expressions.opPri[state.expressions.opIndex] == 128 {
        if state.expressions.argIndex < state.expressions.argIndexBase + 1 {
            asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
                   0 as *const libc::c_char);
            state.expressions.argIndex = state.expressions.argIndexBase;
            return
        }
        state.expressions.argIndex -= 1;
        ::std::mem::transmute::<_, fn(_: _, _: _)>(
                Some(
                    (
                        *Opdis.as_mut_ptr().offset(state.expressions.opIndex as isize)
                    ).expect("non-null function pointer")
                ).expect("non-null function pointer")
            )(
                state.expressions.argStack[state.expressions.argIndex],
                state.expressions.argFlags[state.expressions.argIndex]
            );
    } else {
        if state.expressions.argIndex < state.expressions.argIndexBase + 2 {
            asmerr(AsmErrorEquates::SyntaxError, 0 as libc::c_int != 0,
                   0 as *const libc::c_char);
            state.expressions.argIndex = state.expressions.argIndexBase;
            return
        }
        state.expressions.argIndex -= 2;
        ::std::mem::transmute::<_, fn(_: _, _: _, _: _, _: _)>(
                Some(
                    (
                        *Opdis.as_mut_ptr().offset(state.expressions.opIndex as isize)
                    ).expect("non-null function pointer")
                ).expect("non-null function pointer")
            )(
                state.expressions.argStack[state.expressions.argIndex],
                state.expressions.argStack[state.expressions.argIndex + 1],
                state.expressions.argFlags[state.expressions.argIndex],
                state.expressions.argFlags[state.expressions.argIndex + 1]
            );
    };
}
unsafe extern "C" fn stackarg(mut val: libc::c_long, mut flags: libc::c_int,
                              mut ptr1: *const libc::c_char) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if state.parameters.debug {
        printf(b"stackarg %ld (@%d)\n\x00" as *const u8 as
                   *const libc::c_char, val, state.expressions.argIndex);
    }
    state.expressions.lastWasOp = false;
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
    state.expressions.argStack[state.expressions.argIndex] = val;
    Argstring[state.expressions.argIndex] = str;
    state.expressions.argFlags[state.expressions.argIndex] = flags as u8; // FIXME: truncate, check source flags type...
    state.expressions.argIndex += 1;
    if state.expressions.argIndex == 64 {
        println!("stackarg: maxargs stacked");
        state.expressions.argIndex = state.expressions.argIndexBase
    }
    while state.expressions.opIndex != state.expressions.opIndexBase && state.expressions.opPri[state.expressions.opIndex - 1] == 128 {
        evaltop();
    };
}
#[no_mangle]
pub unsafe extern "C" fn doop(mut func: opfunc_t, mut pri: usize) {
    if state.parameters.debug {
        println!("doop");
    }
    state.expressions.lastWasOp = true;
    if state.expressions.opIndex == state.expressions.opIndexBase || pri == 128 {
        if state.parameters.debug {
            printf(b"doop @ %d unary\n\x00" as *const u8 as
                       *const libc::c_char, state.expressions.opIndex);
        }
        Opdis[state.expressions.opIndex] = func;
        state.expressions.opPri[state.expressions.opIndex] = pri;
        state.expressions.opIndex += 1;
        return
    }
    while state.expressions.opIndex != state.expressions.opIndexBase && state.expressions.opPri[state.expressions.opIndex - 1] != 0 && pri <= state.expressions.opPri[state.expressions.opIndex - 1] {
        evaltop();
    }
    if state.parameters.debug {
        printf(b"doop @ %d\n\x00" as *const u8 as *const libc::c_char, state.expressions.opIndex);
    }
    Opdis[state.expressions.opIndex] = func;
    state.expressions.opPri[state.expressions.opIndex] = pri;
    state.expressions.opIndex += 1;
    if state.expressions.opIndex == 32 {
        println!("doop: too many operators");
        state.expressions.opIndex = state.expressions.opIndexBase
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
    state.expressions.lastWasOp = true;
}
#[no_mangle]
pub unsafe extern "C" fn op_div(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    state.expressions.lastWasOp = true;
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
    state.expressions.lastWasOp = true;
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
    state.expressions.lastWasOp = true;
}
#[no_mangle]
pub unsafe extern "C" fn op_sub(mut v1: libc::c_long, mut v2: libc::c_long,
                                mut f1: libc::c_int, mut f2: libc::c_int) {
    stackarg(v1 - v2, f1 | f2, 0 as *const libc::c_char);
    state.expressions.lastWasOp = true;
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
pub unsafe extern "C" fn pushchar(mut str: *const libc::c_char) -> *const libc::c_char {
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
pub unsafe extern "C" fn pushhex(mut str: *const libc::c_char) -> *const libc::c_char {
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
pub unsafe extern "C" fn pushoct(mut str: *const libc::c_char) -> *const libc::c_char {
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
pub unsafe extern "C" fn pushdec(mut str: *const libc::c_char) -> *const libc::c_char {
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
pub unsafe extern "C" fn pushbin(mut str: *const libc::c_char) -> *const libc::c_char {
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
pub unsafe extern "C" fn pushstr(mut str: *const libc::c_char) -> *const libc::c_char {
    stackarg(0 as libc::c_int as libc::c_long, 0x8 as libc::c_int, str);
    while *str as libc::c_int != 0 && *str as libc::c_int != '\"' as i32 {
        str = str.offset(1)
    }
    if *str as libc::c_int == '\"' as i32 { str = str.offset(1) }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushsymbol(mut str: *const libc::c_char) -> *const libc::c_char {
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
            state.execution.redoEval += 1
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
        state.execution.redoEval += 1
    }
    return ptr;
}
