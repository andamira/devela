//
//! Blinks the SuperMini's built-in GPIO8 LED using direct ESP32-C3 MMIO.
//
// 272 bytes

#![no_std]
#![no_main]

use devela::asm;
use devela_micros::{BoardSuperMiniOled042 as Board, devela};

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

fn main() -> ! {
    unsafe { Board::LED.set_output_high() } // Active-low LED

    loop {
        for _ in 0..500_000 {
            unsafe { asm!("nop") };
        }
        unsafe { Board::LED.set_output_low() }

        for _ in 0..500_000 {
            unsafe { asm!("nop") };
        }
        unsafe { Board::LED.set_output_high() }
    }
}
