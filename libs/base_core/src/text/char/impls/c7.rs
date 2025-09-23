// devela::text::char::impls::char7
//
//!
//

use super::*;

use crate::{ASCII_TABLE, AsciiChar, Char, DataOverflow, NonExtremeU8};

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
        #[cfg(any(base_safe_text, not(feature = "unsafe_niche")))]
        if let Some(c) = NonExtremeU8::new(value) {
            char7(c)
        } else {
            unreachable![]
        }
        #[cfg(all(not(base_safe_text), feature = "unsafe_niche"))]
        unsafe {
            char7(NonExtremeU8::new_unchecked(value))
        }
    }

    /* constants */

    /// The lowest unicode scalar a `char7` can represent, `'\u{00}'`.
    pub const MIN: char7 = char7::new_unchecked(0x00);

    /// The highest unicode scalar a `char7` can represent, `'\u{7F}'`.
    pub const MAX: char7 = char7::new_unchecked(0x7F);

    /* conversions */

    /// Converts an `AsciiChar` to `char7`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> char7 {
        char7::new_unchecked(c as u8)
    }

    /// Tries to convert a `char8` to `char7`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if the character can't fit in 7 bits.
    pub const fn try_from_char8(c: char8) -> Result<char7, DataOverflow> {
        if Char(c.to_u32()).is_7bit() {
            Ok(char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(DataOverflow(Some(c.to_u32() as usize)))
        }
    }
    /// Tries to convert a `char16` to `char7`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if the character can't fit in 7 bits.
    pub const fn try_from_char16(c: char16) -> Result<char7, DataOverflow> {
        if Char(c.to_u32()).is_7bit() {
            Ok(char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(DataOverflow(Some(c.to_u32() as usize)))
        }
    }
    /// Tries to convert a `char` to `char7`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if the character can't fit in 7 bits.
    pub const fn try_from_char(c: char) -> Result<char7, DataOverflow> {
        if Char(c as u32).is_7bit() {
            Ok(char7::new_unchecked(c as u32 as u8))
        } else {
            Err(DataOverflow(Some(c as u32 as usize)))
        }
    }

    //

    /// Returns the byte representation.
    #[inline(always)]
    pub const fn to_byte(&self) -> u8 {
        self.0.get()
    }
    /// Returns the string slice representation.
    #[inline(always)]
    pub const fn to_str(&self) -> &'static str {
        ASCII_TABLE[self.to_byte() as usize]
    }

    /// Converts a `char7` to `AsciiChar`.
    #[must_use]
    pub const fn to_ascii_char(c: char7) -> AsciiChar {
        #[cfg(any(base_safe_text, not(feature = "unsafe_niche")))]
        return if let Some(c) = AsciiChar::from_u8(c.0.get()) { c } else { unreachable!() };

        #[cfg(all(not(base_safe_text), feature = "unsafe_niche"))]
        unsafe {
            AsciiChar::from_u8_unchecked(c.0.get())
        }
    }

    /// Converts this `char7` to `char8`.
    #[must_use]
    pub const fn to_char8(self) -> char8 {
        char8::from_char7(self)
    }
    /// Converts this `char7` to `char16`.
    #[must_use]
    pub const fn to_char16(self) -> char16 {
        char16::from_char7(self)
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
