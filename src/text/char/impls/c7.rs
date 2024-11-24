// devela::text::char::impls::char7

use super::*;
use crate::{
    text::char::NonExtremeU8, AsciiChar, Char, TextError::CharConversion, TextResult as Result,
};

impl char7 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn from_char_unchecked(c: char) -> char7 {
        char7::new_unchecked(c as u32 as u8)
    }

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn new_unchecked(value: u8) -> char7 {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        if let Some(c) = NonExtremeU8::new(value) {
            char7(c)
        } else {
            unreachable![]
        }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            char7(NonExtremeU8::new_unchecked(value))
        }
    }

    /* constants */

    /// The highest unicode scalar a `char7` can represent, `'\u{7F}'`.
    pub const MAX: char7 = char7::new_unchecked(0x7F);

    /* conversions */

    /// Converts an `AsciiChar` to `char7`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> char7 {
        char7::new_unchecked(c as u8)
    }

    /// Tries to convert a `char8` to `char7`.
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    pub const fn try_from_char8(c: char8) -> Result<char7> {
        if Char::is_7bit(c.to_u32()) {
            Ok(char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `char16` to `char7`.
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char16")))]
    pub const fn try_from_char16(c: char16) -> Result<char7> {
        if Char::is_7bit(c.to_u32()) {
            Ok(char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `char24` to `char7`.
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    pub const fn try_from_char24(c: char24) -> Result<char7> {
        let c = c.to_u32();
        if Char::is_7bit(c) {
            Ok(char7::new_unchecked(c as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `char32` to `char8`.
    #[cfg(feature = "_char32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char32")))]
    pub const fn try_from_char32(c: char32) -> Result<char7> {
        if Char::is_7bit(c.to_u32()) {
            Ok(char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `char` to `char7`.
    pub const fn try_from_char(c: char) -> Result<char7> {
        if Char::is_7bit(c as u32) {
            Ok(char7::new_unchecked(c as u32 as u8))
        } else {
            Err(CharConversion)
        }
    }

    //

    /// Converts a `char7` to `AsciiChar`.
    #[must_use]
    pub const fn to_ascii_char(c: char7) -> AsciiChar {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        return if let Some(c) = AsciiChar::from_u8(c.0.get()) { c } else { unreachable!() };

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            AsciiChar::from_u8_unchecked(c.0.get())
        }
    }

    /// Converts this `char7` to `char8`.
    #[must_use]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    pub const fn to_char8(self) -> char8 {
        char8::from_char7(self)
    }
    /// Converts this `char7` to `char16`.
    #[must_use]
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char16")))]
    pub const fn to_char16(self) -> char16 {
        char16::from_char7(self)
    }
    /// Converts this `char7` to `char24`.
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    pub const fn to_char24(self) -> char24 {
        char24::from_char7(self)
    }
    /// Converts this `char7` to `char32`.
    #[must_use]
    #[cfg(feature = "_char32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char32")))]
    pub const fn to_char32(self) -> char32 {
        char32::from_char7(self)
    }
    /// Converts this `char7` to `char`.
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0.get() as char
    }
    /// Converts this `char7` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `char7` to an UTF-8 encoded sequence of bytes.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 1] {
        // From 0x0000 to 0x007F:
        // the UTF-8 encoding is the same as the scalar value.
        [self.0.get()]
    }

    //

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        false
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[must_use]
    pub const fn is_character(self) -> bool {
        true
    }

    /// Checks if the value is within the ASCII range.
    #[must_use]
    pub const fn is_ascii(self) -> bool {
        true
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_uppercase(self) -> char7 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_lowercase(self) -> char7 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}
