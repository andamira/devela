// devela::char::always_fns
//
//! `char` standalone functions.
//!
//! Always available for internal use.
//

#![allow(unused)]

/// Returns the number of bytes needed to store the given unicode scalar `code`,
/// utf-8 encoded in 2 bytes.
#[inline]
#[must_use]
pub const fn char_utf8_2bytes_len(code: [u8; 2]) -> u8 {
    1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
}

/// Returns the number of bytes needed to store the given unicode scalar `code`,
/// utf-8 encoded in 3 bytes.
#[inline]
#[must_use]
pub const fn char_utf8_3bytes_len(code: [u8; 3]) -> u8 {
    1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
}

/// Returns the number of bytes needed to store the given unicode scalar `code`,
/// utf-8 encoded in 4 bytes.
#[inline]
#[must_use]
pub const fn char_utf8_4bytes_len(code: [u8; 4]) -> u8 {
    1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((code[3] > 0) & (code[3] & 0b1100_0000 != 0b1000_0000)) as u8
}
