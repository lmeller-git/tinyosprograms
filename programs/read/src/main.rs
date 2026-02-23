#![no_std]
#![no_main]
#![feature(str_from_raw_parts)]

use libtinyos::{
    os::args,
    process::ProcessError,
    syscalls::{self, STDIN_FILENO, STDOUT_FILENO},
};

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let arg = args().unwrap().as_str();
    let fd = arg.parse().unwrap_or(STDIN_FILENO);

    let mut buf = [0; 128];

    while let Ok(n) = unsafe { syscalls::read(fd, buf.as_mut_ptr(), buf.len(), -1_isize as usize) }
        && n >= 0
    {
        unsafe { syscalls::write(STDOUT_FILENO, buf[..n as usize].as_ptr(), n as usize) }.unwrap();
    }
    Ok(())
}
