use std::num::ParseIntError;

/**
 * Parses a character into its hexadecimal equivalent.
 * In original C code, "gethexdig()" in ops.c
 */
pub fn parse_hex_digit(c: u8) -> Result<u8, ParseIntError> {
    u8::from_str_radix(c, 16)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_hex_digit() {
		assert_eq!(parse_hex_digit('0').unwrap(), 0);
        assert_eq!(parse_hex_digit('1').unwrap(), 1);
        assert_eq!(parse_hex_digit('9').unwrap(), 9);
        assert_eq!(parse_hex_digit('a').unwrap(), 10);
        assert_eq!(parse_hex_digit('F').unwrap(), 15);
	}
}
