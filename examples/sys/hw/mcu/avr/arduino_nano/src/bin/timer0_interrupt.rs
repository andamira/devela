// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/timer0_interrupt.rs
//
//! Blinks the built-in LED using a Timer0 compare interrupt.
//

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use devela::{Arch, BoardArduinoNano, McuAtmega328p, set_panic_handler};

set_panic_handler! { loop }

// 16 MHz / 64 = 250 kHz.
// 250 timer counts = 1 ms.
// CTC counts 0..=OCR0A, therefore OCR0A = 249.
const TIMER0_TOP_1MS: u8 = (BoardArduinoNano::CPU_HZ / 64 / 1_000 - 1) as u8;

// ATmega328P Timer/Counter0 Compare Match A vector.
#[unsafe(export_name = "__vector_14")]
pub extern "avr-interrupt" fn timer0_compare_a() {
    static mut MILLIS: u8 = 0;

    // SAFETY: this state is accessed only by this blocking ISR.
    unsafe {
        MILLIS += 1;

        if MILLIS == 250 {
            BoardArduinoNano::LED.toggle();
            MILLIS = 0;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let timer = McuAtmega328p::TIMER_0;

    Arch::disable_interrupts();

    unsafe {
        BoardArduinoNano::LED.set_output_low();

        timer.configure_ctc(TIMER0_TOP_1MS, 64);
        timer.enable_compare_a_interrupt();

        // Enable globally only after the handler and peripheral are ready.
        Arch::enable_interrupts();
    }

    loop {}
}
