#![no_std]
#![no_main]

use libtinyos::syscalls::{self, OpenOptions};

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: *const u8, _: *const u8, buf_size: usize) -> ! {
    if argc.is_null() || buf_size == 0 {
        unsafe { syscalls::exit(0) };
    }
    if let Ok(f) = unsafe { syscalls::open(argc, buf_size, OpenOptions::CREATE) } {
        // might want to keep this open, as touch is likely done before some edit.
        _ = unsafe { syscalls::close(f) };
    }
    unsafe { syscalls::exit(0) }
}
