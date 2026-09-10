// devela/examples/hw/mcu/avr/atmega328p/nano_led/main.rs
//
//! Turns on the Arduino Nano built-in LED using direct ATmega328P MMIO.
//

#![no_std]
#![no_main]

use devela::{Atmega328p, PanicInfo};

const LED: u8 = 1 << 5; // Nano D13 → ATmega328P PB5.

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

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
