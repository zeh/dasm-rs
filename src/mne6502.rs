use libc;

extern "C" {
    #[no_mangle]
    fn v_mnemonic(str: *mut libc::c_char, mne: *mut _MNE);
}
pub type ADDRESS_MODES = libc::c_uint;
pub const NUMOC: ADDRESS_MODES = 21;
pub const AM_BSS: ADDRESS_MODES = 20;
pub const AM_LONG: ADDRESS_MODES = 19;
pub const AM_EXPLIST: ADDRESS_MODES = 18;
pub const AM_SYMBOL: ADDRESS_MODES = 17;
pub const AM_BITBRAMOD: ADDRESS_MODES = 16;
pub const AM_BITMOD: ADDRESS_MODES = 15;
pub const AM_0Y: ADDRESS_MODES = 14;
pub const AM_0X: ADDRESS_MODES = 13;
pub const AM_INDWORD: ADDRESS_MODES = 12;
pub const AM_INDBYTEY: ADDRESS_MODES = 11;
pub const AM_INDBYTEX: ADDRESS_MODES = 10;
pub const AM_REL: ADDRESS_MODES = 9;
pub const AM_WORDADRY: ADDRESS_MODES = 8;
pub const AM_WORDADRX: ADDRESS_MODES = 7;
pub const AM_WORDADR: ADDRESS_MODES = 6;
pub const AM_BYTEADRY: ADDRESS_MODES = 5;
pub const AM_BYTEADRX: ADDRESS_MODES = 4;
pub const AM_BYTEADR: ADDRESS_MODES = 3;
pub const AM_IMM16: ADDRESS_MODES = 2;
pub const AM_IMM8: ADDRESS_MODES = 1;
pub const AM_IMP: ADDRESS_MODES = 0;
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
 *  MNE6502.C
 */
