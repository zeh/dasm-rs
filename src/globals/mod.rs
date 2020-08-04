use crate::types::enums::{
	Format,
	ErrorFormat,
    SortMode,
    Verbosity,
};
use crate::types::structs::{
    GlobalState,
};

// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

pub static mut state: GlobalState = GlobalState {
    debug: false,
    errorFormat: ErrorFormat::Woe, // Special static case, since we can't use ::default();
    format: Format::Default, // Special static case, since we can't use ::default();
    listAllPasses: false,
    sortMode: SortMode::Alpha, // Special static case, since we can't use ::default();
    strictMode: false,
    trace: false,
    verbosity: Verbosity::None, // Special static case, since we can't use ::default();
};
