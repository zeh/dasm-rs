// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use std::fs::File;

use crate::constants;
use crate::expressions;
use crate::types::enums::{
	AddressModes,
	AsmErrorEquates,
	BitOrder,
	ErrorFormat,
	Format,
	ListMode,
	Processors,
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

	// Set by ::operations (and others) during execution
	pub execution: ExecutionState,

	// Set by ::expressions during execution
	pub expressions: ExpressionsState,

	// Written to
	pub output: OutputState,
}

pub struct ParametersState {
	pub debug: bool,
	pub errorFormat: ErrorFormat,
	pub format: Format,
	pub listAllPasses: bool,
	pub listFile: String,
	pub maxPasses: u8,
	pub sortMode: SortMode,
	pub strictMode: bool,
	pub symbolsFile: String,
	pub verbosity: Verbosity,
}

pub struct OtherMainState {
	pub currentSegment: usize,
	pub incLevel: i8,
	pub stopAtEnd: bool,
	pub segments: Vec<Segment>,
}

pub struct ExecutionState {
	pub bitOrder: BitOrder,
	pub extraString: String,
	pub isClear: bool,
	pub includeDirList: Vec<String>,
	pub listMode: ListMode,
	pub modeNext: AddressModes,
	pub pass: u8,
	pub processor: Processors,
	pub programOrg: u64,
	pub programFlags: u8,
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

pub struct OutputState {
	pub generated: [u8; constants::MAX_LINES],
	pub generatedLength: usize,
	pub listFile: Option<File>,
	pub orgFill: u8, // ? where?
}

// In original C code, "_SEGMENT" and used as a linked list
pub struct Segment {
	pub name: String,
	pub flags: u8,
	pub rflags: u8,
	pub org: u64,
	pub rorg: u64,
	pub initorg: u64,
	pub initrorg: u64,
	pub initflags: u8,
	pub initrflags: u8,
}
