#![no_std]
#![no_main]

use libtinyos::{
    os::args,
    process::ProcessError,
    syscalls::{self, OpenOptions, STDOUT_FILENO},
};

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    // TODO check if target is not file, if so do not read
    let path = args().unwrap().as_bytes();
    let mut buf = [0; 128];
    let file = unsafe { syscalls::open(path.as_ptr(), path.len(), OpenOptions::READ) }
        .map_err(ProcessError::Sys)?;
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
    Ok(())
}
