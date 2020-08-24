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
    fn fclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fopen(__filename: *const i8, __modes: *const i8)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn putc(__c: i32, __stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32)
     -> i32;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> i64;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn atoi(__nptr: *const i8) -> i32;
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
unsafe fn main_0(mut ac: i32, mut av: *mut *mut i8)
 -> i32 {
    let mut format: i32 = 0;
    let mut infile: *mut FILE = 0 as *mut FILE;
    let mut outfile: *mut FILE = 0 as *mut FILE;
    if ac < 3 {
        println!("FTOHEX format infile [outfile]");
        println!("format 1 = DEFAULT, 2 = RAS, or 3 = RAW");
        println!("Copyright (c) 1988-2008 by various authors (see file AUTHORS).");
        std::process::exit(1);
    }
    format = atoi(*av.offset(1));
    if format < 1 || format > 3 {
        exit_with_error("specify infile format 1, 2, or 3");
    }
    infile =
        fopen(*av.offset(2), b"r\x00" as *const u8 as *const i8);
    if infile.is_null() {
        exit_with_error("unable to open input file");
    }
    outfile =
        if !(*av.offset(3)).is_null() {
            fopen(*av.offset(3), b"w\x00" as *const u8 as *const i8)
        } else { stdout };
    if outfile.is_null() {
        exit_with_error("unable to open output file");
    }
    convert(format, infile, outfile);
    fclose(infile);
    fclose(outfile);
    return 0;
}

pub fn exit_with_error(message: &str) {
    eprintln!("{}", message);
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
pub unsafe extern "C" fn convert(mut format: i32, mut in_0: *mut FILE,
                                 mut out: *mut FILE) {
    let mut org: u32 = 0;
    let mut idx: u32 = 0;
    let mut len: i64 = 0;
    let mut buf: [u8; 256] = [0; 256];
    if format < 3 { org = getwlh(in_0) }
    if format == 2 {
        len = getwlh(in_0) as i64
    } else {
        let mut begin: i64 = ftell(in_0);
        fseek(in_0, 0, 2);
        len = ftell(in_0) - begin;
        fseek(in_0, begin, 0);
    }
    loop  {
        while len > 0 {
            let mut chk: u8 = 0;
            let mut i: u32 = 0;
            idx =
                if len > 16 {
                    16
                } else { len } as u32;
            fread(buf.as_mut_ptr() as *mut libc::c_void, idx as size_t,
                  1 as size_t, in_0);
            putc(':' as i32, out);
            puth(idx as u8, out);
            puth((org >> 8) as u8, out);
            puth((org & 0xff as i32 as u32) as u8,
                 out);
            putc('0' as i32, out);
            putc('0' as i32, out);
            chk =
                idx.wrapping_add(org >>
                                     8).wrapping_add(org &
                                                                       0xff as i32 as u32) as u8;
            i = 0;
            while i < idx {
                chk =
                    (chk as i32 + buf[i as usize] as i32) as u8;
                puth(buf[i as usize], out);
                i = i.wrapping_add(1)
            }
            puth(-(chk as i32) as u8, out);
            putc('\r' as i32, out);
            putc('\n' as i32, out);
            len -= idx as i64;
            org = org.wrapping_add(idx)
        }
        if !(format == 2) { break ; }
        org = getwlh(in_0);
        if feof(in_0) != 0 { break ; }
        len = getwlh(in_0) as i64
    }
    fprintf(out, b":00000001FF\r\n\x00" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn getwlh(mut in_0: *mut FILE) -> u32 {
    let mut result: u32 = 0;
    result = getc(in_0) as u32;
    result =
        result.wrapping_add((getc(in_0) << 8) as u32);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn puth(mut c: u8, mut out: *mut FILE) {
    static mut dig: [i8; 17] =
        [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 0];
    putc(dig[(c as i32 >> 4) as usize] as i32,
         out);
    putc(dig[(c as i32 & 15) as usize] as i32,
         out);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        std::process::exit(
            main_0(
                (args.len() - 1) as i32,
                args.as_mut_ptr() as *mut *mut i8
            ) as i32
        );
    }
}
