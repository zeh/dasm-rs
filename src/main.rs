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
#![feature(seek_convenience)]
#![register_tool(c2rust)]

#[macro_use]
extern crate smart_default;

use libc;
use std::convert::TryFrom;
use std::fs::File;

pub mod constants;
pub mod expressions;
pub mod globals;
pub mod macros;
pub mod mnemonics;
pub mod operations;
pub mod processors;
pub mod segments;
pub mod symbols;
pub mod types;
pub mod utils;

use constants::{
    ALLOC_SIZE,
    DASM_ID,
    MAX_LINES,
    S_HASH_SIZE,
};
use globals::state;
use segments::{
    clear_segments,
};
use types::flags:: {
    FileFlags,
    IfFlags,
    ReasonCodes,
    SegmentTypes,
    SymbolTypes,
};
use types::enums::{
    AddressModes,
    AsmErrorEquates,
    ErrorFormat,
    Format,
    ListMode,
    SortMode,
    Verbosity,
};
use types::legacy::{
    __compar_fn_t,
    _INCFILE,
    _MACRO,
    _MNE,
    _STRLIST,
    _SYMBOL,
    align,
    FILE,
};
use types::structs::{
    Segment,
    StackIf,
};
use utils::{
    filesystem,
    find_error_definition,
    formatting,
    hash_string,
    panic,
    transient,
};

