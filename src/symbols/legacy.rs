use libc;

use crate::constants::{
    MAX_SYMBOLS,
};
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
    transient,
};

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
    static mut Av: [*mut libc::c_char; 0];
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
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sftos(val: libc::c_long, flags: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn permalloc(bytes: libc::c_int) -> *mut libc::c_char;
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

// FIXME: move to symbols module
#[no_mangle]
pub unsafe extern "C" fn set_special_symbol(value: u64, flags: u8) {
    let st = &mut state.lock().unwrap();
    st.execution.specialSymbol.value = value;
    st.execution.specialSymbol.flags = flags;
}

// FIXME: move to symbols module
#[no_mangle]
pub unsafe extern "C" fn find_symbol(name: &str) -> Option<&mut Symbol> {
    let st = &mut state.lock().unwrap();
    let mut usedName: String = String::from(name);
    if name.starts_with(".") {
        if name.len() == 1 {
            let mut currentSegment = &mut st.other.segments[st.other.currentSegment];
            if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
                st.execution.orgSymbol.flags = currentSegment.rflags & SymbolTypes::Unknown;
                st.execution.orgSymbol.value = currentSegment.rorg;
            } else {
                st.execution.orgSymbol.flags = currentSegment.flags & SymbolTypes::Unknown;
                st.execution.orgSymbol.value = currentSegment.org;
            }
            return Some(&mut st.execution.orgSymbol);
        } else if name.len() == 2 && name == ".." {
            return Some(&mut st.execution.specialSymbol);
        } else if name.len() == 3 && name == "..." {
            st.execution.specialCheckSymbol.flags = 0;
            st.execution.specialCheckSymbol.value = CheckSum;
            return Some(&mut st.execution.specialCheckSymbol);
        }

        usedName = format!("{}{}", Localindex, name);
    } else if name.ends_with("$") {
        usedName = format!("{}${}", Localdollarindex, name);
    }

    return st.execution.symbols.iter_mut().find(|&symbol| symbol.name == usedName);
}

// FIXME: move to module
#[no_mangle]
pub unsafe extern "C" fn create_symbol(name: &str) -> Symbol {
    let st = &mut state.lock().unwrap();
    // FIXME: this is a bit duplicated from find_symbol(), extract?
    let usedName = String::from(name);
    if name.starts_with(".") {
        usedName = format!("{}{}", Localindex, name);
    } else if name.ends_with("$") {
        usedName = format!("{}${}", Localdollarindex, name);
    }

    let mut symbol = Symbol::new();
    symbol.name = usedName;
    symbol.flags = SymbolTypes::Unknown;
    st.execution.symbols.insert(0, symbol);
    return symbol;
}

/*
*  Label Support Routines
*/
#[no_mangle]
pub unsafe extern "C" fn programlabel() {
    let st = &mut state.lock().unwrap();
    let currentSegment = &st.other.segments[st.other.currentSegment];
    let rorg = currentSegment.flags & SegmentTypes::RelocatableOrigin;
    let (cflags, pc) = if rorg != 0 {
        (currentSegment.rflags, currentSegment.rorg)
    } else {
        (currentSegment.flags, currentSegment.org)
    };
    Plab = currentSegment.org;
    Pflags = currentSegment.flags as libc::c_ulong;
    let mut name = transient::str_pointer_to_string(*Av.as_mut_ptr());
    if name.len() == 0 {
        return;
    }

    if name.ends_with(":") {
        name.truncate(name.len() - 1);
    }
    if name.starts_with(".") && name.ends_with("$") {
        Lastlocaldollarindex = Lastlocaldollarindex.wrapping_add(1);
        Localdollarindex = Lastlocaldollarindex
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
                st.execution.redoIndex += 1;
                st.execution.redoWhy |= ReasonCodes::ForwardReference;
                if st.parameters.debug {
                    println!(
                        "redo 13: '{}' {:04x} {:04x}",
                        symbol.name,
                        symbol.flags,
                        cflags,
                    );
                }
            } else if cflags & SymbolTypes::Unknown != 0 && symbol.flags & SymbolTypes::Referenced != 0 {
                st.execution.redoIndex += 1;
                st.execution.redoWhy |= ReasonCodes::ForwardReference;
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
                    if st.execution.redoIf & ReasonCodes::Obscure == 0 {
                        let mut sBuffer: [libc::c_char; MAX_SYMBOLS * 4] = [0; MAX_SYMBOLS * 4];
                        sprintf(
                            sBuffer.as_mut_ptr(),
                            b"%s %s\x00" as *const u8 as *const libc::c_char,
                            symbol.name,
                            sftos(symbol.value as i64, 0)
                        );
                        asmerr(AsmErrorEquates::LabelMismatch, false, sBuffer.as_mut_ptr());
                    }
                    st.execution.redoIndex += 1;
                    st.execution.redoWhy |= ReasonCodes::PhaseError
                }
            }
        },
        None => {
            symbol = &mut create_symbol(name.as_str());
        },
    }

    symbol.value = pc;
    symbol.flags = symbol.flags & !(SymbolTypes::Unknown) | cflags & SymbolTypes::Unknown;
}
#[no_mangle]
pub static mut SymAlloc: *mut _SYMBOL = 0 as *const _SYMBOL as *mut _SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn allocsymbol() -> *mut _SYMBOL {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if !SymAlloc.is_null() {
        sym = SymAlloc;
        SymAlloc = (*SymAlloc).next;
        memset(sym as *mut libc::c_void, 0,
               ::std::mem::size_of::<_SYMBOL>() as libc::c_ulong);
    } else {
        sym =
            permalloc(::std::mem::size_of::<_SYMBOL>() as libc::c_ulong as
                          libc::c_int) as *mut _SYMBOL
    }
    return sym;
}
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
