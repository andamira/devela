//
//! Blinks the built-in LED from the Timer1-backed monotonic time source.
//

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use devela::Arch;
use devela_micros::{Atmega328pTimer1Clock as Clock, BoardArduinoNano as Board, devela};

devela::set_panic_handler! { loop }

// ATmega328P Timer/Counter1 Overflow vector.
#[unsafe(export_name = "__vector_13")]
pub extern "avr-interrupt" fn timer1_overflow() {
    // SAFETY: this is the Timer1 overflow ISR and accounts exactly one overflow.
    unsafe { Clock::on_overflow_interrupt() };
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Keep global interrupts off until both the peripheral
    // and vector-backed software extension are ready.
    Arch::disable_interrupts();

    let clock = unsafe {
        Board::LED.set_output_low();
        Clock::start(Board::CPU_HZ, Clock::DEFAULT_PRESCALER)
    };

    unsafe { Arch::enable_interrupts() };

    let mut previous = clock.now();
    loop {
        let now = clock.now();
        if now - previous >= 125_000 {
            unsafe { Board::LED.toggle() };
            previous = now;
        }
    }
}
