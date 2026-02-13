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
pub unsafe extern "C" fn main(argc: usize, argv: *const u8) -> ! {
    serial_println!("received: {:#x} and {}", argv as usize, argc);
    if !argv.is_null() {
        let s = unsafe { str::from_raw_parts(argv, argc) };
        serial_println!("received: {}", s);
        if unsafe { syscalls::write(STDOUT_FILENO, argv, argc) }.is_err() {
            let bytes = b"could not write messgae to stdout";
            _ = unsafe { syscalls::write(STDERR_FILENO, bytes.as_ptr(), bytes.len()) };
        }
    } else {
        _ = unsafe { syscalls::write(STDOUT_FILENO, b"no_arg".as_ptr(), b"no_arg".len()) };
    }

    unsafe { syscalls::exit(0) }
}
