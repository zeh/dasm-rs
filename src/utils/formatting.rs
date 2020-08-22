use crate::types::flags::SymbolTypes;

/**
 * Formats a segment address and flags to a string, also called a "PC" ("Program Counter").
 * In original C code, "sftos()" in main.c
 */
pub fn segment_address_to_string(value: u64, flags: u8) -> String {
	let mut result = String::new();
	result.push_str(format!("{:04x} ", value).as_str());

	result.push_str(if flags & SymbolTypes::Unknown != 0 {
		"???? "
	} else {
		"     "
	});

	result.push_str(if flags & SymbolTypes::StringResult != 0 {
		"str "
	} else {
		"    "
	});

	result.push_str(if flags & SymbolTypes::Macro != 0 {
		"eqm "
	} else {
		"    "
	});

	result.push_str(if flags & (SymbolTypes::MasterReference | SymbolTypes::Set) != 0 {
		"("
	} else {
		" "
	});

	result.push_str(if flags & SymbolTypes::MasterReference != 0 {
		"R"
	} else {
		" "
	});

	result.push_str(if flags & SymbolTypes::Set != 0 {
		"S"
	} else {
		" "
	});

	result.push_str(if flags & (SymbolTypes::MasterReference | SymbolTypes::Set) != 0 {
		")"
	} else {
		" "
	});

	result
}

/**
 * Formats a line, reorganizing tabs into specific column locations.
 * In original C code, "tabit()" in main.rs
 */
pub fn format_line_tabs(line: &str) -> String {
	let mut result: String = String::new();
	let mut spaces: i16 = 0;
	for c in line.chars() {
		if c == '\n' {
			break;
		}

		if c == '\t' {
			// Optimize out spaces before the tab
			while spaces > 0 && result.ends_with(" ") {
				result.remove(result.len() - 1);
				spaces -= 1
			}
			spaces = 0;
			// Recopy the tab
			result.push_str("\t");
		} else {
			result.push(c);
		}

		if spaces == 7 && result.ends_with("  ") {
			let mut spaces_to_remove = spaces;
			while spaces_to_remove >= 0 && result.ends_with(" ") {
				spaces_to_remove -= 1;
				result.remove(result.len() - 1);
			}
			result.push_str("\t");
		}
		spaces = spaces + 1 & 7
	}

	while result.len() > 0 && (result.ends_with(" ") || result.ends_with("\t")) {
		result.remove(result.len() - 1);
	}

	result.push_str("\n");
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_segment_address_to_string() {
		assert_eq!(segment_address_to_string(0xb33f, 0), "b33f                  ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolTypes::Unknown), "b33f ????             ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolTypes::Referenced), "b33f                  ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolTypes::StringResult), "b33f      str         ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolTypes::Set), "b33f              ( S)");
		assert_eq!(segment_address_to_string(0xb33f, SymbolTypes::Macro), "b33f          eqm     ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolTypes::MasterReference), "b33f              (R )");
		assert_eq!(
			segment_address_to_string(0xb33f, SymbolTypes::MasterReference | SymbolTypes::Set),
			"b33f              (RS)"
		);
		assert_eq!(
			segment_address_to_string(
				0xb33f,
				SymbolTypes::Unknown
					| SymbolTypes::Referenced
					| SymbolTypes::StringResult
					| SymbolTypes::Set | SymbolTypes::Macro
					| SymbolTypes::MasterReference
			),
			"b33f ???? str eqm (RS)"
		);
	}

	#[test]
	fn test_format_line_tabs() {
		assert_eq!(format_line_tabs("a\n"), "a\n");
		assert_eq!(format_line_tabs("a"), "a\n");
		assert_eq!(format_line_tabs("ab"), "ab\n");
		assert_eq!(format_line_tabs("a b"), "a b\n");
		assert_eq!(format_line_tabs("a  b"), "a  b\n");
		assert_eq!(format_line_tabs("a   b"), "a   b\n");
		assert_eq!(format_line_tabs("a    b"), "a    b\n");
		assert_eq!(format_line_tabs("a     b"), "a     b\n");
		assert_eq!(format_line_tabs("a      b"), "a      b\n");
		assert_eq!(format_line_tabs("a       b"), "a\tb\n");
		assert_eq!(format_line_tabs("a        b"), "a\t b\n");
		assert_eq!(format_line_tabs("a         b"), "a\t  b\n");
		assert_eq!(format_line_tabs("a          b"), "a\t   b\n");
		assert_eq!(format_line_tabs("a           b"), "a\t    b\n");
		assert_eq!(format_line_tabs("a            b"), "a\t     b\n");
		assert_eq!(format_line_tabs("a             b"), "a\t      b\n");
		assert_eq!(format_line_tabs("a              b"), "a\t       b\n");
		assert_eq!(format_line_tabs("a               b"), "a\t\tb\n");
		assert_eq!(format_line_tabs("a                b"), "a\t\t b\n");
		assert_eq!(format_line_tabs("aa               b"), "aa\t\t b\n");
		assert_eq!(format_line_tabs("aa                b"), "aa\t\t  b\n");
		assert_eq!(format_line_tabs("aa                 b"), "aa\t\t   b\n");
		assert_eq!(
			format_line_tabs("aa               b                  c       d e             f        "),
			"aa\t\t b\t\t    c\t    d e \t    f\n"
		);
		assert_eq!(
			format_line_tabs("aa               b                  c       d e             f        \n"),
			"aa\t\t b\t\t    c\t    d e \t    f\n"
		);
	}
}
