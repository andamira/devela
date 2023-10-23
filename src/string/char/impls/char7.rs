// devela::string::char::impls::char7

use super::*;
use crate::string::AsciiChar;

impl Char7 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[inline]
    #[must_use]
    const fn from_char_unchecked(c: char) -> Char7 {
        Char7::new_unchecked(c as u32 as u8)
    }

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[inline]
    #[must_use]
    const fn new_unchecked(value: u8) -> Char7 {
        #[cfg(not(all(feature = "unsafe_string", feature = "unsafe_num")))]
        if let Some(c) = NonMaxU8::new(value) {
            Char7(c)
        } else {
            unreachable![]
        }
        #[cfg(all(feature = "unsafe_string", feature = "unsafe_num"))]
        unsafe {
            Char7(NonMaxU8::new_unchecked(value))
        }
    }

    /* constants */

    /// The highest unicode scalar a `Char7` can represent, `'\u{7F}'`.
    pub const MAX: Char7 = Char7::new_unchecked(0x7F);

    /* conversions */

    /// Converts an `AsciiChar` to `Char7`.
    #[inline]
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> Char7 {
        Char7::new_unchecked(c as u8)
    }

    /// Tries to convert a `Char8` to `Char7`.
    #[inline]
    pub const fn try_from_char8(c: Char8) -> Result<Char7> {
        if char_is_7bit(c.to_u32()) {
            Ok(Char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char16` to `Char7`.
    #[inline]
    pub const fn try_from_char16(c: Char16) -> Result<Char7> {
        if char_is_7bit(c.to_u32()) {
            Ok(Char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char24` to `Char7`.
    #[inline]
    pub const fn try_from_char24(c: Char24) -> Result<Char7> {
        let c = c.to_u32();
        if char_is_7bit(c) {
            Ok(Char7::new_unchecked(c as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char32` to `Char8`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char7> {
        if char_is_7bit(c.to_u32()) {
            Ok(Char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `char` to `Char8`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char7> {
        if char_is_7bit(c as u32) {
            Ok(Char7::new_unchecked(c as u32 as u8))
        } else {
            Err(CharConversionError(()))
        }
    }

    //

    /// Converts a `Char7` to `AsciiChar`.
    #[inline]
    #[must_use]
    pub const fn to_ascii_char(c: Char7) -> AsciiChar {
        #[cfg(not(all(feature = "unsafe_string", feature = "unsafe_ascii")))]
        if let Some(c) = AsciiChar::from_u8(c.0.get()) {
            c
        } else {
            unreachable!()
        }

        #[cfg(all(feature = "unsafe_string", feature = "unsafe_ascii"))]
        unsafe {
            AsciiChar::from_u8_unchecked(c.0.get())
        }
    }

    /// Converts this `Char7` to `Char8`.
    #[inline]
    #[must_use]
    pub const fn to_char8(self) -> Char8 {
        Char8::from_char7(self)
    }
    /// Converts this `Char7` to `Char16`.
    #[inline]
    #[must_use]
    pub const fn to_char16(self) -> Char16 {
        Char16::from_char7(self)
    }
    /// Converts this `Char7` to `Char24`.
    #[inline]
    #[must_use]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char7(self)
    }
    /// Converts this `Char7` to `Char32`.
    #[inline]
    #[must_use]
    pub const fn to_char32(self) -> Char32 {
        Char32::from_char7(self)
    }
    /// Converts this `Char7` to `char`.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        self.0.get() as char
    }
    /// Converts this `Char7` to `u32`.
    #[inline]
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `Char7` to an UTF-8 encoded sequence of bytes.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
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
    #[inline]
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        false
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    #[must_use]
    pub const fn is_character(self) -> bool {
        true
    }

    /// Checks if the value is within the ASCII range.
    #[inline]
    #[must_use]
    pub const fn is_ascii(self) -> bool {
        true
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char7 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char7 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}
