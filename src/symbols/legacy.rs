use libc;

use crate::constants::{
    MAX_SYMBOLS,
    S_HASH_AND,
};
use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
};
use crate::types::enums::{
    AsmErrorEquates,
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
    static mut SHash: [*mut _SYMBOL; 0];
    #[no_mangle]
    static mut Csegment: *mut _SEGMENT;
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
    let mut buf: [libc::c_char; MAX_SYMBOLS + 14] = [0; MAX_SYMBOLS + 14];
    if len > MAX_SYMBOLS as libc::c_int { len = MAX_SYMBOLS as libc::c_int }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        if len == 1 as libc::c_int {
            if (*Csegment).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                org.flags =
                    ((*Csegment).rflags as libc::c_int & 0x1 as libc::c_int)
                        as libc::c_uchar;
                org.value = (*Csegment).rorg as libc::c_long
            } else {
                org.flags =
                    ((*Csegment).flags as libc::c_int & 0x1 as libc::c_int) as
                        libc::c_uchar;
                org.value = (*Csegment).org as libc::c_long
            }
            return &mut org
        }
        if len == 2 as libc::c_int &&
               *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '.' as i32 {
            return &mut special
        }
        if len == 3 as libc::c_int &&
               *str.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '.' as i32 &&
               *str.offset(2 as libc::c_int as isize) as libc::c_int ==
                   '.' as i32 {
            specchk.flags = 0 as libc::c_int as libc::c_uchar;
            specchk.value = CheckSum as libc::c_long;
            return &mut specchk
        }
        sprintf(buf.as_mut_ptr(),
                b"%ld%.*s\x00" as *const u8 as *const libc::c_char,
                Localindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    } else if *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int ==
                  '$' as i32 {
        sprintf(buf.as_mut_ptr(),
                b"%ld$%.*s\x00" as *const u8 as *const libc::c_char,
                Localdollarindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
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
    let mut buf: [libc::c_char; MAX_SYMBOLS + 14] = [0; MAX_SYMBOLS + 14];
    if len > MAX_SYMBOLS as libc::c_int { len = MAX_SYMBOLS as libc::c_int }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        sprintf(buf.as_mut_ptr(),
                b"%ld%.*s\x00" as *const u8 as *const libc::c_char,
                Localindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    } else if *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int ==
                  '$' as i32 {
        sprintf(buf.as_mut_ptr(),
                b"%ld$%.*s\x00" as *const u8 as *const libc::c_char,
                Localdollarindex, len, str);
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        str = buf.as_mut_ptr()
    }
    sym = allocsymbol();
    (*sym).name = permalloc(len + 1 as libc::c_int);
    memcpy((*sym).name as *mut libc::c_void, str as *const libc::c_void,
           len as libc::c_ulong);
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
    let mut result: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 != 0) { break ; }
        let fresh2 = str;
        str = str.offset(1);
        result = result << 2 as libc::c_int ^ *fresh2 as libc::c_uint
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
    let mut cseg: *mut _SEGMENT = Csegment;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rorg: libc::c_uchar =
        ((*cseg).flags as libc::c_int & 0x20 as libc::c_int) as libc::c_uchar;
    let mut cflags: libc::c_uchar =
        if rorg as libc::c_int != 0 {
            (*cseg).rflags as libc::c_int
        } else { (*cseg).flags as libc::c_int } as libc::c_uchar;
    let mut pc: libc::c_ulong =
        if rorg as libc::c_int != 0 { (*cseg).rorg } else { (*cseg).org };
    Plab = (*cseg).org;
    Pflags = (*cseg).flags as libc::c_ulong;
    str = *Av.as_mut_ptr().offset(0 as libc::c_int as isize);
    if *str as libc::c_int == 0 as libc::c_int { return }
    len = strlen(str) as libc::c_int;
    if *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int ==
           ':' as i32 {
        len -= 1
    }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 &&
           *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int !=
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
        if (*sym).flags as libc::c_int &
               (0x1 as libc::c_int | 0x4 as libc::c_int) ==
               0x1 as libc::c_int | 0x4 as libc::c_int {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::ForwardReference;
            if state.parameters.debug {
                printf(b"redo 13: \'%s\' %04x %04x\n\x00" as *const u8 as
                           *const libc::c_char, (*sym).name,
                       (*sym).flags as libc::c_int, cflags as libc::c_int);
            }
        } else if cflags as libc::c_int & 0x1 as libc::c_int != 0 &&
                      (*sym).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            state.execution.redoIndex += 1;
            state.execution.redoWhy |= ReasonCodes::ForwardReference
        } else if cflags as libc::c_int & 0x1 as libc::c_int == 0 &&
                      (*sym).flags as libc::c_int & 0x1 as libc::c_int == 0 {
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
                    let mut sBuffer: [libc::c_char; MAX_SYMBOLS * 4] = [0; MAX_SYMBOLS * 4];
                    sprintf(sBuffer.as_mut_ptr(),
                            b"%s %s\x00" as *const u8 as *const libc::c_char,
                            (*sym).name,
                            sftos((*sym).value, 0 as libc::c_int));
                    asmerr(AsmErrorEquates::LabelMismatch,
                           0 as libc::c_int != 0, sBuffer.as_mut_ptr());
                }
                state.execution.redoIndex += 1;
                state.execution.redoWhy |= ReasonCodes::PhaseError
            }
        }
    } else { sym = CreateSymbol(str, len) }
    (*sym).value = pc as libc::c_long;
    (*sym).flags =
        ((*sym).flags as libc::c_int & !(0x1 as libc::c_int) |
             cflags as libc::c_int & 0x1 as libc::c_int) as libc::c_uchar;
}
#[no_mangle]
pub static mut SymAlloc: *mut _SYMBOL = 0 as *const _SYMBOL as *mut _SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn allocsymbol() -> *mut _SYMBOL {
    let mut sym: *mut _SYMBOL = 0 as *mut _SYMBOL;
    if !SymAlloc.is_null() {
        sym = SymAlloc;
        SymAlloc = (*SymAlloc).next;
        memset(sym as *mut libc::c_void, 0 as libc::c_int,
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
