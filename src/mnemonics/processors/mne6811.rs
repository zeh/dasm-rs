use crate::mnemonics;
use crate::types::enums::{
    AddressModes,
};
use crate::types::legacy::{
    _MNE,
};

pub static mut mnemonics_68HC11: [_MNE; 145] = [
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"aba\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"abx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"aby\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x183a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"adca\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x89, 0x99, 0xa9, 0x18a9, 0xb9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"adcb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc9, 0xd9, 0xe9, 0x18e9, 0xf9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"adda\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x8b, 0x9b, 0xab, 0x18ab, 0xbb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"addb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xcb, 0xdb, 0xeb, 0x18eb, 0xfb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"addd\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc3, 0xd3, 0xe3, 0x18e3, 0xf3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"anda\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x84, 0x94, 0xa4, 0x18a4, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"andb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc4, 0xd4, 0xe4, 0x18e4, 0xf4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"asla\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"aslb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"asl\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x68, 0x1868, 0x78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"asld\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"asra\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"asrb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"asr\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x67, 0x1867, 0x77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bcc\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bclr\x00" as *const u8 as *const i8,
		flags: 0x10,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
		opcode: [0x15, 0x1d, 0x181d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bcs\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"beq\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bge\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x2c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bgt\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x2e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bhi\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bhs\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bita\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x85, 0x95, 0xa5, 0x18a5, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bitb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc5, 0xd5, 0xe5, 0x18e5, 0xf5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ble\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x2f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"blo\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bls\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"blt\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x2d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bmi\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x2b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bne\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bpl\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x2a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bra\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"brclr\x00" as *const u8 as *const i8,
		flags:
                          (0x10 as i32 | 0x20 as i32) as u8,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
		opcode: [0x13, 0x1f, 0x181f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"brn\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"brset\x00" as *const u8 as *const i8,
		flags:
                          (0x10 as i32 | 0x20 as i32) as u8,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
		opcode: [0x12, 0x1e, 0x181e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bset\x00" as *const u8 as *const i8,
		flags: 0x10,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
		opcode: [0x14, 0x1c, 0x181c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bsr\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x8d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bvc\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"bvs\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Rel as i32) as u64,
		opcode: [0x29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cba\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"clc\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cli\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"clra\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x4f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"clrb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x5f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"clr\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x6f, 0x186f, 0x7f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"clv\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cmpa\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x81, 0x91, 0xa1, 0x18a1, 0xb1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cmpb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc1, 0xd1, 0xe1, 0x18e1, 0xf1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"coma\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"comb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"com\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x63, 0x1863, 0x73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cpd\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x1a83, 0x1a93, 0x1aa3, 0xcda3, 0x1ab3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cpx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x8c, 0x9c, 0xac, 0xcdac, 0xbc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"cpy\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x188c, 0x189c, 0x1aac, 0x18ac, 0x18bc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"daa\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"deca\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x4a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"decb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x5a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"dec\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x6a, 0x186a, 0x7a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"des\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"dex\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"dey\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1809, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"eora\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x88, 0x98, 0xa8, 0x18a8, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"eorb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc8, 0xd8, 0xe8, 0x18e8, 0xf8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"fdiv\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"idiv\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"inca\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x4c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"incb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x5c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"inc\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x6c, 0x186c, 0x7c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ins\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"inx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"iny\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1808, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"jmp\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x6e, 0x186e, 0x7e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"jsr\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x9d, 0xad, 0x18ad, 0xbd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ldaa\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x86, 0x96, 0xa6, 0x18a6, 0xb6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ldab\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc6, 0xd6, 0xe6, 0x18e6, 0xf6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ldd\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xcc, 0xdc, 0xec, 0x18ec, 0xfc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lds\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x8e, 0x9e, 0xae, 0x18ae, 0xbe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ldx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xce, 0xde, 0xee, 0xcdee, 0xfe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ldy\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x18ce, 0x18de, 0x1aee, 0x18ee, 0x18fe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsla\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lslb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsl\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x68, 0x1868, 0x78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsld\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsra\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsrb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsr\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x64, 0x1864, 0x74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"lsrd\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"mul\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"nega\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"negb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"neg\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x60, 0x1860, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"nop\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"oraa\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x8a, 0x9a, 0xaa, 0x18aa, 0xba, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"orab\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xca, 0xda, 0xea, 0x18ea, 0xfa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"psha\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"pshb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"pshx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"pshy\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x183c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"pula\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"pulb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"pulx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"puly\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1838, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rola\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rolb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rol\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x69, 0x1869, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rora\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rorb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"ror\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x66, 0x1866, 0x76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rti\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"rts\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sba\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sbca\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x82, 0x92, 0xa2, 0x18a2, 0xb2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sbcb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc2, 0xd2, 0xe2, 0x18e2, 0xf2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sec\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sei\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sev\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"staa\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x97, 0xa7, 0x18a7, 0xb7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"stab\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0xd7, 0xe7, 0x18e7, 0xf7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"std\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0xdd, 0xed, 0x18ed, 0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"stop\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0xcf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sts\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x9f, 0xaf, 0x18af, 0xbf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"stx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0xdf, 0xef, 0xcdef, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"sty\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdr as i32 | (1) <<
                                   AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x18df, 0x1aef, 0x18ef, 0x18ff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"suba\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x80, 0x90, 0xa0, 0x18a0, 0xb0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"subb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm8 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0xc0, 0xd0, 0xe0, 0x18e0, 0xf0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"subd\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imm16 as i32 | ((1) <<
                                    AddressModes::ByteAdr as i32 | (1) <<
                                        AddressModes::ByteAdrX as i32 | (1) <<
                                        AddressModes::ByteAdrY as i32 | (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
		opcode: [0x83, 0x93, 0xa3, 0x18a3, 0xb3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"swi\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tab\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tap\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tba\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"test\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tpa\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tsta\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x4d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tstb\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x5d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tst\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::ByteAdrX as i32 | (1) <<
                                   AddressModes::ByteAdrY as i32 | (1) <<
                                   AddressModes::WordAdr as i32) as u64,
		opcode: [0x6d, 0x186d, 0x7d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tsx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tsy\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1830, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"txs\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"tys\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x1835, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"wai\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x3e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"xgdx\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x8f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
	_MNE {
		vect: mnemonics::operations::v_mnemonic,
		name: b"xgdy\x00" as *const u8 as *const i8,
		flags: 0,
		okmask: ((1) << AddressModes::Imp as i32) as u64,
		opcode: [0x188f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	},
];
