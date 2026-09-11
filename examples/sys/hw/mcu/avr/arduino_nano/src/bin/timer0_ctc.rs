// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/timer0_ctc.rs
//
//! Blinks the built-in LED using Timer0 CTC polling.
//

#![no_std]
#![no_main]

use devela::{BoardArduinoNano, McuAtmega328p, set_panic_handler};

set_panic_handler! { loop }

// 16 MHz / 64 = 250 kHz.
// 250 timer counts = 1 ms.
// CTC counts 0..=OCR0A, therefore OCR0A = 249.
const TIMER0_TOP_1MS: u8 = (BoardArduinoNano::CPU_HZ / 64 / 1_000 - 1) as u8;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let timer = McuAtmega328p::TIMER_0;
    let led = BoardArduinoNano::LED;

    unsafe {
        led.set_output_low();
        timer.configure_ctc(TIMER0_TOP_1MS, 64);
    }

    let mut millis = 0_u16;

    loop {
        while !unsafe { timer.compare_a_match_pending() } {}
        unsafe { timer.clear_compare_a_match() };

        millis += 1;
        if millis == 500 {
            unsafe { led.toggle() };
            millis = 0;
        }
    }
}
