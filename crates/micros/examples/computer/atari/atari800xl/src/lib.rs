//
//! Rust portion of an Atari 800XL DOS program, linked by LLVM-MOS SDK.
//

#![no_std]

use devela_micros::{ComputerAtari800XL as Atari, devela};

devela::set_panic_handler! { loop }

// LLVM-MOS SDK's Atari DOS runtime provides `putchar`, mapping newline
// to ATASCII 0x9B and routing output to the OS screen editor (`E:`).
// On this toolchain, C `int` is 16 bits.
unsafe extern "C" {
    fn putchar(ch: i16) -> i16;
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_atari_main() {
    for &byte in b"Hello from devela on the Atari 800XL!\n" {
        unsafe {
            putchar(byte as i16);
        }
    }

    // The OS copies COLOR4 to GTIA COLBK during vertical blank.
    unsafe {
        Atari::set_background_color(0x34);
    }
}
