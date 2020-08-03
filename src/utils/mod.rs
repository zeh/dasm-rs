use crate::constants::{
    M_HASH_AND,
};

pub fn hash_string(text: String) -> u16 {
    let text_str = text.as_str();
    let mut result: u16 = 0;
    for c in text_str.chars() {
        result = result << 2 ^ c as u16;
    }
    return result & M_HASH_AND;
}

/**
 * Converts a *str (typically used for strings in DASM) to a proper Rust String.
 * This is likely inneficient, but should suffice for now.
 * FIXME: this should likely be removed in the future, once we drop all uses of *str.
 */
pub fn transient_str_pointer_to_string(str: *const i8) -> String {
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
