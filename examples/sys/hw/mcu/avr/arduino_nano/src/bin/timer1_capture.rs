// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/timer1_capture.rs
//
//! Demonstrates Timer1 input capture using the ICP1 pin.
//

#![no_std]
#![no_main]

use devela::{AvrPin, BoardArduinoNano, McuAtmega328p, set_panic_handler};

set_panic_handler! { loop }

// 16 MHz / 64 = 250 kHz.
// 50,000 ticks = 200 ms.
const TRIGGER_AT: u16 = 50_000;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let timer = McuAtmega328p::TIMER_1;
    let icp1 = AvrPin::new(McuAtmega328p::PORT_B, 0); // PB0 / Nano D8
    let led = BoardArduinoNano::LED;

    unsafe {
        led.set_output_low();
        icp1.set_output_low();

        timer.configure_normal(64);
        timer.set_capture_rising_edge();
        timer.clear_input_capture();
    }

    loop {
        unsafe {
            timer.set_counter(0);
            icp1.set_low();
            timer.clear_input_capture();
        }

        while unsafe { timer.counter() < TRIGGER_AT } {}

        // Rising transition on PB0/ICP1 causes hardware to snapshot TCNT1.
        unsafe { icp1.set_high() };

        while !unsafe { timer.input_capture_pending() } {}

        let captured = unsafe { timer.capture_value() };
        unsafe { timer.clear_input_capture() };

        // The captured timestamp should be at or just after TRIGGER_AT.
        if captured >= TRIGGER_AT {
            unsafe { led.toggle() };
        }
    }
}
