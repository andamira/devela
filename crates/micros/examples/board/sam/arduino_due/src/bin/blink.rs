//
//! Blinks the Arduino Due built-in LED using direct SAM3X8E MMIO.
//!
//! The busy-wait delay is intentionally uncalibrated and depends on the
//! active CPU clock.
//
// 416 bytes

#![no_std]
#![no_main]

use devela::spin_loop;
use devela_micros::{BoardArduinoDue as Board, devela};

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { Board::LED.set_output_low() } // Active-high LED

    loop {
        for _ in 0..500_000 {
            spin_loop();
        }
        unsafe { Board::LED.set_high() }

        for _ in 0..500_000 {
            spin_loop();
        }
        unsafe { Board::LED.set_low() }
    }
}
