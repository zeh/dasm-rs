use crate::constants::{
	M_HASH_AND,
	ErrorDefinitions,
};
use crate::types::enums::{
	AsmErrorEquates,
};
use crate::types::structs::{
	ErrorDefinition,
};

// FIXME: drop this once it's not needed anymore
pub mod transient;

pub mod comparing;
pub mod filesystem;
pub mod formatting;

/**
 * Creates a simple hash for a string.
 * In original C code, "hash1()" in main.c
 */
pub fn hash_string(text: String) -> u16 {
	let text_str = text.as_str();
	let mut result: u16 = 0;
	for c in text_str.chars() {
		result = result << 2 ^ c as u16;
	}
	return result & M_HASH_AND;
}

/**
 * Searches for an ErrorDefinition in the ErrorDefinitions list,
 * based on its errorType. This replaces the old index access
 * where the entry would be in the index of the errorType's uint value.
 */
pub fn find_error_definition(errorType: AsmErrorEquates) -> &'static ErrorDefinition {
	return &ErrorDefinitions.iter().find(|e| e.errorType == errorType).unwrap();
}

/**
 * Prints a message and exists.
 * In original C code, "panic()" in main.c
 * FIXME: this should probably be a panic!() instead, but for now we use this to follow original DASM C behavior.
 * The output for errors should also use eprintln!(), but once again, the original used puts() instead.
 */
pub fn panic(message: &str) {
	println!("{}", message);
	std::process::exit(1);
}

/**
 * Extract a typical filename from a string, removing wrapping quotes when neeeded.
 * In original C code, "getfilename()" in ops.c
 */
pub fn get_filename(string: &str) -> &str {
	if string.len() > 0 && &string[0..1] == "\"" {
		match &string[1..].find("\"") {
			Some(pos) => {
				return &string[1..pos + 1];
			}
			_ => {
				return &string[1..];
			}
		}
	}

	&string
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash_string() {
		assert_eq!(hash_string(String::from("")), 0);
		assert_eq!(hash_string(String::from("cli")), 1001);
		assert_eq!(hash_string(String::from("ds")), 483);
		assert_eq!(hash_string(String::from("incbin")), 298);
		assert_eq!(hash_string(String::from("incdir")), 342);
		assert_eq!(hash_string(String::from("list")), 40);
		assert_eq!(hash_string(String::from("processor")), 830);
		assert_eq!(hash_string(String::from("ror")), 750);
		assert_eq!(hash_string(String::from("slo")), 751);
		assert_eq!(hash_string(String::from("subroutine")), 845);
		assert_eq!(hash_string(String::from("tya")), 709);
		assert_eq!(hash_string(String::from("word")), 668);
	}

	#[test]
	fn test_find_error_definition() {
		assert_eq!(find_error_definition(AsmErrorEquates::ValueMustBeLowerThan10).errorType, AsmErrorEquates::ValueMustBeLowerThan10);
		assert_eq!(find_error_definition(AsmErrorEquates::LabelMismatch).errorType, AsmErrorEquates::LabelMismatch);
		assert_eq!(find_error_definition(AsmErrorEquates::EquValueMismatch).fatal, false);
		assert_eq!(find_error_definition(AsmErrorEquates::NotResolvable).description, "Source is not resolvable.");
	}

	#[test]
	fn test_get_filename() {
		assert_eq!(get_filename("a"), "a");
		assert_eq!(get_filename("another"), "another");
		assert_eq!(get_filename("some word or another"), "some word or another");
		assert_eq!(get_filename("../a/name\\a.txt"), "../a/name\\a.txt");
		assert_eq!(get_filename("\"quoted\""), "quoted");
		assert_eq!(get_filename("\"quoted_but.with.chars\""), "quoted_but.with.chars");
		assert_eq!(get_filename("\"../a/name\\a.txt\""), "../a/name\\a.txt");
		assert_eq!(get_filename("\"interrupted"), "interrupted");
	}
}
