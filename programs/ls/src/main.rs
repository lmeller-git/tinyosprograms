#![no_std]
#![no_main]

use alloc::borrow::ToOwned;
use libtinyos::{
    os::args,
    path::Path,
    process::ProcessError,
    syscalls::{self, OpenOptions, STDOUT_FILENO},
};

extern crate alloc;

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    // TODO check if target is file, if so do not read
    let path = args().map(|arg| arg.as_str()).unwrap_or_default();
    let mut path = Path::new(path).to_owned();
    path.canonicalize();

    let mut buf = [0; 128];

    let dir = unsafe {
        syscalls::open(
            path.as_str().as_ptr(),
            path.as_str().len(),
            OpenOptions::READ,
        )
    }
    .map_err(ProcessError::Sys)?;

    while let Ok(n_read) =
        unsafe { syscalls::read(dir, buf.as_mut_ptr(), buf.len(), -1_isize as usize) }
        && n_read > 0
    {
        _ = unsafe {
            syscalls::write(
                STDOUT_FILENO,
                buf[..n_read as usize].as_ptr(),
                n_read as usize,
            )
        };
        if (n_read as usize) < buf.len() {
            break;
        }
    }
    Ok(())
}
