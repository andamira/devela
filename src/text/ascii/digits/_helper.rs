// devela/src/text/ascii/digits/_helper.rs

use crate::AsciiLut;

pub(crate) const fn ascii_digit_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'Z' => Some(byte - b'A' + 10),
        b'a'..=b'z' => Some(byte - b'a' + 10),
        _ => None,
    }
}

pub(crate) const fn ascii_digit_upper(value: u8) -> Option<u8> {
    if value < 36 { Some(AsciiLut::DIGITS_BASE36[value as usize]) } else { None }
}

pub(crate) const fn ascii_digit_lower(value: u8) -> Option<u8> {
    if value < 36 {
        Some(AsciiLut::DIGITS_BASE36_LOWER[value as usize])
    } else {
        None
    }
}
