// devela::text::char::impls::char32

use super::*;
use crate::{AsciiChar, Char, TextError::CharConversion, TextResult as Result};

impl char32 {
    /* constants */

    /// The highest unicode scalar a `char32` can represent, `'\u{10FFFF}'`.
    pub const MAX: char32 = char32(char::MAX);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: char32 = char32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts an `AsciiChar` to `char32`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> char32 {
        char32(c.as_char())
    }

    /// Converts a `char7` to `char32`.
    #[must_use]
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char7")))]
    pub const fn from_char7(c: char7) -> char32 {
        char32(c.to_char())
    }
    /// Converts a `char8` to `char32`.
    #[must_use]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    pub const fn from_char8(c: char8) -> char32 {
        char32(c.to_char())
    }
    /// Converts a `char16` to `char32`.
    #[must_use]
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char16")))]
    pub const fn from_char16(c: char16) -> char32 {
        char32(c.to_char())
    }
    /// Converts a `char24` to `char32`.
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    pub const fn from_char24(c: char24) -> char32 {
        char32(c.to_char())
    }
    /// Converts a `char` to `char32`.
    #[must_use]
    pub const fn from_char(c: char) -> char32 {
        char32(c)
    }

    //

    /// Tries to convert this `char32` to `AsciiChar`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if Char::is_7bit(self.to_u32()) {
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

    /// Tries to convert this `char32` to `char7`.
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char7")))]
    pub const fn try_to_char7(self) -> Result<char7> {
        char7::try_from_char32(self)
    }
    /// Tries to convert this `char32` to `char8`.
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    pub const fn try_to_char8(self) -> Result<char8> {
        char8::try_from_char32(self)
    }
    /// Tries to convert this `char32` to `char16`.
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char16")))]
    pub const fn try_to_char16(self) -> Result<char16> {
        char16::try_from_char32(self)
    }
    /// Converts this `char32` to `char24`.
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    pub const fn to_char24(self) -> char24 {
        char24::from_char32(self)
    }
    /// Converts this `char32` to `char`.
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0
    }
    /// Converts this `char32` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `char32` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[must_use]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        Char::to_utf8_bytes(self.0)
    }

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_uppercase(self) -> char32 {
        char32(char::to_ascii_uppercase(&self.0))
    }
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_lowercase(self) -> char32 {
        char32(char::to_ascii_lowercase(&self.0))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        Char::is_noncharacter(self.0 as u32)
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[must_use]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }

    /// Checks if the value is within the ASCII range.
    #[must_use]
    pub const fn is_ascii(self) -> bool {
        char::is_ascii(&self.0)
    }
}
