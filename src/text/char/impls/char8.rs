// devela::text::char::impls::char8

use super::*;
use crate::text::AsciiChar;

impl Char8 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[inline]
    #[must_use]
    const fn from_char_unchecked(c: char) -> Char8 {
        Char8(c as u32 as u8)
    }

    /* constants */

    /// The highest unicode scalar a `Char8` can represent, `'\u{FF}'`.
    pub const MAX: Char8 = Char8(0xFF);

    /* conversions */

    /// Converts an `AsciiChar` to `Char8`.
    #[inline]
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> Char8 {
        Char8(c as u8)
    }

    /// Converts a `Char7` to `Char8`.
    #[inline]
    #[must_use]
    pub const fn from_char7(c: Char7) -> Char8 {
        Char8(c.0.get())
    }
    /// Tries to convert a `Char16` to `Char8`.
    #[inline]
    pub const fn try_from_char16(c: Char16) -> Result<Char8> {
        if char_byte_len(c.to_u32()) == 1 {
            Ok(Char8(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char24` to `Char8`.
    #[inline]
    pub const fn try_from_char24(c: Char24) -> Result<Char8> {
        let c = c.to_u32();
        if char_byte_len(c) == 1 {
            Ok(Char8(c as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char32` to `Char8`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char8> {
        if char_byte_len(c.to_u32()) == 1 {
            Ok(Char8(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `char` to `Char8`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char8> {
        if char_byte_len(c as u32) == 1 {
            Ok(Char8(c as u32 as u8))
        } else {
            Err(CharConversionError(()))
        }
    }

    //

    /// Tries to convert this `Char8` to `AsciiChar`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[inline]
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if char_is_7bit(self.to_u32()) {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            if let Some(c) = AsciiChar::from_u8(self.0) {
                Ok(c)
            } else {
                unreachable![]
            }

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { AsciiChar::from_u8_unchecked(self.0) });
        } else {
            Err(CharConversionError(()))
        }
    }

    /// Tries to convert this `Char8` to `Char7`.
    #[inline]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char8(self)
    }
    /// Converts this `Char8` to `Char16`.
    #[inline]
    #[must_use]
    pub const fn to_char16(self) -> Char16 {
        Char16::from_char8(self)
    }
    /// Converts this `Char8` to `Char24`.
    #[inline]
    #[must_use]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char8(self)
    }
    /// Converts this `Char8` to `Char32`.
    #[inline]
    #[must_use]
    pub const fn to_char32(self) -> Char32 {
        Char32::from_char8(self)
    }
    /// Converts this `Char8` to `char`.
    #[inline]
    #[rustfmt::skip]
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0 as char
    }
    /// Converts this `Char8` to `u32`.
    #[inline]
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char8` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 2-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 2] {
        let c = self.0;
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c, 0],

            // from 0x0080 to 0x00FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080.. => {
                let y = 0b10_000000 | (0b0011_1111 & c);
                let x = 0b110_00000 | (c >> 6);
                [x, y]
            }
        }
    }

    //

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        char_is_noncharacter(self.0 as u32)
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    #[must_use]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }

    /// Checks if the value is within the ASCII range.
    #[inline]
    #[must_use]
    pub const fn is_ascii(self) -> bool {
        self.0 <= 0x7F
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char8 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char8 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}
