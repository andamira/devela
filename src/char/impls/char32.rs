// devela::ascii::char::char8

use super::*;
#[cfg(feature = "ascii")]
use crate::ascii::AsciiChar;

impl Char32 {
    /* constants */

    /// The highest unicode scalar a `Char32` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char32 = Char32(char::MAX);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char32 = Char32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts an `AsciiChar` to `Char32`.
    #[inline]
    #[cfg(feature = "ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
    pub const fn from_ascii_char(c: AsciiChar) -> Char32 {
        Char32(c.as_char())
    }

    /// Converts a `Char7` to `Char32`.
    #[inline]
    pub const fn from_char7(c: Char7) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char8` to `Char32`.
    #[inline]
    pub const fn from_char8(c: Char8) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char16` to `Char32`.
    #[inline]
    pub const fn from_char16(c: Char16) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char24` to `Char32`.
    #[inline]
    pub const fn from_char24(c: Char24) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `char` to `Char32`.
    #[inline]
    pub const fn from_char(c: char) -> Char32 {
        Char32(c)
    }

    //

    /// Tries to convert this `Char32` to `AsciiChar`.
    #[inline]
    #[cfg(feature = "ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if char_is_7bit(self.to_u32()) {
            #[cfg(not(feature = "unsafe_char"))]
            if let Some(c) = AsciiChar::from_u8(self.0 as u8) {
                Ok(c)
            } else {
                unreachable![]
            }

            #[cfg(feature = "unsafe_char")]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { AsciiChar::from_u8_unchecked(self.0 as u8) });
        } else {
            Err(CharConversionError(()))
        }
    }

    /// Tries to convert this `Char32` to `Char7`.
    #[inline]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char32(self)
    }
    /// Tries to convert this `Char32` to `Char8`.
    #[inline]
    pub const fn try_to_char8(self) -> Result<Char8> {
        Char8::try_from_char32(self)
    }
    /// Tries to convert this `Char32` to `Char16`.
    #[inline]
    pub const fn try_to_char16(self) -> Result<Char16> {
        Char16::try_from_char32(self)
    }
    /// Converts this `Char32` to `Char24`.
    #[inline]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char32(self)
    }
    /// Converts this `Char32` to `char`.
    #[inline]
    pub const fn to_char(self) -> char {
        self.0
    }
    /// Converts this `Char32` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char32` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        let c = self.0 as u32;
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (0b0011_1111 & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (0b0011_1111 & (c as u8));
                let z = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let y = 0b10_000000 | ((c >> 12) & 0b0011_1111) as u8;
                let x = 0b11110_000 | ((c >> 18) as u8);
                [x, y, z, w]
            }
        }
    }

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char32 {
        Char32(char::to_ascii_uppercase(&self.0))
    }
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char32 {
        Char32(char::to_ascii_lowercase(&self.0))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        char_is_noncharacter(self.0 as u32)
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        char::is_ascii(&self.0)
    }
}
