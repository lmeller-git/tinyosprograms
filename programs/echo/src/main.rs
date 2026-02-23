#![no_std]
#![no_main]

use libtinyos::{
    os::args,
    process::ProcessError,
    syscalls::{self, STDOUT_FILENO},
};

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let msg = args().map(|arg| arg.as_str()).unwrap_or_default();
    unsafe { syscalls::write(STDOUT_FILENO, msg.as_ptr(), msg.len()) }
        .map(|_| ())
        .map_err(ProcessError::Sys)
}
