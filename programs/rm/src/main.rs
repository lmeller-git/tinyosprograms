#![no_std]
#![no_main]

use libtinyos::syscalls::{self, STDERR_FILENO};

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: *const u8, _: *const u8, _buf_size: usize) -> ! {
    if _argc.is_null() || _buf_size == 0 {
        unsafe { syscalls::exit(0) };
    }

    let msg = b"currently not implemented";
    _ = unsafe { syscalls::write(STDERR_FILENO, msg.as_ptr(), msg.len()) };

    unsafe { syscalls::exit(0) }
}
