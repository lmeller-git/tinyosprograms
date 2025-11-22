#![no_std]
#![no_main]
#![feature(str_from_raw_parts)]

use alloc::str;
use libtinyos::{
    serial_println,
    syscalls::{self, STDERR_FILENO, STDOUT_FILENO},
};
extern crate alloc;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: *const u8, _: *const u8, buf_size: usize) -> ! {
    serial_println!("received: {:#x} and {}", argc as usize, buf_size);
    if !argc.is_null() {
        let s = unsafe { str::from_raw_parts(argc, buf_size) };
        serial_println!("received: {}", s);
        if unsafe { syscalls::write(STDOUT_FILENO, argc, buf_size) }.is_err() {
            let bytes = b"could not write messgae to stdout";
            _ = unsafe { syscalls::write(STDERR_FILENO, bytes.as_ptr(), bytes.len()) };
        }
    } else {
        _ = unsafe { syscalls::write(STDOUT_FILENO, b"no_arg".as_ptr(), b"no_arg".len()) };
    }

    unsafe { syscalls::exit(0) }
}
