#![no_std]
#![no_main]

use libtinyos::syscalls::{self, STDERR_FILENO, STDOUT_FILENO};

#[unsafe(no_mangle)]
pub extern "C" fn _start(arg: *const u8, _: *const u8, buf_size: usize) -> ! {
    if !arg.is_null() {
        if unsafe { syscalls::write(STDOUT_FILENO, arg, buf_size) }.is_err() {
            let bytes = b"could not write messgae to stdout";
            _ = unsafe { syscalls::write(STDERR_FILENO, bytes.as_ptr(), bytes.len()) };
        }
    } else {
        _ = unsafe { syscalls::write(STDOUT_FILENO, b"no_arg".as_ptr(), b"no_arg".len()) };
    }

    unsafe { syscalls::exit(0) }
}
