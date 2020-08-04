// FIXME: used for conversion only, remove this once transient functions are not needed anymore
use std::os::raw::c_char;

/**
 * Converts a *str (typically used for strings in DASM) to a proper Rust String.
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_pointer_to_string() {
        assert_eq!(str_pointer_to_string((b"\x00").as_ptr() as *const c_char), String::from(""));
        assert_eq!(str_pointer_to_string((b"Word 123\x00").as_ptr() as *const c_char), String::from("Word 123"));
	}
}
