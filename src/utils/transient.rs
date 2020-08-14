// FIXME: used for conversion only, remove this once transient functions are not needed anymore
use std::ffi::CString;
use std::os::raw::c_char;

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
        while *sstr != 0 {
            sstr_all.push_str(std::str::from_utf8(&[*sstr as u8]).unwrap());
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
 * Converts a String to a *str pointer.
 * FIXME: this should likely be removed in the future, once we drop all uses of *str.
 */
pub fn string_to_str_pointer(str: String) -> *mut c_char {
	CString::new(str).expect("Failed to convert argument into CString.").into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_pointer_to_string() {
        assert_eq!(str_pointer_to_string((b"\x00").as_ptr() as *const c_char), String::from(""));
        assert_eq!(str_pointer_to_string((b"Word 123\x00").as_ptr() as *const c_char), String::from("Word 123"));
    }

    #[test]
    fn test_str_pointer_and_len_to_string() {
        assert_eq!(str_pointer_and_len_to_string((b"\x0012345").as_ptr() as *const c_char, 4), String::from("\u{0}123"));
        assert_eq!(str_pointer_and_len_to_string((b"Word 123\x00").as_ptr() as *const c_char, 5), String::from("Word "));
    }

	#[test]
	fn test_string_to_str_pointer() {
		assert_eq!(
			str_pointer_to_string(string_to_str_pointer(String::from("Word 123"))),
			str_pointer_to_string((b"Word 123\x00").as_ptr() as *const c_char)
		);
	}
}
