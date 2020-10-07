#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![feature(seek_convenience)]
#![register_tool(c2rust)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate smart_default;

use libc;
use std::env;
use std::process;
use std::convert::TryFrom;
use std::fs::File;

pub mod constants;
pub mod expressions;
pub mod globals;
pub mod macros;
pub mod mnemonics;
pub mod operations;
pub mod segments;
pub mod symbols;
pub mod types;
pub mod utils;

use constants::{
    DASM_ID,
    MAX_LINES,
    S_HASH_SIZE,
};
use globals::state;
use macros::{
    find_macro,
};
use mnemonics::{
    find_mnemonic,
    parse_mnemonic_name,
};
use segments::{
    clear_segments,
};
use types::flags:: {
    FileFlags,
    IfFlags,
    MnemonicsFlags,
    ReasonCodes,
    SegmentTypes,
    SymbolTypes,
};
use types::enums::{
    AddressModes,
    AsmErrorEquates,
    ErrorFormat,
    ExitCode,
    Format,
    ListMode,
    SortMode,
    Verbosity,
};
use types::legacy::{
    __compar_fn_t,
    _INCFILE,
    _MNE,
    _STRLIST,
    _SYMBOL,
    FILE,
    MacroOrMnemonicPointer,
};
use types::structs::{
    CommandLineOptions,
    Segment,
    StackIf,
};
use utils::{
    filesystem,
    find_error_definition,
    formatting,
    panic,
    transient,
};
use utils::extensions::{
    StringExtensions,
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
    #[no_mangle]
    static mut SHash: [*mut _SYMBOL; 0];
    /* exp.c */
    #[no_mangle]
    fn eval(str: *const i8, wantmode: i32) -> *mut _SYMBOL;
    #[no_mangle]
    fn pfopen(_: *const i8, _: *const i8) -> *mut FILE;
    #[no_mangle]
    fn programlabel();
    #[no_mangle]
    fn closegenerate();
}

