#![feature(panic_implementation)] // use unstable `panic_implementation` attribute
#![no_std] // do not implicitly link the standard library
#![no_main] // do not use the normal entry point chain

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Overwrite linker entry point
///
/// macOS does not support statically linked binaries, so we have to link the libSystem library.
/// The entry point is called main
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
