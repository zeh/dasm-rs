use std::cmp::Ordering;

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
			}
			Ordering::Greater => {
				return Ordering::Greater;
			}
			_ => {}
		}
	}
	Ordering::Equal
}

/**
 * Compares two values, returning whether the first value
 * lesser, equal, or greater in alphabetic order.
 * In original C code, "CompareAddress()" in main.c
 */
pub fn compare_address(arg1: u64, arg2: u64) -> Ordering {
	if arg1 < arg2 {
		return Ordering::Less;
	}
	if arg1 > arg2 {
		return Ordering::Greater;
	}
	Ordering::Equal
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

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
	fn test_compare_address() {
		assert_eq!(compare_address(0, 1), Ordering::Less);
		assert_eq!(compare_address(0, 0), Ordering::Equal);
		assert_eq!(compare_address(1, 0), Ordering::Greater);
		assert_eq!(compare_address(2, 1000), Ordering::Less);
		assert_eq!(compare_address(13134, 13134), Ordering::Equal);
		assert_eq!(compare_address(883, 23), Ordering::Greater);
	}
}
