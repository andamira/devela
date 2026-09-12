// devela/examples/sys/hw/mcu/esp/c3_supermini_oled042/src/bin/usb_serial_tx.rs
//
//! Turns on the board's GPIO8 LED using direct ESP32-C3 MMIO.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042, esp32_c3_direct_boot, set_panic_handler};

set_panic_handler! { loop }
esp32_c3_direct_boot! { main }

fn main() -> ! {
    let serial = BoardSuperMiniOled042::USB_SERIAL;

    unsafe {
        // Flashing and console access share the USB port, so the firmware can start
        // before the terminal opens. Wait for input without consuming the first byte.
        while !serial.rx_ready() {}

        serial.write_bytes_blocking(b"hello from devela\r\n");
    }
    loop {}
}
