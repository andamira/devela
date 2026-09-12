// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/usart_chat.rs
//
//! Runs a small interactive command console over USART0.
//

#![no_std]
#![no_main]

use devela::{AvrUsart, BoardArduinoNano, is, set_panic_handler};

set_panic_handler! { loop }

const LINE_CAPACITY: usize = 32;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let (uart, led) = (BoardArduinoNano::USART, BoardArduinoNano::LED);

    unsafe {
        led.set_output_low();
        uart.configure_rx_tx_8n1(BoardArduinoNano::CPU_HZ, 9_600);

        uart.write_bytes_blocking(
            b"devela nano ready\r\n\
              commands: ping, led on, led off, status\r\n\
              > ",
        );

        let (mut line, mut len) = ([0_u8; LINE_CAPACITY], 0);

        loop {
            let byte = uart.read_byte_blocking();
            match byte {
                b'\r' | b'\n' => {
                    is! { len == 0, continue } // Treat CRLF as one line ending
                    uart.write_bytes_blocking(b"\r\n");
                    handle_command(uart, &line[..len]);
                    uart.write_bytes_blocking(b"> ");
                    len = 0;
                }
                8 | 127 => {
                    if len != 0 {
                        len -= 1;
                        uart.write_bytes_blocking(b"\x08 \x08");
                    }
                }
                byte if len < line.len() => {
                    line[len] = byte;
                    len += 1;
                    // uart.write_byte_blocking(byte); // Echo characters received from picocom
                }
                _ => {
                    // Line full: ignore further bytes until Enter/backspace.
                }
            }
        }
    }
}

unsafe fn handle_command(uart: AvrUsart, command: &[u8]) {
    let led = BoardArduinoNano::LED;
    unsafe {
        if command == b"ping" {
            uart.write_bytes_blocking(b"pong\r\n");
        } else if command == b"led on" {
            led.set_high();
            uart.write_bytes_blocking(b"ok\r\n");
        } else if command == b"led off" {
            led.set_low();
            uart.write_bytes_blocking(b"ok\r\n");
        } else if command == b"status" {
            if led.is_high() {
                uart.write_bytes_blocking(b"led=on\r\n");
            } else {
                uart.write_bytes_blocking(b"led=off\r\n");
            }
        } else {
            uart.write_bytes_blocking(b"unknown command\r\n");
        }
    }
}
