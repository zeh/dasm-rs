use crate::types::structs::{
	ParsedValue,
};

/// Returns true if an alphanumeric (A-Z, a-z, 0-9) char is passed
pub fn is_alpha_num(c: char) -> bool {
	c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9'
}

/// Given a string, returns a struct with the parsed value
/// and the size of the string used.
/// This replaces "pushbin" in the original C code.
pub fn parse_binary(source: &str) -> ParsedValue<i64> {
	let mut val: i64 = 0;
	let mut size: usize = 0;
	for ch in source.chars() {
		match ch {
			'0' => val = val << 1,
			'1' => val = val << 1 | 1,
			_ => break,
		}
		size += 1;
	}
	ParsedValue::<i64> {
		value: val,
		original_size: size,
	}
}

/// Given a string, returns a struct with the parsed value,
/// and the size of the string used.
/// This replaces "pushchar" in the original C code.
pub fn parse_char(source: &str) -> ParsedValue<char> {
	if source.len() > 0 {
		ParsedValue::<char> {
			value: source.chars().next().unwrap(),
			original_size: 1,
		}
	} else {
		ParsedValue::<char> {
			value: ' ',
			original_size: 0,
		}
	}
}

/// Given a string, returns a struct with the parsed value,
/// and the size of the string used.
/// This replaces "pushhex" in the original C code.
pub fn parse_hexa(source: &str) -> ParsedValue<i64> {
	let mut val: i64 = 0;
	let mut size: usize = 0;
	for ch in source.chars() {
		if ch >= '0' && ch <= '9' {
			val = (val << 4) + (ch as i64 - '0' as i64);
		} else if ch >= 'a' && ch <= 'f' {
			val = (val << 4) + (ch as i64 - 'a' as i64) + 10;
		} else if ch >= 'A' && ch <= 'F' {
			val = (val << 4) + (ch as i64 - 'A' as i64) + 10;
		} else {
			break;
		}
		size += 1;
	}
	ParsedValue::<i64> {
		value: val,
		original_size: size,
	}
}

/// Given a string, returns a struct with the parsed value,
/// and the size of the string used.
/// This replaces "pushoct" in the original C code.
pub fn parse_octal(source: &str) -> ParsedValue<i64> {
	let mut val: i64 = 0;
	let mut size: usize = 0;
	for ch in source.chars() {
		if ch >= '0' && ch <= '7' {
			val = (val << 3) + (ch as i64 - '0' as i64);
		} else {
			break;
		}
		size += 1;
	}
	ParsedValue::<i64> {
		value: val,
		original_size: size,
	}
}

