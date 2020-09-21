// FIXME: used for conversion only, remove this once transient functions are not needed anymore
use std::ffi::CString;
use std::os::raw::c_char;

use crate::types::legacy::{
	_STRLIST,
};

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
			if str_value == 0x80 {
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
