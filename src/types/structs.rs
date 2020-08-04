// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use crate::types::enums::{
	AsmErrorEquates,
	Format,
	ErrorFormat,
	SortMode,
	Verbosity,
};

// In original C code, both "ERRORSTRUCT" and "ERROR_DEFINITION"
#[derive(Clone)]
pub struct ErrorDefinition {
	pub errorType: AsmErrorEquates,
	pub fatal: bool, // If true, cannot continue compilation
	pub description: &'static str,
}

pub struct GlobalState {
	pub debug: bool,
	pub errorFormat: ErrorFormat,
	pub format: Format,
	pub listAllPasses: bool,
	pub sortMode: SortMode,
	pub strictMode: bool,
	pub trace: bool,
	pub verbosity: Verbosity,
}
