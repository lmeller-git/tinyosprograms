#![no_std]
#![no_main]

use core::slice;

use alloc::string::{String, ToString};
use libtinyos::{
    serial_println,
    syscalls::{self, OpenOptions, STDERR_FILENO, STDOUT_FILENO},
};

extern crate alloc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: usize, argv: *const u8, envc: usize, envp: *const u8) -> ! {
    // TODO check if target is file, if so do not read

    let args = unsafe { slice::from_raw_parts(argv, argc) };
    let env = unsafe { slice::from_raw_parts(envp, envc) };

    let path = args
        .split(|b| *b == b' ')
        .find(|arg| !arg.starts_with(b"-"))
        .unwrap_or(b".");
    let cwd = env
        .split(|b| *b == b'\0')
        .find(|ele| ele.starts_with(b"CWD="))
        .unwrap_or(b"CWD=/");

    let str_path = str::from_utf8(path).unwrap();
    let str_cwd = str::from_utf8(cwd)
        .unwrap_or_default()
        .strip_prefix("CWD=")
        .unwrap_or("/");

    let path = if let Some(resolved) = resolve_relative(str_cwd, str_path) {
        resolved
    } else {
        str_path.to_string()
    };

    let mut buf = [0; 128];

    if let Ok(dir) = unsafe { syscalls::open(path.as_ptr(), path.len(), OpenOptions::READ) } {
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
    } else {
        let msg = b"could not read specified dir\n";
        _ = unsafe { syscalls::write(STDERR_FILENO, msg.as_ptr(), msg.len()) };
    }
    unsafe { syscalls::exit(0) }
}

fn resolve_relative(root: &str, append: &str) -> Option<String> {
    if append.starts_with('/') {
        return None;
    }

    let mut root = root.to_string();
    if root.ends_with('/') {
        root.pop();
    }
    let segments = append
        .split('/')
        .filter(|&segment| !segment.is_empty() && segment != ".");
    for segment in segments {
        if segment == ".." {
            if let Some((r, _)) = root.rsplit_once('/') {
                root.truncate(r.len());
            }
        } else {
            root.push('/');
            root.push_str(segment);
        }
    }

    Some(root)
}
