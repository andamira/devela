//
//! Blink the blue active-high LED on a nice!nano, without USB/BLE stack.
//

#![no_std]
#![no_main]

use devela::Arch;
use devela_micros::{BoardNiceNano as Board, devela};

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { Board::LED.set_output_low() } // Active-high; start off.
    loop {
        unsafe { Board::LED.set_high() }
        delay();
        unsafe { Board::LED.set_low() }
        delay();
    }
}

fn delay() {
    for _ in 0..5_000_000 {
        Arch::relax();
    }
}
