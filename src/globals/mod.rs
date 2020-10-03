use crate::constants;
use crate::types::enums::{
	AddressModes,
	BitOrder,
	ListMode,
	Processors,
};
use crate::types::legacy::{
	_INCFILE,
	_MACRO,
	_MNE,
	_SYMBOL,
};
use crate::types::structs::{
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
		includeFiles: Vec::<*mut _INCFILE>::new(),
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
		// FIXME: temporary, move to a new Mnemonic struct when possible
		symbols: Vec::<*mut _SYMBOL>::new(),
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
