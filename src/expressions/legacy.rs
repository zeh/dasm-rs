use libc;

use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
};
use crate::types::enums::{
    AddressModes,
    AsmErrorEquates,
};
use crate::types::legacy::{
    _SYMBOL,
};
use crate::utils::{
    filesystem,
    transient,
};
use crate::expressions::{
    is_alpha_num,
    operations,
};
use crate::expressions::operations::{
    ExpressionOperationFunc,
};

pub const MAX_OPS: usize = 32;
pub const MAX_ARGS: usize = 64;

extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const i8) -> i32;
    #[no_mangle]
    fn ckmalloc(bytes: i32) -> *mut i8;
    #[no_mangle]
    fn allocsymbol() -> *mut _SYMBOL;
    #[no_mangle]
    fn findsymbol(str: *const i8, len: i32) -> *mut _SYMBOL;
    #[no_mangle]
    fn CreateSymbol(str: *const i8, len: i32) -> *mut _SYMBOL;
    #[no_mangle]
    fn FreeSymbolList(sym: *mut _SYMBOL);
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
#[no_mangle]
pub unsafe extern "C" fn eval(mut str: *const i8, mut wantmode: i32) -> *mut _SYMBOL {
    let mut base: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut cur: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let oldArgIndexBase = state.expressions.argIndexBase;
    let oldOpIndexBase = state.expressions.opIndexBase;
    let mut scr: i32 = 0;
    let pLine: *const i8 = str;
    state.expressions.argIndexBase = state.expressions.argStack.len();
    state.expressions.opIndexBase = state.expressions.opFunc.len();
    state.expressions.lastWasOp = true;
    cur = allocsymbol();
    base = cur;
    while *str != 0 {
        if state.parameters.debug {
            println!("char '{}'", transient::str_pointer_and_len_to_string(str, 1));
        }
        let current_block_184: u64;
        match *str as u8 as char {
            ' ' | '\n' => {
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '~' => {
                if state.expressions.lastWasOp {
                    doop(operations::invert, 128);
                } else {
                    asmerr(AsmErrorEquates::SyntaxError, false, pLine);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '*' => {
                if state.expressions.lastWasOp {
                    pushsymbol(b".\x00" as *const u8 as *const i8);
                } else {
                    doop(operations::multiply, 20);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '/' => {
                doop(operations::divide, 20);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '%' => {
                if state.expressions.lastWasOp {
                    str = pushbin(str.offset(1));
                } else {
                    doop(operations::modulo, 20);
                    str = str.offset(1);
                }
                current_block_184 = 3166194604430448652;
            }
            '?' => {
                /*  10      */
                doop(operations::question, 10);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '+' => {
                /*  19      */
                doop(operations::add, 19);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '-' => {
                /*  19: -   (or - unary)        */
                if state.expressions.lastWasOp {
                    doop(operations::negate, 128);
                } else {
                    doop(operations::subtract, 19);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '>' => {
                /*  18: >> <<  17: > >= <= <    */
                if state.expressions.lastWasOp {
                    doop(operations::take_most_significant_byte, 128);
                    str = str.offset(1);
                } else {
                    if *str.offset(1) as i32 == '>' as i32 {
                        doop(operations::shift_right, 18);
                        str = str.offset(1);
                    } else if *str.offset(1) as i32 == '=' as i32 {
                        doop(operations::greater_or_equal, 17);
                        str = str.offset(1);
                    } else {
                        doop(operations::greater, 17);
                    }
                    str = str.offset(1);
                }
                current_block_184 = 3166194604430448652;
            }
            '<' => {
                if state.expressions.lastWasOp {
                    doop(operations::take_least_significant_byte, 128);
                    str = str.offset(1);
                } else {
                    if *str.offset(1) as i32 == '<' as i32 {
                        doop(operations::shift_left, 18);
                        str = str.offset(1);
                    } else if *str.offset(1) as i32 == '=' as i32 {
                        doop(operations::lesser_or_equal, 17);
                        str = str.offset(1)
                    } else {
                        doop(operations::lesser, 17);
                    }
                    str = str.offset(1);
                }
                current_block_184 = 3166194604430448652;
            }
            '=' => {
                /*  16: ==  (= same as ==)      */
                if *str.offset(1) as i32 == '=' as i32 {
                    str = str.offset(1);
                }
                doop(operations::equal, 16);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '!' => {
                /*  16: !=                      */
                if state.expressions.lastWasOp {
                    doop(operations::not, 128);
                } else {
                    doop(operations::not_equal, 16);
                    str = str.offset(1);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '&' => {
                /*  15: &   12: &&              */
                if *str.offset(1) as i32 == '&' as i32 {
                    doop(operations::and_and, 12);
                    str = str.offset(1);
                } else {
                    doop(operations::and, 15);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '^' => {
                /*  14: ^                       */
                doop(operations::xor, 14);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '|' => {
                /*  13: |   11: ||              */
                if *str.offset(1) as i32 == '|' as i32 {
                    doop(operations::or_or, 11);
                    str = str.offset(1);
                } else {
                    doop(operations::or, 13);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '(' => {
                if wantmode != 0 {
                    (*cur).addrmode = AddressModes::IndWord as u8;
                    str = str.offset(1);
                    current_block_184 = 3166194604430448652;
                } else { current_block_184 = 18384894229789369419; }
            }
            '[' => { current_block_184 = 18384894229789369419; }
            ')' => {
                if wantmode != 0 {
                    if (*cur).addrmode == AddressModes::IndWord as u8 && *str.offset(1) as u8 == ',' as u8 && *str.offset(2) as u8 | 0x20 == 'y' as u8 {
                        (*cur).addrmode = AddressModes::IndByteY as u8;
                        str = str.offset(2)
                    }
                    //FIX: detect illegal opc (zp),x syntax...
                    if (*cur).addrmode == AddressModes::IndByteY as u8 && *str.offset(1) as u8 == ',' as u8 && *str.offset(2) as u8 | 0x20 == 'x' as u8 {
                        // FIXME: strangely, this is never used, so we have it here but commented out
                        // let buffer: String = transient::str_pointer_to_string(str);
                        asmerr(AsmErrorEquates::IllegalAddressingMode, false, pLine);
                        state.execution.redoIndex += 1;
                        state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved
                        // We treat the opcode as valid to allow passes to continue, which should
                        // allow other errors (like phase errros) to resolve before our "++Redo"
                        // ultimately forces a failure.
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
            ',' => { // ',' - FIXME: convert back to original char
                while state.expressions.opFunc.len() != state.expressions.opIndexBase {
                    evaltop();
                }
                state.expressions.lastWasOp = true;
                scr = *str.offset(1) as i32 | 0x20 as i32;
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
                    let pNewSymbol: *mut _SYMBOL = allocsymbol();
                    (*cur).next = pNewSymbol;
                    let arg_stack = state.expressions.argStack.pop().unwrap();
                    let arg_flags = state.expressions.argFlags.pop().unwrap();
                    let arg_string = state.expressions.argString.pop().unwrap();
                    if state.expressions.argStack.len() < state.expressions.argIndexBase {
                        asmerr(AsmErrorEquates::SyntaxError, false, pLine);
                    }
                    if state.expressions.argStack.len() > state.expressions.argIndexBase {
                        asmerr(AsmErrorEquates::SyntaxError, false, pLine);
                    }
                    (*cur).value = arg_stack;
                    (*cur).flags = arg_flags;
                    (*cur).string = if arg_string.is_empty() { 0 as *mut i8 } else { transient::string_to_str_pointer(arg_string) };
                    if !(*cur).string.is_null() {
                        (*cur).flags = ((*cur).flags as i32 | 0x8 as i32) as u8;
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
            '\"' => {
                str = pushstr(str.offset(1));
                current_block_184 = 3166194604430448652;
            }
            _ => {
                let mut dol: *const i8 = str;
                while *dol as i32 >= '0' as i32 && *dol as i32 <= '9' as i32 {
                    dol = dol.offset(1)
                }
                if *dol as i32 == '$' as i32 {
                    str = pushsymbol(str)
                } else if *str as i32 == '0' as i32 {
                    str = pushoct(str)
                } else if *str as i32 > '0' as i32 && *str as i32 <= '9' as i32 {
                    str = pushdec(str)
                } else { str = pushsymbol(str) }
                current_block_184 = 3166194604430448652;
            }
        }
        match current_block_184 {
            8741107198128373303 =>
            /* fall thru OK */
            {
                while state.expressions.opFunc.len() != state.expressions.opIndexBase && *state.expressions.opPri.last().unwrap() != 0 {
                    evaltop();
                }
                if state.expressions.opFunc.len() != state.expressions.opIndexBase {
                    state.expressions.opFunc.pop();
                    state.expressions.opPri.pop();
                }
                str = str.offset(1);
                if state.expressions.argStack.len() == state.expressions.argIndexBase {
                    println!("\']\' error, no arg on stack");
                } else {
                    if *str as i32 == 'd' as i32 {
                        /*  STRING CONVERSION   */
                        str = str.offset(1);
                        if *state.expressions.argFlags.last().unwrap() == 0 {
                            let buffer = format!("{}", state.expressions.argStack.last().unwrap());
                            let new_arg_string_pos = state.expressions.argFlags.len() - 1;
                            if state.expressions.argString.len() > new_arg_string_pos {
                                state.expressions.argString[new_arg_string_pos] = buffer;
                            } else {
                                state.expressions.argString.push(buffer);
                            }
                        }
                    }
                    state.expressions.lastWasOp = false
                }
            }
            18384894229789369419 =>
            /* fall thru OK */
            /*  eventually an argument      */
            {
                if state.expressions.opFunc.len() == MAX_OPS {
                    println!("too many ops");
                } else {
                    state.expressions.opFunc.push(operations::noop);
                    state.expressions.opPri.push(0);
                }
                str = str.offset(1)
            }
            _ => { }
        }
    }
    while state.expressions.opFunc.len() != state.expressions.opIndexBase {
        evaltop();
    }
    if state.expressions.argStack.len() != state.expressions.argIndexBase {
        (*cur).value = state.expressions.argStack.pop().unwrap();
        (*cur).flags = state.expressions.argFlags.pop().unwrap();
        let arg_string = state.expressions.argString.pop().unwrap();
        (*cur).string = if arg_string.is_empty() { 0 as *mut i8 } else { transient::string_to_str_pointer(arg_string) };
        if !(*cur).string.is_null() {
            (*cur).flags = ((*cur).flags as i32 | 0x8 as i32) as u8;
            if state.parameters.debug {
                println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
            }
        }
        if (*base).addrmode as i32 == 0 {
            (*base).addrmode = AddressModes::ByteAdr as i32 as u8
        }
    }
    if state.expressions.argStack.len() != state.expressions.argIndexBase || state.expressions.opFunc.len() != state.expressions.opIndexBase {
        asmerr(AsmErrorEquates::SyntaxError, false, pLine);
    }
    state.expressions.argStack.truncate(state.expressions.argIndexBase);
    state.expressions.argFlags.truncate(state.expressions.argIndexBase);
    state.expressions.argString.truncate(state.expressions.argIndexBase);
    state.expressions.opFunc.truncate(state.expressions.opIndexBase);
    state.expressions.opPri.truncate(state.expressions.opIndexBase);
    state.expressions.argIndexBase = oldArgIndexBase;
    state.expressions.opIndexBase = oldOpIndexBase;
    return base;
}

pub unsafe fn execute_op_func(op_func: ExpressionOperationFunc, v1: i64, v2: i64, f1: i32, f2: i32) {
    match op_func(v1, v2, f1, f2) {
        Ok(value) => {
            let (val, flags, wasOp) = value;
            stackarg(val, flags, 0 as *const i8);
            if wasOp {
                state.expressions.lastWasOp = true;
            }
        }
        Err(error) => {
            asmerr(error, true, 0 as *const i8);
            // Still execute something, as the original code does that
            stackarg(0, 0, 0 as *const i8);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn evaltop() {
    if state.parameters.debug {
        println!("evaltop @(A,O) {} {}", state.expressions.argStack.len(), state.expressions.opFunc.len());
    }
    if state.expressions.opFunc.len() <= state.expressions.opIndexBase {
        asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
        state.expressions.opFunc.truncate(state.expressions.opIndexBase);
        state.expressions.opPri.truncate(state.expressions.opIndexBase);
        return;
    }
    let op_func = state.expressions.opFunc.pop().unwrap();
    let op_pri = state.expressions.opPri.pop().unwrap();
    if op_pri == 128 {
        if state.expressions.argStack.len() < state.expressions.argIndexBase + 1 {
            asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
            state.expressions.argStack.truncate(state.expressions.argIndexBase);
            state.expressions.argFlags.truncate(state.expressions.argIndexBase);
            state.expressions.argString.truncate(state.expressions.argIndexBase);
            return;
        }
        state.expressions.argString.truncate(state.expressions.argStack.len() - 1);
        execute_op_func(
            op_func,
            state.expressions.argStack.pop().unwrap(),
            0,
            state.expressions.argFlags.pop().unwrap() as i32,
            0,
        );
    } else {
        if state.expressions.argStack.len() < state.expressions.argIndexBase + 2 {
            asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
            state.expressions.argStack.truncate(state.expressions.argIndexBase);
            state.expressions.argFlags.truncate(state.expressions.argIndexBase);
            state.expressions.argString.truncate(state.expressions.argIndexBase);
            return;
        }
        state.expressions.argString.truncate(state.expressions.argStack.len() - 2);
        let s0 = state.expressions.argStack.pop().unwrap();
        let f0 = state.expressions.argFlags.pop().unwrap() as i32;
        execute_op_func(
            op_func,
            state.expressions.argStack.pop().unwrap(),
            s0,
            state.expressions.argFlags.pop().unwrap() as i32,
            f0,
        );
    };
}
unsafe extern "C" fn stackarg(mut val: i64, mut flags: i32, ptr1: *const i8) {
    let mut str: *mut i8 = 0 as *mut i8;
    if state.parameters.debug {
        println!("stackarg {} (@{})", val, state.expressions.argStack.len());
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
    state.expressions.argStack.push(val);
    state.expressions.argFlags.push(flags as u8); // FIXME: truncate, check source flags type...
    if str.is_null() {
        state.expressions.argString.push(String::new());
    } else {
        state.expressions.argString.push(transient::str_pointer_to_string(str));
    }
    if state.expressions.argStack.len() == MAX_ARGS {
        println!("stackarg: maxargs stacked");
        state.expressions.argStack.truncate(state.expressions.argIndexBase);
        state.expressions.argFlags.truncate(state.expressions.argIndexBase);
        state.expressions.argString.truncate(state.expressions.argIndexBase);
    }
    while state.expressions.opFunc.len() != state.expressions.opIndexBase && *state.expressions.opPri.last().unwrap() == 128 {
        evaltop();
    };
}
#[no_mangle]
pub unsafe extern "C" fn doop(func: ExpressionOperationFunc, pri: usize) {
    if state.parameters.debug {
        println!("doop");
    }
    state.expressions.lastWasOp = true;
    if state.expressions.opFunc.len() == state.expressions.opIndexBase || pri == 128 {
        if state.parameters.debug {
            println!("doop @ {} unary", state.expressions.opFunc.len());
        }
        state.expressions.opFunc.push(func);
        state.expressions.opPri.push(pri);
        return;
    }
    while state.expressions.opFunc.len() != state.expressions.opIndexBase && *state.expressions.opPri.last().unwrap() != 0 && pri <= *state.expressions.opPri.last().unwrap() {
        evaltop();
    }
    if state.parameters.debug {
        println!("doop @ {}", state.expressions.opFunc.len());
    }
    state.expressions.opFunc.push(func);
    state.expressions.opPri.push(pri);
    if state.expressions.opFunc.len() == MAX_OPS {
        println!("doop: too many operators");
        state.expressions.opFunc.truncate(state.expressions.opIndexBase);
        state.expressions.opPri.truncate(state.expressions.opIndexBase);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pushchar(mut str: *const i8) -> *const i8 {
    if *str != 0 {
        stackarg(*str as i64, 0, 0 as *const i8);
        str = str.offset(1);
    } else {
        stackarg(' ' as i32 as i64, 0, 0 as *const i8);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushhex(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    loop  {
        if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
            val = (val << 4) + (*str as i32 - '0' as i32) as i64
        } else {
            if !(*str as i32 >= 'a' as i32 && *str as i32 <= 'f' as i32 || *str as i32 >= 'A' as i32 && *str as i32 <= 'F' as i32) {
                break;
            }
            val = (val << 4) + ((*str as i32 & 0x1f as i32) + 9) as i64
        }
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushoct(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 >= '0' as i32 && *str as i32 <= '7' as i32 {
        val = (val << 3) + (*str as i32 - '0' as i32) as i64;
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushdec(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
        val = val * 10 + (*str as i32 - '0' as i32) as i64;
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushbin(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 == '0' as i32 || *str as i32 == '1' as i32 {
        val = val << 1 | (*str as i32 - '0' as i32) as i64;
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushstr(mut str: *const i8) -> *const i8 {
    stackarg(0, 0x8 as i32, str);
    while *str as i32 != 0 && *str as i32 != '\"' as i32 {
        str = str.offset(1);
    }
    if *str as i32 == '\"' as i32 { str = str.offset(1) }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pushsymbol(str: *const i8) -> *const i8 {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut ptr: *const i8 = 0 as *const i8;
    let mut macro_0: u8 = 0;
    ptr = str;
    while *ptr as i32 == '_' as i32 || *ptr as i32 == '.' as i32 || *ptr as i32 >= 'a' as i32 && *ptr as i32 <= 'z' as i32 || *ptr as i32 >= 'A' as i32 && *ptr as i32 <= 'Z' as i32 || *ptr as i32 >= '0' as i32 && *ptr as i32 <= '9' as i32 {
        ptr = ptr.offset(1);
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
    sym = findsymbol(str, ptr.wrapping_offset_from(str) as i64 as i32);
    if !sym.is_null() {
        if (*sym).flags as i32 & 0x1 as i32 != 0 {
            state.execution.redoEval += 1;
        }
        if (*sym).flags as i32 & 0x20 as i32 != 0 {
            macro_0 = 1;
            sym = eval((*sym).string, 0);
        }
        if (*sym).flags as i32 & 0x8 as i32 != 0 {
            stackarg(0, 0x8 as i32, (*sym).string);
        } else {
            stackarg((*sym).value, (*sym).flags as i32 & 0x1 as i32, 0 as *const i8);
        }
        (*sym).flags = ((*sym).flags as i32 | (0x4 as i32 | 0x40 as i32)) as u8;
        if macro_0 != 0 { FreeSymbolList(sym); }
    } else {
        stackarg(0, 0x1 as i32, 0 as *const i8);
        sym = CreateSymbol(str, ptr.wrapping_offset_from(str) as i64 as i32);
        (*sym).flags = (0x4 as i32 | 0x40 as i32 | 0x1 as i32) as u8;
        state.execution.redoEval += 1;
    }
    return ptr;
}
