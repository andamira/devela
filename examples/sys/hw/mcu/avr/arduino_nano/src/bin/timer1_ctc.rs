// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/timer1_ctc.rs
//
//! Blinks the built-in LED using 16-bit Timer1 CTC polling.
//

#![no_std]
#![no_main]

use devela::{BoardArduinoNano, McuAtmega328p, set_panic_handler};

set_panic_handler! { loop }

// 16 MHz / 256 = 62.5 kHz.
// 31,250 counts = 500 ms.
// CTC counts 0..=OCR1A, therefore OCR1A = 31,249.
const TIMER1_TOP_500MS: u16 = (BoardArduinoNano::CPU_HZ / 256 / 2 - 1) as u16;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let timer = McuAtmega328p::TIMER_1;
    let led = BoardArduinoNano::LED;

    unsafe {
        led.set_output_low();
        timer.configure_ctc(TIMER1_TOP_500MS, 256);
    }

    loop {
        while !unsafe { timer.compare_a_match_pending() } {}
        unsafe {
            timer.clear_compare_a_match();
            led.toggle();
        }
    }
}
