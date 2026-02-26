#![no_std]
#![no_main]
#![feature(str_from_raw_parts)]

use libtinyos::{
    os::{args, env},
    process::ProcessError,
    syscalls::{self, FileDescriptor, STDOUT_FILENO},
};

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let env = env().unwrap().as_str();
    let fd = args()
        .map(|args| args.as_str())
        .unwrap_or_default()
        .parse::<FileDescriptor>()
        .unwrap_or(STDOUT_FILENO);

    unsafe { syscalls::write(fd, env.as_ptr(), env.len()) }.unwrap();

    Ok(())
}
