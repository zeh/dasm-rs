use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut Csegment: *mut _SEGMENT;
    #[no_mangle]
    static mut Redo_why: libc::c_ulong;
    #[no_mangle]
    static mut Redo: libc::c_int;
    #[no_mangle]
    fn asmerr(err: libc::c_int, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn ckmalloc(bytes: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn FreeSymbolList(sym: *mut _SYMBOL);
    #[no_mangle]
    fn programlabel();
    /* ops.c */
    #[no_mangle]
    static mut Gen: [libc::c_uchar; 0];
    #[no_mangle]
    static mut Glen: libc::c_int;
    #[no_mangle]
    fn generate();
    #[no_mangle]
    fn v_dc(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_ds(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_mnemonic(str: *mut libc::c_char, mne: *mut _MNE);
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const libc::c_char, wantmode: libc::c_int) -> *mut _SYMBOL;
}
pub type C2RustUnnamed = libc::c_uint;
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
pub struct _MNE {
    pub next: *mut _MNE,
    pub vect: Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut _MNE)
                         -> ()>,
    pub name: *const libc::c_char,
    pub flags: libc::c_uchar,
    pub okmask: libc::c_ulong,
    pub opcode: [libc::c_uint; 21],
}
/*      accumulatively true (not incl this one) */
/*      ORG unknown            */
/*      ORG referenced        */
/*      uninitialized area (U flag)    */
/*      relocatable origin active    */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SEGMENT {
    pub next: *mut _SEGMENT,
    pub name: *mut libc::c_char,
    pub flags: libc::c_uchar,
    pub rflags: libc::c_uchar,
    pub org: libc::c_ulong,
    pub rorg: libc::c_ulong,
    pub initorg: libc::c_ulong,
    pub initrorg: libc::c_ulong,
    pub initflags: libc::c_uchar,
    pub initrflags: libc::c_uchar,
}
/*      value unknown     */
/*      referenced        */
/*      result is a string    */
/*      SET instruction used    */
/*      symbol is a macro    */
/*      master reference    */
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
    the DASM macro assembler (aka small systems cross assembler)

    Copyright (c) 1988-2002 by Matthew Dillon.
    Copyright (c) 1995 by Olaf "Rhialto" Seibert.
    Copyright (c) 2003-2008 by Andrew Davie.
    Copyright (c) 2008 by Peter H. Froehlich.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/
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
pub type REGISTERS = libc::c_uint;
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
unsafe extern "C" fn f8err(mut err: libc::c_int,
                           mut mnename: *const libc::c_char,
                           mut opstring: *const libc::c_char,
                           mut bAbort: bool) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf =
        ckmalloc(strlen(mnename).wrapping_add(strlen(opstring)).wrapping_add(64
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                     as libc::c_int);
    strcpy(buf, mnename);
    strcat(buf, b" \x00" as *const u8 as *const libc::c_char);
    strcat(buf, opstring);
    asmerr(err, bAbort, buf);
    free(buf as *mut libc::c_void);
}
/*
 * emits a one byte opcode.
 */
unsafe extern "C" fn emit_opcode1(mut opcode: libc::c_uchar) {
    Glen = 1 as libc::c_int;
    *Gen.as_mut_ptr().offset(0 as libc::c_int as isize) = opcode;
    generate();
}
/*
 * emits a two byte opcode
 *
 * byte0 : first byte (lower address)
 * byte1 : second byte (higher address)
 */
unsafe extern "C" fn emit_opcode2(mut byte0: libc::c_uchar,
                                  mut byte1: libc::c_uchar) {
    Glen = 2 as libc::c_int;
    *Gen.as_mut_ptr().offset(0 as libc::c_int as isize) = byte0;
    *Gen.as_mut_ptr().offset(1 as libc::c_int as isize) = byte1;
    generate();
}
/*
 * emits a three byte opcode
 *
 * byte0 : first byte (lowest address)
 * byte1 : second byte (middle address)
 * byte2 : third byte (highest address)
 */
unsafe extern "C" fn emit_opcode3(mut byte0: libc::c_uchar,
                                  mut byte1: libc::c_uchar,
                                  mut byte2: libc::c_uchar) {
    Glen = 3 as libc::c_int;
    *Gen.as_mut_ptr().offset(0 as libc::c_int as isize) = byte0;
    *Gen.as_mut_ptr().offset(1 as libc::c_int as isize) = byte1;
    *Gen.as_mut_ptr().offset(2 as libc::c_int as isize) = byte2;
    generate();
}
/*
 * check wether the current program counter is known.
 *
 * result : zero = current program counter is unknown
 *          nonzero = current program counter is known
 */
unsafe extern "C" fn isPCKnown() -> libc::c_int {
    let mut pcf: libc::c_uchar = 0;
    pcf =
        if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            (*Csegment).rflags as libc::c_int
        } else { (*Csegment).flags as libc::c_int } as libc::c_uchar;
    return if pcf as libc::c_int & (0x1 as libc::c_int | 2 as libc::c_int) ==
                  0 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/*
 * returns the current program counter
 */
unsafe extern "C" fn getPC() -> libc::c_long {
    return if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0 {
               (*Csegment).rorg
           } else { (*Csegment).org } as libc::c_long;
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
unsafe extern "C" fn parse_value(mut str: *mut libc::c_char,
                                 mut value: *mut libc::c_ulong)
 -> libc::c_int {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut result: libc::c_int = 0 as libc::c_int;
    *value = 0 as libc::c_int as libc::c_ulong;
    sym = eval(str, 0 as libc::c_int);
    if !(*sym).next.is_null() ||
           AM_BYTEADR as libc::c_int != (*sym).addrmode as libc::c_int {
        asmerr(ERROR_SYNTAX_ERROR as libc::c_int, 1 as libc::c_int != 0, str);
    } else if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        Redo += 1;
        Redo_why |=
            REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as libc::c_ulong;
        result = 1 as libc::c_int
    } else { *value = (*sym).value as libc::c_ulong }
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
unsafe extern "C" fn parse_scratchpad_register(mut str: *mut libc::c_char,
                                               mut reg: *mut libc::c_uchar)
 -> libc::c_int {
    let mut regnum: libc::c_ulong = 0;
    /* parse special cases where ISAR is used as index */
    if strcasecmp(b"s\x00" as *const u8 as *const libc::c_char, str) == 0 ||
           strcasecmp(b"(is)\x00" as *const u8 as *const libc::c_char, str) ==
               0 {
        *reg = 0xc as libc::c_int as libc::c_uchar;
        return 0 as libc::c_int
    }
    if strcasecmp(b"i\x00" as *const u8 as *const libc::c_char, str) == 0 ||
           strcasecmp(b"(is)+\x00" as *const u8 as *const libc::c_char, str)
               == 0 {
        *reg = 0xd as libc::c_int as libc::c_uchar;
        return 0 as libc::c_int
    }
    if strcasecmp(b"d\x00" as *const u8 as *const libc::c_char, str) == 0 ||
           strcasecmp(b"(is)-\x00" as *const u8 as *const libc::c_char, str)
               == 0 {
        *reg = 0xe as libc::c_int as libc::c_uchar;
        return 0 as libc::c_int
    }
    /* parse aliases for scratchpad registers */
    if strcasecmp(b"j\x00" as *const u8 as *const libc::c_char, str) == 0 {
        *reg = 0x9 as libc::c_int as libc::c_uchar;
        return 0 as libc::c_int
    }
    if strcasecmp(b"hu\x00" as *const u8 as *const libc::c_char, str) == 0 {
        *reg = 0xa as libc::c_int as libc::c_uchar;
        return 0 as libc::c_int
    }
    if strcasecmp(b"hl\x00" as *const u8 as *const libc::c_char, str) == 0 {
        *reg = 0xb as libc::c_int as libc::c_uchar;
        return 0 as libc::c_int
    }
    /* parse register number */
    if parse_value(str, &mut regnum) != 0 {
        return 1 as libc::c_int
        /* unresolved expr */
    } else {
        if regnum > 14 as libc::c_int as libc::c_ulong {
            asmerr(ERROR_VALUE_MUST_BE_LT_F as libc::c_int,
                   1 as libc::c_int != 0, str);
        }
        *reg = regnum as libc::c_uchar;
        return 0 as libc::c_int
    };
}
/*
 * attempts to parse a special register name from str
 *
 * result : one of the REG_xxx constants (possibly also REG_NONE)
 */
unsafe extern "C" fn parse_special_register(mut str: *mut libc::c_char)
 -> libc::c_int {
    if strcasecmp(b"a\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_A as libc::c_int
    }
    if strcasecmp(b"dc0\x00" as *const u8 as *const libc::c_char, str) == 0 ||
           strcasecmp(b"dc\x00" as *const u8 as *const libc::c_char, str) == 0
       {
        return REG_DC0 as libc::c_int
    }
    if strcasecmp(b"h\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_H as libc::c_int
    }
    if strcasecmp(b"is\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_IS as libc::c_int
    }
    if strcasecmp(b"k\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_K as libc::c_int
    }
    if strcasecmp(b"ku\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_KU as libc::c_int
    }
    if strcasecmp(b"kl\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_KL as libc::c_int
    }
    if strcasecmp(b"pc0\x00" as *const u8 as *const libc::c_char, str) == 0 ||
           strcasecmp(b"p0\x00" as *const u8 as *const libc::c_char, str) == 0
       {
        return REG_PC0 as libc::c_int
    }
    if strcasecmp(b"pc1\x00" as *const u8 as *const libc::c_char, str) == 0 ||
           strcasecmp(b"p\x00" as *const u8 as *const libc::c_char, str) == 0
       {
        return REG_PC1 as libc::c_int
    }
    if strcasecmp(b"q\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_Q as libc::c_int
    }
    if strcasecmp(b"qu\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_QU as libc::c_int
    }
    if strcasecmp(b"ql\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_QL as libc::c_int
    }
    if strcasecmp(b"w\x00" as *const u8 as *const libc::c_char, str) == 0 {
        return REG_W as libc::c_int
    } else { return REG_NONE as libc::c_int };
}
unsafe extern "C" fn v_ins_outs(mut str: *mut libc::c_char,
                                mut mne: *mut _MNE) {
    let mut operand: libc::c_ulong = 0;
    programlabel();
    parse_value(str, &mut operand);
    if operand > 15 as libc::c_int as libc::c_ulong {
        f8err(ERROR_VALUE_MUST_BE_LT_10 as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
    }
    emit_opcode1(((*mne).opcode[0 as libc::c_int as usize] as libc::c_ulong |
                      operand & 15 as libc::c_int as libc::c_ulong) as
                     libc::c_uchar);
}
unsafe extern "C" fn v_sl_sr(mut str: *mut libc::c_char, mut mne: *mut _MNE) {
    let mut operand: libc::c_ulong = 0;
    programlabel();
    if parse_value(str, &mut operand) != 0 {
        /* unresolved expression, reserve space */
        emit_opcode1(0 as libc::c_int as libc::c_uchar);
    } else {
        match operand {
            1 => {
                emit_opcode1((*mne).opcode[0 as libc::c_int as usize] as
                                 libc::c_uchar);
            }
            4 => {
                emit_opcode1((*mne).opcode[0 as libc::c_int as
                                               usize].wrapping_add(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                 as libc::c_uchar);
            }
            _ => {
                f8err(ERROR_VALUE_MUST_BE_1_OR_4 as libc::c_int, (*mne).name,
                      str, 0 as libc::c_int != 0);
                emit_opcode1(0 as libc::c_int as libc::c_uchar);
            }
        }
    };
}
unsafe extern "C" fn v_lis(mut str: *mut libc::c_char, mut mne: *mut _MNE) {
    let mut operand: libc::c_ulong = 0;
    programlabel();
    parse_value(str, &mut operand);
    if operand > 15 as libc::c_int as libc::c_ulong {
        f8err(ERROR_VALUE_MUST_BE_LT_10 as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
    }
    emit_opcode1((0x70 as libc::c_int as libc::c_ulong |
                      operand & 15 as libc::c_int as libc::c_ulong) as
                     libc::c_uchar);
}
unsafe extern "C" fn v_lisu_lisl(mut str: *mut libc::c_char,
                                 mut mne: *mut _MNE) {
    let mut operand: libc::c_ulong = 0;
    programlabel();
    parse_value(str, &mut operand);
    if operand > 7 as libc::c_int as libc::c_ulong {
        f8err(ERROR_VALUE_MUST_BE_LT_8 as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
    }
    emit_opcode1(((*mne).opcode[0 as libc::c_int as usize] as libc::c_ulong |
                      operand & 7 as libc::c_int as libc::c_ulong) as
                     libc::c_uchar);
}
/*
 * handles opcodes with a scratchpad register operand:
 * as, asd, ds, ns, xs
 */
unsafe extern "C" fn v_sreg_op(mut str: *mut libc::c_char,
                               mut mne: *mut _MNE) {
    let mut reg: libc::c_uchar = 0;
    programlabel();
    parse_scratchpad_register(str, &mut reg);
    emit_opcode1(((*mne).opcode[0 as libc::c_int as usize] |
                      reg as libc::c_uint) as libc::c_uchar);
}
unsafe extern "C" fn v_lr(mut str: *mut libc::c_char, mut mne: *mut _MNE) {
    let mut i: libc::c_int = 0;
    let mut ncommas: libc::c_int = 0;
    let mut cindex: libc::c_int = 0;
    let mut op1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut op2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reg_dst: libc::c_uchar = 0;
    let mut reg_src: libc::c_uchar = 0;
    let mut opcode: libc::c_int = 0;
    programlabel();
    /* a valid operand string must contain exactly one comma. find it. */
    ncommas = 0 as libc::c_int;
    cindex = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) != 0 {
        if ',' as i32 == *str.offset(i as isize) as libc::c_int {
            ncommas += 1;
            cindex = i
        }
        i += 1
    }
    if 1 as libc::c_int != ncommas {
        f8err(ERROR_SYNTAX_ERROR as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
        return
    }
    /* extract operand strings  */
    *str.offset(cindex as isize) = 0 as libc::c_int as libc::c_char;
    op1 = str;
    op2 =
        &mut *str.offset((cindex + 1 as libc::c_int) as isize) as
            *mut libc::c_char;
    if 0 as libc::c_int != cindex &&
           *(*__ctype_b_loc()).offset(*str.offset((cindex - 1 as libc::c_int)
                                                      as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        *str.offset((cindex - 1 as libc::c_int) as isize) =
            0 as libc::c_int as libc::c_char
    }
    if *(*__ctype_b_loc()).offset(*op2 as libc::c_int as isize) as libc::c_int
           & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        op2 = op2.offset(1)
    }
    /* parse operand strings for register names */
    reg_dst = parse_special_register(op1) as libc::c_uchar;
    if REG_NONE as libc::c_int == reg_dst as libc::c_int {
        if parse_scratchpad_register(op1, &mut reg_dst) != 0 {
            /* unresolved expression, reserve space */
            emit_opcode1(0 as libc::c_int as libc::c_uchar);
            return
        }
    }
    reg_src = parse_special_register(op2) as libc::c_uchar;
    if REG_NONE as libc::c_int == reg_src as libc::c_int {
        if parse_scratchpad_register(op2, &mut reg_src) != 0 {
            /* unresolved expression, reserve space */
            emit_opcode1(0 as libc::c_int as libc::c_uchar);
            return
        }
    }
    /* restore operand string */
    *str.offset(cindex as isize) = ',' as i32 as libc::c_char;
    if 0 as libc::c_int != cindex &&
           0 as libc::c_int ==
               *str.offset((cindex - 1 as libc::c_int) as isize) as
                   libc::c_int {
        *str.offset((cindex - 1 as libc::c_int) as isize) =
            ' ' as i32 as libc::c_char
    }
    /* generate opcode */
    opcode = -(1 as libc::c_int);
    match reg_dst as libc::c_int {
        16 => {
            /* lr a,xxx */
            match reg_src as libc::c_int {
                19 => { opcode = 0xa as libc::c_int }
                22 => { opcode = 0x1 as libc::c_int }
                21 => { opcode = 0 as libc::c_int }
                27 => { opcode = 0x3 as libc::c_int }
                26 => { opcode = 0x2 as libc::c_int }
                _ => {
                    if (reg_src as libc::c_int) < 15 as libc::c_int {
                        opcode = 0x40 as libc::c_int | reg_src as libc::c_int
                    }
                }
            }
        }
        17 => {
            match reg_src as libc::c_int {
                18 => { opcode = 0x10 as libc::c_int }
                25 => { opcode = 0xf as libc::c_int }
                _ => { }
            }
        }
        18 => {
            if REG_DC0 as libc::c_int == reg_src as libc::c_int {
                opcode = 0x11 as libc::c_int
            }
        }
        19 => {
            if REG_A as libc::c_int == reg_src as libc::c_int {
                opcode = 0xb as libc::c_int
            }
        }
        20 => {
            if REG_PC1 as libc::c_int == reg_src as libc::c_int {
                opcode = 0x8 as libc::c_int
            }
        }
        22 => {
            if REG_A as libc::c_int == reg_src as libc::c_int {
                opcode = 0x5 as libc::c_int
            }
        }
        21 => {
            if REG_A as libc::c_int == reg_src as libc::c_int {
                opcode = 0x4 as libc::c_int
            }
        }
        23 => {
            if REG_Q as libc::c_int == reg_src as libc::c_int {
                opcode = 0xd as libc::c_int
            }
        }
        24 => {
            if REG_K as libc::c_int == reg_src as libc::c_int {
                opcode = 0x9 as libc::c_int
            }
        }
        25 => {
            if REG_DC0 as libc::c_int == reg_src as libc::c_int {
                opcode = 0xe as libc::c_int
            }
        }
        27 => {
            if REG_A as libc::c_int == reg_src as libc::c_int {
                opcode = 0x7 as libc::c_int
            }
        }
        26 => {
            if REG_A as libc::c_int == reg_src as libc::c_int {
                opcode = 0x6 as libc::c_int
            }
        }
        28 => {
            if 0x9 as libc::c_int == reg_src as libc::c_int {
                opcode = 0x1d as libc::c_int
            }
        }
        _ => {
            /* lr sreg,xxx*/
            if 15 as libc::c_int > reg_dst as libc::c_int &&
                   REG_A as libc::c_int == reg_src as libc::c_int {
                /* lr sreg,a */
                opcode = 0x50 as libc::c_int | reg_dst as libc::c_int
            } else if 9 as libc::c_int == reg_dst as libc::c_int &&
                          REG_W as libc::c_int == reg_src as libc::c_int {
                /* special case : lr j,w */
                opcode = 0x1e as libc::c_int
            }
        }
    }
    if opcode < 0 as libc::c_int {
        f8err(ERROR_ILLEGAL_OPERAND_COMBINATION as libc::c_int, (*mne).name,
              str, 1 as libc::c_int != 0);
    } else { emit_opcode1(opcode as libc::c_uchar); };
}
/*
 * generates branch opcodes
 *
 * opcode : opcode of the branch (for instance 0x8f for BR7)
 * str    : operand string
 */
unsafe extern "C" fn generate_branch(mut opcode: libc::c_uchar,
                                     mut str: *mut libc::c_char) {
    let mut target_adr: libc::c_ulong = 0;
    let mut disp: libc::c_long = 0;
    programlabel();
    /* get target address */
    if parse_value(str, &mut target_adr) != 0 {
        /* unresolved target address, reserve space */
        emit_opcode2(0 as libc::c_int as libc::c_uchar,
                     0 as libc::c_int as libc::c_uchar);
        return
    }
    /* calculate displacement */
    if isPCKnown() != 0 {
        disp =
            target_adr.wrapping_sub(getPC() as
                                        libc::c_ulong).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                as libc::c_long;
        if disp > 127 as libc::c_int as libc::c_long ||
               disp < -(128 as libc::c_int) as libc::c_long {
            let mut buf: [libc::c_char; 64] = [0; 64];
            sprintf(buf.as_mut_ptr(),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    disp as libc::c_int);
            asmerr(ERROR_BRANCH_OUT_OF_RANGE as libc::c_int,
                   0 as libc::c_int != 0, buf.as_mut_ptr());
        }
    } else {
        /* unknown pc, will be (hopefully) resolved in future passes */
        disp = 0 as libc::c_int as libc::c_long
    }
    emit_opcode2(opcode,
                 (disp & 255 as libc::c_int as libc::c_long) as
                     libc::c_uchar);
}
/*
 * handles the following branch mnemonics:
 * bc, bm, bnc, bno, bnz, bp, br, br7, bz
 */
unsafe extern "C" fn v_branch(mut str: *mut libc::c_char,
                              mut mne: *mut _MNE) {
    generate_branch((*mne).opcode[0 as libc::c_int as usize] as libc::c_uchar,
                    str);
}
unsafe extern "C" fn v_bf_bt(mut str: *mut libc::c_char, mut mne: *mut _MNE) {
    let mut ncommas: libc::c_int = 0;
    let mut cindex: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut op1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut op2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: libc::c_ulong = 0;
    /* a valid operand string must contain exactly one comma. find it. */
    ncommas = 0 as libc::c_int;
    cindex = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) != 0 {
        if ',' as i32 == *str.offset(i as isize) as libc::c_int {
            ncommas += 1;
            cindex = i
        }
        i += 1
    }
    if 1 as libc::c_int != ncommas {
        f8err(ERROR_SYNTAX_ERROR as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
        return
    }
    /* extract operands */
    *str.offset(cindex as isize) = 0 as libc::c_int as libc::c_char;
    op1 = str;
    op2 =
        &mut *str.offset((cindex + 1 as libc::c_int) as isize) as
            *mut libc::c_char;
    /* parse first operand*/
    if parse_value(op1, &mut value) != 0 {
        /* unresolved expression, reserve space */
        emit_opcode2(0 as libc::c_int as libc::c_uchar,
                     0 as libc::c_int as libc::c_uchar);
        return
    }
    /* check first operand */
    *str.offset(cindex as isize) =
        ',' as i32 as libc::c_char; /* restore operand string */
    if 'f' as i32 ==
           *(*mne).name.offset(1 as libc::c_int as isize) as libc::c_int {
        /* bf */
        if value > 15 as libc::c_int as libc::c_ulong {
            f8err(ERROR_VALUE_MUST_BE_LT_10 as libc::c_int, (*mne).name, str,
                  0 as libc::c_int != 0);
            value &= 15 as libc::c_int as libc::c_ulong
        }
    } else if value > 7 as libc::c_int as libc::c_ulong {
        f8err(ERROR_VALUE_MUST_BE_LT_8 as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
        value &= 7 as libc::c_int as libc::c_ulong
    }
    generate_branch(((*mne).opcode[0 as libc::c_int as usize] as libc::c_ulong
                         | value) as libc::c_uchar, op2);
}
/* bt */
/*
 * handles instructions that take a word operand:
 * dci, jmp, pi
 */
unsafe extern "C" fn v_wordop(mut str: *mut libc::c_char,
                              mut mne: *mut _MNE) {
    let mut value: libc::c_ulong = 0;
    programlabel();
    parse_value(str, &mut value);
    if value > 0xffff as libc::c_int as libc::c_ulong {
        f8err(ERROR_VALUE_MUST_BE_LT_10000 as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
    }
    emit_opcode3((*mne).opcode[0 as libc::c_int as usize] as libc::c_uchar,
                 (value >> 8 as libc::c_int &
                      0xff as libc::c_int as libc::c_ulong) as libc::c_uchar,
                 (value & 0xff as libc::c_int as libc::c_ulong) as
                     libc::c_uchar);
}
/*
 * handles instructions that take a byte operand:
 * ai, ci, in, li, ni, oi, out, xi
 */
unsafe extern "C" fn v_byteop(mut str: *mut libc::c_char,
                              mut mne: *mut _MNE) {
    let mut value: libc::c_ulong = 0;
    programlabel();
    parse_value(str, &mut value);
    if value > 0xff as libc::c_int as libc::c_ulong {
        f8err(ERROR_ADDRESS_MUST_BE_LT_100 as libc::c_int, (*mne).name, str,
              0 as libc::c_int != 0);
    }
    emit_opcode2((*mne).opcode[0 as libc::c_int as usize] as libc::c_uchar,
                 (value & 0xff as libc::c_int as libc::c_ulong) as
                     libc::c_uchar);
}
#[no_mangle]
pub static mut MneF8: [_MNE; 59] =
    unsafe {
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_ds as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"res\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"db\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"dw\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"dd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"adc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ai\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x24 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"am\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x88 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"amd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x89 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sreg_op as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"as\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sreg_op as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xd0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x82 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_bf_bt as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bf\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x90 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bm\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x91 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bnc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x92 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bno\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x98 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bnz\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x94 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x81 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"br\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x90 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"br7\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_bf_bt as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bt\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x80 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_branch as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bz\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x84 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ci\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x25 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"clr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x70 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"cm\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"com\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x18 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_wordop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"dci\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x2a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"di\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x1a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sreg_op as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ds\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x30 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ei\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x1b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"in\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x26 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"inc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x1f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_ins_outs as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ins\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_wordop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"jmp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x29 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"li\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x20 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_lis as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lis\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_lisu_lisl as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lisl\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x68 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_lisu_lisl as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lisu\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x60 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lm\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x16 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lnk\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x19 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_lr as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ni\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x21 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"nm\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"nop\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x2b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sreg_op as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ns\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xf0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"oi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x22 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"om\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"out\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x27 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_ins_outs as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"outs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xb0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_wordop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x28 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pk\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pop\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x1c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sl_sr as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sl\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x13 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sl_sr as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x12 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"st\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x17 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"xdc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x2c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_byteop as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"xi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x23 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"xm\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_sreg_op as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"xs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xe0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect: None,
                      name: 0 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         }]
    };