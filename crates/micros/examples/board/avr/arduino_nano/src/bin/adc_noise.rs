//
//! Reports weak ADC-noise bytes and their extraction time over USART0.
//!
//! Each line contains an extracted byte and the Timer1 ticks required to
//! produce it, both in hexadecimal. Timer1 runs at 4 µs per tick.
//!
//! Suggested experiment: compare A0 with a loose wire attached, touch A0
//! with a finger, connect A0 to GND, then leave it floating. Observe both
//! the extracted values and the extraction times. A sufficiently stable
//! input may exhaust the bounded extractor and print `!!`.
//!
//! This is an interactive bring-up experiment, not an entropy-quality test.
//
// 1294 bytes

#![no_std]
#![no_main]

use devela::{Radix, RandTry};
use devela_micros::{BoardArduinoNano as Board, McuAtmega328p as Mcu, devela};

devela::set_panic_handler! { loop }

// 16 MHz / 64 = 250 kHz → 4 µs/tick.
const PAUSE_TICKS: u16 = 50_000; // 200 ms (keep each pause below Timer1's ~262 ms wrap period)
const PAUSE_CHUNKS: u8 = 5; // ~1 s between reports

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let (uart, timer) = (Board::USART, Mcu::TIMER_1);

    unsafe {
        uart.configure_tx_8n1(Board::CPU_HZ, 9_600);
        timer.configure_normal(64);
    }

    let mut noise = unsafe { Board::MCU.adc().prepare_noise(0, 128) };

    unsafe {
        uart.write_bytes_blocking(b"ADC0 weak noise; byte / Timer1 ticks (4 us)\r\n");
    }

    let mut hex = [0; 4];
    loop {
        let start = unsafe { timer.counter() };
        let value = noise.rand_try_next_u8();
        let ticks = unsafe { timer.counter() }.wrapping_sub(start);

        unsafe {
            match value {
                Ok(value) => {
                    Radix::<16>::HEX_LOWER.encode_to_slice(&[value], &mut hex).unwrap();
                    uart.write_bytes_blocking(&hex[..2]);
                }
                Err(_) => {
                    uart.write_bytes_blocking(b"!!");
                }
            }
            uart.write_bytes_blocking(b"  ");
            Radix::<16>::HEX_LOWER.encode_to_slice(&ticks.to_be_bytes(), &mut hex).unwrap();
            uart.write_bytes_blocking(&hex);
            uart.write_bytes_blocking(b"\r\n");
        }

        // Slow the stream so changes in the physical setup can be observed live.
        for _ in 0..PAUSE_CHUNKS {
            let start = unsafe { timer.counter() };
            while unsafe { timer.counter() }.wrapping_sub(start) < PAUSE_TICKS {}
        }
    }
}
