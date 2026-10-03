//
//! Minimal compilation example for the AVR ATtiny4.
//

#![no_std]
#![no_main]

use devela_micros::devela;

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    loop {}
}
