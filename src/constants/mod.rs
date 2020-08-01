// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

pub const ALLOC_SIZE: u16 = 16384;   // In original C code, "ALLOCSIZE"
pub const DEF_ORG_FILL: u8 = 255;    // In original C code, "DEFORGFILL"
pub const M_HASH_AND: u16 = 0x03ff;  // In original C code, "MHASHAND"
pub const M_HASH_SIZE: usize = 1024; // In original C code, "MHASHSIZE"
pub const MAX_LINES: usize = 1024;   // In original C code, "MAXLINE"
pub const MAX_MACRO_LEVEL: u8 = 32;  // In original C code, "MAXMACLEVEL"
pub const MAX_SYMBOLS: usize = 1024; // In original C code, "MAX_SYM_LEN"
pub const S_HASH_AND: u16 = 0x03ff;  // In original C code, "SHASHAND"
pub const S_HASH_SIZE: usize = 1024; // In original C code, "SHASHSIZE"

// FIXME: expand this or use a proper char table
pub const CHAR_TAB: i32 = 9;         // In original C code, "TAB"
