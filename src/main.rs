#![feature(panic_handler)] // use unstable `panic_implementation` attribute
#![no_std] // do not implicitly link the standard library
#![no_main] // do not use the normal entry point chain

extern crate bootloader_precompiled;

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Overwrite linker entry point
///
/// lld looks for `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
