// devela::text::char::impls::char32

use super::*;
use crate::text::{
    char_is_7bit, char_is_noncharacter, char_to_utf8_bytes, AsciiChar, TextError::CharConversion,
    TextResult as Result,
};

impl Char32 {
    /* constants */

    /// The highest unicode scalar a `Char32` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char32 = Char32(char::MAX);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char32 = Char32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts an `AsciiChar` to `Char32`.
    #[inline]
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> Char32 {
        Char32(c.as_char())
    }

    /// Converts a `Char7` to `Char32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn from_char7(c: Char7) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char8` to `Char32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn from_char8(c: Char8) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char16` to `Char32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn from_char16(c: Char16) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char24` to `Char32`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn from_char24(c: Char24) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `char` to `Char32`.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Char32 {
        Char32(c)
    }

    //

    /// Tries to convert this `Char32` to `AsciiChar`.
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

    /// Tries to convert this `Char32` to `Char7`.
    #[inline]
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char32(self)
    }
    /// Tries to convert this `Char32` to `Char8`.
    #[inline]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn try_to_char8(self) -> Result<Char8> {
        Char8::try_from_char32(self)
    }
    /// Tries to convert this `Char32` to `Char16`.
    #[inline]
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn try_to_char16(self) -> Result<Char16> {
        Char16::try_from_char32(self)
    }
    /// Converts this `Char32` to `Char24`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char32(self)
    }
    /// Converts this `Char32` to `char`.
    #[inline]
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0
    }
    /// Converts this `Char32` to `u32`.
    #[inline]
    #[must_use]
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
    pub const fn to_ascii_uppercase(self) -> Char32 {
        Char32(char::to_ascii_uppercase(&self.0))
    }
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char32 {
        Char32(char::to_ascii_lowercase(&self.0))
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
