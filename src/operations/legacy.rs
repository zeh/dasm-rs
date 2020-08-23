use libc;
use std::cmp::Ordering;
use std::convert::TryFrom;

use crate::constants::{
    MAX_MACRO_LEVEL,
};
use crate::globals::state;
use crate::types::flags::{
    FileFlags,
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
    fn fclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fopen(__filename: *const i8, __modes: *const i8)
     -> *mut FILE;
    #[no_mangle]
    fn snprintf(_: *mut i8, _: u64,
                _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32)
     -> i32;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> i64;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8)
     -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64)
     -> *mut i8;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8,
               _: u64) -> i32;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut pIncfile: *mut _INCFILE;
    #[no_mangle]
    static mut Reploop: *mut _REPLOOP;
    #[no_mangle]
    static mut Ifstack: *mut _IFSTACK;
    #[no_mangle]
    static mut Av: [*mut i8; 0];
    #[no_mangle]
    static mut Cvt: [u32; 0];
    #[no_mangle]
    static mut Opsize: [u32; 0];
    #[no_mangle]
    fn findext(str: *mut i8);
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const i8)
     -> i32;
    #[no_mangle]
    fn rmnode(base: *mut *mut libc::c_void, bytes: i32);
    #[no_mangle]
    fn addhashtable(mne: *mut _MNE);
    #[no_mangle]
    fn pushinclude(str: *mut i8);
    #[no_mangle]
    fn permalloc(bytes: i32) -> *mut i8;
    #[no_mangle]
    fn zmalloc(bytes: i32) -> *mut i8;
    #[no_mangle]
    fn ckmalloc(bytes: i32) -> *mut i8;
    #[no_mangle]
    fn addmsg(message: *mut i8);
    #[no_mangle]
    fn setspecial(value: i32, flags: i32);
    #[no_mangle]
    fn findsymbol(str: *const i8, len: i32) -> *mut _SYMBOL;
    #[no_mangle]
    fn CreateSymbol(str: *const i8, len: i32)
     -> *mut _SYMBOL;
    #[no_mangle]
    fn FreeSymbolList(sym: *mut _SYMBOL);
    #[no_mangle]
    static mut Lastlocaldollarindex: u64;
    #[no_mangle]
    fn programlabel();
    #[no_mangle]
    static mut Localdollarindex: u64;
    #[no_mangle]
    static mut Localindex: u64;
    #[no_mangle]
    static mut Lastlocalindex: u64;
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const i8, wantmode: i32) -> *mut _SYMBOL;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _STRLIST {
    pub next: *mut _STRLIST,
    // Conversion note: this buffer size was 4 originally, but increased to fix a buffer overrun
    pub buf: [i8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MNE {
    pub next: *mut _MNE,
    pub vect: Option<unsafe extern "C" fn(_: *mut i8, _: *mut _MNE)
                         -> ()>,
    pub name: *const i8,
    pub flags: u8,
    pub okmask: u64,
    pub opcode: [u32; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MACRO {
    pub next: *mut _MACRO,
    pub vect: Option<unsafe extern "C" fn(_: *mut i8,
                                          _: *mut _MACRO) -> ()>,
    pub name: *mut i8,
    pub flags: u8,
    pub strlist: *mut _STRLIST,
    pub defpass: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _INCFILE {
    pub next: *mut _INCFILE,
    pub name: *mut i8,
    pub fi: *mut FILE,
    pub lineno: u64,
    pub flags: u8,
    pub args: *mut _STRLIST,
    pub strlist: *mut _STRLIST,
    pub saveidx: u64,
    pub savedolidx: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _REPLOOP {
    pub next: *mut _REPLOOP,
    pub count: u64,
    pub seek: u64,
    pub lineno: u64,
    pub file: *mut _INCFILE,
    pub flags: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFSTACK {
    pub next: *mut _IFSTACK,
    pub file: *mut _INCFILE,
    pub flags: u8,
    pub xtrue: u8,
    pub acctrue: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SEGMENT {
    pub next: *mut _SEGMENT,
    pub name: *mut i8,
    pub flags: u8,
    pub rflags: u8,
    pub org: u64,
    pub rorg: u64,
    pub initorg: u64,
    pub initrorg: u64,
    pub initflags: u8,
    pub initrflags: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SYMBOL {
    pub next: *mut _SYMBOL,
    pub name: *mut i8,
    pub string: *mut i8,
    pub flags: u8,
    pub addrmode: u8,
    pub value: i64,
    pub namelen: u32,
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
pub unsafe extern "C" fn v_processor(mut str: *mut i8,
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
pub unsafe extern "C" fn v_mnemonic(mut str: *mut i8,
                                    mut mne: *mut _MNE) {
    let mut addressMode: AddressModes = AddressModes::Imp;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut opcode: u32 = 0;
    let mut opidx: usize = 0;
    let mut symbase: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut opsize: i32 = 0;
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.flags |= SegmentTypes::Referenced;
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
        if (*sym).flags & SymbolTypes::Unknown != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved
        }
        sym = (*sym).next
    }
    sym = symbase;
    if (*mne).flags as i32 & 0x40 as i32 != 0 {
        if !(*sym).next.is_null() {
            (*sym).addrmode = AddressModes::BitMod as u8;
            if (*mne).flags as i32 & 0x20 as i32 != 0 &&
                   !(*sym).next.is_null() {
                (*sym).addrmode = AddressModes::BitBraMod as u8
            }
        }
    }
    addressMode = AddressModes::try_from((*sym).addrmode).unwrap();
    if (*sym).flags & SymbolTypes::Unknown != 0 || (*sym).value >= 0x100 as i32 as i64 {
        opsize = 2
    } else {
        opsize = if (*sym).value != 0 {
            1
        } else {
            0
        }
    }
    while (*mne).okmask & ((1) << addressMode as isize) as u64 == 0 && *Cvt.as_mut_ptr().offset(addressMode as isize) != 0 {
        addressMode = AddressModes::try_from(*Cvt.as_mut_ptr().offset(addressMode as isize) as u8).unwrap();
    }
    if state.execution.trace {
        println!(
            "memask: {:08x} adrmode: {}  Cvt[am]: {}",
            (*mne).okmask,
            addressMode as u8,
            *Cvt.as_mut_ptr().offset(addressMode as isize)
        );
    }
    if (*mne).okmask & ((1) << addressMode as u8) as u64 == 0 {
        let buffer = format!(
            "{} {}",
            transient::str_pointer_to_string((*mne).name),
            transient::str_pointer_to_string(str),
        );
        asmerr(AsmErrorEquates::IllegalAddressingMode, false, transient::string_to_str_pointer(buffer));
        FreeSymbolList(symbase);
        //FIX
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved;
        return
    }
    if state.execution.modeNext != AddressModes::None {
        /*	Force	*/
        addressMode = state.execution.modeNext;
        if (*mne).okmask & ((1) << addressMode as u8) as u64 == 0 {
            asmerr(AsmErrorEquates::IllegalForcedAddressingMode, false, (*mne).name);
            FreeSymbolList(symbase);
            //FIX: Cause assembly to fail when an invalid mode is used for an opcode...
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::MnemonicNotResolved;
            return
        }
    }
    if state.execution.trace {
        println!("final addrmode = {}", addressMode as u8);
    }
    while opsize as u32 > *Opsize.as_mut_ptr().offset(addressMode as isize) {
        if *Cvt.as_mut_ptr().offset(addressMode as isize) == 0 || (*mne).okmask & ((1) << *Cvt.as_mut_ptr().offset(addressMode as isize)) as u64 == 0 {
            if (*sym).flags as i32 & 0x1 as i32 != 0 {
                break;
            }
            //FIX: for negative operands...
            if addressMode == AddressModes::Imm8 && (*sym).value < 0 {
                opsize = 1; /*  to end of instruction   */
                (*sym).value = ((*sym).value & 255 as i32 as i64) as i8 as i64;
                break;
            } else {
                let buffer = format!(
                    "{} {}",
                    transient::str_pointer_to_string((*mne).name),
                    transient::str_pointer_to_string(str),
                );
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100, false, transient::string_to_str_pointer(buffer));
                break;
            }
        } else {
            addressMode = AddressModes::try_from(*Cvt.as_mut_ptr().offset(addressMode as isize) as u8).unwrap();
        }
    }
    opcode = (*mne).opcode[addressMode as usize];
    opidx = (1 + (opcode > 0xff) as i32) as usize;
    if opidx == 2 {
        state.output.generated[0] = (opcode >> 8) as u8;
        state.output.generated[1] = opcode as u8;
    } else {
        state.output.generated[0] = opcode as u8;
    }
    match addressMode {
        AddressModes::BitMod => {
            sym = (*symbase).next;
            if (*sym).flags as i32 & 0x1 as i32 == 0 && (*sym).value >= 0x100 as i32 as i64 {
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100, false, 0 as *const i8);
            }
            state.output.generated[opidx] = (*sym).value as u8;
            opidx = opidx + 1;
            if (*symbase).flags as i32 & 0x1 as i32 == 0 {
                if (*symbase).value > 7 {
                    asmerr(AsmErrorEquates::IllegalBitSpecification, false, str);
                } else {
                    state.output.generated[0] = (
                        state.output.generated[0] as i64 +
                        ((*symbase).value << 1)
                    ) as u8
                }
            }
        }
        AddressModes::BitBraMod => {
            if (*symbase).flags as i32 & 0x1 as i32 == 0 {
                if (*symbase).value > 7 {
                    asmerr(AsmErrorEquates::IllegalBitSpecification,
                           false, str);
                } else {
                    state.output.generated[0] = (
                        state.output.generated[0] as i64 +
                        ((*symbase).value << 1)
                    ) as u8
                }
            }
            sym = (*symbase).next;
            if (*sym).flags as i32 & 0x1 as i32 == 0 &&
                   (*sym).value >= 0x100 as i32 as i64 {
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100,
                       false, 0 as *const i8);
            }
            state.output.generated[opidx] = (*sym).value as u8;
            opidx = opidx + 1;
            sym = (*sym).next
        }
        AddressModes::Rel => { }
        _ => {
            if *Opsize.as_mut_ptr().offset(addressMode as isize) > 0 {
                state.output.generated[opidx] = (*sym).value as u8;
                opidx = opidx + 1;
            }
            if *Opsize.as_mut_ptr().offset(addressMode as isize) == 2 {
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
    if (*mne).flags & 0x10 != 0 {
        if !sym.is_null() {
            if (*sym).flags & SymbolTypes::Unknown == 0 && (*sym).value >= 0x100 as i32 as i64 {
                asmerr(AsmErrorEquates::AddressMustBeLowerThan100, false, 0 as *const i8);
            }
            state.output.generated[opidx as usize] = (*sym).value as u8;
            sym = (*sym).next
        } else {
            asmerr(AsmErrorEquates::NotEnoughArgs, true, 0 as *const i8);
        }
        opidx += 1
    }
    if (*mne).flags & 0x20 != 0 || addressMode == AddressModes::Rel {
        opidx += 1;
        if sym.is_null() {
            asmerr(AsmErrorEquates::NotEnoughArgs, true, 0 as *const i8);
        } else if (*sym).flags & SymbolTypes::Unknown == 0 {
            let mut pc: u64 = 0;
            let mut pcf: u8 = 0;
            let mut dest: i64 = 0;
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
                if dest >= 128 || dest < -128 {
                    /*	byte before end of inst.    */
                    //FIX: sometimes zero page addressing will first be assumed to be absolute until
                    //     another pass. ERROR_BRANCH_OUT_OF_RANGE was made non-fatal, but we keep
                    //     pushing for Redo so assembly won't actually be succesfull until the branch
                    //     actually works.
                    let buffer = format!("{}", dest);
                    asmerr(AsmErrorEquates::BranchOutOfRange, false, transient::string_to_str_pointer(buffer));
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::BranchOutOfRange;
                    (*sym).flags = ((*sym).flags as i32 | 0x1 as i32) as u8;
                    dest = 0;
                }
            } else {
                /* Don't bother - we'll take another pass */
                dest = 0;
            } /*  Only so outlist() works */
            state.output.generated[(opidx - 1) as usize] = (dest & 0xff) as u8;
        }
    }
    state.output.generatedLength = opidx;
    generate();
    FreeSymbolList(symbase);
}
#[no_mangle]
pub unsafe extern "C" fn v_trace(mut str: *mut i8,
                                 mut _dummy: *mut _MNE) {
    state.execution.trace =
        *str.offset(1 as isize) as i32 == 'n' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn v_list(mut str: *mut i8,
                                mut _dummy: *mut _MNE) {
    programlabel();

    // Only so outlist() works
    state.output.generatedLength = 0;

    if strncmp(str, b"localoff\x00" as *const u8 as *const i8, 7) == 0 ||
        strncmp(str, b"LOCALOFF\x00" as *const u8 as *const i8, 7) == 0 {
        (*pIncfile).flags = (*pIncfile).flags | FileFlags::NoList;
    } else if strncmp(str, b"localon\x00" as *const u8 as *const i8, 7) == 0 ||
        strncmp(str, b"LOCALON\x00" as *const u8 as *const i8, 7) == 0 {
        (*pIncfile).flags = ((*pIncfile).flags & !(FileFlags::NoList)) as u8;
    } else if strncmp(str, b"off\x00" as *const u8 as *const i8, 2) == 0 ||
        strncmp(str, b"OFF\x00" as *const u8 as *const i8, 2) == 0 {
        state.execution.listMode = ListMode::None
    } else {
        state.execution.listMode = ListMode::List
    };
}
unsafe extern "C" fn getfilename(mut str: *mut i8)
 -> *mut i8 {
    if *str as i32 == '\"' as i32 {
        let mut buf: *mut i8 = 0 as *mut i8;
        str = str.offset(1);
        buf =
            ckmalloc(strlen(str).wrapping_add(1 as u64) as i32);
        strcpy(buf, str);
        str = buf;
        while *str as i32 != 0 && *str as i32 != '\"' as i32 {
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
pub unsafe extern "C" fn v_include(mut str: *mut i8,
                                   mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    // only eval the string if it's compliant with symbol naming
    if (*str as i32) < '0' as i32 || *str as i32 > '9' as i32
       {
        //check could be more comprehensive
        sym = eval(str, 0)
    } else { sym = 0 as *mut _SYMBOL }
    if !sym.is_null() && (*sym).flags as i32 & 0x8 as i32 != 0
       {
        pushinclude((*sym).string);
    } else {
        let mut buf: *mut i8 = 0 as *mut i8;
        buf = getfilename(str);
        pushinclude(buf);
        if buf != str { free(buf as *mut libc::c_void); }
    }
    if !sym.is_null() { FreeSymbolList(sym); };
}
#[no_mangle]
pub unsafe extern "C" fn v_incbin(mut str: *mut i8,
                                  mut _dummy: *mut _MNE) {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut binfile: *mut FILE = 0 as *mut FILE;
    programlabel();
    buf = getfilename(str);
    binfile = pfopen(buf, b"rb\x00" as *const u8 as *const i8);
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
                        ::std::mem::size_of::<[u8; 1024]>() as u64,
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
        println!("unable to open {}", transient::str_pointer_to_string(buf));
    }
    if buf != str { free(buf as *mut libc::c_void); }
    state.output.generatedLength = 0;
    /* don't list hexdump */
}
#[no_mangle]
pub unsafe extern "C" fn v_seg(mut str: *mut i8, mut _dummy: *mut _MNE) {
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
pub unsafe extern "C" fn v_hex(mut str: *mut i8,
                               mut _dummy: *mut _MNE) {
    let mut i: u8 = 0;
    let mut result: u8 = 0;
    programlabel();
    state.output.generatedLength = 0;
    i = 0;
    while *str.offset(i as isize) != 0 {
        if !(*str.offset(i as isize) as i32 == ' ' as i32) {
            result = (
                (gethexdig(*str.offset(i as isize) as i32) << 4)
                +
                gethexdig(*str.offset((i + 1) as isize) as i32)
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
pub unsafe extern "C" fn gethexdig(mut c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 { return c - '0' as i32 }
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10
    }
    let buffer = format!("Bad Hex Digit {}", c as u8 as char);
    asmerr(AsmErrorEquates::SyntaxError, false, transient::string_to_str_pointer(buffer));
    println!("(Must be a valid hex digit)");
    filesystem::writeln_to_file_maybe(&mut state.output.listFile, "(Must be a valid hex digit)");
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn v_err(mut _str: *mut i8,
                               mut _dummy: *mut _MNE) {
    programlabel();
    asmerr(AsmErrorEquates::ErrPseudoOpEncountered,
           true, 0 as *const i8);
    std::process::exit(1);
}
#[no_mangle]
pub unsafe extern "C" fn v_dc(mut str: *mut i8,
                              mut mne: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut tmp: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut value: i64 = 0;
    let mut macstr: *mut i8 = 0 as *mut i8;
    let mut vmode: i8 = 0;
    state.output.generatedLength = 0;
    programlabel();
    /* for byte, .byte, word, .word, long, .long */
    if *(*mne).name.offset(0 as isize) as i32 !=
           'd' as i32 {
        static mut sTmp: [i8; 4] = [0; 4];
        strcpy(sTmp.as_mut_ptr(),
               b"x.x\x00" as *const u8 as *const i8);
        sTmp[2 as usize] =
            *(*mne).name.offset(0 as isize);
        findext(sTmp.as_mut_ptr());
    }
    /* F8... */
    /* db, dw, dd */
    if *(*mne).name.offset(0 as isize) as i32 ==
           'd' as i32 &&
           *(*mne).name.offset(1 as isize) as i32 !=
               'c' as i32 &&
           *(*mne).name.offset(1 as isize) as i32 !=
               'v' as i32 {
        static mut sTmp_0: [i8; 4] = [0; 4];
        strcpy(sTmp_0.as_mut_ptr(),
               b"x.x\x00" as *const u8 as *const i8);
        if 'd' as i32 ==
               *(*mne).name.offset(1 as isize) as i32 {
            sTmp_0[2 as usize] = 'l' as i32 as i8
        } else {
            sTmp_0[2 as usize] =
                *(*mne).name.offset(1 as isize)
        }
        findext(sTmp_0.as_mut_ptr());
    }
    /* ...F8 */
    if *(*mne).name.offset(1 as isize) as i32 ==
           'v' as i32 {
        let mut i: i32 = 0;
        vmode = 1;
        i = 0;
        while *str.offset(i as isize) as i32 != 0 &&
                  *str.offset(i as isize) as i32 != ' ' as i32 {
            i += 1
        }
        tmp = findsymbol(str, i);
        str = str.offset(i as isize);
        if tmp.is_null() {
            println!("EQM label not found");
            return
        }
        if (*tmp).flags as i32 & 0x20 as i32 != 0 {
            macstr = (*tmp).string as *mut libc::c_void as *mut i8
        } else {
            println!("must specify EQM label for DV");
            return
        }
    }
    sym = eval(str, 0);
    while !sym.is_null() {
        value = (*sym).value;
        if (*sym).flags as i32 & 0x1 as i32 != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::DCNotResolved
        }
        if (*sym).flags as i32 & 0x8 as i32 != 0 {
            let mut ptr: *mut u8 =
                (*sym).string as *mut libc::c_void as *mut u8;
            loop  {
                value = *ptr as i64;
                if !(value != 0) { break ; }
                if vmode != 0 {
                    setspecial(value as i32, 0);
                    tmp = eval(macstr, 0);
                    value = (*tmp).value;
                    if (*tmp).flags as i32 & 0x1 as i32 != 0 {
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
                setspecial(value as i32, (*sym).flags as i32);
                tmp = eval(macstr, 0);
                value = (*tmp).value;
                if (*tmp).flags as i32 & 0x1 as i32 != 0 {
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::DVNotResolvedCould
                }
                FreeSymbolList(tmp);
            }
            match state.execution.modeNext {
                AddressModes::WordAdr => {
                    // Any value outside two's complement +ve and +ve word representation is invalid...
                    if state.parameters.strictMode && (value < -0xffff || value > 0xffff) {
                        let buffer = format!(
                            "{} {}",
                            transient::str_pointer_to_string((*mne).name),
                            value,
                        );
                        asmerr(AsmErrorEquates::AddressMustBeLowerThan10000, false, transient::string_to_str_pointer(buffer));
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
                    if value < -0xff || value > 0xff  {
                        let buffer = format!(
                            "{} {}",
                            transient::str_pointer_to_string((*mne).name),
                            value,
                        );
                        asmerr(AsmErrorEquates::AddressMustBeLowerThan100, false, transient::string_to_str_pointer(buffer));
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
pub unsafe extern "C" fn v_ds(mut str: *mut i8,
                              mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut mult: i32 = 1;
    let mut filler: i64 = 0;
    if state.execution.modeNext == AddressModes::WordAdr { mult = 2 }
    if state.execution.modeNext == AddressModes::Long { mult = 4 }
    programlabel();
    sym = eval(str, 0);
    if !sym.is_null() {
        if !(*sym).next.is_null() { filler = (*(*sym).next).value }
        if (*sym).flags as i32 & 0x1 as i32 != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::DSNotResolved
        } else {
            if !(*sym).next.is_null() &&
                   (*(*sym).next).flags as i32 & 0x1 as i32 !=
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
pub unsafe extern "C" fn v_org(mut str: *mut i8,
                               mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    sym = eval(str, 0);
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.org = (*sym).value as u64;
    if (*sym).flags & SymbolTypes::Unknown != 0 {
        currentSegment.flags |= SymbolTypes::Unknown;
    } else {
        currentSegment.flags &= !SymbolTypes::Unknown;
    }
    if currentSegment.initflags & SymbolTypes::Unknown != 0 {
        currentSegment.initorg = (*sym).value as u64;
        currentSegment.initflags = (*sym).flags;
    }
    if !(*sym).next.is_null() {
        state.output.orgFill = (*(*sym).next).value as u8;
        if (*(*sym).next).flags & SegmentTypes::Unknown != 0 {
            asmerr(AsmErrorEquates::ValueUndefined, true, 0 as *const i8);
        }
    }
    programlabel();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_rorg(mut str: *mut i8,
                                mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.flags |= SegmentTypes::RelocatableOrigin;
    if (*sym).addrmode != AddressModes::Imp as u8 {
        currentSegment.rorg = (*sym).value as u64;
        if (*sym).flags & SymbolTypes::Unknown != 0 {
            currentSegment.rflags |= SymbolTypes::Unknown;
        } else {
            currentSegment.rflags &= !SymbolTypes::Unknown;
        }
        if currentSegment.initrflags & SymbolTypes::Unknown != 0 {
            currentSegment.initrorg = (*sym).value as u64;
            currentSegment.initrflags = (*sym).flags
        }
    }
    programlabel();
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_rend(mut _str: *mut i8,
                                mut _dummy: *mut _MNE) {
    programlabel();
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    currentSegment.flags &= !SegmentTypes::RelocatableOrigin;
}
#[no_mangle]
pub unsafe extern "C" fn v_align(mut str: *mut i8,
                                 mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    let mut fill: u8 = 0;
    let mut rorg: u8 = currentSegment.flags & SegmentTypes::RelocatableOrigin;
    if rorg != 0 {
        currentSegment.rflags |= SegmentTypes::Referenced;
    } else {
        currentSegment.flags |= SegmentTypes::Referenced;
    }
    if !(*sym).next.is_null() {
        if (*(*sym).next).flags & SymbolTypes::Unknown != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::AlignNotResolved
        } else {
            fill = (*(*sym).next).value as u8
        }
    }
    if rorg != 0 {
        if (currentSegment.rflags | (*sym).flags) & SymbolTypes::Unknown != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::AlignRelocatableOriginNotKnown
        } else {
            let mut n: i64 = ((*sym).value as u64).wrapping_sub(currentSegment.rorg.wrapping_rem((*sym).value as u64)) as i64;
            if n != (*sym).value {
                genfill(fill as i64, n, 1);
            }
        }
    } else if (currentSegment.flags | (*sym).flags) & SymbolTypes::Unknown != 0 {
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::AlignNormalOriginNotKnown
    } else {
        let mut n_0: i64 = ((*sym).value as u64).wrapping_sub(currentSegment.org.wrapping_rem((*sym).value as u64)) as i64;
        if n_0 != (*sym).value {
            genfill(fill as i64, n_0, 1);
        }
    }
    FreeSymbolList(sym);
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_subroutine(mut _str: *mut i8,
                                      mut _dummy: *mut _MNE) {
    Lastlocalindex = Lastlocalindex.wrapping_add(1);
    Localindex = Lastlocalindex;
    programlabel();
}
#[no_mangle]
pub unsafe extern "C" fn v_equ(mut str: *mut i8,
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
            *(*Av.as_mut_ptr().offset(0)).offset(0) as i32 == '.' as i32 ||
            *(*Av.as_mut_ptr().offset(0)).offset(0) as i32 == '*' as i32 && {
                let ref mut fresh32 = *(*Av.as_mut_ptr().offset(0)).offset(0);
                *fresh32 = '.' as i32 as i8;
                (*fresh32 as i32) != 0
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
    lab = findsymbol(*Av.as_mut_ptr().offset(0), strlen(*Av.as_mut_ptr().offset(0)) as i32);
    if lab.is_null() {
        lab = CreateSymbol(*Av.as_mut_ptr().offset(0), strlen(*Av.as_mut_ptr().offset(0)) as i32)
    }
    if (*lab).flags as i32 & SymbolTypes::Unknown as i32 == 0 {
        if (*sym).flags as i32 & SymbolTypes::Unknown as i32 != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::EquNotResolved
        } else if (*lab).value != (*sym).value {
            asmerr(AsmErrorEquates::EquValueMismatch, false, 0 as *const i8);
            println!(
                "INFO: Label '{}' changed from ${:04x} to ${:04x}",
                transient::str_pointer_to_string(*Av.as_mut_ptr().offset(0)),
                (*lab).value,
                (*sym).value,
            );
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::EquValueMismatch
        }
    }
    (*lab).value = (*sym).value;
    (*lab).flags =
        ((*sym).flags as i32 &
             (SegmentTypes::Unknown as i32 | SymbolTypes::StringResult as i32)) as u8;
    (*lab).string = (*sym).string;
    (*sym).flags =
        ((*sym).flags as i32 &
             !(0x8 as i32 | 0x20 as i32)) as u8;
    /* List the value */
    let mut v: u64 = (*lab).value as u64;
    state.output.generatedLength = 0;
    if v > 0xffff as i32 as u64 {
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
pub unsafe extern "C" fn v_eqm(mut str: *mut i8,
                               mut _dummy: *mut _MNE) {
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut len: i32 =
        strlen(*Av.as_mut_ptr().offset(0 as isize)) as i32;
    lab = findsymbol(*Av.as_mut_ptr().offset(0 as isize), len);
    if !lab.is_null() {
        if (*lab).flags as i32 & 0x8 as i32 != 0 {
            free((*lab).string as *mut libc::c_void);
        }
    } else {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as isize),
                         len)
    }
    (*lab).value = 0;
    (*lab).flags =
        (0x8 as i32 | 0x10 as i32 | 0x20 as i32) as u8;
    (*lab).string =
        strcpy(ckmalloc(strlen(str).wrapping_add(1 as u64) as i32), str);
}
#[no_mangle]
pub unsafe extern "C" fn v_echo(mut str: *mut i8,
                                mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = eval(str, 0);
    let mut s: *mut _SYMBOL = 0 as *mut _SYMBOL;

    s = sym;
    while !s.is_null() {
        if (*s).flags & SymbolTypes::Unknown == 0 {
            let mut buffer: String = String::new();
            if (*s).flags & (SymbolTypes::Macro | SymbolTypes::StringResult) != 0 {
                buffer.push_str(transient::str_pointer_to_string((*s).string).as_str());
            } else {
                buffer.push_str(format!("${:x}", (*s).value).as_str());
            }
            filesystem::write_to_file_maybe(
                &mut state.output.listFile,
                format!(" {}", buffer).as_str(),
            );
            addmsg(b" \x00" as *const u8 as *const i8 as *mut i8); // -FXQ supress output until final pass
            addmsg(transient::string_to_str_pointer(buffer));
        }
        s = (*s).next
    }
    addmsg(b"\n\x00" as *const u8 as *const i8 as *mut i8);
    filesystem::writeln_to_file_maybe(&mut state.output.listFile, "");
}
#[no_mangle]
pub unsafe extern "C" fn v_set(mut str: *mut i8,
                               mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut lab: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut dynamicname: [i8; 257] = [0; 257];
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut setundefined: i32 = 0;
    while *str.offset(i as isize) as i32 != 0 &&
              *str.offset(i as isize) as i32 != '\"' as i32 &&
              *str.offset(i as isize) as i32 != ' ' as i32 &&
              *str.offset(i as isize) as i32 != ',' as i32 {
        i += 1
    }
    if *str.offset(i as isize) as i32 != 0 &&
           *str.offset(i as isize) as i32 == ',' as i32 {
        // is this SET is using the "," eval-concat operator?
        strncpy(dynamicname.as_mut_ptr(), str, 256);
        if i < 256 {
            dynamicname[i] = 0;
        }
        dynamicname[256] = 0;
        j = strlen(dynamicname.as_mut_ptr()) as usize;
        // eval-concat argument processing loop...
        while *str.offset(i as isize) as i32 != 0 &&
                  *str.offset(i as isize) as i32 != '\"' as i32 &&
                  *str.offset(i as isize) as i32 != ' ' as i32 {
            if *str.offset(i as isize) as i32 == 0 ||
                   *str.offset(i as isize) as i32 == ' ' as i32 {
                break ;
                // process any remaining arguments
            } // argument was symbol
            if *str.offset((i + 1) as isize) as i32 == '\"' as i32 {
                // is this a string constant?
                i = i + 2; // argument was string constant
                while *str.offset(i as isize) as i32 != 0 &&
                          *str.offset(i as isize) as i32 !=
                              '\"' as i32 &&
                          *str.offset(i as isize) as i32 != ' ' as i32
                          &&
                          *str.offset(i as isize) as i32 != ',' as i32
                      {
                    let fresh37 = i; // advance to string contents
                    i = i + 1;
                    let fresh38 = j;
                    j = j + 1;
                    dynamicname[fresh38] = *str.offset(fresh37 as isize)
                }
                if *str.offset(i as isize) as i32 != 0 &&
                       *str.offset(i as isize) as i32 == '\"' as i32 {
                    dynamicname[j as usize] = 0;
                    i += 1
                } else {
                    asmerr(AsmErrorEquates::SyntaxError, false, str);
                }
            } else {
                // this argument is a symbol to be evaluated
                let mut t: i32 = 0;
                let mut tempbuf: [i8; 257] = [0; 257];
                let mut tempval: [i8; 257] = [0; 257];
                let mut symarg: *mut _SYMBOL = 0 as *mut _SYMBOL;
                strncpy(tempbuf.as_mut_ptr(),
                        str.offset(i as
                                       isize).offset(1 as
                                                         isize),
                        256 as i32 as u64);
                tempbuf[256 as i32 as usize] =
                    0;
                t = 0;
                while (t as u64) < strlen(tempbuf.as_mut_ptr()) {
                    if tempbuf[t as usize] as i32 == ',' as i32 {
                        tempbuf[t as usize] = 0
                    }
                    t += 1
                }
                symarg = eval(tempbuf.as_mut_ptr(), 0);
                if !symarg.is_null() {
                    if (*symarg).flags as i32 & 0x1 as i32 !=
                           0 {
                        // one of the arguments isn't defined yet
                        setundefined += 1
                    } else {
                        snprintf(tempval.as_mut_ptr(),
                                 256 as i32 as u64,
                                 b"%d\x00" as *const u8 as
                                     *const i8,
                                 (*symarg).value as u32); // ensure the set doesn't actually happen
                        strcpy(dynamicname.as_mut_ptr().offset(j as isize),
                               tempval.as_mut_ptr());
                        j = (j).wrapping_add(strlen(tempval.as_mut_ptr()) as usize);
                    }
                }
                i += 1;
                while *str.offset(i as isize) as i32 != 0 &&
                          *str.offset(i as isize) as i32 != ' ' as i32
                          &&
                          *str.offset(i as isize) as i32 != ',' as i32
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
                   strlen(*Av.as_mut_ptr().offset(0 as isize)) as i32);
    if lab.is_null() {
        lab =
            CreateSymbol(*Av.as_mut_ptr().offset(0 as isize),
                         strlen(*Av.as_mut_ptr().offset(0 as
                                                            isize)) as i32)
    }
    (*lab).value = (*sym).value;
    (*lab).flags =
        ((*sym).flags as i32 &
             (0x1 as i32 | 0x8 as i32)) as u8;
    (*lab).string = (*sym).string;
    (*sym).flags =
        ((*sym).flags as i32 &
             !(0x8 as i32 | 0x20 as i32)) as u8;
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_setstr(mut symstr: *mut i8,
                                  mut dummy: *mut _MNE) {
    let mut str: [i8; 1024] = [0; 1024];
    snprintf(str.as_mut_ptr(), 1024 as i32 as u64,
             b"\"%s\"\x00" as *const u8 as *const i8, symstr);
    v_set(str.as_mut_ptr(), dummy);
}
#[no_mangle]
pub unsafe extern "C" fn v_execmac(mut str: *mut i8,
                                   mut mac: *mut _MACRO) {
    let mut inc: *mut _INCFILE = 0 as *mut _INCFILE;
    let mut base: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut psl: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut sl: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut s1: *mut i8 = 0 as *mut i8;
    programlabel();
    if state.execution.macroLevel == MAX_MACRO_LEVEL {
        println!("infinite macro recursion");
        return
    }
    state.execution.macroLevel += 1;
    base = ckmalloc((std::mem::size_of::<*mut _STRLIST>() as u64).wrapping_add(strlen(str)).wrapping_add(1) as i32) as *mut _STRLIST;
    (*base).next = 0 as *mut _STRLIST;
    strcpy((*base).buf.as_mut_ptr(), str);
    psl = &mut (*base).next;
    while *str as i32 != 0 && *str as i32 != '\n' as i32 {
        s1 = str;
        while *str as i32 != 0 && *str as i32 != '\n' as i32
                  && *str as i32 != ',' as i32 {
            str = str.offset(1)
        }
        sl =
            ckmalloc((::std::mem::size_of::<*mut _STRLIST>() as u64).wrapping_add(1 as u64) as i32) as *mut _STRLIST;
        // Conversion note: in the above line, removed additional wrapping data...
        //     .wrapping_add(str.wrapping_offset_from(s1) as i64 as u64)
        // ...because it was relying on allocating more memory than the buffer needed, THEN
        // overrunning the buffer to set it. Instead, the fix just increased the buffer length
        // from 4 to 16 to be sure.
        (*sl).next = 0 as *mut _STRLIST;
        *psl = sl;
        psl = &mut (*sl).next;
        memcpy((*sl).buf.as_mut_ptr() as *mut libc::c_void,
               s1 as *const libc::c_void,
               str.wrapping_offset_from(s1) as i64 as u64);
        // Conversion note: this is the line that was causing a buffer overrun during tests,
        // as the code was trying to read up to 9 (and it's size 4)
        (*sl).buf[str.wrapping_offset_from(s1) as i64 as usize] =
            0;
        if *str as i32 == ',' as i32 { str = str.offset(1) }
        while *str as i32 == ' ' as i32 { str = str.offset(1) }
    }
    inc =
        zmalloc(::std::mem::size_of::<_INCFILE>() as u64 as i32) as *mut _INCFILE;
    (*inc).next = pIncfile;
    (*inc).name = (*mac).name;
    (*inc).fi = (*pIncfile).fi;
    (*inc).lineno = 0;
    (*inc).flags = 0x1 as i32 as u8;
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
pub unsafe extern "C" fn v_end(mut _str: *mut i8,
                               mut _dummy: *mut _MNE) {
    /* Only ENDs current file and any macro calls within it */
    while (*pIncfile).flags as i32 & 0x1 as i32 != 0 {
        v_endm(0 as *mut i8, 0 as *mut _MNE);
    }
    fseek((*pIncfile).fi, 0, 2);
}
#[no_mangle]
pub unsafe extern "C" fn v_endm(mut _str: *mut i8,
                                mut _dummy: *mut _MNE) {
    let mut inc: *mut _INCFILE = pIncfile;
    let mut args: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut an: *mut _STRLIST = 0 as *mut _STRLIST;
    /* programlabel(); contrary to documentation */
    if (*inc).flags as i32 & 0x1 as i32 != 0 {
        state.execution.macroLevel -= 1;
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
pub unsafe extern "C" fn v_mexit(mut _str: *mut i8,
                                 mut _dummy: *mut _MNE) {
    v_endm(0 as *mut i8, 0 as *mut _MNE);
}
#[no_mangle]
pub unsafe extern "C" fn v_ifconst(mut str: *mut i8,
                                   mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    sym = eval(str, 0);
    pushif((*sym).flags as i32 == 0);
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_ifnconst(mut str: *mut i8,
                                    mut _dummy: *mut _MNE) {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    programlabel();
    sym = eval(str, 0);
    pushif((*sym).flags as i32 != 0);
    FreeSymbolList(sym);
}
#[no_mangle]
pub unsafe extern "C" fn v_if(mut str: *mut i8,
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
pub unsafe extern "C" fn v_else(mut _str: *mut i8,
                                mut _dummy: *mut _MNE) {
    if (*Ifstack).acctrue as i32 != 0 &&
           (*Ifstack).flags as i32 & 0x4 as i32 == 0 {
        programlabel();
        (*Ifstack).xtrue =
            ((*Ifstack).xtrue == 0) as i32 as u8
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_endif(mut _str: *mut i8,
                                 mut _dummy: *mut _MNE) {
    let mut ifs: *mut _IFSTACK = Ifstack;
    if (*ifs).flags as i32 & 0x4 as i32 == 0 {
        if (*ifs).acctrue != 0 { programlabel(); }
        if (*ifs).file != pIncfile {
            println!("too many endif\'s");
        } else { Ifstack = (*ifs).next; free(ifs as *mut libc::c_void); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn v_repeat(mut str: *mut i8,
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
               0 as *const i8);
        return
    }
    rp =
        zmalloc(::std::mem::size_of::<_REPLOOP>() as u64 as i32) as *mut _REPLOOP;
    (*rp).next = Reploop;
    (*rp).file = pIncfile;
    if (*pIncfile).flags as i32 & 0x1 as i32 != 0 {
        (*rp).seek = (*pIncfile).strlist as i64 as u64
    } else { (*rp).seek = ftell((*pIncfile).fi) as u64 }
    (*rp).lineno = (*pIncfile).lineno;
    (*rp).count = (*sym).value as u64;
    (*rp).flags = (*sym).flags;
    if (*rp).flags as i32 != 0 {
        state.execution.redoIndex += 1;
        state.execution.redoWhy |= ReasonCodes::RepeatNotResolved
    }
    Reploop = rp;
    FreeSymbolList(sym);
    pushif(true);
}
#[no_mangle]
pub unsafe extern "C" fn v_repend(mut _str: *mut i8,
                                  mut _dummy: *mut _MNE) {
    if (*Ifstack).xtrue == 0 || (*Ifstack).acctrue == 0 {
        v_endif(0 as *mut i8, 0 as *mut _MNE);
        return
    }
    if !Reploop.is_null() && (*Reploop).file == pIncfile {
        if (*Reploop).flags as i32 == 0 &&
               {
                   (*Reploop).count = (*Reploop).count.wrapping_sub(1);
                   ((*Reploop).count) != 0
               } {
            if (*pIncfile).flags as i32 & 0x1 as i32 != 0 {
                (*pIncfile).strlist = (*Reploop).seek as *mut _STRLIST
            } else {
                fseek((*pIncfile).fi, (*Reploop).seek as i64,
                      0);
            }
            (*pIncfile).lineno = (*Reploop).lineno
        } else {
            rmnode(&mut Reploop as *mut *mut _REPLOOP as
                       *mut *mut libc::c_void,
                   ::std::mem::size_of::<_REPLOOP>() as u64 as i32);
            v_endif(0 as *mut i8, 0 as *mut _MNE);
        }
        return
    }
    println!("no repeat");
}
#[no_mangle]
pub unsafe extern "C" fn v_incdir(mut str: *mut i8, mut _dummy: *mut _MNE) {
    let filename = get_filename(transient::str_pointer_to_string(str).as_str()).to_owned();
    if state.execution.includeDirList.iter().find(|dir| dir.cmp(&&filename) == Ordering::Equal).is_none() {
        state.execution.includeDirList.push(filename);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pfopen(mut name: *const i8,
                                mut mode: *const i8) -> *mut FILE {
    // FIXME: replace with filesystem::try_open_file_with_locations()
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(name, mode);
    if !f.is_null() {
        return f;
    }
    /* Don't use the incdirlist for absolute pathnames */
    if !strchr(name, ':' as i32).is_null() {
        return 0 as *mut FILE
    } /*	multiplied later    */
    for incDir in &state.execution.includeDirList {
        let path = filesystem::combine_paths(incDir.as_str(), transient::str_pointer_to_string(name).as_str());
        f = fopen(transient::string_to_str_pointer(path), mode);
        if !f.is_null() {
            break;
        }
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn generate() {
    let mut seekpos: i64 = 0;
    static mut org: u64 = 0;
    let mut i: i32 = 0;
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    if state.execution.redoIndex == 0 {
        if currentSegment.flags & SegmentTypes::BSS == 0 {
            i = state.output.generatedLength as i32 - 1;
            while i >= 0 {
                // In practice, this will never wrap since the value is never high enough, but just in case,
                // we use "wrapping_add()" instead of "+="
                state.output.checksum = state.output.checksum.wrapping_add(state.output.generated[i as usize] as u64);
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
                    filesystem::write_buffer_to_file_maybe(
                        &mut state.output.outFile,
                        &[(org & 0xff) as u8, (org >> 8 & 0xff) as u8]
                    );
                    if state.parameters.format == Format::Ras {
                        state.output.seekBack = filesystem::get_stream_position_maybe(&mut state.output.outFile) as i64;
                        state.output.segmentLength = 0;
                        filesystem::write_buffer_to_file_maybe(&mut state.output.outFile, &[0u8, 0u8]);
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
                               0 as *const i8);
                        std::process::exit(1);
                    }
                    while currentSegment.org != org {
                        filesystem::write_char_to_file_maybe(&mut state.output.outFile, state.output.orgFill as char);
                        org = org.wrapping_add(1)
                    }
                    filesystem::write_buffer_to_file_maybe(
                        &mut state.output.outFile,
                        &state.output.generated[0..state.output.generatedLength]
                    );
                }
                Format::Ras => {
                    if org != currentSegment.org {
                        org = currentSegment.org;
                        seekpos = filesystem::get_stream_position_maybe(&mut state.output.outFile) as i64;
                        filesystem::seek_maybe(&mut state.output.outFile, state.output.seekBack as u64);
                        filesystem::write_buffer_to_file_maybe(
                            &mut state.output.outFile,
                            &[(state.output.segmentLength & 0xff) as u8, (state.output.segmentLength >> 8 & 0xff) as u8]
                        );
                        filesystem::seek_maybe(&mut state.output.outFile, seekpos as u64);
                        filesystem::write_buffer_to_file_maybe(
                            &mut state.output.outFile,
                            &[(org & 0xff) as u8, (org >> 8 & 0xff) as u8]
                        );
                        state.output.seekBack = filesystem::get_stream_position_maybe(&mut state.output.outFile) as i64;
                        state.output.segmentLength = 0;
                        filesystem::write_buffer_to_file_maybe(&mut state.output.outFile, &[0u8, 0u8]);
                    }
                    filesystem::write_buffer_to_file_maybe(
                        &mut state.output.outFile,
                        &state.output.generated[0..state.output.generatedLength]
                    );
                    state.output.segmentLength += state.output.generatedLength;
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
            filesystem::seek_maybe(&mut state.output.outFile, state.output.seekBack as u64);
            filesystem::write_buffer_to_file_maybe(
                &mut state.output.outFile,
                &[(state.output.segmentLength & 0xff) as u8, (state.output.segmentLength >> 8 & 0xff) as u8]
            );
            filesystem::seek_end_maybe(&mut state.output.outFile, 0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn genfill(mut fill: i64,
                                 mut entries: i64,
                                 mut size: i32) {
    let mut bytes: i64 = entries;
    let mut i: i32 = 0;
    let mut c3: u8 = 0;
    let mut c2: u8 = 0;
    let mut c1: u8 = 0;
    let mut c0: u8 = 0;
    if bytes == 0 { return }
    c3 = (fill >> 24) as u8;
    c2 = (fill >> 16) as u8;
    c1 = (fill >> 8) as u8;
    c0 = fill as u8;
    match size {
        1 => {
            memset(state.output.generated.as_mut_ptr() as *mut libc::c_void, c0 as i32,
                   ::std::mem::size_of::<[u8; 1024]>() as u64);
        }
        2 => {
            bytes <<= 1;
            i = 0;
            while (i as u64) <
                      ::std::mem::size_of::<[u8; 1024]>() as u64 {
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
            while (i as u64) <
                      ::std::mem::size_of::<[u8; 1024]>() as u64 {
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
    state.output.generatedLength = ::std::mem::size_of::<[u8; 1024]>();
    while bytes as u64 > ::std::mem::size_of::<[u8; 1024]>() as u64 {
        generate();
        bytes = (bytes as u64)
            .wrapping_sub(::std::mem::size_of::<[u8; 1024]>() as u64) as i64 as i64
    }
    state.output.generatedLength = bytes as usize;
    generate();
}
#[no_mangle]
pub unsafe extern "C" fn pushif(mut xbool: bool) {
    let mut ifs: *mut _IFSTACK =
        zmalloc(::std::mem::size_of::<_IFSTACK>() as u64 as i32) as *mut _IFSTACK;
    (*ifs).next = Ifstack;
    (*ifs).file = pIncfile;
    (*ifs).flags = 0;
    (*ifs).xtrue = xbool as u8;
    (*ifs).acctrue =
        ((*Ifstack).acctrue as i32 != 0 &&
             (*Ifstack).xtrue as i32 != 0) as i32 as u8;
    Ifstack = ifs;
}
