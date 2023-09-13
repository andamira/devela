// devela::ascii
//
//! ASCII strings and characters, extends [`core::ascii`].
//

/// Converts a one-digit number to the corresponding `1` ASCII digit.
///
/// # Panics
/// This function panics in debug if the given number is > 9.
#[inline]
pub const fn ascii_d1(n: u8) -> u8 {
    debug_assert![n <= 9];
    n + b'0'
}

/// Converts a two-digit number to the corresponding `2` ASCII digits.
///
/// # Panics
/// This function panics in debug if the given number is > 99.
#[inline]
pub const fn ascii_d2(n: u8) -> [u8; 2] {
    debug_assert![n <= 99];
    [calc_digit(n as usize, 10), calc_digit(n as usize, 1)]
}

/// Converts a three-digit number to the corresponding `3` ASCII digits.
///
/// # Panics
/// This function panics in debug if the given number is > 999.
#[inline]
pub const fn ascii_d3(n: u16) -> [u8; 3] {
    debug_assert![n <= 999];
    [
        calc_digit(n as usize, 100),
        calc_digit(n as usize, 10),
        calc_digit(n as usize, 1),
    ]
}

/// Converts a four-digit number to the corresponding `4` ASCII digits.
///
/// # Panics
/// This function panics in debug if the given number is > 9999.
#[inline]
pub const fn ascii_d4(n: u16) -> [u8; 4] {
    debug_assert![n <= 9999];
    [
        calc_digit(n as usize, 1000),
        calc_digit(n as usize, 100),
        calc_digit(n as usize, 10),
        calc_digit(n as usize, 1),
    ]
}

// -----------------------------------------------------------------------------

/// Calculates the ascii byte of a digit
#[inline]
const fn calc_digit(n: usize, divisor: usize) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
// #[inline]
// const fn calc_digit_u8(n: u8, divisor: u8) -> u8 {
//     (n / divisor % 10) + b'0'
// }
// #[inline]
// const fn calc_digit_u16(n: u16, divisor: u16) -> u8 {
//     (n / divisor % 10) as u8 + b'0'
// }
#[inline]
const fn calc_digit_u32(n: u32, divisor: u32) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
// a maximum of 20 digits
#[inline]
const fn calc_digit_u64(n: u64, divisor: u64) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}
#[inline]
const fn calc_digit_u128(n: u128, divisor: u128) -> u8 {
    (n / divisor % 10) as u8 + b'0'
}

// -----------------------------------------------------------------------------

/// Converts a `u8` into a byte array of `3` ascii digits, padded with zeros.
#[inline]
pub const fn u8_to_ascii(n: u8) -> [u8; 3] {
    [
        //                     321
        //                     255 u8::MAX
        calc_digit(n as usize, 100),
        calc_digit(n as usize, 10),
        calc_digit(n as usize, 1),
    ]
}

/// Converts a `u16` into a byte array of `5` ascii digits, padded with zeros.
#[inline]
pub const fn u16_to_ascii(n: u16) -> [u8; 5] {
    [
        //                     54321
        //                     65535 u16::MAX
        calc_digit(n as usize, 10000),
        calc_digit(n as usize, 1000),
        calc_digit(n as usize, 100),
        calc_digit(n as usize, 10),
        calc_digit(n as usize, 1),
    ]
}

/// Converts a `u32` into a byte array of `10` ascii digits, padded with zeros.
#[inline]
pub const fn u32_to_ascii(n: u32) -> [u8; 10] {
    [
        //                0987654321
        //                4294967295 u32::MAX
        calc_digit_u32(n, 1000000000), // 10 digits
        calc_digit_u32(n, 100000000),
        calc_digit_u32(n, 10000000),
        calc_digit_u32(n, 1000000),
        calc_digit_u32(n, 100000),
        calc_digit_u32(n, 10000), // 5 digits
        calc_digit_u32(n, 1000),
        calc_digit_u32(n, 100),
        calc_digit_u32(n, 10),
        calc_digit_u32(n, 1),
    ]
}

