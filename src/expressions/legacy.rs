use libc;

// FIXME: remove these unsafe calls coming from main.rs
use crate::{
    asmerr,
    OPTIONS,
};
use crate::expressions::{
    is_alpha_num,
    operations,
};
use crate::expressions::operations::{
    ExpressionOperationFunc,
};
use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SymbolTypes,
};
use crate::types::enums::{
    AddressModes,
    AsmErrorEquates,
};
use crate::types::structs::{
    ExpressionStackArgument,
    ExpressionStackOperation,
};
use crate::types::legacy::{
    _SYMBOL,
};
use crate::utils::{
    filesystem,
    transient,
};

#[cfg(debug_assertions)]
use crate::{
    log_function,
    log_function_with,
};

pub const MAX_OPS: usize = 32;
pub const MAX_ARGS: usize = 64;

extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut base: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut cur: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let old_argument_len_base = state.expressions.argument_len_base;
    let old_operation_len_base = state.expressions.operation_len_base;
    let mut scr: i32 = 0;
    let pLine: *const i8 = str;
    state.expressions.argument_len_base = state.expressions.arguments.len();
    state.expressions.operation_len_base = state.expressions.operations.len();
    state.expressions.last_was_operation = true;
    cur = allocsymbol();
    base = cur;
    while *str != 0 {
        if OPTIONS.debug {
            println!("char '{}'", transient::str_pointer_and_len_to_string(str, 1));
        }
        let current_block_184: u64;
        match *str as u8 as char {
            ' ' | '\n' => {
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '~' => {
                if state.expressions.last_was_operation {
                    doop(operations::invert, 128);
                } else {
                    asmerr(AsmErrorEquates::SyntaxError, false, pLine);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '*' => {
                if state.expressions.last_was_operation {
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
                if state.expressions.last_was_operation {
                    str = pushbin(str.offset(1));
                } else {
                    doop(operations::modulo, 20);
                    str = str.offset(1);
                }
                current_block_184 = 3166194604430448652;
            }
            '?' => {
                // 10
                doop(operations::question, 10);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '+' => {
                // 19
                doop(operations::add, 19);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '-' => {
                // 19: - (or - unary)
                if state.expressions.last_was_operation {
                    doop(operations::negate, 128);
                } else {
                    doop(operations::subtract, 19);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '>' => {
                //  18: >> <<
                // 17: > >= <= <
                if state.expressions.last_was_operation {
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
                if state.expressions.last_was_operation {
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
                //  16: == (= same as ==)
                if *str.offset(1) as i32 == '=' as i32 {
                    str = str.offset(1);
                }
                doop(operations::equal, 16);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '!' => {
                // 16: !=
                if state.expressions.last_was_operation {
                    doop(operations::not, 128);
                } else {
                    doop(operations::not_equal, 16);
                    str = str.offset(1);
                }
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '&' => {
                // 15: &
                // 12: &&
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
                // 14: ^
                doop(operations::xor, 14);
                str = str.offset(1);
                current_block_184 = 3166194604430448652;
            }
            '|' => {
                // 13: |
                // 11: ||
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
                    // FIX: detect illegal opc (zp),x syntax...
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
                // No other addressing mode is possible from now on
                // so we might as well allow () instead of [].
                wantmode = 0; /* to lower case */
                current_block_184 = 3166194604430448652;
            }
            ',' => {
                while state.expressions.operations.len() != state.expressions.operation_len_base {
                    evaltop();
                }
                state.expressions.last_was_operation = true;
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
                    // FIX: detect illegal opc (zp,y) syntax...
                    // we treat the opcode as valid to allow passes to continue, which should
                    // allow other errors (like phase errros) to resolve before our "++Redo"
                    // ultimately forces a failure.
                    (*cur).addrmode = AddressModes::ZeroY as u8;
                    str = str.offset(1)
                } else if scr == 'x' as i32 && !is_alpha_num(*str.offset(2) as u8 as char) {
                    (*cur).addrmode = AddressModes::ZeroX as u8;
                    str = str.offset(1);
                    // FIX: OPCODE.FORCE needs to be adjusted for x indexing...
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
                    // FIX: OPCODE.FORCE needs to be adjusted for x indexing...
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
                    let new_symbol: *mut _SYMBOL = allocsymbol();
                    state.execution.symbols.push(new_symbol);
                    let last_argument = state.expressions.arguments.pop().unwrap();
                    if state.expressions.arguments.len() < state.expressions.argument_len_base {
                        asmerr(AsmErrorEquates::SyntaxError, false, pLine);
                    }
                    if state.expressions.arguments.len() > state.expressions.argument_len_base {
                        asmerr(AsmErrorEquates::SyntaxError, false, pLine);
                    }
                    (*cur).value = last_argument.value;
                    (*cur).flags = last_argument.flags;
                    (*cur).string = match last_argument.string {
                        Some(result) => transient::string_to_str_pointer(result),
                        None => 0 as *mut i8,
                    };
                    if !(*cur).string.is_null() {
                        (*cur).flags |= SymbolTypes::StringResult;
                        if OPTIONS.debug {
                            println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
                        }
                    }
                    cur = new_symbol;
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
            8741107198128373303 => {
                while state.expressions.operations.len() != state.expressions.operation_len_base && state.expressions.operations.last().unwrap().pri != 0 {
                    evaltop();
                }
                if state.expressions.operations.len() != state.expressions.operation_len_base {
                    state.expressions.operations.pop();
                }
                str = str.offset(1);
                if state.expressions.arguments.len() == state.expressions.argument_len_base {
                    println!("\']\' error, no arg on stack");
                } else {
                    if *str as i32 == 'd' as i32 {
                        /*  STRING CONVERSION   */
                        str = str.offset(1);
                        let last_argument = state.expressions.arguments.last_mut().unwrap();
                        if last_argument.flags == 0 {
                            let buffer = format!("{}", last_argument.value);
                            last_argument.string = Some(buffer);
                        }
                    }
                    state.expressions.last_was_operation = false;
                }
            }
            18384894229789369419 => {
                /*  eventually an argument      */
                if state.expressions.operations.len() == MAX_OPS {
                    println!("too many ops");
                } else {
                    state.expressions.operations.push(
                        ExpressionStackOperation {
                            func: None,
			                pri: 0,
                        }
                    );
                }
                str = str.offset(1);
            }
            _ => { }
        }
    }
    while state.expressions.operations.len() != state.expressions.operation_len_base {
        evaltop();
    }
    if state.expressions.arguments.len() != state.expressions.argument_len_base {
        let last_argument = state.expressions.arguments.pop().unwrap();
        (*cur).value = last_argument.value;
        (*cur).flags = last_argument.flags;
        (*cur).string = match last_argument.string {
            Some(result) => transient::string_to_str_pointer(result),
            None => 0 as *mut i8
        };
        if !(*cur).string.is_null() {
            (*cur).flags |= SymbolTypes::StringResult;
            if OPTIONS.debug {
                println!("STRING: {}", transient::str_pointer_to_string((*cur).string));
            }
        }
        if (*base).addrmode as i32 == 0 {
            (*base).addrmode = AddressModes::ByteAdr as i32 as u8;
        }
    }
    if state.expressions.arguments.len() != state.expressions.argument_len_base || state.expressions.operations.len() != state.expressions.operation_len_base {
        asmerr(AsmErrorEquates::SyntaxError, false, pLine);
    }
    state.expressions.arguments.truncate(state.expressions.argument_len_base);
    state.expressions.operations.truncate(state.expressions.operation_len_base);
    state.expressions.argument_len_base = old_argument_len_base;
    state.expressions.operation_len_base = old_operation_len_base;
    return base;
}

pub unsafe fn execute_op_func(op_func: ExpressionOperationFunc, v1: i64, v2: i64, f1: u8, f2: u8) {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("{} {} {} {}", v1, v2, f1, f2); } }

    match op_func(v1, v2, f1, f2) {
        Ok(value) => {
            let (val, flags, wasOp) = value;
            stackarg(val, flags, 0 as *const i8);
            if wasOp {
                state.expressions.last_was_operation = true;
            }
        }
        Err(error) => {
            asmerr(error, true, 0 as *const i8);
            // Conversion note: still executes something, as the original code does that
            stackarg(0, 0, 0 as *const i8);
        }
    }

    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("state.expressions.last_was_operation = {}", state.expressions.last_was_operation); } }
}

pub unsafe fn evaltop() {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function!(); } }

    if OPTIONS.debug {
        println!("evaltop @(A,O) {} {}", state.expressions.arguments.len(), state.expressions.operations.len());
    }
    if state.expressions.operations.len() <= state.expressions.operation_len_base {
        asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
        state.expressions.operations.truncate(state.expressions.operation_len_base);
        return;
    }
    let operation = state.expressions.operations.pop().unwrap();
    if operation.pri == 128 {
        if state.expressions.arguments.len() < state.expressions.argument_len_base + 1 {
            asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
            state.expressions.arguments.truncate(state.expressions.argument_len_base);
            return;
        }
        let argument = state.expressions.arguments.pop().unwrap();
        if operation.func.is_some() {
            execute_op_func(
                operation.func.unwrap(),
                argument.value,
                0,
                argument.flags,
                0,
            );
        }
    } else {
        if state.expressions.arguments.len() < state.expressions.argument_len_base + 2 {
            asmerr(AsmErrorEquates::SyntaxError, false, 0 as *const i8);
            state.expressions.arguments.truncate(state.expressions.argument_len_base);
            return;
        }
        let argument1 = state.expressions.arguments.pop().unwrap();
        let argument0 = state.expressions.arguments.pop().unwrap();
        if operation.func.is_some() {
            execute_op_func(
                operation.func.unwrap(),
                argument0.value,
                argument1.value,
                argument0.flags,
                argument1.flags,
            );
        }
    };
}
unsafe fn stackarg(mut val: i64, mut flags: u8, ptr1: *const i8) {
    let mut str: *mut i8 = 0 as *mut i8;
    if OPTIONS.debug {
        println!("stackarg {} (@{})", val, state.expressions.arguments.len());
        #[cfg(debug_assertions)]
        { if OPTIONS.debug_extended { log_function_with!("{} {} [[{}]]", val, flags, if ptr1.is_null() { String::from("null") } else { transient::str_pointer_to_string(ptr1) }); } }
    }
    state.expressions.last_was_operation = false;
    if flags & SymbolTypes::StringResult != 0 {
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
        new = transient::ckmalloc(len + 1);
        memcpy(new as *mut libc::c_void, ptr1 as *const libc::c_void, len as u64);
        *new.offset(len as isize) = 0;
        flags &= !(SymbolTypes::StringResult);
        str = new
    }
    state.expressions.arguments.push(
        ExpressionStackArgument {
            value: val,
            flags,
            string: if str.is_null() {
                None
            } else {
                Some(transient::str_pointer_to_string(str))
            }
        }
    );
    if state.expressions.arguments.len() == MAX_ARGS {
        println!("stackarg: maxargs stacked");
        state.expressions.arguments.truncate(state.expressions.argument_len_base);
    }
    while state.expressions.operations.len() != state.expressions.operation_len_base && state.expressions.operations.last().unwrap().pri == 128 {
        evaltop();
    };
}
pub unsafe fn doop(func: ExpressionOperationFunc, pri: usize) {
    if OPTIONS.debug {
        println!("doop");
    }
    state.expressions.last_was_operation = true;
    if state.expressions.operations.len() == state.expressions.operation_len_base || pri == 128 {
        if OPTIONS.debug {
            println!("doop @ {} unary", state.expressions.operations.len());
        }
        state.expressions.operations.push(
            ExpressionStackOperation {
                func: Some(func),
                pri,
            }
        );
        return;
    }
    while state.expressions.operations.len() != state.expressions.operation_len_base && state.expressions.operations.last().unwrap().pri != 0 && pri <= state.expressions.operations.last().unwrap().pri {
        evaltop();
    }
    if OPTIONS.debug {
        println!("doop @ {}", state.expressions.operations.len());
    }
    state.expressions.operations.push(
        ExpressionStackOperation {
            func: Some(func),
            pri,
        }
    );
    if state.expressions.operations.len() == MAX_OPS {
        println!("doop: too many operators");
        state.expressions.operations.truncate(state.expressions.operation_len_base);
    };
}
pub unsafe fn pushchar(mut str: *const i8) -> *const i8 {
    if *str != 0 {
        stackarg(*str as i64, 0, 0 as *const i8);
        str = str.offset(1);
    } else {
        stackarg(' ' as i32 as i64, 0, 0 as *const i8);
    }
    return str;
}
pub unsafe fn pushhex(mut str: *const i8) -> *const i8 {
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
pub unsafe fn pushoct(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 >= '0' as i32 && *str as i32 <= '7' as i32 {
        val = (val << 3) + (*str as i32 - '0' as i32) as i64;
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
pub unsafe fn pushdec(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
        val = val * 10 + (*str as i32 - '0' as i32) as i64;
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
pub unsafe fn pushbin(mut str: *const i8) -> *const i8 {
    let mut val: i64 = 0;
    while *str as i32 == '0' as i32 || *str as i32 == '1' as i32 {
        val = val << 1 | (*str as i32 - '0' as i32) as i64;
        str = str.offset(1);
    }
    stackarg(val, 0, 0 as *const i8);
    return str;
}
pub unsafe fn pushstr(mut str: *const i8) -> *const i8 {
    stackarg(0, SymbolTypes::StringResult, str);
    while *str as i32 != 0 && *str as i32 != '\"' as i32 {
        str = str.offset(1);
    }
    if *str as i32 == '\"' as i32 { str = str.offset(1) }
    return str;
}
pub unsafe fn pushsymbol(str: *const i8) -> *const i8 {
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
        if (*sym).flags & SymbolTypes::Unknown != 0 {
            state.execution.redoEval += 1;
        }
        if (*sym).flags & SymbolTypes::Macro != 0 {
            macro_0 = 1;
            sym = eval((*sym).string, 0);
        }
        if (*sym).flags & SymbolTypes::StringResult != 0 {
            stackarg(0, SymbolTypes::StringResult, (*sym).string);
        } else {
            stackarg((*sym).value, (*sym).flags & SymbolTypes::Unknown, 0 as *const i8);
        }
        (*sym).flags |= SymbolTypes::Referenced | SymbolTypes::MasterReference;
        if macro_0 != 0 {
            FreeSymbolList(sym);
        }
    } else {
        stackarg(0, SymbolTypes::Unknown, 0 as *const i8);
        sym = CreateSymbol(str, ptr.wrapping_offset_from(str) as i64 as i32);
        (*sym).flags |= SymbolTypes::Referenced | SymbolTypes::MasterReference | SymbolTypes::Unknown;
        state.execution.redoEval += 1;
    }
    return ptr;
}
