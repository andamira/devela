// devela::text::char::impls::char7

use super::*;
use crate::text::{
    char::NonExtremeU8, char_is_7bit, AsciiChar, TextError::CharConversion, TextResult as Result,
};

impl CharU7 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn from_char_unchecked(c: char) -> CharU7 {
        CharU7::new_unchecked(c as u32 as u8)
    }

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn new_unchecked(value: u8) -> CharU7 {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        if let Some(c) = NonExtremeU8::new(value) {
            CharU7(c)
        } else {
            unreachable![]
        }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            CharU7(NonExtremeU8::new_unchecked(value))
        }
    }

    /* constants */

    /// The highest unicode scalar a `CharU7` can represent, `'\u{7F}'`.
    pub const MAX: CharU7 = CharU7::new_unchecked(0x7F);

    /* conversions */

    /// Converts an `AsciiChar` to `CharU7`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> CharU7 {
        CharU7::new_unchecked(c as u8)
    }

    /// Tries to convert a `CharU8` to `CharU7`.
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn try_from_char_u8(c: CharU8) -> Result<CharU7> {
        if char_is_7bit(c.to_u32()) {
            Ok(CharU7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `CharU16` to `CharU7`.
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn try_from_char_u16(c: CharU16) -> Result<CharU7> {
        if char_is_7bit(c.to_u32()) {
            Ok(CharU7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `CharU24` to `CharU7`.
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn try_from_char_u24(c: CharU24) -> Result<CharU7> {
        let c = c.to_u32();
        if char_is_7bit(c) {
            Ok(CharU7::new_unchecked(c as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `CharU32` to `CharU8`.
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn try_from_char_u32(c: CharU32) -> Result<CharU7> {
        if char_is_7bit(c.to_u32()) {
            Ok(CharU7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `char` to `CharU7`.
    pub const fn try_from_char(c: char) -> Result<CharU7> {
        if char_is_7bit(c as u32) {
            Ok(CharU7::new_unchecked(c as u32 as u8))
        } else {
            Err(CharConversion)
        }
    }

    //

    /// Converts a `CharU7` to `AsciiChar`.
    #[must_use]
    pub const fn to_ascii_char(c: CharU7) -> AsciiChar {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        return if let Some(c) = AsciiChar::from_u8(c.0.get()) { c } else { unreachable!() };

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            AsciiChar::from_u8_unchecked(c.0.get())
        }
    }

    /// Converts this `CharU7` to `CharU8`.
    #[must_use]
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn to_char_u8(self) -> CharU8 {
        CharU8::from_char_u7(self)
    }
    /// Converts this `CharU7` to `CharU16`.
    #[must_use]
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn to_char_u16(self) -> CharU16 {
        CharU16::from_char_u7(self)
    }
    /// Converts this `CharU7` to `CharU24`.
    #[must_use]
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn to_char_u24(self) -> CharU24 {
        CharU24::from_char_u7(self)
    }
    /// Converts this `CharU7` to `CharU32`.
    #[must_use]
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn to_char_u32(self) -> CharU32 {
        CharU32::from_char_u7(self)
    }
    /// Converts this `CharU7` to `char`.
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0.get() as char
    }
    /// Converts this `CharU7` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `CharU7` to an UTF-8 encoded sequence of bytes.
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
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> CharU7 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> CharU7 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}
