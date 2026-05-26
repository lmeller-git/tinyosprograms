#![no_std]
#![no_main]

use alloc::{borrow::ToOwned, string::String, vec::Vec};
use libtinyos::{
    eprintln,
    os::{args, env},
    path::Path,
    process::ProcessError,
    syscalls::{self, FStat, NodeType, OpenOptions, STDOUT_FILENO},
};

extern crate alloc;

#[derive(Debug, noshell::Parser)]
struct MyArgs {
    path: String,
}

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let path: Vec<&str> = args()
        .map(|arg| arg.as_str().split_ascii_whitespace().collect())
        .unwrap_or_default();

    let args = MyArgs::parse_from(&path);

    let mut path = Path::new(&args.path).to_owned();
    if let Some(cwd) = env().and_then(|env| env.get("CWD")) {
        let mut base_path = Path::new(cwd).to_owned();
        base_path.push(path.as_ref());
        path = base_path
    }
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

    let mut stat_buf = FStat::default();

    unsafe { syscalls::fstat(dir, &mut stat_buf as *mut FStat) }.map_err(ProcessError::Sys)?;

    if stat_buf.node_type != NodeType::DIR {
        eprintln!(
            "Tried to call ls on a non-dir node: {}. Use cat for that.",
            path
        );
        return Err(ProcessError::Sys(syscalls::SysErrCode::OpDenied));
    }

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
