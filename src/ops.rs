use libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut bTrace: bool;
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
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
    static mut Cvt: [libc::c_uint; 0];
    #[no_mangle]
    static mut Opsize: [libc::c_uint; 0];
    #[no_mangle]
    static mut Mnext: libc::c_int;
    #[no_mangle]
    static mut Mlevel: libc::c_uint;
    #[no_mangle]
    static mut ListMode: libc::c_char;
    #[no_mangle]
    static mut Processor: libc::c_ulong;
    #[no_mangle]
    static mut CheckSum: libc::c_ulong;
    #[no_mangle]
    fn findext(str: *mut libc::c_char);
    #[no_mangle]
    fn asmerr(err: libc::c_int, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sftos(val: libc::c_long, flags: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn rmnode(base: *mut *mut libc::c_void, bytes: libc::c_int);
    #[no_mangle]
    fn addhashtable(mne: *mut _MNE);
    #[no_mangle]
    fn pushinclude(str: *mut libc::c_char);
    #[no_mangle]
    fn permalloc(bytes: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn zmalloc(bytes: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ckmalloc(bytes: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn addmsg(message: *mut libc::c_char);
    #[no_mangle]
    fn setspecial(value: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn findsymbol(str: *const libc::c_char, len: libc::c_int) -> *mut _SYMBOL;
    #[no_mangle]
    fn CreateSymbol(str: *const libc::c_char, len: libc::c_int)
     -> *mut _SYMBOL;
    #[no_mangle]
    fn FreeSymbolList(sym: *mut _SYMBOL);
    #[no_mangle]
    static mut Fisclear: libc::c_uchar;
    #[no_mangle]
    static mut FI_temp: *mut FILE;
    #[no_mangle]
    static mut F_listfile: *mut libc::c_char;
    #[no_mangle]
    static mut FI_listfile: *mut FILE;
    #[no_mangle]
    static mut bStrictMode: bool;
    #[no_mangle]
    static mut MsbOrder: libc::c_uchar;
    #[no_mangle]
    static mut Redo_why: libc::c_ulong;
    #[no_mangle]
    static mut F_format: libc::c_int;
    #[no_mangle]
    static mut Lastlocaldollarindex: libc::c_ulong;
    #[no_mangle]
    fn programlabel();
    #[no_mangle]
    static mut Localdollarindex: libc::c_ulong;
    #[no_mangle]
    static mut Redo: libc::c_int;
    #[no_mangle]
    static mut Redo_if: libc::c_ulong;
    #[no_mangle]
    static mut Localindex: libc::c_ulong;
    #[no_mangle]
    static mut Lastlocalindex: libc::c_ulong;
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const libc::c_char, wantmode: libc::c_int) -> *mut _SYMBOL;
    #[no_mangle]
    static mut Mne6502: [_MNE; 0];
    #[no_mangle]
    static mut Mne6803: [_MNE; 0];
    #[no_mangle]
    static mut MneHD6303: [_MNE; 0];
    #[no_mangle]
    static mut Mne68705: [_MNE; 0];
    #[no_mangle]
    static mut Mne68HC11: [_MNE; 0];
    #[no_mangle]
    static mut MneF8: [_MNE; 0];
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
pub type FORMAT = libc::c_uint;
pub const FORMAT_MAX: FORMAT = 4;
pub const FORMAT_RAW: FORMAT = 3;
pub const FORMAT_RAS: FORMAT = 2;
pub const FORMAT_DEFAULT: FORMAT = 1;
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
pub struct _STRLIST {
    pub next: *mut _STRLIST,
    // Conversion note: this buffer size was 4 originally, but increased to fix a buffer overrun
    pub buf: [libc::c_char; 16],
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
/*
 *  OPS.C
 *
 *  Handle mnemonics and pseudo ops
 */
#[no_mangle]
pub static mut Gen: [libc::c_uchar; 1024] = [0; 1024];
#[no_mangle]
pub static mut OrgFill: libc::c_uchar = 255 as libc::c_int as libc::c_uchar;
#[no_mangle]
pub static mut Glen: libc::c_int = 0;
/*
*  An opcode modifies the SEGMENT flags in the following ways:
*/
#[no_mangle]
pub unsafe extern "C" fn v_processor(mut str: *mut libc::c_char,
                                     mut dummy: *mut _MNE) {
    static mut bCalled: bool = 0 as libc::c_int != 0; /*	lsb,msb */
    let mut PreviousProcessor: libc::c_ulong = Processor; /*	msb,lsb */
    Processor = 0 as libc::c_int as libc::c_ulong; /*	msb,lsb */
    if strcmp(str, b"6502\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        if !bCalled {
            addhashtable(Mne6502.as_mut_ptr()); /*	msb,lsb */
        } /*	msb,lsb */
        MsbOrder = 0 as libc::c_int as libc::c_uchar;
        Processor = 6502 as libc::c_int as libc::c_ulong
    }
    if strcmp(str, b"6803\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        if !bCalled { addhashtable(Mne6803.as_mut_ptr()); }
        MsbOrder = 1 as libc::c_int as libc::c_uchar;
        Processor = 6803 as libc::c_int as libc::c_ulong
    }
    if strcmp(str, b"HD6303\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(str, b"hd6303\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        if !bCalled {
            addhashtable(Mne6803.as_mut_ptr());
            addhashtable(MneHD6303.as_mut_ptr());
        }
        MsbOrder = 1 as libc::c_int as libc::c_uchar;
        Processor = 6303 as libc::c_int as libc::c_ulong
    }
    if strcmp(str, b"68705\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        if !bCalled { addhashtable(Mne68705.as_mut_ptr()); }
        MsbOrder = 1 as libc::c_int as libc::c_uchar;
        Processor = 68705 as libc::c_int as libc::c_ulong
    }
    if strcmp(str, b"68HC11\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(str, b"68hc11\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        if !bCalled { addhashtable(Mne68HC11.as_mut_ptr()); }
        MsbOrder = 1 as libc::c_int as libc::c_uchar;
        Processor = 6811 as libc::c_int as libc::c_ulong
    }
    if strcmp(str, b"F8\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(str, b"f8\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        if !bCalled { addhashtable(MneF8.as_mut_ptr()); }
        MsbOrder = 1 as libc::c_int as libc::c_uchar;
        Processor = 0xf8 as libc::c_int as libc::c_ulong
    }
    bCalled = 1 as libc::c_int != 0;
    if Processor == 0 {
        asmerr(ERROR_PROCESSOR_NOT_SUPPORTED as libc::c_int,
               1 as libc::c_int != 0, str);
    }
    if PreviousProcessor != 0 && Processor != PreviousProcessor {
        asmerr(ERROR_ONLY_ONE_PROCESSOR_SUPPORTED as libc::c_int,
               1 as libc::c_int != 0, str);
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_mnemonic(mut str: *mut libc::c_char,
                                    mut mne: *mut _MNE) {
    let mut addrmode: libc::c_int = 0;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut opcode: libc::c_uint = 0;
    let mut opidx: libc::c_short = 0;
    let mut symbase: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut opsize: libc::c_int = 0;
    (*Csegment).flags =
        ((*Csegment).flags as libc::c_int | 0x4 as libc::c_int) as
            libc::c_uchar;
    programlabel();
    symbase = eval(str, 1 as libc::c_int);
    if bTrace {
        printf(b"PC: %04lx  MNEMONIC: %s  addrmode: %d  \x00" as *const u8 as
                   *const libc::c_char, (*Csegment).org, (*mne).name,
               (*symbase).addrmode as libc::c_int);
    }
    sym = symbase;
    while !sym.is_null() {
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |=
                REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as libc::c_ulong
        }
        sym = (*sym).next
    }
    sym = symbase;
    if (*mne).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        if !(*sym).next.is_null() {
            (*sym).addrmode = AM_BITMOD as libc::c_int as libc::c_uchar;
            if (*mne).flags as libc::c_int & 0x20 as libc::c_int != 0 &&
                   !(*sym).next.is_null() {
                (*sym).addrmode = AM_BITBRAMOD as libc::c_int as libc::c_uchar
            }
        }
    }
    addrmode = (*sym).addrmode as libc::c_int;
    if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 ||
           (*sym).value >= 0x100 as libc::c_int as libc::c_long {
        opsize = 2 as libc::c_int
    } else {
        opsize =
            if (*sym).value != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }
    }
    while (*mne).okmask & ((1 as libc::c_long) << addrmode) as libc::c_ulong
              == 0 && *Cvt.as_mut_ptr().offset(addrmode as isize) != 0 {
        addrmode = *Cvt.as_mut_ptr().offset(addrmode as isize) as libc::c_int
    }
    if bTrace {
        printf(b"mnemask: %08lx adrmode: %d  Cvt[am]: %d\n\x00" as *const u8
                   as *const libc::c_char, (*mne).okmask, addrmode,
               *Cvt.as_mut_ptr().offset(addrmode as isize));
    }
    if (*mne).okmask & ((1 as libc::c_long) << addrmode) as libc::c_ulong == 0
       {
        let mut sBuffer: [libc::c_char; 128] = [0; 128];
        sprintf(sBuffer.as_mut_ptr(),
                b"%s %s\x00" as *const u8 as *const libc::c_char, (*mne).name,
                str);
        asmerr(ERROR_ILLEGAL_ADDRESSING_MODE as libc::c_int,
               0 as libc::c_int != 0, sBuffer.as_mut_ptr());
        FreeSymbolList(symbase);
        //FIX
        Redo += 1;
        Redo_why |=
            REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as libc::c_ulong;
        return
    }
    if Mnext >= 0 as libc::c_int && Mnext < NUMOC as libc::c_int {
        /*	Force	*/
        addrmode = Mnext;
        if (*mne).okmask & ((1 as libc::c_long) << addrmode) as libc::c_ulong
               == 0 {
            asmerr(ERROR_ILLEGAL_FORCED_ADDRESSING_MODE as libc::c_int,
                   0 as libc::c_int != 0, (*mne).name);
            FreeSymbolList(symbase);
            //FIX: Cause assembly to fail when an invalid mode is used for an opcode...
            Redo += 1;
            Redo_why |=
                REASON_MNEMONIC_NOT_RESOLVED as libc::c_int as libc::c_ulong;
            return
        }
    }
    if bTrace {
        printf(b"final addrmode = %d\n\x00" as *const u8 as
                   *const libc::c_char, addrmode);
    }
    while opsize as libc::c_uint >
              *Opsize.as_mut_ptr().offset(addrmode as isize) {
        if *Cvt.as_mut_ptr().offset(addrmode as isize) ==
               0 as libc::c_int as libc::c_uint ||
               (*mne).okmask &
                   ((1 as libc::c_long) <<
                        *Cvt.as_mut_ptr().offset(addrmode as isize)) as
                       libc::c_ulong == 0 {
            let mut sBuffer_0: [libc::c_char; 128] = [0; 128];
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                break ;
            }
            //FIX: for negative operands...
            if addrmode == AM_IMM8 as libc::c_int &&
                   (*sym).value < 0 as libc::c_int as libc::c_long {
                opsize = 1 as libc::c_int; /*  to end of instruction   */
                (*sym).value =
                    ((*sym).value & 255 as libc::c_int as libc::c_long) as
                        libc::c_char as libc::c_long;
                break ;
            } else {
                sprintf(sBuffer_0.as_mut_ptr(),
                        b"%s %s\x00" as *const u8 as *const libc::c_char,
                        (*mne).name, str);
                asmerr(ERROR_ADDRESS_MUST_BE_LT_100 as libc::c_int,
                       0 as libc::c_int != 0, sBuffer_0.as_mut_ptr());
                break ;
            }
        } else {
            addrmode =
                *Cvt.as_mut_ptr().offset(addrmode as isize) as libc::c_int
        }
    }
    opcode = (*mne).opcode[addrmode as usize];
    opidx =
        (1 as libc::c_int +
             (opcode > 0xff as libc::c_int as libc::c_uint) as libc::c_int) as
            libc::c_short;
    if opidx as libc::c_int == 2 as libc::c_int {
        Gen[0 as libc::c_int as usize] =
            (opcode >> 8 as libc::c_int) as libc::c_uchar;
        Gen[1 as libc::c_int as usize] = opcode as libc::c_uchar
    } else { Gen[0 as libc::c_int as usize] = opcode as libc::c_uchar }
    match addrmode {
        15 => {
            sym = (*symbase).next;
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*sym).value >= 0x100 as libc::c_int as libc::c_long {
                asmerr(ERROR_ADDRESS_MUST_BE_LT_100 as libc::c_int,
                       0 as libc::c_int != 0, 0 as *const libc::c_char);
            }
            let fresh0 = opidx;
            opidx = opidx + 1;
            Gen[fresh0 as usize] = (*sym).value as libc::c_uchar;
            if (*symbase).flags as libc::c_int & 0x1 as libc::c_int == 0 {
                if (*symbase).value > 7 as libc::c_int as libc::c_long {
                    asmerr(ERROR_ILLEGAL_BIT_SPECIFICATION as libc::c_int,
                           0 as libc::c_int != 0, str);
                } else {
                    Gen[0 as libc::c_int as usize] =
                        (Gen[0 as libc::c_int as usize] as libc::c_long +
                             ((*symbase).value << 1 as libc::c_int)) as
                            libc::c_uchar
                }
            }
        }
        16 => {
            if (*symbase).flags as libc::c_int & 0x1 as libc::c_int == 0 {
                if (*symbase).value > 7 as libc::c_int as libc::c_long {
                    asmerr(ERROR_ILLEGAL_BIT_SPECIFICATION as libc::c_int,
                           0 as libc::c_int != 0, str);
                } else {
                    Gen[0 as libc::c_int as usize] =
                        (Gen[0 as libc::c_int as usize] as libc::c_long +
                             ((*symbase).value << 1 as libc::c_int)) as
                            libc::c_uchar
                }
            }
            sym = (*symbase).next;
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*sym).value >= 0x100 as libc::c_int as libc::c_long {
                asmerr(ERROR_ADDRESS_MUST_BE_LT_100 as libc::c_int,
                       0 as libc::c_int != 0, 0 as *const libc::c_char);
            }
            let fresh1 = opidx;
            opidx = opidx + 1;
            Gen[fresh1 as usize] = (*sym).value as libc::c_uchar;
            sym = (*sym).next
        }
        9 => { }
        _ => {
            if *Opsize.as_mut_ptr().offset(addrmode as isize) >
                   0 as libc::c_int as libc::c_uint {
                let fresh2 = opidx;
                opidx = opidx + 1;
                Gen[fresh2 as usize] = (*sym).value as libc::c_uchar
            }
            if *Opsize.as_mut_ptr().offset(addrmode as isize) ==
                   2 as libc::c_int as libc::c_uint {
                if MsbOrder != 0 {
                    Gen[(opidx as libc::c_int - 1 as libc::c_int) as usize] =
                        ((*sym).value >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh3 = opidx;
                    opidx = opidx + 1;
                    Gen[fresh3 as usize] = (*sym).value as libc::c_uchar
                } else {
                    let fresh4 = opidx;
                    opidx = opidx + 1;
                    Gen[fresh4 as usize] =
                        ((*sym).value >> 8 as libc::c_int) as libc::c_uchar
                }
            }
            sym = (*sym).next
        }
    }
    if (*mne).flags as libc::c_int & 0x10 as libc::c_int != 0 {
        if !sym.is_null() {
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*sym).value >= 0x100 as libc::c_int as libc::c_long {
                asmerr(ERROR_ADDRESS_MUST_BE_LT_100 as libc::c_int,
                       0 as libc::c_int != 0, 0 as *const libc::c_char);
            }
            Gen[opidx as usize] = (*sym).value as libc::c_uchar;
            sym = (*sym).next
        } else {
            asmerr(ERROR_NOT_ENOUGH_ARGS as libc::c_int,
                   1 as libc::c_int != 0, 0 as *const libc::c_char);
        }
        opidx += 1
    }
    if (*mne).flags as libc::c_int & 0x20 as libc::c_int != 0 ||
           addrmode == AM_REL as libc::c_int {
        opidx += 1;
        if sym.is_null() {
            asmerr(ERROR_NOT_ENOUGH_ARGS as libc::c_int,
                   1 as libc::c_int != 0, 0 as *const libc::c_char);
        } else if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 {
            let mut pc: libc::c_long = 0;
            let mut pcf: libc::c_uchar = 0;
            let mut dest: libc::c_long = 0;
            pc =
                if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0
                   {
                    (*Csegment).rorg
                } else { (*Csegment).org } as libc::c_long;
            pcf =
                if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0
                   {
                    (*Csegment).rflags as libc::c_int
                } else { (*Csegment).flags as libc::c_int } as libc::c_uchar;
            if pcf as libc::c_int & (0x1 as libc::c_int | 2 as libc::c_int) ==
                   0 as libc::c_int {
                dest = (*sym).value - pc - opidx as libc::c_long;
                if dest >= 128 as libc::c_int as libc::c_long ||
                       dest < -(128 as libc::c_int) as libc::c_long {
                    /*	byte before end of inst.    */
                    //FIX: sometimes zero page addressing will first be assumed to be absolute until
                    //     another pass. ERROR_BRANCH_OUT_OF_RANGE was made non-fatal, but we keep
                    //     pushing for Redo so assembly won't actually be succesfull until the branch
                    //     actually works.
                    let mut sBuffer_1: [libc::c_char; 64] = [0; 64];
                    sprintf(sBuffer_1.as_mut_ptr(),
                            b"%ld\x00" as *const u8 as *const libc::c_char,
                            dest);
                    asmerr(ERROR_BRANCH_OUT_OF_RANGE as libc::c_int,
                           0 as libc::c_int != 0, sBuffer_1.as_mut_ptr());
                    Redo += 1;
                    Redo_why |=
                        REASON_BRANCH_OUT_OF_RANGE as libc::c_int as
                            libc::c_ulong;
                    (*sym).flags =
                        ((*sym).flags as libc::c_int | 0x1 as libc::c_int) as
                            libc::c_uchar;
                    dest = 0 as libc::c_int as libc::c_long
                }
            } else {
                /* Don't bother - we'll take another pass */
                dest = 0 as libc::c_int as libc::c_long
            } /*  Only so outlist() works */
            Gen[(opidx as libc::c_int - 1 as libc::c_int) as usize] =
                (dest & 0xff as libc::c_int as libc::c_long) as libc::c_uchar
        }
    }
    Glen = opidx as libc::c_int;
    generate();
    FreeSymbolList(symbase);
}
#[no_mangle]
pub unsafe extern "C" fn v_trace(mut str: *mut libc::c_char,
                                 mut dummy: *mut _MNE) {
    bTrace =
        *str.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn v_list(mut str: *mut libc::c_char,
                                mut dummy: *mut _MNE) {
    programlabel();
    Glen = 0 as libc::c_int;
    if strncmp(str, b"localoff\x00" as *const u8 as *const libc::c_char,
               7 as libc::c_int as libc::c_ulong) == 0 as libc::c_int ||
           strncmp(str, b"LOCALOFF\x00" as *const u8 as *const libc::c_char,
                   7 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        (*pIncfile).flags =
            ((*pIncfile).flags as libc::c_int | 0x2 as libc::c_int) as
                libc::c_uchar
    } else if strncmp(str, b"localon\x00" as *const u8 as *const libc::c_char,
                      7 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
                  ||
                  strncmp(str,
                          b"LOCALON\x00" as *const u8 as *const libc::c_char,
                          7 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
        (*pIncfile).flags =
            ((*pIncfile).flags as libc::c_int & !(0x2 as libc::c_int)) as
                libc::c_uchar
    } else if strncmp(str, b"off\x00" as *const u8 as *const libc::c_char,
                      2 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
                  ||
                  strncmp(str, b"OFF\x00" as *const u8 as *const libc::c_char,
                          2 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
        ListMode = 0 as libc::c_int as libc::c_char
    } else { ListMode = 1 as libc::c_int as libc::c_char };
}
unsafe extern "C" fn getfilename(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    if *str as libc::c_int == '\"' as i32 {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        str = str.offset(1);
        buf =
            ckmalloc(strlen(str).wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong) as
                         libc::c_int);
        strcpy(buf, str);
        str = buf;
        while *str as libc::c_int != 0 && *str as libc::c_int != '\"' as i32 {
            str = str.offset(1)
        }
        *str = 0 as libc::c_int as libc::c_char;
        return buf
    }
    return str;
}
/* -T option [phf] */
/* -E option [phf] */
/*extern unsigned int _fmode;*/
/* main.c */
/*extern unsigned char Listing;*/
/* symbols.c */
/* ops.c */
#[no_mangle]
pub unsafe extern "C" fn v_include(mut str: *mut libc::c_char,
                                   mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    // only eval the string if it's compliant with symbol naming
    if (*str as libc::c_int) < '0' as i32 || *str as libc::c_int > '9' as i32
       {
        //check could be more comprehensive
        sym = eval(str, 0 as libc::c_int)
    } else { sym = 0 as *mut _SYMBOL }
    if !sym.is_null() && (*sym).flags as libc::c_int & 0x8 as libc::c_int != 0
       {
        pushinclude((*sym).string);
    } else {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        buf = getfilename(str);
        pushinclude(buf);
        if buf != str { free(buf as *mut libc::c_void); }
    }
    if !sym.is_null() { FreeSymbolList(sym); };
}
#[no_mangle]
pub unsafe extern "C" fn v_incbin(mut str: *mut libc::c_char,
                                  mut dummy: *mut _MNE) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut binfile: *mut FILE = 0 as *mut FILE;
    programlabel();
    buf = getfilename(str);
    binfile = pfopen(buf, b"rb\x00" as *const u8 as *const libc::c_char);
    if !binfile.is_null() {
        if Redo != 0 {
            /* optimize: don't actually read the file if not needed */
            fseek(binfile, 0 as libc::c_int as libc::c_long,
                  2 as libc::c_int);
            Glen = ftell(binfile) as libc::c_int;
            generate();
            /* does not access Gen[] if Redo is set */
        } else {
            loop  {
                Glen =
                    fread(Gen.as_mut_ptr() as *mut libc::c_void,
                          1 as libc::c_int as size_t,
                          ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                              libc::c_ulong, binfile) as libc::c_int;
                if Glen <= 0 as libc::c_int { break ; }
                generate();
            }
        }
        fclose(binfile);
    } else {
        printf(b"unable to open %s\n\x00" as *const u8 as *const libc::c_char,
               buf);
    }
    if buf != str { free(buf as *mut libc::c_void); }
    Glen = 0 as libc::c_int;
    /* don't list hexdump */
}
#[no_mangle]
pub unsafe extern "C" fn v_seg(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut seg: *mut _SEGMENT =
        0 as *mut _SEGMENT; /* "might be used uninitialised" */
    seg = Seglist;
    while !seg.is_null() {
        if strcmp(str, (*seg).name) == 0 as libc::c_int {
            Csegment = seg;
            programlabel();
            return
        }
        seg = (*seg).next
    }
    seg =
        zmalloc(::std::mem::size_of::<_SEGMENT>() as libc::c_ulong as
                    libc::c_int) as *mut _SEGMENT;
    Csegment = seg;
    (*seg).next = Seglist;
    (*seg).name =
        strcpy(ckmalloc(strlen(str).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong) as
                            libc::c_int), str);
    (*seg).initrflags = 0x1 as libc::c_int as libc::c_uchar;
    (*seg).initflags = (*seg).initrflags;
    (*seg).rflags = (*seg).initflags;
    (*seg).flags = (*seg).rflags;
    Seglist = seg;
    if Mnext == AM_BSS as libc::c_int {
        (*seg).flags =
            ((*seg).flags as libc::c_int | 0x10 as libc::c_int) as
                libc::c_uchar
    }
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_hex(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    programlabel();
    Glen = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) != 0 {
        if !(*str.offset(i as isize) as libc::c_int == ' ' as i32) {
            result =
                (gethexdig(*str.offset(i as isize) as libc::c_int) <<
                     4 as libc::c_int) +
                    gethexdig(*str.offset((i + 1 as libc::c_int) as isize) as
                                  libc::c_int);
            i += 1;
            if *str.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                break ;
            }
            let fresh5 = Glen;
            Glen = Glen + 1;
            Gen[fresh5 as usize] = result as libc::c_uchar
        }
        i += 1
    }
    generate();
}
#[no_mangle]
pub unsafe extern "C" fn gethexdig(mut c: libc::c_int) -> libc::c_int {
    let mut sBuffer: [libc::c_char; 64] = [0; 64];
    if c >= '0' as i32 && c <= '9' as i32 { return c - '0' as i32 }
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as libc::c_int
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as libc::c_int
    }
    sprintf(sBuffer.as_mut_ptr(),
            b"Bad Hex Digit %c\x00" as *const u8 as *const libc::c_char, c);
    asmerr(ERROR_SYNTAX_ERROR as libc::c_int, 0 as libc::c_int != 0,
           sBuffer.as_mut_ptr());
    puts(b"(Must be a valid hex digit)\x00" as *const u8 as
             *const libc::c_char);
    if !F_listfile.is_null() {
        fputs(b"(Must be a valid hex digit)\n\x00" as *const u8 as
                  *const libc::c_char, FI_listfile);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn v_err(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    programlabel();
    asmerr(ERROR_ERR_PSEUDO_OP_ENCOUNTERED as libc::c_int,
           1 as libc::c_int != 0, 0 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn v_dc(mut str: *mut libc::c_char,
                              mut mne: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut tmp: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut value: libc::c_long = 0;
    let mut macstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vmode: libc::c_char = 0 as libc::c_int as libc::c_char;
    Glen = 0 as libc::c_int;
    programlabel();
    /* for byte, .byte, word, .word, long, .long */
    if *(*mne).name.offset(0 as libc::c_int as isize) as libc::c_int !=
           'd' as i32 {
        static mut sTmp: [libc::c_char; 4] = [0; 4];
        strcpy(sTmp.as_mut_ptr(),
               b"x.x\x00" as *const u8 as *const libc::c_char);
        sTmp[2 as libc::c_int as usize] =
            *(*mne).name.offset(0 as libc::c_int as isize);
        findext(sTmp.as_mut_ptr());
    }
    /* F8... */
    /* db, dw, dd */
    if *(*mne).name.offset(0 as libc::c_int as isize) as libc::c_int ==
           'd' as i32 &&
           *(*mne).name.offset(1 as libc::c_int as isize) as libc::c_int !=
               'c' as i32 &&
           *(*mne).name.offset(1 as libc::c_int as isize) as libc::c_int !=
               'v' as i32 {
        static mut sTmp_0: [libc::c_char; 4] = [0; 4];
        strcpy(sTmp_0.as_mut_ptr(),
               b"x.x\x00" as *const u8 as *const libc::c_char);
        if 'd' as i32 ==
               *(*mne).name.offset(1 as libc::c_int as isize) as libc::c_int {
            sTmp_0[2 as libc::c_int as usize] = 'l' as i32 as libc::c_char
        } else {
            sTmp_0[2 as libc::c_int as usize] =
                *(*mne).name.offset(1 as libc::c_int as isize)
        }
        findext(sTmp_0.as_mut_ptr());
    }
    /* ...F8 */
    if *(*mne).name.offset(1 as libc::c_int as isize) as libc::c_int ==
           'v' as i32 {
        let mut i: libc::c_int = 0;
        vmode = 1 as libc::c_int as libc::c_char;
        i = 0 as libc::c_int;
        while *str.offset(i as isize) as libc::c_int != 0 &&
                  *str.offset(i as isize) as libc::c_int != ' ' as i32 {
            i += 1
        }
        tmp = findsymbol(str, i);
        str = str.offset(i as isize);
        if tmp.is_null() {
            puts(b"EQM label not found\x00" as *const u8 as
                     *const libc::c_char);
            return
        }
        if (*tmp).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            macstr = (*tmp).string as *mut libc::c_void as *mut libc::c_char
        } else {
            puts(b"must specify EQM label for DV\x00" as *const u8 as
                     *const libc::c_char);
            return
        }
    }
    sym = eval(str, 0 as libc::c_int);
    while !sym.is_null() {
        value = (*sym).value;
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |= REASON_DC_NOT_RESOVED as libc::c_int as libc::c_ulong
        }
        if (*sym).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            let mut ptr: *mut libc::c_uchar =
                (*sym).string as *mut libc::c_void as *mut libc::c_uchar;
            loop  {
                value = *ptr as libc::c_long;
                if !(value != 0 as libc::c_int as libc::c_long) { break ; }
                if vmode != 0 {
                    setspecial(value as libc::c_int, 0 as libc::c_int);
                    tmp = eval(macstr, 0 as libc::c_int);
                    value = (*tmp).value;
                    if (*tmp).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                        Redo += 1;
                        Redo_why |=
                            REASON_DV_NOT_RESOLVED_PROBABLY as libc::c_int as
                                libc::c_ulong
                    }
                    FreeSymbolList(tmp);
                }
                match Mnext {
                    6 => {
                        if MsbOrder != 0 {
                            let fresh7 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh7 as usize] =
                                (value >> 8 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar;
                            let fresh8 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh8 as usize] =
                                (value & 0xff as libc::c_int as libc::c_long)
                                    as libc::c_uchar
                        } else {
                            let fresh9 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh9 as usize] =
                                (value & 0xff as libc::c_int as libc::c_long)
                                    as libc::c_uchar;
                            let fresh10 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh10 as usize] =
                                (value >> 8 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar
                        }
                    }
                    19 => {
                        if MsbOrder != 0 {
                            let fresh11 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh11 as usize] =
                                (value >> 24 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar;
                            let fresh12 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh12 as usize] =
                                (value >> 16 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar;
                            let fresh13 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh13 as usize] =
                                (value >> 8 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar;
                            let fresh14 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh14 as usize] =
                                (value & 0xff as libc::c_int as libc::c_long)
                                    as libc::c_uchar
                        } else {
                            let fresh15 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh15 as usize] =
                                (value & 0xff as libc::c_int as libc::c_long)
                                    as libc::c_uchar;
                            let fresh16 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh16 as usize] =
                                (value >> 8 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar;
                            let fresh17 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh17 as usize] =
                                (value >> 16 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar;
                            let fresh18 = Glen;
                            Glen = Glen + 1;
                            Gen[fresh18 as usize] =
                                (value >> 24 as libc::c_int &
                                     0xff as libc::c_int as libc::c_long) as
                                    libc::c_uchar
                        }
                    }
                    3 | _ => {
                        let fresh6 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh6 as usize] =
                            (value & 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar
                    }
                }
                ptr = ptr.offset(1)
            }
        } else {
            if vmode != 0 {
                setspecial(value as libc::c_int, (*sym).flags as libc::c_int);
                tmp = eval(macstr, 0 as libc::c_int);
                value = (*tmp).value;
                if (*tmp).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                    Redo += 1;
                    Redo_why |=
                        REASON_DV_NOT_RESOLVED_COULD as libc::c_int as
                            libc::c_ulong
                }
                FreeSymbolList(tmp);
            }
            match Mnext {
                6 => {
                    //any value outside two's complement +ve and +ve word representation is invalid...
                    if bStrictMode as libc::c_int != 0 &&
                           (value < -(0xffff as libc::c_int) as libc::c_long
                                ||
                                value > 0xffff as libc::c_int as libc::c_long)
                       {
                        let mut sBuffer_0: [libc::c_char; 128] = [0; 128];
                        sprintf(sBuffer_0.as_mut_ptr(),
                                b"%s %ld\x00" as *const u8 as
                                    *const libc::c_char, (*mne).name, value);
                        asmerr(ERROR_ADDRESS_MUST_BE_LT_10000 as libc::c_int,
                               0 as libc::c_int != 0, sBuffer_0.as_mut_ptr());
                    }
                    if MsbOrder != 0 {
                        let fresh20 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh20 as usize] =
                            (value >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh21 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh21 as usize] =
                            (value & 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar
                    } else {
                        let fresh22 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh22 as usize] =
                            (value & 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh23 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh23 as usize] =
                            (value >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar
                    }
                }
                19 => {
                    if MsbOrder != 0 {
                        let fresh24 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh24 as usize] =
                            (value >> 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh25 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh25 as usize] =
                            (value >> 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh26 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh26 as usize] =
                            (value >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh27 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh27 as usize] =
                            (value & 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar
                    } else {
                        let fresh28 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh28 as usize] =
                            (value & 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh29 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh29 as usize] =
                            (value >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh30 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh30 as usize] =
                            (value >> 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar;
                        let fresh31 = Glen;
                        Glen = Glen + 1;
                        Gen[fresh31 as usize] =
                            (value >> 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_long) as
                                libc::c_uchar
                    }
                }
                3 | _ => {
                    //any value outside two's complement +ve and +ve byte representation is invalid...
                    if value < -(0xff as libc::c_int) as libc::c_long ||
                           value > 0xff as libc::c_int as libc::c_long {
                        let mut sBuffer: [libc::c_char; 128] = [0; 128];
                        sprintf(sBuffer.as_mut_ptr(),
                                b"%s %ld\x00" as *const u8 as
                                    *const libc::c_char, (*mne).name, value);
                        asmerr(ERROR_ADDRESS_MUST_BE_LT_100 as libc::c_int,
                               0 as libc::c_int != 0, sBuffer.as_mut_ptr());
                    }
                    let fresh19 = Glen;
                    Glen = Glen + 1;
                    Gen[fresh19 as usize] =
                        (value & 0xff as libc::c_int as libc::c_long) as
                            libc::c_uchar
                }
            }
        }
        sym = (*sym).next
    }
    generate();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_ds(mut str: *mut libc::c_char,
                              mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut mult: libc::c_int = 1 as libc::c_int;
    let mut filler: libc::c_long = 0 as libc::c_int as libc::c_long;
    if Mnext == AM_WORDADR as libc::c_int { mult = 2 as libc::c_int }
    if Mnext == AM_LONG as libc::c_int { mult = 4 as libc::c_int }
    programlabel();
    sym = eval(str, 0 as libc::c_int);
    if !sym.is_null() {
        if !(*sym).next.is_null() { filler = (*(*sym).next).value }
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |= REASON_DS_NOT_RESOLVED as libc::c_int as libc::c_ulong
        } else {
            if !(*sym).next.is_null() &&
                   (*(*sym).next).flags as libc::c_int & 0x1 as libc::c_int !=
                       0 {
                Redo += 1;
                Redo_why |=
                    REASON_DS_NOT_RESOLVED as libc::c_int as libc::c_ulong
            }
            genfill(filler, (*sym).value, mult);
        }
        FreeSymbolList(sym);
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_org(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    sym = eval(str, 0 as libc::c_int);
    (*Csegment).org = (*sym).value as libc::c_ulong;
    if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        (*Csegment).flags =
            ((*Csegment).flags as libc::c_int | 0x1 as libc::c_int) as
                libc::c_uchar
    } else {
        (*Csegment).flags =
            ((*Csegment).flags as libc::c_int & !(0x1 as libc::c_int)) as
                libc::c_uchar
    }
    if (*Csegment).initflags as libc::c_int & 0x1 as libc::c_int != 0 {
        (*Csegment).initorg = (*sym).value as libc::c_ulong;
        (*Csegment).initflags = (*sym).flags
    }
    if !(*sym).next.is_null() {
        OrgFill = (*(*sym).next).value as libc::c_uchar;
        if (*(*sym).next).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            asmerr(ERROR_VALUE_UNDEFINED as libc::c_int,
                   1 as libc::c_int != 0, 0 as *const libc::c_char);
        }
    }
    programlabel();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_rorg(mut str: *mut libc::c_char,
                                mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0 as libc::c_int);
    (*Csegment).flags =
        ((*Csegment).flags as libc::c_int | 0x20 as libc::c_int) as
            libc::c_uchar;
    if (*sym).addrmode as libc::c_int != AM_IMP as libc::c_int {
        (*Csegment).rorg = (*sym).value as libc::c_ulong;
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            (*Csegment).rflags =
                ((*Csegment).rflags as libc::c_int | 0x1 as libc::c_int) as
                    libc::c_uchar
        } else {
            (*Csegment).rflags =
                ((*Csegment).rflags as libc::c_int & !(0x1 as libc::c_int)) as
                    libc::c_uchar
        }
        if (*Csegment).initrflags as libc::c_int & 0x1 as libc::c_int != 0 {
            (*Csegment).initrorg = (*sym).value as libc::c_ulong;
            (*Csegment).initrflags = (*sym).flags
        }
    }
    programlabel();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_rend(mut str: *mut libc::c_char,
                                mut dummy: *mut _MNE) {
    programlabel();
    (*Csegment).flags =
        ((*Csegment).flags as libc::c_int & !(0x20 as libc::c_int)) as
            libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn v_align(mut str: *mut libc::c_char,
                                 mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0 as libc::c_int);
    let mut fill: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut rorg: libc::c_uchar =
        ((*Csegment).flags as libc::c_int & 0x20 as libc::c_int) as
            libc::c_uchar;
    if rorg != 0 {
        (*Csegment).rflags =
            ((*Csegment).rflags as libc::c_int | 0x4 as libc::c_int) as
                libc::c_uchar
    } else {
        (*Csegment).flags =
            ((*Csegment).flags as libc::c_int | 0x4 as libc::c_int) as
                libc::c_uchar
    }
    if !(*sym).next.is_null() {
        if (*(*sym).next).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |=
                REASON_ALIGN_NOT_RESOLVED as libc::c_int as libc::c_ulong
        } else { fill = (*(*sym).next).value as libc::c_uchar }
    }
    if rorg != 0 {
        if ((*Csegment).rflags as libc::c_int | (*sym).flags as libc::c_int) &
               0x1 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |=
                REASON_ALIGN_RELOCATABLE_ORIGIN_NOT_KNOWN as libc::c_int as
                    libc::c_ulong
        } else {
            let mut n: libc::c_long =
                ((*sym).value as
                     libc::c_ulong).wrapping_sub((*Csegment).rorg.wrapping_rem((*sym).value
                                                                                   as
                                                                                   libc::c_ulong))
                    as libc::c_long;
            if n != (*sym).value {
                genfill(fill as libc::c_long, n, 1 as libc::c_int);
            }
        }
    } else if ((*Csegment).flags as libc::c_int | (*sym).flags as libc::c_int)
                  & 0x1 as libc::c_int != 0 {
        Redo += 1;
        Redo_why |=
            REASON_ALIGN_NORMAL_ORIGIN_NOT_KNOWN as libc::c_int as
                libc::c_ulong
    } else {
        let mut n_0: libc::c_long =
            ((*sym).value as
                 libc::c_ulong).wrapping_sub((*Csegment).org.wrapping_rem((*sym).value
                                                                              as
                                                                              libc::c_ulong))
                as libc::c_long;
        if n_0 != (*sym).value {
            genfill(fill as libc::c_long, n_0, 1 as libc::c_int);
        }
    }
    FreeSymbolList(sym);
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_subroutine(mut str: *mut libc::c_char,
                                      mut dummy: *mut _MNE) {
    Lastlocalindex = Lastlocalindex.wrapping_add(1);
    Localindex = Lastlocalindex;
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_equ(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0 as libc::c_int);
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    /*
    * If we encounter a line of the form
    *   . = expr	; or . EQU expr
    * treat it as one of
    *     org expr
    *     rorg expr
    * depending on whether we have a relocatable origin now or not.
    */
    if strlen(*Av.as_mut_ptr().offset(0 as libc::c_int as isize)) ==
           1 as libc::c_int as libc::c_ulong &&
           (*(*Av.as_mut_ptr().offset(0 as libc::c_int as
                                          isize)).offset(0 as libc::c_int as
                                                             isize) as
                libc::c_int == '.' as i32 ||
                *(*Av.as_mut_ptr().offset(0 as libc::c_int as
                                              isize)).offset(0 as libc::c_int
                                                                 as isize) as
                    libc::c_int == '*' as i32 &&
                    {
                        let ref mut fresh32 =
                            *(*Av.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize)).offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize);
                        *fresh32 = '.' as i32 as libc::c_char;
                        (*fresh32 as libc::c_int) != 0
                    } && 1 as libc::c_int != 0) {
        /* Av[0][0] = '\0'; */
        if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            v_rorg(str, dummy);
        } else { v_org(str, dummy); }
        return
    }
    lab =
        findsymbol(*Av.as_mut_ptr().offset(0 as libc::c_int as isize),
                   strlen(*Av.as_mut_ptr().offset(0 as libc::c_int as isize))
                       as libc::c_int);
    if lab.is_null() {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as libc::c_int as isize),
                         strlen(*Av.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize)) as
                             libc::c_int)
    }
    if (*lab).flags as libc::c_int & 0x1 as libc::c_int == 0 {
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            Redo += 1;
            Redo_why |=
                REASON_EQU_NOT_RESOLVED as libc::c_int as libc::c_ulong
        } else if (*lab).value != (*sym).value {
            asmerr(ERROR_EQU_VALUE_MISMATCH as libc::c_int,
                   0 as libc::c_int != 0, 0 as *const libc::c_char);
            printf(b"INFO: Label \'%s\' changed from $%04lx to $%04lx\n\x00"
                       as *const u8 as *const libc::c_char,
                   *Av.as_mut_ptr().offset(0 as libc::c_int as isize),
                   (*lab).value, (*sym).value);
            Redo += 1;
            Redo_why |=
                REASON_EQU_VALUE_MISMATCH as libc::c_int as libc::c_ulong
        }
    }
    (*lab).value = (*sym).value;
    (*lab).flags =
        ((*sym).flags as libc::c_int &
             (0x1 as libc::c_int | 0x8 as libc::c_int)) as libc::c_uchar;
    (*lab).string = (*sym).string;
    (*sym).flags =
        ((*sym).flags as libc::c_int &
             !(0x8 as libc::c_int | 0x20 as libc::c_int)) as libc::c_uchar;
    /* List the value */
    let mut v: libc::c_ulong = (*lab).value as libc::c_ulong;
    Glen = 0 as libc::c_int;
    if v > 0xffff as libc::c_int as libc::c_ulong {
        let fresh33 = Glen;
        Glen = Glen + 1;
        Gen[fresh33 as usize] = (v >> 24 as libc::c_int) as libc::c_uchar;
        let fresh34 = Glen;
        Glen = Glen + 1;
        Gen[fresh34 as usize] = (v >> 16 as libc::c_int) as libc::c_uchar
    }
    let fresh35 = Glen;
    Glen = Glen + 1;
    Gen[fresh35 as usize] = (v >> 8 as libc::c_int) as libc::c_uchar;
    let fresh36 = Glen;
    Glen = Glen + 1;
    Gen[fresh36 as usize] = v as libc::c_uchar;
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_eqm(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut len: libc::c_int =
        strlen(*Av.as_mut_ptr().offset(0 as libc::c_int as isize)) as
            libc::c_int;
    lab = findsymbol(*Av.as_mut_ptr().offset(0 as libc::c_int as isize), len);
    if !lab.is_null() {
        if (*lab).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            free((*lab).string as *mut libc::c_void);
        }
    } else {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as libc::c_int as isize),
                         len)
    }
    (*lab).value = 0 as libc::c_int as libc::c_long;
    (*lab).flags =
        (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int) as
            libc::c_uchar;
    (*lab).string =
        strcpy(ckmalloc(strlen(str).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong) as
                            libc::c_int), str);
}
#[no_mangle]
pub unsafe extern "C" fn v_echo(mut str: *mut libc::c_char,
                                mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0 as libc::c_int);
    let mut s: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut buf: [libc::c_char; 256] = [0; 256];
    s = sym;
    while !s.is_null() {
        if (*s).flags as libc::c_int & 0x1 as libc::c_int == 0 {
            if (*s).flags as libc::c_int &
                   (0x20 as libc::c_int | 0x8 as libc::c_int) != 0 {
                sprintf(buf.as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*s).string);
            } else {
                sprintf(buf.as_mut_ptr(),
                        b"$%lx\x00" as *const u8 as *const libc::c_char,
                        (*s).value);
            }
            if !FI_listfile.is_null() {
                fprintf(FI_listfile,
                        b" %s\x00" as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr());
            }
            //printf(" %s", buf);
            addmsg(b" \x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char); // -FXQ supress output until final pass
            addmsg(buf.as_mut_ptr());
        }
        s = (*s).next
    }
    //puts("");
    addmsg(b"\n\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
    if !FI_listfile.is_null() { putc('\n' as i32, FI_listfile); };
}
#[no_mangle]
pub unsafe extern "C" fn v_set(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut dynamicname: [libc::c_char; 257] = [0; 257];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut setundefined: libc::c_int = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0 &&
              *str.offset(i as isize) as libc::c_int != '\"' as i32 &&
              *str.offset(i as isize) as libc::c_int != ' ' as i32 &&
              *str.offset(i as isize) as libc::c_int != ',' as i32 {
        i += 1
    }
    if *str.offset(i as isize) as libc::c_int != 0 &&
           *str.offset(i as isize) as libc::c_int == ',' as i32 {
        // is this SET is using the "," eval-concat operator?
        strncpy(dynamicname.as_mut_ptr(), str,
                256 as libc::c_int as libc::c_ulong);
        if i < 256 as libc::c_int {
            dynamicname[i as usize] = 0 as libc::c_int as libc::c_char
        }
        dynamicname[256 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        j = strlen(dynamicname.as_mut_ptr()) as libc::c_int;
        // eval-concat argument processing loop...
        while *str.offset(i as isize) as libc::c_int != 0 &&
                  *str.offset(i as isize) as libc::c_int != '\"' as i32 &&
                  *str.offset(i as isize) as libc::c_int != ' ' as i32 {
            if *str.offset(i as isize) as libc::c_int == 0 as libc::c_int ||
                   *str.offset(i as isize) as libc::c_int == ' ' as i32 {
                break ;
                // process any remaining arguments
            } // argument was symbol
            if *str.offset((i + 1 as libc::c_int) as isize) as libc::c_int ==
                   '\"' as i32 {
                // is this a string constant?
                i = i + 2 as libc::c_int; // argument was string constant
                while *str.offset(i as isize) as libc::c_int != 0 &&
                          *str.offset(i as isize) as libc::c_int !=
                              '\"' as i32 &&
                          *str.offset(i as isize) as libc::c_int != ' ' as i32
                          &&
                          *str.offset(i as isize) as libc::c_int != ',' as i32
                      {
                    let fresh37 = i; // advance to string contents
                    i = i + 1;
                    let fresh38 = j;
                    j = j + 1;
                    dynamicname[fresh38 as usize] =
                        *str.offset(fresh37 as isize)
                }
                if *str.offset(i as isize) as libc::c_int != 0 &&
                       *str.offset(i as isize) as libc::c_int == '\"' as i32 {
                    dynamicname[j as usize] =
                        0 as libc::c_int as libc::c_char;
                    i += 1
                } else {
                    asmerr(ERROR_SYNTAX_ERROR as libc::c_int,
                           0 as libc::c_int != 0, str);
                }
            } else {
                // this argument is a symbol to be evaluated
                let mut t: libc::c_int = 0;
                let mut tempbuf: [libc::c_char; 257] = [0; 257];
                let mut tempval: [libc::c_char; 257] = [0; 257];
                let mut symarg: *mut _SYMBOL = 0 as *mut _SYMBOL;
                strncpy(tempbuf.as_mut_ptr(),
                        str.offset(i as
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
                        setundefined += 1
                    } else {
                        snprintf(tempval.as_mut_ptr(),
                                 256 as libc::c_int as libc::c_ulong,
                                 b"%d\x00" as *const u8 as
                                     *const libc::c_char,
                                 (*symarg).value as
                                     libc::c_uint); // ensure the set doesn't actually happen
                        strcpy(dynamicname.as_mut_ptr().offset(j as isize),
                               tempval.as_mut_ptr());
                        j =
                            (j as
                                 libc::c_ulong).wrapping_add(strlen(tempval.as_mut_ptr()))
                                as libc::c_int
                    }
                }
                i += 1;
                while *str.offset(i as isize) as libc::c_int != 0 &&
                          *str.offset(i as isize) as libc::c_int != ' ' as i32
                          &&
                          *str.offset(i as isize) as libc::c_int != ',' as i32
                      {
                    i += 1
                }
            }
        }
        let fresh39 = i;
        i = i + 1;
        dynamicname[fresh39 as usize] = 0 as libc::c_int as libc::c_char;
        if setundefined != 0 {
            // not all of the arguments are defined yet, so skip this SET
            return
        }
        sym = eval(dynamicname.as_mut_ptr(), 0 as libc::c_int)
    } else {
        // traditional SET behavior
        sym = eval(str, 0 as libc::c_int)
    } /* garbage */
    lab =
        findsymbol(*Av.as_mut_ptr().offset(0 as libc::c_int as isize),
                   strlen(*Av.as_mut_ptr().offset(0 as libc::c_int as isize))
                       as libc::c_int);
    if lab.is_null() {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as libc::c_int as isize),
                         strlen(*Av.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize)) as
                             libc::c_int)
    }
    (*lab).value = (*sym).value;
    (*lab).flags =
        ((*sym).flags as libc::c_int &
             (0x1 as libc::c_int | 0x8 as libc::c_int)) as libc::c_uchar;
    (*lab).string = (*sym).string;
    (*sym).flags =
        ((*sym).flags as libc::c_int &
             !(0x8 as libc::c_int | 0x20 as libc::c_int)) as libc::c_uchar;
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_setstr(mut symstr: *mut libc::c_char,
                                  mut dummy: *mut _MNE) {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    snprintf(str.as_mut_ptr(), 1024 as libc::c_int as libc::c_ulong,
             b"\"%s\"\x00" as *const u8 as *const libc::c_char, symstr);
    v_set(str.as_mut_ptr(), dummy);
}
#[no_mangle]
pub unsafe extern "C" fn v_execmac(mut str: *mut libc::c_char,
                                   mut mac: *mut _MACRO) {
    let mut inc: *mut _INCFILE = 0 as *mut _INCFILE;
    let mut base: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut psl: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut sl: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    programlabel();
    if Mlevel == 32 as libc::c_int as libc::c_uint {
        puts(b"infinite macro recursion\x00" as *const u8 as
                 *const libc::c_char);
        return
    }
    Mlevel = Mlevel.wrapping_add(1);
    base =
        ckmalloc((::std::mem::size_of::<*mut _STRLIST>() as
                      libc::c_ulong).wrapping_add(strlen(str)).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                     as libc::c_int) as *mut _STRLIST;
    (*base).next = 0 as *mut _STRLIST;
    strcpy((*base).buf.as_mut_ptr(), str);
    psl = &mut (*base).next;
    while *str as libc::c_int != 0 && *str as libc::c_int != '\n' as i32 {
        s1 = str;
        while *str as libc::c_int != 0 && *str as libc::c_int != '\n' as i32
                  && *str as libc::c_int != ',' as i32 {
            str = str.offset(1)
        }
        sl =
            ckmalloc((::std::mem::size_of::<*mut _STRLIST>() as
                          libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                          libc::c_ulong)
                         as libc::c_int) as *mut _STRLIST;
        // Conversion note: in the above line, removed additional wrapping data...
        //     .wrapping_add(str.wrapping_offset_from(s1) as libc::c_long as libc::c_ulong)
        // ...because it was relying on allocating more memory than the buffer needed, THEN
        // overrunning the buffer to set it. Instead, the fix just increased the buffer length
        // from 4 to 16 to be sure.
        (*sl).next = 0 as *mut _STRLIST;
        *psl = sl;
        psl = &mut (*sl).next;
        memcpy((*sl).buf.as_mut_ptr() as *mut libc::c_void,
               s1 as *const libc::c_void,
               str.wrapping_offset_from(s1) as libc::c_long as libc::c_ulong);
        // Conversion note: this is the line that was causing a buffer overrun during tests,
        // as the code was trying to read up to 9 (and it's size 4)
        (*sl).buf[str.wrapping_offset_from(s1) as libc::c_long as usize] =
            0 as libc::c_int as libc::c_char;
        if *str as libc::c_int == ',' as i32 { str = str.offset(1) }
        while *str as libc::c_int == ' ' as i32 { str = str.offset(1) }
    }
    inc =
        zmalloc(::std::mem::size_of::<_INCFILE>() as libc::c_ulong as
                    libc::c_int) as *mut _INCFILE;
    (*inc).next = pIncfile;
    (*inc).name = (*mac).name;
    (*inc).fi = (*pIncfile).fi;
    (*inc).lineno = 0 as libc::c_int as libc::c_ulong;
    (*inc).flags = 0x1 as libc::c_int as libc::c_uchar;
    (*inc).saveidx = Localindex;
    (*inc).savedolidx = Localdollarindex;
    (*inc).strlist = (*mac).strlist;
    (*inc).args = base;
    pIncfile = inc;
    Lastlocalindex = Lastlocalindex.wrapping_add(1);
    Localindex = Lastlocalindex;
    Lastlocaldollarindex = Lastlocaldollarindex.wrapping_add(1);
    Localdollarindex = Lastlocaldollarindex;
}
#[no_mangle]
pub unsafe extern "C" fn v_end(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    /* Only ENDs current file and any macro calls within it */
    while (*pIncfile).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        v_endm(0 as *mut libc::c_char, 0 as *mut _MNE);
    }
    fseek((*pIncfile).fi, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn v_endm(mut str: *mut libc::c_char,
                                mut dummy: *mut _MNE) {
    let mut inc: *mut _INCFILE = pIncfile;
    let mut args: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut an: *mut _STRLIST = 0 as *mut _STRLIST;
    /* programlabel(); contrary to documentation */
    if (*inc).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        Mlevel = Mlevel.wrapping_sub(1);
        args = (*inc).args;
        while !args.is_null() {
            an = (*args).next;
            free(args as *mut libc::c_void);
            args = an
        }
        Localindex = (*inc).saveidx;
        Localdollarindex = (*inc).savedolidx;
        pIncfile = (*inc).next;
        free(inc as *mut libc::c_void);
        return
    }
    puts(b"not within a macro\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn v_mexit(mut str: *mut libc::c_char,
                                 mut dummy: *mut _MNE) {
    v_endm(0 as *mut libc::c_char, 0 as *mut _MNE);
}
#[no_mangle]
pub unsafe extern "C" fn v_ifconst(mut str: *mut libc::c_char,
                                   mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    sym = eval(str, 0 as libc::c_int);
    pushif((*sym).flags as libc::c_int == 0 as libc::c_int);
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_ifnconst(mut str: *mut libc::c_char,
                                    mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    sym = eval(str, 0 as libc::c_int);
    pushif((*sym).flags as libc::c_int != 0 as libc::c_int);
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_if(mut str: *mut libc::c_char,
                              mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        pushif(0 as libc::c_int != 0);
        return
    }
    programlabel();
    sym = eval(str, 0 as libc::c_int);
    if (*sym).flags != 0 {
        Redo += 1;
        Redo_why |= REASON_IF_NOT_RESOLVED as libc::c_int as libc::c_ulong;
        pushif(0 as libc::c_int != 0);
        (*Ifstack).acctrue = 0 as libc::c_int as libc::c_uchar;
        Redo_if |= 1 as libc::c_int as libc::c_ulong
    } else { pushif((*sym).value != 0); }
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_else(mut str: *mut libc::c_char,
                                mut dummy: *mut _MNE) {
    if (*Ifstack).acctrue as libc::c_int != 0 &&
           (*Ifstack).flags as libc::c_int & 0x4 as libc::c_int == 0 {
        programlabel();
        (*Ifstack).xtrue =
            ((*Ifstack).xtrue == 0) as libc::c_int as libc::c_uchar
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_endif(mut str: *mut libc::c_char,
                                 mut dummy: *mut _MNE) {
    let mut ifs: *mut _IFSTACK = Ifstack;
    if (*ifs).flags as libc::c_int & 0x4 as libc::c_int == 0 {
        if (*ifs).acctrue != 0 { programlabel(); }
        if (*ifs).file != pIncfile {
            puts(b"too many endif\'s\x00" as *const u8 as
                     *const libc::c_char);
        } else { Ifstack = (*ifs).next; free(ifs as *mut libc::c_void); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_repeat(mut str: *mut libc::c_char,
                                  mut dummy: *mut _MNE) {
    let mut rp: *mut _REPLOOP = 0 as *mut _REPLOOP;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        pushif(0 as libc::c_int != 0);
        return
    }
    programlabel();
    sym = eval(str, 0 as libc::c_int);
    if (*sym).value == 0 as libc::c_int as libc::c_long {
        pushif(0 as libc::c_int != 0);
        FreeSymbolList(sym);
        return
    }
    /* Don't allow negative values for REPEAT loops */
    if (*sym).value < 0 as libc::c_int as libc::c_long {
        pushif(0 as libc::c_int != 0);
        FreeSymbolList(sym);
        asmerr(ERROR_REPEAT_NEGATIVE as libc::c_int, 0 as libc::c_int != 0,
               0 as *const libc::c_char);
        return
    }
    rp =
        zmalloc(::std::mem::size_of::<_REPLOOP>() as libc::c_ulong as
                    libc::c_int) as *mut _REPLOOP;
    (*rp).next = Reploop;
    (*rp).file = pIncfile;
    if (*pIncfile).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        (*rp).seek = (*pIncfile).strlist as libc::c_long as libc::c_ulong
    } else { (*rp).seek = ftell((*pIncfile).fi) as libc::c_ulong }
    (*rp).lineno = (*pIncfile).lineno;
    (*rp).count = (*sym).value as libc::c_ulong;
    (*rp).flags = (*sym).flags;
    if (*rp).flags as libc::c_int != 0 as libc::c_int {
        Redo += 1;
        Redo_why |= REASON_REPEAT_NOT_RESOLVED as libc::c_int as libc::c_ulong
    }
    Reploop = rp;
    FreeSymbolList(sym);
    pushif(1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn v_repend(mut str: *mut libc::c_char,
                                  mut dummy: *mut _MNE) {
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        v_endif(0 as *mut libc::c_char, 0 as *mut _MNE);
        return
    }
    if !Reploop.is_null() && (*Reploop).file == pIncfile {
        if (*Reploop).flags as libc::c_int == 0 as libc::c_int &&
               {
                   (*Reploop).count = (*Reploop).count.wrapping_sub(1);
                   ((*Reploop).count) != 0
               } {
            if (*pIncfile).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                (*pIncfile).strlist = (*Reploop).seek as *mut _STRLIST
            } else {
                fseek((*pIncfile).fi, (*Reploop).seek as libc::c_long,
                      0 as libc::c_int);
            }
            (*pIncfile).lineno = (*Reploop).lineno
        } else {
            rmnode(&mut Reploop as *mut *mut _REPLOOP as
                       *mut *mut libc::c_void,
                   ::std::mem::size_of::<_REPLOOP>() as libc::c_ulong as
                       libc::c_int);
            v_endif(0 as *mut libc::c_char, 0 as *mut _MNE);
        }
        return
    }
    puts(b"no repeat\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub static mut incdirlist: *mut _STRLIST =
    0 as *const _STRLIST as *mut _STRLIST;
#[no_mangle]
pub unsafe extern "C" fn v_incdir(mut str: *mut libc::c_char,
                                  mut dummy: *mut _MNE) {
    let mut tail: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: libc::c_int = 0 as libc::c_int;
    buf = getfilename(str);
    tail = &mut incdirlist;
    while !(*tail).is_null() {
        if strcmp((**tail).buf.as_mut_ptr(), buf) == 0 as libc::c_int {
            found = 1 as libc::c_int
        }
        tail = &mut (**tail).next
    }
    if found == 0 {
        let mut newdir: *mut _STRLIST = 0 as *mut _STRLIST;
        newdir =
            permalloc((::std::mem::size_of::<*mut _STRLIST>() as
                           libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong).wrapping_add(strlen(buf))
                          as libc::c_int) as *mut _STRLIST;
        strcpy((*newdir).buf.as_mut_ptr(), buf);
        *tail = newdir
    }
    if buf != str { free(buf as *mut libc::c_void); };
}
unsafe extern "C" fn addpart(mut dest: *mut libc::c_char,
                             mut dir: *const libc::c_char,
                             mut file: *const libc::c_char) {
    /* not needed here */
    let mut pos: libc::c_int = 0;
    strcpy(dest, dir);
    pos = strlen(dest) as libc::c_int;
    if pos > 0 as libc::c_int &&
           *dest.offset((pos - 1 as libc::c_int) as isize) as libc::c_int !=
               ':' as i32 &&
           *dest.offset((pos - 1 as libc::c_int) as isize) as libc::c_int !=
               '/' as i32 {
        *dest.offset(pos as isize) = '/' as i32 as libc::c_char;
        pos += 1
    }
    strcpy(dest.offset(pos as isize), file);
}
#[no_mangle]
pub unsafe extern "C" fn pfopen(mut name: *const libc::c_char,
                                mut mode: *const libc::c_char) -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut incdir: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    f = fopen(name, mode);
    if !f.is_null() { return f }
    /* Don't use the incdirlist for absolute pathnames */
    if !strchr(name, ':' as i32).is_null() {
        return 0 as *mut FILE
    } /*	multiplied later    */
    buf = zmalloc(512 as libc::c_int);
    incdir = incdirlist;
    while !incdir.is_null() {
        addpart(buf, (*incdir).buf.as_mut_ptr(), name);
        f = fopen(buf, mode);
        if !f.is_null() { break ; }
        incdir = (*incdir).next
    }
    free(buf as *mut libc::c_void);
    return f;
}
static mut Seglen: libc::c_long = 0;
static mut Seekback: libc::c_long = 0;
#[no_mangle]
pub unsafe extern "C" fn generate() {
    let mut seekpos: libc::c_long = 0;
    static mut org: libc::c_ulong = 0;
    let mut i: libc::c_int = 0;
    if Redo == 0 {
        if (*Csegment).flags as libc::c_int & 0x10 as libc::c_int == 0 {
            i = Glen - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                CheckSum =
                    CheckSum.wrapping_add(Gen[i as usize] as libc::c_ulong);
                i -= 1
            }
            if Fisclear != 0 {
                Fisclear = 0 as libc::c_int as libc::c_uchar;
                if (*Csegment).flags as libc::c_int & 0x1 as libc::c_int != 0
                   {
                    Redo += 1;
                    Redo_why |=
                        REASON_OBSCURE as libc::c_int as libc::c_ulong;
                    return
                }
                org = (*Csegment).org;
                if F_format < FORMAT_RAW as libc::c_int {
                    putc((org & 0xff as libc::c_int as libc::c_ulong) as
                             libc::c_int, FI_temp);
                    putc((org >> 8 as libc::c_int &
                              0xff as libc::c_int as libc::c_ulong) as
                             libc::c_int, FI_temp);
                    if F_format == FORMAT_RAS as libc::c_int {
                        Seekback = ftell(FI_temp);
                        Seglen = 0 as libc::c_int as libc::c_long;
                        putc(0 as libc::c_int, FI_temp);
                        putc(0 as libc::c_int, FI_temp);
                    }
                }
            }
            match F_format {
                3 | 1 => {
                    if (*Csegment).org < org {
                        printf(b"segment: %s %s  vs current org: %04lx\n\x00"
                                   as *const u8 as *const libc::c_char,
                               (*Csegment).name,
                               sftos((*Csegment).org as libc::c_long,
                                     (*Csegment).flags as libc::c_int), org);
                        asmerr(ERROR_ORIGIN_REVERSE_INDEXED as libc::c_int,
                               1 as libc::c_int != 0,
                               0 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    while (*Csegment).org != org {
                        putc(OrgFill as libc::c_int, FI_temp);
                        org = org.wrapping_add(1)
                    }
                    fwrite(Gen.as_mut_ptr() as *const libc::c_void,
                           Glen as size_t, 1 as libc::c_int as size_t,
                           FI_temp);
                }
                2 => {
                    if org != (*Csegment).org {
                        org = (*Csegment).org;
                        seekpos = ftell(FI_temp);
                        fseek(FI_temp, Seekback, 0 as libc::c_int);
                        putc((Seglen & 0xff as libc::c_int as libc::c_long) as
                                 libc::c_int, FI_temp);
                        putc((Seglen >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_long) as
                                 libc::c_int, FI_temp);
                        fseek(FI_temp, seekpos, 0 as libc::c_int);
                        putc((org & 0xff as libc::c_int as libc::c_ulong) as
                                 libc::c_int, FI_temp);
                        putc((org >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_ulong) as
                                 libc::c_int, FI_temp);
                        Seekback = ftell(FI_temp);
                        Seglen = 0 as libc::c_int as libc::c_long;
                        putc(0 as libc::c_int, FI_temp);
                        putc(0 as libc::c_int, FI_temp);
                    }
                    fwrite(Gen.as_mut_ptr() as *const libc::c_void,
                           Glen as size_t, 1 as libc::c_int as size_t,
                           FI_temp);
                    Seglen += Glen as libc::c_long
                }
                _ => {
                    asmerr(ERROR_BAD_FORMAT as libc::c_int,
                           1 as libc::c_int != 0,
                           b"Unhandled internal format specifier\x00" as
                               *const u8 as *const libc::c_char);
                }
            }
            org = org.wrapping_add(Glen as libc::c_ulong)
        }
    }
    (*Csegment).org = (*Csegment).org.wrapping_add(Glen as libc::c_ulong);
    if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        (*Csegment).rorg =
            (*Csegment).rorg.wrapping_add(Glen as libc::c_ulong)
    };
}
#[no_mangle]
pub unsafe extern "C" fn closegenerate() {
    if Redo == 0 {
        if F_format == FORMAT_RAS as libc::c_int {
            fseek(FI_temp, Seekback, 0 as libc::c_int);
            putc((Seglen & 0xff as libc::c_int as libc::c_long) as
                     libc::c_int, FI_temp);
            putc((Seglen >> 8 as libc::c_int &
                      0xff as libc::c_int as libc::c_long) as libc::c_int,
                 FI_temp);
            fseek(FI_temp, 0 as libc::c_long, 2 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn genfill(mut fill: libc::c_long,
                                 mut entries: libc::c_long,
                                 mut size: libc::c_int) {
    let mut bytes: libc::c_long = entries;
    let mut i: libc::c_int = 0;
    let mut c3: libc::c_uchar = 0;
    let mut c2: libc::c_uchar = 0;
    let mut c1: libc::c_uchar = 0;
    let mut c0: libc::c_uchar = 0;
    if bytes == 0 { return }
    c3 = (fill >> 24 as libc::c_int) as libc::c_uchar;
    c2 = (fill >> 16 as libc::c_int) as libc::c_uchar;
    c1 = (fill >> 8 as libc::c_int) as libc::c_uchar;
    c0 = fill as libc::c_uchar;
    match size {
        1 => {
            memset(Gen.as_mut_ptr() as *mut libc::c_void, c0 as libc::c_int,
                   ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                       libc::c_ulong);
        }
        2 => {
            bytes <<= 1 as libc::c_int;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                          libc::c_ulong {
                if MsbOrder != 0 {
                    Gen[(i + 0 as libc::c_int) as usize] = c1;
                    Gen[(i + 1 as libc::c_int) as usize] = c0
                } else {
                    Gen[(i + 0 as libc::c_int) as usize] = c0;
                    Gen[(i + 1 as libc::c_int) as usize] = c1
                }
                i += 2 as libc::c_int
            }
        }
        4 => {
            bytes <<= 2 as libc::c_int;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                          libc::c_ulong {
                if MsbOrder != 0 {
                    Gen[(i + 0 as libc::c_int) as usize] = c3;
                    Gen[(i + 1 as libc::c_int) as usize] = c2;
                    Gen[(i + 2 as libc::c_int) as usize] = c1;
                    Gen[(i + 3 as libc::c_int) as usize] = c0
                } else {
                    Gen[(i + 0 as libc::c_int) as usize] = c0;
                    Gen[(i + 1 as libc::c_int) as usize] = c1;
                    Gen[(i + 2 as libc::c_int) as usize] = c2;
                    Gen[(i + 3 as libc::c_int) as usize] = c3
                }
                i += 4 as libc::c_int
            }
        }
        _ => { }
    }
    Glen =
        ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong as
            libc::c_int;
    while bytes as libc::c_ulong >
              ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong
          {
        generate();
        bytes =
            (bytes as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                 as libc::c_ulong) as
                libc::c_long as libc::c_long
    }
    Glen = bytes as libc::c_int;
    generate();
}
#[no_mangle]
pub unsafe extern "C" fn pushif(mut xbool: bool) {
    let mut ifs: *mut _IFSTACK =
        zmalloc(::std::mem::size_of::<_IFSTACK>() as libc::c_ulong as
                    libc::c_int) as *mut _IFSTACK;
    (*ifs).next = Ifstack;
    (*ifs).file = pIncfile;
    (*ifs).flags = 0 as libc::c_int as libc::c_uchar;
    (*ifs).xtrue = xbool as libc::c_uchar;
    (*ifs).acctrue =
        ((*Ifstack).acctrue as libc::c_int != 0 &&
             (*Ifstack).xtrue as libc::c_int != 0) as libc::c_int as
            libc::c_uchar;
    Ifstack = ifs;
}
