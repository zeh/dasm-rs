use libc;

use crate::constants::{
    MAX_SYMBOLS,
    S_HASH_AND,
};
use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SegmentTypes,
    SymbolTypes,
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut SHash: [*mut _SYMBOL; 0];
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
pub struct _SYMBOL {
    pub next: *mut _SYMBOL,
    pub name: *mut i8,
    pub string: *mut i8,
    pub flags: u8,
    pub addrmode: u8,
    pub value: i64,
    pub namelen: u32,
}
static mut org: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const i8 as *mut i8,
            string: 0 as *const i8 as *mut i8,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
static mut special: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const i8 as *mut i8,
            string: 0 as *const i8 as *mut i8,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
static mut specchk: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const i8 as *mut i8,
            string: 0 as *const i8 as *mut i8,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
#[no_mangle]
pub unsafe extern "C" fn setspecial(mut value: i32,
                                    mut flags: i32) {
    special.value = value as i64; /* historical */
    special.flags = flags as u8; /* historical */
}
#[no_mangle]
pub unsafe extern "C" fn findsymbol(mut str: *const i8,
                                    mut len: i32) -> *mut _SYMBOL {
    let mut h1: u32 = 0; /*	permalloc zeros the array for us */
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if len > MAX_SYMBOLS as i32 { len = MAX_SYMBOLS as i32 }
    if *str.offset(0) as i32 == '.' as i32 {
        if len == 1 {
            let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
            if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
                org.flags = currentSegment.rflags & SymbolTypes::Unknown;
                org.value = currentSegment.rorg as i64;
            } else {
                org.flags = currentSegment.flags & SymbolTypes::Unknown;
                org.value = currentSegment.org as i64;
            }
            return &mut org
        }
        if len == 2 &&
               *str.offset(1) as i32 ==
                   '.' as i32 {
            return &mut special
        }
        if len == 3 &&
               *str.offset(1) as i32 ==
                   '.' as i32 &&
               *str.offset(2) as i32 ==
                   '.' as i32 {
            specchk.flags = 0;
            specchk.value = state.output.checksum as i64;
            return &mut specchk
        }

        let buffer = format!(
            "{}{:2$}",
            state.execution.localIndex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    } else if *str.offset((len - 1) as isize) as i32 == '$' as i32 {
        let buffer = format!(
            "{}${:2$}",
            state.execution.localDollarIndex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    }
    h1 = hash1(str, len);
    sym = *SHash.as_mut_ptr().offset(h1 as isize);
    while !sym.is_null() {
        if (*sym).namelen == len as u32 &&
               memcmp((*sym).name as *const libc::c_void,
                      str as *const libc::c_void, len as u64) == 0 {
            break ;
        }
        sym = (*sym).next
    }
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn CreateSymbol(mut str: *const i8,
                                      mut len: i32) -> *mut _SYMBOL {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut h1: u32 = 0;
    if len > MAX_SYMBOLS as i32 { len = MAX_SYMBOLS as i32 }
    // FIXME: this duplicates functionality in findsymbol(). Merge or extract?
    if *str.offset(0) as i32 == '.' as i32 {
        let buffer = format!(
            "{}{:2$}",
            state.execution.localIndex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    } else if *str.offset((len - 1) as isize) as i32 == '$' as i32 {
        let buffer = format!(
            "{}${:2$}",
            state.execution.localDollarIndex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    }
    sym = allocsymbol();
    (*sym).name = permalloc(len + 1);
    memcpy((*sym).name as *mut libc::c_void, str as *const libc::c_void, len as u64);
    (*sym).namelen = len as u32;
    h1 = hash1(str, len);
    (*sym).next = *SHash.as_mut_ptr().offset(h1 as isize);
    (*sym).flags = 0x1 as i32 as u8;
    let ref mut fresh0 = *SHash.as_mut_ptr().offset(h1 as isize);
    *fresh0 = sym;
    return sym;
}
/*
 *  SYMBOLS.C
 */
unsafe extern "C" fn hash1(mut str: *const i8, mut len: i32)
 -> u32 {
    let mut result: u32 = 0;
    loop  {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 != 0) { break ; }
        let fresh2 = str;
        str = str.offset(1);
        result = result << 2 ^ *fresh2 as u32
    }
    return result & S_HASH_AND as i32 as u32;
}
/*
*  Label Support Routines
*/
#[no_mangle]
pub unsafe extern "C" fn programlabel() {
    let currentSegment = &state.other.segments[state.other.currentSegment];
    let mut rorg: u8 = currentSegment.flags & SegmentTypes::RelocatableOrigin;
    let mut cflags: u8 = if rorg != 0 {
        currentSegment.rflags
    } else {
        currentSegment.flags
    };
    let mut pc: u64 = if rorg != 0 {
        currentSegment.rorg
    } else {
        currentSegment.org
    };
    state.execution.programOrg = currentSegment.org;
    state.execution.programFlags = currentSegment.flags;
    let str = transient::str_pointer_to_string(*Av.as_mut_ptr());
    let mut len = str.len();
    if len == 0 {
        return;
    }
    if str.ends_with(":") {
        len -= 1
    }
    if !str.starts_with(".") && !str.ends_with("$") {
        state.execution.lastLocalDollarIndex += 1;
        state.execution.localDollarIndex = state.execution.lastLocalDollarIndex;
    }
    /*
    *	Redo:	unknown and referenced
    *		referenced and origin not known
    *		known and phase error	 (origin known)
    */
    let mut sym: *mut _SYMBOL = findsymbol(transient::string_to_str_pointer(str.clone()), len as i32);
    if !sym.is_null() {
        if (*sym).flags & (SymbolTypes::Unknown | SymbolTypes::Referenced) == SymbolTypes::Unknown | SymbolTypes::Referenced {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::ForwardReference;
            if state.parameters.debug {
                println!(
                    "redo 13: '{}' {:04x} {:04x}",
                    transient::str_pointer_to_string((*sym).name).as_str(),
                    (*sym).flags,
                    cflags,
                );
            }
        } else if cflags & SymbolTypes::Unknown != 0 && (*sym).flags & SymbolTypes::Referenced != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::ForwardReference
        } else if cflags & SymbolTypes::Unknown == 0 && (*sym).flags & SymbolTypes::Unknown == 0 {
            if pc != (*sym).value as u64 {
                /*
                * If we had an unevaluated IF expression in the
                * previous pass, don't complain about phase errors
                * too loudly.
                */
                //FIX: calling asmerr with ERROR_LABEL_MISMATCH is fatal. The clause
                //     below was causing aborts if verbosity was up, even when the
                //     phase errors were the result of unevaluated IF expressions in
                //     the previous pass.
                //if (state.verbose >= 1 || !(state.execution.redoIf & (ReasonCodes::Obscure)))
                if state.execution.redoIf & ReasonCodes::Obscure == 0 {
                    let buffer = format!(
                        "{} {}",
                        transient::str_pointer_to_string((*sym).name),
                        formatting::segment_address_to_string((*sym).value as u64, 0),
                    );
                    asmerr(AsmErrorEquates::LabelMismatch, false, transient::string_to_str_pointer(buffer));
                }
                state.execution.redoIndex += 1;
                state.execution.redoWhy |= ReasonCodes::PhaseError
            }
        }
    } else {
        sym = CreateSymbol(transient::string_to_str_pointer(str), len as i32);
    }
    (*sym).value = pc as i64;
    (*sym).flags = (*sym).flags & !(SymbolTypes::Unknown) | cflags & SymbolTypes::Unknown;
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
               ::std::mem::size_of::<_SYMBOL>() as u64);
    } else {
        sym =
            permalloc(::std::mem::size_of::<_SYMBOL>() as u64 as i32) as *mut _SYMBOL
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
        if (*sym).flags as i32 & 0x8 as i32 != 0 {
            free((*sym).string as *mut libc::c_void);
        }
        SymAlloc = sym;
        sym = next
    };
}
