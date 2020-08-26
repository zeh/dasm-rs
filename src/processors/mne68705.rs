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
 *  MNE68705.C
 */
#[no_mangle]
pub static mut Mne68705: [_MNE; 89] =
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa9,
                           0xb9,
                           0xe9,
                           0xc9,
                           0xd9,
                           0xf9, 0, 0, 0, 0, 0,
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
                      name: b"add\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xab,
                           0xbb,
                           0xeb,
                           0xcb,
                           0xdb,
                           0xfb, 0, 0, 0, 0, 0,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa4,
                           0xb4,
                           0xe4,
                           0xc4,
                           0xd4,
                           0xf4, 0, 0, 0, 0, 0,
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
                      name: b"asl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x48,
                           0x38,
                           0x68,
                           0x78, 0, 0, 0, 0, 0,
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
                      name: b"aslx\x00" as *const u8 as *const i8,
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
                      name: b"asr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x47,
                           0x37,
                           0x67,
                           0x77, 0, 0, 0, 0, 0,
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
                      name: b"asrx\x00" as *const u8 as *const i8,
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
                      name: b"bclr\x00" as *const u8 as *const i8,
                      flags: 0x40,
                      okmask:
                          ((1) << AddressModes::BitMod as i32) as u64,
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
                      name: b"bhcc\x00" as *const u8 as *const i8,
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
                      name: b"bhcs\x00" as *const u8 as *const i8,
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
                      name: b"bih\x00" as *const u8 as *const i8,
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
                      name: b"bil\x00" as *const u8 as *const i8,
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
                      name: b"bit\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa5,
                           0xb5,
                           0xe5,
                           0xc5,
                           0xd5,
                           0xf5, 0, 0, 0, 0, 0,
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
                      name: b"bmc\x00" as *const u8 as *const i8,
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
                      name: b"bms\x00" as *const u8 as *const i8,
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
                      name: b"brclr\x00" as *const u8 as *const i8,
                      flags:
                          (0x40 as i32 | 0x20 as i32) as u8,
                      okmask:
                          ((1) << AddressModes::BitBraMod as i32) as u64,
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
                      name: b"brset\x00" as *const u8 as *const i8,
                      flags:
                          (0x40 as i32 | 0x20 as i32) as u8,
                      okmask:
                          ((1) << AddressModes::BitBraMod as i32) as u64,
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
                      name: b"bset\x00" as *const u8 as *const i8,
                      flags: 0x40,
                      okmask:
                          ((1) << AddressModes::BitMod as i32) as u64,
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
                      name: b"bsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0xad, 0, 0, 0, 0, 0,
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
                          [0x98, 0, 0, 0, 0, 0,
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
                      name: b"clr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x4f,
                           0x3f,
                           0x6f,
                           0x7f, 0, 0, 0, 0, 0,
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
                      name: b"clrx\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa1,
                           0xb1,
                           0xe1,
                           0xc1,
                           0xd1,
                           0xf1, 0, 0, 0, 0, 0,
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
                      name: b"com\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x43,
                           0x33,
                           0x63,
                           0x73, 0, 0, 0, 0, 0,
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
                      name: b"comx\x00" as *const u8 as *const i8,
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
                      name: b"cpx\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa3,
                           0xb3,
                           0xe3,
                           0xc3,
                           0xd3,
                           0xf3, 0, 0, 0, 0, 0,
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
                      name: b"dec\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x4a,
                           0x3a,
                           0x6a,
                           0x7a, 0, 0, 0, 0, 0,
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
                      name: b"decx\x00" as *const u8 as *const i8,
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
                      name: b"dex\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa8,
                           0xb8,
                           0xe8,
                           0xc8,
                           0xd8,
                           0xf8, 0, 0, 0, 0, 0,
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
                      name: b"inc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x4c,
                           0x3c,
                           0x6c,
                           0x7c, 0, 0, 0, 0, 0,
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
                      name: b"incx\x00" as *const u8 as *const i8,
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
                      name: b"inx\x00" as *const u8 as *const i8,
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
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xbc,
                           0xec,
                           0xcc,
                           0xdc,
                           0xfc, 0, 0, 0, 0, 0,
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
                      name: b"jsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xbd,
                           0xed,
                           0xcd,
                           0xdd,
                           0xfd, 0, 0, 0, 0, 0,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa6,
                           0xb6,
                           0xe6,
                           0xc6,
                           0xd6,
                           0xf6, 0, 0, 0, 0, 0,
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
                      name: b"ldx\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xae,
                           0xbe,
                           0xee,
                           0xce,
                           0xde,
                           0xfe, 0, 0, 0, 0, 0,
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
                      name: b"lsl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x48,
                           0x38,
                           0x68,
                           0x78, 0, 0, 0, 0, 0,
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
                      name: b"lslx\x00" as *const u8 as *const i8,
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
                      name: b"lsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x44,
                           0x34,
                           0x64,
                           0x74, 0, 0, 0, 0, 0,
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
                      name: b"lsrx\x00" as *const u8 as *const i8,
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
                      name: b"neg\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x40,
                           0x30,
                           0x60,
                           0x70, 0, 0, 0, 0, 0,
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
                      name: b"negx\x00" as *const u8 as *const i8,
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
                      name: b"nop\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x9d, 0, 0, 0, 0, 0,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xaa,
                           0xba,
                           0xea,
                           0xca,
                           0xda,
                           0xfa, 0, 0, 0, 0, 0,
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
                      name: b"rol\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x49,
                           0x39,
                           0x69,
                           0x79, 0, 0, 0, 0, 0,
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
                      name: b"rolx\x00" as *const u8 as *const i8,
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
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x46,
                           0x36,
                           0x66,
                           0x76, 0, 0, 0, 0, 0,
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
                      name: b"rorx\x00" as *const u8 as *const i8,
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
                      name: b"rsp\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"rti\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x80, 0, 0, 0, 0, 0,
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
                          [0x81, 0, 0, 0, 0, 0,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa2,
                           0xb2,
                           0xe2,
                           0xc2,
                           0xd2,
                           0xf2, 0, 0, 0, 0, 0,
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
                      name: b"sec\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x99, 0, 0, 0, 0, 0,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xb7,
                           0xe7,
                           0xc7,
                           0xd7,
                           0xf7, 0, 0, 0, 0, 0,
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
                      name: b"stx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32 |
                               (1) <<
                                   AddressModes::WordAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xbf,
                           0xef,
                           0xcf,
                           0xdf,
                           0xff, 0, 0, 0, 0, 0,
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
                      name: b"sub\x00" as *const u8 as *const i8,
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
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0xa0,
                           0xb0,
                           0xe0,
                           0xc0,
                           0xd0,
                           0xf0, 0, 0, 0, 0, 0,
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
                      name: b"swi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x83, 0, 0, 0, 0, 0,
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
                      name: b"tax\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x97, 0, 0, 0, 0, 0,
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
                          ((1) << AddressModes::Imp as i32 |
                               (1) <<
                                   AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) << AddressModes::ZeroX as i32) as u64,
                      opcode:
                          [0x4d,
                           0x3d,
                           0x6d,
                           0x7d, 0, 0, 0, 0, 0,
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
                      name: b"tstx\x00" as *const u8 as *const i8,
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
                      name: b"txa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x9f, 0, 0, 0, 0, 0,
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
