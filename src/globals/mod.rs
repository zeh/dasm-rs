use crate::constants;
use crate::types::enums::{
	AddressModes,
	BitOrder,
	ErrorFormat,
	Format,
	ListMode,
	Processors,
	SortMode,
	Verbosity,
};
use crate::types::legacy::{
	_MACRO,
	_MNE,
};
use crate::types::structs::{
	CommandLineOptions,
	ExecutionState,
	ExpressionsState,
	ExpressionStackArgument,
	ExpressionStackOperation,
	GlobalState,
	OtherMainState,
	OutputState,
	Segment,
	StackIf,
	StackRepeat,
};

// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

// Enums use explicit options since we can't use ::default()
pub static mut state: GlobalState = GlobalState {
	// FIXME: this is a duplicate of CommandLineOptions::new()
	parameters: CommandLineOptions {
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
	},

	other: OtherMainState {
		currentSegment: 0,
		incLevel: 0,
		stopAtEnd: false,
		segments: Vec::<Segment>::new(),
	},

	execution: ExecutionState {
		argumentStack: Vec::<String>::new(),
		bitOrder: BitOrder::MostLeast,
		extraString: String::new(),
		ifs: Vec::<StackIf>::new(),
		includeDirList: Vec::<String>::new(),
		isClear: false,
		lastLocalDollarIndex: 0,
		lastLocalIndex: 0,
		listMode: ListMode::List,
		localDollarIndex: 0,
		localIndex: 0,
		macroLevel: 0,
		// FIXME: temporary, move to a new Macro struct when possible
		macros: Vec::<*mut _MACRO>::new(),
		// FIXME: temporary, move to a new Mnemonic struct when possible
		mnemonics: Vec::<*mut _MNE>::new(),
		modeNext: AddressModes::Imp,
		pass: 0,
		processor: Processors::None,
		programFlags: 0,
		programOrg: 0,
		redoEval: 0,
		redoIf: 0,
		redoIndex: 0,
		redoWhy: 0,
		repeats: Vec::<StackRepeat>::new(),
		trace: false,
	},

	expressions: ExpressionsState {
		argument_len_base: 0,
		arguments: Vec::<ExpressionStackArgument>::new(),
		last_was_operation: false,
		operation_len_base: 0,
		operations: Vec::<ExpressionStackOperation>::new(),
	},

	output: OutputState {
		checksum: 0,
		generated: [0; constants::MAX_LINES],
		generatedLength: 0,
		listFile: None,
		orgFill: constants::DEF_ORG_FILL,
		outFile: None,
		passBufferErrors: Vec::<String>::new(),
		passBufferMessages: Vec::<String>::new(),
		seekBack: 0,
		segmentLength: 0,
	},
};
