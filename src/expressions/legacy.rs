use libc;

use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SymbolTypes,
};
use crate::types::enums::{
    AddressModes,
    AsmErrorEquates,
};
use crate::utils::{
    filesystem,
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
     -> *mut libc::c_void;
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const i8)
     -> i32;
    #[no_mangle]
    fn ckmalloc(bytes: i32) -> *mut i8;
    #[no_mangle]
    fn CreateSymbol(str: *const i8, len: i32)
     -> *mut _SYMBOL;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type FILE = _IO_FILE;
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
pub static mut Argstring: [*mut i8; MAX_ARGS] =
    [0 as *const i8 as *mut i8; MAX_ARGS];
#[no_mangle]
pub static mut Opdis: [opfunc_t; MAX_OPS] = [None; MAX_OPS];
#[no_mangle]
pub unsafe extern "C" fn eval(str: &str, mut wantmode: i32) -> Symbol {
    let mut base: Symbol = Symbol::new();
    let mut cur: Symbol = Symbol::new();
    let mut oldArgIndexBase = state.expressions.argIndexBase;
    let mut oldOpIndexBase = state.expressions.opIndexBase;
    let mut scr: i32 = 0;
    let mut pLine: *const i8 = str;
    state.expressions.argIndexBase = state.expressions.argIndex;
    state.expressions.opIndexBase = state.expressions.opIndex;
    state.expressions.lastWasOp = true;
    base = &cur;
    for c in str.chars() {
        if state.parameters.debug {
            println!("char '{}'", transient::str_pointer_and_len_to_string(str, 1));
        }
        let mut current_block_184: u64;
        match c {
            ' ' | '\n' => {
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '~' => {
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i32) -> ()>, opfunc_t>(
                            Some(op_invert as unsafe extern "C" fn(_: i64, _: i32) -> ())
                        ),
                        128
                    );
                } else {
                    asmerr(AsmErrorEquates::SyntaxError,
                           false, pLine);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '*' => {
                if state.expressions.lastWasOp {
                    pushsymbol(b".\x00" as *const u8 as *const i8);
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ()>, opfunc_t>(
                            Some(op_mult as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        20
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '/' => {
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ()>, opfunc_t>(
                        Some(op_div as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                    ),
                    20
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '%' => {
                if state.expressions.lastWasOp {
                    str = pushbin(str.offset(1))
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_mod as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        20
                    );
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            '?' => {
                /*  10      */
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                        Some(op_question as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                    ),
                    10
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '+' => {
                /*  19      */
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                        Some(op_add as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                    ),
                    19
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '-' => {
                /*  19: -   (or - unary)        */
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i32)-> ()>, opfunc_t>(
                            Some(op_negate as unsafe extern "C" fn(_: i64, _: i32) -> ())
                        ),
                        128
                    );
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_sub as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        19
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '>' => {
                /*  18: >> <<  17: > >= <= <    */
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i32)-> ()>, opfunc_t>(
                            Some(op_takemsb as unsafe extern "C" fn(_: i64, _: i32) -> ())
                        ),
                        128
                    );
                    str = str.offset(1)
                } else {
                    if *str.offset(1) as i32 == '>' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                                Some(op_shiftright as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                            ),
                            18
                        );
                        str = str.offset(1)
                    } else if *str.offset(1) as i32 == '=' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                                Some(op_greatereq as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                            ),
                            17
                        );
                        str = str.offset(1)
                    } else {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                                Some(op_greater as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                            ),
                            17
                        );
                    }
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            '<' => {
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i32)-> ()>, opfunc_t>(
                            Some(op_takelsb as unsafe extern "C" fn(_: i64, _: i32) -> ())
                        ),
                        128
                    );
                    str = str.offset(1)
                } else {
                    if *str.offset(1) as i32 == '<' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                                Some(op_shiftleft as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())),
                            18
                        );
                        str = str.offset(1)
                    } else if *str.offset(1) as i32 == '=' as i32 {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                                Some(op_smallereq as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                            ),
                            17
                        );
                        str = str.offset(1)
                    } else {
                        doop(
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                                Some(op_smaller as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                            ),
                            17
                        );
                    }
                    str = str.offset(1)
                }
                current_block_184 = 3166194604430448652;
            }
            '=' => {
                /*  16: ==  (= same as ==)      */
                if *str.offset(1) as i32 == '=' as i32 {
                    str = str.offset(1)
                }
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                        Some(op_eqeq as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                    ),
                    16
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '!' => {
                /*  16: !=                      */
                if state.expressions.lastWasOp {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i32)-> ()>, opfunc_t>(
                            Some(op_not as unsafe extern "C" fn(_: i64, _: i32) -> ())
                        ),
                        128
                    );
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_noteq as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        16
                    );
                    str = str.offset(1)
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '&' => {
                /*  15: &   12: &&              */
                if *str.offset(1) as i32 == '&' as i32 {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_andand as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        12
                    );
                    str = str.offset(1)
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_and as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        15
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '^'  => {
                /*  14: ^                       */
                doop(
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                        Some(op_xor as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                    ),
                    14
                );
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '|' => {
                /*  13: |   11: ||              */
                if *str.offset(1) as i32 == '|' as i32 {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_oror as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        11
                    );
                    str = str.offset(1)
                } else {
                    doop(
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32)-> ()>, opfunc_t>(
                            Some(op_or as unsafe extern "C" fn(_: i64, _: i64, _: i32, _: i32) -> ())
                        ),
                        13
                    );
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '(' => {
                if wantmode != 0 {
                    (*cur).addrmode =
                        AddressModes::IndWord as u8;
                    str = str.offset(1);
                    current_block_184 = 3166194604430448652;
                } else { current_block_184 = 18384894229789369419; }
            }
            '[' => { current_block_184 = 18384894229789369419; }
            ')' => {
                if wantmode != 0 {
                    if (*cur).addrmode as i32 ==
                        AddressModes::IndWord as i32 &&
                           *str.offset(1) as i32 == ',' as i32 &&
                           *str.offset(2) as i32 | 0x20 as i32 == 'y' as i32
                       {
                        (*cur).addrmode =
                            AddressModes::IndByteY as u8;
                        str = str.offset(2)
                    }
                    //FIX: detect illegal opc (zp),x syntax...
                    if (*cur).addrmode as u8 == AddressModes::IndByteY as u8 && *str.offset(1) as u8 == ',' as u8 && *str.offset(2) as u8 | 0x20 == 'x' as u8 {
                        // FIXME: strangely, this is never used, so we have it here but commented out
                        // let buffer: String = transient::str_pointer_to_string(str);
                        asmerr(AsmErrorEquates::IllegalAddressingMode, false, pLine);
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
            ']' => { current_block_184 = 8741107198128373303; }
            '#' => {
                (*cur).addrmode = AddressModes::Imm8 as u8;
                str = str.offset(1);
                /*
            * No other addressing mode is possible from now on
            * so we might as well allow () instead of [].
            */
                wantmode = 0; /* to lower case */
                current_block_184 = 3166194604430448652;
            }
            ',' => {
                while state.expressions.opIndex != state.expressions.opIndexBase { evaltop(); }
                state.expressions.lastWasOp = true;
                scr =
                    *str.offset(1) as i32 |
                        0x20 as i32;
                if (*cur).addrmode == AddressModes::IndWord as u8 && scr == 'x' as i32 && !is_alpha_num(*str.offset(2) as u8 as char) {
                    (*cur).addrmode = AddressModes::IndByteX as u8;
                    str = str.offset(1);
                } else if (*cur).addrmode as u8 == AddressModes::IndWord as u8 && scr == 'y' as i32 && *str.offset(2) as u8 == ')' as u8 && wantmode != 0 {
                    // FIXME: strangely, this is never used, so we have it here but commented out
                    // let buffer: String = transient::str_pointer_to_string(str);
                    asmerr(AsmErrorEquates::IllegalAddressingMode, false, pLine);
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
                    if state.execution.modeNext == AddressModes::WordAdr {
                        state.execution.modeNext = AddressModes::WordAdrX
                    }
                    if state.execution.modeNext == AddressModes::ByteAdr {
                        state.execution.modeNext = AddressModes::ByteAdrX
                    }
                    if state.execution.modeNext == AddressModes::IndWord {
                        state.execution.modeNext = AddressModes::ZeroX
                    }
                } else if scr == 'y' as i32 && !is_alpha_num(*str.offset(2) as u8 as char) {
                    (*cur).addrmode = AddressModes::ZeroY as u8;
                    str = str.offset(1);
                    //FIX: OPCODE.FORCE needs to be adjusted for x indexing...
                    if state.execution.modeNext == AddressModes::WordAdr {
                        state.execution.modeNext = AddressModes::WordAdrY
                    }
                    if state.execution.modeNext == AddressModes::ByteAdr {
                        state.execution.modeNext = AddressModes::ByteAdrY
                    }
                    if state.execution.modeNext == AddressModes::IndWord {
                        state.execution.modeNext = AddressModes::ZeroY
                    }
                } else {
                    let mut pNewSymbol: *mut _SYMBOL = allocsymbol();
                    (*cur).next = pNewSymbol;
                    state.expressions.argIndex -= 1;
                    if state.expressions.argIndex < state.expressions.argIndexBase {
                        asmerr(AsmErrorEquates::SyntaxError,
                               false, pLine);
                    }
                    if state.expressions.argIndex > state.expressions.argIndexBase {
                        asmerr(AsmErrorEquates::SyntaxError,
                               false, pLine);
                    }
                    (*cur).value = state.expressions.argStack[state.expressions.argIndex];
                    (*cur).flags = state.expressions.argFlags[state.expressions.argIndex];
                    (*cur).string =
                        Argstring[state.expressions.argIndex] as *mut libc::c_void as
                            *mut i8;
                    if !(*cur).string.is_null() {
                        (*cur).flags =
                            ((*cur).flags as i32 | 0x8 as i32) as u8;
                        if state.parameters.debug {
                            println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
                        }
                    }
                    cur = pNewSymbol
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '$' => {
                str = pushhex(str.offset(1));
                current_block_184 = 3166194604430448652;
            }
            '\'' => {
                str = pushchar(str.offset(1));
                current_block_184 = 3166194604430448652;
            }
            '"' => {
                str = pushstr(str.offset(1));
                current_block_184 = 3166194604430448652;
            }
            _ => {
                let mut dol: *const i8 = str;
                while *dol as i32 >= '0' as i32 &&
                          *dol as i32 <= '9' as i32 {
                    dol = dol.offset(1)
                }
                if *dol as i32 == '$' as i32 {
                    str = pushsymbol(str)
                } else if *str as i32 == '0' as i32 {
                    str = pushoct(str)
                } else if *str as i32 > '0' as i32 &&
                              *str as i32 <= '9' as i32 {
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
                    if *str as i32 == 'd' as i32 {
                        /*  STRING CONVERSION   */
                        str = str.offset(1);
                        if state.expressions.argFlags[state.expressions.argIndex - 1] == 0 {
                            let buffer = format!("{}", state.expressions.argStack[state.expressions.argIndex - 1]);
                            Argstring[state.expressions.argIndex - 1] = transient::string_to_str_pointer(buffer);
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
    while state.expressions.opIndex != state.expressions.opIndexBase {
        evaltop();
    }
    if state.expressions.argIndex != state.expressions.argIndexBase {
        state.expressions.argIndex -= 1;
        (*cur).value = state.expressions.argStack[state.expressions.argIndex];
        (*cur).flags = state.expressions.argFlags[state.expressions.argIndex];
        (*cur).string =
            Argstring[state.expressions.argIndex] as *mut libc::c_void as
                *mut i8;
        if !(*cur).string.is_null() {
            (*cur).flags =
                ((*cur).flags as i32 | 0x8 as i32) as u8;
            if state.parameters.debug {
                println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
            }
        }
        if (*base).addrmode as i32 == 0 {
            (*base).addrmode = AddressModes::ByteAdr as i32 as u8
        }
    }
    if state.expressions.argIndex != state.expressions.argIndexBase || state.expressions.opIndex != state.expressions.opIndexBase {
        asmerr(AsmErrorEquates::SyntaxError, false,
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
        println!("evaltop @(A,O) {} {}", state.expressions.argIndex, state.expressions.opIndex);
    }
    if state.expressions.opIndex <= state.expressions.opIndexBase {
        asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
        state.expressions.opIndex = state.expressions.opIndexBase;
        return
    }
    state.expressions.opIndex -= 1;
    if state.expressions.opPri[state.expressions.opIndex] == 128 {
        if state.expressions.argIndex < state.expressions.argIndexBase + 1 {
            asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
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
            asmerr(AsmErrorEquates::SyntaxError, false,
                   0 as *const i8);
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
unsafe extern "C" fn stackarg(mut val: i64, mut flags: i32,
                              mut ptr1: *const i8) {
    let mut str: *mut i8 = 0 as *mut i8;
    if state.parameters.debug {
        println!("stackarg {} (@{})", val, state.expressions.argIndex);
    }
    state.expressions.lastWasOp = false;
    if flags & 0x8 as i32 != 0 {
        /*
           Why unsigned char? Looks like we're converting to
           long in a very strange way... [phf]
        */
        let mut ptr: *const u8 = ptr1 as *const u8;
        let mut new: *mut i8 = 0 as *mut i8;
        let mut len: i32 = 0;
        len = 0;
        val = len as i64;
        while *ptr as i32 != 0 && *ptr as i32 != '\"' as i32 {
            val = val << 8 | *ptr as i64;
            ptr = ptr.offset(1);
            len += 1
        }
        new = ckmalloc(len + 1);
        memcpy(new as *mut libc::c_void, ptr1 as *const libc::c_void,
               len as u64);
        *new.offset(len as isize) = 0;
        flags &= !(0x8 as i32);
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
            println!("doop @ {} unary", state.expressions.opIndex);
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
        println!("doop @ {}", state.expressions.opIndex);
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
pub unsafe extern "C" fn op_takelsb(mut v1: i64,
                                    mut f1: i32) {
    stackarg(v1 & 0xff as i64, f1, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_takemsb(mut v1: i64,
                                    mut f1: i32) {
    stackarg(v1 >> 8 & 0xff as i32 as i64, f1,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_negate(mut v1: i64,
                                   mut f1: i32) {
    stackarg(-v1, f1, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_invert(mut v1: i64,
                                   mut f1: i32) {
    stackarg(!v1, f1, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_not(mut v1: i64, mut f1: i32) {
    stackarg((v1 == 0) as i32 as i64, f1,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_mult(mut v1: i64, mut v2: i64,
                                 mut f1: i32, mut f2: i32) {
    stackarg(v1 * v2, f1 | f2, 0 as *const i8);
    state.expressions.lastWasOp = true;
}
#[no_mangle]
pub unsafe extern "C" fn op_div(mut v1: i64, mut v2: i64,
                                mut f1: i32, mut f2: i32) {
    state.expressions.lastWasOp = true;
    if f1 | f2 != 0 {
        stackarg(0, f1 | f2, 0 as *const i8);
        return
    }
    if v2 == 0 {
        asmerr(AsmErrorEquates::DivisionByZero, true,
               0 as *const i8);
        stackarg(0, 0,
                 0 as *const i8);
    } else { stackarg(v1 / v2, 0, 0 as *const i8); };
}
#[no_mangle]
pub unsafe extern "C" fn op_mod(mut v1: i64, mut v2: i64,
                                mut f1: i32, mut f2: i32) {
    if f1 | f2 != 0 {
        stackarg(0, f1 | f2, 0 as *const i8);
        return
    }
    if v2 == 0 {
        stackarg(v1, 0, 0 as *const i8);
    } else { stackarg(v1 % v2, 0, 0 as *const i8); }
    state.expressions.lastWasOp = true;
}
#[no_mangle]
pub unsafe extern "C" fn op_question(mut v1: i64,
                                     mut v2: i64,
                                     mut f1: i32,
                                     mut f2: i32) {
    if f1 != 0 {
        stackarg(0, f1, 0 as *const i8);
    } else {
        stackarg(if v1 != 0 { v2 } else { 0 },
                 if v1 != 0 { f2 } else { 0 },
                 0 as *const i8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_add(mut v1: i64, mut v2: i64,
                                mut f1: i32, mut f2: i32) {
    stackarg(v1 + v2, f1 | f2, 0 as *const i8);
    state.expressions.lastWasOp = true;
}
#[no_mangle]
pub unsafe extern "C" fn op_sub(mut v1: i64, mut v2: i64,
                                mut f1: i32, mut f2: i32) {
    stackarg(v1 - v2, f1 | f2, 0 as *const i8);
    state.expressions.lastWasOp = true;
}
#[no_mangle]
pub unsafe extern "C" fn op_shiftright(mut v1: i64,
                                       mut v2: i64,
                                       mut f1: i32,
                                       mut f2: i32) {
    if f1 | f2 != 0 {
        stackarg(0, f1 | f2, 0 as *const i8);
    } else {
        stackarg(v1 >> v2, 0, 0 as *const i8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_shiftleft(mut v1: i64,
                                      mut v2: i64,
                                      mut f1: i32,
                                      mut f2: i32) {
    if f1 | f2 != 0 {
        stackarg(0, f1 | f2, 0 as *const i8);
    } else {
        stackarg(v1 << v2, 0, 0 as *const i8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_greater(mut v1: i64,
                                    mut v2: i64, mut f1: i32,
                                    mut f2: i32) {
    stackarg((v1 > v2) as i32 as i64, f1 | f2,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_greatereq(mut v1: i64,
                                      mut v2: i64,
                                      mut f1: i32,
                                      mut f2: i32) {
    stackarg((v1 >= v2) as i32 as i64, f1 | f2,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_smaller(mut v1: i64,
                                    mut v2: i64, mut f1: i32,
                                    mut f2: i32) {
    stackarg((v1 < v2) as i32 as i64, f1 | f2,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_smallereq(mut v1: i64,
                                      mut v2: i64,
                                      mut f1: i32,
                                      mut f2: i32) {
    stackarg((v1 <= v2) as i32 as i64, f1 | f2,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_eqeq(mut v1: i64, mut v2: i64,
                                 mut f1: i32, mut f2: i32) {
    stackarg((v1 == v2) as i32 as i64, f1 | f2,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_noteq(mut v1: i64, mut v2: i64,
                                  mut f1: i32, mut f2: i32) {
    stackarg((v1 != v2) as i32 as i64, f1 | f2,
             0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_andand(mut v1: i64, mut v2: i64,
                                   mut f1: i32, mut f2: i32) {
    if f1 == 0 && v1 == 0 || f2 == 0 && v2 == 0 {
        stackarg(0, 0,
                 0 as *const i8);
        return
    }
    stackarg(1, f1 | f2, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_oror(mut v1: i64, mut v2: i64,
                                 mut f1: i32, mut f2: i32) {
    if f1 == 0 && v1 != 0 || f2 == 0 && v2 != 0 {
        stackarg(1, 0,
                 0 as *const i8);
        return
    }
    stackarg(0, f1 | f2, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_xor(mut v1: i64, mut v2: i64,
                                mut f1: i32, mut f2: i32) {
    stackarg(v1 ^ v2, f1 | f2, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_and(mut v1: i64, mut v2: i64,
                                mut f1: i32, mut f2: i32) {
    stackarg(v1 & v2, f1 | f2, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn op_or(mut v1: i64, mut v2: i64,
                               mut f1: i32, mut f2: i32) {
    stackarg(v1 | v2, f1 | f2, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn pushchar(mut str: *const i8) -> *const i8 {
    if *str != 0 {
        stackarg(*str as i64, 0,
                 0 as *const i8);
        str = str.offset(1)
    } else {
        stackarg(' ' as i32 as i64, 0,
                 0 as *const i8);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushhex(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    loop  {
        if *str as i32 >= '0' as i32 &&
               *str as i32 <= '9' as i32 {
            val =
                (val << 4) +
                    (*str as i32 - '0' as i32) as i64
        } else {
            if !(*str as i32 >= 'a' as i32 &&
                     *str as i32 <= 'f' as i32 ||
                     *str as i32 >= 'A' as i32 &&
                         *str as i32 <= 'F' as i32) {
                break ;
            }
            val =
                (val << 4) +
                    ((*str as i32 & 0x1f as i32) +
                         9) as i64
        }
        str = str.offset(1)
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushoct(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 >= '0' as i32 &&
              *str as i32 <= '7' as i32 {
        val =
            (val << 3) +
                (*str as i32 - '0' as i32) as i64;
        str = str.offset(1)
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushdec(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 >= '0' as i32 &&
              *str as i32 <= '9' as i32 {
        val =
            val * 10 +
                (*str as i32 - '0' as i32) as i64;
        str = str.offset(1)
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushbin(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 == '0' as i32 ||
              *str as i32 == '1' as i32 {
        val =
            val << 1 |
                (*str as i32 - '0' as i32) as i64;
        str = str.offset(1)
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushstr(mut str: *const i8) -> *const i8 {
    stackarg(0, 0x8 as i32, str);
    while *str as i32 != 0 && *str as i32 != '\"' as i32 {
        str = str.offset(1)
    }
    if *str as i32 == '\"' as i32 { str = str.offset(1) }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushsymbol(mut str: *const i8) -> *const i8 {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut ptr: *const i8 = 0 as *const i8;
    let mut macro_0: u8 = 0;
    ptr = str;
    while *ptr as i32 == '_' as i32 ||
              *ptr as i32 == '.' as i32 ||
              *ptr as i32 >= 'a' as i32 &&
                  *ptr as i32 <= 'z' as i32 ||
              *ptr as i32 >= 'A' as i32 &&
                  *ptr as i32 <= 'Z' as i32 ||
              *ptr as i32 >= '0' as i32 &&
                  *ptr as i32 <= '9' as i32 {
        ptr = ptr.offset(1)
    }
    if ptr == str {
        asmerr(AsmErrorEquates::IllegalCharacter, false, str);
        println!("char = '{}' {} (-1: {})",
            transient::str_pointer_to_string(str),
            transient::str_pointer_to_string(str).as_bytes()[0],
            transient::str_pointer_to_string(str.offset(-1)).as_bytes()[0],
        );
        filesystem::writeln_to_file_maybe(
            &mut state.output.listFile,
            format!(
                "char = '{}' code {}",
                // FIXME: pass correct code
                transient::str_pointer_to_string(str), // %c
                transient::str_pointer_to_string(str).as_bytes()[0], // %d
            ).as_str(),
        );
        return str.offset(1);
    }
    if *ptr as i32 == '$' as i32 { ptr = ptr.offset(1) }
    match symbols::find_symbol(&mut state, transient::str_pointer_to_string(str).as_str()) {
        Some(symbol) => {
            if symbol.flags & SymbolTypes::Unknown != 0 {
                state.execution.redoEval += 1
            }
            if symbol.flags & SymbolTypes::Macro != 0 {
                macro_0 = 1;
                sym = eval(symbol.string, 0)
            }
            if symbol.flags & SymbolTypes::StringResult != 0 {
                stackarg(0, SymbolTypes::StringResult, symbol.string);
            } else {
                stackarg(symbol.value, symbol.flags & SymbolTypes::Unknown, 0);
            }
            symbol.flags = symbol.flags | (SymbolTypes::Referenced | SymbolTypes::MasterReference);
            if macro_0 != 0 {
                FreeSymbolList(sym);
            }
        },
        None => {
            stackarg(0, SymbolTypes::Unknown, 0);
            sym = CreateSymbol(str, ptr.wrapping_offset_from(str) as i64 as i32);
            symbol.flags = (SymbolTypes::Referenced | SymbolTypes::MasterReference | SymbolTypes::Unknown);
            state.execution.redoEval += 1;
        }
    }
    return ptr;
}
