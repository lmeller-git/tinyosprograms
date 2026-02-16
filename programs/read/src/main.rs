#![no_std]
#![no_main]
#![feature(str_from_raw_parts)]

use libtinyos::{
    serial_println,
    syscalls::{self, STDIN_FILENO, STDOUT_FILENO},
};
extern crate alloc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: usize, argv: *const u8) -> ! {
    serial_println!("received: {:#x} and {}", argv as usize, argc);
    let fd = if argv.is_null() {
        STDIN_FILENO
    } else {
        alloc::str::from_utf8(unsafe { core::slice::from_raw_parts(argv, argc) })
            .unwrap()
            .parse()
            .unwrap_or(STDIN_FILENO)
    };

    let mut buf = [0; 128];

    while let Ok(n) = unsafe { syscalls::read(fd, buf.as_mut_ptr(), buf.len(), -1_isize as usize) }
        && n >= 0
    {
        unsafe { syscalls::write(STDOUT_FILENO, buf[..n as usize].as_ptr(), n as usize) }.unwrap();
    }

    unsafe { syscalls::exit(0) }
}
