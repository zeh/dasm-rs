use crate::constants::{
    M_HASH_SIZE,
    S_HASH_SIZE,
};
use crate::types::legacy::{
    _INCFILE,
    _MNE,
    _SYMBOL,
};

extern "C" {
    #[no_mangle]
    fn v_dc(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_mexit(str: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_list(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_include(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_setstr(str: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_set(str: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_seg(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_align(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_rend(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_rorg(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_org(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_trace(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_end(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_equ(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_ds(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_err(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_hex(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_eqm(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_macro(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_endm(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_incdir(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_incbin(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_ifconst(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_processor(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_repend(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_repeat(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_endif(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_else(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_if(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_ifnconst(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_echo(_: *mut i8, _: *mut _MNE);
    #[no_mangle]
    fn v_subroutine(_: *mut i8, _: *mut _MNE);
}
/*
 *  GLOBALS.C
 */
#[no_mangle]
pub static mut SHash: [*mut _SYMBOL; S_HASH_SIZE] =
    [0 as *const _SYMBOL as *mut _SYMBOL; S_HASH_SIZE];
/*	symbol hash table   */
#[no_mangle]
pub static mut MHash: [*mut _MNE; M_HASH_SIZE] =
    [0 as *const _MNE as *mut _MNE; M_HASH_SIZE];
/*	mnemonic hash table */
#[no_mangle]
pub static mut pIncfile: *mut _INCFILE =
    0 as *const _INCFILE as *mut _INCFILE;
/*	include file stack  */
#[no_mangle]
pub static mut Av: [*mut i8; 256] =
    [0 as *const i8 as *mut i8; 256];
/*	up to 256 arguments */
#[no_mangle]
pub static mut Avbuf: [i8; 512] = [0; 512];
#[no_mangle]
pub static mut Ops: [_MNE; 39] =
        [{
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_list as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"list\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_include as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"include\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_seg as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"seg\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_hex as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"hex\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_err as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"err\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"dc\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"byte\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"word\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"long\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_ds as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"ds\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_dc as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"dv\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_end as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"end\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_trace as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"trace\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_org as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"org\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_rorg as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rorg\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_rend as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"rend\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_align as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"align\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_subroutine as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"subroutine\x00" as *const u8 as
                              *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_equ as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"equ\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_equ as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"=\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_eqm as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"eqm\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_set as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"set\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_setstr as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"setstr\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_macro as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"mac\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_endm as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"endm\x00" as *const u8 as *const i8,
                      flags: 0x80,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_mexit as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"mexit\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_ifconst as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"ifconst\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_ifnconst as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"ifnconst\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_if as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"if\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_else as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"else\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_endif as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"endif\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_endif as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"eif\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_repeat as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"repeat\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_repend as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"repend\x00" as *const u8 as *const i8,
                      flags: 0x4,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_echo as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"echo\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_processor as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name:
                          b"processor\x00" as *const u8 as
                              *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_incbin as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"incbin\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
             init
         },
         {
             let mut init =
                 _MNE{next: 0 as *const _MNE as *mut _MNE,
                      vect:
                          Some(v_incdir as
                                   unsafe extern "C" fn(_: *mut i8,
                                                        _: *mut _MNE) -> ()),
                      name: b"incdir\x00" as *const u8 as *const i8,
                      flags: 0,
                      okmask: 0,
                      opcode:
                          [0, 0, 0, 0, 0, 0, 0,
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
