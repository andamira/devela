// devela/examples/sys/hw/mcu/esp/c3_supermini_oled042/src/bin/led.rs
//
//! Turns on the board's GPIO8 LED using direct ESP32-C3 MMIO.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042, esp32_c3_direct_boot, set_panic_handler};

set_panic_handler! { loop }
esp32_c3_direct_boot! { main }

fn main() -> ! {
    unsafe {
        BoardSuperMiniOled042::LED.set_output_low() // Active-low LED: low means ON
        // BoardSuperMiniOled042::LED.set_output_high() // OFF
    }
    loop {}
}
