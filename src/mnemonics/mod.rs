pub mod operations;
pub mod processors;

use crate::types::legacy::{
    _MNE,
};
use crate::utils::{
    transient,
};
use crate::utils::extensions::{
    StringExtensions,
};
use crate::types::enums::{
    AddressModes,
};

// FIXME: make this safe once it's possible
pub type MnemonicFunc = unsafe fn(str: *mut i8, mnemonic: *mut _MNE) -> ();

/**
 * Given a mnemonic (e.g "dc.b") parses the name ("dc") and the extension ("b")
 * if one exists.
 * Names starting with "." (e.g. ".op") are simple instructions with no extension,
 * for compatibility.
 * This is made to replace some strange functionality by findext() and findmne()
 * in main.c.
 */
pub fn parse_mnemonic_name(full_mnemonic: &str) -> (&str, &str) {
    if full_mnemonic.starts_with(".") {
        (&full_mnemonic, "")
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
pub unsafe fn find_mnemonic<'a>(mnemonics: &'a Vec<_MNE>, name: &str) -> Option<&'a _MNE> {
    let name_to_find = (if name.starts_with(".") {
		&name[1..]
	} else {
		name
	})
	.to_ascii_lowercase();
    // Search for a mnemonic in the list. This is done *in reverse* because mnemonics might
    // have been added later with duplicate names, effectively overriding the initially added
    // mnemonics.
    // Ideally, we'd instead overwrite the names in the list (since the old mnemonics are simply
    // taking up space) but, for now, we keep them in the list to mimic the original dasm C
    // code behavior (where it used a linked list to search, but added new items to the head).
    match mnemonics.iter().rev().find(|&m| {
        transient::str_pointer_to_string(m.name).to_ascii_lowercase() == name_to_find
    }) {
        Some(result) => Some(result),
        None => None,
    }
}

/**
 * Given a mnemonic extension, find the next address mode
 */
pub fn find_mnemonic_extension_address_mode(extension: &str) -> AddressModes {
	match extension.at(0).to_ascii_lowercase().as_str() {
		"0" | "i" => match extension.at(1).to_ascii_lowercase().as_str() {
			"x" => AddressModes::ZeroX,
			"y" => AddressModes::ZeroY,
			"n" => AddressModes::IndWord,
			_ => AddressModes::Imp,
		},
		"d" | "b" | "z" => match extension.at(1).to_ascii_lowercase().as_str() {
			"x" => AddressModes::ByteAdrX,
			"y" => AddressModes::ByteAdrY,
			"i" => AddressModes::BitMod,
			"b" => AddressModes::BitBraMod,
			_ => AddressModes::ByteAdr,
		},
		"e" | "w" | "a" => match extension.at(1).to_ascii_lowercase().as_str() {
			"x" => AddressModes::WordAdrX,
			"y" => AddressModes::WordAdrY,
			_ => AddressModes::WordAdr,
		},
		"l" => AddressModes::Long,
		"r" => AddressModes::Rel,
		"u" => AddressModes::BSS,
		_ => AddressModes::None,
	}
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_mnemonic_name() {
        assert_eq!(parse_mnemonic_name(".ab"), (".ab", ""));
        assert_eq!(parse_mnemonic_name("ab"), ("ab", ""));
        assert_eq!(parse_mnemonic_name("ab.cd"), ("ab", "cd"));
	}
}