/// Given a string, returns a struct with the parsed value,
/// and the size of the string used.
/// This replaces "pushdec" in the original C code.
pub fn parse_decimal(source: &str) -> ParsedValue<i64> {
	let mut val: i64 = 0;
	let mut size: usize = 0;
	for ch in source.chars() {
		if ch >= '0' && ch <= '9' {
			val = val * 10 + (ch as i64 - '0' as i64);
		} else {
			break;
		}
		size += 1;
	}
	ParsedValue::<i64> {
		value: val,
		original_size: size,
	}
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_alpha_num() {
		assert_eq!(is_alpha_num('A'), true);
		assert_eq!(is_alpha_num('X'), true);
		assert_eq!(is_alpha_num('Z'), true);
		assert_eq!(is_alpha_num('a'), true);
		assert_eq!(is_alpha_num('y'), true);
		assert_eq!(is_alpha_num('z'), true);
		assert_eq!(is_alpha_num('0'), true);
		assert_eq!(is_alpha_num('5'), true);
		assert_eq!(is_alpha_num('9'), true);
		assert_eq!(is_alpha_num('-'), false);
		assert_eq!(is_alpha_num('!'), false);
		assert_eq!(is_alpha_num('@'), false);
		assert_eq!(is_alpha_num(' '), false);
		assert_eq!(is_alpha_num('.'), false);
		assert_eq!(is_alpha_num('é'), false);
	}

	#[test]
	fn test_parse_binary() {
		assert_eq!(
			parse_binary(""),
			ParsedValue::<i64> {
				value: 0,
				original_size: 0
			}
		);
		assert_eq!(
			parse_binary("b"),
			ParsedValue::<i64> {
				value: 0,
				original_size: 0
			}
		);
		assert_eq!(
			parse_binary("0"),
			ParsedValue::<i64> {
				value: 0,
				original_size: 1
			}
		);
		assert_eq!(
			parse_binary("1"),
			ParsedValue::<i64> {
				value: 1,
				original_size: 1
			}
		);
		assert_eq!(
			parse_binary("0a"),
			ParsedValue::<i64> {
				value: 0,
				original_size: 1
			}
		);
		assert_eq!(
			parse_binary("1a"),
			ParsedValue::<i64> {
				value: 1,
				original_size: 1
			}
		);
		assert_eq!(
			parse_binary("100b"),
			ParsedValue::<i64> {
				value: 4,
				original_size: 3
			}
		);
		assert_eq!(
			parse_binary("111b"),
			ParsedValue::<i64> {
				value: 7,
				original_size: 3
			}
		);
		assert_eq!(
			parse_binary("010101"),
			ParsedValue::<i64> {
				value: 21,
				original_size: 6
			}
		);
		assert_eq!(
			parse_binary("10101\n"),
			ParsedValue::<i64> {
				value: 21,
				original_size: 5
			}
		);
		assert_eq!(
			parse_binary("101010-"),
			ParsedValue::<i64> {
				value: 42,
				original_size: 6
			}
		);
		assert_eq!(
			parse_binary("01101 1"),
			ParsedValue::<i64> {
				value: 13,
				original_size: 5
			}
		);
		assert_eq!(
			parse_binary("12"),
			ParsedValue::<i64> {
				value: 1,
				original_size: 1
			}
		);
		assert_eq!(
			parse_binary("0000"),
			ParsedValue::<i64> {
				value: 0,
				original_size: 4
			}
		);
		assert_eq!(
			parse_binary("10000001 "),
			ParsedValue::<i64> {
				value: 129,
				original_size: 8
			}
		);
	}

	#[test]
	fn test_parse_char() {
		assert_eq!(
			parse_char(""),
			ParsedValue::<char> {
				value: ' ',
				original_size: 0
			}
		);
		assert_eq!(
			parse_char(" "),
			ParsedValue::<char> {
				value: ' ',
				original_size: 1
			}
		);
		assert_eq!(
			parse_char("a"),
			ParsedValue::<char> {
				value: 'a',
				original_size: 1
			}
		);
		assert_eq!(
			parse_char("ÇAý"),
			ParsedValue::<char> {
				value: 'Ç',
				original_size: 1
			}
		);
	}

	#[test]
	fn test_parse_hexa() {
		assert_eq!(
			parse_hexa(""),
			ParsedValue::<i64> {
				value: 0,
				original_size: 0
			}
		);
		assert_eq!(
			parse_hexa("f"),
			ParsedValue::<i64> {
				value: 15,
				original_size: 1
			}
		);
		assert_eq!(
			parse_hexa("f_"),
			ParsedValue::<i64> {
				value: 15,
				original_size: 1
			}
		);
		assert_eq!(
			parse_hexa("DEADc0ff33"),
			ParsedValue::<i64> {
				value: 956397846323,
				original_size: 10
			}
		);
		assert_eq!(
			parse_hexa("b33F!"),
			ParsedValue::<i64> {
				value: 45887,
				original_size: 4
			}
		);
	}

	#[test]
	fn test_parse_octal() {
		assert_eq!(
			parse_octal(""),
			ParsedValue::<i64> {
				value: 0,
				original_size: 0
			}
		);
		assert_eq!(
			parse_octal("7"),
			ParsedValue::<i64> {
				value: 7,
				original_size: 1
			}
		);
		assert_eq!(
			parse_octal("78"),
			ParsedValue::<i64> {
				value: 7,
				original_size: 1
			}
		);
		assert_eq!(
			parse_octal("1234567"),
			ParsedValue::<i64> {
				value: 342391,
				original_size: 7
			}
		);
		assert_eq!(
			parse_octal("1234!"),
			ParsedValue::<i64> {
				value: 668,
				original_size: 4
			}
		);
	}

	#[test]
	fn test_parse_decimal() {
		assert_eq!(
			parse_decimal(""),
			ParsedValue::<i64> {
				value: 0,
				original_size: 0
			}
		);
		assert_eq!(
			parse_decimal("1"),
			ParsedValue::<i64> {
				value: 1,
				original_size: 1
			}
		);
		assert_eq!(
			parse_decimal("010_"),
			ParsedValue::<i64> {
				value: 10,
				original_size: 3
			}
		);
		assert_eq!(
			parse_decimal("1097630257"),
			ParsedValue::<i64> {
				value: 1097630257,
				original_size: 10
			}
		);
		assert_eq!(
			parse_decimal("9632!"),
			ParsedValue::<i64> {
				value: 9632,
				original_size: 4
			}
		);
	}
}
