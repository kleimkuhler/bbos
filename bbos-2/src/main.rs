#![feature(panic_implementation)] // use unstable `panic_implementation` attribute
#![no_std] // do not implicitly link the standard library
#![no_main] // do not use the normal entry point chain

extern crate bootloader_precompiled;

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello there!";

/// Overwrite linker entry point
///
/// lld looks for `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
