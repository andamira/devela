#![cfg_attr(target_arch = "avr", no_std)]
#![cfg_attr(target_arch = "avr", no_main)]

#[cfg(target_arch = "avr")]
use devela_micros::{AvrUsartTx, BoardArduinoNano as Board};
#[cfg(target_arch = "avr")]
use sentry_log::exercise_diag;

#[cfg(target_arch = "avr")]
devela::set_panic_handler! { loop }

#[cfg(target_arch = "avr")]
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut out = unsafe { AvrUsartTx::configure_8n1(Board::USART, Board::CPU_HZ, 9_600) };

    let _ = exercise_diag(&mut out);

    loop {}
}

#[cfg(not(target_arch = "avr"))]
fn main() {}
