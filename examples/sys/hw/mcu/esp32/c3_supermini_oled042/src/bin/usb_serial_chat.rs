// devela/examples/sys/hw/mcu/esp/c3_supermini_oled042/src/bin/usb_serial_tx.rs
//
//!
//

#![no_std]
#![no_main]

use devela::{
    BoardSuperMiniOled042, EspUsbSerialJtag, esp32_c3_direct_boot, is, set_panic_handler,
};

set_panic_handler! { loop }
esp32_c3_direct_boot! { main }

const LINE_CAPACITY: usize = 32;

fn main() -> ! {
    let (serial, led) = (BoardSuperMiniOled042::USB_SERIAL, BoardSuperMiniOled042::LED);

    unsafe {
        led.set_output_high(); // Active-low LED: high means OFF

        // Flashing and console access share the USB port, so the firmware can start
        // before the terminal opens. Wait for input without consuming the first byte.
        while !serial.rx_ready() {}

        serial.write_bytes_blocking(
            b"devela esp32-c3 ready\r\n\
              commands: ping, led on, led off, status\r\n\
              > ",
        );

        let (mut line, mut len) = ([0_u8; LINE_CAPACITY], 0);

        loop {
            let byte = serial.read_byte_blocking();

            match byte {
                b'\r' | b'\n' => {
                    is! { len == 0, continue }
                    serial.write_bytes_blocking(b"\r\n");
                    handle_command(serial, &line[..len]);
                    serial.write_bytes_blocking(b"> ");
                    len = 0;
                }
                8 | 127 => {
                    if len != 0 {
                        len -= 1;
                        serial.write_bytes_blocking(b"\x08 \x08");
                    }
                }
                byte if len < line.len() => {
                    line[len] = byte;
                    len += 1;
                    serial.write_byte_blocking(byte); // Echo characters received
                }
                _ => {}
            }
        }
    }
}

unsafe fn handle_command(serial: EspUsbSerialJtag, command: &[u8]) {
    let led = BoardSuperMiniOled042::LED;
    unsafe {
        if command == b"ping" {
            serial.write_bytes_blocking(b"pong\r\n");
        } else if command == b"led on" {
            led.set_low();
            serial.write_bytes_blocking(b"ok\r\n");
        } else if command == b"led off" {
            led.set_high();
            serial.write_bytes_blocking(b"ok\r\n");
        } else if command == b"status" {
            if led.is_output_low() {
                serial.write_bytes_blocking(b"led=on\r\n");
            } else {
                serial.write_bytes_blocking(b"led=off\r\n");
            }
        } else if command == b"help" {
            serial.write_bytes_blocking(b"commands: ping, led on, led off, status\r\n");
        } else {
            serial.write_bytes_blocking(b"unknown command\r\n");
        }
    }
}
