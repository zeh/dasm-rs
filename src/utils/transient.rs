// FIXME: used for conversion only, remove this once transient functions are not needed anymore
use std::ffi::CString;
use std::os::raw::c_char;

use crate::constants::{
    ALLOC_SIZE,
};
use crate::types::legacy::{
	_STRLIST,
	align,
};
use crate::utils::{
	panic,
};

// FIXME: remove when possible
extern "C" {
	#[no_mangle]
	fn malloc(_: u64) -> *mut libc::c_void;
	#[no_mangle]
	fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}

/**
 * Converts a *str until it finds a char 0 to a proper Rust String.
 * This is likely inneficient, but should suffice for now.
 * FIXME: this should likely be removed in the future, once we drop all uses of *str.
 */
pub fn str_pointer_to_string(str: *const c_char) -> String {
	// FIXME: can we use CString/Cstr for this?
	let mut sstr = str;
	let mut sstr_all = String::from("");
	unsafe {
		while !sstr.is_null() && *sstr != 0 {
			let mut str_value = *sstr as u8;
			// Detect 0x80 because it's used internally as some sort of char token
			if str_value >= 0x80 {
				sstr_all.push_str(" ");
			} else {
				sstr_all.push_str(std::str::from_utf8(&[str_value]).unwrap());
			}
			sstr = sstr.offset(1);
		}
	}
	return sstr_all;
}

/**
 * Converts a *str with a length to a proper Rust String.
 * This is likely inneficient, but should suffice for now.
 * FIXME: this should likely be removed in the future, once we drop all uses of *str.
 */
pub fn str_pointer_and_len_to_string(str: *const c_char, len: usize) -> String {
	// FIXME: can we use CString/Cstr for this?
	let mut sstr = str;
	let mut sstr_all = String::from("");
	unsafe {
		while sstr_all.len() < len {
			sstr_all.push_str(std::str::from_utf8(&[*sstr as u8]).unwrap());
			sstr = sstr.offset(1);
		}
	}
	return sstr_all;
}

/**
 * Updates a *str in memory, replacing its contents with the new string.
 * This is needed because in some cases dasm does string changes in
 * memory, while other places depend on the same pointer.
 */
pub fn update_str_pointer_in_place(ptr: *mut i8, new_content: &str) {
	unsafe {
		for (i, char) in new_content.as_bytes().iter().enumerate() {
			*(ptr.offset(i as isize)) = *char as i8;
		}
		*(ptr.offset(new_content.len() as isize)) = 0;
	}
}

/**
 * Converts a String to a *str pointer.
 * FIXME: this should likely be removed in the future, once we drop all uses of *str.
 */
pub fn string_to_str_pointer(str: String) -> *mut c_char {
	CString::new(str).expect("Failed to convert argument into CString.").into_raw()
}

/**
 * Converts a _STRLST, which is a linked list of [i8; 4] buffers, to a String.
 * FIXME: this should likely be removed in the future, once we drop all uses of _STRLST.
 */
pub fn strlist_to_string(strlist: *mut _STRLIST) -> String {
	let mut sstr = strlist;
	let mut sstr_all = String::from("");
	unsafe {
		while !sstr.is_null() {
			let mut i: usize = 0;
			while i < (*sstr).buf.len() && (*sstr).buf[i] != 0 {
				sstr_all.push_str(std::str::from_utf8(&[(*sstr).buf[i] as u8]).unwrap());
				i += 1;
			}
			sstr = (*sstr).next;
		}
	}
	return sstr_all;
}

pub unsafe fn zmalloc(bytes: i32) -> *mut i8 {
	let ptr: *mut i8 = ckmalloc(bytes);
	if !ptr.is_null() {
		memset(ptr as *mut libc::c_void, 0, bytes as u64);
	}
	return ptr;
}

pub unsafe fn ckmalloc(bytes: i32) -> *mut i8 {
	let ptr: *mut i8 = malloc(bytes as u64) as *mut i8;
	if !ptr.is_null() {
		return ptr;
	}
	panic("unable to malloc");
	return 0 as *mut i8;
}

pub unsafe fn permalloc(mut bytes: i32) -> *mut i8 {
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
		memset(buf as *mut libc::c_void, 0, ALLOC_SIZE as i32 as u64);
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_str_pointer_to_string() {
		assert_eq!(str_pointer_to_string((b"\x00").as_ptr() as *const c_char), String::from(""));
		assert_eq!(
			str_pointer_to_string((b"Word 123\x00").as_ptr() as *const c_char),
			String::from("Word 123")
		);
	}

	#[test]
	fn test_str_pointer_and_len_to_string() {
		assert_eq!(
			str_pointer_and_len_to_string((b"\x0012345").as_ptr() as *const c_char, 4),
			String::from("\u{0}123")
		);
		assert_eq!(
			str_pointer_and_len_to_string((b"Word 123\x00").as_ptr() as *const c_char, 5),
			String::from("Word ")
		);
	}

	#[test]
	fn test_string_to_str_pointer() {
		assert_eq!(
			str_pointer_to_string(string_to_str_pointer(String::from("Word 123"))),
			str_pointer_to_string((b"Word 123\x00").as_ptr() as *const c_char)
		);
	}
}
