#![no_std]
#![no_main]

use alloc::borrow::ToOwned;
use libtinyos::{
    os::{args, env},
    path::Path,
    println,
    process::ProcessError,
    syscalls::{self, FStat, OpenOptions},
};

extern crate alloc;

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let Some(path) = args().map(|arg| arg.as_str()) else {
        return Ok(());
    };

    let mut path = Path::new(path).to_owned();

    if let Some(cwd) = env().and_then(|env| env.get("CWD")) {
        let mut base_path = Path::new(cwd).to_owned();
        base_path.push(path.as_ref());
        path = base_path
    }

    path.canonicalize();

    let f = unsafe {
        syscalls::open(
            path.as_str().as_ptr(),
            path.as_str().len(),
            OpenOptions::empty(),
        )
    }
    .map_err(ProcessError::Sys)?;

    let mut stat_buf = FStat::default();

    unsafe { syscalls::fstat(f, &mut stat_buf as *mut FStat) }.map_err(ProcessError::Sys)?;

    println!(
        "Stat for file {}:\nsize: {}\nt_create: {}\nt_modify: {}\npermissions: {}",
        path, stat_buf.size, stat_buf.t_create, stat_buf.t_mod, stat_buf.permissions
    );

    unsafe { syscalls::close(f) }
        .map_err(ProcessError::Sys)
        .map(|_| ())
}
