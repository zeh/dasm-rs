use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    /* ops.c */
    #[no_mangle]
    fn v_dc(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_mexit(str: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_list(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_include(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_setstr(str: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_set(str: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_seg(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_align(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_rend(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_rorg(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_org(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_trace(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_end(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_equ(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_ds(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_err(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_hex(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_eqm(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_macro(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_endm(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_incdir(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_incbin(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_ifconst(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_processor(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_repend(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_repeat(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_endif(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_else(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_if(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_ifnconst(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_echo(_: *mut libc::c_char, _: *mut _MNE);
    #[no_mangle]
    fn v_subroutine(_: *mut libc::c_char, _: *mut _MNE);
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
pub type sortmode_t = libc::c_uint;
pub const SORTMODE_MAX: sortmode_t = 2;
pub const SORTMODE_ADDRESS: sortmode_t = 1;
pub const SORTMODE_ALPHA: sortmode_t = 0;
pub const SORTMODE_DEFAULT: sortmode_t = 0;
pub type errorformat_t = libc::c_uint;
pub const ERRORFORMAT_MAX: errorformat_t = 3;
pub const ERRORFORMAT_GNU: errorformat_t = 2;
pub const ERRORFORMAT_DILLON: errorformat_t = 1;
pub const ERRORFORMAT_WOE: errorformat_t = 0;
pub const ERRORFORMAT_DEFAULT: errorformat_t = 0;
pub type FORMAT = libc::c_uint;
pub const FORMAT_MAX: FORMAT = 4;
pub const FORMAT_RAW: FORMAT = 3;
pub const FORMAT_RAS: FORMAT = 2;
pub const FORMAT_DEFAULT: FORMAT = 1;
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
 *  GLOBALS.C
 */
#[no_mangle]
pub static mut SHash: [*mut _SYMBOL; 1024] =
    [0 as *const _SYMBOL as *mut _SYMBOL; 1024];
/*	symbol hash table   */
#[no_mangle]
pub static mut MHash: [*mut _MNE; 1024] =
    [0 as *const _MNE as *mut _MNE; 1024];
/*	mnemonic hash table */
#[no_mangle]
pub static mut pIncfile: *mut _INCFILE =
    0 as *const _INCFILE as *mut _INCFILE;
/*	include file stack  */
#[no_mangle]
pub static mut Reploop: *mut _REPLOOP = 0 as *const _REPLOOP as *mut _REPLOOP;
/*	repeat loop stack   */
#[no_mangle]
pub static mut Seglist: *mut _SEGMENT = 0 as *const _SEGMENT as *mut _SEGMENT;
/*	segment list	    */
#[no_mangle]
pub static mut Csegment: *mut _SEGMENT =
    0 as *const _SEGMENT as *mut _SEGMENT;
/*	current segment     */
#[no_mangle]
pub static mut Ifstack: *mut _IFSTACK = 0 as *const _IFSTACK as *mut _IFSTACK;
/*	IF/ELSE/ENDIF stack */
#[no_mangle]
pub static mut Av: [*mut libc::c_char; 256] =
    [0 as *const libc::c_char as *mut libc::c_char; 256];
/*	up to 256 arguments */
#[no_mangle]
pub static mut Avbuf: [libc::c_char; 512] = [0; 512];
#[no_mangle]
pub static mut MsbOrder: libc::c_uchar = 1 as libc::c_int as libc::c_uchar;
#[no_mangle]
pub static mut Mnext: libc::c_int = 0;
#[no_mangle]
pub static mut Inclevel: libc::c_char = 0;
#[no_mangle]
pub static mut Mlevel: libc::c_uint = 0;
#[no_mangle]
pub static mut Localindex: libc::c_ulong = 0;
/*  to generate local variables */
#[no_mangle]
pub static mut Lastlocalindex: libc::c_ulong = 0;
#[no_mangle]
pub static mut Localdollarindex: libc::c_ulong = 0;
#[no_mangle]
pub static mut Lastlocaldollarindex: libc::c_ulong = 0;
#[no_mangle]
pub static mut Processor: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub static mut bTrace: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut Xdebug: bool = false;
#[no_mangle]
pub static mut bStrictMode: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut Outputformat: libc::c_uchar = 0;
#[no_mangle]
pub static mut Redo_why: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub static mut Redo_eval: libc::c_int = 0 as libc::c_int;
/*  infinite loop detection only    */
#[no_mangle]
pub static mut Redo: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut Redo_if: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub static mut ListMode: libc::c_char = 1 as libc::c_int as libc::c_char;
#[no_mangle]
pub static mut CheckSum: libc::c_ulong = 0;
/*	output data checksum		*/
#[no_mangle]
pub static mut F_format: libc::c_int = FORMAT_DEFAULT as libc::c_int;
/* -T option [phf] */
#[no_mangle]
pub static mut F_sortmode: sortmode_t = SORTMODE_DEFAULT;
/* -E option [phf] */
#[no_mangle]
pub static mut F_errorformat: errorformat_t = ERRORFORMAT_DEFAULT;
#[no_mangle]
pub static mut F_verbose: libc::c_uchar = 0;
#[no_mangle]
pub static mut F_outfile: *const libc::c_char =
    b"a.out\x00" as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut F_listfile: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut F_symfile: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut FI_listfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut FI_temp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut Fisclear: libc::c_uchar = 0;
#[no_mangle]
pub static mut Plab: libc::c_ulong = 0;
#[no_mangle]
pub static mut Pflags: libc::c_ulong = 0;
/*unsigned int	Adrbytes[]  = { 1, 2, 3, 2, 2, 2, 3, 3, 3, 2, 2, 2, 3, 1, 1, 2, 3 };*/
#[no_mangle]
pub static mut Cvt: [libc::c_uint; 17] =
    [0 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 6 as libc::c_int as libc::c_uint,
     7 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     9 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     5 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint];
#[no_mangle]
pub static mut Opsize: [libc::c_uint; 17] =
    [0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint];
#[no_mangle]
pub static mut Ops: [_MNE; 39] =
    unsafe {
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_list as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"list\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_include as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"include\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_seg as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"seg\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_hex as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"hex\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_err as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"err\x00" as *const u8 as *const libc::c_char,
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
                      name: b"dc\x00" as *const u8 as *const libc::c_char,
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
                      name: b"byte\x00" as *const u8 as *const libc::c_char,
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
                      name: b"word\x00" as *const u8 as *const libc::c_char,
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
                      name: b"long\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_ds as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ds\x00" as *const u8 as *const libc::c_char,
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
                      name: b"dv\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_end as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"end\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_trace as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"trace\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_org as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"org\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_rorg as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rorg\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_rend as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rend\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_align as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"align\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_subroutine as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"subroutine\x00" as *const u8 as
                              *const libc::c_char,
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
                          Some(v_equ as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"equ\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_equ as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"=\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_eqm as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"eqm\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_set as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"set\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_setstr as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"setstr\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_macro as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"mac\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_endm as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"endm\x00" as *const u8 as *const libc::c_char,
                      flags: 0x80 as libc::c_int as libc::c_uchar,
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
                          Some(v_mexit as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"mexit\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_ifconst as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"ifconst\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_ifnconst as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"ifnconst\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_if as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"if\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_else as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"else\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_endif as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"endif\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_endif as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"eif\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_repeat as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"repeat\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_repend as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"repend\x00" as *const u8 as *const libc::c_char,
                      flags: 0x4 as libc::c_int as libc::c_uchar,
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
                          Some(v_echo as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"echo\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_processor as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"processor\x00" as *const u8 as
                              *const libc::c_char,
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
                          Some(v_incbin as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"incbin\x00" as *const u8 as *const libc::c_char,
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
                          Some(v_incdir as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"incdir\x00" as *const u8 as *const libc::c_char,
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