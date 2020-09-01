use crate::{
    log_function_with,
};
use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SegmentTypes,
};
use crate::types::enums::{
    AddressModes,
    AsmErrorEquates,
};
use crate::types::legacy::{
    _MNE,
    _SYMBOL,
};
use crate::utils::{
    transient,
};

extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const i8) -> i32;
    #[no_mangle]
    fn FreeSymbolList(sym: *mut _SYMBOL);
    #[no_mangle]
    fn programlabel();
    /* ops.c */
    #[no_mangle]
    fn generate();
    #[no_mangle]
    fn v_dc(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_ds(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_mnemonic(str: *mut i8, mne: *mut _MNE);
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const i8, wantmode: i32) -> *mut _SYMBOL;
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/*
 *  MNEF8.C
 *
 *  Fairchild F8 support code for DASM
 *  Copyright (c) 2004 by Thomas Mathys.
 */
/*
 * special registers. must use numbers from 16 and up,
 * since numbers below 16 are used for scratchpad registers.
 *
 * there is no REG_J, since J is really just scratchpad register 9.
 */
pub type REGISTERS = u32;
pub const REG_NONE: REGISTERS = 29;
pub const REG_W: REGISTERS = 28;
pub const REG_QL: REGISTERS = 27;
pub const REG_QU: REGISTERS = 26;
pub const REG_Q: REGISTERS = 25;
pub const REG_PC1: REGISTERS = 24;
pub const REG_PC0: REGISTERS = 23;
pub const REG_KL: REGISTERS = 22;
pub const REG_KU: REGISTERS = 21;
pub const REG_K: REGISTERS = 20;
pub const REG_IS: REGISTERS = 19;
pub const REG_H: REGISTERS = 18;
pub const REG_DC0: REGISTERS = 17;
pub const REG_A: REGISTERS = 16;
/*
 * used to print error messages.
 * mnename and opstring are copied into a single error message,
 * which is passed to asmerr.
 *
 * err      : error code (ERROR_xxx constant, passed to asmerr)
 * mnename  : name of the mnemonic
 * opstring : operand string
 * abort    : false = don't abort assembly
 *            true = abort assembly
 */
unsafe extern "C" fn f8err(err: AsmErrorEquates, mnename: *const i8, opstring: *const i8, bAbort: bool) {
    let mut buffer = String::new();
    buffer.push_str(transient::str_pointer_to_string(mnename).as_str());
    buffer.push_str(" ");
    buffer.push_str(transient::str_pointer_to_string(opstring).as_str());
    asmerr(err, bAbort, transient::string_to_str_pointer(buffer));
}
/*
 * emits a one byte opcode.
 */
unsafe extern "C" fn emit_opcode1(opcode: u8) {
    state.output.generatedLength = 1;
    state.output.generated[0] = opcode;
    generate();
}
/*
 * emits a two byte opcode
 *
 * byte0 : first byte (lower address)
 * byte1 : second byte (higher address)
 */
unsafe extern "C" fn emit_opcode2(byte0: u8, byte1: u8) {
    state.output.generatedLength = 2;
    state.output.generated[0] = byte0;
    state.output.generated[1] = byte1;
    generate();
}
/*
 * emits a three byte opcode
 *
 * byte0 : first byte (lowest address)
 * byte1 : second byte (middle address)
 * byte2 : third byte (highest address)
 */
unsafe extern "C" fn emit_opcode3(byte0: u8, byte1: u8, byte2: u8) {
    state.output.generatedLength = 3;
    state.output.generated[0] = byte0;
    state.output.generated[1] = byte1;
    state.output.generated[2] = byte2;
    generate();
}
/*
 * check wether the current program counter is known.
 *
 * result : zero = current program counter is unknown
 *          nonzero = current program counter is known
 */
unsafe extern "C" fn isPCKnown() -> i32 {
    let mut pcf: u8 = 0;
    let currentSegment = &state.other.segments[state.other.currentSegment];
    pcf = if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
        currentSegment.rflags
    } else {
        currentSegment.flags
    };
    return if pcf & (SegmentTypes::Unknown | 2) == 0 {
        1
    } else {
        0
    };
}
/*
 * returns the current program counter
 */
unsafe extern "C" fn getPC() -> i64 {
    let currentSegment = &state.other.segments[state.other.currentSegment];
    return if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
        currentSegment.rorg
    } else {
        currentSegment.org
    } as i64;
}
/*
 * attempts to parse a 32 bit unsigned value from a string.
 *
 * str    : string to parse the value from
 * value  : parsed value is stored here
 *
 * result : zero = ok or syntax error
 *          nonzero = unresolved expression
 */
