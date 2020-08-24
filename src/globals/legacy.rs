use libc;

use crate::constants::{
    M_HASH_SIZE,
    S_HASH_SIZE,
};

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type FILE = _IO_FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _STRLIST {
    pub next: *mut _STRLIST,
    pub buf: [i8; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _INCFILE {
    pub next: *mut _INCFILE,
    pub name: *mut i8,
    pub fi: *mut FILE,
    pub lineno: u64,
    pub flags: u8,
    pub args: *mut _STRLIST,
    pub strlist: *mut _STRLIST,
    pub saveidx: u64,
    pub savedolidx: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _REPLOOP {
    pub next: *mut _REPLOOP,
    pub count: u64,
    pub seek: u64,
    pub lineno: u64,
    pub file: *mut _INCFILE,
    pub flags: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFSTACK {
    pub next: *mut _IFSTACK,
    pub file: *mut _INCFILE,
    pub flags: u8,
    pub xtrue: u8,
    pub acctrue: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SEGMENT {
    pub next: *mut _SEGMENT,
    pub name: *mut i8,
    pub flags: u8,
    pub rflags: u8,
    pub org: u64,
    pub rorg: u64,
    pub initorg: u64,
    pub initrorg: u64,
    pub initflags: u8,
    pub initrflags: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SYMBOL {
    pub next: *mut _SYMBOL,
    pub name: *mut i8,
    pub string: *mut i8,
    pub flags: u8,
    pub addrmode: u8,
    pub value: i64,
    pub namelen: u32,
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
pub static mut Reploop: *mut _REPLOOP = 0 as *const _REPLOOP as *mut _REPLOOP;
/*	repeat loop stack   */
#[no_mangle]
pub static mut Ifstack: *mut _IFSTACK = 0 as *const _IFSTACK as *mut _IFSTACK;
/*	IF/ELSE/ENDIF stack */
#[no_mangle]
pub static mut Av: [*mut i8; 256] =
    [0 as *const i8 as *mut i8; 256];
/*	up to 256 arguments */
#[no_mangle]
pub static mut Avbuf: [i8; 512] = [0; 512];
/*unsigned int	Adrbytes[]  = { 1, 2, 3, 2, 2, 2, 3, 3, 3, 2, 2, 2, 3, 1, 1, 2, 3 };*/
#[no_mangle]
pub static mut Cvt: [u32; 17] =
    [0, 2,
     0, 6,
     7, 8,
     9, 0,
     0, 0,
     0, 0,
     0, 4,
     5, 0,
     0];
#[no_mangle]
pub static mut Opsize: [u32; 17] =
    [0, 1,
     2, 1,
     1, 1,
     2, 2,
     2, 2,
     1, 1,
     2, 0,
     0, 1,
     1];
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x80 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
                      flags: 0x4 as i32 as u8,
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
