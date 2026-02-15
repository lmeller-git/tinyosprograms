#![no_std]
#![no_main]

use libtinyos::syscalls::{self, OpenOptions, STDERR_FILENO, STDOUT_FILENO};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: usize, argv: *const u8) -> ! {
    // TODO check if target is not file, if so do not read
    if argv.is_null() || argc == 0 {
        unsafe { syscalls::exit(0) };
    }

    let mut buf = [0; 128];
    if let Ok(file) = unsafe { syscalls::open(argv, argc, OpenOptions::READ) } {
        while let Ok(n_read) =
            unsafe { syscalls::read(file, buf.as_mut_ptr(), buf.len(), -1_isize as usize) }
            && n_read > 0
        {
            _ = unsafe {
                syscalls::write(
                    STDOUT_FILENO,
                    buf[..n_read as usize].as_ptr(),
                    n_read as usize,
                )
            };
        }
    } else {
        let msg = b"could not read specified file";
        _ = unsafe { syscalls::write(STDERR_FILENO, msg.as_ptr(), msg.len()) };
    }

    unsafe { syscalls::exit(0) }
}
