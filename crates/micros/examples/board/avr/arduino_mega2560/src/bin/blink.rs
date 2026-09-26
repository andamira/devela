//
//! Blinks the Arduino Mega 2560 built-in LED using direct ATmega2560 MMIO.
//
// 296 bytes

#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use devela::asm;
use devela_micros::{BoardArduinoMega2560 as Board, devela};

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { Board::LED.set_output_low() } // Active-high LED

    // Explicit 8/16-bit counters keep the busy-wait compact on 8-bit AVR.
    loop {
        for _ in 0..8u8 {
            for _ in 0..62_500u16 {
                unsafe { asm!("nop") };
            }
        }
        unsafe { Board::LED.toggle() }
    }
}
