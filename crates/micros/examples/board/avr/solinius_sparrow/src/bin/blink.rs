//
//! Blinks an external LED on Sparrow PD2 using direct ATmega1284 MMIO.
//
// 204 bytes

#![no_std]
#![no_main]

use devela::Arch;
use devela_micros::{AvrPin, McuAtmega1284 as Mcu, devela};

devela::set_panic_handler! { loop }

const LED: AvrPin = AvrPin::new(Mcu::PORT_D, 2);

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { LED.set_output_low() } // Active-high external LED.

    loop {
        // Scaled from the 16 MHz AVR blink examples for 14.7456 MHz.
        for _ in 0..8u8 {
            for _ in 0..57_600u16 {
                Arch::nop();
            }
        }
        unsafe { LED.toggle() }
    }
}
