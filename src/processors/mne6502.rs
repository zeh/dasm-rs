use crate::types::enums::{
    AddressModes,
};

extern "C" {
    #[no_mangle]
    fn v_mnemonic(str: *mut i8, mne: *mut _MNE);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MNE {
    pub next: *mut _MNE,
    pub vect: Option<unsafe extern "C" fn(_: *mut i8, _: *mut _MNE)
                         -> ()>,
    pub name: *const i8,
    pub flags: u8,
    pub okmask: u64,
    pub opcode: [u32; 21],
}
/*
 *  MNE6502.C
 */
#[no_mangle]
pub static mut Mne6502: [_MNE; 76] =
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"adc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x69 as i32 as u32,
                           0x65 as i32 as u32,
                           0x75 as i32 as u32,
                           0x6d as i32 as u32,
                           0x7d as i32 as u32,
                           0x79 as i32 as u32,
                           0x61 as i32 as u32,
                           0x71 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"anc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32) as u64,
                      opcode:
                          [0xb as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"and\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x29 as i32 as u32,
                           0x25 as i32 as u32,
                           0x35 as i32 as u32,
                           0x2d as i32 as u32,
                           0x3d as i32 as u32,
                           0x39 as i32 as u32,
                           0x21 as i32 as u32,
                           0x31 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"ane\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32) as u64,
                      opcode:
                          [0x8b as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"arr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32) as u64,
                      opcode:
                          [0x6b as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"asl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0xa as i32 as u32,
                           0x6 as i32 as u32,
                           0x16 as i32 as u32,
                           0xe as i32 as u32,
                           0x1e as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"asr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32) as u64,
                      opcode:
                          [0x4b as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bcc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x90 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bcs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0xb0 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"beq\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0xf0 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bit\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x24 as i32 as u32,
                           0x2c as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bmi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x30 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bne\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0xd0 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bpl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x10 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"brk\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bvc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x50 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"bvs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x70 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"clc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x18 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"cld\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xd8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"cli\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x58 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"clv\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xb8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"cmp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0xc9 as i32 as u32,
                           0xc5 as i32 as u32,
                           0xd5 as i32 as u32,
                           0xcd as i32 as u32,
                           0xdd as i32 as u32,
                           0xd9 as i32 as u32,
                           0xc1 as i32 as u32,
                           0xd1 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"cpx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xe0 as i32 as u32,
                           0xe4 as i32 as u32,
                           0xec as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"cpy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc0 as i32 as u32,
                           0xc4 as i32 as u32,
                           0xcc as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"dcp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0xc7 as i32 as u32,
                           0xd7 as i32 as u32,
                           0xcf as i32 as u32,
                           0xdf as i32 as u32,
                           0xdb as i32 as u32,
                           0xc3 as i32 as u32,
                           0xd3 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"dec\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0xc6 as i32 as u32,
                           0xd6 as i32 as u32,
                           0xce as i32 as u32,
                           0xde as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"dex\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xca as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"dey\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x88 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"eor\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x49 as i32 as u32,
                           0x45 as i32 as u32,
                           0x55 as i32 as u32,
                           0x4d as i32 as u32,
                           0x5d as i32 as u32,
                           0x59 as i32 as u32,
                           0x41 as i32 as u32,
                           0x51 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"inc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0xe6 as i32 as u32,
                           0xf6 as i32 as u32,
                           0xee as i32 as u32,
                           0xfe as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"inx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xe8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"iny\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xc8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"isb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0xe7 as i32 as u32,
                           0xf7 as i32 as u32,
                           0xef as i32 as u32,
                           0xff as i32 as u32,
                           0xfb as i32 as u32,
                           0xe3 as i32 as u32,
                           0xf3 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"jmp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::IndWord as i32) as u64,
                      opcode:
                          [0x4c as i32 as u32,
                           0x6c as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"jsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x20 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"las\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdrY as i32) as u64,
                      opcode:
                          [0xbb as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"lax\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0xa7 as i32 as u32,
                           0xb7 as i32 as u32,
                           0xaf as i32 as u32,
                           0xbf as i32 as u32,
                           0xa3 as i32 as u32,
                           0xb3 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"lda\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0xa9 as i32 as u32,
                           0xa5 as i32 as u32,
                           0xb5 as i32 as u32,
                           0xad as i32 as u32,
                           0xbd as i32 as u32,
                           0xb9 as i32 as u32,
                           0xa1 as i32 as u32,
                           0xb1 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32) as u64,
                      opcode:
                          [0xa2 as i32 as u32,
                           0xa6 as i32 as u32,
                           0xb6 as i32 as u32,
                           0xae as i32 as u32,
                           0xbe as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0xa0 as i32 as u32,
                           0xa4 as i32 as u32,
                           0xb4 as i32 as u32,
                           0xac as i32 as u32,
                           0xbc as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0x4a as i32 as u32,
                           0x46 as i32 as u32,
                           0x56 as i32 as u32,
                           0x4e as i32 as u32,
                           0x5e as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"lxa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32) as u64,
                      opcode:
                          [0xab as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"nop\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0xea as i32 as u32,
                           0x80 as i32 as u32,
                           0x4 as i32 as u32,
                           0x14 as i32 as u32,
                           0xc as i32 as u32,
                           0x1c as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"ora\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x9 as i32 as u32,
                           0x5 as i32 as u32,
                           0x15 as i32 as u32,
                           0xd as i32 as u32,
                           0x1d as i32 as u32,
                           0x19 as i32 as u32,
                           0x1 as i32 as u32,
                           0x11 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"pha\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x48 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"php\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"pla\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x68 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"plp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x28 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rla\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x27 as i32 as u32,
                           0x37 as i32 as u32,
                           0x2f as i32 as u32,
                           0x3f as i32 as u32,
                           0x3b as i32 as u32,
                           0x23 as i32 as u32,
                           0x33 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rol\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0x2a as i32 as u32,
                           0x26 as i32 as u32,
                           0x36 as i32 as u32,
                           0x2e as i32 as u32,
                           0x3e as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"ror\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0x6a as i32 as u32,
                           0x66 as i32 as u32,
                           0x76 as i32 as u32,
                           0x6e as i32 as u32,
                           0x7e as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x67 as i32 as u32,
                           0x77 as i32 as u32,
                           0x6f as i32 as u32,
                           0x7f as i32 as u32,
                           0x7b as i32 as u32,
                           0x63 as i32 as u32,
                           0x73 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rti\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x40 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rts\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x60 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sax\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32) as u64,
                      opcode:
                          [0x87 as i32 as u32,
                           0x97 as i32 as u32,
                           0x8f as i32 as u32,
                           0x83 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sbc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0xe9 as i32 as u32,
                           0xe5 as i32 as u32,
                           0xf5 as i32 as u32,
                           0xed as i32 as u32,
                           0xfd as i32 as u32,
                           0xf9 as i32 as u32,
                           0xe1 as i32 as u32,
                           0xf1 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sbx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32) as u64,
                      opcode:
                          [0xcb as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sec\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x38 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sed\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xf8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sei\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x78 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sha\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x9f as i32 as u32,
                           0x93 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"shs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdrY as i32) as u64,
                      opcode:
                          [0x9b as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"shx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdrY as i32) as u64,
                      opcode:
                          [0x9e as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"shy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::WordAdrX as i32) as u64,
                      opcode:
                          [0x9c as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"slo\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x7 as i32 as u32,
                           0x17 as i32 as u32,
                           0xf as i32 as u32,
                           0x1f as i32 as u32,
                           0x1b as i32 as u32,
                           0x3 as i32 as u32,
                           0x13 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sre\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x47 as i32 as u32,
                           0x57 as i32 as u32,
                           0x4f as i32 as u32,
                           0x5f as i32 as u32,
                           0x5b as i32 as u32,
                           0x43 as i32 as u32,
                           0x53 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sta\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdrY as i32 |
                               (1) <<
                                   AddressModes::IndByteX as i32 |
                               (1) <<
                                   AddressModes::IndByteY as i32) as u64,
                      opcode:
                          [0x85 as i32 as u32,
                           0x95 as i32 as u32,
                           0x8d as i32 as u32,
                           0x9d as i32 as u32,
                           0x99 as i32 as u32,
                           0x81 as i32 as u32,
                           0x91 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"stx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x86 as i32 as u32,
                           0x96 as i32 as u32,
                           0x8e as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"sty\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x84 as i32 as u32,
                           0x94 as i32 as u32,
                           0x8c as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"tax\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xaa as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"tay\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xa8 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"tsx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xba as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"txa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x8a as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"txs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x9a as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"tya\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x98 as i32 as u32, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect: None,
                      name: 0 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         }]
;
