// devela::text::char::impls::char32

use super::*;
use crate::text::{
    char_is_7bit, char_is_noncharacter, char_to_utf8_bytes, AsciiChar, TextError::CharConversion,
    TextResult as Result,
};

impl CharU32 {
    /* constants */

    /// The highest unicode scalar a `CharU32` can represent, `'\u{10FFFF}'`.
    pub const MAX: CharU32 = CharU32(char::MAX);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: CharU32 = CharU32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts an `AsciiChar` to `CharU32`.
    #[inline]
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> CharU32 {
        CharU32(c.as_char())
    }

    /// Converts a `CharU7` to `CharU32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn from_char_u7(c: CharU7) -> CharU32 {
        CharU32(c.to_char())
    }
    /// Converts a `CharU8` to `CharU32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn from_char_u8(c: CharU8) -> CharU32 {
        CharU32(c.to_char())
    }
    /// Converts a `CharU16` to `CharU32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn from_char_u16(c: CharU16) -> CharU32 {
        CharU32(c.to_char())
    }
    /// Converts a `CharU24` to `CharU32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn from_char_u24(c: CharU24) -> CharU32 {
        CharU32(c.to_char())
    }
    /// Converts a `char` to `CharU32`.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> CharU32 {
        CharU32(c)
    }

    //

    /// Tries to convert this `CharU32` to `AsciiChar`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[inline]
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if char_is_7bit(self.to_u32()) {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            if let Some(c) = AsciiChar::from_u8(self.0 as u8) {
                return Ok(c);
            } else {
                unreachable![]
            }

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { AsciiChar::from_u8_unchecked(self.0 as u8) });
        }
        Err(CharConversion)
    }

    /// Tries to convert this `CharU32` to `CharU7`.
    #[inline]
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn try_to_char_u7(self) -> Result<CharU7> {
        CharU7::try_from_char_u32(self)
    }
    /// Tries to convert this `CharU32` to `CharU8`.
    #[inline]
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn try_to_char_u8(self) -> Result<CharU8> {
        CharU8::try_from_char_u32(self)
    }
    /// Tries to convert this `CharU32` to `CharU16`.
    #[inline]
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn try_to_char_u16(self) -> Result<CharU16> {
        CharU16::try_from_char_u32(self)
    }
    /// Converts this `CharU32` to `CharU24`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn to_char_u24(self) -> CharU24 {
        CharU24::from_char_u32(self)
    }
    /// Converts this `CharU32` to `char`.
    #[inline]
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0
    }
    /// Converts this `CharU32` to `u32`.
    #[inline]
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `CharU32` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[must_use]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        char_to_utf8_bytes(self.0)
    }

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> CharU32 {
        CharU32(char::to_ascii_uppercase(&self.0))
    }
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> CharU32 {
        CharU32(char::to_ascii_lowercase(&self.0))
    }

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
        char::is_ascii(&self.0)
    }
}
