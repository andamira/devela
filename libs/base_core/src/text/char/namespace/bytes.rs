// devela_base_core::text::char::namespace::bytes

use crate::{Char, unwrap};

/// # Methods over `u8`.
#[rustfmt::skip]
impl Char<u8> {
    /// Returns the expected UTF-8 byte length based on the given first byte.
    ///
    /// This function does **not** validate UTF-8 but determines how many bytes
    /// a valid sequence **should** occupy based on the leading byte.
    ///
    /// - ASCII (0xxxxxxx) → 1 byte
    /// - 2-byte (110xxxxx) → 2 bytes
    /// - 3-byte (1110xxxx) → 3 bytes
    /// - 4-byte (11110xxx) → 4 bytes
    ///
    /// ### Caveat
    /// - If used on malformed UTF-8, it may suggest a length longer than the actual valid sequence.
    /// - Always use in conjunction with proper UTF-8 validation if handling untrusted input.
    ///
    /// For a stricter check, see [`utf8_len_checked`][Self::utf8_len_checked].
    pub const fn utf8_len(self) -> usize {
        match self.0 {
            0x00..=0x7F => 1, // ASCII (1 byte)
            0xC0..=0xDF => 2, // 2-byte sequence
            0xE0..=0xEF => 3, // 3-byte sequence
            0xF0..=0xF7 => 4, // 4-byte sequence
            _ => 0,           // Invalid leading byte
        }
    }

    /// Returns the expected UTF-8 byte length based on the given first byte, or `None` if invalid.
    ///
    /// This function detects invalid UTF-8 leading bytes and ensures
    /// they fall within valid Unicode scalar boundaries.
    ///
    /// - Returns `Some(len)` for valid leading bytes.
    /// - Returns `None` for invalid first bytes that cannot start a UTF-8 sequence.
    ///
    /// ### Stricter Handling
    /// - Rejects overlong sequences (C0, C1).
    /// - Enforces the valid UTF-8 upper bound (max `F4`).
    /// - Safer for processing untrusted input where malformed UTF-8 must be detected.
    ///
    /// For a simpler length-only function, see [`utf8_len`][Self::utf8_len].
    #[must_use]
    pub const fn utf8_len_checked(self) -> Option<usize> {
        match self.0 {
            0x00..=0x7F => Some(1),
            0xC2..=0xDF => Some(2),
            0xE0..=0xEF => Some(3),
            0xF0..=0xF4 => Some(4),
            _ => None,
        }
    }
}

/// # Methods over `u8` slice.
#[rustfmt::skip]
impl Char<&[u8]> {
    /// Decodes a UTF-8 code point from the given byte slice, starting at `index`.
    ///
    /// Returns `Some((code, len))` if the input is a valid UTF-8 sequence
    /// and the decoded code point is a valid Unicode scalar.
    ///
    /// Returns `None` if:
    /// - The index is out of bounds.
    /// - The bytes do not form a valid UTF-8 sequence.
    /// - The decoded value is not a valid Unicode scalar.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// assert_eq!(Char("Ħ".as_bytes()).utf8_bytes_to_code(0), Some((u32::from('Ħ'), 2)));
    ///
    /// let invalid = b"\x80"; // Invalid leading byte
    /// assert_eq!(Char(invalid).utf8_bytes_to_code(0), None);
    /// ```
    #[must_use]
    pub const fn utf8_bytes_to_code(self, index: usize) -> Option<(u32, usize)> {
        let bytes = self.0;
        if index >= bytes.len() { return None; } // out of bounds
        let first = bytes[index];
        let len = unwrap![some? Char(first).utf8_len_checked()]; // invalid leading byte?
        if index + len > bytes.len() { return None; } // not enough bytes
        Some(Char(bytes).utf8_bytes_to_code_unchecked(index))
    }

    /// Decodes a UTF-8 code point from `bytes`, starting at `index`.
    ///
    /// Returns `(code, len)`, where `code` is the decoded Unicode scalar,
    /// and `len` is the number of bytes consumed.
    ///
    /// Assumes `bytes[index..]` contains a valid UTF-8 sequence.
    #[must_use]
    pub const fn utf8_bytes_to_code_unchecked(self, index: usize) -> (u32, usize) {
        let bytes = self.0;
        let first = bytes[index];
        match first {
            0x00..=0x7F => (first as u32, 1), // 1-byte ASCII
            0xC2..=0xDF => ( // 2-byte UTF-8
                (((first as u32 & 0b0001_1111) << 6) |
                 (bytes[index + 1] as u32 & 0b0011_1111)),
                2
            ),
            0xE0..=0xEF => ( // 3-byte UTF-8
                (((first as u32 & 0b0000_1111) << 12) |
                 ((bytes[index + 1] as u32 & 0b0011_1111) << 6) |
                 (bytes[index + 2] as u32 & 0b0011_1111)),
                3
            ),
            0xF0..=0xF4 => ( // 4-byte UTF-8
                (((first as u32 & 0b0000_0111) << 18) |
                 ((bytes[index + 1] as u32 & 0b0011_1111) << 12) |
                 ((bytes[index + 2] as u32 & 0b0011_1111) << 6) |
                 (bytes[index + 3] as u32 & 0b0011_1111)),
                4
            ),
            _ => (0xFFFD, 1), // Invalid sequence → Return Unicode replacement char (U+FFFD)
        }
    }
}
/// Methods over a byte array, referring to a byte slice.
impl<const N: usize> Char<&[u8; N]> {
    /// See [utf8_bytes_to_code()][Char::<&[u8]>::utf8_bytes_to_code].
    #[inline(always)] // eliminate the wrapper entirely
    pub const fn utf8_bytes_to_code(self, index: usize) -> Option<(u32, usize)> {
        let bytes: &[u8] = self.0;
        Char(bytes).utf8_bytes_to_code(index)
    }
}
