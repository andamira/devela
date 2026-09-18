//
//! Tests ESP32-C3 UART0 transmission and reception.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042 as Board, McuEsp32C3 as Mcu, is};
use devela_micros::devela;

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

const BAUD: u32 = 115_200;

fn main() -> ! {
    let (uart, led) = (Board::UART0, Board::LED);

    unsafe {
        led.set_output_high(); // Active-low LED: start OFF.

        Mcu::prepare_uart0_default();
        uart.configure_8n1_xtal(Mcu::XTAL_HZ, BAUD);

        // Seeing this verifies the TX path.
        uart.write_bytes_blocking(b"\r\ndevela UART0 echo ready\r\n");

        loop {
            let byte = uart.read_byte_blocking();

            // Typing any character toggles the LED, so RX can be tested
            is! { led.is_output_high(), led.set_low(), led.set_high() }

            // With both directions connected, received bytes are echoed.
            uart.write_byte_blocking(byte);
        }
    }
}
