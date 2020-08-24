// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

use crate::formatting;
use crate::types::flags::{
    SymbolTypes,
};
use crate::types::structs::{
    GlobalState,
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
pub fn count_unresolved_symbols(symbols: &Vec<Symbol>) -> usize {
	symbols.iter().filter(|&symbol| is_symbol_unresolved(&symbol)).count()
}

/**
 * Create a list of all unresolved symbols.
 * In original C code, "ShowUnresolvedSymbols()" in main.c
 */
pub fn list_unresolved_symbols(symbols: &Vec<Symbol>) -> String {
	let mut result = String::new();

	let numUnresolved = count_unresolved_symbols(symbols);
	if numUnresolved != 0 {
		result.push_str("--- Unresolved Symbol List\n");

		for symbol in symbols {
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
pub fn clear_references(symbols: &mut Vec<Symbol>) {
	for symbol in symbols {
		symbol.flags &= !SymbolTypes::Referenced;
	}
}

/**
 * Sets the special symbol.
 * In original C code, "setspecial()" in symbols.c
 */
pub fn set_special_symbol(specialSymbol: &mut Symbol, value: u64, flags: u8) {
    specialSymbol.value = value;
    specialSymbol.flags = flags;
}

/**
 * Find a symbol.
 * In original C code, "findsymbol()" in symbols.c
 */
pub fn find_symbol(state: &mut GlobalState, name: &str) -> Option<&mut Symbol> {
    let mut usedName: String = String::from(name);
    if name.starts_with(".") {
        if name.len() == 1 {
            let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
            if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
                state.execution.orgSymbol.flags = currentSegment.rflags & SymbolTypes::Unknown;
                state.execution.orgSymbol.value = currentSegment.rorg;
            } else {
                state.execution.orgSymbol.flags = currentSegment.flags & SymbolTypes::Unknown;
                state.execution.orgSymbol.value = currentSegment.org;
            }
            return Some(&mut state.execution.orgSymbol);
        } else if name.len() == 2 && name == ".." {
            return Some(&mut state.execution.specialSymbol);
        } else if name.len() == 3 && name == "..." {
            state.execution.specialCheckSymbol.flags = 0;
            state.execution.specialCheckSymbol.value = state.output.checksum;
            return Some(&mut state.execution.specialCheckSymbol);
        }

        usedName = format!("{}{}", state.execution.localIndex, name);
    } else if name.ends_with("$") {
        usedName = format!("{}${}", state.execution.localDollarIndex, name);
    }

    state.execution.symbols.iter_mut().find(|symbol| symbol.name == usedName)
}
