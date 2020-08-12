use libc;
use std::cmp::Ordering;

use crate::constants::{
    MAX_MACRO_LEVEL,
};
use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SegmentTypes,
    SymbolTypes,
};
use crate::types::enums::{
    AddressModes,
    AsmErrorEquates,
    BitOrder,
    Format,
    ListMode,
    Processors,
};
use crate::types::structs::{
    Segment,
};
use crate::utils::{
    filesystem,
    formatting,
    get_filename,
    transient,
};

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    static mut pIncfile: *mut _INCFILE;
    #[no_mangle]
    static mut Reploop: *mut _REPLOOP;
    #[no_mangle]
    static mut Ifstack: *mut _IFSTACK;
    #[no_mangle]
    static mut Av: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut Cvt: [libc::c_uint; 0];
    #[no_mangle]
    static mut Opsize: [libc::c_uint; 0];
    #[no_mangle]
    static mut Mlevel: libc::c_uint;
    #[no_mangle]
    static mut CheckSum: libc::c_ulong;
    #[no_mangle]
    fn findext(str: *mut libc::c_char);
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
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
    static mut FI_temp: *mut FILE;
    #[no_mangle]
    static mut Lastlocaldollarindex: libc::c_ulong;
    #[no_mangle]
    fn programlabel();
    #[no_mangle]
    static mut Localdollarindex: libc::c_ulong;
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
/*
*  An opcode modifies the SEGMENT flags in the following ways:
*/
#[no_mangle]
pub unsafe extern "C" fn v_processor(mut str: *mut libc::c_char,
                                     mut _dummy: *mut _MNE) {
    // FIXME: this is a bit dumb and shouldn't be needed. Check if a processor exists only.
    static mut called: bool = false;
    let mut previousProcessor: Processors = state.execution.processor;
    state.execution.processor = Processors::None;
    let processorName = transient::str_pointer_to_string(str);
    match processorName.as_str() {
        "6502" => {
            if !called {
                addhashtable(Mne6502.as_mut_ptr());
            }
            state.execution.bitOrder = BitOrder::LeastMost;
            state.execution.processor = Processors::MOS_6502;
        },
        "6803" => {
            if !called {
                addhashtable(Mne6803.as_mut_ptr());
            }
            state.execution.bitOrder = BitOrder::MostLeast;
            state.execution.processor = Processors::MOTOROLA_6803;
        },
        "HD6303" | "hd6303" => {
            if !called {
                addhashtable(Mne6803.as_mut_ptr());
                addhashtable(MneHD6303.as_mut_ptr());
            }
            state.execution.bitOrder = BitOrder::MostLeast;
            state.execution.processor = Processors::HD_6303;
        },
        "68705" => {
            if !called {
                addhashtable(Mne68705.as_mut_ptr());
            }
            state.execution.bitOrder = BitOrder::MostLeast;
            state.execution.processor = Processors::MOTOROLA_68705;
        },
        "68HC11" | "68hc11" => {
            if !called {
                addhashtable(Mne68HC11.as_mut_ptr());
            }
            state.execution.bitOrder = BitOrder::MostLeast;
            state.execution.processor = Processors::MOTOROLA_68HC11;
        },
        "F8" | "f8" => {
            if !called {
                addhashtable(MneF8.as_mut_ptr());
            }
            state.execution.bitOrder = BitOrder::MostLeast;
            state.execution.processor = Processors::FAIRCHILD_F8;
        },
        _ => {
            asmerr(AsmErrorEquates::ProcessorNotSupported, true, str);
        }
    }

    called = true;
    if previousProcessor != Processors::None && state.execution.processor != previousProcessor {
        asmerr(AsmErrorEquates::OnlyOneProcessorSupported, true, str);
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_mnemonic(mut str: *mut libc::c_char,
                                    mut mne: *mut _MNE) {
    let mut addrmode: libc::c_int = 0;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut opcode: libc::c_uint = 0;
    let mut opidx: usize = 0;
    let mut symbase: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut opsize: libc::c_int = 0;
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.flags = currentSegment.flags | SegmentTypes::Referenced;
    programlabel();
    symbase = eval(str, 1);
    if state.execution.trace {
        println!(
            "PC: {:04x}  MNEMONIC: {}  addrmode: {}  ",
            currentSegment.org,
            transient::str_pointer_to_string((*mne).name).as_str(),
            (*symbase).addrmode
        );
    }
    sym = symbase;
    while !sym.is_null() {
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved
        }
        sym = (*sym).next
    }
    sym = symbase;
    if (*mne).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        if !(*sym).next.is_null() {
            (*sym).addrmode = AddressModes::BitMod as u8;
            if (*mne).flags as libc::c_int & 0x20 as libc::c_int != 0 &&
                   !(*sym).next.is_null() {
                (*sym).addrmode = AddressModes::BitBraMod as u8
            }
        }
    }
    addrmode = (*sym).addrmode as libc::c_int;
    if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 ||
           (*sym).value >= 0x100 as libc::c_int as libc::c_long {
        opsize = 2
    } else {
        opsize =
            if (*sym).value != 0 {
                1
            } else { 0 }
    }
    while (*mne).okmask & ((1) << addrmode) as libc::c_ulong
              == 0 && *Cvt.as_mut_ptr().offset(addrmode as isize) != 0 {
        addrmode = *Cvt.as_mut_ptr().offset(addrmode as isize) as libc::c_int
    }
    if state.execution.trace {
        printf(b"mnemask: %08lx adrmode: %d  Cvt[am]: %d\n\x00" as *const u8
                   as *const libc::c_char, (*mne).okmask, addrmode,
               *Cvt.as_mut_ptr().offset(addrmode as isize));
    }
    if (*mne).okmask & ((1) << addrmode) as libc::c_ulong == 0
       {
        let mut sBuffer: [libc::c_char; 128] = [0; 128];
        sprintf(sBuffer.as_mut_ptr(),
                b"%s %s\x00" as *const u8 as *const libc::c_char, (*mne).name,
                str);
        asmerr(AsmErrorEquates::IllegalAddressingMode,
               false, sBuffer.as_mut_ptr());
        FreeSymbolList(symbase);
        //FIX
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved;
        return
    }
    if state.execution.modeNext != AddressModes::None {
        /*	Force	*/
        addrmode = state.execution.modeNext as u8 as i32;
        if (*mne).okmask & ((1) << addrmode) as libc::c_ulong
               == 0 {
            asmerr(AsmErrorEquates::IllegalForcedAddressingMode,
                   false, (*mne).name);
            FreeSymbolList(symbase);
            //FIX: Cause assembly to fail when an invalid mode is used for an opcode...
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved;
            return
        }
    }
    if state.execution.trace {
        printf(b"final addrmode = %d\n\x00" as *const u8 as
                   *const libc::c_char, addrmode);
    }
    while opsize as libc::c_uint >
              *Opsize.as_mut_ptr().offset(addrmode as isize) {
        if *Cvt.as_mut_ptr().offset(addrmode as isize) ==
               0 ||
               (*mne).okmask &
                   ((1) <<
                        *Cvt.as_mut_ptr().offset(addrmode as isize)) as
                       libc::c_ulong == 0 {
            let mut sBuffer_0: [libc::c_char; 128] = [0; 128];
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                break ;
            }
            //FIX: for negative operands...
            if addrmode == AddressModes::Imm8 as i32 &&
                   (*sym).value < 0 {
                opsize = 1; /*  to end of instruction   */
                (*sym).value =
                    ((*sym).value & 255 as libc::c_int as libc::c_long) as
                        libc::c_char as libc::c_long;
                break ;
            } else {
                sprintf(sBuffer_0.as_mut_ptr(),
                        b"%s %s\x00" as *const u8 as *const libc::c_char,
                        (*mne).name, str);
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100,
                       false, sBuffer_0.as_mut_ptr());
                break ;
            }
        } else {
            addrmode =
                *Cvt.as_mut_ptr().offset(addrmode as isize) as libc::c_int
        }
    }
    opcode = (*mne).opcode[addrmode as usize];
    opidx = (1 + (opcode > 0xff) as libc::c_int) as usize;
    if opidx == 2 {
        state.output.generated[0] = (opcode >> 8) as u8;
        state.output.generated[1] = opcode as u8;
    } else {
        state.output.generated[0] = opcode as u8;
    }
    match addrmode {
        15 => {
            sym = (*symbase).next;
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*sym).value >= 0x100 as libc::c_int as libc::c_long {
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100,
                       false, 0 as *const libc::c_char);
            }
            let fresh0 = opidx;
            opidx = opidx + 1;
            state.output.generated[fresh0] = (*sym).value as u8;
            if (*symbase).flags as libc::c_int & 0x1 as libc::c_int == 0 {
                if (*symbase).value > 7 {
                    asmerr(AsmErrorEquates::IllegalBitSpecification,
                           false, str);
                } else {
                    state.output.generated[0] = (
                        state.output.generated[0] as libc::c_long +
                        ((*symbase).value << 1)
                    ) as u8
                }
            }
        }
        16 => {
            if (*symbase).flags as libc::c_int & 0x1 as libc::c_int == 0 {
                if (*symbase).value > 7 {
                    asmerr(AsmErrorEquates::IllegalBitSpecification,
                           false, str);
                } else {
                    state.output.generated[0] = (
                        state.output.generated[0] as libc::c_long +
                        ((*symbase).value << 1)
                    ) as u8
                }
            }
            sym = (*symbase).next;
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*sym).value >= 0x100 as libc::c_int as libc::c_long {
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100,
                       false, 0 as *const libc::c_char);
            }
            state.output.generated[opidx] = (*sym).value as u8;
            opidx = opidx + 1;
            sym = (*sym).next
        }
        9 => { }
        _ => {
            if *Opsize.as_mut_ptr().offset(addrmode as isize) > 0 {
                state.output.generated[opidx] = (*sym).value as u8;
                opidx = opidx + 1;
            }
            if *Opsize.as_mut_ptr().offset(addrmode as isize) == 2 {
                if state.execution.bitOrder != BitOrder::LeastMost {
                    state.output.generated[(opidx - 1) as usize] = ((*sym).value >> 8) as u8;
                    state.output.generated[opidx] = (*sym).value as u8;
                    opidx = opidx + 1;
                } else {
                    state.output.generated[opidx] = ((*sym).value >> 8) as u8;
                    opidx = opidx + 1;
                }
            }
            sym = (*sym).next
        }
    }
    if (*mne).flags as libc::c_int & 0x10 as libc::c_int != 0 {
        if !sym.is_null() {
            if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*sym).value >= 0x100 as libc::c_int as libc::c_long {
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100,
                       false, 0 as *const libc::c_char);
            }
            state.output.generated[opidx as usize] = (*sym).value as u8;
            sym = (*sym).next
        } else {
            asmerr(AsmErrorEquates::NotEnoughArgs,
                   true, 0 as *const libc::c_char);
        }
        opidx += 1
    }
    if (*mne).flags as libc::c_int & 0x20 as libc::c_int != 0 ||
           addrmode == AddressModes::Rel as i32 {
        opidx += 1;
        if sym.is_null() {
            asmerr(AsmErrorEquates::NotEnoughArgs,
                   true, 0 as *const libc::c_char);
        } else if (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 {
            let mut pc: u64 = 0;
            let mut pcf: u8 = 0;
            let mut dest: libc::c_long = 0;
            pc = if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
                currentSegment.rorg
            } else {
                currentSegment.org
            };
            pcf = if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
                currentSegment.rflags
            } else {
                currentSegment.flags
            };
            // FIXME: weird comparison, "2" is like a ghost flag
            if pcf & (SegmentTypes::Unknown | 2) == 0 {
                dest = (*sym).value - (pc as i64) - (opidx as i64);
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
                    asmerr(AsmErrorEquates::BranchOutOfRange,
                           false, sBuffer_1.as_mut_ptr());
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::BranchOutOfRange;
                    (*sym).flags =
                        ((*sym).flags as libc::c_int | 0x1 as libc::c_int) as u8;
                    dest = 0
                }
            } else {
                /* Don't bother - we'll take another pass */
                dest = 0
            } /*  Only so outlist() works */
            state.output.generated[(opidx - 1) as usize] = (dest & 0xff) as u8;
        }
    }
    state.output.generatedLength = opidx;
    generate();
    FreeSymbolList(symbase);
}
#[no_mangle]
pub unsafe extern "C" fn v_trace(mut str: *mut libc::c_char,
                                 mut _dummy: *mut _MNE) {
    state.execution.trace =
        *str.offset(1 as isize) as libc::c_int == 'n' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn v_list(mut str: *mut libc::c_char,
                                mut _dummy: *mut _MNE) {
    programlabel();
    state.output.generatedLength = 0;
    if strncmp(str, b"localoff\x00" as *const u8 as *const libc::c_char,
               7) == 0 ||
           strncmp(str, b"LOCALOFF\x00" as *const u8 as *const libc::c_char,
                   7) == 0 {
        (*pIncfile).flags =
            ((*pIncfile).flags as libc::c_int | 0x2 as libc::c_int) as u8;
    } else if strncmp(str, b"localon\x00" as *const u8 as *const libc::c_char,
                      7) == 0
                  ||
                  strncmp(str,
                          b"LOCALON\x00" as *const u8 as *const libc::c_char,
                          7) ==
                      0 {
        (*pIncfile).flags =
            ((*pIncfile).flags as libc::c_int & !(0x2 as libc::c_int)) as u8;
    } else if strncmp(str, b"off\x00" as *const u8 as *const libc::c_char,
                      2) == 0
                  ||
                  strncmp(str, b"OFF\x00" as *const u8 as *const libc::c_char,
                          2) ==
                      0 {
        state.execution.listMode = ListMode::None
    } else {
        state.execution.listMode = ListMode::List
    };
}
unsafe extern "C" fn getfilename(mut str: *mut libc::c_char)
 -> *mut libc::c_char {
    if *str as libc::c_int == '\"' as i32 {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        str = str.offset(1);
        buf =
            ckmalloc(strlen(str).wrapping_add(1 as
                                                  libc::c_ulong) as
                         libc::c_int);
        strcpy(buf, str);
        str = buf;
        while *str as libc::c_int != 0 && *str as libc::c_int != '\"' as i32 {
            str = str.offset(1)
        }
        *str = 0;
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
                                   mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    // only eval the string if it's compliant with symbol naming
    if (*str as libc::c_int) < '0' as i32 || *str as libc::c_int > '9' as i32
       {
        //check could be more comprehensive
        sym = eval(str, 0)
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
                                  mut _dummy: *mut _MNE) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut binfile: *mut FILE = 0 as *mut FILE;
    programlabel();
    buf = getfilename(str);
    binfile = pfopen(buf, b"rb\x00" as *const u8 as *const libc::c_char);
    if !binfile.is_null() {
        if state.execution.redoIndex != 0 {
            /* optimize: don't actually read the file if not needed */
            fseek(binfile, 0,
                  2);
            state.output.generatedLength = ftell(binfile) as usize;
            generate();
            /* does not access state.output.generated[] if Redo is set */
        } else {
            loop  {
                state.output.generatedLength =
                    fread(
                        state.output.generated.as_mut_ptr() as *mut libc::c_void,
                        1,
                        ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong,
                        binfile
                    ) as usize;
                if state.output.generatedLength <= 0 {
                    break;
                }
                generate();
            }
        }
        fclose(binfile);
    } else {
        printf(b"unable to open %s\n\x00" as *const u8 as *const libc::c_char,
               buf);
    }
    if buf != str { free(buf as *mut libc::c_void); }
    state.output.generatedLength = 0;
    /* don't list hexdump */
}
#[no_mangle]
pub unsafe extern "C" fn v_seg(mut str: *mut libc::c_char, mut _dummy: *mut _MNE) {
    let strNew = transient::str_pointer_to_string(str);
    for (index, seg) in &mut state.other.segments.iter().enumerate() {
        if strNew.eq(&seg.name) {
            state.other.currentSegment = index;
            programlabel();
            return;
        }
    }
    let mut seg = Segment {
        name: strNew,
        flags: SegmentTypes::Unknown,
        rflags: SegmentTypes::Unknown,
        initflags: SegmentTypes::Unknown,
        initrflags: SegmentTypes::Unknown,
        org: 0,
        rorg: 0,
        initorg: 0,
        initrorg: 0,
    };
    if state.execution.modeNext == AddressModes::BSS {
        seg.flags = seg.flags | SegmentTypes::BSS;
    }
    state.other.segments.insert(0, seg);
    state.other.currentSegment = 0;
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_hex(mut str: *mut libc::c_char,
                               mut _dummy: *mut _MNE) {
    let mut i: u8 = 0;
    let mut result: u8 = 0;
    programlabel();
    state.output.generatedLength = 0;
    i = 0;
    while *str.offset(i as isize) != 0 {
        if !(*str.offset(i as isize) as libc::c_int == ' ' as i32) {
            result = (
                (gethexdig(*str.offset(i as isize) as libc::c_int) << 4)
                +
                gethexdig(*str.offset((i + 1) as isize) as libc::c_int)
            ) as u8;
            i += 1;
            if *str.offset(i as isize) == 0 {
                break;
            }
            state.output.generated[state.output.generatedLength] = result;
            state.output.generatedLength += 1;
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
        return c - 'a' as i32 + 10
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10
    }
    sprintf(sBuffer.as_mut_ptr(),
            b"Bad Hex Digit %c\x00" as *const u8 as *const libc::c_char, c);
    asmerr(AsmErrorEquates::SyntaxError, false,
           sBuffer.as_mut_ptr());
    println!("(Must be a valid hex digit)");
    filesystem::writeln_to_file_maybe(&mut state.output.listFile, "(Must be a valid hex digit)");
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn v_err(mut _str: *mut libc::c_char,
                               mut _dummy: *mut _MNE) {
    programlabel();
    asmerr(AsmErrorEquates::ErrPseudoOpEncountered,
           true, 0 as *const libc::c_char);
    std::process::exit(1);
}
#[no_mangle]
pub unsafe extern "C" fn v_dc(mut str: *mut libc::c_char,
                              mut mne: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut tmp: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut value: libc::c_long = 0;
    let mut macstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vmode: libc::c_char = 0;
    state.output.generatedLength = 0;
    programlabel();
    /* for byte, .byte, word, .word, long, .long */
    if *(*mne).name.offset(0 as isize) as libc::c_int !=
           'd' as i32 {
        static mut sTmp: [libc::c_char; 4] = [0; 4];
        strcpy(sTmp.as_mut_ptr(),
               b"x.x\x00" as *const u8 as *const libc::c_char);
        sTmp[2 as usize] =
            *(*mne).name.offset(0 as isize);
        findext(sTmp.as_mut_ptr());
    }
    /* F8... */
    /* db, dw, dd */
    if *(*mne).name.offset(0 as isize) as libc::c_int ==
           'd' as i32 &&
           *(*mne).name.offset(1 as isize) as libc::c_int !=
               'c' as i32 &&
           *(*mne).name.offset(1 as isize) as libc::c_int !=
               'v' as i32 {
        static mut sTmp_0: [libc::c_char; 4] = [0; 4];
        strcpy(sTmp_0.as_mut_ptr(),
               b"x.x\x00" as *const u8 as *const libc::c_char);
        if 'd' as i32 ==
               *(*mne).name.offset(1 as isize) as libc::c_int {
            sTmp_0[2 as usize] = 'l' as i32 as libc::c_char
        } else {
            sTmp_0[2 as usize] =
                *(*mne).name.offset(1 as isize)
        }
        findext(sTmp_0.as_mut_ptr());
    }
    /* ...F8 */
    if *(*mne).name.offset(1 as isize) as libc::c_int ==
           'v' as i32 {
        let mut i: libc::c_int = 0;
        vmode = 1;
        i = 0;
        while *str.offset(i as isize) as libc::c_int != 0 &&
                  *str.offset(i as isize) as libc::c_int != ' ' as i32 {
            i += 1
        }
        tmp = findsymbol(str, i);
        str = str.offset(i as isize);
        if tmp.is_null() {
            println!("EQM label not found");
            return
        }
        if (*tmp).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            macstr = (*tmp).string as *mut libc::c_void as *mut libc::c_char
        } else {
            println!("must specify EQM label for DV");
            return
        }
    }
    sym = eval(str, 0);
    while !sym.is_null() {
        value = (*sym).value;
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::DCNotResolved
        }
        if (*sym).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            let mut ptr: *mut libc::c_uchar =
                (*sym).string as *mut libc::c_void as *mut libc::c_uchar;
            loop  {
                value = *ptr as libc::c_long;
                if !(value != 0) { break ; }
                if vmode != 0 {
                    setspecial(value as libc::c_int, 0);
                    tmp = eval(macstr, 0);
                    value = (*tmp).value;
                    if (*tmp).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                        state.execution.redoIndex += 1;
                        state.execution.redoWhy |= ReasonCodes::DVNotResolvedProbably
                    }
                    FreeSymbolList(tmp);
                }
                match state.execution.modeNext {
                    AddressModes::WordAdr => {
                        if state.execution.bitOrder != BitOrder::LeastMost {
                            state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                            state.output.generatedLength += 1;
                        } else {
                            state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                            state.output.generatedLength += 1;
                        }
                    }
                    AddressModes::Long => {
                        if state.execution.bitOrder != BitOrder::LeastMost {
                            state.output.generated[state.output.generatedLength] = (value >> 24 & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value >> 16 & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                            state.output.generatedLength += 1;
                        } else {
                            state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value >> 16 & 0xff) as u8;
                            state.output.generatedLength += 1;
                            state.output.generated[state.output.generatedLength] = (value >> 24 & 0xff) as u8;
                            state.output.generatedLength += 1;
                        }
                    }
                    AddressModes::ByteAdr | _ => {
                        state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                        state.output.generatedLength += 1;
                    }
                }
                ptr = ptr.offset(1)
            }
        } else {
            if vmode != 0 {
                setspecial(value as libc::c_int, (*sym).flags as libc::c_int);
                tmp = eval(macstr, 0);
                value = (*tmp).value;
                if (*tmp).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::DVNotResolvedCould
                }
                FreeSymbolList(tmp);
            }
            match state.execution.modeNext {
                AddressModes::WordAdr => {
                    //any value outside two's complement +ve and +ve word representation is invalid...
                    if state.parameters.strictMode && (
                        value < -(0xffff as libc::c_int) as libc::c_long
                        ||
                        value > 0xffff as libc::c_int as libc::c_long
                    )
                       {
                        let mut sBuffer_0: [libc::c_char; 128] = [0; 128];
                        sprintf(sBuffer_0.as_mut_ptr(),
                                b"%s %ld\x00" as *const u8 as *const libc::c_char, (*mne).name, value);
                        asmerr(AsmErrorEquates::AddressMustBeLowerThan10000,
                               false, sBuffer_0.as_mut_ptr());
                    }
                    if state.execution.bitOrder != BitOrder::LeastMost {
                        state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                        state.output.generatedLength += 1;
                    } else {
                        state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                        state.output.generatedLength += 1;
                    }
                }
                AddressModes::Long => {
                    if state.execution.bitOrder != BitOrder::LeastMost {
                        state.output.generated[state.output.generatedLength] = (value >> 24 & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value >> 16 & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                        state.output.generatedLength += 1;
                    } else {
                        state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value >> 8 & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value >> 16 & 0xff) as u8;
                        state.output.generatedLength += 1;
                        state.output.generated[state.output.generatedLength] = (value >> 24 & 0xff) as u8;
                        state.output.generatedLength += 1;
                    }
                }
                AddressModes::ByteAdr | _ => {
                    //any value outside two's complement +ve and +ve byte representation is invalid...
                    if value < -(0xff as libc::c_int) as libc::c_long ||
                           value > 0xff as libc::c_int as libc::c_long {
                        let mut sBuffer: [libc::c_char; 128] = [0; 128];
                        sprintf(sBuffer.as_mut_ptr(),
                                b"%s %ld\x00" as *const u8 as
                                    *const libc::c_char, (*mne).name, value);
                        asmerr(AsmErrorEquates::AddressMustBeLowerThan100,
                               false, sBuffer.as_mut_ptr());
                    }
                    state.output.generated[state.output.generatedLength] = (value & 0xff) as u8;
                    state.output.generatedLength += 1;
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
                              mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut mult: libc::c_int = 1;
    let mut filler: libc::c_long = 0;
    if state.execution.modeNext == AddressModes::WordAdr { mult = 2 }
    if state.execution.modeNext == AddressModes::Long { mult = 4 }
    programlabel();
    sym = eval(str, 0);
    if !sym.is_null() {
        if !(*sym).next.is_null() { filler = (*(*sym).next).value }
        if (*sym).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::DSNotResolved
        } else {
            if !(*sym).next.is_null() &&
                   (*(*sym).next).flags as libc::c_int & 0x1 as libc::c_int !=
                       0 {
                state.execution.redoIndex += 1;
                state.execution.redoWhy |= ReasonCodes::DSNotResolved
            }
            genfill(filler, (*sym).value, mult);
        }
        FreeSymbolList(sym);
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_org(mut str: *mut libc::c_char,
                               mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    sym = eval(str, 0);
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.org = (*sym).value as u64;
    if (*sym).flags & SegmentTypes::Unknown != 0 {
        currentSegment.flags = currentSegment.flags | SegmentTypes::Unknown;
    } else {
        currentSegment.flags = currentSegment.flags & !SegmentTypes::Unknown;
    }
    if currentSegment.initflags & SegmentTypes::Unknown != 0 {
        currentSegment.initorg = (*sym).value as u64;
        currentSegment.initflags = (*sym).flags;
    }
    if !(*sym).next.is_null() {
        state.output.orgFill = (*(*sym).next).value as u8;
        if (*(*sym).next).flags & SegmentTypes::Unknown != 0 {
            asmerr(AsmErrorEquates::ValueUndefined, true, 0 as *const libc::c_char);
        }
    }
    programlabel();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_rorg(mut str: *mut libc::c_char,
                                mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.flags = currentSegment.flags | SegmentTypes::RelocatableOrigin;
    if (*sym).addrmode != AddressModes::Imp as u8 {
        currentSegment.rorg = (*sym).value as u64;
        if (*sym).flags & SegmentTypes::Unknown != 0 {
            currentSegment.rflags = currentSegment.rflags | SegmentTypes::Unknown;
        } else {
            currentSegment.rflags = currentSegment.rflags & !SegmentTypes::Unknown;
        }
        if currentSegment.initrflags & SegmentTypes::Unknown != 0 {
            currentSegment.initrorg = (*sym).value as u64;
            currentSegment.initrflags = (*sym).flags
        }
    }
    programlabel();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_rend(mut _str: *mut libc::c_char,
                                mut _dummy: *mut _MNE) {
    programlabel();
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.flags = currentSegment.flags & !SegmentTypes::RelocatableOrigin;
}
#[no_mangle]
pub unsafe extern "C" fn v_align(mut str: *mut libc::c_char,
                                 mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    let mut fill: u8 = 0;
    let mut rorg: u8 = currentSegment.flags & SegmentTypes::RelocatableOrigin;
    if rorg != 0 {
        currentSegment.rflags = currentSegment.rflags | SegmentTypes::Referenced;
    } else {
        currentSegment.flags = currentSegment.flags | SegmentTypes::Referenced;
    }
    if !(*sym).next.is_null() {
        if (*(*sym).next).flags & SymbolTypes::Unknown != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::AlignNotResolved
        } else {
            fill = (*(*sym).next).value as libc::c_uchar
        }
    }
    if rorg != 0 {
        if (currentSegment.rflags | (*sym).flags) & SegmentTypes::Unknown != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::AlignRelocatableOriginNotKnown
        } else {
            let mut n: libc::c_long = ((*sym).value as u64).wrapping_sub(currentSegment.rorg.wrapping_rem((*sym).value as u64)) as libc::c_long;
            if n != (*sym).value {
                genfill(fill as libc::c_long, n, 1);
            }
        }
    } else if (currentSegment.flags | (*sym).flags) & SymbolTypes::Unknown != 0 {
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::AlignNormalOriginNotKnown
    } else {
        let mut n_0: libc::c_long = ((*sym).value as u64).wrapping_sub(currentSegment.org.wrapping_rem((*sym).value as u64)) as libc::c_long;
        if n_0 != (*sym).value {
            genfill(fill as libc::c_long, n_0, 1);
        }
    }
    FreeSymbolList(sym);
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_subroutine(mut _str: *mut libc::c_char,
                                      mut _dummy: *mut _MNE) {
    Lastlocalindex = Lastlocalindex.wrapping_add(1);
    Localindex = Lastlocalindex;
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_equ(mut str: *mut libc::c_char,
                               mut dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let currentSegment = &state.other.segments[state.other.currentSegment];
    /*
    * If we encounter a line of the form
    *   . = expr	; or . EQU expr
    * treat it as one of
    *     org expr
    *     rorg expr
    * depending on whether we have a relocatable origin now or not.
    */
    if
        strlen(*Av.as_mut_ptr().offset(0)) == 1 &&
        (
            *(*Av.as_mut_ptr().offset(0)).offset(0) as libc::c_int == '.' as i32 ||
            *(*Av.as_mut_ptr().offset(0)).offset(0) as libc::c_int == '*' as i32 && {
                let ref mut fresh32 = *(*Av.as_mut_ptr().offset(0)).offset(0);
                *fresh32 = '.' as i32 as libc::c_char;
                (*fresh32 as libc::c_int) != 0
            } && true
        ) {
        /* Av[0][0] = '\0'; */
        if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
            v_rorg(str, dummy);
        } else {
            v_org(str, dummy);
        }
        return
    }
    lab = findsymbol(*Av.as_mut_ptr().offset(0), strlen(*Av.as_mut_ptr().offset(0)) as libc::c_int);
    if lab.is_null() {
        lab = CreateSymbol(*Av.as_mut_ptr().offset(0), strlen(*Av.as_mut_ptr().offset(0)) as libc::c_int)
    }
    if (*lab).flags as libc::c_int & SymbolTypes::Unknown as libc::c_int == 0 {
        if (*sym).flags as libc::c_int & SymbolTypes::Unknown as libc::c_int != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::EquNotResolved
        } else if (*lab).value != (*sym).value {
            asmerr(AsmErrorEquates::EquValueMismatch,
                   false, 0 as *const libc::c_char);
            printf(b"INFO: Label \'%s\' changed from $%04lx to $%04lx\n\x00"
                       as *const u8 as *const libc::c_char,
                   *Av.as_mut_ptr().offset(0 as isize),
                   (*lab).value, (*sym).value);
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::EquValueMismatch
        }
    }
    (*lab).value = (*sym).value;
    (*lab).flags =
        ((*sym).flags as libc::c_int &
             (SegmentTypes::Unknown as libc::c_int | SymbolTypes::StringResult as libc::c_int)) as libc::c_uchar;
    (*lab).string = (*sym).string;
    (*sym).flags =
        ((*sym).flags as libc::c_int &
             !(0x8 as libc::c_int | 0x20 as libc::c_int)) as libc::c_uchar;
    /* List the value */
    let mut v: libc::c_ulong = (*lab).value as libc::c_ulong;
    state.output.generatedLength = 0;
    if v > 0xffff as libc::c_int as libc::c_ulong {
        state.output.generated[state.output.generatedLength] = (v >> 24) as u8;
        state.output.generatedLength += 1;
        state.output.generated[state.output.generatedLength] = (v >> 16) as u8;
        state.output.generatedLength += 1;
    }
    state.output.generated[state.output.generatedLength] = (v >> 8) as u8;
    state.output.generatedLength += 1;
    state.output.generated[state.output.generatedLength] = v as u8;
    state.output.generatedLength += 1;
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_eqm(mut str: *mut libc::c_char,
                               mut _dummy: *mut _MNE) {
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut len: libc::c_int =
        strlen(*Av.as_mut_ptr().offset(0 as isize)) as
            libc::c_int;
    lab = findsymbol(*Av.as_mut_ptr().offset(0 as isize), len);
    if !lab.is_null() {
        if (*lab).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            free((*lab).string as *mut libc::c_void);
        }
    } else {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as isize),
                         len)
    }
    (*lab).value = 0;
    (*lab).flags =
        (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int) as u8;
    (*lab).string =
        strcpy(ckmalloc(strlen(str).wrapping_add(1 as
                                                     libc::c_ulong) as
                            libc::c_int), str);
}
#[no_mangle]
pub unsafe extern "C" fn v_echo(mut str: *mut libc::c_char,
                                mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
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
            filesystem::write_to_file_maybe(
                &mut state.output.listFile,
                format!(" {}", transient::str_pointer_to_string(buf.as_mut_ptr())).as_str(),
            );
            //printf(" %s", buf);
            addmsg(b" \x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char); // -FXQ supress output until final pass
            addmsg(buf.as_mut_ptr());
        }
        s = (*s).next
    }
    //puts("");
    addmsg(b"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    filesystem::writeln_to_file_maybe(&mut state.output.listFile, "");
}
#[no_mangle]
pub unsafe extern "C" fn v_set(mut str: *mut libc::c_char,
                               mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut dynamicname: [libc::c_char; 257] = [0; 257];
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut setundefined: libc::c_int = 0;
    while *str.offset(i as isize) as libc::c_int != 0 &&
              *str.offset(i as isize) as libc::c_int != '\"' as i32 &&
              *str.offset(i as isize) as libc::c_int != ' ' as i32 &&
              *str.offset(i as isize) as libc::c_int != ',' as i32 {
        i += 1
    }
    if *str.offset(i as isize) as libc::c_int != 0 &&
           *str.offset(i as isize) as libc::c_int == ',' as i32 {
        // is this SET is using the "," eval-concat operator?
        strncpy(dynamicname.as_mut_ptr(), str, 256);
        if i < 256 {
            dynamicname[i] = 0;
        }
        dynamicname[256] = 0;
        j = strlen(dynamicname.as_mut_ptr()) as usize;
        // eval-concat argument processing loop...
        while *str.offset(i as isize) as libc::c_int != 0 &&
                  *str.offset(i as isize) as libc::c_int != '\"' as i32 &&
                  *str.offset(i as isize) as libc::c_int != ' ' as i32 {
            if *str.offset(i as isize) as libc::c_int == 0 ||
                   *str.offset(i as isize) as libc::c_int == ' ' as i32 {
                break ;
                // process any remaining arguments
            } // argument was symbol
            if *str.offset((i + 1) as isize) as libc::c_int == '\"' as i32 {
                // is this a string constant?
                i = i + 2; // argument was string constant
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
                    dynamicname[fresh38] = *str.offset(fresh37 as isize)
                }
                if *str.offset(i as isize) as libc::c_int != 0 &&
                       *str.offset(i as isize) as libc::c_int == '\"' as i32 {
                    dynamicname[j as usize] = 0;
                    i += 1
                } else {
                    asmerr(AsmErrorEquates::SyntaxError, false, str);
                }
            } else {
                // this argument is a symbol to be evaluated
                let mut t: libc::c_int = 0;
                let mut tempbuf: [libc::c_char; 257] = [0; 257];
                let mut tempval: [libc::c_char; 257] = [0; 257];
                let mut symarg: *mut _SYMBOL = 0 as *mut _SYMBOL;
                strncpy(tempbuf.as_mut_ptr(),
                        str.offset(i as
                                       isize).offset(1 as
                                                         isize),
                        256 as libc::c_int as libc::c_ulong);
                tempbuf[256 as libc::c_int as usize] =
                    0;
                t = 0;
                while (t as libc::c_ulong) < strlen(tempbuf.as_mut_ptr()) {
                    if tempbuf[t as usize] as libc::c_int == ',' as i32 {
                        tempbuf[t as usize] = 0
                    }
                    t += 1
                }
                symarg = eval(tempbuf.as_mut_ptr(), 0);
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
                        j = (j).wrapping_add(strlen(tempval.as_mut_ptr()) as usize);
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
        dynamicname[fresh39] = 0;
        if setundefined != 0 {
            // not all of the arguments are defined yet, so skip this SET
            return
        }
        sym = eval(dynamicname.as_mut_ptr(), 0)
    } else {
        // traditional SET behavior
        sym = eval(str, 0)
    } /* garbage */
    lab =
        findsymbol(*Av.as_mut_ptr().offset(0 as isize),
                   strlen(*Av.as_mut_ptr().offset(0 as isize))
                       as libc::c_int);
    if lab.is_null() {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as isize),
                         strlen(*Av.as_mut_ptr().offset(0 as
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
    if Mlevel == MAX_MACRO_LEVEL as libc::c_int as libc::c_uint {
        println!("infinite macro recursion");
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
                          libc::c_ulong).wrapping_add(1 as
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
            0;
        if *str as libc::c_int == ',' as i32 { str = str.offset(1) }
        while *str as libc::c_int == ' ' as i32 { str = str.offset(1) }
    }
    inc =
        zmalloc(::std::mem::size_of::<_INCFILE>() as libc::c_ulong as
                    libc::c_int) as *mut _INCFILE;
    (*inc).next = pIncfile;
    (*inc).name = (*mac).name;
    (*inc).fi = (*pIncfile).fi;
    (*inc).lineno = 0;
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
pub unsafe extern "C" fn v_end(mut _str: *mut libc::c_char,
                               mut _dummy: *mut _MNE) {
    /* Only ENDs current file and any macro calls within it */
    while (*pIncfile).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        v_endm(0 as *mut libc::c_char, 0 as *mut _MNE);
    }
    fseek((*pIncfile).fi, 0, 2);
}
#[no_mangle]
pub unsafe extern "C" fn v_endm(mut _str: *mut libc::c_char,
                                mut _dummy: *mut _MNE) {
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
    println!("not within a macro");
}
#[no_mangle]
pub unsafe extern "C" fn v_mexit(mut _str: *mut libc::c_char,
                                 mut _dummy: *mut _MNE) {
    v_endm(0 as *mut libc::c_char, 0 as *mut _MNE);
}
#[no_mangle]
pub unsafe extern "C" fn v_ifconst(mut str: *mut libc::c_char,
                                   mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    sym = eval(str, 0);
    pushif((*sym).flags as libc::c_int == 0);
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_ifnconst(mut str: *mut libc::c_char,
                                    mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    sym = eval(str, 0);
    pushif((*sym).flags as libc::c_int != 0);
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_if(mut str: *mut libc::c_char,
                              mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        pushif(false);
        return
    }
    programlabel();
    sym = eval(str, 0);
    if (*sym).flags != 0 {
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::IfNotResolved;
        pushif(false);
        (*Ifstack).acctrue = 0;
        state.execution.redoIf |= 1
    } else { pushif((*sym).value != 0); }
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_else(mut _str: *mut libc::c_char,
                                mut _dummy: *mut _MNE) {
    if (*Ifstack).acctrue as libc::c_int != 0 &&
           (*Ifstack).flags as libc::c_int & 0x4 as libc::c_int == 0 {
        programlabel();
        (*Ifstack).xtrue =
            ((*Ifstack).xtrue == 0) as libc::c_int as libc::c_uchar
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_endif(mut _str: *mut libc::c_char,
                                 mut _dummy: *mut _MNE) {
    let mut ifs: *mut _IFSTACK = Ifstack;
    if (*ifs).flags as libc::c_int & 0x4 as libc::c_int == 0 {
        if (*ifs).acctrue != 0 { programlabel(); }
        if (*ifs).file != pIncfile {
            println!("too many endif\'s");
        } else { Ifstack = (*ifs).next; free(ifs as *mut libc::c_void); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_repeat(mut str: *mut libc::c_char,
                                  mut _dummy: *mut _MNE) {
    let mut rp: *mut _REPLOOP = 0 as *mut _REPLOOP;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        pushif(false);
        return
    }
    programlabel();
    sym = eval(str, 0);
    if (*sym).value == 0 {
        pushif(false);
        FreeSymbolList(sym);
        return
    }
    /* Don't allow negative values for REPEAT loops */
    if (*sym).value < 0 {
        pushif(false);
        FreeSymbolList(sym);
        asmerr(AsmErrorEquates::RepeatNegative, false,
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
    if (*rp).flags as libc::c_int != 0 {
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::RepeatNotResolved
    }
    Reploop = rp;
    FreeSymbolList(sym);
    pushif(true);
}
#[no_mangle]
pub unsafe extern "C" fn v_repend(mut _str: *mut libc::c_char,
                                  mut _dummy: *mut _MNE) {
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        v_endif(0 as *mut libc::c_char, 0 as *mut _MNE);
        return
    }
    if !Reploop.is_null() && (*Reploop).file == pIncfile {
        if (*Reploop).flags as libc::c_int == 0 &&
               {
                   (*Reploop).count = (*Reploop).count.wrapping_sub(1);
                   ((*Reploop).count) != 0
               } {
            if (*pIncfile).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                (*pIncfile).strlist = (*Reploop).seek as *mut _STRLIST
            } else {
                fseek((*pIncfile).fi, (*Reploop).seek as libc::c_long,
                      0);
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
    println!("no repeat");
}
#[no_mangle]
pub static mut incdirlist: *mut _STRLIST =
    0 as *const _STRLIST as *mut _STRLIST;
#[no_mangle]
pub unsafe extern "C" fn v_incdir(mut str: *mut libc::c_char, mut _dummy: *mut _MNE) {
    let mut tail: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: libc::c_int = 0;
    // FIXME: new, start
    let filename = get_filename(transient::str_pointer_to_string(str).as_str()).to_owned();
    if state.execution.includeDirList.iter().find(|dir| dir.cmp(&&filename) == Ordering::Equal).is_none() {
        state.execution.includeDirList.push(filename);
    }
    // FIXME: new, end
    buf = getfilename(str);
    tail = &mut incdirlist;
    while !(*tail).is_null() {
        if strcmp((**tail).buf.as_mut_ptr(), buf) == 0 {
            found = 1
        }
        tail = &mut (**tail).next
    }
    if found == 0 {
        let mut newdir: *mut _STRLIST = 0 as *mut _STRLIST;
        newdir =
            permalloc((::std::mem::size_of::<*mut _STRLIST>() as
                           libc::c_ulong).wrapping_add(1 as
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
    if pos > 0 &&
           *dest.offset((pos - 1) as isize) as libc::c_int !=
               ':' as i32 &&
           *dest.offset((pos - 1) as isize) as libc::c_int !=
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
static mut Seglen: usize = 0;
static mut Seekback: libc::c_long = 0;
#[no_mangle]
pub unsafe extern "C" fn generate() {
    let mut seekpos: libc::c_long = 0;
    static mut org: libc::c_ulong = 0;
    let mut i: i32 = 0;
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    if state.execution.redoIndex == 0 {
        if currentSegment.flags & SegmentTypes::BSS == 0 {
            i = state.output.generatedLength as i32 - 1;
            while i >= 0 {
                CheckSum = CheckSum.wrapping_add(state.output.generated[i as usize] as libc::c_ulong);
                i -= 1;
            }
            if state.execution.isClear {
                state.execution.isClear = false;
                if currentSegment.flags & SegmentTypes::Unknown != 0 {
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::Obscure;
                    return
                }
                org = currentSegment.org;
                if state.parameters.format == Format::Default || state.parameters.format == Format::Ras {
                    putc((org & 0xff as libc::c_int as libc::c_ulong) as libc::c_int, FI_temp);
                    putc((org >> 8 & 0xff as libc::c_int as libc::c_ulong) as libc::c_int, FI_temp);
                    if state.parameters.format == Format::Ras {
                        Seekback = ftell(FI_temp);
                        Seglen = 0;
                        putc(0, FI_temp);
                        putc(0, FI_temp);
                    }
                }
            }
            match state.parameters.format {
                Format::Raw | Format::Default => {
                    if currentSegment.org < org {
                        println!(
                            "segment: {} {}  vs current org: {:04x}",
                            currentSegment.name,
                            formatting::segment_address_to_string(currentSegment.org, currentSegment.flags),
                            org
                        );
                        asmerr(AsmErrorEquates::OriginReverseIndexed,
                               true,
                               0 as *const libc::c_char);
                        std::process::exit(1);
                    }
                    while currentSegment.org != org {
                        putc(state.output.orgFill as libc::c_int, FI_temp);
                        org = org.wrapping_add(1)
                    }
                    fwrite(state.output.generated.as_mut_ptr() as *const libc::c_void,
                           state.output.generatedLength as size_t, 1 as size_t,
                           FI_temp);
                }
                Format::Ras => {
                    if org != currentSegment.org {
                        org = currentSegment.org;
                        seekpos = ftell(FI_temp);
                        fseek(FI_temp, Seekback, 0);
                        putc((Seglen & 0xff) as libc::c_int, FI_temp);
                        putc((Seglen >> 8 & 0xff) as libc::c_int, FI_temp);
                        fseek(FI_temp, seekpos, 0);
                        putc((org & 0xff) as libc::c_int, FI_temp);
                        putc((org >> 8 & 0xff) as libc::c_int, FI_temp);
                        Seekback = ftell(FI_temp);
                        Seglen = 0;
                        putc(0, FI_temp);
                        putc(0, FI_temp);
                    }
                    fwrite(state.output.generated.as_mut_ptr() as *const libc::c_void,
                           state.output.generatedLength as size_t, 1 as size_t,
                           FI_temp);
                    Seglen += state.output.generatedLength;
                }
            }
            org = org.wrapping_add(state.output.generatedLength as u64)
        }
    }
    currentSegment.org = currentSegment.org.wrapping_add(state.output.generatedLength as u64);
    if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
        currentSegment.rorg = currentSegment.rorg.wrapping_add(state.output.generatedLength as u64)
    };
}
#[no_mangle]
pub unsafe extern "C" fn closegenerate() {
    if state.execution.redoIndex == 0 {
        if state.parameters.format == Format::Ras {
            fseek(FI_temp, Seekback, 0);
            putc((Seglen & 0xff) as libc::c_int, FI_temp);
            putc((Seglen >> 8 & 0xff) as libc::c_int, FI_temp);
            fseek(FI_temp, 0, 2);
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
    c3 = (fill >> 24) as libc::c_uchar;
    c2 = (fill >> 16) as libc::c_uchar;
    c1 = (fill >> 8) as libc::c_uchar;
    c0 = fill as libc::c_uchar;
    match size {
        1 => {
            memset(state.output.generated.as_mut_ptr() as *mut libc::c_void, c0 as libc::c_int,
                   ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                       libc::c_ulong);
        }
        2 => {
            bytes <<= 1;
            i = 0;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                          libc::c_ulong {
                if state.execution.bitOrder != BitOrder::LeastMost {
                    state.output.generated[(i + 0) as usize] = c1;
                    state.output.generated[(i + 1) as usize] = c0
                } else {
                    state.output.generated[(i + 0) as usize] = c0;
                    state.output.generated[(i + 1) as usize] = c1
                }
                i += 2
            }
        }
        4 => {
            bytes <<= 2;
            i = 0;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[libc::c_uchar; 1024]>() as
                          libc::c_ulong {
                if state.execution.bitOrder != BitOrder::LeastMost {
                    state.output.generated[(i + 0) as usize] = c3;
                    state.output.generated[(i + 1) as usize] = c2;
                    state.output.generated[(i + 2) as usize] = c1;
                    state.output.generated[(i + 3) as usize] = c0
                } else {
                    state.output.generated[(i + 0) as usize] = c0;
                    state.output.generated[(i + 1) as usize] = c1;
                    state.output.generated[(i + 2) as usize] = c2;
                    state.output.generated[(i + 3) as usize] = c3
                }
                i += 4
            }
        }
        _ => { }
    }
    state.output.generatedLength = ::std::mem::size_of::<[libc::c_uchar; 1024]>();
    while bytes as libc::c_ulong > ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong {
        generate();
        bytes = (bytes as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong) as libc::c_long as libc::c_long
    }
    state.output.generatedLength = bytes as usize;
    generate();
}
#[no_mangle]
pub unsafe extern "C" fn pushif(mut xbool: bool) {
    let mut ifs: *mut _IFSTACK =
        zmalloc(::std::mem::size_of::<_IFSTACK>() as libc::c_ulong as
                    libc::c_int) as *mut _IFSTACK;
    (*ifs).next = Ifstack;
    (*ifs).file = pIncfile;
    (*ifs).flags = 0;
    (*ifs).xtrue = xbool as libc::c_uchar;
    (*ifs).acctrue =
        ((*Ifstack).acctrue as libc::c_int != 0 &&
             (*Ifstack).xtrue as libc::c_int != 0) as libc::c_int as u8;
    Ifstack = ifs;
}
