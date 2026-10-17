//
//! Blinks the MSP-EXP430G2 LaunchPad red LED (P1.0).
//

#![no_std]
#![no_main]

use devela::Ptr;
use devela_micros::devela;
use msp430_rt::entry;

devela::set_panic_handler! { loop }

const WDTCTL: *mut u16 = 0x0120 as *mut u16;
const P1IN: *const u8 = 0x0020 as *const u8;
const P1OUT: *mut u8 = 0x0021 as *mut u8;
const P1DIR: *mut u8 = 0x0022 as *mut u8;

#[entry]
fn main() -> ! {
    unsafe {
        // Stop watchdog.
        Ptr::write_volatile(WDTCTL, 0x5A80);

        // Set P1.0 as output.
        Ptr::write_volatile(P1DIR, Ptr::read_volatile(P1DIR) | 1);

        loop {
            // Toggle red LED.
            Ptr::write_volatile(P1OUT, Ptr::read_volatile(P1OUT) ^ 1);

            // Temporary busy-wait.
            for _ in 0..40_000u16 {
                let _ = Ptr::read_volatile(P1IN);
            }
        }
    }
}
