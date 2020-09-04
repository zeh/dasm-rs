use crate::constants::{
    S_HASH_SIZE,
};
use crate::types::legacy::{
    _INCFILE,
    _MNE,
    _SYMBOL,
};
use crate::mnemonics::{
    operations,
};

/*
 *  GLOBALS.C
 */
#[no_mangle]
pub static mut SHash: [*mut _SYMBOL; S_HASH_SIZE] = [0 as *const _SYMBOL as *mut _SYMBOL; S_HASH_SIZE];
/*	symbol hash table   */
#[no_mangle]
pub static mut pIncfile: *mut _INCFILE = 0 as *const _INCFILE as *mut _INCFILE;
/*	include file stack  */
#[no_mangle]
pub static mut Av: [*mut i8; 256] = [0 as *const i8 as *mut i8; 256];
/*	up to 256 arguments */
#[no_mangle]
pub static mut Avbuf: [i8; 512] = [0; 512];

pub static mut mnemonics_operations: [_MNE; 38] = [
	_MNE {
		vect: operations::v_list,
		name: b"list\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_include,
		name: b"include\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_seg,
		name: b"seg\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_hex,
		name: b"hex\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_err,
		name: b"err\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_dc,
		name: b"dc\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_dc,
		name: b"byte\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_dc,
		name: b"word\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_dc,
		name: b"long\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_ds,
		name: b"ds\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_dc,
		name: b"dv\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_end,
		name: b"end\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_trace,
		name: b"trace\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_org,
		name: b"org\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_rorg,
		name: b"rorg\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_rend,
		name: b"rend\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_align,
		name: b"align\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_subroutine,
		name: b"subroutine\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_equ,
		name: b"equ\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_equ,
		name: b"=\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_eqm,
		name: b"eqm\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_set,
		name: b"set\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_setstr,
		name: b"setstr\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_macro,
		name: b"mac\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_endm,
		name: b"endm\x00" as *const u8 as *const i8,
		flags: 0x80,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_mexit,
		name: b"mexit\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_ifconst,
		name: b"ifconst\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_ifnconst,
		name: b"ifnconst\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_if,
		name: b"if\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_else,
		name: b"else\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_endif,
		name: b"endif\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_endif,
		name: b"eif\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_repeat,
		name: b"repeat\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_repend,
		name: b"repend\x00" as *const u8 as *const i8,
		flags: 0x4,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_echo,
		name: b"echo\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_processor,
		name: b"processor\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_incbin,
		name: b"incbin\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: operations::v_incdir,
		name: b"incdir\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: 0,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
];
