#![no_std]
#![no_main]

use libtinyos::{
    process::ProcessError,
    syscalls::{self, STDERR_FILENO},
};

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let msg = b"currently not implemented";
    _ = unsafe { syscalls::write(STDERR_FILENO, msg.as_ptr(), msg.len()) };
    Ok(())
}
