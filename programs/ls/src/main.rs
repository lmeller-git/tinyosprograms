#![no_std]
#![no_main]

use libtinyos::syscalls::{self, OpenOptions, STDERR_FILENO, STDOUT_FILENO};

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: *const u8, _: *const u8, buf_size: usize) -> ! {
    // TODO allocatre larfer buffer if necessary to read all
    // TODO check if target is file, if so do not read
    let mut buf = [0; 128];
    let has_args = !argc.is_null();

    if let Ok(dir) = unsafe {
        syscalls::open(
            if has_args { argc } else { b"/".as_ptr() },
            if has_args { buf_size } else { 1 },
            OpenOptions::READ,
        )
    } && let Ok(n_read) = unsafe { syscalls::read(dir, buf.as_mut_ptr(), buf.len(), 0) }
        && n_read >= 0
    {
        _ = unsafe {
            syscalls::write(
                STDOUT_FILENO,
                buf[..n_read as usize].as_ptr(),
                n_read as usize,
            )
        };
    } else {
        let msg = b"could not read specified dir";
        _ = unsafe { syscalls::write(STDERR_FILENO, msg.as_ptr(), msg.len()) };
    }
    unsafe { syscalls::exit(0) }
}
