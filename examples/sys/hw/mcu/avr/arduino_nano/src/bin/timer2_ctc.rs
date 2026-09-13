// devela/examples/sys/hw/mcu/avr/arduino_nano/src/bin/timer1_ctc.rs
//
//! Blinks the built-in LED using Timer2 CTC polling.
//

#![no_std]
#![no_main]

use devela::{BoardArduinoNano, McuAtmega328p, set_panic_handler};

set_panic_handler! { loop }

// 16 MHz / 128 = 125 kHz.
// 125 timer counts = 1 ms.
// CTC counts 0..=OCR2A, therefore OCR2A = 124.
const TIMER2_TOP_1MS: u8 = (BoardArduinoNano::CPU_HZ / 128 / 1_000 - 1) as u8;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let (led, timer) = (BoardArduinoNano::LED, McuAtmega328p::TIMER_2);

    unsafe {
        led.set_output_low();
        timer.configure_ctc(TIMER2_TOP_1MS, 128);
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
