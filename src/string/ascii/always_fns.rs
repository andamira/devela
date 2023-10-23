// devela::ascii::always_fns
//
//! `ascii` standalone functions.
//!
//! Always available for internal use.
//

#![allow(unused)]

use crate::num::count_digits;

/// Converts a one-digit number to the corresponding `1` ASCII digit.
///
/// # Panics
/// This function panics in debug if the given number is > 9.
#[inline]
#[must_use]
pub const fn ascii_1digit(n: u8) -> u8 {
    debug_assert![n <= 9];
    n + b'0'
}

/// Converts a two-digit number to the corresponding `2` ASCII digits.
///
/// # Panics
/// This function panics in debug if the given number is > 99.
#[inline]
#[must_use]
pub const fn ascii_2digit(n: u8) -> [u8; 2] {
    debug_assert![n <= 99];
    [ascii_calc_digit_u8(n, 10), ascii_calc_digit_u8(n, 1)]
}

/// Converts a three-digit number to the corresponding `3` ASCII digits.
///
/// # Panics
/// This function panics in debug if the given number is > 999.
#[inline]
#[must_use]
pub const fn ascii_3digit(n: u16) -> [u8; 3] {
    debug_assert![n <= 999];
    [
        ascii_calc_digit_u16(n, 100),
        ascii_calc_digit_u16(n, 10),
        ascii_calc_digit_u16(n, 1),
    ]
}

/// Converts a four-digit number to the corresponding `4` ASCII digits.
///
/// # Panics
/// This function panics in debug if the given number is > 9999.
#[inline]
#[must_use]
pub const fn ascii_4digit(n: u16) -> [u8; 4] {
    debug_assert![n <= 9999];
    [
        ascii_calc_digit_u16(n, 1000),
        ascii_calc_digit_u16(n, 100),
        ascii_calc_digit_u16(n, 10),
        ascii_calc_digit_u16(n, 1),
    ]
}

// -----------------------------------------------------------------------------

/// Calculates the ASCII byte of a digit.
#[inline]
#[must_use]
pub const fn ascii_calc_digit(n: usize, divisor: usize) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
#[inline]
#[must_use]
pub(crate) const fn ascii_calc_digit_u8(n: u8, divisor: u8) -> u8 {
    (n / divisor % 10) + b'0'
}
#[inline]
#[must_use]
pub(crate) const fn ascii_calc_digit_u16(n: u16, divisor: u16) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
#[inline]
#[must_use]
pub(crate) const fn ascii_calc_digit_u32(n: u32, divisor: u32) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
#[inline]
#[must_use]
pub(crate) const fn ascii_calc_digit_u64(n: u64, divisor: u64) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
#[inline]
#[must_use]
pub(crate) const fn ascii_calc_digit_u128(n: u128, divisor: u128) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}

// -----------------------------------------------------------------------------

/// Converts a `u8` into a byte array of `3` ascii digits, padded with zeros.
///
/// You can trim the leading zeros with
/// [`slice_trim_leading_bytes`][crate::data::slice_trim_leading_bytes].
#[inline]
#[must_use]
pub const fn ascii_u8_digits(n: u8) -> [u8; 3] {
    [
        //                           321
        //                           255 u8::MAX
        ascii_calc_digit_u8(n, 100),
        ascii_calc_digit_u8(n, 10),
        ascii_calc_digit_u8(n, 1),
    ]
}
