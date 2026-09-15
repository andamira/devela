// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/led_on.rs
//
//! Turns on the Arduino Nano built-in LED using direct ATmega328P MMIO.
//

#![no_std]
#![no_main]

use devela::BoardArduinoNano as Board;

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { Board::LED.set_output_high() }
    loop {}
}
