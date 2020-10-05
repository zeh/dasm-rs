// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;
pub mod operations;
pub mod parsing;

pub const MAX_OPS: usize = 32;  // In original C code, "MAXOPS"
pub const MAX_ARGS: usize = 64; // In original C code, "MAXARGS"
