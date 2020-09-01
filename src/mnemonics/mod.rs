use crate::types::legacy::{
    _MNE,
};
use crate::utils::{
    transient,
};

/**
 * Given a mnemonic (e.g "dc.b") parses the name ("dc") and the extension ("b")
 * if one exists.
 * Names starting with "." (e.g. ".op") are simple instructions with no extension,
 * for compatibility, so the period is removed.
 * This is made to replace some strange functionality by findext() and findmne()
 * in main.c.
 */
pub fn parse_mnemonic_name(full_mnemonic: &str) -> (&str, &str) {
    // Compatibility mode: ".OP" has name "OP"
    if full_mnemonic.starts_with(".") {
        (&full_mnemonic[1..], "")
    } else {
        match full_mnemonic.find(".") {
            Some(pos) => (&full_mnemonic[..pos], &full_mnemonic[pos + 1..]),
            _ => (&full_mnemonic, "")
        }
    }
}

/**
 * Find a mnemonic in a list, by name.
 * FIXME: this is terrible because of the pointer song-and-dance; convert to a simple
 * list search later
 * FIXME: remove the unsafe
 */
pub unsafe fn find_mnemonic(mnemonics: &Vec<*mut _MNE>, name: &str) -> Option<*mut _MNE> {
    let name_to_find = name.to_ascii_lowercase();
    // Search for a mnemonic in the list. This is done *in reverse* because mnemonics might
    // have been added later with duplicate names, effectively overriding the initially added
    // mnemonics.
    // Ideally, we'd instead overwrite the names in the list (since the old mnemonics are simply
    // taking up space) but, for now, we keep them in the list to mimic the original dasm C
    // code behavior (where it used a linked list to search, but added new items to the head).
    match mnemonics.iter().rev().find(|&&m| {
        transient::str_pointer_to_string((*m).name).to_ascii_lowercase() == name_to_find
    }) {
        Some(result) => Some(*result),
        None => None,
    }
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_mnemonic_name() {
        assert_eq!(parse_mnemonic_name("ab"), ("ab", ""));
        assert_eq!(parse_mnemonic_name("ab"), ("ab", ""));
        assert_eq!(parse_mnemonic_name("ab.cd"), ("ab", "cd"));
	}
}
