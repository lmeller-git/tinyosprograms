#![no_std]
#![no_main]

extern crate alloc;
use libtinyos::{exit, println};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    println!("hello from userland");
    exit(0);
}
