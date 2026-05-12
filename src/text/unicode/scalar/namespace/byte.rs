// devela::text::unicode::scalar::namespace::byte

use crate::{Char, is};

/// # Methods over `u8`.
#[rustfmt::skip]
impl Char<u8> {
    /* private helpers */

    /// Bitmask for extracting the 6-bit payload from a UTF-8 continuation byte (`10xxxxxx`).
    pub(crate) const CONT_MASK: u8 = 0b0011_1111;

    // https://tools.ietf.org/html/rfc3629
    // https://github.com/rust-lang/rust/blob/master/library/core/src/str/validations.rs
    pub(crate) const UTF8_CHAR_LEN: &[u8; 256] = &[
        // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0 0x00..=0x7F => 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
        0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C 0xC2..=0xDF => 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E 0xE0..=0xEF => 3,
        4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F 0xF0..=0xF4 => 4,
    ];

    /* public methods */

    /// Returns the expected UTF-8 byte length based on the given first byte, or `None` if invalid.
    ///
    /// LUT based (256-byte array).
    #[must_use]
    pub const fn len_utf8(self) -> Option<usize> {
        let width = self.len_utf8_unchecked();
        is![width == 0, None, Some(width)]
    }

    /// Returns the expected UTF-8 byte length based on the given first byte, or `0` if invalid.
    ///
    /// LUT based (256-byte array).
    #[must_use]
    pub const fn len_utf8_unchecked(self) -> usize {
        Self::UTF8_CHAR_LEN[self.0 as usize] as usize
    }

    /// Returns the expected UTF-8 byte length based on the given first byte, or `None` if invalid.
    ///
    /// Match based, for when memory accesses are more expensive than branches.
    #[must_use]
    pub const fn len_utf8_match(self) -> Option<usize> {
        match self.0 { // same logic as Self::UTF8_CHAR_LEN
            0x00..=0x7F => Some(1),
            0xC2..=0xDF => Some(2), // skips invalid C0, C1
            0xE0..=0xEF => Some(3),
            0xF0..=0xF4 => Some(4), // skips invalid 0xF5..0x=F7
            _ => None, // invalid leading byte
        }
    }

    /// Returns the expected UTF-8 byte length based on the given first byte.
    ///
    /// Match based, for when memory accesses are more expensive than branches.
    ///
    /// This function does **not** validate UTF-8 but determines how many bytes
    /// a valid sequence **should** occupy based on the leading byte.
    ///
    /// ### Caveat
    /// - If used on malformed UTF-8, it may suggest a length longer than the actual valid sequence.
    /// - Always use in conjunction with proper UTF-8 validation if handling untrusted input.
    #[must_use]
    pub const fn len_utf8_match_naive(self) -> usize {
        match self.0 {
            0x00..=0x7F => 1, // 1-byte ASCII
            0xC0..=0xDF => 2, // 2-byte sequence
            0xE0..=0xEF => 3, // 3-byte sequence
            0xF0..=0xF7 => 4, // 4-byte sequence
            _ => 0,           // invalid leading byte
        }
    }

    /// Returns `true` if this byte is a valid starting point for a UTF-8 sequence.
    ///
    /// This checks if the byte is not a UTF-8 continuation byte (i.e., it's either
    /// an ASCII character or a valid leading byte of a multi-byte sequence).
    #[must_use] #[inline(always)]
    pub const fn is_utf8_boundary(self) -> bool {
        // Equivalent to: b < 128 || b >= 192 (== not a continuation byte (0b10xxxxxx))
        (self.0 as i8) >= -0x40
    }
    /// Returns `true` if this byte is a UTF-8 continuation byte.
    ///
    /// Continuation bytes have the bit pattern `10xxxxxx`.
    #[must_use] #[inline(always)]
    pub const fn is_utf8_continuation(self) -> bool { !self.is_utf8_boundary() }
}