#[no_mangle]
pub static mut Mne6502: [_MNE; 76] =
    unsafe {
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"adc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x69 as libc::c_int as libc::c_uint,
                           0x65 as libc::c_int as libc::c_uint,
                           0x75 as libc::c_int as libc::c_uint,
                           0x6d as libc::c_int as libc::c_uint,
                           0x7d as libc::c_int as libc::c_uint,
                           0x79 as libc::c_int as libc::c_uint,
                           0x61 as libc::c_int as libc::c_uint,
                           0x71 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"anc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int) as
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
                      name: b"and\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x29 as libc::c_int as libc::c_uint,
                           0x25 as libc::c_int as libc::c_uint,
                           0x35 as libc::c_int as libc::c_uint,
                           0x2d as libc::c_int as libc::c_uint,
                           0x3d as libc::c_int as libc::c_uint,
                           0x39 as libc::c_int as libc::c_uint,
                           0x21 as libc::c_int as libc::c_uint,
                           0x31 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ane\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"arr\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x6b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa as libc::c_int as libc::c_uint,
                           0x6 as libc::c_int as libc::c_uint,
                           0x16 as libc::c_int as libc::c_uint,
                           0xe as libc::c_int as libc::c_uint,
                           0x1e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x4b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x90 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xb0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xf0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"bit\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x24 as libc::c_int as libc::c_uint,
                           0x2c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"bmi\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
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
                      name: b"bne\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xd0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
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
                      name: b"brk\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0,
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
                      name: b"bvc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
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
                      name: b"bvs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_REL as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x70 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"cld\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xd8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"clv\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xb8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"cmp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc9 as libc::c_int as libc::c_uint,
                           0xc5 as libc::c_int as libc::c_uint,
                           0xd5 as libc::c_int as libc::c_uint,
                           0xcd as libc::c_int as libc::c_uint,
                           0xdd as libc::c_int as libc::c_uint,
                           0xd9 as libc::c_int as libc::c_uint,
                           0xc1 as libc::c_int as libc::c_uint,
                           0xd1 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xe0 as libc::c_int as libc::c_uint,
                           0xe4 as libc::c_int as libc::c_uint,
                           0xec as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"cpy\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc0 as libc::c_int as libc::c_uint,
                           0xc4 as libc::c_int as libc::c_uint,
                           0xcc as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"dcp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc7 as libc::c_int as libc::c_uint,
                           0xd7 as libc::c_int as libc::c_uint,
                           0xcf as libc::c_int as libc::c_uint,
                           0xdf as libc::c_int as libc::c_uint,
                           0xdb as libc::c_int as libc::c_uint,
                           0xc3 as libc::c_int as libc::c_uint,
                           0xd3 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc6 as libc::c_int as libc::c_uint,
                           0xd6 as libc::c_int as libc::c_uint,
                           0xce as libc::c_int as libc::c_uint,
                           0xde as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xca as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"dey\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x88 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"eor\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x49 as libc::c_int as libc::c_uint,
                           0x45 as libc::c_int as libc::c_uint,
                           0x55 as libc::c_int as libc::c_uint,
                           0x4d as libc::c_int as libc::c_uint,
                           0x5d as libc::c_int as libc::c_uint,
                           0x59 as libc::c_int as libc::c_uint,
                           0x41 as libc::c_int as libc::c_uint,
                           0x51 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xe6 as libc::c_int as libc::c_uint,
                           0xf6 as libc::c_int as libc::c_uint,
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
                      name: b"inx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xe8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"iny\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xc8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"isb\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xe7 as libc::c_int as libc::c_uint,
                           0xf7 as libc::c_int as libc::c_uint,
                           0xef as libc::c_int as libc::c_uint,
                           0xff as libc::c_int as libc::c_uint,
                           0xfb as libc::c_int as libc::c_uint,
                           0xe3 as libc::c_int as libc::c_uint,
                           0xf3 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDWORD as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x4c as libc::c_int as libc::c_uint,
                           0x6c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_WORDADR as libc::c_int)
                              as libc::c_ulong,
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
                      name: b"las\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_WORDADRY as libc::c_int)
                              as libc::c_ulong,
                      opcode:
                          [0xbb as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"lax\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa7 as libc::c_int as libc::c_uint,
                           0xb7 as libc::c_int as libc::c_uint,
                           0xaf as libc::c_int as libc::c_uint,
                           0xbf as libc::c_int as libc::c_uint,
                           0xa3 as libc::c_int as libc::c_uint,
                           0xb3 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lda\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa9 as libc::c_int as libc::c_uint,
                           0xa5 as libc::c_int as libc::c_uint,
                           0xb5 as libc::c_int as libc::c_uint,
                           0xad as libc::c_int as libc::c_uint,
                           0xbd as libc::c_int as libc::c_uint,
                           0xb9 as libc::c_int as libc::c_uint,
                           0xa1 as libc::c_int as libc::c_uint,
                           0xb1 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa2 as libc::c_int as libc::c_uint,
                           0xa6 as libc::c_int as libc::c_uint,
                           0xb6 as libc::c_int as libc::c_uint,
                           0xae as libc::c_int as libc::c_uint,
                           0xbe as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ldy\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa0 as libc::c_int as libc::c_uint,
                           0xa4 as libc::c_int as libc::c_uint,
                           0xb4 as libc::c_int as libc::c_uint,
                           0xac as libc::c_int as libc::c_uint,
                           0xbc as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x4a as libc::c_int as libc::c_uint,
                           0x46 as libc::c_int as libc::c_uint,
                           0x56 as libc::c_int as libc::c_uint,
                           0x4e as libc::c_int as libc::c_uint,
                           0x5e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"lxa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xab as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int |
                               (1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xea as libc::c_int as libc::c_uint,
                           0x80 as libc::c_int as libc::c_uint,
                           0x4 as libc::c_int as libc::c_uint,
                           0x14 as libc::c_int as libc::c_uint,
                           0xc as libc::c_int as libc::c_uint,
                           0x1c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"ora\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x9 as libc::c_int as libc::c_uint,
                           0x5 as libc::c_int as libc::c_uint,
                           0x15 as libc::c_int as libc::c_uint,
                           0xd as libc::c_int as libc::c_uint,
                           0x1d as libc::c_int as libc::c_uint,
                           0x19 as libc::c_int as libc::c_uint,
                           0x1 as libc::c_int as libc::c_uint,
                           0x11 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"pha\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"php\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"pla\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x68 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"plp\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"rla\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x27 as libc::c_int as libc::c_uint,
                           0x37 as libc::c_int as libc::c_uint,
                           0x2f as libc::c_int as libc::c_uint,
                           0x3f as libc::c_int as libc::c_uint,
                           0x3b as libc::c_int as libc::c_uint,
                           0x23 as libc::c_int as libc::c_uint,
                           0x33 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x2a as libc::c_int as libc::c_uint,
                           0x26 as libc::c_int as libc::c_uint,
                           0x36 as libc::c_int as libc::c_uint,
                           0x2e as libc::c_int as libc::c_uint,
                           0x3e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x6a as libc::c_int as libc::c_uint,
                           0x66 as libc::c_int as libc::c_uint,
                           0x76 as libc::c_int as libc::c_uint,
                           0x6e as libc::c_int as libc::c_uint,
                           0x7e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"rra\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x67 as libc::c_int as libc::c_uint,
                           0x77 as libc::c_int as libc::c_uint,
                           0x6f as libc::c_int as libc::c_uint,
                           0x7f as libc::c_int as libc::c_uint,
                           0x7b as libc::c_int as libc::c_uint,
                           0x63 as libc::c_int as libc::c_uint,
                           0x73 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"rts\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x60 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"sax\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x87 as libc::c_int as libc::c_uint,
                           0x97 as libc::c_int as libc::c_uint,
                           0x8f as libc::c_int as libc::c_uint,
                           0x83 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"sbc\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xe9 as libc::c_int as libc::c_uint,
                           0xe5 as libc::c_int as libc::c_uint,
                           0xf5 as libc::c_int as libc::c_uint,
                           0xed as libc::c_int as libc::c_uint,
                           0xfd as libc::c_int as libc::c_uint,
                           0xf9 as libc::c_int as libc::c_uint,
                           0xe1 as libc::c_int as libc::c_uint,
                           0xf1 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sbx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMM8 as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xcb as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
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
                      name: b"sed\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xf8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x78 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"sha\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x9f as libc::c_int as libc::c_uint,
                           0x93 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"shs\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_WORDADRY as libc::c_int)
                              as libc::c_ulong,
                      opcode:
                          [0x9b as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"shx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_WORDADRY as libc::c_int)
                              as libc::c_ulong,
                      opcode:
                          [0x9e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"shy\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_WORDADRX as libc::c_int)
                              as libc::c_ulong,
                      opcode:
                          [0x9c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"slo\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x7 as libc::c_int as libc::c_uint,
                           0x17 as libc::c_int as libc::c_uint,
                           0xf as libc::c_int as libc::c_uint,
                           0x1f as libc::c_int as libc::c_uint,
                           0x1b as libc::c_int as libc::c_uint,
                           0x3 as libc::c_int as libc::c_uint,
                           0x13 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sre\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x47 as libc::c_int as libc::c_uint,
                           0x57 as libc::c_int as libc::c_uint,
                           0x4f as libc::c_int as libc::c_uint,
                           0x5f as libc::c_int as libc::c_uint,
                           0x5b as libc::c_int as libc::c_uint,
                           0x43 as libc::c_int as libc::c_uint,
                           0x53 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mnemonic as
                                   unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut _MNE) -> ()),
                      name: b"sta\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_INDBYTEY as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x85 as libc::c_int as libc::c_uint,
                           0x95 as libc::c_int as libc::c_uint,
                           0x8d as libc::c_int as libc::c_uint,
                           0x9d as libc::c_int as libc::c_uint,
                           0x99 as libc::c_int as libc::c_uint,
                           0x81 as libc::c_int as libc::c_uint,
                           0x91 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0],};
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
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRY as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x86 as libc::c_int as libc::c_uint,
                           0x96 as libc::c_int as libc::c_uint,
                           0x8e as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"sty\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_BYTEADR as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_BYTEADRX as libc::c_int |
                               (1 as libc::c_long) <<
                                   AM_WORDADR as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x84 as libc::c_int as libc::c_uint,
                           0x94 as libc::c_int as libc::c_uint,
                           0x8c as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"tax\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xaa as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"tay\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xa8 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0xba as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"txa\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x8a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x9a as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
                      name: b"tya\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as libc::c_uchar,
                      okmask:
                          ((1 as libc::c_long) << AM_IMP as libc::c_int) as
                              libc::c_ulong,
                      opcode:
                          [0x98 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0,
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
