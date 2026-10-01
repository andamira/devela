//
//! Blinks the LCD backlight to verify ESP32-S3 startup and T-Deck board wiring.
//

#![no_std]
#![no_main]

use devela::Arch;
use devela_micros::{BoardLilygoTDeckS3 as Board, devela};
use xtensa_lx_rt::entry;

devela::set_panic_handler! { loop }
devela::esp32_s3_startup!();

fn delay() {
    for _ in 0..8_000_000 {
        Arch::relax();
    }
}

#[entry]
fn main() -> ! {
    unsafe {
        // Hold the pulse-controlled backlight input low while power comes up.
        Board::LCD_BL.set_output_low();
        Board::enable_peripherals();
        delay();

        loop {
            // After a sufficiently long low period, high starts at maximum backlight brightness.
            Board::LCD_BL.set_output_high();
            delay();

            // A long low period switches the backlight off and resets its level.
            Board::LCD_BL.set_output_low();
            delay();
        }
    }
}