lazy_static! {
    // Gather arguments so we can treat them C-like.
    // Normally, a Rust application would use StructOpt for this,
    // but we keep the custom parser for consistency.
    pub static ref OPTIONS: CommandLineOptions = parse_command_line(env::args().skip(1).collect());
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
unsafe fn CountUnresolvedSymbols() -> i32 {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut nUnresolved: i32 = 0;
    let mut i: i32 = 0;
    /* Pre-count unresolved symbols */
    i = 0;
    while i < S_HASH_SIZE as i32 {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() {
            if (*sym).flags & SymbolTypes::Unknown != 0 {
                nUnresolved += 1;
            }
            sym = (*sym).next;
        }
        i += 1;
    }
    return nUnresolved;
}
unsafe fn ShowUnresolvedSymbols() -> i32 {
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
                if (*sym).flags & SymbolTypes::Unknown != 0 {
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
    symArray = transient::ckmalloc((::std::mem::size_of::<*mut _SYMBOL>() as u64).wrapping_mul(nSymbols as u64) as i32) as *mut *mut _SYMBOL;
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
            if (**symArray.offset(i as isize)).flags & SymbolTypes::StringResult != 0 {
                result.push_str(format!(
                    " \"{}\"",
                    transient::str_pointer_to_string((**symArray.offset(i as isize)).string),
                ).as_str());
            }
            result.push_str("\n");
            i += 1;
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
    if !OPTIONS.symbols_file.is_empty() {
        let mut file = filesystem::create_new_file(OPTIONS.symbols_file.as_str()).expect(
            format!("Warning: Unable to open Symbol Dump file '{}'", OPTIONS.symbols_file).as_str()
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

unsafe fn ShowSegments() {
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

/**
 * Parses arguments from the command line only, creating a struct with options.
 * This only has two side effects:
 *   - println!() when a debug param is set
 *   - panicking for invalid parameters
 */
fn parse_command_line(args: Vec<String>) -> CommandLineOptions {
    let mut options = CommandLineOptions::new();

    // Other default
    options.out_file = String::from("a.out");

    if args.len() > 0 {
        options.input = args[0].clone();
        let mut i: usize = 1;
        while i < args.len() && !options.parsing_failed {
            let arg = args[i].clone();
            if arg.at(0) == "-" || arg.at(0) == "/" {
                let mut arg_value = String::from(arg.from(2));
                match arg.at(1) {
                    "E" => {
                        if let Some(result) = arg_value.parse::<u8>().ok().and_then(|digit| ErrorFormat::try_from(digit).ok()) {
                            options.error_format = result;
                        } else {
                            panic("Invalid error format for -E, must be 0, 1, 2");
                        }
                    }
                    "T" => {
                        if let Some(result) = arg_value.parse::<u8>().ok().and_then(|digit| SortMode::try_from(digit).ok()) {
                            options.sort_mode = result;
                        } else {
                            panic("Invalid sorting mode for -T option, must be 0 or 1");
                        }
                    }
                    "d" => {
                        let digit = arg_value.parse::<u8>().unwrap_or(0);
                        let d_equals_z = arg_value.to_ascii_lowercase() == "z";
                        options.debug = digit != 0 || d_equals_z;
                        options.debug_extended = d_equals_z;
                        println!("Debug trace {}", if options.debug { "ON" } else { "OFF" });
                    }
                    "M" | "D" => {
                        let arg_values: Vec<&str> = arg_value.split('=').collect();
                        let arg_expression = if arg_values.len() > 1 { String::from(arg_values[1]) } else { String::from("0") };

                        if arg.at(1) == "M" {
                            options.symbols_eqm.push((String::from(arg_values[0]), arg_expression));
                        } else {
                            options.symbols_set.push((String::from(arg_values[0]), arg_expression));
                        }
                    }
                    "f" => {
                        if let Some(result) = arg_value.parse::<u8>().ok().and_then(|digit| Format::try_from(digit).ok()) {
                            options.format = result;
                        } else {
                            panic("Illegal format specification");
                        }
                    }
                    "o" => {
                        options.out_file = arg_value.clone();
                        if arg_value.len() == 0 {
                            panic("-o Switch requires file name.");
                        }
                    }
                    "L" | "l" => {
                        options.list_file = arg_value.clone();
                        if arg.at(1) == "L" {
                            options.list_all_passes = true;
                        }
                        if arg_value.len() == 0 {
                            // Conversion note: this error outputs "-o" incorrectly in the original C code
                            panic("-l Switch requires file name.");
                        }
                    }
                    "P" | "p" => {
                        options.max_passes = arg_value.parse::<u8>().unwrap_or(0);
                        if arg.at(1) == "P" {
                            options.do_all_passes = true;
                        }
                    }
                    "s" => {
                        options.symbols_file = arg_value.clone();
                        if arg_value.len() == 0 {
                            // Conversion note: this error outputs "-o" incorrectly in the original C code
                            panic("-s Switch requires file name.");
                        }
                    }
                    "v" => {
                        if arg_value.len() == 0 {
                            options.verbosity = Verbosity::None;
                        } else if let Some(result) = arg_value.parse::<u8>().ok().and_then(|digit| Verbosity::try_from(digit).ok()) {
                            options.verbosity = result;
                        } else {
                            // Conversion note: verbosity errors are ignored in the original C code, but we check them here
                            panic("Illegal verbosity specification");
                        }
                    }
                    "I" => {
                        options.include_dirs.push(arg_value);
                    }
                    "S" => {
                        options.strict_mode = true;
                    }
                    _ => {
                        options.parsing_failed = true;
                    }
                }
            }

            i += 1;
        }
    } else {
        options.parsing_failed = true;
    }

    options
}

/**
 * Prints generic command line help, usually if no operation has taken place or in the case
 * of error.
 */
fn print_command_line_help() {
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
}

unsafe fn assemble() -> AsmErrorEquates {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function!(); } }

    let mut nError: AsmErrorEquates = AsmErrorEquates::None;
    let mut buf: [i8; MAX_LINES] = [0; MAX_LINES];
    let mut oldRedoIndex: i32 = -1;
    let mut oldRedoWhy: u64 = 0;
    let mut oldRedoEval: i32 = 0;
    addhashtable(globals::legacy::mnemonics_operations.as_mut_ptr(), globals::legacy::mnemonics_operations.len());
    state.execution.pass = 1;

    // Perform side effects based on command line options
    for (name, expression) in OPTIONS.symbols_eqm.clone() {
        set_argument(0, name);
        mnemonics::operations::v_eqm(transient::string_to_str_pointer(expression), 0 as *mut _MNE);
    }

    for (name, expression) in OPTIONS.symbols_set.clone() {
        set_argument(0, name);
        mnemonics::operations::v_set(transient::string_to_str_pointer(expression), 0 as *mut _MNE);
    }

    for dir in OPTIONS.include_dirs.clone() {
        mnemonics::operations::v_incdir(transient::string_to_str_pointer(dir), 0 as *mut _MNE);
    }

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
    loop {
        if OPTIONS.verbosity != Verbosity::None {
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
        let fileOutOption = filesystem::create_new_file(OPTIONS.out_file.as_str());
        match fileOutOption {
            Ok(file) => {
                state.output.outFile = Some(file);
            },
            _ => {
                println!("Warning: Unable to [re]open '{}'", OPTIONS.out_file);
                return AsmErrorEquates::FileError;
            },
        }

        // Create list output file
        if !OPTIONS.list_file.is_empty() {
            let fileOption = filesystem::create_new_file(OPTIONS.list_file.as_str());
            match fileOption {
                Ok(file) => {
                    state.output.listFile = Some(file);
                },
                _ => {
                    println!("Warning: Unable to [re]open '{}'", OPTIONS.list_file);
                    return AsmErrorEquates::FileError;
                },
            }
        }

        pushinclude(transient::string_to_str_pointer(OPTIONS.input.clone()));
        while state.execution.includeFiles.len() > 0 {
            loop {
                let include_file = *state.execution.includeFiles.last().unwrap();

                #[cfg(debug_assertions)]
                { if OPTIONS.debug_extended { log_function_with!("loop start for incfile '{}' @ {} with flags {}", transient::str_pointer_to_string((*include_file).name), (*include_file).lineno, (*include_file).flags); } }

                let mut comment: *const i8 = 0 as *const i8;
                if (*include_file).flags & FileFlags::Macro != 0 {
                    if (*include_file).strlist.is_null() {
                        set_argument(0, String::new());
                        mnemonics::operations::v_mexit(0 as *mut i8, 0 as *mut _MNE);
                        continue;
                    } else {
                        strcpy(buf.as_mut_ptr(), (*(*include_file).strlist).buf.as_mut_ptr());
                        (*include_file).strlist = (*(*include_file).strlist).next;
                    }
                } else if fgets(buf.as_mut_ptr(), MAX_LINES as i32, (*include_file).fi).is_null() {
                    break;
                }
                if OPTIONS.debug {
                    println!("{:08x} {}", include_file as u64, transient::str_pointer_to_string(buf.as_mut_ptr()));
                }
                comment = cleanup(buf.as_mut_ptr(), false);
                (*include_file).lineno = (*include_file).lineno.wrapping_add(1);
                let mne_or_macro = parse(buf.as_ptr());
                let current_if = &state.execution.ifs.last().unwrap();

                #[cfg(debug_assertions)]
                { if OPTIONS.debug_extended { log_function_with!("current_if = {} {}", current_if.result, current_if.result_acc); } }

                if !get_argument(1).is_empty() {
                    match mne_or_macro {
                        MacroOrMnemonicPointer::Macro(mac) => {
                            if (*mac).flags & MnemonicsFlags::If != 0 || current_if.result && current_if.result_acc {
                                #[cfg(debug_assertions)]
                                {
                                    if OPTIONS.debug_extended {
                                        log_function_with!("calling vect on [[{}]] [[{}]]", transient::str_pointer_to_string((*mac).name), get_argument(2));
                                    }
                                }
                                ((*mac).vect)(transient::string_to_str_pointer(get_argument(2)), mac);
                            }
                        }
                        MacroOrMnemonicPointer::Mnemonic(mne) => {
                            if (*mne).flags & MnemonicsFlags::If != 0 || current_if.result && current_if.result_acc {
                                #[cfg(debug_assertions)]
                                {
                                    if OPTIONS.debug_extended {
                                        log_function_with!("calling vect on [[{}]] [[{}]]", transient::str_pointer_to_string((*mne).name), get_argument(2));
                                    }
                                }
                                ((*mne).vect)(transient::string_to_str_pointer(get_argument(2)), mne);
                            }
                        }
                        MacroOrMnemonicPointer::None => {
                            if current_if.result && current_if.result_acc {
                                asmerr(AsmErrorEquates::UnknownMnemonic, false, transient::string_to_str_pointer(get_argument(1)));
                            }
                        }
                    }
                } else if current_if.result && current_if.result_acc {
                    programlabel();
                }
                if !OPTIONS.list_file.is_empty() && state.execution.listMode != ListMode::None {
                    outlistfile(comment);
                }
            }

            let include_file = *state.execution.includeFiles.last().unwrap();

            #[cfg(debug_assertions)]
            { if OPTIONS.debug_extended { log_function_with!("loop continue :: incfile '{}' @ {} with flags {}", transient::str_pointer_to_string((*include_file).name), (*include_file).lineno, (*include_file).flags); } }

            while state.execution.repeats.len() > 0 && state.execution.repeats.last().unwrap().file == include_file {
                state.execution.repeats.pop();
            }
            while state.execution.ifs.len() > 0 && state.execution.ifs.last().unwrap().file == include_file {
                state.execution.ifs.pop();
            }
            fclose((*include_file).fi);
            free((*include_file).name as *mut libc::c_void);
            state.other.incLevel -= 1;
            state.execution.includeFiles.pop();
            if state.execution.includeFiles.len() > 0 {
                /*
                if (state.options.verbosity as u8 > 1)
                println!("back to: {}", Incfile->name);
                */
                filesystem::writeln_to_file_maybe(
                    &mut state.output.listFile,
                    format!(
                        "------- FILE {}",
                        transient::str_pointer_to_string((*(*state.execution.includeFiles.last().unwrap())).name),
                    ).as_str(),
                );
            }
        }

        if OPTIONS.verbosity as u8 >= Verbosity::One as u8 {
            ShowSegments();
        }
        if OPTIONS.verbosity as u8  >= Verbosity::Three as u8  {
            if state.execution.redoIndex == 0 || OPTIONS.verbosity as u8  >= Verbosity::Four as u8 {
                write_symbols_to_stdout(OPTIONS.sort_mode == SortMode::Address);
            }
            ShowUnresolvedSymbols();
        }
        closegenerate();
        filesystem::close_file_maybe(&mut state.output.outFile);
        filesystem::close_file_maybe(&mut state.output.listFile);
        if state.execution.redoIndex != 0 {
            if !OPTIONS.do_all_passes {
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
            if state.execution.pass > OPTIONS.max_passes {
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

unsafe fn main_shadow() -> AsmErrorEquates {
    // FIXME: change this to options.* once we don't have options in the state anymore
    if !OPTIONS.parsing_failed {
        return assemble();
    }

    // If nothing done, just show help
    print_command_line_help();

    // Finally, bail with generic error
    AsmErrorEquates::CommandLine
}

pub unsafe fn outlistfile(comment: *const i8) {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(comment)); } }

    let current_if = &state.execution.ifs.last().unwrap();
    let mut xtrue: char = 0 as char;
    let mut c: char = 0 as char;
    let mut buffer: String = String::new();
    let mut i: usize = 0;
    let include_file = *state.execution.includeFiles.last().unwrap();
    if (*include_file).flags & FileFlags::NoList != 0 {
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
            (*include_file).lineno,
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
        i += 1;
    }
    buffer.pop();
    buffer.push_str(
        // FIXME: this is a good example of how messed up the "argument"
        // strings are, probably because things have been added over time.
        // It would be better to get rid of them, or move them into a
        // unique struct.
        format!(
            "{}{:10} {}{}{}\t{}\n",
            xtrue,
            get_argument(0),                                    // Label
            parse_mnemonic_name(get_argument(1).as_str()).0,    // Mnemonic name (simple, sans extension)
            dot,                                                // ".", if extension present
            state.execution.extraString.clone(),                // Mnemonic extension, if any
            get_argument(2),                                    // Parameters
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

unsafe fn clearrefs() {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut i: i16 = 0;
    i = 0;
    while (i as i32) < S_HASH_SIZE as i32 {
        sym = *SHash.as_mut_ptr().offset(i as isize);
        while !sym.is_null() {
            (*sym).flags &= !SymbolTypes::Referenced;
            sym = (*sym).next;
        }
        i += 1;
    };
}
/*
   replace old atoi() calls; I wanted to protect this using
   #ifdef strtol but the C preprocessor doesn't recognize
   function names, at least not GCC's; we should be safe
   since MS compilers document strtol as well... [phf]
*/
pub unsafe fn cleanup(buf: *mut i8, bDisable: bool) -> *const i8 {
    let mut str: *mut i8 = 0 as *mut i8;
    let mut strlist: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut arg: i32 = 0;
    let mut add: i32 = 0;
    let mut comment: *const i8 = b"\x00" as *const u8 as *const i8;
    let include_file = *state.execution.includeFiles.last().unwrap();
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
                    if OPTIONS.debug {
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
                        if OPTIONS.debug {
                            println!("add/str: {} '{}'", add, transient::str_pointer_to_string(str));
                        }
                        strlist = (*include_file).args;
                        while arg != 0 && !strlist.is_null() {
                            arg -= 1;
                            strlist = (*strlist).next
                        }
                        if !strlist.is_null() {
                            add = (add as u64).wrapping_add(strlen((*strlist).buf.as_mut_ptr())) as i32;
                            if OPTIONS.debug {
                                println!(
                                    "strlist: '{}' {}",
                                    transient::str_pointer_to_string((*strlist).buf.as_mut_ptr()),
                                    strlen((*strlist).buf.as_mut_ptr())
                                );
                            }
                            if str.offset(add as isize).offset(strlen(str) as isize).offset(1) > buf.offset(MAX_LINES as isize) {
                                if OPTIONS.debug {
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
* Parse into three arguments: Av[0], Av[1], Av[2]
*/
pub unsafe fn parse(buf: *const i8) -> MacroOrMnemonicPointer {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(buf)); } }

    let mut labelundefined: i32 = 0;
    let mut buffer = transient::str_pointer_to_string(buf);
    let mut buffer_pos: usize = 0;
    let mut av_0 = String::new();
    let mut av_1 = String::new();
    let mut av_2 = String::new();

    // Skip all spaces
    while buffer.at(buffer_pos) == " " {
        buffer_pos += 1;
    }

    if buffer.at(buffer_pos) == "^" {
        // If the first non-space is a ^, skip all further spaces too.
        // This means what follows is a label.
        buffer_pos += 1;
        while buffer.at(buffer_pos) == " " {
            buffer_pos += 1;
        }
    } else if buffer.at(buffer_pos) == "#" {
        // If the first non-space is a #, what follows is a directive/opcode.
        buffer.replace_range(buffer_pos..buffer_pos + 1, " "); // Label separator
    } else {
        // Otherwise, don't skip anything
        buffer_pos = 0;
    }

    while buffer_pos < buffer.len() && buffer.at(buffer_pos) != " " && buffer.at(buffer_pos) != "=" {
        if buffer.at(buffer_pos) == ":" {
            buffer_pos += 1;
            break;
        } else if buffer.at(buffer_pos) == "," {
            // We have label arguments

            // Is it a string constant?
            if buffer.at(buffer_pos + 1) == "\"" {
                // Advance to string contents
                buffer_pos += 2;

                let mut bc = buffer.at(buffer_pos);
                while buffer_pos < buffer.len() && bc != "\"" && bc != " " && bc != "," && bc != ":" {
                    av_0.push_str(bc);
                    buffer_pos += 1;
                    bc = buffer.at(buffer_pos);
                }

                if buffer_pos < buffer.len() && bc == "\"" {
                    buffer_pos += 1;
                } else {
                    labelundefined += 1;
                    asmerr(AsmErrorEquates::SyntaxError, false, transient::string_to_str_pointer(buffer.clone()));
                }
            } else {
                // Or else it's a symbol to be evaluated, and added to the label
                let mut t: usize = 0;
                let mut temp_buffer = String::new();
                let mut symarg: *mut _SYMBOL = 0 as *mut _SYMBOL;
                temp_buffer.push_str(&buffer[buffer_pos + 1..]);
                temp_buffer.truncate(256);

                while t < temp_buffer.len() {
                    if temp_buffer.at(t) == "," {
                        temp_buffer.truncate(t);
                    }
                    t += 1;
                }
                symarg = eval(transient::string_to_str_pointer(temp_buffer), 0);
                if !symarg.is_null() {
                    if (*symarg).flags & SymbolTypes::Unknown != 0 {
                        // One of the arguments isn't defined yet
                        // Ensure the label we're creating doesn't get used
                        labelundefined += 1
                    } else {
                        let mut temp_value = format!("{}", (*symarg).value);
                        av_0.push_str(temp_value.as_str());
                    }
                }

                buffer_pos += 1;
                let mut bc = buffer.at(buffer_pos);
                while buffer_pos < buffer.len() && bc != " " && bc != "=" && bc != "," && bc != ":" {
                    buffer_pos += 1;
                    bc = buffer.at(buffer_pos);
                }
            }
        } else {
            if buffer_pos < buffer.len() && buffer.at(buffer_pos).as_bytes()[0] as u8 == 0x80 {
                buffer.replace_range(buffer_pos..buffer_pos + 1, " ");
            }
            av_0.push_str(buffer.at(buffer_pos));
            buffer_pos += 1;
        }
    }

    // If the label has arguments that aren't defined, we need to scuttle it
    // to avoid a partial label being used.
    if labelundefined != 0 {
        av_0.clear();
    }

    // Set label
    set_argument(0, av_0);

    /* Parse the second word of the line */
    while buffer_pos < buffer.len() && buffer.at(buffer_pos) == " " {
        buffer_pos += 1;
    }

    if buffer_pos < buffer.len() && buffer.at(buffer_pos) == "=" {
        /* '=' directly seperates Av[0] and Av[2] */
        av_1.push_str(buffer.at(buffer_pos));
        buffer_pos += 1;
    } else {
        while buffer_pos < buffer.len() && buffer.at(buffer_pos) != " " {
            if buffer.at(buffer_pos).as_bytes()[0] as u8 == 0x80 {
                buffer.replace_range(buffer_pos..buffer_pos + 1, " ");
            }
            av_1.push_str(buffer.at(buffer_pos));
            buffer_pos += 1;
        }
    }

    // Set full mnemonic name
    set_argument(1, av_1);

    // Analyse it as an opcode
    let full_mnemonic_or_macro_name = get_argument(1);
    let (mnemonic_name, mnemonic_extension) = parse_mnemonic_name(full_mnemonic_or_macro_name.as_str());

    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("find_mnemonic_extension_address_mode :: [[{}]]", mnemonic_extension); } }

    let new_address_mode = mnemonics::find_mnemonic_extension_address_mode(mnemonic_extension);
    state.execution.modeNext = new_address_mode;
    state.execution.extraString = String::from(mnemonic_extension);
    let mnemonic_maybe = find_mnemonic(&state.execution.mnemonics, mnemonic_name);
    let macro_maybe = find_macro(&state.execution.macros, full_mnemonic_or_macro_name.as_str());

    /* Parse the rest of the line */
    while buffer_pos < buffer.len() && buffer.at(buffer_pos) == " " {
        buffer_pos += 1;
    }

    while buffer_pos < buffer.len() {
        if buffer.at(buffer_pos) == " " {
            while buffer_pos < buffer.len() && buffer.at(buffer_pos + 1) == " " {
                buffer_pos += 1;
            }
        }

        if buffer.at(buffer_pos).as_bytes()[0] as u8 == 0x80 {
            buffer.replace_range(buffer_pos..buffer_pos + 1, " ");
        }

        av_2.push_str(buffer.at(buffer_pos));
        buffer_pos += 1;
    }

    // Set value
    set_argument(2, av_2);

    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("Ended with mne is_some :: [[{}]]", if mnemonic_maybe.is_some() { "true" } else { "false" }); } }

    if mnemonic_maybe.is_some() {
        MacroOrMnemonicPointer::Mnemonic(mnemonic_maybe.unwrap())
    } else if macro_maybe.is_some() {
        MacroOrMnemonicPointer::Macro(macro_maybe.unwrap())
    } else {
        MacroOrMnemonicPointer::None
    }
}

// FIXME: this should use vectors instead
pub unsafe fn addhashtable(first_mne: *mut _MNE, len: usize) {
    let mut i: usize;
    let mut j: usize;
    let mut opcode: [u32; 21] = [0; 21];
    for m in 0..len {
        let mne = first_mne.offset(m as isize);
        // FIXME: this should be just:
        //   opcode = (*mne).opcode.clone();
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

        state.execution.mnemonics.push(mne);
    };
}

pub unsafe fn pushinclude(str: *const i8) {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("[[{}]]", transient::str_pointer_to_string(str)); } }

    let mut inf: *mut _INCFILE = 0 as *mut _INCFILE;
    let mut fi: *mut FILE = 0 as *mut FILE;
    fi = pfopen(str, b"rb\x00" as *const u8 as *const i8);
    if !fi.is_null() {
        if OPTIONS.verbosity as u8 > Verbosity::Two as u8 {
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
        inf = transient::zmalloc(::std::mem::size_of::<_INCFILE>() as u64 as i32) as *mut _INCFILE;
        (*inf).name = strcpy(transient::ckmalloc(strlen(str).wrapping_add(1) as i32), str);
        (*inf).fi = fi;
        (*inf).lineno = 0;
        state.execution.includeFiles.push(inf);
        return;
    }
    println!("Warning: Unable to open '{}'", transient::str_pointer_to_string(str));
}

/**
 * Safely set an argument for our argument stack at a position.
 * This is made to replicate the *Av functionality.
 * FIXME: make this more elegant.
 */
pub unsafe fn set_argument(position: usize, new_value: String) {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("{} => [[{}]]", position, new_value); } }

    while state.execution.argumentStack.len() <= position {
        state.execution.argumentStack.push(String::new());
    }
    state.execution.argumentStack[position] = new_value;
}

pub unsafe fn get_argument(position: usize) -> String {
    match state.execution.argumentStack.get(position) {
        Some(result) => result.to_owned(),
        None => String::new()
    }
}

pub unsafe fn asmerr(err: AsmErrorEquates, abort: bool, sText: *const i8) -> AsmErrorEquates {
    let mut errorOutput: String = String::new();
    /* file pointer we print error messages to */
    let errorToFile = !OPTIONS.list_file.is_empty();
    let errorFile = &mut state.output.listFile;

    if find_error_definition(err).fatal {
        state.other.stopAtEnd = true;
    }

    let mut last_include_file = *state.execution.includeFiles.last().unwrap();
    while state.execution.includeFiles.len() > 0 {
        if (*last_include_file).flags & FileFlags::Macro != 0 {
            state.execution.includeFiles.pop();
            last_include_file = *state.execution.includeFiles.last().unwrap();
        } else {
            break;
        }
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
    match OPTIONS.error_format {
        ErrorFormat::Woe => {
            /*
                Error format for MS VisualStudio and relatives:
                "file (line): error: string"
            */
            let errorMessage = format!(
                "{} ({}): error: ",
                transient::str_pointer_to_string((*last_include_file).name),
                (*last_include_file).lineno
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
                (*last_include_file).lineno,
                transient::str_pointer_to_string((*last_include_file).name),
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
                transient::str_pointer_to_string((*last_include_file).name),
                (*last_include_file).lineno,
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
        std::process::exit(ExitCode::Failure as u8 as i32);
    }
    return err;
}

pub fn main() {
    // FIXME: remove this unsafe when possible
    unsafe {
        let nError: AsmErrorEquates = main_shadow();
        if nError != AsmErrorEquates::None && nError != AsmErrorEquates::NonAbort {
            // Dump messages when aborting due to errors
            operations::output_passbuffer(&mut state.output.passBufferMessages);
            // Only print errors if assembly is unsuccessful
            operations::output_passbuffer(&mut state.output.passBufferErrors);
            println!("Fatal assembly error: {}", find_error_definition(nError).description);
        }
        dump_symbol_table(OPTIONS.sort_mode == SortMode::Address);
        operations::clear_passbuffer(&mut state.output.passBufferErrors);
        operations::clear_passbuffer(&mut state.output.passBufferMessages);
        process::exit(nError as u8 as i32);
    }
}
