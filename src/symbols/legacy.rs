use libc;

use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SegmentTypes,
    SymbolTypes,
};
use crate::types::structs::{
    Symbol,
};
use crate::types::enums::{
    AsmErrorEquates,
};
use crate::utils::{
    formatting,
    transient,
};

extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: u64) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut Av: [*mut i8; 0];
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const i8)
     -> i32;
    #[no_mangle]
    fn permalloc(bytes: i32) -> *mut i8;
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

// FIXME: move to module
pub unsafe extern "C" fn create_symbol(name: &str) -> &mut Symbol {
    // FIXME: this is a bit duplicated from find_symbol(), extract?
    let mut usedName = String::from(name);
    if name.starts_with(".") {
        usedName = format!("{}{}", state.execution.localIndex, name);
    } else if name.ends_with("$") {
        usedName = format!("{}${}", state.execution.localDollarIndex, name);
    }

    let mut symbol = Symbol::new();
    symbol.name = usedName;
    symbol.flags = SymbolTypes::Unknown;
    state.execution.symbols.insert(0, symbol);
    &mut state.execution.symbols[0]
}

/*
*  Label Support Routines
*/
#[no_mangle]
pub unsafe extern "C" fn programlabel() {
    let currentSegment = &mut state.other.segments[state.other.currentSegment];
    let rorg = currentSegment.flags & SegmentTypes::RelocatableOrigin;
    let (cflags, pc) = if rorg != 0 {
        (currentSegment.rflags, currentSegment.rorg)
    } else {
        (currentSegment.flags, currentSegment.org)
    };
    state.execution.programOrg = currentSegment.org;
    state.execution.programFlags = currentSegment.flags;
    let mut name = transient::str_pointer_to_string(*Av.as_mut_ptr());
    if name.len() == 0 {
        return;
    }

    if name.ends_with(":") {
        name.pop();
    }
    if name.starts_with(".") && name.ends_with("$") {
        state.execution.lastLocalDollarIndex += 1;
        state.execution.localDollarIndex = state.execution.lastLocalDollarIndex;
    }

    // Redo: unknown and referenced
    //   referenced and origin not known
    //   known and phase error	 (origin known)

    let symbolSearch = find_symbol(name.as_str());
    let symbol: &mut Symbol;
    match symbolSearch {
        Some(result) => {
            symbol = result;
            if symbol.flags & (SymbolTypes::Unknown | SymbolTypes::Referenced) == SymbolTypes::Unknown | SymbolTypes::Referenced {
                state.execution.redoIndex += 1;
                state.execution.redoWhy |= ReasonCodes::ForwardReference;
                if state.parameters.debug {
                    println!(
                        "redo 13: '{}' {:04x} {:04x}",
                        symbol.name,
                        symbol.flags,
                        cflags,
                    );
                }
            } else if cflags & SymbolTypes::Unknown != 0 && symbol.flags & SymbolTypes::Referenced != 0 {
                state.execution.redoIndex += 1;
                state.execution.redoWhy |= ReasonCodes::ForwardReference;
            } else if cflags & SymbolTypes::Unknown == 0 && symbol.flags & SymbolTypes::Referenced == 0 {
                if pc != symbol.value {
                    // If we had an unevaluated IF expression in the
                    // previous pass, don't complain about phase errors
                    // too loudly

                    // FIX: calling asmerr with ERROR_LABEL_MISMATCH is fatal. The clause
                    //      below was causing aborts if verbosity was up, even when the
                    //      phase errors were the result of unevaluated IF expressions in
                    //      the previous pass.
                    // if (state.verbose >= 1 || !(st.execution.redoIf & (ReasonCodes::Obscure)))
                    if state.execution.redoIf & ReasonCodes::Obscure == 0 {
                        let buffer = format!(
                            "{} {}",
                            symbol.name,
                            formatting::segment_address_to_string(symbol.value, 0),
                        );
                        asmerr(AsmErrorEquates::LabelMismatch, false, transient::string_to_str_pointer(buffer));
                    }
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::PhaseError
                }
            }
        },
        None => {
            symbol = create_symbol(name.as_str());
        },
    }

    symbol.value = pc;
    symbol.flags = symbol.flags & !(SymbolTypes::Unknown) | cflags & SymbolTypes::Unknown;
}
