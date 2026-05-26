#![no_std]
#![no_main]

use alloc::borrow::ToOwned;
use libtinyos::{
    eprintln,
    os::{args, env},
    path::Path,
    println,
    process::ProcessError,
    syscalls::{self, FStat, NodePermissions, OpenOptions, PermUpdateStrategy},
};

extern crate alloc;

// args are +-rwx --help

#[unsafe(no_mangle)]
pub fn main() -> Result<(), ProcessError> {
    let Some(options) = args().map(|arg| arg.as_str().split_ascii_whitespace()) else {
        return Ok(());
    };

    let mut path = None;
    let mut to_add = NodePermissions::empty();
    let mut to_remove = NodePermissions::empty();

    for opt in options {
        match opt {
            _ if opt.starts_with(['+', '-']) => {
                let is_add = opt.starts_with('+');

                for perm_char in opt[1..].chars() {
                    let flag = match perm_char {
                        'r' => NodePermissions::R,
                        'w' => NodePermissions::W,
                        'x' => NodePermissions::X,
                        _ => {
                            eprintln!("wrong arg in chmod: {}{}", &opt[0..1], perm_char);
                            return Err(ProcessError::Sys(syscalls::SysErrCode::InvalidArg));
                        }
                    };

                    if is_add {
                        to_add.insert(flag);
                    } else {
                        to_remove.insert(flag);
                    }
                }
            }
            "--help" => {
                println!(
                    "chmod\nUsage:\nchmod +-rwx file\n flags:\n\t+rwx: add the permissions\n\t-rwx: remove the permission\n\t--help: display this message"
                );
                return Ok(());
            }
            // assume this is the filename
            _ => _ = path.replace(Path::new(opt)),
        }
    }

    if path.is_none() {
        eprintln!("No file path was provided to chmod");
        return Err(ProcessError::Sys(syscalls::SysErrCode::InvalidArg));
    }

    let mut path = path.unwrap().to_owned();

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

    if to_add.is_empty() && to_remove.is_empty() {
        eprintln!("No permission modification was supplied.");
        return Ok(());
    }

    let strategy = if !to_add.is_empty() && to_remove.is_empty() {
        PermUpdateStrategy::OR
    } else if to_add.is_empty() && !to_remove.is_empty() {
        PermUpdateStrategy::AND
    } else {
        PermUpdateStrategy::OVERWRITE
    };

    let final_mask = match strategy {
        PermUpdateStrategy::OR => to_add,
        PermUpdateStrategy::AND => !to_remove,
        PermUpdateStrategy::OVERWRITE => {
            let mut stat_buf = FStat::default();
            unsafe { syscalls::fstat(f, &mut stat_buf as *mut FStat) }
                .map_err(ProcessError::Sys)?;

            let mut perms = stat_buf.permissions;
            perms.insert(to_add);
            perms.remove(to_remove);
            perms
        }
    };

    unsafe { syscalls::update_perms(f, final_mask, strategy) }.map_err(ProcessError::Sys)?;

    unsafe { syscalls::close(f) }
        .map_err(ProcessError::Sys)
        .map(|_| ())
}
