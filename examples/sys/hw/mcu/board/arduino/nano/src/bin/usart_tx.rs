// devela/examples/sys/hw/mcu/board/arduino/nano/src/bin/usart_tx.rs
//
//! Sends `hello from devela` over serial.
//

#![no_std]
#![no_main]

use devela::{ArduinoNano, Atmega328p};

set_panic_handler! { loop }

const BAUD: u32 = 9_600;
const UBRR: u16 = (ArduinoNano::CPU_HZ / (16 * BAUD) - 1) as u16;

const TXEN0: u8 = 1 << 3;
const UDRE0: u8 = 1 << 5;
const UCSZ00: u8 = 1 << 1;
const UCSZ01: u8 = 1 << 2;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let uart = Atmega328p::USART_0;

    unsafe {
        uart.ucsra_reg().write(0); // Normal asynchronous speed: U2X0 = 0

        // Baud-rate divisor. Write high byte before low byte.
        uart.ubrrh_reg().write((UBRR >> 8) as u8);
        uart.ubrrl_reg().write(UBRR as u8);

        uart.ucsrb_reg().write(TXEN0); // Enable transmitter only
        uart.ucsrc_reg().write(UCSZ01 | UCSZ00); // Asynchronous, no parity, 1 stop bit, 8 data bits

        for &byte in b"hello from devela\r\n" {
            while uart.ucsra_reg().read() & UDRE0 == 0 {}
            uart.udr_reg().write(byte);
        }
    }

    loop {}
}
