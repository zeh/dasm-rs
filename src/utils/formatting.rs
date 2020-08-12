use crate::types::flags::SymbolType;

/**
 * Formats a segment address and flags to a string, also called a "PC" ("Program Counter").
 * In original C code, "sftos()" in main.c
 */
pub fn segment_address_to_string(value: u64, flags: u8) -> String {
	let mut result = String::new();
	result.push_str(format!("{:04x} ", value).as_str());

	result.push_str(if flags & SymbolType::Unknown != 0 {
		"???? "
	} else {
		"     "
	});

	result.push_str(if flags & SymbolType::StringResult != 0 {
		"str "
	} else {
		"    "
	});

	result.push_str(if flags & SymbolType::Macro != 0 {
		"eqm "
	} else {
		"    "
	});

	result.push_str(if flags & (SymbolType::MasterReference | SymbolType::Set) != 0 {
		"("
	} else {
		" "
	});

	result.push_str(if flags & SymbolType::MasterReference != 0 {
		"R"
	} else {
		" "
	});

	result.push_str(if flags & SymbolType::Set != 0 {
		"S"
	} else {
		" "
	});

	result.push_str(if flags & (SymbolType::MasterReference | SymbolType::Set) != 0 {
		")"
	} else {
		" "
	});

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_segment_address_to_string() {
		assert_eq!(segment_address_to_string(0xb33f, 0), "b33f                  ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolType::Unknown), "b33f ????             ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolType::Referenced), "b33f                  ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolType::StringResult), "b33f      str         ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolType::Set), "b33f              ( S)");
		assert_eq!(segment_address_to_string(0xb33f, SymbolType::Macro), "b33f          eqm     ");
		assert_eq!(segment_address_to_string(0xb33f, SymbolType::MasterReference), "b33f              (R )");
		assert_eq!(
			segment_address_to_string(0xb33f, SymbolType::MasterReference | SymbolType::Set),
			"b33f              (RS)"
		);
		assert_eq!(
			segment_address_to_string(
				0xb33f,
				SymbolType::Unknown
					| SymbolType::Referenced
					| SymbolType::StringResult
					| SymbolType::Set | SymbolType::Macro
					| SymbolType::MasterReference
			),
			"b33f ???? str eqm (RS)"
		);
	}
}
