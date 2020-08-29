use crate::constants;
use crate::expressions::operations::{
	ExpressionOperationFunc
};
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
use crate::types::structs::{
	ExecutionState,
	ExpressionsState,
	GlobalState,
	OtherMainState,
	OutputState,
	ParametersState,
	Segment,
	StackIf,
	StackRepeat,
};

// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

// Enums use explicit options since we can't use ::default()
pub static mut state: GlobalState = GlobalState {
	parameters: ParametersState {
		debug: false,
		errorFormat: ErrorFormat::Woe,
		format: Format::Default,
		listAllPasses: false,
		listFile: String::new(),
		maxPasses: 10,
		outFile: String::new(),
		sortMode: SortMode::Alpha,
		strictMode: false,
		symbolsFile: String::new(),
		verbosity: Verbosity::None,
	},

	other: OtherMainState {
		currentSegment: 0,
		incLevel: 0,
		stopAtEnd: false,
		segments: Vec::<Segment>::new(),
	},

	execution: ExecutionState {
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
		argFlags: Vec::<u8>::new(),
		argIndexBase: 0,
		argStack: Vec::<i64>::new(),
		argString: Vec::<String>::new(),
		lastWasOp: false,
		opFunc: Vec::<ExpressionOperationFunc>::new(),
		opIndexBase: 0,
		opPri: Vec::<usize>::new(),
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
