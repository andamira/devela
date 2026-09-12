// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/timer1_pwm.rs
//
//! Fades an external LED using Timer1 fast PWM on OC1A.
//

#![no_std]
#![no_main]

use devela::{AvrPin, BoardArduinoNano, McuAtmega328p, is, set_panic_handler};

set_panic_handler! { loop }

// 16 MHz / 8 / (1999 + 1) = 1 kHz PWM.
const PWM_TOP: u16 = 1_999;

const DUTY_MIN: u16 = 20;
const DUTY_MAX: u16 = 1_980;
const DUTY_STEP: u16 = 2;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let timer = McuAtmega328p::TIMER_1;
    let pwm = AvrPin::new(McuAtmega328p::PORT_B, 1); // PB1 / OC1A / Nano D9

    unsafe {
        pwm.set_output_low();
        timer.configure_fast_pwm(PWM_TOP, 8);
        timer.set_compare_a(DUTY_MIN);
        timer.enable_pwm_a_non_inverting();
    }

    let (mut duty, mut rising) = (DUTY_MIN, true);

    loop {
        // In fast PWM, TOV1 marks the end of each PWM cycle.
        while !unsafe { timer.overflow_pending() } {}
        unsafe { timer.clear_overflow() };

        if rising {
            duty += DUTY_STEP;
            is! { duty == DUTY_MAX, rising = false }
        } else {
            duty -= DUTY_STEP;
            is! { duty == DUTY_MIN, rising = true }
        }

        // OCR1A is double-buffered in PWM mode,
        // so the new compare value takes effect cleanly at the next BOTTOM.
        unsafe { timer.set_compare_a(duty) };
    }
}