unsafe fn parse_value(str: *const i8, value: *mut u64) -> i32 {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut result: i32 = 0;
    *value = 0;
    sym = eval(str, 0);
    if !(*sym).next.is_null() || AddressModes::ByteAdr as u8 != (*sym).addrmode {
        asmerr(AsmErrorEquates::SyntaxError, true, str);
    } else if (*sym).flags as i32 & 0x1 as i32 != 0 {
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved;
        result = 1
    } else { *value = (*sym).value as u64 }
    FreeSymbolList(sym);
    return result;
}
/*
 * attempts to parse a scratchpad register name.
 * register numbers are parsed as expressions.
 * if an expression is invalid, asmerr is called
 * and the assembly aborted.
 *
 * accepts the following input:
 *
 * - numbers 0..14 (as expressions, numbers 12-14 map to S, I and D)
 * - J  (alias for register  9)
 * - HU (alias for register 10)
 * - HL (alias for register 11)
 * - S and (IS)
 * - I and (IS)+
 * - D and (IS)-
 *
 * str    : string to parse the scratchpad register from
 * reg    : parsed scratchpad register is stored here.
 *          this is the value which will become the lower
 *          nibble of the opcodes.
 *
 * result : zero = ok or syntax error
 *          nonzero = unresolved expression
 */
unsafe fn parse_scratchpad_register(str: *mut i8, reg: *mut u8) -> i32 {
    let mut regnum: u64 = 0;
    /* parse special cases where ISAR is used as index */
    if strcasecmp(b"s\x00" as *const u8 as *const i8, str) == 0 || strcasecmp(b"(is)\x00" as *const u8 as *const i8, str) == 0 {
        *reg = 0xc;
        return 0;
    }
    if strcasecmp(b"i\x00" as *const u8 as *const i8, str) == 0 || strcasecmp(b"(is)+\x00" as *const u8 as *const i8, str) == 0 {
        *reg = 0xd;
        return 0;
    }
    if strcasecmp(b"d\x00" as *const u8 as *const i8, str) == 0 || strcasecmp(b"(is)-\x00" as *const u8 as *const i8, str) == 0 {
        *reg = 0xe;
        return 0;
    }
    /* parse aliases for scratchpad registers */
    if strcasecmp(b"j\x00" as *const u8 as *const i8, str) == 0 {
        *reg = 0x9;
        return 0;
    }
    if strcasecmp(b"hu\x00" as *const u8 as *const i8, str) == 0 {
        *reg = 0xa;
        return 0;
    }
    if strcasecmp(b"hl\x00" as *const u8 as *const i8, str) == 0 {
        *reg = 0xb;
        return 0;
    }
    /* parse register number */
    if parse_value(str, &mut regnum) != 0 {
        return 1;
        /* unresolved expr */
    } else {
        if regnum > 14 {
            asmerr(AsmErrorEquates::ValueMustBeLowerThanF, true, str);
        }
        *reg = regnum as u8;
        return 0;
    };
}
/*
 * attempts to parse a special register name from str
 *
 * result : one of the REG_xxx constants (possibly also REG_NONE)
 */