extern "C" {
    #[no_mangle]
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    #[no_mangle]
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    #[no_mangle]
    fn v_incdir(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
    #[no_mangle]
    static mut SHash: [*mut _SYMBOL; 0];
    #[no_mangle]
    static mut MHash: [*mut _MNE; 0];
    #[no_mangle]
    static mut pIncfile: *mut _INCFILE;
    #[no_mangle]
    static mut Av: [*mut i8; 0];
    #[no_mangle]
    static mut Avbuf: [i8; 0];
    #[no_mangle]
    static mut Ops: [_MNE; 0];
    #[no_mangle]
    fn v_execmac(str: *mut i8, mac: *mut _MACRO);
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const i8, wantmode: i32) -> *mut _SYMBOL;
    #[no_mangle]
    fn pfopen(_: *const i8, _: *const i8) -> *mut FILE;
    #[no_mangle]
    fn v_eqm(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn programlabel();
    #[no_mangle]
    fn v_set(str: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_mexit(str: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn closegenerate();
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
/*unsigned char     Listing = 1;*/
unsafe extern "C" fn CountUnresolvedSymbols() -> i32 {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut nUnresolved: i32 = 0;
    let mut i: i32 = 0;
    /* Pre-count unresolved symbols */
    i = 0;
    while i < S_HASH_SIZE as i32 {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() {
            if (*sym).flags as i32 & 0x1 as i32 != 0 {
                nUnresolved += 1;
            }
            sym = (*sym).next;
        }
        i += 1;
    }
    return nUnresolved;
}
unsafe extern "C" fn ShowUnresolvedSymbols() -> i32 {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: i32 = 0;
    let nUnresolved: i32 = CountUnresolvedSymbols();
    if nUnresolved != 0 {
        println!("--- Unresolved Symbol List");
        /* Display unresolved symbols */
        i = 0;
        while i < S_HASH_SIZE as i32 {
            sym = *SHash.as_mut_ptr().offset(i as isize);
            while !sym.is_null() {
                if (*sym).flags as i32 & 0x1 as i32 != 0 {
                    println!("{:24} {}",
                        transient::str_pointer_to_string((*sym).name),
                        formatting::segment_address_to_string((*sym).value as u64, (*sym).flags),
                    );
                }
                sym = (*sym).next
            }
            i += 1
        }
        println!("--- {} Unresolved Symbol{}\n",
            nUnresolved,
            if nUnresolved == 1 {
                " "
            } else {
                "s"
            }
        );
    }
    return nUnresolved;
}
unsafe extern "C" fn CompareAlpha(arg1: *const libc::c_void, arg2: *const libc::c_void) -> i32 {
    /* Simple alphabetic ordering comparison function for quicksort */
    let sym1: *const _SYMBOL = *(arg1 as *const *mut _SYMBOL);
    let sym2: *const _SYMBOL = *(arg2 as *const *mut _SYMBOL);
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
unsafe extern "C" fn CompareAddress(arg1: *const libc::c_void, arg2: *const libc::c_void) -> i32 {
    /* Simple numeric ordering comparison function for quicksort */
    let sym1: *const _SYMBOL = *(arg1 as *const *mut _SYMBOL);
    let sym2: *const _SYMBOL = *(arg2 as *const *mut _SYMBOL);
    return ((*sym1).value - (*sym2).value) as i32;
}
// FIXME: update parameters, and move to symbols/mod
/* bTableSort true -> by address, false -> by name [phf] */
// In original C code, part of ShowSymbols()" in main.c
unsafe fn generate_resolved_symbols_list(sorted: bool) -> String {
    let mut result = String::new();
    /* Display sorted (!) symbol table - if it runs out of memory, table will be displayed unsorted */
    let mut symArray: *mut *mut _SYMBOL = 0 as *mut *mut _SYMBOL;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: i32 = 0;
    let mut nSymbols: i32 = 0;
    result.push_str("--- Symbol List");
    /* Sort the symbol list either via name, or by value */
    /* First count the number of symbols */
    i = 0;
    while i < S_HASH_SIZE as i32 {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() { nSymbols += 1; sym = (*sym).next }
        i += 1
    }
    /* Malloc an array of pointers to data */
    symArray = ckmalloc((::std::mem::size_of::<*mut _SYMBOL>() as u64).wrapping_mul(nSymbols as u64) as i32) as *mut *mut _SYMBOL;
    if symArray.is_null() {
        result.push_str(" (unsorted - not enough memory to sort!)\n");
        /* Display complete symbol table */
        i = 0;
        while i < S_HASH_SIZE as i32 {
            sym = *SHash.as_mut_ptr().offset(i as isize);
            while !sym.is_null() {
                result.push_str(format!(
                    "{:24} {}\n",
                    transient::str_pointer_to_string((*sym).name),
                    formatting::segment_address_to_string((*sym).value as u64, (*sym).flags)
                ).as_str());
                sym = (*sym).next
            }
            i += 1
        }
    } else {
        /* Copy the element pointers into the symbol array */
        let mut nPtr: i32 = 0;
        i = 0;
        while i < S_HASH_SIZE as i32 {
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
        if sorted {
            // Sort via address
            result.push_str(" (sorted by address)\n");
            qsort(
                symArray as *mut libc::c_void,
                nPtr as u64,
                ::std::mem::size_of::<*mut _SYMBOL>() as u64,
                Some(
                    CompareAddress as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void
                    ) -> i32
                )
            );
        } else {
            // Sort via name
            result.push_str(" (sorted by symbol)\n");
            qsort(
                symArray as *mut libc::c_void,
                nPtr as u64,
                ::std::mem::size_of::<*mut _SYMBOL>() as u64,
                Some(
                    CompareAlpha as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void
                    ) -> i32
                )
            );
        }
        /* now display sorted list */
        i = 0; /* If a string, display actual string */
        while i < nPtr {
            result.push_str(format!(
                "{:24} {:12}",
                transient::str_pointer_to_string((**symArray.offset(i as isize)).name),
                formatting::segment_address_to_string((**symArray.offset(i as isize)).value as u64, (**symArray.offset(i as isize)).flags)
            ).as_str());
            if (**symArray.offset(i as isize)).flags as i32 & 0x8 as i32 != 0 {
                result.push_str(format!(
                    " \"{}\"",
                    transient::str_pointer_to_string((**symArray.offset(i as isize)).string),
                ).as_str());
            }
            result.push_str("\n");
            i += 1
        }
        free(symArray as *mut libc::c_void);
    }
    result.push_str("--- End of Symbol List.\n");
    result
}

// FIXME: move to symbols/mod.rs
/**
 * Write symbol list to a file.
 * In original C code, part of DumpSymbolTable()" in main.c
 */
unsafe fn dump_symbol_table(sorted: bool) {
    if !state.parameters.symbolsFile.is_empty() {
        let mut file = filesystem::create_new_file(state.parameters.symbolsFile.as_str()).expect(
            format!("Warning: Unable to open Symbol Dump file '{}'", state.parameters.symbolsFile).as_str()
        );
        write_symbols_to_file(&mut file, sorted);
        filesystem::close_file(&mut file);
    }
}

// FIXME: move to symbols/mod.rs
/**
 * Write symbol list to a file.
 * In original C code, part of ShowSymbols()" in main.c
 */
unsafe fn write_symbols_to_file(file: &mut File, sorted: bool) {
    let result = generate_resolved_symbols_list(sorted);
    filesystem::write_to_file(file, result.as_str());
}

// FIXME: move to symbols/mod.rs
/**
 * Write symbol list to stdout.
 * In original C code, part of ShowSymbols()" in main.c
 */
unsafe fn write_symbols_to_stdout(sorted: bool) {
    let result = generate_resolved_symbols_list(sorted);
    print!("{}", result);
}

unsafe extern "C" fn ShowSegments() {
    println!("\n----------------------------------------------------------------------");
    println!(
        "{:24} {:3} {:8} {:8} {:8} {:8}",
        "SEGMENT NAME", "", "INIT PC", "INIT RPC", "FINAL PC", "FINAL RPC",
    );
    for seg in &state.other.segments {
        let bss = if seg.flags & SegmentTypes::BSS != 0 {
            "[u]"
        } else {
            "   "
        };
        // Originally, "%-24s %-3s %-8s %-8s %-8s %-8s"

        // Strange behavior: the commented-out println!() code below SEEMS to be the correct,
        // expected code. For example.asm, with -d1 (or higher), it generates this:
        //
        //    SEGMENT NAME                 INIT PC  INIT RPC FINAL PC FINAL RPC
        //    vector                       fffa     0000 ??? 10000    0000 ???
        //    code                         f000     0000 ??? f076     0000 ???
        //    data                     [u] 0000     0000 ??? 0100     0000 ???
        //    bss                      [u] 0000     0000 ??? c010     0000 ???
        //    INITIAL CODE SEGMENT         0000 ??? 0000 ??? 0000 ??? 0000 ???
        //
        // However.. this is not what the original (C) dasm generates. It outputs this:
        //
        //    SEGMENT NAME                 INIT PC  INIT RPC FINAL PC FINAL RPC
        //    vector                       fffa                            fffa
        //    code                         f000                            f000
        //    data                     [u] 0000                            0000
        //    bss                      [u] 0000                            0000
        //    INITIAL CODE SEGMENT         0000 ????                       0000 ????
        //
        // Dasm's output make no sense to me, but I've decided to emulate it rather than
        // try and have a "fixed" version until I'm sure this is the correct version.
        //
        // "Probably correct" version
        // println!(
        //     "{:24.24} {:3.3} {:8.8} {:8.8} {:8.8} {:8.8}",
        //     seg.name,
        //     bss,
        //     formatting::segment_address_to_string(seg.initorg, seg.initflags),
        //     formatting::segment_address_to_string(seg.initrorg, seg.initrflags),
        //     formatting::segment_address_to_string(seg.org, seg.flags),
        //     formatting::segment_address_to_string(seg.rorg, seg.rflags),
        // );

        // "Probably incorrect" version that replicates dasm
        println!(
            "{:24} {:3.3} {:8} {:8} {:8} {:8}",
            seg.name,
            bss,
            formatting::segment_address_to_string(seg.initorg, seg.initflags),
            "",
            formatting::segment_address_to_string(seg.initorg, seg.initflags),
            "",
        );
    }
    println!("----------------------------------------------------------------------");
    println!("{} references to unknown symbols.", state.execution.redoEval);
    println!("{} events requiring another assembler pass.", state.execution.redoIndex);

    // FIXME: rewrite more succinctly
    if state.execution.redoWhy != 0 {
        if state.execution.redoWhy & ReasonCodes::MnemonicNotResolved != 0 {
            println!(" - Expression in mnemonic not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::Obscure != 0 {
            println!(" - Obscure reason - to be documented :)");
        }
        if state.execution.redoWhy & ReasonCodes::DCNotResolved != 0 {
            println!(" - Expression in a DC not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::DVNotResolvedProbably != 0 {
            println!(" - Expression in a DV not resolved (probably in DV\'s EQM symbol).");
        }
        if state.execution.redoWhy & ReasonCodes::DVNotResolvedCould != 0 {
            println!(" - Expression in a DV not resolved (could be in DV\'s EQM symbol).");
        }
        if state.execution.redoWhy & ReasonCodes::DSNotResolved != 0 {
            println!(" - Expression in a DS not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::AlignNotResolved != 0 {
            println!(" - Expression in an ALIGN not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::AlignRelocatableOriginNotKnown != 0 {
            println!(" - ALIGN: Relocatable origin not known (if in RORG at the time).");
        }
        if state.execution.redoWhy & ReasonCodes::AlignNormalOriginNotKnown != 0 {
            println!(" - ALIGN: Normal origin not known (if in ORG at the time).");
        }
        if state.execution.redoWhy & ReasonCodes::EquNotResolved != 0 {
            println!(" - EQU: Expression not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::EquValueMismatch != 0 {
            println!(" - EQU: Value mismatch from previous pass (phase error).");
        }
        if state.execution.redoWhy & ReasonCodes::IfNotResolved != 0 {
            println!(" - IF: Expression not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::RepeatNotResolved != 0 {
            println!(" - REPEAT: Expression not resolved.");
        }
        if state.execution.redoWhy & ReasonCodes::ForwardReference != 0 {
            println!(" - Label defined after it has been referenced (forward reference).");
        }
        if state.execution.redoWhy & ReasonCodes::PhaseError != 0 {
            println!(" - Label value is different from that of the previous pass (phase error).");
        }
        if state.execution.redoWhy & ReasonCodes::BranchOutOfRange != 0 {
            println!(" - Branch was out of range.");
        }
    }
    println!();
}

unsafe extern "C" fn MainShadow(ac: i32, av: *mut *mut i8, pbTableSort: *mut bool) -> AsmErrorEquates {
    let mut current_block: u64;
    let mut nError: AsmErrorEquates = AsmErrorEquates::None;
    let mut doAllPasses: bool = false;
    let mut buf: [i8; MAX_LINES] = [0; MAX_LINES];
    let mut i: i32 = 0;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut oldRedoIndex: i32 = -1;
    let mut oldRedoWhy: u64 = 0;
    let mut oldRedoEval: i32 = 0;
    addhashtable(Ops.as_mut_ptr());
    state.execution.pass = 1;
    if !(ac < 2) {
        i = 2;
        loop  {
            if !(i < ac) { current_block = 16231175055492490595; break ; }
            if !(*(*av.offset(i as isize)).offset(0) as i32 == '-' as i32 || *(*av.offset(i as isize)).offset(0) as i32 == '/' as i32) {
                current_block = 15878785573848117940;
                break;
            }
            let mut str: *mut i8 = (*av.offset(i as isize)).offset(2);
            // FIXME: use better strings for parsing chars. These are temporary.
            let str_rs = transient::str_pointer_to_string(str);
            match *(*av.offset(i as isize)).offset(1) as u8 as char {
                'E' => {
                    /* TODO: need to improve option parsing and errors for it */
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match ErrorFormat::try_from(digit) {
                                Ok(result) => { state.parameters.errorFormat = result; }
                                Err(_) => { panic("Invalid error format for -E, must be 0, 1, 2"); }
                            }
                        }
                        Err(_) => { panic("Invalid error format for -E, must be 0, 1, 2"); }
                    }
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                'T' => {
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match SortMode::try_from(digit) {
                                Ok(result) => { state.parameters.sortMode = result; }
                                Err(_) => { panic("Invalid sorting mode for -T option, must be 0 or 1"); }
                            }
                        }
                        Err(_) => { panic("Invalid sorting mode for -T option, must be 0 or 1"); }
                    }
                    *pbTableSort = state.parameters.sortMode != SortMode::default();
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                'd' => {
                    let digit = str_rs.parse::<u8>().unwrap_or(0);
                    let d_equals_z = str_rs.to_ascii_lowercase() == "z";
                    state.parameters.debug = digit != 0 || d_equals_z;
                    state.parameters.debug_extended = d_equals_z;
                    println!("Debug trace {}", if state.parameters.debug { "ON" } else { "OFF" });
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                'M' | 'D' => {
                    while *str as i32 != 0 && *str as i32 != '=' as i32 {
                        str = str.offset(1);
                    }
                    if *str as i32 == '=' as i32 {
                        *str = 0;
                        str = str.offset(1);
                    } else {
                        str = b"0\x00" as *const u8 as *const i8 as *mut i8;
                    }
                    let ref mut fresh2 = *Av.as_mut_ptr().offset(0);
                    *fresh2 = (*av.offset(i as isize)).offset(2);
                    if *(*av.offset(i as isize)).offset(1) as i32 == 'M' as i32 {
                        v_eqm(str, 0 as *mut _MNE);
                    } else {
                        v_set(str, 0 as *mut _MNE);
                    }
                    current_block = 17788412896529399552;
                }
                'f' => {
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match Format::try_from(digit) {
                                Ok(result) => { state.parameters.format = result; }
                                Err(_) => { panic("Illegal format specification"); }
                            }
                        }
                        Err(_) => { panic("Illegal format specification"); }
                    }
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                'o' => {
                    state.parameters.outFile = transient::str_pointer_to_string(str);
                    current_block = 15042310719884093888; // FIXME: remove this
                }
                'L' => {
                    state.parameters.listAllPasses = true;
                    current_block = 14976246946730902058; // FIXME: remove this
                }
                'l' => {
                    // FIXME: this is supposed to be `F_listfile = str;` but is
                    // handled by the current_block craziness.
                    current_block = 14976246946730902058; // FIXME: remove this
                }
                'P' => {
                    doAllPasses = true;
                    current_block = 3124391281584211484; // FIXME: remove this
                }
                'p' => {
                    // FIXME: this is supposed to be `nMaxPasses = atoi(str);` but is
                    // handled by the current_block craziness.
                    current_block = 3124391281584211484; // FIXME: remove this
                }
                's' => {
                    state.parameters.symbolsFile = transient::str_pointer_to_string(str);
                    current_block = 15042310719884093888; // FIXME: remove this
                }
                'v' => {
                    // FIXME: simplify this double match
                    match str_rs.parse::<u8>() {
                        Ok(digit) => {
                            match Verbosity::try_from(digit) {
                                Ok(result) => { state.parameters.verbosity = result; }
                                Err(_) => { panic("Illegal verbosity specification"); }
                            }
                        }
                        Err(_) => { panic("Illegal verbosity specification"); }
                    }
                    current_block = 17788412896529399552;
                }
                'I' => {
                    v_incdir(str, 0 as *mut _MNE);
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                'S' => {
                    state.parameters.strictMode = true;
                    current_block = 17788412896529399552; // FIXME: remove this
                }
                _ => { current_block = 15878785573848117940; break ; }
            }
            match current_block {
                14976246946730902058 =>
                /* fall through to 'l' */
                /*  F_listfile  */
                {
                    state.parameters.listFile = transient::str_pointer_to_string(str);
                    current_block = 15042310719884093888;
                }
                3124391281584211484 =>
                /* fall through to 'p' */
                /*  F_passes   */
                {
                    state.parameters.maxPasses = str_rs.parse::<u8>().unwrap_or(0);
                    current_block = 17788412896529399552;
                }
                _ => { }
            }
            match current_block {
                15042310719884093888 => {
                    if *str as i32 == 0 {
                        panic("-o Switch requires file name.");
                    }
                }
                _ => { }
            }
            i += 1;
        }
        match current_block {
            15878785573848117940 => { }
            _ => {
                /*    INITIAL SEGMENT */
                let seg = Segment {
                    name: String::from("INITIAL CODE SEGMENT"),
                    flags: SegmentTypes::Unknown,
                    rflags: SegmentTypes::Unknown,
                    initflags: SegmentTypes::Unknown,
                    initrflags: SegmentTypes::Unknown,
                    org: 0,
                    rorg: 0,
                    initorg: 0,
                    initrorg: 0,
                };
                state.other.segments.clear();
                state.other.segments.push(seg);
                state.other.currentSegment = 0;

                // Top level IF
                state.execution.ifs.push(
                    StackIf {
                        file: 0 as *mut _INCFILE,
                        flags: IfFlags::Base,
                        result_acc: true,
                        result: true,
                    }
                );

                // Ready error and message buffer...
                operations::clear_passbuffer(&mut state.output.passBufferErrors);
                operations::clear_passbuffer(&mut state.output.passBufferMessages);
                loop  {
                    if state.parameters.verbosity != Verbosity::None {
                        println!();
                        println!("START OF PASS: {}", state.execution.pass);
                    }
                    state.execution.lastLocalIndex = 0;
                    state.execution.localIndex = 0;
                    state.execution.lastLocalDollarIndex = 0;
                    state.execution.localDollarIndex = 0;
                    state.execution.isClear = true;
                    state.output.checksum = 0;

                    // Create basic output file
                    if state.parameters.outFile.is_empty() {
                        state.parameters.outFile = String::from("a.out");
                    }
                    let fileOutOption = filesystem::create_new_file(state.parameters.outFile.as_str());
                    match fileOutOption {
                        Ok(file) => {
                            state.output.outFile = Some(file);
                        },
                        _ => {
                            println!("Warning: Unable to [re]open '{}'", state.parameters.outFile);
                            return AsmErrorEquates::FileError;
                        },
                    }

                    // Create list output file
                    if !state.parameters.listFile.is_empty() {
                        let fileOption = filesystem::create_new_file(state.parameters.listFile.as_str());
                        match fileOption {
                            Ok(file) => {
                                state.output.listFile = Some(file);
                            },
                            _ => {
                                println!("Warning: Unable to [re]open '{}'", state.parameters.listFile);
                                return AsmErrorEquates::FileError;
                            },
                        }
                    }

                    pushinclude(*av.offset(1));
                    while !pIncfile.is_null() {
                        loop  {
                            let mut comment: *const i8 = 0 as *const i8;
                            if (*pIncfile).flags & FileFlags::Macro != 0 {
                                if (*pIncfile).strlist.is_null() {
                                    let ref mut fresh3 = *Av.as_mut_ptr().offset(0);
                                    *fresh3 = b"\x00" as *const u8 as *const i8 as *mut i8;
                                    v_mexit(0 as *mut i8, 0 as *mut _MNE);
                                    continue;
                                } else {
                                    strcpy(buf.as_mut_ptr(), (*(*pIncfile).strlist).buf.as_mut_ptr());
                                    (*pIncfile).strlist = (*(*pIncfile).strlist).next;
                                }
                            } else if fgets(buf.as_mut_ptr(), MAX_LINES as i32, (*pIncfile).fi).is_null() {
                                break;
                            }
                            if state.parameters.debug {
                                println!("{:08x} {}", pIncfile as u64, transient::str_pointer_to_string(buf.as_mut_ptr()));
                            }
                            comment = cleanup(buf.as_mut_ptr(), false);
                            (*pIncfile).lineno = (*pIncfile).lineno.wrapping_add(1);
                            mne = parse(buf.as_mut_ptr());
                            let current_if = &state.execution.ifs.last().unwrap();
                            #[cfg(debug_assertions)]
                            {
                                if state.parameters.debug_extended {
                                    log_function_with!("current_if = {} {}", current_if.result, current_if.result_acc);
                                }
                            }
                            if *(*Av.as_ptr().offset(1)).offset(0) != 0 {
                                if !mne.is_null() {
                                    if (*mne).flags as i32 & 0x4 as i32 != 0 || current_if.result && current_if.result_acc {
                                        #[cfg(debug_assertions)]
                                        {
                                            if state.parameters.debug_extended {
                                                log_function_with!("calling vect on [[{}]] [[{}]]", transient::str_pointer_to_string((*mne).name), transient::str_pointer_to_string(*Av.as_ptr().offset(2)));
                                            }
                                        }
                                        Some((*mne).vect.expect("non-null function pointer")).expect("non-null function pointer")(
                                            *Av.as_ptr().offset(2),
                                            mne
                                        );
                                    }
                                } else if current_if.result && current_if.result_acc {
                                    asmerr(AsmErrorEquates::UnknownMnemonic, false, *Av.as_ptr().offset(1));
                                }
                            } else if current_if.result && current_if.result_acc {
                                programlabel();
                            }
                            if !state.parameters.listFile.is_empty() && state.execution.listMode != ListMode::None {
                                outlistfile(comment);
                            }
                        }
                        while state.execution.repeats.len() > 0 && state.execution.repeats.last().unwrap().file == pIncfile {
                            state.execution.repeats.pop();
                        }
                        while state.execution.ifs.len() > 0 && state.execution.ifs.last().unwrap().file == pIncfile {
                            state.execution.ifs.pop();
                        }
                        fclose((*pIncfile).fi);
                        free((*pIncfile).name as *mut libc::c_void);
                        state.other.incLevel -= 1;
                        rmnode(&mut pIncfile as *mut *mut _INCFILE as *mut *mut libc::c_void, ::std::mem::size_of::<_INCFILE>() as u64 as i32);
                        if !pIncfile.is_null() {
                            /*
                            if (state.parameters.verbosity as u8 > 1)
                            println!("back to: {}", Incfile->name);
                            */
                            filesystem::writeln_to_file_maybe(
                                &mut state.output.listFile,
                                format!(
                                    "------- FILE {}",
                                    transient::str_pointer_to_string((*pIncfile).name),
                                ).as_str(),
                            );
                        }
                    }
                    if state.parameters.verbosity as u8 >= Verbosity::One as u8 {
                        ShowSegments();
                    }
                    if state.parameters.verbosity as u8  >= Verbosity::Three as u8  {
                        if state.execution.redoIndex == 0 || state.parameters.verbosity as u8  >= Verbosity::Four as u8 {
                            write_symbols_to_stdout(*pbTableSort);
                        }
                        ShowUnresolvedSymbols();
                    }
                    closegenerate();
                    filesystem::close_file_maybe(&mut state.output.outFile);
                    filesystem::close_file_maybe(&mut state.output.listFile);
                    if state.execution.redoIndex != 0 {
                        if !doAllPasses {
                            if state.execution.redoIndex == oldRedoIndex && state.execution.redoWhy == oldRedoWhy && state.execution.redoEval == oldRedoEval {
                                ShowUnresolvedSymbols();
                                return AsmErrorEquates::NotResolvable;
                            }
                        }
                        oldRedoIndex = state.execution.redoIndex;
                        oldRedoWhy = state.execution.redoWhy;
                        oldRedoEval = state.execution.redoEval;
                        state.execution.redoIndex = 0;
                        state.execution.redoWhy = 0;
                        state.execution.redoEval = 0;
                        state.execution.redoIf <<= 1;
                        state.execution.pass += 1;
                        if state.execution.pass > state.parameters.maxPasses {
                            let buffer = format!("{}", state.execution.pass);
                            return asmerr(AsmErrorEquates::TooManyPasses, false, transient::string_to_str_pointer(buffer));
                        } else {
                            operations::clear_passbuffer(&mut state.output.passBufferErrors);
                            operations::clear_passbuffer(&mut state.output.passBufferMessages);
                            clearrefs();
                            clear_segments(&mut state.other.segments);
                        }
                    } else {
                        // Do not print any errors if assembly is successful!!!!! -FXQ
                        // only print messages from last pass and if there's no errors
                        if !state.other.stopAtEnd {
                            operations::output_passbuffer(&mut state.output.passBufferMessages);
                        } else {
                            // Only print errors if assembly is unsuccessful!!!!!
                            // by FXQ
                            operations::output_passbuffer(&mut state.output.passBufferErrors);
                            println!("Unrecoverable error(s) in pass, aborting assembly!");
                            nError = AsmErrorEquates::NonAbort;
                        }
                        println!("Complete.");
                        return nError;
                    }
                }
            }
        }
    }
    println!("{}", DASM_ID);
    println!("Copyright (c) 1988-2020 by the DASM team.");
    println!("License GPLv2+: GNU GPL version 2 or later (see file LICENSE).");
    println!("DASM is free software: you are free to change and redistribute it.");
    println!("There is ABSOLUTELY NO WARRANTY, to the extent permitted by law.");
    println!();
    println!("Usage: dasm sourcefile [options]");
    println!();
    println!("-f#      output format 1-3 (default 1)");
    println!("-oname   output file name (else a.out)");
    println!("-lname   list file name (else none generated)");
    println!("-Lname   list file, containing all passes");
    println!("-sname   symbol dump file name (else none generated)");
    println!("-v#      verboseness 0-4 (default 0)");
    println!("-d       debug mode (for developers)");
    println!("-Dsymbol              define symbol, set to 0");
    println!("-Dsymbol=expression   define symbol, set to expression");
    println!("-Msymbol=expression   define symbol using EQM (same as -D)");
    println!("-Idir    search directory for INCLUDE and INCBIN");
    println!("-p#      maximum number of passes");
    println!("-P#      maximum number of passes, with fewer checks");
    println!("-T#      symbol table sorting (default 0 = alphabetical, 1 = address/value)");
    println!("-E#      error format (default 0 = MS, 1 = Dillon, 2 = GNU)");
    println!("-S       strict syntax checking");
    println!();
    println!("Report bugs on https://github.com/dasm-assembler/dasm please!");
    return AsmErrorEquates::CommandLine;
}
unsafe extern "C" fn outlistfile(comment: *const i8) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(comment)); } }

    let current_if = &state.execution.ifs.last().unwrap();
    let mut xtrue: char = 0 as char;
    let mut c: char = 0 as char;
    let mut buffer: String = String::new();
    let mut i: usize = 0;
    if (*pIncfile).flags & FileFlags::NoList != 0 {
        return;
    }
    xtrue = if current_if.result && current_if.result_acc {
        ' '
    } else {
        '-'
    };
    c = if state.execution.programFlags & SegmentTypes::BSS != 0 {
        'U'
    } else {
        ' '
    };
    let mut dot = String::from("");
    if !state.execution.extraString.is_empty() {
        dot.push_str(".");
    } else {
        state.execution.extraString.clear();
    }
    buffer.push_str(
        format!(
            "{:7} {}{}",
            (*pIncfile).lineno,
            c,
            formatting::segment_address_to_string(state.execution.programOrg, state.execution.programFlags & 7),
        ).as_str()
    );
    i = 0;
    while i < state.output.generatedLength && i < 4 {
        buffer.push_str(
            format!(
                "{:02x} ",
                state.output.generated[i]
            ).as_str()
        );
        i += 1;
    }
    if i < state.output.generatedLength && i == 4 {
        xtrue = '*'
    }
    while i < 4 {
        buffer.push_str("   ");
        i += 1
    }
    buffer.pop();
    buffer.push_str(
        format!(
            "{}{:10} {}{}{}\t{}\n",
            xtrue,
            transient::str_pointer_to_string(*Av.as_ptr().offset(0)),
            transient::str_pointer_to_string(*Av.as_ptr().offset(1)),
            dot,
            state.execution.extraString.clone(),
            transient::str_pointer_to_string(*Av.as_ptr().offset(2)),
        ).as_str()
    );
    if *comment.offset(0) != 0 {
        /*  tab and comment */
        buffer.pop();
        buffer.push_str(
            format!(
                "\t;{}",
                transient::str_pointer_to_string(comment)
            ).as_str()
        );
    }
    filesystem::write_to_file_maybe(
        &mut state.output.listFile,
        formatting::format_line_tabs(buffer.as_str()).as_str(),
    );
    state.output.generatedLength = 0;
    state.execution.extraString.clear();
}
#[no_mangle]
pub unsafe extern "C" fn clearrefs() {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: i16 = 0;
    i = 0;
    while (i as i32) < S_HASH_SIZE as i32 {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() {
            (*sym).flags = ((*sym).flags as i32 & !(0x4 as i32)) as u8;
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
unsafe extern "C" fn cleanup(buf: *mut i8, bDisable: bool) -> *const i8 {
    let mut str: *mut i8 = 0 as *mut i8;
    let mut strlist: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut arg: i32 = 0;
    let mut add: i32 = 0;
    let mut comment: *const i8 = b"\x00" as *const u8 as *const i8;
    str = buf;
    while *str != 0 {
        match *str as u8 as char {
            ';' => {
                comment = str.offset(1); break;
            }
            '\r' | '\n' => {
                break;
            }
            '\t' => {
                *str = ' ' as i32 as i8;
            }
            '\'' => {
                str = str.offset(1);
                if *str as i32 == '\t' as i32 {
                    *str = ' ' as i32 as i8;
                }
                if *str as i32 == '\n' as i32 || *str as i32 == 0 {
                    *str.offset(0) = ' ' as i32 as i8;
                    *str.offset(1) = 0;
                }
                if *str.offset(0) as i32 == ' ' as i32 {
                    *str.offset(0) = -128i32 as i8;
                }
            }
            '"' => {
                str = str.offset(1);
                while *str as i32 != 0 && *str as i32 != '\"' as i32 {
                    if *str as i32 == ' ' as i32 {
                        *str = -128i32 as i8
                    }
                    str = str.offset(1)
                }
                if *str as i32 != '\"' as i32 {
                    asmerr(AsmErrorEquates::SyntaxError, false, buf);
                    str = str.offset(-1)
                }
            }
            '{' => {
                if !bDisable {
                    if state.parameters.debug {
                        println!("macro tail: '{}'", transient::str_pointer_to_string(str));
                    }
                    // FIXME: this should be:
                    //   arg = transient::str_pointer_to_string(str.offset(1)).parse::<i32>().unwrap_or(0);
                    // But somehow it's creating problems below. It parses correctly but
                    // truncates the strlist string?!
                    arg = strtol(str.offset(1), 0 as *mut libc::c_void as *mut *mut i8, 10) as i32;
                    add = 0;
                    while *str as i32 != 0 && *str as i32 != '}' as i32 {
                        add -= 1;
                        str = str.offset(1)
                    }
                    if *str as i32 != '}' as i32 {
                        println!("end brace required");
                        str = str.offset(-1)
                    } else {
                        add -= 1;
                        str = str.offset(1);
                        if state.parameters.debug {
                            println!("add/str: {} '{}'", add, transient::str_pointer_to_string(str));
                        }
                        strlist = (*pIncfile).args;
                        while arg != 0 && !strlist.is_null() {
                            arg -= 1;
                            strlist = (*strlist).next
                        }
                        if !strlist.is_null() {
                            add = (add as u64).wrapping_add(strlen((*strlist).buf.as_mut_ptr())) as i32;
                            if state.parameters.debug {
                                println!(
                                    "strlist: '{}' {}",
                                    transient::str_pointer_to_string((*strlist).buf.as_mut_ptr()),
                                    strlen((*strlist).buf.as_mut_ptr())
                                );
                            }
                            if str.offset(add as isize).offset(strlen(str) as isize).offset(1) > buf.offset(MAX_LINES as isize) {
                                if state.parameters.debug {
                                    println!(
                                        "str {:8} buf {:8} (add/strlen(str)): {} {}",
                                        str as u64,
                                        buf as u64,
                                        add,
                                        strlen(str) as i64
                                    );
                                }
                                panic("failure1");
                            }
                            memmove(str.offset(add as isize) as *mut libc::c_void, str as *const libc::c_void, strlen(str).wrapping_add(1));
                            str = str.offset(add as isize);
                            if str.offset(-(strlen((*strlist).buf.as_mut_ptr()) as isize)) < buf {
                                panic("failure2");
                            }
                            memmove(str.offset(-(strlen((*strlist).buf.as_mut_ptr()) as isize)) as *mut libc::c_void, (*strlist).buf.as_mut_ptr() as *const libc::c_void, strlen((*strlist).buf.as_mut_ptr()));
                            str = str.offset(-(strlen((*strlist).buf.as_mut_ptr()) as isize));
                            if str < buf || str >= buf.offset(MAX_LINES as isize) {
                                panic("failure 3");
                            }
                            str = str.offset(-1)
                            /*  for loop increments string    */
                        } else {
                            asmerr(AsmErrorEquates::NotEnoughArgumentsPassedToMacro, false, 0 as *const i8);
                            break;
                        }
                    }
                }
            }
            _ => { }
        }
        str = str.offset(1)
    }
    /*    FALL THROUGH    */
    while str != buf && *str.offset(-(1)) as i32 == ' ' as i32 {
        str = str.offset(-1)
    }
    *str = 0;
    return comment;
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
pub unsafe extern "C" fn findext(mut str: *mut i8) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    state.execution.modeNext = AddressModes::None;
    state.execution.extraString.clear();
    if *str.offset(0) as i32 == '.' as i32 {
        /* Allow .OP for OP */
        return;
    }
    while *str as i32 != 0 && *str as i32 != '.' as i32 {
        str = str.offset(1)
    }
    if *str != 0 {
        *str = 0;
        str = str.offset(1);
        state.execution.extraString = transient::str_pointer_to_string(str);

        #[cfg(debug_assertions)]
        { if state.parameters.debug_extended { log_function_with!("state.execution.extraString = [[{}]]", state.execution.extraString); } }

        match (*str.offset(0) as u8 | 0x20) as char {
            '0' | 'i' => {
                state.execution.modeNext = AddressModes::Imp;
                match (*str.offset(1) as u8 | 0x20) as char {
                    'x' => { state.execution.modeNext = AddressModes::ZeroX; }
                    'y' => { state.execution.modeNext = AddressModes::ZeroY; }
                    'n' => { state.execution.modeNext = AddressModes::IndWord; }
                    _ => { }
                }
                return;
            }
            'd' | 'b' | 'z' => {
                match (*str.offset(1) as u8 | 0x20) as char {
                    'x' => { state.execution.modeNext = AddressModes::ByteAdrX; }
                    'y' => { state.execution.modeNext = AddressModes::ByteAdrY; }
                    'i' => { state.execution.modeNext = AddressModes::BitMod; }
                    'b' => { state.execution.modeNext = AddressModes::BitBraMod; }
                    _ => { state.execution.modeNext = AddressModes::ByteAdr; }
                }
                return;
            }
            'e' | 'w' | 'a' => {
                match (*str.offset(1) as u8 | 0x20) as char {
                    'x' => { state.execution.modeNext = AddressModes::WordAdrX; }
                    'y' => { state.execution.modeNext = AddressModes::WordAdrY; }
                    _ => { state.execution.modeNext = AddressModes::WordAdr; }
                }
                return;
            }
            'l' => { state.execution.modeNext = AddressModes::Long; return; }
            'r' => { state.execution.modeNext = AddressModes::Rel; return; }
            'u' => { state.execution.modeNext = AddressModes::BSS; return; }
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
pub unsafe extern "C" fn rmnode(base: *mut *mut libc::c_void, mut _bytes: i32) {
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
pub unsafe extern "C" fn parse(buf: *mut i8) -> *mut _MNE {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(buf)); } }

    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut labelundefined: i32 = 0;
    /*
        If the first non-space is a ^, skip all further spaces too.
        This means what follows is a label.
        If the first non-space is a #, what follows is a directive/opcode.
    */
    while *buf.offset(i as isize) as i32 == ' ' as i32 {
        i += 1;
    }
    if *buf.offset(i as isize) as i32 == '^' as i32 {
        i += 1;
        while *buf.offset(i as isize) as i32 == ' ' as i32 {
            i += 1;
        }
    } else if *buf.offset(i as isize) as i32 == '#' as i32 {
        *buf.offset(i as isize) = ' ' as i32 as i8
        /* label separator */
    } else {
        i = 0;
    }

    let ref mut fresh6 = *Av.as_mut_ptr().offset(0);
    *fresh6 = Avbuf.as_mut_ptr().offset(j as isize);
    while *buf.offset(i as isize) as i32 != 0 && *buf.offset(i as isize) as i32 != ' ' as i32 && *buf.offset(i as isize) as i32 != '=' as i32 {
        if *buf.offset(i as isize) as i32 == ':' as i32 {
            i += 1;
            break;
        } else if *buf.offset(i as isize) as i32 == ',' as i32 {
            // we have label arguments
            if *buf.offset((i + 1) as isize) as i32 == '\"' as i32 {
                // is it a string constant?
                i = i + 2; // advance to string contents
                while *buf.offset(i as isize) as i32 != 0
                    && *buf.offset(i as isize) as i32 != '\"' as i32
                    && *buf.offset(i as isize) as i32 != ' ' as i32
                    && *buf.offset(i as isize) as i32 != ',' as i32
                    && *buf.offset(i as isize) as i32 != ':' as i32
                {
                    *Avbuf.as_mut_ptr().offset(j as isize) = *buf.offset(i as isize);
                    i += 1;
                    j += 1;
                }
                if *buf.offset(i as isize) as i32 != 0 && *buf.offset(i as isize) as i32 == '\"' as i32 {
                    i += 1
                } else {
                    labelundefined += 1;
                    asmerr(AsmErrorEquates::SyntaxError, false, buf);
                }
            } else {
                // or else it's a symbol to be evaluated, and added to the label
                let mut t: usize = 0;
                let mut temp_buffer = String::new();
                let buffer = transient::str_pointer_to_string(buf);
                let mut symarg: *mut _SYMBOL = 0 as *mut _SYMBOL;
                temp_buffer.push_str(&buffer[i + 1..]);
                temp_buffer.truncate(256);

                while t < temp_buffer.len() {
                    if temp_buffer.as_bytes()[t] == ',' as u8 {
                        temp_buffer.truncate(t);
                    }
                    t += 1;
                }
                symarg = eval(transient::string_to_str_pointer(temp_buffer), 0);
                if !symarg.is_null() {
                    if (*symarg).flags & SymbolTypes::Unknown != 0 {
                        // One of the arguments isn't defined yet
                        // Ensure the label we're creating doesn't get used
                        labelundefined += 1;
                    } else {
                        let temp_value = format!("{}", (*symarg).value);
                        let temp_value_len = temp_value.len();
                        strcpy(Avbuf.as_mut_ptr().offset(j as isize), transient::string_to_str_pointer(temp_value));
                        j += temp_value_len;
                    }
                }
                i += 1;
                while *buf.offset(i as isize) as i32 != 0
                    && *buf.offset(i as isize) as i32 != ' ' as i32
                    && *buf.offset(i as isize) as i32 != '=' as i32
                    && *buf.offset(i as isize) as i32 != ',' as i32
                    && *buf.offset(i as isize) as i32 != ':' as i32
                {
                    i += 1;
                }
            }
        } else {
            if *buf.offset(i as isize) as u8 as i32 == 0x80 as i32 {
                *buf.offset(i as isize) = ' ' as i32 as i8;
            }
            *Avbuf.as_mut_ptr().offset(j as isize) = *buf.offset(i as isize);
            i += 1;
            j += 1;
        }
    }
    *Avbuf.as_mut_ptr().offset(j as isize) = 0;
    j += 1;
    // if the label has arguments that aren't defined, we need to scuttle it
    // to avoid a partial label being used.
    if labelundefined != 0 {
        j = 1;
        *Avbuf.as_mut_ptr().offset(j as isize) = 0;
        j += 1;
    }
    /* Parse the second word of the line */
    while *buf.offset(i as isize) as i32 == ' ' as i32 { i += 1 }
    let ref mut fresh13 = *Av.as_mut_ptr().offset(1);
    *fresh13 = Avbuf.as_mut_ptr().offset(j as isize);
    if *buf.offset(i as isize) as i32 == '=' as i32 {
        /* '=' directly seperates Av[0] and Av[2] */
        let fresh14 = i;
        i = i + 1;
        let fresh15 = j;
        j = j + 1;
        *Avbuf.as_mut_ptr().offset(fresh15 as isize) = *buf.offset(fresh14 as isize)
    } else {
        while *buf.offset(i as isize) as i32 != 0 && *buf.offset(i as isize) as i32 != ' ' as i32 {
            if *buf.offset(i as isize) as u8 as i32 == 0x80 as i32 {
                *buf.offset(i as isize) = ' ' as i32 as i8
            }
            *Avbuf.as_mut_ptr().offset(j as isize) = *buf.offset(i as isize);
            i += 1;
            j += 1;
        }
    }
    let fresh18 = j;
    j = j + 1;
    *Avbuf.as_mut_ptr().offset(fresh18 as isize) = 0;
    /* and analyse it as an opcode */
    findext(*Av.as_mut_ptr().offset(1));
    mne = findmne(*Av.as_ptr().offset(1));
    /* Parse the rest of the line */
    while *buf.offset(i as isize) as i32 == ' ' as i32 { i += 1 }
    let ref mut fresh19 = *Av.as_mut_ptr().offset(2);
    *fresh19 = Avbuf.as_mut_ptr().offset(j as isize);
    while *buf.offset(i as isize) != 0 {
        if *buf.offset(i as isize) as i32 == ' ' as i32 {
            while *buf.offset((i + 1) as isize) as i32 == ' ' as i32 {
                i += 1;
            }
        }
        if *buf.offset(i as isize) as u8 as i32 == 0x80 as i32 {
            *buf.offset(i as isize) = ' ' as i32 as i8;
        }
        *Avbuf.as_mut_ptr().offset(j as isize) = *buf.offset(i as isize);
        i += 1;
        j += 1;
    }
    *Avbuf.as_mut_ptr().offset(j as isize) = 0;
    return mne;
}
pub unsafe fn findmne(mut str: *const i8) -> *mut _MNE {
    let mut i: i32 = 0;
    let mut c: i8 = 0;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut buf: [i8; 64] = [0; 64];
    if *str.offset(0) as i32 == '.' as i32 {
        /* Allow .OP for OP */
        str = str.offset(1)
    }
    i = 0;
    loop  {
        c = *str.offset(i as isize);
        if !(c != 0) { break ; }
        if c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32 {
            c = (c as i32 + ('a' as i32 - 'A' as i32)) as i8
        }
        buf[i as usize] = c;
        i += 1
    }
    buf[i as usize] = 0;
    mne = *MHash.as_mut_ptr().offset(hash_string(transient::str_pointer_to_string(buf.as_mut_ptr())) as isize);
    while !mne.is_null() {
        if strcmp(buf.as_mut_ptr(), (*mne).name) == 0 {
            break;
        }
        mne = (*mne).next
    }
    return mne;
}
/* symbols.c */
/* ops.c */
#[no_mangle]
pub unsafe extern "C" fn v_macro(str: *mut i8, _dummy: *mut _MNE) {
    #[cfg(debug_assertions)]
    { if state.parameters.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut base: *mut _STRLIST = 0 as *mut _STRLIST; /* slp, mac: might be used uninitialised */
    let mut defined: bool = false; // Conversion note: "not really needed" according to the original code
    let mut slp: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut sl: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut mac: *mut _MACRO = 0 as *mut _MACRO;
    let mut mne: *mut _MNE = 0 as *mut _MNE;
    let mut i: u16 = 0;
    let mut buf: [i8; MAX_LINES] = [0; MAX_LINES];

    let current_if = &state.execution.ifs.last().unwrap();
    let skipit = !current_if.result || !current_if.result_acc;

    // Updates the *str in memory.
    // This could have been just...
    //   str = transient::string_to_str_pointer(transient::str_pointer_to_string(str).to_ascii_lowercase());
    // ...but, other parts of the code reuse the same string, so we need to update
    // the original location rather than just a copy of the string.
    // FIXME: drop all of this in favor of properly renamed macros
    let newStr = transient::str_pointer_to_string(str).to_ascii_lowercase();
    transient::update_str_pointer_in_place(str, newStr.as_str());

    mne = findmne(str);
    if skipit {
        defined = true
    } else {
        defined = mne != 0 as *mut libc::c_void as *mut _MNE;
        if !state.parameters.listFile.is_empty() && state.execution.listMode != ListMode::None {
            outlistfile(b"\x00" as *const u8 as *const i8);
        }
    }
    if !defined {
        base = 0 as *mut _STRLIST;
        slp = &mut base;
        mac = permalloc(::std::mem::size_of::<_MACRO>() as u64 as i32) as *mut _MACRO;
        i = hash_string(transient::str_pointer_to_string(str));
        (*mac).next = *MHash.as_mut_ptr().offset(i as isize) as *mut _MACRO;
        (*mac).vect = Some(v_execmac as unsafe extern "C" fn(_: *mut i8, _: *mut _MACRO) -> ());
        (*mac).name = strcpy(permalloc(strlen(str).wrapping_add(1) as i32), str);
        (*mac).flags = 0x8;
        (*mac).defpass = state.execution.pass as i32;
        let ref mut fresh22 = *MHash.as_mut_ptr().offset(i as isize);
        *fresh22 = mac as *mut _MNE
    } else {
        mac = mne as *mut _MACRO;
        if state.parameters.strictMode && !mac.is_null() && (*mac).defpass == state.execution.pass as i32 {
            asmerr(AsmErrorEquates::MacroRepeated, true, str);
        }
    }
    while !fgets(buf.as_mut_ptr(), MAX_LINES as i32, (*pIncfile).fi).is_null() {
        let mut comment: *const i8 = 0 as *const i8;
        if state.parameters.debug {
            println!("{:08x} {}", pIncfile as u64, transient::str_pointer_to_string(buf.as_mut_ptr()));
        }
        (*pIncfile).lineno = (*pIncfile).lineno.wrapping_add(1);
        comment = cleanup(buf.as_mut_ptr(), true);
        mne = parse(buf.as_mut_ptr());
        if *(*Av.as_ptr().offset(1)).offset(0) != 0 {
            if !mne.is_null() && (*mne).flags as i32 & 0x80 as i32 != 0 {
                if !defined { (*mac).strlist = base }
                return;
            }
        }
        if !skipit && !state.parameters.listFile.is_empty() && state.execution.listMode != ListMode::None {
            outlistfile(comment);
        }
        if !defined {
            sl = permalloc((::std::mem::size_of::<*mut _STRLIST>() as u64).wrapping_add(1).wrapping_add(strlen(buf.as_mut_ptr())) as i32) as *mut _STRLIST;
            strcpy((*sl).buf.as_mut_ptr(), buf.as_mut_ptr());
            *slp = sl;
            slp = &mut (*sl).next
        }
    }
    asmerr(AsmErrorEquates::PrematureEOF, true, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn addhashtable(mut mne: *mut _MNE) {
    let mut i: usize;
    let mut j: usize;
    let mut opcode: [u32; 21] = [0; 21];
    while (*mne).vect.is_some() {
        memcpy(opcode.as_mut_ptr() as *mut libc::c_void,
               (*mne).opcode.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[u32; 21]>() as u64);
        j = 0;
        i = j;
        while i < AddressModes::length() as usize {
            (*mne).opcode[i as usize] = 0;
            if (*mne).okmask & ((1) << i) as u64 != 0 {
                (*mne).opcode[i as usize] = opcode[j];
                j += 1;
            }
            i += 1
        }
        i = hash_string(transient::str_pointer_to_string((*mne).name)) as usize;
        (*mne).next = *MHash.as_mut_ptr().offset(i as isize);
        let ref mut fresh24 = *MHash.as_mut_ptr().offset(i as isize);
        *fresh24 = mne;
        mne = mne.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn pushinclude(str: *const i8) {
    let mut inf: *mut _INCFILE = 0 as *mut _INCFILE;
    let mut fi: *mut FILE = 0 as *mut FILE;
    fi = pfopen(str, b"rb\x00" as *const u8 as *const i8);
    if !fi.is_null() {
        if state.parameters.verbosity as u8 > Verbosity::Two as u8 {
            // Originally this had a strange formatting using
            // "state.other.incLevel * 4" as padding ("%.*s"),
            // but with no discernible effect since an empty
            // string was passed anyway. This drops all flexible
            // padding in favor of a single space.
            println!(
                " Including file \"{}\"",
                transient::str_pointer_to_string(str).as_str()
            );
        }
        state.other.incLevel += 1;
        filesystem::writeln_to_file_maybe(
            &mut state.output.listFile,
            format!(
                "------- FILE {} LEVEL {} PASS {}",
                transient::str_pointer_to_string(str),
                state.other.incLevel,
                state.execution.pass,
            ).as_str(),
        );
        inf = zmalloc(::std::mem::size_of::<_INCFILE>() as u64 as i32) as *mut _INCFILE;
        (*inf).next = pIncfile;
        (*inf).name = strcpy(ckmalloc(strlen(str).wrapping_add(1) as i32), str);
        (*inf).fi = fi;
        (*inf).lineno = 0;
        pIncfile = inf;
        return;
    }
    println!("Warning: Unable to open '{}'", transient::str_pointer_to_string(str));
}
#[no_mangle]
pub unsafe extern "C" fn asmerr(err: AsmErrorEquates, abort: bool, sText: *const i8) -> AsmErrorEquates {
    let mut errorOutput: String = String::new();
    let mut pincfile: *mut _INCFILE = 0 as *mut _INCFILE;
    /* file pointer we print error messages to */
    let errorToFile = !state.parameters.listFile.is_empty();
    let errorFile = &mut state.output.listFile;
    if find_error_definition(err).fatal {
        state.other.stopAtEnd = true
    }
    pincfile = pIncfile;
    while (*pincfile).flags as i32 & 0x1 as i32 != 0 {
        pincfile = (*pincfile).next
    }
    let mut errorDescription = find_error_definition(err).description.clone().to_owned();
    errorDescription.push_str("\n");
    /*
        New error format selection for 2.20.11 since some
        people *don't* use MS products. For historical
        reasons we currently send errors to stdout when
        they should really go to stderr, but we'll switch
        eventually I hope... [phf]
    */
    /* print first part of message, different formats offered */
    match state.parameters.errorFormat {
        ErrorFormat::Woe => {
            /*
                Error format for MS VisualStudio and relatives:
                "file (line): error: string"
            */
            let errorMessage = format!(
                "{} ({}): error: ",
                transient::str_pointer_to_string((*pincfile).name),
                (*pincfile).lineno
            );
            if errorToFile {
                filesystem::write_to_file_maybe(errorFile, errorMessage.as_str());
            }
            errorOutput.push_str(errorMessage.as_str());
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
            let errorMessage = format!(
                "line {:>7} {:10} ",
                (*pincfile).lineno,
                transient::str_pointer_to_string((*pincfile).name),
            );
            if errorToFile {
                filesystem::write_to_file_maybe(errorFile, errorMessage.as_str());
            }
            errorOutput.push_str(errorMessage.as_str());
        }
        ErrorFormat::GNU => {
            /*
                GNU format error messages, from their coding
                standards.
            */
            let errorMessage = format!(
                "{}:{}: error: ",
                transient::str_pointer_to_string((*pincfile).name),
                (*pincfile).lineno,
            );
            if errorToFile {
                filesystem::write_to_file_maybe(errorFile, errorMessage.as_str());
            }
            errorOutput.push_str(errorMessage.as_str());
        }
    }
    // This is a bit of a hack: since we can't use variables as the template in format!(),
    // we simply replace "{}" in the template with the expected string. This works well,
    // but it means the template only supports a single {}, and no other formatting directive.
    if !sText.is_null() {
        errorDescription = errorDescription.replace("{}", transient::str_pointer_to_string(sText).as_str());
    }
    if errorToFile {
        /* print second part of message, always the same for now */
        filesystem::write_to_file_maybe(errorFile, errorDescription.as_str());
    }
    errorOutput.push_str(errorDescription.as_str());
    operations::update_passbuffer(&mut state.output.passBufferErrors, errorOutput.as_str());
    if abort {
        operations::output_passbuffer(&mut state.output.passBufferMessages);
        if errorToFile {
            filesystem::writeln_to_file_maybe(
                errorFile,
                "Aborting assembly"
            );
            filesystem::close_file_maybe(errorFile);
        } else {
            println!("Aborting assembly");
        }
        operations::output_passbuffer(&mut state.output.passBufferErrors);
        std::process::exit(1);
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc(bytes: i32) -> *mut i8 {
    let ptr: *mut i8 = ckmalloc(bytes);
    if !ptr.is_null() {
        memset(ptr as *mut libc::c_void, 0, bytes as u64);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ckmalloc(bytes: i32) -> *mut i8 {
    let ptr: *mut i8 = malloc(bytes as u64) as *mut i8;
    if !ptr.is_null() { return ptr }
    panic("unable to malloc");
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn permalloc(mut bytes: i32) -> *mut i8 {
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut left: i32 = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    /* Assume sizeof(union align) is a power of 2 */
    bytes = ((bytes as u64).wrapping_add(::std::mem::size_of::<align>() as u64).wrapping_sub(1)
             & !(::std::mem::size_of::<align>() as u64).wrapping_sub(1)) as i32;
    if bytes > left {
        buf = malloc(ALLOC_SIZE as i32 as u64) as *mut i8;
        if buf.is_null() {
            panic("unable to malloc");
        }
        memset(buf as *mut libc::c_void, 0,
            ALLOC_SIZE as i32 as u64);
        left = ALLOC_SIZE as i32;
        if bytes > left {
            panic("software error");
        }
    }
    ptr = buf;
    buf = buf.offset(bytes as isize);
    left -= bytes;
    return ptr;
}
unsafe fn main_0(ac: i32, av: *mut *mut i8) -> u8 {
    let mut bTableSort: bool = false;
    let nError: AsmErrorEquates = MainShadow(ac, av, &mut bTableSort);
    if nError != AsmErrorEquates::None && nError != AsmErrorEquates::NonAbort {
        // Dump messages when aborting due to errors
        operations::output_passbuffer(&mut state.output.passBufferMessages);
        // Only print errors if assembly is unsuccessful
        operations::output_passbuffer(&mut state.output.passBufferErrors);
        println!("Fatal assembly error: {}", find_error_definition(nError).description);
    }
    dump_symbol_table(bTableSort);
    operations::clear_passbuffer(&mut state.output.passBufferErrors);
    operations::clear_passbuffer(&mut state.output.passBufferMessages);
    return nError.into();
}
#[main]
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in std::env::args() {
        args.push(transient::string_to_str_pointer(arg));
    }
    args.push(std::ptr::null_mut());
    unsafe {
        std::process::exit(
            main_0(
                (args.len() - 1) as i32,
                args.as_mut_ptr() as *mut *mut i8
            ) as i32
        );
    }
}
