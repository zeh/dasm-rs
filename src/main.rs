#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate smart_default;

use libc;
use std::convert::TryFrom;

pub mod exp;
pub mod globals;
pub mod mne6303;
pub mod mne6502;
pub mod mne6811;
pub mod mne68705;
pub mod mnef8;
pub mod ops;
pub mod symbols;

pub mod constants;
pub mod types;

use constants::{
    ALLOC_SIZE,
    CHAR_TAB,
    M_HASH_AND,
    MAX_LINES,
    MAX_SYMBOLS,
    S_HASH_SIZE,
};

use types::enums::{
    AddressModes,
    AsmErrorEquates,
    ErrorFormat,
    Format,
    SortMode
};

use types::structs::{
    ErrorDefinition,
};

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn v_incdir(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    static mut SHash: [*mut _SYMBOL; 0];
    #[no_mangle]
    static mut MHash: [*mut _MNE; 0];
    #[no_mangle]
    static mut pIncfile: *mut _INCFILE;
    #[no_mangle]
    static mut Reploop: *mut _REPLOOP;
    #[no_mangle]
    static mut Seglist: *mut _SEGMENT;
    #[no_mangle]
    static mut Ifstack: *mut _IFSTACK;
    #[no_mangle]
    static mut Csegment: *mut _SEGMENT;
    #[no_mangle]
    static mut Av: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut Avbuf: [libc::c_char; 0];
    #[no_mangle]
    static mut Ops: [_MNE; 0];
    #[no_mangle]
    static mut Mnext: libc::c_int;
    #[no_mangle]
    static mut Xdebug: bool;
    #[no_mangle]
    static mut bStrictMode: bool;
    #[no_mangle]
    static mut Redo_why: libc::c_ulong;
    #[no_mangle]
    static mut Redo: libc::c_int;
    #[no_mangle]
    static mut Redo_eval: libc::c_int;
    #[no_mangle]
    static mut Redo_if: libc::c_ulong;
    #[no_mangle]
    static mut Localindex: libc::c_ulong;
    #[no_mangle]
    static mut Lastlocalindex: libc::c_ulong;
    #[no_mangle]
    static mut Localdollarindex: libc::c_ulong;
    #[no_mangle]
    static mut Lastlocaldollarindex: libc::c_ulong;
    #[no_mangle]
    static mut F_format: Format;
    #[no_mangle]
    static mut F_sortmode: SortMode;
    #[no_mangle]
    static mut F_errorformat: ErrorFormat;
    #[no_mangle]
    static mut F_verbose: libc::c_uchar;
    #[no_mangle]
    static mut F_outfile: *const libc::c_char;
    #[no_mangle]
    static mut F_listfile: *mut libc::c_char;
    #[no_mangle]
    static mut F_symfile: *mut libc::c_char;
    #[no_mangle]
    static mut FI_listfile: *mut FILE;
    #[no_mangle]
    static mut FI_temp: *mut FILE;
    #[no_mangle]
    static mut Fisclear: libc::c_uchar;
    #[no_mangle]
    static mut Plab: libc::c_ulong;
    #[no_mangle]
    static mut Pflags: libc::c_ulong;
    #[no_mangle]
    static mut Inclevel: libc::c_char;
    #[no_mangle]
    static mut ListMode: libc::c_char;
    #[no_mangle]
    static mut CheckSum: libc::c_ulong;
    #[no_mangle]
    fn v_execmac(str: *mut libc::c_char, mac: *mut _MACRO);
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const libc::c_char, wantmode: libc::c_int) -> *mut _SYMBOL;
    #[no_mangle]
    fn pfopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn v_eqm(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn programlabel();
    #[no_mangle]
    static mut Gen: [libc::c_uchar; 0];
    #[no_mangle]
    static mut Glen: libc::c_int;
    #[no_mangle]
    fn v_set(str: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_mexit(str: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn closegenerate();
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
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
pub struct _STRLIST {
    pub next: *mut _STRLIST,
    pub buf: [libc::c_char; 4],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MACRO {
    pub next: *mut _MACRO,
    pub vect: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                          _: *mut _MACRO) -> ()>,
    pub name: *mut libc::c_char,
    pub flags: libc::c_uchar,
    pub strlist: *mut _STRLIST,
    pub defpass: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _INCFILE {
    pub next: *mut _INCFILE,
    pub name: *mut libc::c_char,
    pub fi: *mut FILE,
    pub lineno: libc::c_ulong,
    pub flags: libc::c_uchar,
    pub args: *mut _STRLIST,
    pub strlist: *mut _STRLIST,
    pub saveidx: libc::c_ulong,
    pub savedolidx: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _REPLOOP {
    pub next: *mut _REPLOOP,
    pub count: libc::c_ulong,
    pub seek: libc::c_ulong,
    pub lineno: libc::c_ulong,
    pub file: *mut _INCFILE,
    pub flags: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFSTACK {
    pub next: *mut _IFSTACK,
    pub file: *mut _INCFILE,
    pub flags: libc::c_uchar,
    pub xtrue: libc::c_uchar,
    pub acctrue: libc::c_uchar,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union align {
    pub l: libc::c_long,
    pub p: *mut libc::c_void,
    pub fp: Option<unsafe extern "C" fn() -> ()>,
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
 *  MAIN.C
 *  DASM   sourcefile
 *  NOTE: must handle mnemonic extensions and expression decode/compare.
 */
static mut dasm_id: [libc::c_char; 22] =
    [68, 65, 83, 77, 32, 50, 46, 50, 48, 46, 49, 52, 45, 83, 78, 65, 80, 83,
     72, 79, 84, 0];
// buffers to supress errors and messages until last pass
#[no_mangle]
pub static mut passbuffer: [*mut libc::c_char; 2] =
    [0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char];
static mut erroradd1: [libc::c_char; 500] = [0; 500];
// temp error holders
static mut erroradd2: [libc::c_char; 500] = [0; 500];
static mut erroradd3: [libc::c_char; 500] = [0; 500];
/* Table encapsulates errors, descriptions, and fatality flags. */
// FIXME: make sure %s interpolation still works (use {} ?)
#[no_mangle]
pub static mut sErrorDef: [ErrorDefinition; 39] = [
	ErrorDefinition {
		errorType: AsmErrorEquates::None,
		fatal: true,
		description: "OK",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::CommandLine,
		fatal: true,
		description: "Check command-line format.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::FileError,
		fatal: true,
		description: "Unable to open file.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NotResolvable,
		fatal: true,
		description: "Source is not resolvable.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::TooManyPasses,
		fatal: true,
		description: "Too many passes (%s).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NonAbort,
		fatal: true,
		description: "See previous output",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::SyntaxError,
		fatal: true,
		description: "Syntax Error \'%s\'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ExpressionTableOverflow,
		fatal: true,
		description: "Expression table overflow.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::UnbalancedBraces,
		fatal: true,
		description: "Unbalanced Braces [].",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::DivisionByZero,
		fatal: true,
		description: "Division by zero.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::UnknownMnemonic,
		fatal: true,
		description: "Unknown Mnemonic \'%s\'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalAddressingMode,
		fatal: false,
		description: "Illegal Addressing mode \'%s\'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalForcedAddressingMode,
		fatal: true,
		description: "Illegal forced Addressing mode on \'%s\'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NotEnoughArgumentsPassedToMacro,
		fatal: true,
		description: "Not enough args passed to Macro.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::PrematureEOF,
		fatal: false,
		description: "Premature EOF.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalCharacter,
		fatal: true,
		description: "Illegal character \'%s\'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::BranchOutOfRange,
		fatal: false,
		description: "Branch out of range (%s bytes).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ErrPseudoOpEncountered,
		fatal: true,
		description: "ERR pseudo-op encountered.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::OriginReverseIndexed,
		fatal: false,
		description: "Origin Reverse-indexed.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::EquValueMismatch,
		fatal: false,
		description: "EQU: Value mismatch.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::AddressMustBeLowerThan100,
		fatal: true,
		description: "Value in \'%s\' must be <$100.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::AddressMustBeLowerThan10000,
		fatal: true,
		description: "Value in \'%s\' must be <$10000.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalBitSpecification,
		fatal: true,
		description: "Illegal bit specification.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NotEnoughArgs,
		fatal: true,
		description: "Not enough arguments.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::LabelMismatch,
		fatal: false,
		description: "Label mismatch...\n --> %s",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::MacroRepeated,
		fatal: true,
		description: "Macro \"%s\" definition is repeated.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueUndefined,
		fatal: true,
		description: "Value Undefined.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ProcessorNotSupported,
		fatal: true,
		description: "Processor \'%s\' not supported.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::RepeatNegative,
		fatal: false,
		description: "REPEAT parameter < 0 (ignored).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::BadError,
		fatal: true,
		description: "Bad error value (internal error).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::OnlyOneProcessorSupported,
		fatal: true,
		description: "Only one processor type may be selected.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::BadFormat,
		fatal: true,
		description: "Bad output format specified.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeOneOrFour,
		fatal: true,
		description: "Value in \'%s\' must be 1 or 4.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThan10,
		fatal: true,
		description: "Value in \'%s\' must be <$10.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThan8,
		fatal: true,
		description: "Value in \'%s\' must be <$8.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThanF,
		fatal: true,
		description: "Value in \'%s\' must be <$f.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThan10000,
		fatal: true,
		description: "Value in \'%s\' must be <$10000.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalOperandCombination,
		fatal: true,
		description: "Illegal combination of operands \'%s\'",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::EndOfTable, // FIXME: remove? This was added but might not be needed
		fatal: true,
		description: "Doh! Internal end-of-table marker, report the bug!",
	},
];
#[no_mangle]
pub static mut bStopAtEnd: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut nMaxPasses: libc::c_int = 10 as libc::c_int;
#[no_mangle]
pub static mut Extstr: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*unsigned char     Listing = 1;*/
#[no_mangle]
pub static mut pass: libc::c_int = 0;
#[no_mangle]
pub static mut F_ListAllPasses: libc::c_uchar =
    0 as libc::c_int as libc::c_uchar;
unsafe extern "C" fn CountUnresolvedSymbols() -> libc::c_int {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut nUnresolved: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    /* Pre-count unresolved symbols */
    i = 0 as libc::c_int;
    while i < S_HASH_SIZE as libc::c_int {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() {
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                nUnresolved += 1
            }
            sym = (*sym).next
        }
        i += 1
    }
    return nUnresolved;
}
unsafe extern "C" fn ShowUnresolvedSymbols() -> libc::c_int {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: libc::c_int = 0;
    let mut nUnresolved: libc::c_int = CountUnresolvedSymbols();
    if nUnresolved != 0 {
        printf(b"--- Unresolved Symbol List\n\x00" as *const u8 as
                   *const libc::c_char);
        /* Display unresolved symbols */
        i = 0 as libc::c_int;
        while i < S_HASH_SIZE as libc::c_int {
            sym = *SHash.as_mut_ptr().offset(i as isize);
            while !sym.is_null() {
                if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                    printf(b"%-24s %s\n\x00" as *const u8 as
                               *const libc::c_char, (*sym).name,
                           sftos((*sym).value, (*sym).flags as libc::c_int));
                }
                sym = (*sym).next
            }
            i += 1
        }
        printf(b"--- %d Unresolved Symbol%c\n\n\x00" as *const u8 as
                   *const libc::c_char, nUnresolved,
               if nUnresolved == 1 as libc::c_int {
                   ' ' as i32
               } else { 's' as i32 });
    }
    return nUnresolved;
}
unsafe extern "C" fn CompareAlpha(mut arg1: *const libc::c_void,
                                  mut arg2: *const libc::c_void)
 -> libc::c_int {
    /* Simple alphabetic ordering comparison function for quicksort */
    let mut sym1: *const _SYMBOL = *(arg1 as *const *mut _SYMBOL);
    let mut sym2: *const _SYMBOL = *(arg2 as *const *mut _SYMBOL);
    /*
       The cast above is wild, thank goodness the Linux man page
       for qsort(3) has an example explaining it... :-) [phf]

       TODO: Note that we compare labels case-insensitive here which
       is not quite right; I believe we should be case-sensitive as
       in other contexts where symbols (labels) are compared. But
       the old CompareAlpha() was case-insensitive as well, so I
       didn't want to change that right now... [phf]
    */
    return strcasecmp((*sym1).name, (*sym2).name);
}
unsafe extern "C" fn CompareAddress(mut arg1: *const libc::c_void,
                                    mut arg2: *const libc::c_void)
 -> libc::c_int {
    /* Simple numeric ordering comparison function for quicksort */
    let mut sym1: *const _SYMBOL = *(arg1 as *const *mut _SYMBOL);
    let mut sym2: *const _SYMBOL = *(arg2 as *const *mut _SYMBOL);
    return ((*sym1).value - (*sym2).value) as libc::c_int;
}
/* bTableSort true -> by address, false -> by name [phf] */
unsafe extern "C" fn ShowSymbols(mut file: *mut FILE, mut bTableSort: bool) {
    /* Display sorted (!) symbol table - if it runs out of memory, table will be displayed unsorted */
    let mut symArray: *mut *mut _SYMBOL = 0 as *mut *mut _SYMBOL;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: libc::c_int = 0;
    let mut nSymbols: libc::c_int = 0 as libc::c_int;
    fprintf(file, b"--- Symbol List\x00" as *const u8 as *const libc::c_char);
    /* Sort the symbol list either via name, or by value */
    /* First count the number of symbols */
    i = 0 as libc::c_int;
    while i < S_HASH_SIZE as libc::c_int {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() { nSymbols += 1; sym = (*sym).next }
        i += 1
    }
    /* Malloc an array of pointers to data */
    symArray =
        ckmalloc((::std::mem::size_of::<*mut _SYMBOL>() as
                      libc::c_ulong).wrapping_mul(nSymbols as libc::c_ulong)
                     as libc::c_int) as *mut *mut _SYMBOL;
    if symArray.is_null() {
        fprintf(file,
                b" (unsorted - not enough memory to sort!)\n\x00" as *const u8
                    as *const libc::c_char);
        /* Display complete symbol table */
        i = 0 as libc::c_int;
        while i < S_HASH_SIZE as libc::c_int {
            sym = *SHash.as_mut_ptr().offset(i as isize);
            while !sym.is_null() {
                fprintf(file,
                        b"%-24s %s\n\x00" as *const u8 as *const libc::c_char,
                        (*sym).name,
                        sftos((*sym).value, (*sym).flags as libc::c_int));
                sym = (*sym).next
            }
            i += 1
        }
    } else {
        /* Copy the element pointers into the symbol array */
        let mut nPtr: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < S_HASH_SIZE as libc::c_int {
            sym = *SHash.as_mut_ptr().offset(i as isize);
            while !sym.is_null() {
                let fresh0 = nPtr;
                nPtr = nPtr + 1;
                let ref mut fresh1 = *symArray.offset(fresh0 as isize);
                *fresh1 = sym;
                sym = (*sym).next
            }
            i += 1
        }
        if bTableSort {
            fprintf(file,
                    b" (sorted by address)\n\x00" as *const u8 as
                        *const libc::c_char);
            qsort(symArray as *mut libc::c_void, nPtr as size_t,
                  ::std::mem::size_of::<*mut _SYMBOL>() as libc::c_ulong,
                  Some(CompareAddress as
                           unsafe extern "C" fn(_: *const libc::c_void,
                                                _: *const libc::c_void)
                               -> libc::c_int));
            /* Sort via address */
        } else {
            fprintf(file,
                    b" (sorted by symbol)\n\x00" as *const u8 as
                        *const libc::c_char);
            qsort(symArray as *mut libc::c_void, nPtr as size_t,
                  ::std::mem::size_of::<*mut _SYMBOL>() as libc::c_ulong,
                  Some(CompareAlpha as
                           unsafe extern "C" fn(_: *const libc::c_void,
                                                _: *const libc::c_void)
                               -> libc::c_int));
            /* Sort via name */
        }
        /* now display sorted list */
        i = 0 as libc::c_int; /* If a string, display actual string */
        while i < nPtr {
            fprintf(file,
                    b"%-24s %-12s\x00" as *const u8 as *const libc::c_char,
                    (**symArray.offset(i as isize)).name,
                    sftos((**symArray.offset(i as isize)).value,
                          (**symArray.offset(i as isize)).flags as
                              libc::c_int));
            if (**symArray.offset(i as isize)).flags as libc::c_int &
                   0x8 as libc::c_int != 0 {
                fprintf(file,
                        b" \"%s\"\x00" as *const u8 as *const libc::c_char,
                        (**symArray.offset(i as isize)).string);
            }
            fprintf(file, b"\n\x00" as *const u8 as *const libc::c_char);
            i += 1
        }
        free(symArray as *mut libc::c_void);
    }
    fputs(b"--- End of Symbol List.\n\x00" as *const u8 as
              *const libc::c_char, file);
}
unsafe extern "C" fn ShowSegments() {
    let mut seg: *mut _SEGMENT = 0 as *mut _SEGMENT;
    let mut bss: *const libc::c_char = 0 as *const libc::c_char;
    let mut sFormat: *const libc::c_char =
        b"%-24s %-3s %-8s %-8s %-8s %-8s\n\x00\x00" as *const u8 as
            *const libc::c_char;
    printf(b"\n----------------------------------------------------------------------\n\x00"
               as *const u8 as *const libc::c_char);
    printf(sFormat, b"SEGMENT NAME\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char,
           b"INIT PC\x00" as *const u8 as *const libc::c_char,
           b"INIT RPC\x00" as *const u8 as *const libc::c_char,
           b"FINAL PC\x00" as *const u8 as *const libc::c_char,
           b"FINAL RPC\x00" as *const u8 as *const libc::c_char);
    seg = Seglist;
    while !seg.is_null() {
        bss =
            if (*seg).flags as libc::c_int & 0x10 as libc::c_int != 0 {
                b"[u]\x00" as *const u8 as *const libc::c_char
            } else { b"   \x00" as *const u8 as *const libc::c_char };
        printf(sFormat, (*seg).name, bss,
               sftos((*seg).initorg as libc::c_long,
                     (*seg).initflags as libc::c_int),
               sftos((*seg).initrorg as libc::c_long,
                     (*seg).initrflags as libc::c_int),
               sftos((*seg).org as libc::c_long, (*seg).flags as libc::c_int),
               sftos((*seg).rorg as libc::c_long,
                     (*seg).rflags as libc::c_int));
        seg = (*seg).next
    }
    puts(b"----------------------------------------------------------------------\x00"
             as *const u8 as *const libc::c_char);
    printf(b"%d references to unknown symbols.\n\x00" as *const u8 as
               *const libc::c_char, Redo_eval);
    printf(b"%d events requiring another assembler pass.\n\x00" as *const u8
               as *const libc::c_char, Redo);
    if Redo_why != 0 {
        if Redo_why &
               REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as libc::c_ulong !=
               0 {
            printf(b" - Expression in mnemonic not resolved.\n\x00" as
                       *const u8 as *const libc::c_char);
        }
        if Redo_why & REASON_OBSCURE as libc::c_int as libc::c_ulong != 0 {
            printf(b" - Obscure reason - to be documented :)\n\x00" as
                       *const u8 as *const libc::c_char);
        }
        if Redo_why & REASON_DC_NOT_RESOVED as libc::c_int as libc::c_ulong !=
               0 {
            printf(b" - Expression in a DC not resolved.\n\x00" as *const u8
                       as *const libc::c_char);
        }
        if Redo_why &
               REASON_DV_NOT_RESOLVED_PROBABLY as libc::c_int as libc::c_ulong
               != 0 {
            printf(b" - Expression in a DV not resolved (probably in DV\'s EQM symbol).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why &
               REASON_DV_NOT_RESOLVED_COULD as libc::c_int as libc::c_ulong !=
               0 {
            printf(b" - Expression in a DV not resolved (could be in DV\'s EQM symbol).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why & REASON_DS_NOT_RESOLVED as libc::c_int as libc::c_ulong
               != 0 {
            printf(b" - Expression in a DS not resolved.\n\x00" as *const u8
                       as *const libc::c_char);
        }
        if Redo_why &
               REASON_ALIGN_NOT_RESOLVED as libc::c_int as libc::c_ulong != 0
           {
            printf(b" - Expression in an ALIGN not resolved.\n\x00" as
                       *const u8 as *const libc::c_char);
        }
        if Redo_why &
               REASON_ALIGN_RELOCATABLE_ORIGIN_NOT_KNOWN as libc::c_int as
                   libc::c_ulong != 0 {
            printf(b" - ALIGN: Relocatable origin not known (if in RORG at the time).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why &
               REASON_ALIGN_NORMAL_ORIGIN_NOT_KNOWN as libc::c_int as
                   libc::c_ulong != 0 {
            printf(b" - ALIGN: Normal origin not known\t(if in ORG at the time).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why & REASON_EQU_NOT_RESOLVED as libc::c_int as libc::c_ulong
               != 0 {
            printf(b" - EQU: Expression not resolved.\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        if Redo_why &
               REASON_EQU_VALUE_MISMATCH as libc::c_int as libc::c_ulong != 0
           {
            printf(b" - EQU: Value mismatch from previous pass (phase error).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why & REASON_IF_NOT_RESOLVED as libc::c_int as libc::c_ulong
               != 0 {
            printf(b" - IF: Expression not resolved.\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        if Redo_why &
               REASON_REPEAT_NOT_RESOLVED as libc::c_int as libc::c_ulong != 0
           {
            printf(b" - REPEAT: Expression not resolved.\n\x00" as *const u8
                       as *const libc::c_char);
        }
        if Redo_why & REASON_FORWARD_REFERENCE as libc::c_int as libc::c_ulong
               != 0 {
            printf(b" - Label defined after it has been referenced (forward reference).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why & REASON_PHASE_ERROR as libc::c_int as libc::c_ulong != 0
           {
            printf(b" - Label value is different from that of the previous pass (phase error).\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        if Redo_why &
               REASON_BRANCH_OUT_OF_RANGE as libc::c_int as libc::c_ulong != 0
           {
            printf(b" - Branch was out of range.\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn DumpSymbolTable(mut bTableSort: bool) {
    if !F_symfile.is_null() {
        let mut fi: *mut FILE =
            fopen(F_symfile, b"w\x00" as *const u8 as *const libc::c_char);
        if !fi.is_null() {
            ShowSymbols(fi, bTableSort);
            fclose(fi);
        } else {
            printf(b"Warning: Unable to open Symbol Dump file \'%s\'\n\x00" as
                       *const u8 as *const libc::c_char, F_symfile);
        }
    };
}
unsafe extern "C" fn MainShadow(mut ac: libc::c_int,
                                mut av: *mut *mut libc::c_char,
                                mut pbTableSort: *mut bool) -> AsmErrorEquates {
    let mut current_block: u64;
    let mut nError: AsmErrorEquates = AsmErrorEquates::None;
    let mut bDoAllPasses: bool = 0 as libc::c_int != 0;
    let mut buf: [libc::c_char; MAX_LINES] = [0; MAX_LINES];
    let mut i: libc::c_int = 0;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut oldredo: libc::c_int = -(1 as libc::c_int);
    let mut oldwhy: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut oldeval: libc::c_int = 0 as libc::c_int;
    addhashtable(Ops.as_mut_ptr());
    pass = 1 as libc::c_int;
    if !(ac < 2 as libc::c_int) {
        i = 2 as libc::c_int;
        loop  {
            if !(i < ac) { current_block = 16231175055492490595; break ; }
            if !(*(*av.offset(i as isize)).offset(0 as libc::c_int as isize)
                     as libc::c_int == '-' as i32 ||
                     *(*av.offset(i as
                                      isize)).offset(0 as libc::c_int as
                                                         isize) as libc::c_int
                         == '/' as i32) {
                current_block = 15878785573848117940;
                break ;
            }
            let mut str: *mut libc::c_char =
                (*av.offset(i as isize)).offset(2 as libc::c_int as isize);
            // FIXME: use better strings for parsing chars. These are temporary.
            let str_rs_1 = &[*str as u8];
            let str_rs = std::str::from_utf8(str_rs_1).unwrap();
            match *(*av.offset(i as isize)).offset(1 as libc::c_int as isize)
                      as libc::c_int {
                69 => { // 'E' - FIXME: convert back to original char
                    /* TODO: need to improve option parsing and errors for it */
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match ErrorFormat::try_from(digit) {
                                Ok(result) => { F_errorformat = result; }
                                Err(_) => { panic_rs("Invalid error format for -E, must be 0, 1, 2"); }
                            }
                        }
                        Err(_) => { panic_rs("Invalid error format for -E, must be 0, 1, 2"); }
                    }
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                84 => { // 'T' - FIXME: convert back to original char
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match SortMode::try_from(digit) {
                                Ok(result) => { F_sortmode = result; }
                                Err(_) => { panic_rs("Invalid sorting mode for -T option, must be 0 or 1"); }
                            }
                        }
                        Err(_) => { panic_rs("Invalid sorting mode for -T option, must be 0 or 1"); }
                    }
                    *pbTableSort = F_sortmode != SortMode::default();
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                100 => {
                    Xdebug =
                        strtol(str,
                               0 as *mut libc::c_void as
                                   *mut *mut libc::c_char, 10 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int;
                    printf(b"Debug trace %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           if Xdebug as libc::c_int != 0 {
                               b"ON\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"OFF\x00" as *const u8 as *const libc::c_char
                           });
                    current_block = 17788412896529399552;
                }
                77 | 68 => {
                    while *str as libc::c_int != 0 &&
                              *str as libc::c_int != '=' as i32 {
                        str = str.offset(1)
                    }
                    if *str as libc::c_int == '=' as i32 {
                        *str = 0 as libc::c_int as libc::c_char;
                        str = str.offset(1)
                    } else {
                        str =
                            b"0\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char
                    }
                    let ref mut fresh2 =
                        *Av.as_mut_ptr().offset(0 as libc::c_int as isize);
                    *fresh2 =
                        (*av.offset(i as
                                        isize)).offset(2 as libc::c_int as
                                                           isize);
                    if *(*av.offset(i as
                                        isize)).offset(1 as libc::c_int as
                                                           isize) as
                           libc::c_int == 'M' as i32 {
                        v_eqm(str, 0 as *mut _MNE);
                    } else { v_set(str, 0 as *mut _MNE); }
                    current_block = 17788412896529399552;
                }
                102 => { // 'f' - FIXME: convert back to original char
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match Format::try_from(digit) {
                                Ok(result) => { F_format = result; }
                                Err(_) => { panic_rs("Illegal format specification"); }
                            }
                        }
                        Err(_) => { panic_rs("Illegal format specification"); }
                    }
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                111 => {
                    /*  F_outfile   */
                    F_outfile = str;
                    current_block = 15042310719884093888;
                }
                76 => {
                    F_ListAllPasses = 1 as libc::c_int as libc::c_uchar;
                    current_block = 14976246946730902058;
                }
                108 => { current_block = 14976246946730902058; }
                80 => {
                    /*  F_Passes   */
                    bDoAllPasses = 1 as libc::c_int != 0;
                    current_block = 3124391281584211484;
                }
                112 => { current_block = 3124391281584211484; }
                115 => {
                    /*  F_symfile   */
                    F_symfile = str;
                    current_block = 15042310719884093888;
                }
                118 => {
                    /*  F_verbose   */
                    F_verbose =
                        strtol(str,
                               0 as *mut libc::c_void as
                                   *mut *mut libc::c_char, 10 as libc::c_int)
                            as libc::c_int as libc::c_uchar;
                    current_block = 17788412896529399552;
                }
                73 => {
                    v_incdir(str, 0 as *mut _MNE);
                    current_block = 17788412896529399552;
                }
                83 => {
                    bStrictMode = 1 as libc::c_int != 0;
                    current_block = 17788412896529399552;
                }
                _ => { current_block = 15878785573848117940; break ; }
            }
            match current_block {
                14976246946730902058 =>
                /* fall through to 'l' */
                /*  F_listfile  */
                {
                    F_listfile = str;
                    current_block = 15042310719884093888;
                }
                3124391281584211484 =>
                /* fall through to 'p' */
                /*  F_passes   */
                {
                    nMaxPasses =
                        strtol(str,
                               0 as *mut libc::c_void as
                                   *mut *mut libc::c_char, 10 as libc::c_int)
                            as libc::c_int;
                    current_block = 17788412896529399552;
                }
                _ => { }
            }
            match current_block {
                15042310719884093888 => {
                    if *str as libc::c_int == 0 as libc::c_int {
                        panic(b"-o Switch requires file name.\x00" as
                                  *const u8 as *const libc::c_char);
                    }
                }
                _ => { }
            }
            i += 1
        }
        match current_block {
            15878785573848117940 => { }
            _ => {
                /*    INITIAL SEGMENT */
                let mut seg: *mut _SEGMENT =
                    permalloc(::std::mem::size_of::<_SEGMENT>() as
                                  libc::c_ulong as libc::c_int) as
                        *mut _SEGMENT;
                (*seg).name =
                    strcpy(permalloc(::std::mem::size_of::<[libc::c_char; 21]>()
                                         as libc::c_ulong as libc::c_int),
                           b"INITIAL CODE SEGMENT\x00" as *const u8 as
                               *const libc::c_char);
                (*seg).initrflags = 0x1 as libc::c_int as libc::c_uchar;
                (*seg).initflags = (*seg).initrflags;
                (*seg).rflags = (*seg).initflags;
                (*seg).flags = (*seg).rflags;
                Seglist = seg;
                Csegment = Seglist;
                /*    TOP LEVEL IF    */
                let mut ifs: *mut _IFSTACK =
                    zmalloc(::std::mem::size_of::<_IFSTACK>() as libc::c_ulong
                                as libc::c_int) as *mut _IFSTACK;
                (*ifs).file = 0 as *mut _INCFILE;
                (*ifs).flags = 0x4 as libc::c_int as libc::c_uchar;
                (*ifs).acctrue = 1 as libc::c_int as libc::c_uchar;
                (*ifs).xtrue = 1 as libc::c_int as libc::c_uchar;
                Ifstack = ifs;
                // ready error and message buffer...
                passbuffer_clear(0 as libc::c_int);
                passbuffer_clear(1 as libc::c_int);
                loop  {
                    if F_verbose != 0 {
                        puts(b"\x00" as *const u8 as *const libc::c_char);
                        printf(b"START OF PASS: %d\n\x00" as *const u8 as
                                   *const libc::c_char, pass);
                    }
                    Lastlocalindex = 0 as libc::c_int as libc::c_ulong;
                    Localindex = Lastlocalindex;
                    Lastlocaldollarindex = 0 as libc::c_int as libc::c_ulong;
                    Localdollarindex = Lastlocaldollarindex;
                    /*_fmode = 0x8000;*/
                    FI_temp =
                        fopen(F_outfile,
                              b"wb\x00" as *const u8 as *const libc::c_char);
                    /*_fmode = 0;*/
                    Fisclear = 1 as libc::c_int as libc::c_uchar;
                    CheckSum = 0 as libc::c_int as libc::c_ulong;
                    if FI_temp.is_null() {
                        printf(b"Warning: Unable to [re]open \'%s\'\n\x00" as
                                   *const u8 as *const libc::c_char,
                               F_outfile);
                        return AsmErrorEquates::FileError
                    }
                    if !F_listfile.is_null() {
                        FI_listfile =
                            fopen(F_listfile,
                                  if F_ListAllPasses as libc::c_int != 0 &&
                                         pass > 1 as libc::c_int {
                                      b"a\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b"w\x00" as *const u8 as
                                          *const libc::c_char
                                  });
                        if FI_listfile.is_null() {
                            printf(b"Warning: Unable to [re]open \'%s\'\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   F_listfile);
                            return AsmErrorEquates::FileError
                        }
                    }
                    pushinclude(*av.offset(1 as libc::c_int as isize));
                    while !pIncfile.is_null() {
                        loop  {
                            let mut comment: *const libc::c_char =
                                0 as *const libc::c_char;
                            if (*pIncfile).flags as libc::c_int &
                                   0x1 as libc::c_int != 0 {
                                if (*pIncfile).strlist.is_null() {
                                    let ref mut fresh3 =
                                        *Av.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize);
                                    *fresh3 =
                                        b"\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char;
                                    v_mexit(0 as *mut libc::c_char,
                                            0 as *mut _MNE);
                                    continue ;
                                } else {
                                    strcpy(buf.as_mut_ptr(),
                                           (*(*pIncfile).strlist).buf.as_mut_ptr());
                                    (*pIncfile).strlist =
                                        (*(*pIncfile).strlist).next
                                }
                            } else if fgets(buf.as_mut_ptr(),
                                            MAX_LINES as libc::c_int,
                                            (*pIncfile).fi).is_null() {
                                break ;
                            }
                            if Xdebug {
                                printf(b"%08lx %s\n\x00" as *const u8 as
                                           *const libc::c_char,
                                       pIncfile as libc::c_ulong,
                                       buf.as_mut_ptr());
                            }
                            comment =
                                cleanup(buf.as_mut_ptr(),
                                        0 as libc::c_int != 0);
                            (*pIncfile).lineno =
                                (*pIncfile).lineno.wrapping_add(1);
                            mne = parse(buf.as_mut_ptr());
                            if *(*Av.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                   != 0 {
                                if !mne.is_null() {
                                    if (*mne).flags as libc::c_int &
                                           0x4 as libc::c_int != 0 ||
                                           (*Ifstack).xtrue as libc::c_int !=
                                               0 &&
                                               (*Ifstack).acctrue as
                                                   libc::c_int != 0 {
                                        Some((*mne).vect.expect("non-null function pointer")).expect("non-null function pointer")(*Av.as_mut_ptr().offset(2
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                              as
                                                                                                                                                              isize),
                                                                                                                                  mne);
                                    }
                                } else if (*Ifstack).xtrue as libc::c_int != 0
                                              &&
                                              (*Ifstack).acctrue as
                                                  libc::c_int != 0 {
                                    asmerr(AsmErrorEquates::UnknownMnemonic,
                                           0 as libc::c_int != 0,
                                           *Av.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize));
                                }
                            } else if (*Ifstack).xtrue as libc::c_int != 0 &&
                                          (*Ifstack).acctrue as libc::c_int !=
                                              0 {
                                programlabel();
                            }
                            if !F_listfile.is_null() &&
                                   ListMode as libc::c_int != 0 {
                                outlistfile(comment);
                            }
                        }
                        while !Reploop.is_null() &&
                                  (*Reploop).file == pIncfile {
                            rmnode(&mut Reploop as *mut *mut _REPLOOP as
                                       *mut *mut libc::c_void,
                                   ::std::mem::size_of::<_REPLOOP>() as
                                       libc::c_ulong as libc::c_int);
                        }
                        while (*Ifstack).file == pIncfile {
                            rmnode(&mut Ifstack as *mut *mut _IFSTACK as
                                       *mut *mut libc::c_void,
                                   ::std::mem::size_of::<_IFSTACK>() as
                                       libc::c_ulong as libc::c_int);
                        }
                        fclose((*pIncfile).fi);
                        free((*pIncfile).name as *mut libc::c_void);
                        Inclevel -= 1;
                        rmnode(&mut pIncfile as *mut *mut _INCFILE as
                                   *mut *mut libc::c_void,
                               ::std::mem::size_of::<_INCFILE>() as
                                   libc::c_ulong as libc::c_int);
                        if !pIncfile.is_null() {
                            /*
        if (F_verbose > 1)
        printf("back to: %s\n", Incfile->name);
            */
                            if !F_listfile.is_null() {
                                fprintf(FI_listfile,
                                        b"------- FILE %s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        (*pIncfile).name);
                            }
                        }
                    }
                    if F_verbose as libc::c_int >= 1 as libc::c_int {
                        ShowSegments();
                    }
                    if F_verbose as libc::c_int >= 3 as libc::c_int {
                        if Redo == 0 ||
                               F_verbose as libc::c_int == 4 as libc::c_int {
                            ShowSymbols(stdout, *pbTableSort);
                        }
                        ShowUnresolvedSymbols();
                    }
                    closegenerate();
                    fclose(FI_temp);
                    if !FI_listfile.is_null() { fclose(FI_listfile); }
                    if Redo != 0 {
                        if !bDoAllPasses {
                            if Redo == oldredo && Redo_why == oldwhy &&
                                   Redo_eval == oldeval {
                                ShowUnresolvedSymbols();
                                return AsmErrorEquates::NotResolvable
                            }
                        }
                        oldredo = Redo;
                        oldwhy = Redo_why;
                        oldeval = Redo_eval;
                        Redo = 0 as libc::c_int;
                        Redo_why = 0 as libc::c_int as libc::c_ulong;
                        Redo_eval = 0 as libc::c_int;
                        Redo_if <<= 1 as libc::c_int;
                        pass += 1;
                        if pass > nMaxPasses {
                            let mut sBuffer: [libc::c_char; 64] = [0; 64];
                            sprintf(sBuffer.as_mut_ptr(),
                                    b"%d\x00" as *const u8 as
                                        *const libc::c_char, pass);
                            return asmerr(AsmErrorEquates::TooManyPasses,
                                          0 as libc::c_int != 0,
                                          sBuffer.as_mut_ptr())
                        } else {
                            passbuffer_clear(0 as libc::c_int);
                            passbuffer_clear(1 as libc::c_int);
                            clearrefs();
                            clearsegs();
                        }
                    } else {
                        // Do not print any errors if assembly is successful!!!!! -FXQ
    // only print messages from last pass and if there's no errors
                        if !bStopAtEnd {
                            passbuffer_output(1 as libc::c_int);
                        } else {
                            // Only print errors if assembly is unsuccessful!!!!!
        // by FXQ
                            passbuffer_output(0 as libc::c_int);
                            printf(b"Unrecoverable error(s) in pass, aborting assembly!\n\x00"
                                       as *const u8 as *const libc::c_char);
                            nError = AsmErrorEquates::NonAbort;
                        }
                        printf(b"Complete.\n\x00" as *const u8 as
                                   *const libc::c_char);
                        return nError
                    }
                }
            }
        }
    }
    puts(dasm_id.as_ptr());
    puts(b"Copyright (c) 1988-2020 by the DASM team.\x00" as *const u8 as
             *const libc::c_char);
    puts(b"License GPLv2+: GNU GPL version 2 or later (see file LICENSE).\x00"
             as *const u8 as *const libc::c_char);
    puts(b"DASM is free software: you are free to change and redistribute it.\x00"
             as *const u8 as *const libc::c_char);
    puts(b"There is ABSOLUTELY NO WARRANTY, to the extent permitted by law.\x00"
             as *const u8 as *const libc::c_char);
    puts(b"\x00" as *const u8 as *const libc::c_char);
    puts(b"Usage: dasm sourcefile [options]\x00" as *const u8 as
             *const libc::c_char);
    puts(b"\x00" as *const u8 as *const libc::c_char);
    puts(b"-f#      output format 1-3 (default 1)\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-oname   output file name (else a.out)\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-lname   list file name (else none generated)\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-Lname   list file, containing all passes\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-sname   symbol dump file name (else none generated)\x00" as
             *const u8 as *const libc::c_char);
    puts(b"-v#      verboseness 0-4 (default 0)\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-d       debug mode (for developers)\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-Dsymbol              define symbol, set to 0\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-Dsymbol=expression   define symbol, set to expression\x00" as
             *const u8 as *const libc::c_char);
    puts(b"-Msymbol=expression   define symbol using EQM (same as -D)\x00" as
             *const u8 as *const libc::c_char);
    puts(b"-Idir    search directory for INCLUDE and INCBIN\x00" as *const u8
             as *const libc::c_char);
    puts(b"-p#      maximum number of passes\x00" as *const u8 as
             *const libc::c_char);
    puts(b"-P#      maximum number of passes, with fewer checks\x00" as
             *const u8 as *const libc::c_char);
    puts(b"-T#      symbol table sorting (default 0 = alphabetical, 1 = address/value)\x00"
             as *const u8 as *const libc::c_char);
    puts(b"-E#      error format (default 0 = MS, 1 = Dillon, 2 = GNU)\x00" as
             *const u8 as *const libc::c_char);
    puts(b"-S       strict syntax checking\x00" as *const u8 as
             *const libc::c_char);
    puts(b"\x00" as *const u8 as *const libc::c_char);
    puts(b"Report bugs on https://github.com/dasm-assembler/dasm please!\x00"
             as *const u8 as *const libc::c_char);
    return AsmErrorEquates::CommandLine;
}
#[no_mangle]
pub unsafe extern "C" fn addmsg(mut message: *mut libc::c_char)
 // add to message buffer (FXQ)
 {
    passbuffer_update(1 as libc::c_int, message);
}
unsafe extern "C" fn tabit(mut buf1: *mut libc::c_char,
                           mut buf2: *mut libc::c_char) -> libc::c_int {
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    bp = buf2;
    ptr = buf1;
    j = 0 as libc::c_int;
    while *ptr as libc::c_int != 0 && *ptr as libc::c_int != '\n' as i32 {
        *bp = *ptr;
        if *ptr as libc::c_int == '\t' as i32 {
            /* optimize out spaces before the tab */
            while j > 0 as libc::c_int &&
                      *bp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                          == ' ' as i32 {
                bp = bp.offset(-1);
                j -= 1
            }
            j = 0 as libc::c_int;
            *bp = '\t' as i32 as libc::c_char
            /* recopy the tab */
        }
        if j == 7 as libc::c_int && *bp as libc::c_int == ' ' as i32 &&
               *bp.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                   ' ' as i32 {
            k = j;
            loop  {
                let fresh4 = k;
                k = k - 1;
                if !(fresh4 >= 0 as libc::c_int &&
                         *bp as libc::c_int == ' ' as i32) {
                    break ;
                }
                bp = bp.offset(-1)
            }
            bp = bp.offset(1);
            *bp = '\t' as i32 as libc::c_char
        }
        ptr = ptr.offset(1);
        bp = bp.offset(1);
        j = j + 1 as libc::c_int & 7 as libc::c_int
    }
    while bp != buf2 &&
              (*bp.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                   ' ' as i32 ||
                   *bp.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                       '\t' as i32) {
        bp = bp.offset(-1)
    }
    let fresh5 = bp;
    bp = bp.offset(1);
    *fresh5 = '\n' as i32 as libc::c_char;
    *bp = '\u{0}' as i32 as libc::c_char;
    return bp.wrapping_offset_from(buf2) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn outlistfile(mut comment: *const libc::c_char) {
    let mut xtrue: libc::c_char = 0;
    let mut c: libc::c_char = 0;
    static mut buf1: [libc::c_char; MAX_LINES + 32] = [0; MAX_LINES + 32];
    static mut buf2: [libc::c_char; MAX_LINES + 32] = [0; MAX_LINES + 32];
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut dot: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*pIncfile).flags as libc::c_int & 0x2 as libc::c_int != 0 { return }
    xtrue =
        if (*Ifstack).xtrue as libc::c_int != 0 &&
               (*Ifstack).acctrue as libc::c_int != 0 {
            ' ' as i32
        } else { '-' as i32 } as libc::c_char;
    c =
        if Pflags & 0x10 as libc::c_int as libc::c_ulong != 0 {
            'U' as i32
        } else { ' ' as i32 } as libc::c_char;
    ptr = Extstr;
    dot = b"\x00" as *const u8 as *const libc::c_char;
    if !ptr.is_null() {
        dot = b".\x00" as *const u8 as *const libc::c_char
    } else { ptr = b"\x00" as *const u8 as *const libc::c_char }
    sprintf(buf1.as_mut_ptr(),
            b"%7ld %c%s\x00" as *const u8 as *const libc::c_char,
            (*pIncfile).lineno, c as libc::c_int,
            sftos(Plab as libc::c_long,
                  (Pflags & 7 as libc::c_int as libc::c_ulong) as
                      libc::c_int));
    j = strlen(buf1.as_mut_ptr()) as libc::c_int;
    i = 0 as libc::c_int;
    while i < Glen && i < 4 as libc::c_int {
        sprintf(buf1.as_mut_ptr().offset(j as isize),
                b"%02x \x00" as *const u8 as *const libc::c_char,
                *Gen.as_mut_ptr().offset(i as isize) as libc::c_int);
        i += 1;
        j += 3 as libc::c_int
    }
    if i < Glen && i == 4 as libc::c_int {
        xtrue = '*' as i32 as libc::c_char
    }
    while i < 4 as libc::c_int {
        buf1[(j + 2 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        buf1[(j + 1 as libc::c_int) as usize] =
            buf1[(j + 2 as libc::c_int) as usize];
        buf1[j as usize] = buf1[(j + 1 as libc::c_int) as usize];
        j += 3 as libc::c_int;
        i += 1
    }
    sprintf(buf1.as_mut_ptr().offset(j as
                                         isize).offset(-(1 as libc::c_int as
                                                             isize)),
            b"%c%-10s %s%s%s\t%s\n\x00" as *const u8 as *const libc::c_char,
            xtrue as libc::c_int,
            *Av.as_mut_ptr().offset(0 as libc::c_int as isize),
            *Av.as_mut_ptr().offset(1 as libc::c_int as isize), dot, ptr,
            *Av.as_mut_ptr().offset(2 as libc::c_int as isize));
    if *comment.offset(0 as libc::c_int as isize) != 0 {
        /*  tab and comment */
        j =
            strlen(buf1.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                libc::c_int;
        sprintf(buf1.as_mut_ptr().offset(j as isize),
                b"\t;%s\x00" as *const u8 as *const libc::c_char, comment);
    }
    fwrite(buf2.as_mut_ptr() as *const libc::c_void,
           tabit(buf1.as_mut_ptr(), buf2.as_mut_ptr()) as size_t,
           1 as libc::c_int as size_t, FI_listfile);
    Glen = 0 as libc::c_int;
    Extstr = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sftos(mut val: libc::c_long, mut flags: libc::c_int)
 -> *mut libc::c_char {
    static mut buf: [libc::c_char; MAX_SYMBOLS + 14] = [0; MAX_SYMBOLS + 14];
    static mut c: libc::c_char = 0;
    let mut ptr: *mut libc::c_char =
        if c as libc::c_int != 0 {
            buf.as_mut_ptr()
        } else {
            buf.as_mut_ptr().offset((::std::mem::size_of::<[libc::c_char; 1038]>()
                                         as
                                         libc::c_ulong).wrapping_div(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as isize)
        };
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 1038]>() as libc::c_ulong);
    c = (1 as libc::c_int - c as libc::c_int) as libc::c_char;
    sprintf(ptr, b"%04lx \x00" as *const u8 as *const libc::c_char, val);
    if flags & 0x1 as libc::c_int != 0 {
        strcat(ptr, b"???? \x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b"     \x00" as *const u8 as *const libc::c_char); }
    if flags & 0x8 as libc::c_int != 0 {
        strcat(ptr, b"str \x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b"    \x00" as *const u8 as *const libc::c_char); }
    if flags & 0x20 as libc::c_int != 0 {
        strcat(ptr, b"eqm \x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b"    \x00" as *const u8 as *const libc::c_char); }
    if flags & (0x40 as libc::c_int | 0x10 as libc::c_int) != 0 {
        strcat(ptr, b"(\x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b" \x00" as *const u8 as *const libc::c_char); }
    if flags & 0x40 as libc::c_int != 0 {
        strcat(ptr, b"R\x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b" \x00" as *const u8 as *const libc::c_char); }
    if flags & 0x10 as libc::c_int != 0 {
        strcat(ptr, b"S\x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b" \x00" as *const u8 as *const libc::c_char); }
    if flags & (0x40 as libc::c_int | 0x10 as libc::c_int) != 0 {
        strcat(ptr, b")\x00" as *const u8 as *const libc::c_char);
    } else { strcat(ptr, b" \x00" as *const u8 as *const libc::c_char); }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn clearsegs() {
    let mut seg: *mut _SEGMENT = 0 as *mut _SEGMENT;
    seg = Seglist;
    while !seg.is_null() {
        (*seg).flags =
            ((*seg).flags as libc::c_int & 0x10 as libc::c_int |
                 0x1 as libc::c_int) as libc::c_uchar;
        (*seg).initrflags = 0x1 as libc::c_int as libc::c_uchar;
        (*seg).initflags = (*seg).initrflags;
        (*seg).rflags = (*seg).initflags;
        seg = (*seg).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn clearrefs() {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: libc::c_short = 0;
    i = 0 as libc::c_int as libc::c_short;
    while (i as libc::c_int) < S_HASH_SIZE as libc::c_int {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() {
            (*sym).flags =
                ((*sym).flags as libc::c_int & !(0x4 as libc::c_int)) as
                    libc::c_uchar;
            sym = (*sym).next
        }
        i += 1
    };
}
/*
   replace old atoi() calls; I wanted to protect this using
   #ifdef strtol but the C preprocessor doesn't recognize
   function names, at least not GCC's; we should be safe
   since MS compilers document strtol as well... [phf]
*/
unsafe extern "C" fn cleanup(mut buf: *mut libc::c_char, mut bDisable: bool)
 -> *const libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strlist: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut arg: libc::c_int = 0;
    let mut add: libc::c_int = 0;
    let mut comment: *const libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char;
    str = buf;
    while *str != 0 {
        match *str as libc::c_int {
            59 => { comment = str.offset(1 as libc::c_int as isize); break ; }
            13 | 10 => { break ; }
            CHAR_TAB => { *str = ' ' as i32 as libc::c_char }
            39 => {
                str = str.offset(1);
                if *str as libc::c_int == CHAR_TAB {
                    *str = ' ' as i32 as libc::c_char
                }
                if *str as libc::c_int == '\n' as i32 ||
                       *str as libc::c_int == 0 as libc::c_int {
                    *str.offset(0 as libc::c_int as isize) =
                        ' ' as i32 as libc::c_char;
                    *str.offset(1 as libc::c_int as isize) =
                        0 as libc::c_int as libc::c_char
                }
                if *str.offset(0 as libc::c_int as isize) as libc::c_int ==
                       ' ' as i32 {
                    *str.offset(0 as libc::c_int as isize) =
                        -128i32 as libc::c_char
                }
            }
            34 => {
                str = str.offset(1);
                while *str as libc::c_int != 0 &&
                          *str as libc::c_int != '\"' as i32 {
                    if *str as libc::c_int == ' ' as i32 {
                        *str = -128i32 as libc::c_char
                    }
                    str = str.offset(1)
                }
                if *str as libc::c_int != '\"' as i32 {
                    asmerr(AsmErrorEquates::SyntaxError,
                           0 as libc::c_int != 0, buf);
                    str = str.offset(-1)
                }
            }
            123 => {
                if !bDisable {
                    if Xdebug {
                        printf(b"macro tail: \'%s\'\n\x00" as *const u8 as
                                   *const libc::c_char, str);
                    }
                    arg =
                        strtol(str.offset(1 as libc::c_int as isize),
                               0 as *mut libc::c_void as
                                   *mut *mut libc::c_char, 10 as libc::c_int)
                            as libc::c_int;
                    add = 0 as libc::c_int;
                    while *str as libc::c_int != 0 &&
                              *str as libc::c_int != '}' as i32 {
                        add -= 1;
                        str = str.offset(1)
                    }
                    if *str as libc::c_int != '}' as i32 {
                        puts(b"end brace required\x00" as *const u8 as
                                 *const libc::c_char);
                        str = str.offset(-1)
                    } else {
                        add -= 1;
                        str = str.offset(1);
                        if Xdebug {
                            printf(b"add/str: %d \'%s\'\n\x00" as *const u8 as
                                       *const libc::c_char, add, str);
                        }
                        strlist = (*pIncfile).args;
                        while arg != 0 && !strlist.is_null() {
                            arg -= 1;
                            strlist = (*strlist).next
                        }
                        if !strlist.is_null() {
                            add =
                                (add as
                                     libc::c_ulong).wrapping_add(strlen((*strlist).buf.as_mut_ptr()))
                                    as libc::c_int as libc::c_int;
                            if Xdebug {
                                printf(b"strlist: \'%s\' %zu\n\x00" as
                                           *const u8 as *const libc::c_char,
                                       (*strlist).buf.as_mut_ptr(),
                                       strlen((*strlist).buf.as_mut_ptr()));
                            }
                            if str.offset(add as
                                              isize).offset(strlen(str) as
                                                                isize).offset(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                   > buf.offset(MAX_LINES as isize)
                               {
                                if Xdebug {
                                    printf(b"str %8ld buf %8ld (add/strlen(str)): %d %ld\n\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           str as libc::c_ulong,
                                           buf as libc::c_ulong, add,
                                           strlen(str) as libc::c_long);
                                }
                                panic(b"failure1\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            memmove(str.offset(add as isize) as
                                        *mut libc::c_void,
                                    str as *const libc::c_void,
                                    strlen(str).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong));
                            str = str.offset(add as isize);
                            if str.offset(-(strlen((*strlist).buf.as_mut_ptr())
                                                as isize)) < buf {
                                panic(b"failure2\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            memmove(str.offset(-(strlen((*strlist).buf.as_mut_ptr())
                                                     as isize)) as
                                        *mut libc::c_void,
                                    (*strlist).buf.as_mut_ptr() as
                                        *const libc::c_void,
                                    strlen((*strlist).buf.as_mut_ptr()));
                            str =
                                str.offset(-(strlen((*strlist).buf.as_mut_ptr())
                                                 as isize));
                            if str < buf ||
                                   str >=
                                       buf.offset(MAX_LINES as isize) {
                                panic(b"failure 3\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            str = str.offset(-1)
                            /*  for loop increments string    */
                        } else {
                            asmerr(AsmErrorEquates::NotEnoughArgumentsPassedToMacro,
                                   0 as libc::c_int != 0,
                                   0 as *const libc::c_char);
                            break ;
                        }
                    }
                }
            }
            _ => { }
        }
        str = str.offset(1)
    }
    /*    FALL THROUGH    */
    while str != buf &&
              *str.offset(-(1 as libc::c_int as isize)) as libc::c_int ==
                  ' ' as i32 {
        str = str.offset(-1)
    }
    *str = 0 as libc::c_int as libc::c_char;
    return comment;
}
#[no_mangle]
pub unsafe extern "C" fn panic(mut str: *const libc::c_char) {
    puts(str);
    exit(1 as libc::c_int);
}

// FIXME: this should probably be a panic!() instead, but for now we use this to follow original DASM C behavior.
// The output for errors should also use eprintln!(), but once again, the original used puts() instead.
pub fn panic_rs(message: &str) {
    println!("{}", message);
    std::process::exit(1);
}

/*
*  .dir    direct              x
*  .ext    extended              x
*  .r          relative              x
*  .x          index, no offset          x
*  .x8     index, byte offset          x
*  .x16    index, word offset          x
*  .bit    bit set/clr
*  .bbr    bit and branch
*  .imp    implied (inherent)          x
*  .b                      x
*  .w                      x
*  .l                      x
*  .u                      x
*/
#[no_mangle]
pub unsafe extern "C" fn findext(mut str: *mut libc::c_char) {
    Mnext = -(1 as libc::c_int);
    Extstr = 0 as *mut libc::c_char;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        /* Allow .OP for OP */
        return
    }
    while *str as libc::c_int != 0 && *str as libc::c_int != '.' as i32 {
        str = str.offset(1)
    }
    if *str != 0 {
        *str = 0 as libc::c_int as libc::c_char;
        str = str.offset(1);
        Extstr = str;
        match *str.offset(0 as libc::c_int as isize) as libc::c_int |
                  0x20 as libc::c_int {
            48 | 105 => {
                Mnext = AddressModes::Imp as i32;
                match *str.offset(1 as libc::c_int as isize) as libc::c_int |
                          0x20 as libc::c_int {
                    120 => { Mnext = AddressModes::ZeroX as i32 }
                    121 => { Mnext = AddressModes::ZeroY as i32 }
                    110 => { Mnext = AddressModes::IndWord as i32 }
                    _ => { }
                }
                return
            }
            100 | 98 | 122 => {
                match *str.offset(1 as libc::c_int as isize) as libc::c_int |
                          0x20 as libc::c_int {
                    120 => { Mnext = AddressModes::ByteAdrX as i32 }
                    121 => { Mnext = AddressModes::ByteAdrY as i32 }
                    105 => { Mnext = AddressModes::BitMod as i32 }
                    98 => { Mnext = AddressModes::BitBraMod as i32 }
                    _ => { Mnext = AddressModes::ByteAdr as i32 }
                }
                return
            }
            101 | 119 | 97 => {
                match *str.offset(1 as libc::c_int as isize) as libc::c_int |
                          0x20 as libc::c_int {
                    120 => { Mnext = AddressModes::WordAdrX as i32 }
                    121 => { Mnext = AddressModes::WordAdrY as i32 }
                    _ => { Mnext = AddressModes::WordAdr as i32 }
                }
                return
            }
            108 => { Mnext = AddressModes::Long as i32; return }
            114 => { Mnext = AddressModes::Rel as i32; return }
            117 => { Mnext = AddressModes::BSS as i32; return }
            _ => { }
        }
    };
}
/*
*  bytes arg will eventually be used to implement a linked list of free
*  nodes.
*  Assumes *base is really a pointer to a structure with .next as the first
*  member.
*/
#[no_mangle]
pub unsafe extern "C" fn rmnode(mut base: *mut *mut libc::c_void,
                                mut bytes: libc::c_int) {
    let mut node: *mut libc::c_void = 0 as *mut libc::c_void;
    node = *base;
    if !node.is_null() {
        *base = *(node as *mut *mut libc::c_void);
        free(node);
    };
}
/*
*  Parse into three arguments: Av[0], Av[1], Av[2]
*/
#[no_mangle]
pub unsafe extern "C" fn parse(mut buf: *mut libc::c_char) -> *mut _MNE {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut labelundefined: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    j = 1 as libc::c_int;
    /*
        If the first non-space is a ^, skip all further spaces too.
        This means what follows is a label.
        If the first non-space is a #, what follows is a directive/opcode.
    */
    while *buf.offset(i as isize) as libc::c_int == ' ' as i32 { i += 1 }
    if *buf.offset(i as isize) as libc::c_int == '^' as i32 {
        i += 1;
        while *buf.offset(i as isize) as libc::c_int == ' ' as i32 { i += 1 }
    } else if *buf.offset(i as isize) as libc::c_int == '#' as i32 {
        *buf.offset(i as isize) = ' ' as i32 as libc::c_char
        /* label separator */
    } else { i = 0 as libc::c_int }
    let ref mut fresh6 = *Av.as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh6 = Avbuf.as_mut_ptr().offset(j as isize);
    while *buf.offset(i as isize) as libc::c_int != 0 &&
              *buf.offset(i as isize) as libc::c_int != ' ' as i32 &&
              *buf.offset(i as isize) as libc::c_int != '=' as i32 {
        if *buf.offset(i as isize) as libc::c_int == ':' as i32 {
            i += 1;
            break ;
        } else if *buf.offset(i as isize) as libc::c_int == ',' as i32 {
            // we have label arguments
            if *buf.offset((i + 1 as libc::c_int) as isize) as libc::c_int ==
                   '\"' as i32 {
                // is it a string constant?
                i = i + 2 as libc::c_int; // advance to string contents
                while *buf.offset(i as isize) as libc::c_int != 0 &&
                          *buf.offset(i as isize) as libc::c_int !=
                              '\"' as i32 &&
                          *buf.offset(i as isize) as libc::c_int != ' ' as i32
                          &&
                          *buf.offset(i as isize) as libc::c_int != ',' as i32
                          &&
                          *buf.offset(i as isize) as libc::c_int != ':' as i32
                      {
                    let fresh7 = i;
                    i = i + 1;
                    let fresh8 = j;
                    j = j + 1;
                    *Avbuf.as_mut_ptr().offset(fresh8 as isize) =
                        *buf.offset(fresh7 as isize)
                }
                if *buf.offset(i as isize) as libc::c_int != 0 &&
                       *buf.offset(i as isize) as libc::c_int == '\"' as i32 {
                    i += 1
                } else {
                    labelundefined += 1;
                    asmerr(AsmErrorEquates::SyntaxError,
                           0 as libc::c_int != 0, buf);
                }
            } else {
                // or else it's a symbol to be evaluated, and added to the label
                let mut t: libc::c_int = 0;
                let mut tempbuf: [libc::c_char; 257] = [0; 257];
                let mut tempval: [libc::c_char; 257] = [0; 257];
                let mut symarg: *mut _SYMBOL = 0 as *mut _SYMBOL;
                strncpy(tempbuf.as_mut_ptr(),
                        buf.offset(i as
                                       isize).offset(1 as libc::c_int as
                                                         isize),
                        256 as libc::c_int as libc::c_ulong);
                tempbuf[256 as libc::c_int as usize] =
                    0 as libc::c_int as libc::c_char;
                t = 0 as libc::c_int;
                while (t as libc::c_ulong) < strlen(tempbuf.as_mut_ptr()) {
                    if tempbuf[t as usize] as libc::c_int == ',' as i32 {
                        tempbuf[t as usize] = 0 as libc::c_int as libc::c_char
                    }
                    t += 1
                }
                symarg = eval(tempbuf.as_mut_ptr(), 0 as libc::c_int);
                if !symarg.is_null() {
                    if (*symarg).flags as libc::c_int & 0x1 as libc::c_int !=
                           0 {
                        // one of the arguments isn't defined yet
                        labelundefined += 1
                    } else {
                        snprintf(tempval.as_mut_ptr(),
                                 256 as libc::c_int as libc::c_ulong,
                                 b"%d\x00" as *const u8 as
                                     *const libc::c_char,
                                 (*symarg).value as
                                     libc::c_uint); // ensure the label we're creating doesn't get used
                        strcpy(Avbuf.as_mut_ptr().offset(j as isize),
                               tempval.as_mut_ptr());
                        j =
                            (j as
                                 libc::c_ulong).wrapping_add(strlen(tempval.as_mut_ptr()))
                                as libc::c_int
                    }
                }
                i += 1;
                while *buf.offset(i as isize) as libc::c_int != 0 &&
                          *buf.offset(i as isize) as libc::c_int != ' ' as i32
                          &&
                          *buf.offset(i as isize) as libc::c_int != '=' as i32
                          &&
                          *buf.offset(i as isize) as libc::c_int != ',' as i32
                          &&
                          *buf.offset(i as isize) as libc::c_int != ':' as i32
                      {
                    i += 1
                }
            }
        } else {
            if *buf.offset(i as isize) as libc::c_uchar as libc::c_int ==
                   0x80 as libc::c_int {
                *buf.offset(i as isize) = ' ' as i32 as libc::c_char
            }
            let fresh9 = i;
            i = i + 1;
            let fresh10 = j;
            j = j + 1;
            *Avbuf.as_mut_ptr().offset(fresh10 as isize) =
                *buf.offset(fresh9 as isize)
        }
    }
    let fresh11 = j;
    j = j + 1;
    *Avbuf.as_mut_ptr().offset(fresh11 as isize) =
        0 as libc::c_int as libc::c_char;
    // if the label has arguments that aren't defined, we need to scuttle it
    // to avoid a partial label being used.
    if labelundefined != 0 {
        j = 1 as libc::c_int;
        let fresh12 = j;
        j = j + 1;
        *Avbuf.as_mut_ptr().offset(fresh12 as isize) =
            0 as libc::c_int as libc::c_char
    }
    /* Parse the second word of the line */
    while *buf.offset(i as isize) as libc::c_int == ' ' as i32 { i += 1 }
    let ref mut fresh13 = *Av.as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh13 = Avbuf.as_mut_ptr().offset(j as isize);
    if *buf.offset(i as isize) as libc::c_int == '=' as i32 {
        /* '=' directly seperates Av[0] and Av[2] */
        let fresh14 = i;
        i = i + 1;
        let fresh15 = j;
        j = j + 1;
        *Avbuf.as_mut_ptr().offset(fresh15 as isize) =
            *buf.offset(fresh14 as isize)
    } else {
        while *buf.offset(i as isize) as libc::c_int != 0 &&
                  *buf.offset(i as isize) as libc::c_int != ' ' as i32 {
            if *buf.offset(i as isize) as libc::c_uchar as libc::c_int ==
                   0x80 as libc::c_int {
                *buf.offset(i as isize) = ' ' as i32 as libc::c_char
            }
            let fresh16 = i;
            i = i + 1;
            let fresh17 = j;
            j = j + 1;
            *Avbuf.as_mut_ptr().offset(fresh17 as isize) =
                *buf.offset(fresh16 as isize)
        }
    }
    let fresh18 = j;
    j = j + 1;
    *Avbuf.as_mut_ptr().offset(fresh18 as isize) =
        0 as libc::c_int as libc::c_char;
    /* and analyse it as an opcode */
    findext(*Av.as_mut_ptr().offset(1 as libc::c_int as isize));
    mne = findmne(*Av.as_mut_ptr().offset(1 as libc::c_int as isize));
    /* Parse the rest of the line */
    while *buf.offset(i as isize) as libc::c_int == ' ' as i32 { i += 1 }
    let ref mut fresh19 = *Av.as_mut_ptr().offset(2 as libc::c_int as isize);
    *fresh19 = Avbuf.as_mut_ptr().offset(j as isize);
    while *buf.offset(i as isize) != 0 {
        if *buf.offset(i as isize) as libc::c_int == ' ' as i32 {
            while *buf.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                      == ' ' as i32 {
                i += 1
            }
        }
        if *buf.offset(i as isize) as libc::c_uchar as libc::c_int ==
               0x80 as libc::c_int {
            *buf.offset(i as isize) = ' ' as i32 as libc::c_char
        }
        let fresh20 = i;
        i = i + 1;
        let fresh21 = j;
        j = j + 1;
        *Avbuf.as_mut_ptr().offset(fresh21 as isize) =
            *buf.offset(fresh20 as isize)
    }
    *Avbuf.as_mut_ptr().offset(j as isize) = 0 as libc::c_int as libc::c_char;
    return mne;
}
#[no_mangle]
pub unsafe extern "C" fn findmne(mut str: *mut libc::c_char) -> *mut _MNE {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut buf: [libc::c_char; 64] = [0; 64];
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        /* Allow .OP for OP */
        str = str.offset(1)
    }
    i = 0 as libc::c_int;
    loop  {
        c = *str.offset(i as isize);
        if !(c != 0) { break ; }
        if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
            c = (c as libc::c_int + ('a' as i32 - 'A' as i32)) as libc::c_char
        }
        buf[i as usize] = c;
        i += 1
    }
    buf[i as usize] = 0 as libc::c_int as libc::c_char;
    mne = *MHash.as_mut_ptr().offset(hash1(buf.as_mut_ptr()) as isize);
    while !mne.is_null() {
        if strcmp(buf.as_mut_ptr(), (*mne).name) == 0 as libc::c_int {
            break ;
        }
        mne = (*mne).next
    }
    return mne;
}
/* symbols.c */
/* ops.c */
#[no_mangle]
pub unsafe extern "C" fn v_macro(mut str: *mut libc::c_char,
                                 mut dummy: *mut _MNE) {
    let mut base: *mut _STRLIST =
        0 as *mut _STRLIST; /* slp, mac: might be used uninitialised */
    let mut defined: libc::c_int = 0 as libc::c_int; /* not really needed */
    let mut slp: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut sl: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut mac: *mut _MACRO = 0 as *mut _MACRO;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut i: libc::c_uint = 0;
    let mut buf: [libc::c_char; MAX_LINES] = [0; MAX_LINES];
    let mut skipit: libc::c_int =
        !((*Ifstack).xtrue as libc::c_int != 0 &&
              (*Ifstack).acctrue as libc::c_int != 0) as libc::c_int;
    strlower(str);
    mne = findmne(str);
    if skipit != 0 {
        defined = 1 as libc::c_int
    } else {
        defined = (mne != 0 as *mut libc::c_void as *mut _MNE) as libc::c_int;
        if !F_listfile.is_null() && ListMode as libc::c_int != 0 {
            outlistfile(b"\x00" as *const u8 as *const libc::c_char);
        }
    }
    if defined == 0 {
        base = 0 as *mut _STRLIST;
        slp = &mut base;
        mac =
            permalloc(::std::mem::size_of::<_MACRO>() as libc::c_ulong as
                          libc::c_int) as *mut _MACRO;
        i = hash1(str);
        (*mac).next = *MHash.as_mut_ptr().offset(i as isize) as *mut _MACRO;
        (*mac).vect =
            Some(v_execmac as
                     unsafe extern "C" fn(_: *mut libc::c_char,
                                          _: *mut _MACRO) -> ());
        (*mac).name =
            strcpy(permalloc(strlen(str).wrapping_add(1 as libc::c_int as
                                                          libc::c_ulong) as
                                 libc::c_int), str);
        (*mac).flags = 0x8 as libc::c_int as libc::c_uchar;
        (*mac).defpass = pass;
        let ref mut fresh22 = *MHash.as_mut_ptr().offset(i as isize);
        *fresh22 = mac as *mut _MNE
    } else {
        mac = mne as *mut _MACRO;
        if bStrictMode as libc::c_int != 0 && !mac.is_null() &&
               (*mac).defpass == pass {
            asmerr(AsmErrorEquates::MacroRepeated, 1 as libc::c_int != 0,
                   str);
        }
    }
    while !fgets(buf.as_mut_ptr(), MAX_LINES as libc::c_int,
                 (*pIncfile).fi).is_null() {
        let mut comment: *const libc::c_char = 0 as *const libc::c_char;
        if Xdebug {
            printf(b"%08lx %s\n\x00" as *const u8 as *const libc::c_char,
                   pIncfile as libc::c_ulong, buf.as_mut_ptr());
        }
        (*pIncfile).lineno = (*pIncfile).lineno.wrapping_add(1);
        comment = cleanup(buf.as_mut_ptr(), 1 as libc::c_int != 0);
        mne = parse(buf.as_mut_ptr());
        if *(*Av.as_mut_ptr().offset(1 as libc::c_int as
                                         isize)).offset(0 as libc::c_int as
                                                            isize) != 0 {
            if !mne.is_null() &&
                   (*mne).flags as libc::c_int & 0x80 as libc::c_int != 0 {
                if defined == 0 { (*mac).strlist = base }
                return
            }
        }
        if skipit == 0 && !F_listfile.is_null() &&
               ListMode as libc::c_int != 0 {
            outlistfile(comment);
        }
        if defined == 0 {
            sl =
                permalloc((::std::mem::size_of::<*mut _STRLIST>() as
                               libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                               libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr()))
                              as libc::c_int) as *mut _STRLIST;
            strcpy((*sl).buf.as_mut_ptr(), buf.as_mut_ptr());
            *slp = sl;
            slp = &mut (*sl).next
        }
    }
    asmerr(AsmErrorEquates::PrematureEOF, 1 as libc::c_int != 0,
           0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn addhashtable(mut mne: *mut _MNE) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut opcode: [libc::c_uint; 21] = [0; 21];
    while (*mne).vect.is_some() {
        memcpy(opcode.as_mut_ptr() as *mut libc::c_void,
               (*mne).opcode.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_uint; 21]>() as libc::c_ulong);
        j = 0 as libc::c_int;
        i = j;
        while i < AddressModes::length() as libc::c_int {
            (*mne).opcode[i as usize] = 0 as libc::c_int as libc::c_uint;
            if (*mne).okmask & ((1 as libc::c_long) << i) as libc::c_ulong !=
                   0 {
                let fresh23 = j;
                j = j + 1;
                (*mne).opcode[i as usize] = opcode[fresh23 as usize]
            }
            i += 1
        }
        i = hash1((*mne).name) as libc::c_int;
        (*mne).next = *MHash.as_mut_ptr().offset(i as isize);
        let ref mut fresh24 = *MHash.as_mut_ptr().offset(i as isize);
        *fresh24 = mne;
        mne = mne.offset(1)
    };
}
unsafe extern "C" fn hash1(mut str: *const libc::c_char) -> libc::c_uint {
    let mut result: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *str != 0 {
        let fresh25 = str;
        str = str.offset(1);
        result = result << 2 as libc::c_int ^ *fresh25 as libc::c_uint
    }
    return result & M_HASH_AND as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn pushinclude(mut str: *mut libc::c_char) {
    let mut inf: *mut _INCFILE = 0 as *mut _INCFILE;
    let mut fi: *mut FILE = 0 as *mut FILE;
    fi = pfopen(str, b"rb\x00" as *const u8 as *const libc::c_char);
    if !fi.is_null() {
        if F_verbose as libc::c_int > 1 as libc::c_int &&
               F_verbose as libc::c_int != 5 as libc::c_int {
            printf(b"%.*s Including file \"%s\"\n\x00" as *const u8 as
                       *const libc::c_char,
                   Inclevel as libc::c_int * 4 as libc::c_int,
                   b"\x00" as *const u8 as *const libc::c_char, str);
        }
        Inclevel += 1;
        if !F_listfile.is_null() {
            fprintf(FI_listfile,
                    b"------- FILE %s LEVEL %d PASS %d\n\x00" as *const u8 as
                        *const libc::c_char, str, Inclevel as libc::c_int,
                    pass);
        }
        inf =
            zmalloc(::std::mem::size_of::<_INCFILE>() as libc::c_ulong as
                        libc::c_int) as *mut _INCFILE;
        (*inf).next = pIncfile;
        (*inf).name =
            strcpy(ckmalloc(strlen(str).wrapping_add(1 as libc::c_int as
                                                         libc::c_ulong) as
                                libc::c_int), str);
        (*inf).fi = fi;
        (*inf).lineno = 0 as libc::c_int as libc::c_ulong;
        pIncfile = inf;
        return
    }
    printf(b"Warning: Unable to open \'%s\'\n\x00" as *const u8 as
               *const libc::c_char, str);
}
#[no_mangle]
pub unsafe extern "C" fn asmerr(mut err: AsmErrorEquates, mut bAbort: bool,
                                mut sText: *const libc::c_char)
 -> AsmErrorEquates {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut pincfile: *mut _INCFILE = 0 as *mut _INCFILE;
    /* file pointer we print error messages to */
    let mut error_file: *mut FILE = 0 as *mut FILE;
    if sErrorDef[err as usize].fatal {
        bStopAtEnd = 1 as libc::c_int != 0
    }
    pincfile = pIncfile;
    while (*pincfile).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        pincfile = (*pincfile).next
    }
    // FIXME: use real strings; this is a bit messy
    let mut desc = sErrorDef[err as usize].description.clone().to_owned();
    desc.push_str("\n\x00");
    str = desc.as_ptr() as *const i8;
    /*
        New error format selection for 2.20.11 since some
        people *don't* use MS products. For historical
        reasons we currently send errors to stdout when
        they should really go to stderr, but we'll switch
        eventually I hope... [phf]
    */
    /* determine the file pointer to use */
    error_file = if !F_listfile.is_null() { FI_listfile } else { stdout };
    /* print first part of message, different formats offered */
    match F_errorformat {
        ErrorFormat::Woe => {
            /*
                Error format for MS VisualStudio and relatives:
                "file (line): error: string"
            */
            if error_file != stdout {
                fprintf(error_file,
                        b"%s (%lu): error: \x00" as *const u8 as
                            *const libc::c_char, (*pincfile).name,
                        (*pincfile).lineno); // -FXQ
            }
            sprintf(erroradd1.as_mut_ptr(),
                    b"%s (%lu): error: \x00" as *const u8 as
                        *const libc::c_char, (*pincfile).name,
                    (*pincfile).lineno);
        }
        ErrorFormat::Dillon => {
            /*
                Matthew Dillon's original format, except that
                we don't distinguish writing to the terminal
                from writing to the list file for now. Matt's
                2.16 uses these:

                    "*line %4ld %-10s %s\n" (list file)
                    "line %4ld %-10s %s\n" (terminal)
            */
            if error_file != stdout {
                fprintf(error_file,
                        b"line %7ld %-10s \x00" as *const u8 as
                            *const libc::c_char, (*pincfile).lineno,
                        (*pincfile).name); // -FXQ
            }
            sprintf(erroradd1.as_mut_ptr(),
                    b"line %7ld %-10s \x00" as *const u8 as
                        *const libc::c_char, (*pincfile).lineno,
                    (*pincfile).name);
        }
        ErrorFormat::GNU => {
            /*
                GNU format error messages, from their coding
                standards.
            */
            if error_file != stdout {
                fprintf(error_file,
                        b"%s:%lu: error: \x00" as *const u8 as
                            *const libc::c_char, (*pincfile).name,
                        (*pincfile).lineno); // -FXQ
            }
            sprintf(erroradd1.as_mut_ptr(),
                    b"%s:%lu: error: \x00" as *const u8 as
                        *const libc::c_char, (*pincfile).name,
                    (*pincfile).lineno);
        }
    }
    if error_file != stdout {
        /* print second part of message, always the same for now */
        fprintf(error_file, str,
                if !sText.is_null() {
                    sText
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                }); // dump messages from this pass
        fprintf(error_file,
                b"\n\x00" as *const u8 as
                    *const libc::c_char); // time to dump the errors from this pass!
    }
    sprintf(erroradd2.as_mut_ptr(), str,
            if !sText.is_null() {
                sText
            } else { b"\x00" as *const u8 as *const libc::c_char });
    sprintf(erroradd3.as_mut_ptr(),
            b"\n\x00" as *const u8 as *const libc::c_char);
    passbuffer_update(0 as libc::c_int, erroradd1.as_mut_ptr());
    passbuffer_update(0 as libc::c_int, erroradd2.as_mut_ptr());
    passbuffer_update(0 as libc::c_int, erroradd3.as_mut_ptr());
    if bAbort {
        passbuffer_output(1 as libc::c_int);
        fprintf(error_file,
                b"Aborting assembly\n\x00" as *const u8 as
                    *const libc::c_char);
        passbuffer_output(0 as libc::c_int);
        exit(1 as libc::c_int);
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc(mut bytes: libc::c_int)
 -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = ckmalloc(bytes);
    if !ptr.is_null() {
        memset(ptr as *mut libc::c_void, 0 as libc::c_int,
               bytes as libc::c_ulong);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ckmalloc(mut bytes: libc::c_int)
 -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char =
        malloc(bytes as libc::c_ulong) as *mut libc::c_char;
    if !ptr.is_null() { return ptr }
    panic(b"unable to malloc\x00" as *const u8 as *const libc::c_char);
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn permalloc(mut bytes: libc::c_int)
 -> *mut libc::c_char {
    static mut buf: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    static mut left: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Assume sizeof(union align) is a power of 2 */
    bytes =
        ((bytes as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<align>() as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<align>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong)) as
            libc::c_int;
    if bytes > left {
        buf =
            malloc(ALLOC_SIZE as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if buf.is_null() {
            panic(b"unable to malloc\x00" as *const u8 as
                      *const libc::c_char);
        }
        memset(buf as *mut libc::c_void, 0 as libc::c_int,
            ALLOC_SIZE as libc::c_int as libc::c_ulong);
        left = ALLOC_SIZE as libc::c_int;
        if bytes > left {
            panic(b"software error\x00" as *const u8 as *const libc::c_char);
        }
    }
    ptr = buf;
    buf = buf.offset(bytes as isize);
    left -= bytes;
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn strlower(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = str;
    loop  {
        c = *ptr;
        if !(c != 0) { break ; }
        if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
            *ptr = (c as libc::c_int | 0x20 as libc::c_int) as libc::c_char
        }
        ptr = ptr.offset(1)
    }
    return str;
}
unsafe fn main_0(mut ac: libc::c_int, mut av: *mut *mut libc::c_char)
 -> u8 {
    let mut bTableSort: bool = 0 as libc::c_int != 0;
    let mut nError: AsmErrorEquates = MainShadow(ac, av, &mut bTableSort);
    if nError != AsmErrorEquates::None && nError != AsmErrorEquates::NonAbort {
        // dump messages when aborting due to errors
        passbuffer_output(1 as libc::c_int);
        // Only print errors if assembly is unsuccessful
        passbuffer_output(0 as libc::c_int);
        printf(b"Fatal assembly error: %s\n\x00" as *const u8 as
                   *const libc::c_char,
               sErrorDef[nError as usize].description);
    }
    DumpSymbolTable(bTableSort);
    passbuffer_cleanup();
    return nError.into();
}
#[no_mangle]
pub unsafe extern "C" fn passbuffer_clear(mut mbindex: libc::c_int) {
    // ensure the buffer is initialized before we attempt to clear it,
    // just in case no messages have been stored prior to this clear.
    if passbuffer[mbindex as usize].is_null() {
        passbuffer_update(mbindex,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
    }
    // clear the requested guffer
    *passbuffer[mbindex as usize].offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn passbuffer_update(mut mbindex: libc::c_int,
                                           mut message: *mut libc::c_char) {
    let mut newsizerequired: libc::c_int = 0;
    // allocate 16k buffers to start...
    static mut passbuffersize: [libc::c_int; 2] =
        [16384 as libc::c_int, 16384 as libc::c_int];
    // check if the buffer we're working with needs initialization
    if passbuffer[mbindex as usize].is_null() {
        passbuffer[mbindex as usize] =
            malloc(passbuffersize[mbindex as usize] as libc::c_ulong) as
                *mut libc::c_char;
        if passbuffer[mbindex as usize].is_null() {
            panic(b"couldn\'t allocate memory for message buffer.\x00" as
                      *const u8 as *const libc::c_char);
        }
        *passbuffer[mbindex as usize].offset(0 as libc::c_int as isize) =
            0 as libc::c_int as libc::c_char
        // empty string
    }
    // check if we need to grow the buffer...
    newsizerequired =
        strlen(passbuffer[mbindex as usize]).wrapping_add(strlen(message)) as
            libc::c_int;
    if newsizerequired > passbuffersize[mbindex as usize] {
        // double the current buffer size, if sufficient, so we don't continually reallocate memory...
        newsizerequired =
            if newsizerequired <
                   passbuffersize[mbindex as usize] * 2 as libc::c_int {
                (passbuffersize[mbindex as usize]) * 2 as libc::c_int
            } else { newsizerequired };
        //fprintf(stderr,"DEBUG: growing buffer %d to %d bytes\n", mbindex, newsizerequired);
        passbuffer[mbindex as usize] =
            realloc(passbuffer[mbindex as usize] as *mut libc::c_void,
                    newsizerequired as libc::c_ulong) as *mut libc::c_char;
        if passbuffer[mbindex as usize].is_null() {
            panic(b"couldn\'t grow memory for message buffer.\x00" as
                      *const u8 as *const libc::c_char);
        }
        passbuffersize[mbindex as usize] = newsizerequired
    }
    // update the buffer with the message...
    strcat(passbuffer[mbindex as usize], message);
}
#[no_mangle]
pub unsafe extern "C" fn passbuffer_output(mut mbindex: libc::c_int) {
    // ensure the buffer is initialized before we attempt to clear it,
    // just in case no messages have been stored yet.
    if passbuffer[mbindex as usize].is_null() {
        passbuffer_update(mbindex,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char);
    }
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           passbuffer[mbindex as usize]);
    // ...do we really still need to put this through stdout, instead stderr?
}
#[no_mangle]
pub unsafe extern "C" fn passbuffer_cleanup() {
    let mut t: libc::c_int = 0;
    t = 0 as libc::c_int;
    while t < 2 as libc::c_int {
        if !passbuffer[t as usize].is_null() {
            free(passbuffer[t as usize] as *mut libc::c_void);
        }
        t += 1
    };
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