unsafe fn parse_special_register(str: *mut i8) -> i32 {
    if strcasecmp(b"a\x00" as *const u8 as *const i8, str) == 0 {
        return REG_A as i32;
    }
    if strcasecmp(b"dc0\x00" as *const u8 as *const i8, str) == 0 || strcasecmp(b"dc\x00" as *const u8 as *const i8, str) == 0 {
        return REG_DC0 as i32;
    }
    if strcasecmp(b"h\x00" as *const u8 as *const i8, str) == 0 {
        return REG_H as i32;
    }
    if strcasecmp(b"is\x00" as *const u8 as *const i8, str) == 0 {
        return REG_IS as i32;
    }
    if strcasecmp(b"k\x00" as *const u8 as *const i8, str) == 0 {
        return REG_K as i32;
    }
    if strcasecmp(b"ku\x00" as *const u8 as *const i8, str) == 0 {
        return REG_KU as i32;
    }
    if strcasecmp(b"kl\x00" as *const u8 as *const i8, str) == 0 {
        return REG_KL as i32;
    }
    if strcasecmp(b"pc0\x00" as *const u8 as *const i8, str) == 0 || strcasecmp(b"p0\x00" as *const u8 as *const i8, str) == 0 {
        return REG_PC0 as i32;
    }
    if strcasecmp(b"pc1\x00" as *const u8 as *const i8, str) == 0 || strcasecmp(b"p\x00" as *const u8 as *const i8, str) == 0 {
        return REG_PC1 as i32;
    }
    if strcasecmp(b"q\x00" as *const u8 as *const i8, str) == 0 {
        return REG_Q as i32;
    }
    if strcasecmp(b"qu\x00" as *const u8 as *const i8, str) == 0 {
        return REG_QU as i32;
    }
    if strcasecmp(b"ql\x00" as *const u8 as *const i8, str) == 0 {
        return REG_QL as i32;
    }
    if strcasecmp(b"w\x00" as *const u8 as *const i8, str) == 0 {
        return REG_W as i32;
    } else {
        return REG_NONE as i32;
    };
}
unsafe extern "C" fn v_ins_outs(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut operand: u64 = 0;
    programlabel();
    parse_value(str, &mut operand);
    if operand > 15 {
        f8err(AsmErrorEquates::ValueMustBeLowerThan10, (*mne).name, str, false);
    }
    emit_opcode1(((*mne).opcode[0] as u64 | operand & 15) as u8);
}
unsafe extern "C" fn v_sl_sr(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut operand: u64 = 0;
    programlabel();
    if parse_value(str, &mut operand) != 0 {
        /* unresolved expression, reserve space */
        emit_opcode1(0);
    } else {
        match operand {
            1 => {
                emit_opcode1((*mne).opcode[0] as u8);
            }
            4 => {
                emit_opcode1((*mne).opcode[0].wrapping_add(2) as u8);
            }
            _ => {
                f8err(AsmErrorEquates::ValueMustBeOneOrFour, (*mne).name,
                      str, false);
                emit_opcode1(0);
            }
        }
    };
}
unsafe extern "C" fn v_lis(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut operand: u64 = 0;
    programlabel();
    parse_value(str, &mut operand);
    if operand > 15 {
        f8err(AsmErrorEquates::ValueMustBeLowerThan10, (*mne).name, str, false);
    }
    emit_opcode1((0x70 | operand & 15) as u8);
}
unsafe extern "C" fn v_lisu_lisl(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut operand: u64 = 0;
    programlabel();
    parse_value(str, &mut operand);
    if operand > 7 {
        f8err(AsmErrorEquates::ValueMustBeLowerThan8, (*mne).name, str, false);
    }
    emit_opcode1(((*mne).opcode[0] as u64 | operand & 7) as u8);
}
/*
 * handles opcodes with a scratchpad register operand:
 * as, asd, ds, ns, xs
 */
