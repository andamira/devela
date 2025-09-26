// devela_base_core::text::char::namespace::u32
//
// TOC
// - methods over u32

use crate::{ASCII_TABLE, Char};

/// # Methods over `u32`.
#[rustfmt::skip]
impl Char<u32> {
    /* private helpers */

    /// Bitmask for extracting the 6-bit payload from a UTF-8 continuation byte (`10xxxxxx`).
    pub(crate) const CONT_MASK: u32 = 0b0011_1111;

    /* public methods */

    /// Returns the bytes required to store the given Unicode scalar code in a non-UTF encoding.
    ///
    /// This function does **not** determine the UTF-8 byte length.
    /// It assumes a simple encoding where values up to `0xFF` use 1 byte,
    /// `0x100..=0xFFFF` use 2 bytes, and anything larger uses 3 bytes.
    #[must_use]
    pub const fn len_bytes(self) -> usize {
        match self.0 {
            0x0000..=0x00FF => 1,
            0x0100..=0xFFFF => 2,
            _ => 3,
        }
    }

    /// Returns the number of bytes required to encode the given code as UTF-8.
    ///
    /// Returns `None` if it's not a valid Unicode scalar.
    #[must_use]
    pub const fn len_utf8(self) -> Option<usize> {
        if self.is_valid() { Some(self.len_utf8_unchecked()) } else { None }
    }

    /// Returns the UTF-8 byte length of the current code **without validation**.
    ///
    /// Assumes the code is a valid Unicode scalar.
    /// Use [`len_utf8`][Self::len_utf8] for a checked version.
    #[must_use]
    pub const fn len_utf8_unchecked(self) -> usize {
        match self.0 {
            0x00_0000..=0x00_007F => 1,
            0x00_0080..=0x00_07FF => 2,
            0x00_0800..=0x00_FFFF => 3,
            _ => 4,
        }
    }

    /// Checks if the code point is a valid Unicode scalar value.
    ///
    /// A valid Unicode scalar value is any integer in the ranges:
    /// - `U+0000` to `U+D7FF` (inclusive), or
    /// - `U+E000` to `U+10FFFF` (inclusive)
    ///
    /// This excludes surrogate code points (`U+D800` to `U+DFFF`), which are
    /// invalid in UTF-8 and cannot be represented as Unicode scalars.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// assert!(Char('A' as u32).is_valid()); // regular character
    /// assert!(Char(0x00).is_valid());       // NULL is valid
    /// assert!(Char(0x10FFFF).is_valid());   // maximum Unicode scalar
    /// // invalid:
    /// assert!(!Char(0xD800).is_valid());    // high surrogate
    /// assert!(!Char(0xDFFF).is_valid());    // low surrogate
    /// assert!(!Char(0x110000).is_valid());  // above max Unicode
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn is_valid(self) -> bool {
        (self.0 <= 0xD7FF) || (self.0 >= 0xE000 && self.0 <= 0x10_FFFF)
    }

    /// Checks if the given code is a 7-bit ASCII character (U+0000..=U+007F).
    #[must_use]
    #[inline(always)]
    pub const fn is_7bit(self) -> bool { self.0 <= 0x7F }

    /// Returns `true` if the given unicode scalar code is a [noncharacter][0].
    ///
    /// Note that this also checks against reserved, potential non-characters.
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    // MAYBE: make a version that checks for surrogates
    pub const fn is_noncharacter(self) -> bool {
        // sub-block of 32 non-characters:
        (self.0 >= 0xFDD0 && self.0 <= 0xFDEF)
            // 2× non-characters at the end of each plane:
            || (self.0 >= 0xFFFE && (self.0 & 0xFF) == 0xFE)
            || (self.0 >= 0xFFFE && (self.0 & 0xFF) == 0xFF)
            // unallocated range (16 potential non-characters):
            || (self.0 >= 0x2FE0 && self.0 <= 0x2FEF)
    }

    /// Returns `true` if the given unicode scalar code is a [surrogate code point][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate(self) -> bool { matches!(self.0, 0xD800..=0xDFFF) }

    /// Returns `true` if the given unicode scalar code is a [leading surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#high_surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate_high(self) -> bool { matches!(self.0, 0xD800..=0xDBFF) }

    /// Returns `true` if the given unicode scalar code is a [trailing surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#low_surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate_low(self) -> bool { matches!(self.0, 0xDC00..=0xDFFF) }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn to_ascii_str(self) -> &'static str {
        if self.is_7bit() { ASCII_TABLE[self.0 as usize] } else { "" }
    }
    /// Returns the ASCII representation as a `&'static str`, or panics if non-ASCII.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn to_ascii_str_unchecked(self) -> &'static str { ASCII_TABLE[self.0 as usize] }

    /// Converts the Unicode scalar code to a UTF-8 encoded byte sequence.
    ///
    /// Returns `None` if the code is not a valid Unicode scalar.
    /// The result is always a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`char::encode_utf8`].
    pub const fn to_utf8_bytes(self) -> Option<[u8; 4]> {
        if self.is_valid() { Some(self.to_utf8_bytes_unchecked()) } else { None }
    }

    /// Converts the Unicode scalar code to a UTF-8 encoded byte sequence **without validation**.
    ///
    /// Assumes the code is a valid Unicode scalar.
    /// Always returns a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`Char::to_utf8_bytes`] for a checked version.
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes_unchecked(self) -> [u8; 4] {
        let code = self.0;
        match code {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [code as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (Char::<u8>::CONT_MASK & (code as u8));
                let x = 0b110_00000 | ((code >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (Char::<u8>::CONT_MASK & (code as u8));
                let y = 0b10_000000 | ((code >> 6) & Char::<u32>::CONT_MASK) as u8;
                let x = 0b1110_0000 | ((code >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (Char::<u8>::CONT_MASK & (code as u8));
                let z = 0b10_000000 | ((code >> 6) & Char::<u32>::CONT_MASK) as u8;
                let y = 0b10_000000 | ((code >> 12) & Char::<u32>::CONT_MASK) as u8;
                let x = 0b11110_000 | ((code >> 18) as u8);
                [x, y, z, w]
            }
        }
    }
}
