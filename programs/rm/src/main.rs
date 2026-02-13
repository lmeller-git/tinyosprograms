#![no_std]
#![no_main]

use libtinyos::syscalls::{self, STDERR_FILENO};

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: usize, argv: *const u8) -> ! {
    if argv.is_null() || argc == 0 {
        unsafe { syscalls::exit(0) };
    }

    let msg = b"currently not implemented";
    _ = unsafe { syscalls::write(STDERR_FILENO, msg.as_ptr(), msg.len()) };

    unsafe { syscalls::exit(0) }
}