unsafe extern "C" fn v_sreg_op(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut reg: u8 = 0;
    programlabel();
    parse_scratchpad_register(str, &mut reg);
    emit_opcode1(((*mne).opcode[0] | reg as u32) as u8);
}
unsafe extern "C" fn v_lr(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut i: i32 = 0;
    let mut ncommas: i32 = 0;
    let mut cindex: i32 = 0;
    let mut op1: *mut i8 = 0 as *mut i8;
    let mut op2: *mut i8 = 0 as *mut i8;
    let mut reg_dst: u8 = 0;
    let mut reg_src: u8 = 0;
    let mut opcode: i32 = 0;
    programlabel();
    /* a valid operand string must contain exactly one comma. find it. */
    ncommas = 0;
    cindex = 0;
    i = 0;
    while *str.offset(i as isize) != 0 {
        if ',' as i32 == *str.offset(i as isize) as i32 {
            ncommas += 1;
            cindex = i
        }
        i += 1
    }
    if 1 != ncommas {
        f8err(AsmErrorEquates::SyntaxError, (*mne).name, str, false);
        return;
    }
    /* extract operand strings  */
    *str.offset(cindex as isize) = 0;
    op1 = str;
    op2 = &mut *str.offset((cindex + 1) as isize) as *mut i8;
    if 0 != cindex && *(*__ctype_b_loc()).offset(*str.offset((cindex - 1) as isize) as i32 as isize) as i32 & _ISspace as i32 as u16 as i32 != 0 {
        *str.offset((cindex - 1) as isize) = 0
    }
    if *(*__ctype_b_loc()).offset(*op2 as i32 as isize) as i32
           & _ISspace as i32 as u16 as i32 != 0 {
        op2 = op2.offset(1)
    }
    /* parse operand strings for register names */
    reg_dst = parse_special_register(op1) as u8;
    if REG_NONE as i32 == reg_dst as i32 {
        if parse_scratchpad_register(op1, &mut reg_dst) != 0 {
            /* unresolved expression, reserve space */
            emit_opcode1(0);
            return;
        }
    }
    reg_src = parse_special_register(op2) as u8;
    if REG_NONE as i32 == reg_src as i32 {
        if parse_scratchpad_register(op2, &mut reg_src) != 0 {
            /* unresolved expression, reserve space */
            emit_opcode1(0);
            return;
        }
    }
    /* restore operand string */
    *str.offset(cindex as isize) = ',' as i32 as i8;
    if 0 != cindex && 0 == *str.offset((cindex - 1) as isize) as i32 {
        *str.offset((cindex - 1) as isize) = ' ' as i32 as i8
    }
    /* generate opcode */
    opcode = -(1);
    match reg_dst as i32 {
        16 => {
            /* lr a,xxx */
            match reg_src as i32 {
                19 => { opcode = 0xa as i32 }
                22 => { opcode = 0x1 as i32 }
                21 => { opcode = 0 }
                27 => { opcode = 0x3 as i32 }
                26 => { opcode = 0x2 as i32 }
                _ => {
                    if (reg_src as i32) < 15 {
                        opcode = 0x40 as i32 | reg_src as i32
                    }
                }
            }
        }
        17 => {
            match reg_src as i32 {
                18 => { opcode = 0x10 as i32 }
                25 => { opcode = 0xf as i32 }
                _ => { }
            }
        }
        18 => {
            if REG_DC0 as i32 == reg_src as i32 {
                opcode = 0x11 as i32
            }
        }
        19 => {
            if REG_A as i32 == reg_src as i32 {
                opcode = 0xb as i32
            }
        }
        20 => {
            if REG_PC1 as i32 == reg_src as i32 {
                opcode = 0x8 as i32
            }
        }
        22 => {
            if REG_A as i32 == reg_src as i32 {
                opcode = 0x5 as i32
            }
        }
        21 => {
            if REG_A as i32 == reg_src as i32 {
                opcode = 0x4 as i32
            }
        }
        23 => {
            if REG_Q as i32 == reg_src as i32 {
                opcode = 0xd as i32
            }
        }
        24 => {
            if REG_K as i32 == reg_src as i32 {
                opcode = 0x9 as i32
            }
        }
        25 => {
            if REG_DC0 as i32 == reg_src as i32 {
                opcode = 0xe as i32
            }
        }
        27 => {
            if REG_A as i32 == reg_src as i32 {
                opcode = 0x7 as i32
            }
        }
        26 => {
            if REG_A as i32 == reg_src as i32 {
                opcode = 0x6 as i32
            }
        }
        28 => {
            if 0x9 as i32 == reg_src as i32 {
                opcode = 0x1d as i32
            }
        }
        _ => {
            /* lr sreg,xxx*/
            if 15 > reg_dst as i32 && REG_A as i32 == reg_src as i32 {
                /* lr sreg,a */
                opcode = 0x50 as i32 | reg_dst as i32
            } else if 9 == reg_dst as i32 && REG_W as i32 == reg_src as i32 {
                /* special case : lr j,w */
                opcode = 0x1e as i32
            }
        }
    }
    if opcode < 0 {
        f8err(AsmErrorEquates::IllegalOperandCombination, (*mne).name,
              str, true);
    } else { emit_opcode1(opcode as u8); };
}
/*
 * generates branch opcodes
 *
 * opcode : opcode of the branch (for instance 0x8f for BR7)
 * str    : operand string
 */
unsafe fn generate_branch(opcode: u8, str: *const i8) {
    let mut target_adr: u64 = 0;
    let mut disp: i64 = 0;
    programlabel();
    /* get target address */
    if parse_value(str, &mut target_adr) != 0 {
        /* unresolved target address, reserve space */
        emit_opcode2(0, 0);
        return;
    }
    /* calculate displacement */
    if isPCKnown() != 0 {
        disp = target_adr.wrapping_sub(getPC() as u64).wrapping_sub(1) as i64;
        if disp > 127 || disp < -128 {
            let buffer = format!("{}", disp);
            asmerr(AsmErrorEquates::BranchOutOfRange, false, transient::string_to_str_pointer(buffer));
        }
    } else {
        /* unknown pc, will be (hopefully) resolved in future passes */
        disp = 0;
    }
    emit_opcode2(opcode, (disp & 255) as u8);
}
/*
 * handles the following branch mnemonics:
 * bc, bm, bnc, bno, bnz, bp, br, br7, bz
 */
