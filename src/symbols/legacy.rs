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
    static mut SHash: [*mut _SYMBOL; 0];
    #[no_mangle]
    static mut Av: [*mut libc::c_char; 0];
    #[no_mangle]
    static mut Localindex: libc::c_ulong;
    #[no_mangle]
    static mut Localdollarindex: libc::c_ulong;
    #[no_mangle]
    static mut Lastlocaldollarindex: libc::c_ulong;
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const libc::c_char)
     -> libc::c_int;
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
static mut org: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
static mut special: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
static mut specchk: _SYMBOL =
    _SYMBOL{next: 0 as *const _SYMBOL as *mut _SYMBOL,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            addrmode: 0,
            value: 0,
            namelen: 0,};
#[no_mangle]
pub unsafe extern "C" fn setspecial(mut value: libc::c_int,
                                    mut flags: libc::c_int) {
    special.value = value as libc::c_long; /* historical */
    special.flags = flags as libc::c_uchar; /* historical */
}
#[no_mangle]
pub unsafe extern "C" fn findsymbol(mut str: *const libc::c_char,
                                    mut len: libc::c_int) -> *mut _SYMBOL {
    let mut h1: libc::c_uint = 0; /*	permalloc zeros the array for us */
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if len > MAX_SYMBOLS as libc::c_int { len = MAX_SYMBOLS as libc::c_int }
    if *str.offset(0 as isize) as libc::c_int == '.' as i32 {
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
               *str.offset(1 as isize) as libc::c_int ==
                   '.' as i32 {
            return &mut special
        }
        if len == 3 &&
               *str.offset(1 as isize) as libc::c_int ==
                   '.' as i32 &&
               *str.offset(2 as isize) as libc::c_int ==
                   '.' as i32 {
            specchk.flags = 0;
            specchk.value = state.output.checksum as i64;
            return &mut specchk
        }

        let buffer = format!(
            "{}{:2$}",
            Localindex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    } else if *str.offset((len - 1) as isize) as libc::c_int == '$' as i32 {
        let buffer = format!(
            "{}${:2$}",
            Localdollarindex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    }
    h1 = hash1(str, len);
    sym = *SHash.as_mut_ptr().offset(h1 as isize);
    while !sym.is_null() {
        if (*sym).namelen == len as libc::c_uint &&
               memcmp((*sym).name as *const libc::c_void,
                      str as *const libc::c_void, len as libc::c_ulong) == 0 {
            break ;
        }
        sym = (*sym).next
    }
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn CreateSymbol(mut str: *const libc::c_char,
                                      mut len: libc::c_int) -> *mut _SYMBOL {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let mut h1: libc::c_uint = 0;
    if len > MAX_SYMBOLS as libc::c_int { len = MAX_SYMBOLS as libc::c_int }
    // FIXME: this duplicates functionality in findsymbol(). Merge or extract?
    if *str.offset(0) as libc::c_int == '.' as i32 {
        let buffer = format!(
            "{}{:2$}",
            Localindex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    } else if *str.offset((len - 1) as isize) as libc::c_int == '$' as i32 {
        let buffer = format!(
            "{}${:2$}",
            Localdollarindex,
            transient::str_pointer_to_string(str),
            len as usize,
        );
        len = buffer.len() as i32;
        str = transient::string_to_str_pointer(buffer);
    }
    sym = allocsymbol();
    (*sym).name = permalloc(len + 1);
    memcpy((*sym).name as *mut libc::c_void, str as *const libc::c_void, len as libc::c_ulong);
    (*sym).namelen = len as libc::c_uint;
    h1 = hash1(str, len);
    (*sym).next = *SHash.as_mut_ptr().offset(h1 as isize);
    (*sym).flags = 0x1 as libc::c_int as libc::c_uchar;
    let ref mut fresh0 = *SHash.as_mut_ptr().offset(h1 as isize);
    *fresh0 = sym;
    return sym;
}
/*
 *  SYMBOLS.C
 */
unsafe extern "C" fn hash1(mut str: *const libc::c_char, mut len: libc::c_int)
 -> libc::c_uint {
    let mut result: libc::c_uint = 0;
    loop  {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 != 0) { break ; }
        let fresh2 = str;
        str = str.offset(1);
        result = result << 2 ^ *fresh2 as libc::c_uint
    }
    return result & S_HASH_AND as libc::c_int as libc::c_uint;
}
/*
*  Label Support Routines
*/
#[no_mangle]
pub unsafe extern "C" fn programlabel() {
    let mut len: libc::c_int = 0;
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    let currentSegment = &state.other.segments[state.other.currentSegment];
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rorg: u8 = currentSegment.flags & SegmentTypes::RelocatableOrigin;
    let mut cflags: u8 = if rorg != 0 {
        currentSegment.rflags
    } else {
        currentSegment.flags
    };
    let mut pc: libc::c_ulong = if rorg != 0 {
        currentSegment.rorg
    } else {
        currentSegment.org
    };
    state.execution.programOrg = currentSegment.org;
    state.execution.programFlags = currentSegment.flags;
    str = *Av.as_mut_ptr().offset(0 as isize);
    if *str as libc::c_int == 0 { return }
    len = strlen(str) as libc::c_int;
    if *str.offset((len - 1) as isize) as libc::c_int ==
           ':' as i32 {
        len -= 1
    }
    if *str.offset(0 as isize) as libc::c_int != '.' as i32 &&
           *str.offset((len - 1) as isize) as libc::c_int !=
               '$' as i32 {
        Lastlocaldollarindex = Lastlocaldollarindex.wrapping_add(1);
        Localdollarindex = Lastlocaldollarindex
    }
    /*
    *	Redo:	unknown and referenced
    *		referenced and origin not known
    *		known and phase error	 (origin known)
    */
    sym = findsymbol(str, len);
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
            if pc != (*sym).value as libc::c_ulong {
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
    } else { sym = CreateSymbol(str, len) }
    (*sym).value = pc as libc::c_long;
    (*sym).flags = ((*sym).flags & !(SymbolTypes::Unknown) | cflags & SymbolTypes::Unknown) as libc::c_uchar;
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
