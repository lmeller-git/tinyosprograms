#![no_std]
#![no_main]

use libtinyos::{
    os::args,
    process::ProcessError,
    syscalls::{self, OpenOptions},
};

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let path = args().as_bytes();

    let f = unsafe { syscalls::open(path.as_ptr(), path.len(), OpenOptions::CREATE) }
        .map_err(ProcessError::Sys)?;
    unsafe { syscalls::close(f) }
        .map_err(ProcessError::Sys)
        .map(|_| ())
}