unsafe extern "C" fn v_branch(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    generate_branch((*mne).opcode[0] as u8, str);
}
unsafe extern "C" fn v_bf_bt(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut ncommas: i32 = 0;
    let mut cindex: i32 = 0;
    let mut i: i32 = 0;
    let mut op1: *mut i8 = 0 as *mut i8;
    let mut op2: *mut i8 = 0 as *mut i8;
    let mut value: u64 = 0;
    /* a valid operand string must contain exactly one comma. find it. */
    ncommas = 0;
    cindex = 0;
    i = 0;
    while *str.offset(i as isize) != 0 {
        if ',' as i32 == *str.offset(i as isize) as i32 {
            ncommas += 1;
            cindex = i
        }
        i += 1
    }
    if 1 != ncommas {
        f8err(AsmErrorEquates::SyntaxError, (*mne).name, str, false);
        return;
    }
    /* extract operands */
    *str.offset(cindex as isize) = 0;
    op1 = str;
    op2 = &mut *str.offset((cindex + 1) as isize) as *mut i8;
    /* parse first operand*/
    if parse_value(op1, &mut value) != 0 {
        /* unresolved expression, reserve space */
        emit_opcode2(0, 0);
        return;
    }
    /* check first operand */
    *str.offset(cindex as isize) = ',' as i32 as i8; /* restore operand string */
    if 'f' as i32 == *(*mne).name.offset(1) as i32 {
        /* bf */
        if value > 15 {
            f8err(AsmErrorEquates::ValueMustBeLowerThan10, (*mne).name, str, false);
            value &= 15
        }
    } else if value > 7 {
        f8err(AsmErrorEquates::ValueMustBeLowerThan8, (*mne).name, str, false);
        value &= 7
    }
    generate_branch(((*mne).opcode[0] as u64
                         | value) as u8, op2);
}
/* bt */
/*
 * handles instructions that take a word operand:
 * dci, jmp, pi
 */
unsafe extern "C" fn v_wordop(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut value: u64 = 0;
    programlabel();
    parse_value(str, &mut value);
    if value > 0xffff {
        f8err(AsmErrorEquates::ValueMustBeLowerThan10000, (*mne).name, str, false);
    }
    emit_opcode3((*mne).opcode[0] as u8,
                 (value >> 8 & 0xff) as u8,
                 (value & 0xff) as u8);
}
/*
 * handles instructions that take a byte operand:
 * ai, ci, in, li, ni, oi, out, xi
 */
unsafe extern "C" fn v_byteop(str: *mut i8, mne: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut value: u64 = 0;
    programlabel();
    parse_value(str, &mut value);
    if value > 0xff {
        f8err(AsmErrorEquates::AddressMustBeLowerThan100, (*mne).name, str, false);
    }
    emit_opcode2((*mne).opcode[0] as u8, (value & 0xff) as u8);
}
#[no_mangle]
pub static mut MneF8: [_MNE; 59] = [{
            let init = _MNE{
                      vect: Some(v_ds as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"res\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_dc as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"db\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_dc as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"dw\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_dc as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"dd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"adc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x8e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ai\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"am\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"amd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sreg_op as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"as\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sreg_op as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"asd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xd0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_bf_bt as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bf\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bm\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bnc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bno\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bnz\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"br\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"br7\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x8f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_bf_bt as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bt\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_branch as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"bz\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ci\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"clr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"cm\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x8d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"com\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_wordop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"dci\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x2a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"di\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x1a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sreg_op as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ds\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ei\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x1b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"in\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"inc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x1f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_ins_outs as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ins\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xa0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_wordop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"jmp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"li\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_lis as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"lis\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_lisu_lisl as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"lisl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_lisu_lisl as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"lisu\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"lm\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"lnk\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_lr as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"lr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ni\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"nm\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x8a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"nop\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x2b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sreg_op as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"ns\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xf0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"oi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"om\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x8b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"out\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_ins_outs as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"outs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xb0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_wordop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"pi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"pk\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"pop\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x1c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sl_sr as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"sl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sl_sr as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"sr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"st\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"xdc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x2c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_byteop as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"xi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_mnemonic as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"xm\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0x8c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: Some(v_sreg_op as unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()),
                      name: b"xs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: ((1) << AddressModes::Imp as i32) as u64,
                      opcode: [0xe0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
            let init = _MNE{
                      vect: None,
                      name: 0 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         }]
;
