//
//! Blinks the Arduino Diecimila built-in LED using direct ATmega168 MMIO.
//
// 168 bytes

#![no_std]
#![no_main]

use devela::Arch;
use devela_micros::{BoardArduinoDiecimila as Board, devela};

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { Board::LED.set_output_low() } // Active-high LED

    loop {
        // Explicit 8/16-bit counters keep the busy-wait compact on 8-bit AVR.
        for _ in 0..8u8 {
            for _ in 0..62_500u16 {
                Arch::nop();
            }
        }
        unsafe { Board::LED.toggle() }
    }
}
