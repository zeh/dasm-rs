// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use std::fs::File;

use crate::constants;
use crate::expressions::operations::ExpressionOperationFunc;
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
use crate::types::legacy::{
	_INCFILE,
	_MACRO,
	_MNE,
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
	pub parameters: CommandLineOptions,

	// Set by main, misc stuff
	pub other: OtherMainState,

	// Set by ::operations (and others) during execution
	pub execution: ExecutionState,

	// Set by ::expressions during execution
	pub expressions: ExpressionsState,

	// Written to
	pub output: OutputState,
}

/**
 * All options, as parsed from the command line or as default otherwise.
 */
pub struct CommandLineOptions {
	pub debug_extended: bool,
	pub debug: bool,
	pub do_all_passes: bool,
	pub error_format: ErrorFormat,
	pub format: Format,
	pub include_dirs: Vec<String>,
	pub input: String,
	pub list_all_passes: bool,
	pub list_file: String,
	pub max_passes: u8,
	pub out_file: String,
	pub parsing_failed: bool,
	pub sort_mode: SortMode,
	pub strict_mode: bool,
	pub symbols_eqm: Vec<(String, String)>,
	pub symbols_file: String,
	pub symbols_set: Vec<(String, String)>,
	pub verbosity: Verbosity,
}

impl CommandLineOptions {
	pub fn new() -> Self {
		return Self {
			debug_extended: false,
			debug: false,
			do_all_passes: false,
			error_format: ErrorFormat::Woe,
			format: Format::Default,
			include_dirs: Vec::<String>::new(),
			input: String::new(),
			list_all_passes: false,
			list_file: String::new(),
			max_passes: 10,
			out_file: String::new(),
			parsing_failed: false,
			sort_mode: SortMode::Alpha,
			strict_mode: false,
			symbols_eqm: Vec::<(String, String)>::new(),
			symbols_file: String::new(),
			symbols_set: Vec::<(String, String)>::new(),
			verbosity: Verbosity::None,
		};
	}
}

pub struct OtherMainState {
	pub currentSegment: usize,
	pub incLevel: i8,
	pub stopAtEnd: bool,
	pub segments: Vec<Segment>,
}

pub struct ExecutionState {
	pub argumentStack: Vec<String>,
	pub bitOrder: BitOrder,
	pub extraString: String,
	pub ifs: Vec<StackIf>, // IF/ELSE/ENDIF stack
	pub includeDirList: Vec<String>,
	pub isClear: bool,
	pub lastLocalDollarIndex: u64,
	pub lastLocalIndex: u64,
	pub listMode: ListMode,
	pub localDollarIndex: u64,
	pub localIndex: u64, // To generate local variables
	pub macroLevel: u8,
	// FIXME: temporary, move to a new Macro struct when possible
	pub macros: Vec<*mut _MACRO>,
	// FIXME: temporary, move to a new Mnemonic struct when possible
	pub mnemonics: Vec<*mut _MNE>,
	pub modeNext: AddressModes,
	pub pass: u8,
	pub processor: Processors,
	pub programFlags: u8,
	pub programOrg: u64,
	pub redoEval: i32,
	pub redoIf: u64,
	pub redoIndex: i32,
	pub redoWhy: u64,
	pub repeats: Vec<StackRepeat>, // Repeat loop stack
	pub trace: bool,
}

pub struct ExpressionsState {
	pub argument_len_base: usize,
	pub arguments: Vec<ExpressionStackArgument>,
	pub last_was_operation: bool,
	pub operation_len_base: usize,
	pub operations: Vec<ExpressionStackOperation>,
}

pub struct OutputState {
	pub checksum: u64,
	pub generated: [u8; constants::MAX_LINES],
	pub generatedLength: usize,
	pub listFile: Option<File>,
	pub orgFill: u8, // ? where?
	pub outFile: Option<File>,
	pub passBufferErrors: Vec<String>,
	pub passBufferMessages: Vec<String>, // Buffers to supress errors and messages until last pass
	pub seekBack: i64,
	pub segmentLength: usize,
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

// In original C code, "_IFSTACK" and used as a linked list
pub struct StackIf {
	// FIXME: temporary, move to a new IncludeFile struct when possible
	pub file: *mut _INCFILE,
	pub flags: u8, // IfFlags
	pub result: bool,
	pub result_acc: bool,
}

// In original C code, "_REPLOOP" and used as a linked list
pub struct StackRepeat {
	pub count: u64,
	// FIXME: temporary, this seems to be a strlst or a file string pointer of some kind
	pub seek: u64,
	pub line_number: u64,
	// FIXME: temporary, move to a new IncludeFile struct when possible
	pub file: *mut _INCFILE,
	pub flags: u8,
}

pub struct ExpressionStackOperation {
	pub func: Option<ExpressionOperationFunc>,
	pub pri: usize,
}

pub struct ExpressionStackArgument {
	pub flags: u8,
	pub value: i64,
	pub string: Option<String>,
}
