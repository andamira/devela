// devela/src/data/codec/symbol/ean/_helper.rs

use crate::{is, lets, whilst};

/// Normal left/right guard pattern.
pub(super) const GUARD: u8 = 0b101;

/// Centre guard pattern.
pub(super) const CENTER: u8 = 0b01010;

/// Number set A, from leftmost to rightmost module.
#[rustfmt::skip]
pub(super) const SET_A: [u8; 10] = [
    0b0001101, // 0
    0b0011001, // 1
    0b0010011, // 2
    0b0111101, // 3
    0b0100011, // 4
    0b0110001, // 5
    0b0101111, // 6
    0b0111011, // 7
    0b0110111, // 8
    0b0001011, // 9
];

pub(super) const fn set_b(digit: u8) -> u8 {
    reverse7(SET_A[digit as usize] ^ 0x7F)
}
pub(super) const fn set_c(digit: u8) -> u8 {
    SET_A[digit as usize] ^ 0x7F
}
pub(super) const fn reverse7(pattern: u8) -> u8 {
    pattern.reverse_bits() >> 1
}
pub(super) const fn decode_a(pattern: u8) -> Option<u8> {
    whilst! { digit in 0..SET_A.len(); {
        is! { SET_A[digit] == pattern, return Some(digit as u8) }
    }}
    None
}
pub(super) const fn decode_b(pattern: u8) -> Option<u8> {
    decode_a(reverse7(pattern) ^ 0x7F)
}
pub(super) const fn decode_c(pattern: u8) -> Option<u8> {
    decode_a(pattern ^ 0x7F)
}
pub(super) const fn ean_check_digit(data: &[u8]) -> Option<u8> {
    lets! { mut sum = 0_usize, mut i = data.len(), mut weight = 3_usize }
    while i != 0 {
        i -= 1;
        let digit = data[i];
        is! { digit > 9, return None }
        sum += digit as usize * weight;
        weight = 4 - weight; // 3 ↔ 1
    }
    Some(((10 - sum % 10) % 10) as u8)
}
