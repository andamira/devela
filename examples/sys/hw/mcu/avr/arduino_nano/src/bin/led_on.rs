// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/led_on.rs
//
//! Turns on the Arduino Nano built-in LED using direct ATmega328P MMIO.
//

#![no_std]
#![no_main]

use devela::{BoardArduinoNano, set_panic_handler};

set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { BoardArduinoNano::LED.set_output_high() }

    loop {}
}
