// FIXME: this file is temporary and should eventually be dropped, as all structs get migrated
// to proper types.

extern "C" {
	pub type _IO_wide_data;
	pub type _IO_codecvt;
	pub type _IO_marker;
}

pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __compar_fn_t = Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _STRLIST {
	pub next: *mut _STRLIST,
    // FIXME: Conversion note: this buffer size was 4 originally. In some cases
    // (labelchanges.asm), increasing it to 16 made it work. In other cases
    // (demo.asm) it's stil broken. That's because those files depend on a
    // buffer overrun behavior of _STRLIST (search for "conversion note").
    // For now, this will stay here, but this behavior should be re-checked
    // once use of _STRLIST is dropped.
	pub buf: [i8; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MNE {
	pub next: *mut _MNE,
	pub vect: Option<unsafe extern "C" fn(_: *mut i8, _: *mut _MNE) -> ()>,
	pub name: *const i8,
	pub flags: u8,
	pub okmask: u64,
	pub opcode: [u32; 21],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MACRO {
	pub next: *mut _MACRO,
	pub vect: Option<unsafe extern "C" fn(_: *mut i8, _: *mut _MACRO) -> ()>,
	pub name: *mut i8,
	pub flags: u8,
	pub strlist: *mut _STRLIST,
	pub defpass: i32,
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
pub struct _SYMBOL {
	pub next: *mut _SYMBOL,
	pub name: *mut i8,
	pub string: *mut i8,
	pub flags: u8,
	pub addrmode: u8,
	pub value: i64,
	pub namelen: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union align {
	pub l: i64,
	pub p: *mut libc::c_void,
	pub fp: Option<unsafe extern "C" fn() -> ()>,
}

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
