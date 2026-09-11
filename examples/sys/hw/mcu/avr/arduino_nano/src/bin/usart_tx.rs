// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/usart_tx.rs
//
//! Sends `hello from devela` over serial.
//

#![no_std]
#![no_main]

use devela::{ArduinoNano, set_panic_handler};

set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let uart = ArduinoNano::USART;

    unsafe {
        uart.configure_tx_8n1(ArduinoNano::CPU_HZ, 9_600);
        uart.write_bytes_blocking(b"hello from devela\r\n");
    }

    loop {}
}
