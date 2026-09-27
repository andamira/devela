//
//! Drives a TM1638 "LED & KEY" module from an Arduino Nano.
//!
//! Wiring:
//! - TM1638 STB -> Nano D8
//! - TM1638 CLK -> Nano D9
//! - TM1638 DIO -> Nano D10
//! - VCC -> +5V
//! - GND -> GND
//!
//! Displays 0..7 and mirrors the eight buttons onto the eight LEDs.
//!
//! NOTE: If the buttons don't work, it may be necessary
//! to add a pull-up 10kΩ resistor between DIO and +5V.
//!
//
// 2186 bytes

#![no_std]
#![no_main]

use devela::{
    BoardArduinoNano as Board, Tm1638, Tm1638AvrBus, Tm1638Brightness, Tm1638Frame, Tm1638LedKey8,
};
use devela_micros::devela;

devela::set_panic_handler! { loop }

const DIGITS: [u8; 8] = [
    0x3F, // 0
    0x06, // 1
    0x5B, // 2
    0x4F, // 3
    0x66, // 4
    0x6D, // 5
    0x7D, // 6
    0x07, // 7
];

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let tm = Tm1638::new();

    // SAFETY:
    // D8, D9 and D10 are dedicated exclusively to this bus for the
    // remainder of the program.
    let mut bus = unsafe { Tm1638AvrBus::new_unchecked(Board::D8, Board::D9, Board::D10) };

    let mut frame = Tm1638Frame::new();

    for position in 0..8 {
        Tm1638LedKey8::set_digit(&mut frame, position, DIGITS[position as usize]);
    }

    tm.init(&mut bus, Tm1638Brightness::MAX).unwrap();
    tm.write_frame(&mut bus, &frame).unwrap();

    loop {
        let raw = tm.read_keys(&mut bus).unwrap();
        let keys = Tm1638LedKey8::decode_keys(raw);

        for position in 0..8 {
            let pressed = keys & (1 << position) != 0;
            Tm1638LedKey8::set_led(&mut frame, position, pressed);
        }

        tm.write_frame(&mut bus, &frame).unwrap();
    }
}
