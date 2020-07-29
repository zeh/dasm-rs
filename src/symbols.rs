use ::libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut SHash: [*mut _SYMBOL; 0];
    #[no_mangle]
    static mut Csegment: *mut _SEGMENT;
    #[no_mangle]
    static mut Av: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut Xdebug: bool;
    #[no_mangle]
    static mut Redo_why: libc::c_ulong;
    #[no_mangle]
    static mut Redo: libc::c_int;
    #[no_mangle]
    static mut Redo_if: libc::c_ulong;
    #[no_mangle]
    static mut Localindex: libc::c_ulong;
    #[no_mangle]
    static mut Localdollarindex: libc::c_ulong;
    #[no_mangle]
    static mut Lastlocaldollarindex: libc::c_ulong;
    #[no_mangle]
    static mut Plab: libc::c_ulong;
    #[no_mangle]
    static mut Pflags: libc::c_ulong;
    #[no_mangle]
    static mut CheckSum: libc::c_ulong;
    #[no_mangle]
    fn asmerr(err: libc::c_int, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sftos(val: libc::c_long, flags: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn permalloc(bytes: libc::c_int) -> *mut libc::c_char;
}
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
static mut org: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
static mut special: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
static mut specchk: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
#[no_mangle]
pub unsafe extern "C" fn setspecial(mut value: libc::c_int,
                                    mut flags: libc::c_int) {
    special.value = value as libc::c_long; /* historical */
    special.flags = flags as libc::c_uchar; /* historical */
}
#[no_mangle]
pub unsafe extern "C" fn findsymbol(mut str: *const libc::c_char,
                                    mut len: libc::c_int) -> *mut _SYMBOL {
    let mut h1: libc::c_uint = 0; /*	permalloc zeros the array for us */
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut buf: [libc::c_char; 1038] = [0; 1038];
    if len > 1024 as libc::c_int { len = 1024 as libc::c_int }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        if len == 1 as libc::c_int {
            if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                org.flags =
                    ((*Csegment).rflags as libc::c_int & 0x1 as libc::c_int)
                        as libc::c_uchar;
                org.value = (*Csegment).rorg as libc::c_long
            } else {
                org.flags =
                    ((*Csegment).flags as libc::c_int & 0x1 as libc::c_int) as
                        libc::c_uchar;
                org.value = (*Csegment).org as libc::c_long
            }
            return &mut org
        }
        if len == 2 as libc::c_int &&
               *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '.' as i32 {
            return &mut special
        }
        if len == 3 as libc::c_int &&
               *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '.' as i32 &&
               *str.offset(2 as libc::c_int as isize) as libc::c_int ==
                   '.' as i32 {
            specchk.flags = 0 as libc::c_int as libc::c_uchar;
            specchk.value = CheckSum as libc::c_long;
            return &mut specchk
        }
        sprintf(buf.as_mut_ptr(),
                b"%ld%.*s\x00" as *const u8 as *const libc::c_char,
                Localindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    } else if *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int ==
                  '$' as i32 {
        sprintf(buf.as_mut_ptr(),
                b"%ld$%.*s\x00" as *const u8 as *const libc::c_char,
                Localdollarindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    }
    h1 = hash1(str, len);
    sym = *SHash.as_mut_ptr().offset(h1 as isize);
    while !sym.is_null() {
        if (*sym).namelen == len as libc::c_uint &&
               memcmp((*sym).name as *const libc::c_void,
                      str as *const libc::c_void, len as libc::c_ulong) == 0 {
            break ;
        }
        sym = (*sym).next
    }
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn CreateSymbol(mut str: *const libc::c_char,
                                      mut len: libc::c_int) -> *mut _SYMBOL {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut h1: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1038] = [0; 1038];
    if len > 1024 as libc::c_int { len = 1024 as libc::c_int }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        sprintf(buf.as_mut_ptr(),
                b"%ld%.*s\x00" as *const u8 as *const libc::c_char,
                Localindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    } else if *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int ==
                  '$' as i32 {
        sprintf(buf.as_mut_ptr(),
                b"%ld$%.*s\x00" as *const u8 as *const libc::c_char,
                Localdollarindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    }
    sym = allocsymbol();
    (*sym).name = permalloc(len + 1 as libc::c_int);
    memcpy((*sym).name as *mut libc::c_void, str as *const libc::c_void,
           len as libc::c_ulong);
    (*sym).namelen = len as libc::c_uint;
    h1 = hash1(str, len);
    (*sym).next = *SHash.as_mut_ptr().offset(h1 as isize);
    (*sym).flags = 0x1 as libc::c_int as libc::c_uchar;
    let ref mut fresh0 = *SHash.as_mut_ptr().offset(h1 as isize);
    *fresh0 = sym;
    return sym;
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
 *  SYMBOLS.C
 */
unsafe extern "C" fn hash1(mut str: *const libc::c_char, mut len: libc::c_int)
 -> libc::c_uint {
    let mut result: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 != 0) { break ; }
        let fresh2 = str;
        str = str.offset(1);
        result = result << 2 as libc::c_int ^ *fresh2 as libc::c_uint
    }
    return result & 0x3ff as libc::c_int as libc::c_uint;
}
/*
*  Label Support Routines
*/
#[no_mangle]
pub unsafe extern "C" fn programlabel() {
    let mut len: libc::c_int = 0;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut cseg: *mut _SEGMENT = Csegment;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rorg: libc::c_uchar =
        ((*cseg).flags as libc::c_int & 0x20 as libc::c_int) as libc::c_uchar;
    let mut cflags: libc::c_uchar =
        if rorg as libc::c_int != 0 {
            (*cseg).rflags as libc::c_int
        } else { (*cseg).flags as libc::c_int } as libc::c_uchar;
    let mut pc: libc::c_ulong =
        if rorg as libc::c_int != 0 { (*cseg).rorg } else { (*cseg).org };
    Plab = (*cseg).org;
    Pflags = (*cseg).flags as libc::c_ulong;
    str = *Av.as_mut_ptr().offset(0 as libc::c_int as isize);
    if *str as libc::c_int == 0 as libc::c_int { return }
    len = strlen(str) as libc::c_int;
    if *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int ==
           ':' as i32 {
        len -= 1
    }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 &&
           *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int !=
               '$' as i32 {
        Lastlocaldollarindex = Lastlocaldollarindex.wrapping_add(1);
        Localdollarindex = Lastlocaldollarindex
    }
    /*
    *	Redo:	unknown and referenced
    *		referenced and origin not known
    *		known and phase error	 (origin known)
    */
    sym = findsymbol(str, len);
    if !sym.is_null() {
        if (*sym).flags as libc::c_int &
               (0x1 as libc::c_int | 0x4 as libc::c_int) ==
               0x1 as libc::c_int | 0x4 as libc::c_int {
            Redo += 1;
            Redo_why |=
                REASON_FORWARD_REFERENCE as libc::c_int as libc::c_ulong;
            if Xdebug {
                printf(b"redo 13: \'%s\' %04x %04x\n\x00" as *const u8 as
                           *const libc::c_char, (*sym).name,
                       (*sym).flags as libc::c_int, cflags as libc::c_int);
            }
        } else if cflags as libc::c_int & 0x1 as libc::c_int != 0 &&
                      (*sym).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |=
                REASON_FORWARD_REFERENCE as libc::c_int as libc::c_ulong
        } else if cflags as libc::c_int & 0x1 as libc::c_int == 0 &&
                      (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 {
            if pc != (*sym).value as libc::c_ulong {
                /*
            * If we had an unevaluated IF expression in the
            * previous pass, don't complain about phase errors
            * too loudly.
                */
                //FIX: calling asmerr with ERROR_LABEL_MISMATCH is fatal. The clause
                //     below was causing aborts if verbosity was up, even when the
                //     phase errors were the result of unevaluated IF expressions in
                //     the previous pass.
                //if (F_verbose >= 1 || !(Redo_if & (REASON_OBSCURE)))
                if Redo_if & REASON_OBSCURE as libc::c_int as libc::c_ulong ==
                       0 {
                    let mut sBuffer: [libc::c_char; 4096] = [0; 4096];
                    sprintf(sBuffer.as_mut_ptr(),
                            b"%s %s\x00" as *const u8 as *const libc::c_char,
                            (*sym).name,
                            sftos((*sym).value, 0 as libc::c_int));
                    asmerr(ERROR_LABEL_MISMATCH as libc::c_int,
                           0 as libc::c_int != 0, sBuffer.as_mut_ptr());
                }
                Redo += 1;
                Redo_why |= REASON_PHASE_ERROR as libc::c_int as libc::c_ulong
            }
        }
    } else { sym = CreateSymbol(str, len) }
    (*sym).value = pc as libc::c_long;
    (*sym).flags =
        ((*sym).flags as libc::c_int & !(0x1 as libc::c_int) |
             cflags as libc::c_int & 0x1 as libc::c_int) as libc::c_uchar;
}
#[no_mangle]
pub static mut SymAlloc: *mut _SYMBOL = 0 as *const _SYMBOL as *mut _SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn allocsymbol() -> *mut _SYMBOL {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if !SymAlloc.is_null() {
        sym = SymAlloc;
        SymAlloc = (*SymAlloc).next;
        memset(sym as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_SYMBOL>() as libc::c_ulong);
    } else {
        sym =
            permalloc(::std::mem::size_of::<_SYMBOL>() as libc::c_ulong as
                          libc::c_int) as *mut _SYMBOL
    }
    return sym;
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
 *  ASM65.H
 *
 *  Structures and definitions
 */
/* for -T option [phf] */
/* for -E option [phf] */
/* Check format of command-line */
/* Unable to open file */
/* Source is not resolvable */
/* Too many passes - something wrong */
/* one or more non-abort errors occured */
/*  0 */
/*  1 */
/*  2 */
/*  3 */
/*  4 */
/*  5 */
/*  6 */
/*  7 */
/*  8 */
/*  9 */
/* 10 */
/* 11 */
/* 12 */
/* 13 */
/* 14 */
/* 15 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* Only allow one type of processor */
/* Bad format specifier */
/* F8 support... */
/* 26 */
/* 27 */
/* 28 */
/* 29 */
/* 30 */
/* 31 */
/* ASM_ERROR_EQUATES value */
/* 0 = OK, non-zero = cannot continue compilation */
/* Error message */
/* fix this! */
/*    implied         */
/*    immediate 8  bits   */
/*    immediate 16 bits   */
/*    address 8 bits        */
/*    address 16 bits     */
/*    relative 8 bits     */
/*    index x 0 bits        */
/*    index x 8 bits        */
/*    index x 16 bits     */
/*    bit inst. special   */
/*    bit-bra inst. spec. */
/*    index y 0 bits        */
/*    index y 8 bits        */
/*    index x 0 bits        */
/*    index y 0 bits        */
/*    ind addr 8 bits     */
/*    ind addr 16 bits    */
/*  has mask argument (byte)    */
/*  has rel. address (byte)    */
/*  instruction byte mod.    */
/*  is v_endm            */
/*    hash        */
/*  dispatch        */
/*    actual name    */
/*    special flags    */
/*    hex codes, byte or word (>xFF) opcodes    */
/* MNEMONIC with all fields 0, used as end-of-table marker. */
/*      previously pushed context */
/*      file name            */
/*      file handle            */
/*      line number in file        */
/*      flags (macro)         */
/*    Only if Macro    */
/*  arguments to macro        */
/*  current string list     */
/*  save localindex        */
/*  save localdollarindex   */
/*      value unknown     */
/*      previously pushed context */
/*      repeat count            */
/*      seek to top of repeat     */
/*      line number of line before  */
/*      which include file are we in*/
/*      value unknown        */
/*      previous IF            */
/*      which include file are we in*/
/*      1 if true, 0 if false     */
/*      accumulatively true (not incl this one) */
/*      ORG unknown            */
/*      ORG referenced        */
/*      uninitialized area (U flag)    */
/*      relocatable origin active    */
/*      next segment in segment list    */
/*      name of segment        */
/*      for ORG            */
/*      for RORG            */
/*      current org            */
/*      current rorg            */
/*      value unknown     */
/*      referenced        */
/*      result is a string    */
/*      SET instruction used    */
/*      symbol is a macro    */
/*      master reference    */
/*  next symbol in hash list        */
/*  symbol name or string if expr.  */
/*  if symbol is actually a string  */
/*  flags                */
/*  addressing mode (expressions)   */
/* current value, never EVER change this to unsigned! */
/*  name length             */
/*      current segment */
/*extern unsigned int Adrbytes[];*/
/*    mnemonic extension    */
/* -T option [phf] */
/* -E option [phf] */
/*extern unsigned int _fmode;*/
/* main.c */
/*extern unsigned char Listing;*/
/* symbols.c */
/* defined but not used [phf] */
/*
static void freesymbol(SYMBOL *sym)
{
    sym->next = SymAlloc;
    SymAlloc = sym;
}
*/
#[no_mangle]
pub unsafe extern "C" fn FreeSymbolList(mut sym: *mut _SYMBOL) {
    let mut next: *mut _SYMBOL = 0 as *mut _SYMBOL;
    while !sym.is_null() {
        next = (*sym).next;
        (*sym).next = SymAlloc;
        if (*sym).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            free((*sym).string as *mut libc::c_void);
        }
        SymAlloc = sym;
        sym = next
    };
}
