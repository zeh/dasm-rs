// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use crate::types::enums::{
	AsmErrorEquates,
};

// In original C code, both "ERRORSTRUCT" and "ERROR_DEFINITION"
#[derive(Clone)]
pub struct ErrorDefinition {
	pub errorType: AsmErrorEquates,
	pub fatal: bool, // If true, cannot continue compilation
	pub description: &'static str,
}
