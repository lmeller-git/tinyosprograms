#![no_std]
#![no_main]

use libtinyos::syscalls::{self, OpenOptions};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: usize, argv: *const u8) -> ! {
    if argv.is_null() || argc == 0 {
        unsafe { syscalls::exit(0) };
    }
    if let Ok(f) = unsafe { syscalls::open(argv, argc, OpenOptions::CREATE) } {
        // might want to keep this open, as touch is likely done before some edit.
        _ = unsafe { syscalls::close(f) };
    }
    unsafe { syscalls::exit(0) }
}
