//
//! Turns on the Arduino Mega 2560 built-in LED using direct ATmega2560 MMIO.
//
// 138 bytes

#![no_std]
#![no_main]

use devela_micros::{BoardArduinoMega2560 as Board, devela};

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { Board::LED.set_output_high() }
    loop {}
}
