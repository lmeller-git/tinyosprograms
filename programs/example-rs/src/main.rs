#![no_std]
#![no_main]

// extern crate libtinyos;

use libtinyos::exit;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    exit(0);
}
