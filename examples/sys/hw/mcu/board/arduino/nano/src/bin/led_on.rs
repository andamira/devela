// devela/examples/sys/hw/mcu/board/arduino/nano/src/bin/led_on.rs
//
//! Turns on the Arduino Nano built-in LED using direct ATmega328P MMIO.
//

#![no_std]
#![no_main]

use devela::{Atmega328p, set_panic_handler};

set_panic_handler! { loop }

const LED: u8 = 1 << 5; // Nano D13 → ATmega328P PB5.

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let port = Atmega328p::PORT_B;

    unsafe {
        let ddr = port.ddr_reg();
        ddr.write(ddr.read() | LED);

        let out = port.port_reg();
        out.write(out.read() | LED);
    }

    loop {}
}
