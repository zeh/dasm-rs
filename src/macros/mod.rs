use crate::types::legacy::{
    _MACRO,
};
use crate::utils::{
    transient,
};

// FIXME: make this safe once it's possible
pub type MacroFunc = unsafe fn(str: *mut i8, mac: *mut _MACRO) -> ();

/**
 * Find a macro in a list, by name.
 * FIXME: this is terrible because of the pointer song-and-dance; convert to a simple
 * list search later
 * FIXME: remove the unsafe
 */
pub unsafe fn find_macro(macros: &Vec<*mut _MACRO>, name: &str) -> Option<*mut _MACRO> {
    let name_to_find = name.to_ascii_lowercase();
    // Search for a macro in the list. This is done *in reverse* because macros might
    // have been added later with duplicate names, effectively overriding the initially added
    // mnemonics.
    // Ideally, we'd instead overwrite the names in the list (since the old macros are simply
    // taking up space) but, for now, we keep them in the list to mimic the original dasm C
    // code behavior (where it used a linked list to search, but added new items to the head).
    match macros.iter().rev().find(|&&m| {
        transient::str_pointer_to_string((*m).name).to_ascii_lowercase() == name_to_find
    }) {
        Some(result) => Some(*result),
        None => None,
    }
}
