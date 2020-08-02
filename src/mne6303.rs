use libc;

use crate::types::enums::{
    AddressModes,
};

extern "C" {
    #[no_mangle]
    fn v_mnemonic(str: *mut libc::c_char, mne: *mut _MNE);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MNE {
    pub next: *mut _MNE,
    pub vect: Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut _MNE)
                         -> ()>,
    pub name: *const libc::c_char,
    pub flags: libc::c_uchar,
    pub okmask: libc::c_ulong,
    pub opcode: [libc::c_uint; 21],
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
    unsafe {
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"aba\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x1b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"abx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x3a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"adca\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x89 as libc::c_int as libc::c_uint,
                           0x99 as libc::c_int as libc::c_uint,
                           0xa9 as libc::c_int as libc::c_uint,
                           0xb9 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"adcb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc9 as libc::c_int as libc::c_uint,
                           0xd9 as libc::c_int as libc::c_uint,
                           0xe9 as libc::c_int as libc::c_uint,
                           0xf9 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"adda\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x8b as libc::c_int as libc::c_uint,
                           0x9b as libc::c_int as libc::c_uint,
                           0xab as libc::c_int as libc::c_uint,
                           0xbb as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"addb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xcb as libc::c_int as libc::c_uint,
                           0xdb as libc::c_int as libc::c_uint,
                           0xeb as libc::c_int as libc::c_uint,
                           0xfb as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"addd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm16 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc3 as libc::c_int as libc::c_uint,
                           0xd3 as libc::c_int as libc::c_uint,
                           0xe3 as libc::c_int as libc::c_uint,
                           0xf3 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"anda\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x84 as libc::c_int as libc::c_uint,
                           0x94 as libc::c_int as libc::c_uint,
                           0xa4 as libc::c_int as libc::c_uint,
                           0xb4 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"andb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc4 as libc::c_int as libc::c_uint,
                           0xd4 as libc::c_int as libc::c_uint,
                           0xe4 as libc::c_int as libc::c_uint,
                           0xf4 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bita\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x85 as libc::c_int as libc::c_uint,
                           0x95 as libc::c_int as libc::c_uint,
                           0xa5 as libc::c_int as libc::c_uint,
                           0xb5 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bitb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc5 as libc::c_int as libc::c_uint,
                           0xd5 as libc::c_int as libc::c_uint,
                           0xe5 as libc::c_int as libc::c_uint,
                           0xf5 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bra\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x20 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"brn\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x21 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bcc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x24 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bhs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x24 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bcs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x25 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"blo\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x25 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"beq\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x27 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bge\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x2c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bgt\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x2e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bhi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x22 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ble\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x2f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bls\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x23 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"blt\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x2d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bmi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x2b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bne\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x26 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bvc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x28 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bvs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x29 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bpl\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x2a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"bsr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Rel as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x8d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"clc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"cli\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xe as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"clv\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xa as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sec\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xd as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sei\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xf as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sev\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xb as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tap\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x6 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tpa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x7 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"clr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x6f as libc::c_int as libc::c_uint,
                           0x7f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"clra\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x4f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"clrb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x5f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"cmpa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x81 as libc::c_int as libc::c_uint,
                           0x91 as libc::c_int as libc::c_uint,
                           0xa1 as libc::c_int as libc::c_uint,
                           0xb1 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"cmpb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc1 as libc::c_int as libc::c_uint,
                           0xd1 as libc::c_int as libc::c_uint,
                           0xe1 as libc::c_int as libc::c_uint,
                           0xf1 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"cba\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x11 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"com\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x63 as libc::c_int as libc::c_uint,
                           0x73 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"coma\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x43 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"comb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x53 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"neg\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x60 as libc::c_int as libc::c_uint,
                           0x70 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"nega\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x40 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"negb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x50 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"daa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x19 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"dec\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x6a as libc::c_int as libc::c_uint,
                           0x7a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"deca\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x4a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"decb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x5a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"eora\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x88 as libc::c_int as libc::c_uint,
                           0x98 as libc::c_int as libc::c_uint,
                           0xa8 as libc::c_int as libc::c_uint,
                           0xb8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"eorb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc8 as libc::c_int as libc::c_uint,
                           0xd8 as libc::c_int as libc::c_uint,
                           0xe8 as libc::c_int as libc::c_uint,
                           0xf8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"inc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x6c as libc::c_int as libc::c_uint,
                           0x7c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"inca\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x4c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"incb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x5c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"jmp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x6e as libc::c_int as libc::c_uint,
                           0x7e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"jsr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x9d as libc::c_int as libc::c_uint,
                           0xad as libc::c_int as libc::c_uint,
                           0xbd as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldaa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x86 as libc::c_int as libc::c_uint,
                           0x96 as libc::c_int as libc::c_uint,
                           0xa6 as libc::c_int as libc::c_uint,
                           0xb6 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldab\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc6 as libc::c_int as libc::c_uint,
                           0xd6 as libc::c_int as libc::c_uint,
                           0xe6 as libc::c_int as libc::c_uint,
                           0xf6 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm16 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xcc as libc::c_int as libc::c_uint,
                           0xdc as libc::c_int as libc::c_uint,
                           0xec as libc::c_int as libc::c_uint,
                           0xfc as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"mul\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x3d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"nop\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x1 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"oraa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x8a as libc::c_int as libc::c_uint,
                           0x9a as libc::c_int as libc::c_uint,
                           0xaa as libc::c_int as libc::c_uint,
                           0xba as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"orab\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xca as libc::c_int as libc::c_uint,
                           0xda as libc::c_int as libc::c_uint,
                           0xea as libc::c_int as libc::c_uint,
                           0xfa as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"psha\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x36 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pshb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x37 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pshx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x3c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pulx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x38 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pula\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x32 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pulb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x33 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rol\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x69 as libc::c_int as libc::c_uint,
                           0x79 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rola\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x49 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rolb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x59 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ror\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x66 as libc::c_int as libc::c_uint,
                           0x76 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rora\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x46 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rorb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x56 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rti\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x3b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rts\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x39 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"swi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x3f as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"wai\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x3e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asl\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x68 as libc::c_int as libc::c_uint,
                           0x78 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsl\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x68 as libc::c_int as libc::c_uint,
                           0x78 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asla\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x48 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"aslb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x58 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asld\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x5 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsla\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x48 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lslb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x58 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsld\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x5 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x67 as libc::c_int as libc::c_uint,
                           0x77 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asra\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x47 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"asrb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x57 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"cpx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm16 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x8c as libc::c_int as libc::c_uint,
                           0x9c as libc::c_int as libc::c_uint,
                           0xac as libc::c_int as libc::c_uint,
                           0xbc as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"dex\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x9 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"des\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x34 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"inx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ins\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x31 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm16 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xce as libc::c_int as libc::c_uint,
                           0xde as libc::c_int as libc::c_uint,
                           0xee as libc::c_int as libc::c_uint,
                           0xfe as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lds\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm16 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x8e as libc::c_int as libc::c_uint,
                           0x9e as libc::c_int as libc::c_uint,
                           0xae as libc::c_int as libc::c_uint,
                           0xbe as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x64 as libc::c_int as libc::c_uint,
                           0x74 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsra\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x44 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsrb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x54 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lsrd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x4 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"staa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x97 as libc::c_int as libc::c_uint,
                           0xa7 as libc::c_int as libc::c_uint,
                           0xb7 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"stab\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xd7 as libc::c_int as libc::c_uint,
                           0xe7 as libc::c_int as libc::c_uint,
                           0xf7 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"std\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xdd as libc::c_int as libc::c_uint,
                           0xed as libc::c_int as libc::c_uint,
                           0xfd as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sts\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x9f as libc::c_int as libc::c_uint,
                           0xaf as libc::c_int as libc::c_uint,
                           0xbf as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"stx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xdf as libc::c_int as libc::c_uint,
                           0xef as libc::c_int as libc::c_uint,
                           0xff as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"suba\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x80 as libc::c_int as libc::c_uint,
                           0x90 as libc::c_int as libc::c_uint,
                           0xa0 as libc::c_int as libc::c_uint,
                           0xb0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"subb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc0 as libc::c_int as libc::c_uint,
                           0xd0 as libc::c_int as libc::c_uint,
                           0xe0 as libc::c_int as libc::c_uint,
                           0xf0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"subd\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm16 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x83 as libc::c_int as libc::c_uint,
                           0x93 as libc::c_int as libc::c_uint,
                           0xa3 as libc::c_int as libc::c_uint,
                           0xb3 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sba\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x10 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sbca\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x82 as libc::c_int as libc::c_uint,
                           0x92 as libc::c_int as libc::c_uint,
                           0xa2 as libc::c_int as libc::c_uint,
                           0xb2 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sbcb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imm8 as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0xc2 as libc::c_int as libc::c_uint,
                           0xd2 as libc::c_int as libc::c_uint,
                           0xe2 as libc::c_int as libc::c_uint,
                           0xf2 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tab\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x16 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tba\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x17 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tst\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdrX as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::WordAdr as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x6d as libc::c_int as libc::c_uint,
                           0x7d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tsta\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x4d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tstb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x5d as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tsx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x30 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"txs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x35 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect: None,
                      name: 0 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         }]
    };
#[no_mangle]
pub static mut MneHD6303: [_MNE; 7] =
    unsafe {
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"slp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x1a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"xgdx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::Imp as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x18 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"aim\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x71 as libc::c_int as libc::c_uint,
                           0x61 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"oim\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x72 as libc::c_int as libc::c_uint,
                           0x62 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"eim\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x75 as libc::c_int as libc::c_uint,
                           0x65 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"tim\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AddressModes::ByteAdr as i32 |
                               (1 as libc::c_long) <<
                                   AddressModes::ByteAdrX as i32) as
                              libc::c_ulong,
                      opcode:
                          [0x7b as libc::c_int as libc::c_uint,
                           0x6b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect: None,
                      name: 0 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask: 0 as libc::c_int as libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         }]
    };
