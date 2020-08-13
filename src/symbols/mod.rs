// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

use crate::formatting;
use crate::globals::state;
use crate::types::flags::{
	SymbolTypes,
};
use crate::types::structs::{
	Symbol,
};

/**
 * Returns true when testing if a symbol is considered unresolved.
 */
pub fn is_symbol_unresolved(symbol: &Symbol) -> bool {
	symbol.flags & SymbolTypes::Unknown != 0
}

/**
 * Pre-count all unresolved symbols.
 * In original C code, "CountUnresolvedSymbols()" in main.c
 */
pub fn count_unresolved_symbols() -> usize {
	let st = &mut state.lock().unwrap();

	st.execution.symbols.iter().filter(|&symbol| is_symbol_unresolved(&symbol)).count()
}

/**
 * Create a list of all unresolved symbols.
 * In original C code, "ShowUnresolvedSymbols()" in main.c
 */
pub fn list_unresolved_symbols() -> String {
	let st = &mut state.lock().unwrap();
	let mut result = String::new();

	let numUnresolved = count_unresolved_symbols();
	if numUnresolved != 0 {
		result.push_str("--- Unresolved Symbol List\n");

		for symbol in &mut st.execution.symbols {
			if is_symbol_unresolved(&symbol) {
				println!(
					"{:24} {}",
					symbol.name,
					// FIXME: maybe this is segment_value_to_string instead?
					formatting::segment_address_to_string(symbol.value as u64, symbol.flags),
				);
			}
		}

		result.push_str(
			format!(
				"--- {} Unresolved Symbol{}\n",
				numUnresolved,
				if numUnresolved == 1 {
					" "
				} else {
					"s"
				}
			)
			.as_str(),
		);
	}

	result
}

/**
 * Create a list of all unresolved symbols.
 * In original C code, "clearrefs()" in main.c
 */
pub fn clear_references() {
	let st = &mut state.lock().unwrap();

	for symbol in &mut st.execution.symbols {
		symbol.flags &= !SymbolTypes::Referenced;
	}
}
