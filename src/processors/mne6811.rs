use crate::types::enums::{
    AddressModes,
};

extern "C" {
    #[no_mangle]
    fn v_mnemonic(str: *mut i8, mne: *mut _MNE);
}
/*  has mask argument (byte)    */
/*  has rel. address (byte)    */
/*  instruction byte mod.    */
/*  is v_endm            */
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
#[no_mangle]
pub static mut Mne68HC11: [_MNE; 146] =
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
                          [0x1b as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x3a as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"aby\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x183a as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x89 as i32 as u32,
                           0x99 as i32 as u32,
                           0xa9 as i32 as u32,
                           0x18a9 as i32 as u32,
                           0xb9 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"adcb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc9 as i32 as u32,
                           0xd9 as i32 as u32,
                           0xe9 as i32 as u32,
                           0x18e9 as i32 as u32,
                           0xf9 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"adda\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x8b as i32 as u32,
                           0x9b as i32 as u32,
                           0xab as i32 as u32,
                           0x18ab as i32 as u32,
                           0xbb as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"addb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xcb as i32 as u32,
                           0xdb as i32 as u32,
                           0xeb as i32 as u32,
                           0x18eb as i32 as u32,
                           0xfb as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"addd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc3 as i32 as u32,
                           0xd3 as i32 as u32,
                           0xe3 as i32 as u32,
                           0x18e3 as i32 as u32,
                           0xf3 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"anda\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x84 as i32 as u32,
                           0x94 as i32 as u32,
                           0xa4 as i32 as u32,
                           0x18a4 as i32 as u32,
                           0xb4 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"andb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc4 as i32 as u32,
                           0xd4 as i32 as u32,
                           0xe4 as i32 as u32,
                           0x18e4 as i32 as u32,
                           0xf4 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"asla\x00" as *const u8 as *const i8,
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
                      name: b"aslb\x00" as *const u8 as *const i8,
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
                      name: b"asl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x68 as i32 as u32,
                           0x1868 as i32 as u32,
                           0x78 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"asld\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"asra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x47 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x57 as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x67 as i32 as u32,
                           0x1867 as i32 as u32,
                           0x77 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"bcc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x24 as i32 as u32, 0, 0, 0, 0, 0,
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
                      flags: 0x10 as i32 as u8,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
                      opcode:
                          [0x15 as i32 as u32,
                           0x1d as i32 as u32,
                           0x181d as i32 as u32, 0, 0, 0, 0,
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
                      name: b"bcs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x25 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x27 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x2c as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x2e as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x22 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x24 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"bita\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x85 as i32 as u32,
                           0x95 as i32 as u32,
                           0xa5 as i32 as u32,
                           0x18a5 as i32 as u32,
                           0xb5 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"bitb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc5 as i32 as u32,
                           0xd5 as i32 as u32,
                           0xe5 as i32 as u32,
                           0x18e5 as i32 as u32,
                           0xf5 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"ble\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x2f as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x25 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x23 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x2d as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x2b as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x26 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x2a as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"brclr\x00" as *const u8 as *const i8,
                      flags:
                          (0x10 as i32 | 0x20 as i32) as u8,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
                      opcode:
                          [0x13 as i32 as u32,
                           0x1f as i32 as u32,
                           0x181f as i32 as u32, 0, 0, 0, 0,
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
                      name: b"brn\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x21 as i32 as u32, 0, 0, 0, 0, 0,
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
                          (0x10 as i32 | 0x20 as i32) as u8,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
                      opcode:
                          [0x12 as i32 as u32,
                           0x1e as i32 as u32,
                           0x181e as i32 as u32, 0, 0, 0, 0,
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
                      flags: 0x10 as i32 as u8,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32) as u64,
                      opcode:
                          [0x14 as i32 as u32,
                           0x1c as i32 as u32,
                           0x181c as i32 as u32, 0, 0, 0, 0,
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
                      name: b"bsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x8d as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"bvs\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Rel as i32) as u64,
                      opcode:
                          [0x29 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"cba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x11 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0xc as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0xe as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"clra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4f as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x5f as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6f as i32 as u32,
                           0x186f as i32 as u32,
                           0x7f as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"clv\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xa as i32 as u32, 0, 0, 0, 0, 0,
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
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x81 as i32 as u32,
                           0x91 as i32 as u32,
                           0xa1 as i32 as u32,
                           0x18a1 as i32 as u32,
                           0xb1 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"cmpb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc1 as i32 as u32,
                           0xd1 as i32 as u32,
                           0xe1 as i32 as u32,
                           0x18e1 as i32 as u32,
                           0xf1 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"coma\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x43 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x53 as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x63 as i32 as u32,
                           0x1863 as i32 as u32,
                           0x73 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"cpd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x1a83 as i32 as u32,
                           0x1a93 as i32 as u32,
                           0x1aa3 as i32 as u32,
                           0xcda3 as i32 as u32,
                           0x1ab3 as i32 as u32, 0, 0, 0, 0,
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
                      name: b"cpx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x8c as i32 as u32,
                           0x9c as i32 as u32,
                           0xac as i32 as u32,
                           0xcdac as i32 as u32,
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
                      name: b"cpy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x188c as i32 as u32,
                           0x189c as i32 as u32,
                           0x1aac as i32 as u32,
                           0x18ac as i32 as u32,
                           0x18bc as i32 as u32, 0, 0, 0, 0,
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
                      name: b"daa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x19 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"deca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4a as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x5a as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6a as i32 as u32,
                           0x186a as i32 as u32,
                           0x7a as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"des\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x34 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x9 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x1809 as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x88 as i32 as u32,
                           0x98 as i32 as u32,
                           0xa8 as i32 as u32,
                           0x18a8 as i32 as u32,
                           0xb8 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"eorb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc8 as i32 as u32,
                           0xd8 as i32 as u32,
                           0xe8 as i32 as u32,
                           0x18e8 as i32 as u32,
                           0xf8 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"fdiv\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"idiv\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x2 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"inca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4c as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x5c as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"inc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6c as i32 as u32,
                           0x186c as i32 as u32,
                           0x7c as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"ins\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x31 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"iny\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1808 as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6e as i32 as u32,
                           0x186e as i32 as u32,
                           0x7e as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"jsr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x9d as i32 as u32,
                           0xad as i32 as u32,
                           0x18ad as i32 as u32,
                           0xbd as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"ldaa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x86 as i32 as u32,
                           0x96 as i32 as u32,
                           0xa6 as i32 as u32,
                           0x18a6 as i32 as u32,
                           0xb6 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"ldab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc6 as i32 as u32,
                           0xd6 as i32 as u32,
                           0xe6 as i32 as u32,
                           0x18e6 as i32 as u32,
                           0xf6 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"ldd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xcc as i32 as u32,
                           0xdc as i32 as u32,
                           0xec as i32 as u32,
                           0x18ec as i32 as u32,
                           0xfc as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"lds\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x8e as i32 as u32,
                           0x9e as i32 as u32,
                           0xae as i32 as u32,
                           0x18ae as i32 as u32,
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
                      name: b"ldx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xce as i32 as u32,
                           0xde as i32 as u32,
                           0xee as i32 as u32,
                           0xcdee as i32 as u32,
                           0xfe as i32 as u32, 0, 0, 0, 0, 0,
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
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x18ce as i32 as u32,
                           0x18de as i32 as u32,
                           0x1aee as i32 as u32,
                           0x18ee as i32 as u32,
                           0x18fe as i32 as u32, 0, 0, 0, 0,
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
                      name: b"lslb\x00" as *const u8 as *const i8,
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
                      name: b"lsl\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x68 as i32 as u32,
                           0x1868 as i32 as u32,
                           0x78 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"lsld\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x5 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"lsra\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x44 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x54 as i32 as u32, 0, 0, 0, 0, 0,
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
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x64 as i32 as u32,
                           0x1864 as i32 as u32,
                           0x74 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"lsrd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"mul\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3d as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"nega\x00" as *const u8 as *const i8,
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
                      name: b"negb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"neg\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x60 as i32 as u32,
                           0x1860 as i32 as u32,
                           0x70 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"nop\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1 as i32 as u32, 0, 0, 0, 0, 0,
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
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x8a as i32 as u32,
                           0x9a as i32 as u32,
                           0xaa as i32 as u32,
                           0x18aa as i32 as u32,
                           0xba as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"orab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xca as i32 as u32,
                           0xda as i32 as u32,
                           0xea as i32 as u32,
                           0x18ea as i32 as u32,
                           0xfa as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"psha\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x36 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x37 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x3c as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"pshy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x183c as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          [0x32 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x33 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"puly\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1838 as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          [0x49 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x59 as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x69 as i32 as u32,
                           0x1869 as i32 as u32,
                           0x79 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"rora\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x46 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x56 as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x66 as i32 as u32,
                           0x1866 as i32 as u32,
                           0x76 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"rti\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3b as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x39 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"sba\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"sbca\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x82 as i32 as u32,
                           0x92 as i32 as u32,
                           0xa2 as i32 as u32,
                           0x18a2 as i32 as u32,
                           0xb2 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"sbcb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc2 as i32 as u32,
                           0xd2 as i32 as u32,
                           0xe2 as i32 as u32,
                           0x18e2 as i32 as u32,
                           0xf2 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"sec\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xd as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0xf as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"staa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x97 as i32 as u32,
                           0xa7 as i32 as u32,
                           0x18a7 as i32 as u32,
                           0xb7 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"stab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xd7 as i32 as u32,
                           0xe7 as i32 as u32,
                           0x18e7 as i32 as u32,
                           0xf7 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"std\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xdd as i32 as u32,
                           0xed as i32 as u32,
                           0x18ed as i32 as u32,
                           0xfd as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"stop\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0xcf as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"sts\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x9f as i32 as u32,
                           0xaf as i32 as u32,
                           0x18af as i32 as u32,
                           0xbf as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"stx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0xdf as i32 as u32,
                           0xef as i32 as u32,
                           0xcdef as i32 as u32,
                           0xff as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"sty\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::ByteAdr as i32 |
                               (1) <<
                                   AddressModes::ByteAdrX as i32 |
                               (1) <<
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x18df as i32 as u32,
                           0x1aef as i32 as u32,
                           0x18ef as i32 as u32,
                           0x18ff as i32 as u32, 0, 0, 0, 0,
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
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x80 as i32 as u32,
                           0x90 as i32 as u32,
                           0xa0 as i32 as u32,
                           0x18a0 as i32 as u32,
                           0xb0 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"subb\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm8 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0xc0 as i32 as u32,
                           0xd0 as i32 as u32,
                           0xe0 as i32 as u32,
                           0x18e0 as i32 as u32,
                           0xf0 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"subd\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imm16 as i32 |
                               ((1) <<
                                    AddressModes::ByteAdr as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrX as i32 |
                                    (1) <<
                                        AddressModes::ByteAdrY as i32 |
                                    (1) <<
                                        AddressModes::WordAdr as i32)) as u64,
                      opcode:
                          [0x83 as i32 as u32,
                           0x93 as i32 as u32,
                           0xa3 as i32 as u32,
                           0x18a3 as i32 as u32,
                           0xb3 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"swi\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x3f as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"tab\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x16 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x6 as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x17 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"test\x00" as *const u8 as *const i8,
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
                      name: b"tpa\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x7 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"tsta\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x4d as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x5d as i32 as u32, 0, 0, 0, 0, 0,
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
                                   AddressModes::ByteAdrY as i32 |
                               (1) <<
                                   AddressModes::WordAdr as i32) as u64,
                      opcode:
                          [0x6d as i32 as u32,
                           0x186d as i32 as u32,
                           0x7d as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"tsx\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
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
                      name: b"tsy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1830 as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          [0x35 as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"tys\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x1835 as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          [0x3e as i32 as u32, 0, 0, 0, 0, 0,
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
                          [0x8f as i32 as u32, 0, 0, 0, 0, 0,
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
                      name: b"xgdy\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask:
                          ((1) << AddressModes::Imp as i32) as u64,
                      opcode:
                          [0x188f as i32 as u32, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
