#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

use libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe fn main_0(mut ac: libc::c_int, mut av: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut format: libc::c_int = 0;
    let mut infile: *mut FILE = 0 as *mut FILE;
    let mut outfile: *mut FILE = 0 as *mut FILE;
    if ac < 3 {
        println!("FTOHEX format infile [outfile]");
        println!("format 1 = DEFAULT, 2 = RAS, or 3 = RAW");
        println!("Copyright (c) 1988-2008 by various authors (see file AUTHORS).");
        std::process::exit(1);
    }
    format = atoi(*av.offset(1 as isize));
    if format < 1 || format > 3 {
        exiterr(b"specify infile format 1, 2, or 3\x00" as *const u8 as
                    *const libc::c_char);
    }
    infile =
        fopen(*av.offset(2 as isize),
              b"r\x00" as *const u8 as *const libc::c_char);
    if infile.is_null() {
        exiterr(b"unable to open input file\x00" as *const u8 as
                    *const libc::c_char);
    }
    outfile =
        if !(*av.offset(3 as isize)).is_null() {
            fopen(*av.offset(3 as isize),
                  b"w\x00" as *const u8 as *const libc::c_char)
        } else { stdout };
    if outfile.is_null() {
        exiterr(b"unable to open output file\x00" as *const u8 as
                    *const libc::c_char);
    }
    convert(format, infile, outfile);
    fclose(infile);
    fclose(outfile);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn exiterr(mut str: *const libc::c_char) {
    fputs(str, stderr);
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stderr);
    std::process::exit(1);
}
/*
 *  Formats:
 *
 *  1:	  origin (word:lsb,msb) + data
 *  2:	  origin (word:lsb,msb) + length (word:lsb,msb) + data	(repeat)
 *  3:	  data
 *
 *  Hex output:
 *
 *  :lloooo00(ll bytes hex code)cc	  ll=# of bytes
 *		      oooo=origin
 *			cc=invert of checksum all codes
 */
#[no_mangle]
pub unsafe extern "C" fn convert(mut format: libc::c_int, mut in_0: *mut FILE,
                                 mut out: *mut FILE) {
    let mut org: libc::c_uint = 0;
    let mut idx: libc::c_uint = 0;
    let mut len: libc::c_long = 0;
    let mut buf: [libc::c_uchar; 256] = [0; 256];
    if format < 3 { org = getwlh(in_0) }
    if format == 2 {
        len = getwlh(in_0) as libc::c_long
    } else {
        let mut begin: libc::c_long = ftell(in_0);
        fseek(in_0, 0, 2);
        len = ftell(in_0) - begin;
        fseek(in_0, begin, 0);
    }
    loop  {
        while len > 0 {
            let mut chk: libc::c_uchar = 0;
            let mut i: libc::c_uint = 0;
            idx =
                if len > 16 {
                    16
                } else { len } as libc::c_uint;
            fread(buf.as_mut_ptr() as *mut libc::c_void, idx as size_t,
                  1 as size_t, in_0);
            putc(':' as i32, out);
            puth(idx as libc::c_uchar, out);
            puth((org >> 8) as libc::c_uchar, out);
            puth((org & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar,
                 out);
            putc('0' as i32, out);
            putc('0' as i32, out);
            chk =
                idx.wrapping_add(org >>
                                     8 as
                                         libc::c_int).wrapping_add(org &
                                                                       0xff as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                    as libc::c_uchar;
            i = 0;
            while i < idx {
                chk =
                    (chk as libc::c_int + buf[i as usize] as libc::c_int) as
                        libc::c_uchar;
                puth(buf[i as usize], out);
                i = i.wrapping_add(1)
            }
            puth(-(chk as libc::c_int) as libc::c_uchar, out);
            putc('\r' as i32, out);
            putc('\n' as i32, out);
            len -= idx as libc::c_long;
            org = org.wrapping_add(idx)
        }
        if !(format == 2) { break ; }
        org = getwlh(in_0);
        if feof(in_0) != 0 { break ; }
        len = getwlh(in_0) as libc::c_long
    }
    fprintf(out, b":00000001FF\r\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn getwlh(mut in_0: *mut FILE) -> libc::c_uint {
    let mut result: libc::c_uint = 0;
    result = getc(in_0) as libc::c_uint;
    result =
        result.wrapping_add((getc(in_0) << 8) as libc::c_uint);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn puth(mut c: libc::c_uchar, mut out: *mut FILE) {
    static mut dig: [libc::c_char; 17] =
        [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 0];
    putc(dig[(c as libc::c_int >> 4) as usize] as libc::c_int,
         out);
    putc(dig[(c as libc::c_int & 15) as usize] as libc::c_int,
         out);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char
            ) as i32
        );
    }
}
