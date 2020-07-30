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

extern crate libc;

// pub mod src {
    pub mod exp;
    // pub mod ftohex;
    pub mod globals;
    // pub mod main;
    pub mod mne6303;
    pub mod mne6502;
    pub mod mne6811;
    pub mod mne68705;
    pub mod mnef8;
    pub mod ops;
    pub mod symbols;
// }

