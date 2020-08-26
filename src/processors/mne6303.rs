use crate::types::enums::{
    AddressModes,
};
use crate::types::legacy::{
    _MNE,
};

extern "C" {
    #[no_mangle]
    fn v_mnemonic(str: *mut i8, mne: *mut _MNE);
}
/*
 *  MNE6303.C
 */
/*
 *  IMP IMM8 IMM16 BYTE BYTEX BYTEY WORD WORDX WORDY REL (,x) (),y (WORD)
 *   0	 1    2     3	 4     5     6    7     8     9   10   11    12
 *
 *  0,x 0,y BIT BITBRA
 *   13  14  15   16
 */
#[no_mangle]
pub static mut Mne6803: [_MNE; 125] =
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"aba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1b, 0, 0, 0, 0, 0,
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
                      name: b"abx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3a, 0, 0, 0, 0, 0,
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
                      name: b"adca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x89,
                           0x99,
                           0xa9,
                           0xb9, 0, 0, 0, 0, 0,
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
                      name: b"adcb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc9,
                           0xd9,
                           0xe9,
                           0xf9, 0, 0, 0, 0, 0,
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
                      name: b"adda\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x8b,
                           0x9b,
                           0xab,
                           0xbb, 0, 0, 0, 0, 0,
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
                      name: b"addb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xcb,
                           0xdb,
                           0xeb,
                           0xfb, 0, 0, 0, 0, 0,
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
                      name: b"addd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc3,
                           0xd3,
                           0xe3,
                           0xf3, 0, 0, 0, 0, 0,
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
                      name: b"anda\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x84,
                           0x94,
                           0xa4,
                           0xb4, 0, 0, 0, 0, 0,
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
                      name: b"andb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc4,
                           0xd4,
                           0xe4,
                           0xf4, 0, 0, 0, 0, 0,
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
                      name: b"bita\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x85,
                           0x95,
                           0xa5,
                           0xb5, 0, 0, 0, 0, 0,
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
                      name: b"bitb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc5,
                           0xd5,
                           0xe5,
                           0xf5, 0, 0, 0, 0, 0,
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
                      name: b"bra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
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
                      name: b"brn\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x21, 0, 0, 0, 0, 0,
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
                          [0x24, 0, 0, 0, 0, 0,
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
                      name: b"bhs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x24, 0, 0, 0, 0, 0,
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
                          [0x25, 0, 0, 0, 0, 0,
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
                      name: b"blo\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x25, 0, 0, 0, 0, 0,
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
                          [0x27, 0, 0, 0, 0, 0,
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
                      name: b"bge\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x2c, 0, 0, 0, 0, 0,
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
                      name: b"bgt\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x2e, 0, 0, 0, 0, 0,
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
                      name: b"bhi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x22, 0, 0, 0, 0, 0,
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
                      name: b"ble\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x2f, 0, 0, 0, 0, 0,
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
                      name: b"bls\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x23, 0, 0, 0, 0, 0,
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
                      name: b"blt\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x2d, 0, 0, 0, 0, 0,
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
                      name: b"bmi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x2b, 0, 0, 0, 0, 0,
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
                          [0x26, 0, 0, 0, 0, 0,
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
                      name: b"bvc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
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
                      name: b"bvs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x29, 0, 0, 0, 0, 0,
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
                          [0x2a, 0, 0, 0, 0, 0,
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
                      name: b"bsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x8d, 0, 0, 0, 0, 0,
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
                          [0xc, 0, 0, 0, 0, 0,
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
                          [0xe, 0, 0, 0, 0, 0,
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
                          [0xa, 0, 0, 0, 0, 0,
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
                          [0xd, 0, 0, 0, 0, 0,
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
                          [0xf, 0, 0, 0, 0, 0,
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
                      name: b"sev\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"tap\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x6, 0, 0, 0, 0, 0,
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
                      name: b"tpa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x7, 0, 0, 0, 0, 0,
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
                      name: b"clr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6f,
                           0x7f, 0, 0, 0, 0, 0,
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
                      name: b"clra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4f, 0, 0, 0, 0, 0,
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
                      name: b"clrb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5f, 0, 0, 0, 0, 0,
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
                      name: b"cmpa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x81,
                           0x91,
                           0xa1,
                           0xb1, 0, 0, 0, 0, 0,
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
                      name: b"cmpb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc1,
                           0xd1,
                           0xe1,
                           0xf1, 0, 0, 0, 0, 0,
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
                      name: b"cba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x11, 0, 0, 0, 0, 0,
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
                      name: b"com\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x63,
                           0x73, 0, 0, 0, 0, 0,
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
                      name: b"coma\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x43, 0, 0, 0, 0, 0,
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
                      name: b"comb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x53, 0, 0, 0, 0, 0,
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
                      name: b"neg\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x60,
                           0x70, 0, 0, 0, 0, 0,
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
                      name: b"nega\x00" as *const u8 as *const i8,
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
                      name: b"negb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"daa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x19, 0, 0, 0, 0, 0,
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
                      name: b"dec\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6a,
                           0x7a, 0, 0, 0, 0, 0,
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
                      name: b"deca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4a, 0, 0, 0, 0, 0,
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
                      name: b"decb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5a, 0, 0, 0, 0, 0,
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
                      name: b"eora\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x88,
                           0x98,
                           0xa8,
                           0xb8, 0, 0, 0, 0, 0,
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
                      name: b"eorb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc8,
                           0xd8,
                           0xe8,
                           0xf8, 0, 0, 0, 0, 0,
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
                      name: b"inc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6c,
                           0x7c, 0, 0, 0, 0, 0,
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
                      name: b"inca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4c, 0, 0, 0, 0, 0,
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
                      name: b"incb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5c, 0, 0, 0, 0, 0,
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
                      name: b"jmp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6e,
                           0x7e, 0, 0, 0, 0, 0,
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
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x9d,
                           0xad,
                           0xbd, 0, 0, 0, 0, 0,
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
                      name: b"ldaa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x86,
                           0x96,
                           0xa6,
                           0xb6, 0, 0, 0, 0, 0,
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
                      name: b"ldab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc6,
                           0xd6,
                           0xe6,
                           0xf6, 0, 0, 0, 0, 0,
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
                      name: b"ldd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xcc,
                           0xdc,
                           0xec,
                           0xfc, 0, 0, 0, 0, 0,
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
                      name: b"mul\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3d, 0, 0, 0, 0, 0,
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
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1, 0, 0, 0, 0, 0,
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
                      name: b"oraa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x8a,
                           0x9a,
                           0xaa,
                           0xba, 0, 0, 0, 0, 0,
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
                      name: b"orab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xca,
                           0xda,
                           0xea,
                           0xfa, 0, 0, 0, 0, 0,
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
                      name: b"psha\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x36, 0, 0, 0, 0, 0,
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
                      name: b"pshb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x37, 0, 0, 0, 0, 0,
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
                      name: b"pshx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3c, 0, 0, 0, 0, 0,
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
                      name: b"pulx\x00" as *const u8 as *const i8,
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
                      name: b"pula\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x32, 0, 0, 0, 0, 0,
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
                      name: b"pulb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x33, 0, 0, 0, 0, 0,
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
                      name: b"rol\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x69,
                           0x79, 0, 0, 0, 0, 0,
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
                      name: b"rola\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x49, 0, 0, 0, 0, 0,
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
                      name: b"rolb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x59, 0, 0, 0, 0, 0,
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
                      name: b"ror\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x66,
                           0x76, 0, 0, 0, 0, 0,
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
                      name: b"rora\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x46, 0, 0, 0, 0, 0,
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
                      name: b"rorb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x56, 0, 0, 0, 0, 0,
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
                      name: b"rti\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3b, 0, 0, 0, 0, 0,
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
                          [0x39, 0, 0, 0, 0, 0,
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
                      name: b"swi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3f, 0, 0, 0, 0, 0,
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
                      name: b"wai\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3e, 0, 0, 0, 0, 0,
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
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x68,
                           0x78, 0, 0, 0, 0, 0,
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
                      name: b"lsl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x68,
                           0x78, 0, 0, 0, 0, 0,
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
                      name: b"asla\x00" as *const u8 as *const i8,
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
                      name: b"aslb\x00" as *const u8 as *const i8,
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
                      name: b"asld\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5, 0, 0, 0, 0, 0,
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
                      name: b"lsla\x00" as *const u8 as *const i8,
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
                      name: b"lslb\x00" as *const u8 as *const i8,
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
                      name: b"lsld\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5, 0, 0, 0, 0, 0,
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
                      name: b"asr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x67,
                           0x77, 0, 0, 0, 0, 0,
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
                      name: b"asra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x47, 0, 0, 0, 0, 0,
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
                      name: b"asrb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x57, 0, 0, 0, 0, 0,
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
                      name: b"cpx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x8c,
                           0x9c,
                           0xac,
                           0xbc, 0, 0, 0, 0, 0,
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
                          [0x9, 0, 0, 0, 0, 0,
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
                      name: b"des\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x34, 0, 0, 0, 0, 0,
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
                      name: b"inx\x00" as *const u8 as *const i8,
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
                      name: b"ins\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x31, 0, 0, 0, 0, 0,
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
                      name: b"ldx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xce,
                           0xde,
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
                      name: b"lds\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x8e,
                           0x9e,
                           0xae,
                           0xbe, 0, 0, 0, 0, 0,
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
                      name: b"lsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x64,
                           0x74, 0, 0, 0, 0, 0,
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
                      name: b"lsra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x44, 0, 0, 0, 0, 0,
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
                      name: b"lsrb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x54, 0, 0, 0, 0, 0,
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
                      name: b"lsrd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4, 0, 0, 0, 0, 0,
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
                      name: b"staa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x97,
                           0xa7,
                           0xb7, 0, 0, 0, 0, 0,
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
                      name: b"stab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xd7,
                           0xe7,
                           0xf7, 0, 0, 0, 0, 0,
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
                      name: b"std\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xdd,
                           0xed,
                           0xfd, 0, 0, 0, 0, 0,
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
                      name: b"sts\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x9f,
                           0xaf,
                           0xbf, 0, 0, 0, 0, 0,
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
                      name: b"stx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xdf,
                           0xef,
                           0xff, 0, 0, 0, 0, 0,
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
                      name: b"suba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x80,
                           0x90,
                           0xa0,
                           0xb0, 0, 0, 0, 0, 0,
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
                      name: b"subb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc0,
                           0xd0,
                           0xe0,
                           0xf0, 0, 0, 0, 0, 0,
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
                      name: b"subd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x83,
                           0x93,
                           0xa3,
                           0xb3, 0, 0, 0, 0, 0,
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
                      name: b"sba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"sbca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x82,
                           0x92,
                           0xa2,
                           0xb2, 0, 0, 0, 0, 0,
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
                      name: b"sbcb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xc2,
                           0xd2,
                           0xe2,
                           0xf2, 0, 0, 0, 0, 0,
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
                      name: b"tab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x16, 0, 0, 0, 0, 0,
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
                      name: b"tba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x17, 0, 0, 0, 0, 0,
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
                      name: b"tst\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6d,
                           0x7d, 0, 0, 0, 0, 0,
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
                      name: b"tsta\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4d, 0, 0, 0, 0, 0,
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
                      name: b"tstb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5d, 0, 0, 0, 0, 0,
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
                      name: b"txs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x35, 0, 0, 0, 0, 0,
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
#[no_mangle]
pub static mut MneHD6303: [_MNE; 7] =
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"slp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1a, 0, 0, 0, 0, 0,
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
                      name: b"xgdx\x00" as *const u8 as *const i8,
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
                      name: b"aim\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32) as u64,
                      opcode:
                          [0x71,
                           0x61, 0, 0, 0, 0, 0,
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
                      name: b"oim\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32) as u64,
                      opcode:
                          [0x72,
                           0x62, 0, 0, 0, 0, 0,
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
                      name: b"eim\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32) as u64,
                      opcode:
                          [0x75,
                           0x65, 0, 0, 0, 0, 0,
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
                      name: b"tim\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32) as u64,
                      opcode:
                          [0x7b,
                           0x6b, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
