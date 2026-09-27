//
//! Blinks the LCD backlight to verify ESP32-C6 direct boot and GPIO routing.
//
// 320 bytes

#![no_std]
#![no_main]

use core::hint::spin_loop;
use devela_micros::{BoardWaveshareC6TouchLcd147 as Board, devela};

devela::set_panic_handler! { loop }
devela::esp32_c6_direct_boot! { main }

fn main() -> ! {
    let backlight = Board::LCD_BL;

    unsafe {
        backlight.set_output_low();

        loop {
            for _ in 0..8_000_000 {
                spin_loop();
            }
            backlight.toggle();
        }
    }
}