/// Converts a `u64` into a byte array of `20` ascii digits, padded with zeros.
pub const fn u64_to_ascii(n: u64) -> [u8; 20] {
    [
        //                0987654321_987654321
        //                18446744073709551615 u64::MAX
        calc_digit_u64(n, 10000000000000000000), // 20 digits
        calc_digit_u64(n, 1000000000000000000),
        calc_digit_u64(n, 100000000000000000),
        calc_digit_u64(n, 10000000000000000),
        calc_digit_u64(n, 1000000000000000),
        calc_digit_u64(n, 100000000000000),
        calc_digit_u64(n, 10000000000000),
        calc_digit_u64(n, 1000000000000),
        calc_digit_u64(n, 100000000000),
        calc_digit_u64(n, 10000000000),
        calc_digit_u64(n, 1000000000), // 10 digits
        calc_digit_u64(n, 100000000),
        calc_digit_u64(n, 10000000),
        calc_digit_u64(n, 1000000),
        calc_digit_u64(n, 100000),
        calc_digit_u64(n, 10000),
        calc_digit_u64(n, 1000),
        calc_digit_u64(n, 100),
        calc_digit_u64(n, 10),
        calc_digit_u64(n, 1),
    ]
}

/// Converts a `u128` into a byte array of `39` ascii digits, padded with zeros.
pub const fn u128_to_ascii(n: u128) -> [u8; 39] {
    [
        //                 987654321_987654321_987654321_987654321
        //                 340282366920938463463374607431768211455 u128::MAX
        calc_digit_u128(n, 100000000000000000000000000000000000000), // 39 digits
        calc_digit_u128(n, 10000000000000000000000000000000000000),
        calc_digit_u128(n, 1000000000000000000000000000000000000),
        calc_digit_u128(n, 100000000000000000000000000000000000),
        calc_digit_u128(n, 10000000000000000000000000000000000),
        calc_digit_u128(n, 1000000000000000000000000000000000),
        calc_digit_u128(n, 100000000000000000000000000000000),
        calc_digit_u128(n, 10000000000000000000000000000000),
        calc_digit_u128(n, 1000000000000000000000000000000),
        calc_digit_u128(n, 100000000000000000000000000000), // 30 digits
        calc_digit_u128(n, 10000000000000000000000000000),
        calc_digit_u128(n, 1000000000000000000000000000),
        calc_digit_u128(n, 100000000000000000000000000),
        calc_digit_u128(n, 10000000000000000000000000),
        calc_digit_u128(n, 1000000000000000000000000),
        calc_digit_u128(n, 100000000000000000000000),
        calc_digit_u128(n, 10000000000000000000000),
        calc_digit_u128(n, 1000000000000000000000),
        calc_digit_u128(n, 100000000000000000000),
        calc_digit_u128(n, 10000000000000000000), // 20 digits
        calc_digit_u128(n, 1000000000000000000),
        calc_digit_u128(n, 100000000000000000),
        calc_digit_u128(n, 10000000000000000),
        calc_digit_u128(n, 1000000000000000),
        calc_digit_u128(n, 100000000000000),
        calc_digit_u128(n, 10000000000000),
        calc_digit_u128(n, 1000000000000),
        calc_digit_u128(n, 100000000000),
        calc_digit_u128(n, 10000000000),
        calc_digit_u128(n, 1000000000), // 10 digits
        calc_digit_u128(n, 100000000),
        calc_digit_u128(n, 10000000),
        calc_digit_u128(n, 1000000),
        calc_digit_u128(n, 100000),
        calc_digit_u128(n, 10000),
        calc_digit_u128(n, 1000),
        calc_digit_u128(n, 100),
        calc_digit_u128(n, 10),
        calc_digit_u128(n, 1),
    ]
}
