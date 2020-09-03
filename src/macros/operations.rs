use libc;

// FIXME: remove all unsafe() used in this module

use crate::constants::{
    MAX_MACRO_LEVEL,
};
use crate::globals::state;
use crate::types::legacy::{
    _INCFILE,
    _MACRO,
    _STRLIST,
};
use crate::utils::{
    transient,
};

#[cfg(debug_assertions)]
use crate::{
    OPTIONS,
};

#[cfg(debug_assertions)]
use crate::{
    log_function_with,
};


extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;

    // From others
    #[no_mangle]
    fn programlabel();
}

pub unsafe fn v_execmac(mut str: *mut i8, mac: *mut _MACRO) {
    #[cfg(debug_assertions)]
    { if OPTIONS.debug_extended { log_function_with!("[[{}]] [[{}]]", transient::str_pointer_to_string(str), if mac.is_null() { String::from("null") } else { transient::str_pointer_to_string((*mac).name) }); } }

    let mut inc: *mut _INCFILE = 0 as *mut _INCFILE;
    let mut base: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut psl: *mut *mut _STRLIST = 0 as *mut *mut _STRLIST;
    let mut sl: *mut _STRLIST = 0 as *mut _STRLIST;
    let mut s1: *mut i8 = 0 as *mut i8;
    programlabel();
    if state.execution.macroLevel == MAX_MACRO_LEVEL {
        println!("infinite macro recursion");
        return;
    }
    state.execution.macroLevel += 1;
    base = transient::ckmalloc((std::mem::size_of::<*mut _STRLIST>() as u64).wrapping_add(strlen(str)).wrapping_add(1) as i32) as *mut _STRLIST;
    (*base).next = 0 as *mut _STRLIST;
    strcpy((*base).buf.as_mut_ptr(), str);
    psl = &mut (*base).next;
    while *str as i32 != 0 && *str as i32 != '\n' as i32 {
        s1 = str;
        while *str as i32 != 0 && *str as i32 != '\n' as i32
                  && *str as i32 != ',' as i32 {
            str = str.offset(1)
        }
        sl = transient::ckmalloc((::std::mem::size_of::<*mut _STRLIST>() as u64).wrapping_add(1) as i32) as *mut _STRLIST;
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
        (*sl).buf[str.wrapping_offset_from(s1) as i64 as usize] = 0;
        if *str as i32 == ',' as i32 { str = str.offset(1) }
        while *str as i32 == ' ' as i32 { str = str.offset(1) }
    }
    inc = transient::zmalloc(::std::mem::size_of::<_INCFILE>() as u64 as i32) as *mut _INCFILE;
    let last_include_file = *state.execution.includeFiles.last().unwrap();
    (*inc).name = (*mac).name;
    (*inc).fi = (*last_include_file).fi;
    (*inc).lineno = 0;
    (*inc).flags = 0x1;
    (*inc).saveidx = state.execution.localIndex;
    (*inc).savedolidx = state.execution.localDollarIndex;
    (*inc).strlist = (*mac).strlist;
    (*inc).args = base;
    println!("!!! EXECMAC for '{}' with [[{}]]", transient::str_pointer_to_string((*mac).name), transient::strlist_to_string((*mac).strlist));
    state.execution.includeFiles.push(inc);
    state.execution.lastLocalIndex += 1;
    state.execution.localIndex = state.execution.lastLocalIndex;
    state.execution.lastLocalDollarIndex += 1;
    state.execution.localDollarIndex = state.execution.lastLocalDollarIndex;
}
