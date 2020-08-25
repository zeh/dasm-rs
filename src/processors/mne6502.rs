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
                          [0x69,
                           0x65,
                           0x75,
                           0x6d,
                           0x7d,
                           0x79,
                           0x61,
                           0x71, 0, 0, 0, 0, 0,
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
                          [0xb, 0, 0, 0, 0, 0,
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
                          [0x29,
                           0x25,
                           0x35,
                           0x2d,
                           0x3d,
                           0x39,
                           0x21,
                           0x31, 0, 0, 0, 0, 0,
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
                          [0x8b, 0, 0, 0, 0, 0,
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
                          [0x6b, 0, 0, 0, 0, 0,
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
                          [0xa,
                           0x6,
                           0x16,
                           0xe,
                           0x1e, 0, 0, 0, 0, 0,
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
                          [0x4b, 0, 0, 0, 0, 0,
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
                          [0x90, 0, 0, 0, 0, 0,
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
                          [0xb0, 0, 0, 0, 0, 0,
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
                          [0xf0, 0, 0, 0, 0, 0,
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
                          [0x24,
                           0x2c, 0, 0, 0, 0, 0,
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
                          [0x30, 0, 0, 0, 0, 0,
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
                          [0xd0, 0, 0, 0, 0, 0,
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
                          [0x10, 0, 0, 0, 0, 0,
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
                          [0x50, 0, 0, 0, 0, 0,
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
                          [0x70, 0, 0, 0, 0, 0,
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
                          [0x18, 0, 0, 0, 0, 0,
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
                          [0xd8, 0, 0, 0, 0, 0,
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
                          [0x58, 0, 0, 0, 0, 0,
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
                          [0xb8, 0, 0, 0, 0, 0,
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
                          [0xc9,
                           0xc5,
                           0xd5,
                           0xcd,
                           0xdd,
                           0xd9,
                           0xc1,
                           0xd1, 0, 0, 0, 0, 0,
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
                          [0xe0,
                           0xe4,
                           0xec, 0, 0, 0, 0, 0,
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
                          [0xc0,
                           0xc4,
                           0xcc, 0, 0, 0, 0, 0,
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
                          [0xc7,
                           0xd7,
                           0xcf,
                           0xdf,
                           0xdb,
                           0xc3,
                           0xd3, 0, 0, 0, 0, 0,
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
                          [0xc6,
                           0xd6,
                           0xce,
                           0xde, 0, 0, 0, 0, 0,
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
                          [0xca, 0, 0, 0, 0, 0,
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
                          [0x88, 0, 0, 0, 0, 0,
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
                          [0x49,
                           0x45,
                           0x55,
                           0x4d,
                           0x5d,
                           0x59,
                           0x41,
                           0x51, 0, 0, 0, 0, 0,
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
                          [0xe6,
                           0xf6,
                           0xee,
                           0xfe, 0, 0, 0, 0, 0,
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
                          [0xe8, 0, 0, 0, 0, 0,
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
                          [0xc8, 0, 0, 0, 0, 0,
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
                          [0xe7,
                           0xf7,
                           0xef,
                           0xff,
                           0xfb,
                           0xe3,
                           0xf3, 0, 0, 0, 0, 0,
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
                          [0x4c,
                           0x6c, 0, 0, 0, 0, 0,
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
                          [0x20, 0, 0, 0, 0, 0,
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
                          [0xbb, 0, 0, 0, 0, 0,
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
                          [0xa7,
                           0xb7,
                           0xaf,
                           0xbf,
                           0xa3,
                           0xb3, 0, 0, 0, 0, 0,
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
                          [0xa9,
                           0xa5,
                           0xb5,
                           0xad,
                           0xbd,
                           0xb9,
                           0xa1,
                           0xb1, 0, 0, 0, 0, 0,
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
                          [0xa2,
                           0xa6,
                           0xb6,
                           0xae,
                           0xbe, 0, 0, 0, 0, 0,
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
                          [0xa0,
                           0xa4,
                           0xb4,
                           0xac,
                           0xbc, 0, 0, 0, 0, 0,
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
                          [0x4a,
                           0x46,
                           0x56,
                           0x4e,
                           0x5e, 0, 0, 0, 0, 0,
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
                          [0xab, 0, 0, 0, 0, 0,
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
                          [0xea,
                           0x80,
                           0x4,
                           0x14,
                           0xc,
                           0x1c, 0, 0, 0, 0, 0,
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
                          [0x9,
                           0x5,
                           0x15,
                           0xd,
                           0x1d,
                           0x19,
                           0x1,
                           0x11, 0, 0, 0, 0, 0,
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
                          [0x48, 0, 0, 0, 0, 0,
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
                          [0x8, 0, 0, 0, 0, 0,
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
                          [0x68, 0, 0, 0, 0, 0,
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
                          [0x28, 0, 0, 0, 0, 0,
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
                          [0x27,
                           0x37,
                           0x2f,
                           0x3f,
                           0x3b,
                           0x23,
                           0x33, 0, 0, 0, 0, 0,
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
                          [0x2a,
                           0x26,
                           0x36,
                           0x2e,
                           0x3e, 0, 0, 0, 0, 0,
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
                          [0x6a,
                           0x66,
                           0x76,
                           0x6e,
                           0x7e, 0, 0, 0, 0, 0,
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
                          [0x67,
                           0x77,
                           0x6f,
                           0x7f,
                           0x7b,
                           0x63,
                           0x73, 0, 0, 0, 0, 0,
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
                          [0x40, 0, 0, 0, 0, 0,
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
                          [0x60, 0, 0, 0, 0, 0,
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
                          [0x87,
                           0x97,
                           0x8f,
                           0x83, 0, 0, 0, 0, 0,
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
                          [0xe9,
                           0xe5,
                           0xf5,
                           0xed,
                           0xfd,
                           0xf9,
                           0xe1,
                           0xf1, 0, 0, 0, 0, 0,
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
                          [0xcb, 0, 0, 0, 0, 0,
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
                          [0x38, 0, 0, 0, 0, 0,
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
                          [0xf8, 0, 0, 0, 0, 0,
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
                          [0x78, 0, 0, 0, 0, 0,
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
                          [0x9f,
                           0x93, 0, 0, 0, 0, 0,
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
                          [0x9b, 0, 0, 0, 0, 0,
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
                          [0x9e, 0, 0, 0, 0, 0,
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
                          [0x9c, 0, 0, 0, 0, 0,
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
                          [0x7,
                           0x17,
                           0xf,
                           0x1f,
                           0x1b,
                           0x3,
                           0x13, 0, 0, 0, 0, 0,
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
                          [0x47,
                           0x57,
                           0x4f,
                           0x5f,
                           0x5b,
                           0x43,
                           0x53, 0, 0, 0, 0, 0,
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
                          [0x85,
                           0x95,
                           0x8d,
                           0x9d,
                           0x99,
                           0x81,
                           0x91, 0, 0, 0, 0, 0,
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
                          [0x86,
                           0x96,
                           0x8e, 0, 0, 0, 0, 0,
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
                          [0x84,
                           0x94,
                           0x8c, 0, 0, 0, 0, 0,
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
                          [0xaa, 0, 0, 0, 0, 0,
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
                          [0xa8, 0, 0, 0, 0, 0,
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
                          [0xba, 0, 0, 0, 0, 0,
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
                          [0x8a, 0, 0, 0, 0, 0,
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
                          [0x9a, 0, 0, 0, 0, 0,
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
                          [0x98, 0, 0, 0, 0, 0,
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
