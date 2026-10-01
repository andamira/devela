//
//! Blinks the LCD backlight to verify ESP32-S3 startup and board GPIO routing.
//

#![no_std]
#![no_main]

use devela::Arch;
use devela_micros::{BoardLilygoTDisplayS3 as Board, devela};
use xtensa_lx_rt::entry;

devela::set_panic_handler! { loop }
devela::esp32_s3_startup!();

#[entry]
fn main() -> ! {
    unsafe {
        // Keep the display dark before powering its peripherals
        Board::LCD_BL.set_output_low();
        Board::enable_peripherals();

        loop {
            for _ in 0..8_000_000 {
                Arch::relax();
            }
            Board::LCD_BL.toggle();
        }
    }
}
