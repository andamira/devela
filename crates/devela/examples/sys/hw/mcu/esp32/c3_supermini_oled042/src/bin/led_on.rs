//
//! Turns on the board's GPIO8 LED using direct ESP32-C3 MMIO.
//

#![no_std]
#![no_main]

use devela::BoardSuperMiniOled042 as Board;

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

fn main() -> ! {
    unsafe {
        Board::LED.set_output_low() // Active-low LED: low means ON
        // Board::LED.set_output_high() // OFF
    }
    loop {}
}
