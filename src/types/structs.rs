// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use crate::expressions;
use crate::types::enums::{
	AsmErrorEquates,
	BitOrder,
	Format,
	ErrorFormat,
	ListMode,
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
	// Set by main, coming from command line switches
	pub parameters: ParametersState,

	// Set by main, misc stuff
	pub other: OtherMainState,

	// Set by ::operations during execution
	pub execution: ExecutionState,

	// Set by ::expressions during execution
	pub expressions: ExpressionsState,
}

pub struct ParametersState {
	pub debug: bool,
	pub errorFormat: ErrorFormat,
	pub format: Format,
	pub listAllPasses: bool,
	pub maxPasses: u16,
	pub sortMode: SortMode,
	pub strictMode: bool,
	pub verbosity: Verbosity,
}

pub struct OtherMainState {
	pub incLevel: i8,
	pub stopAtEnd: bool,
}

pub struct ExecutionState {
	pub bitOrder: BitOrder,
	pub isClear: bool,
	pub listMode: ListMode,
	pub redoEval: i32,
	pub redoIf: u64,
	pub redoIndex: i32,
	pub redoWhy: u64,
	pub trace: bool,
}

pub struct ExpressionsState {
	pub argFlags: [u8; expressions::MAX_ARGS],
	pub argIndex: usize,
	pub argIndexBase: usize,
	pub argStack: [i64; expressions::MAX_ARGS],
	pub lastWasOp: bool,
	pub opIndex: usize,
	pub opIndexBase: usize,
	pub opPri: [usize; expressions::MAX_OPS],
}
