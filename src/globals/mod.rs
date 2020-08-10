use crate::expressions;
use crate::types::enums::{
    AddressModes,
    BitOrder,
	Format,
    ErrorFormat,
    ListMode,
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
        sortMode: SortMode::Alpha,
        strictMode: false,
        symbolsFile: String::new(),
        verbosity: Verbosity::None,
    },

    other: OtherMainState {
        incLevel: 0,
        stopAtEnd: false,
    },

    execution: ExecutionState {
        bitOrder: BitOrder::MostLeast,
        isClear: false,
        listMode: ListMode::List,
        modeNext: AddressModes::Imp,
        redoEval: 0,
        redoIf: 0,
        redoIndex: 0,
        redoWhy: 0,
        trace: false,
    },

    expressions: ExpressionsState {
        argFlags: [0; expressions::MAX_ARGS],
        argIndex: 0,
        argIndexBase: 0,
	    argStack: [0; expressions::MAX_ARGS],
        lastWasOp: false,
        opIndex: 0,
        opIndexBase: 0,
        opPri: [0; expressions::MAX_OPS],
    },

    output: OutputState {
        listFile: None,
    },
};
