//
//! Minimal MSP430 runtime compilation probe.
//

#![no_std]
#![no_main]

use devela_micros::devela;
use msp430_rt::entry;

devela::set_panic_handler! { loop }

#[entry]
fn main() -> ! {
    loop {}
}
