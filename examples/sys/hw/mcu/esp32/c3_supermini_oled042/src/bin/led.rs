// devela/examples/sys/hw/mcu/esp/c3_supermini_oled042/src/bin/led.rs
//
//! Turns on the board's GPIO8 LED using direct ESP32-C3 MMIO.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042, McuEsp32C3, esp32_c3_direct_boot, set_panic_handler};

esp32_c3_direct_boot! { main }
set_panic_handler! { loop }

fn main() -> ! {
    let led = BoardSuperMiniOled042::LED_MASK;

    unsafe {
        // Set the intended level before enabling the output driver.
        McuEsp32C3::GPIO_OUT_W1TC.write(led); // low → LED ON
        McuEsp32C3::GPIO_ENABLE_W1TS.write(led);
    }

    loop {}
}
