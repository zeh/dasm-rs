use std::cmp::Ordering;

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
 * Compares two strings, case insensitive, returning whether the first string is
 * lesser, equal, or greater in alphabetic order.
 * In original C code, "CompareAlpha()" in main.c
 */
pub fn compare_alpha(arg1: &str, arg2: &str) -> Ordering {
    // FIXME: there might be smarter/faster/shorter ways of doing this in Rust
    let s1 = arg1.to_lowercase();
    let s2 = arg2.to_lowercase();
    let mut iter = s1.chars().zip(s2.chars());
    for (c1, c2) in iter {
        let result = &c1.cmp(&c2);
        match result {
            Ordering::Less => {
                return Ordering::Less;
            },
            Ordering::Greater => {
                return Ordering::Greater;
            },
            _ => {},
        }
    }
    Ordering::Equal
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
            },
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
    fn test_compare_alpha() {
        assert_eq!(compare_alpha("b", "a"), Ordering::Greater);
        assert_eq!(compare_alpha("b", "b"), Ordering::Equal);
        assert_eq!(compare_alpha("b", "c"), Ordering::Less);
        assert_eq!(compare_alpha("b", "A"), Ordering::Greater);
        assert_eq!(compare_alpha("b", "B"), Ordering::Equal);
        assert_eq!(compare_alpha("b", "C"), Ordering::Less);
        assert_eq!(compare_alpha("B", "A"), Ordering::Greater);
        assert_eq!(compare_alpha("B", "B"), Ordering::Equal);
        assert_eq!(compare_alpha("B", "C"), Ordering::Less);
        assert_eq!(compare_alpha("B", "a"), Ordering::Greater);
        assert_eq!(compare_alpha("B", "b"), Ordering::Equal);
        assert_eq!(compare_alpha("B", "c"), Ordering::Less);
        assert_eq!(compare_alpha("another", "BANANA"), Ordering::Less);
        assert_eq!(compare_alpha("Banana", "another"), Ordering::Greater);
        assert_eq!(compare_alpha("banana", "BANANA"), Ordering::Equal);
        assert_eq!(compare_alpha("2", "1"), Ordering::Greater);
        assert_eq!(compare_alpha("2", "2"), Ordering::Equal);
        assert_eq!(compare_alpha("2", "3"), Ordering::Less);
        assert_eq!(compare_alpha("a", "1"), Ordering::Greater); // FIXME: technically correct,
        assert_eq!(compare_alpha("z", "1"), Ordering::Greater); // but we might want to revisit
        assert_eq!(compare_alpha("1", "a"), Ordering::Less); // this depending on what the
        assert_eq!(compare_alpha("1", "z"), Ordering::Less); // original actually does
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
