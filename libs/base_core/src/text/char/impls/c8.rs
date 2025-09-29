// devela::text::char::impls::c8

use super::*;
use crate::{Char, CharAscii, DataOverflow};

impl char8 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // a few selected operations in this module.
    #[must_use]
    const fn from_char_unchecked(c: char) -> char8 {
        char8(c as u32 as u8)
    }

    /* constants */

    /// The lowest Unicode scalar a `char8` can represent, `'\u{00}'`.
    pub const MIN: char8 = char8(0x00);

    /// The highest Unicode scalar a `char8` can represent, `'\u{FF}'`.
    pub const MAX: char8 = char8(0xFF);

    /* from_* conversions */

    /// Converts an `CharAscii` to `char8`.
    #[must_use]
    pub const fn from_char_ascii(c: CharAscii) -> char8 {
        char8(c as u8)
    }

    /// Converts a `char7` to `char8`.
    #[must_use]
    pub const fn from_char7(c: char7) -> char8 {
        char8(c.0.get())
    }
    /// Tries to convert a `char16` to `char8`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if the character can't fit in 8 bits.
    pub const fn try_from_char16(c: char16) -> Result<char8, DataOverflow> {
        let scalar = c.to_scalar();
        if Char(scalar).len_bytes() == 1 {
            Ok(char8(scalar as u8))
        } else {
            Err(DataOverflow(Some(scalar as usize)))
        }
    }
    /// Tries to convert a `char_utf8` to `char8`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if the character can't fit in 8 bits.
    pub const fn try_from_char_utf8(c: char_utf8) -> Result<char8, DataOverflow> {
        let scalar = c.to_scalar();
        if Char(scalar).len_bytes() == 1 {
            Ok(char8(scalar as u8))
        } else {
            Err(DataOverflow(Some(scalar as usize)))
        }
    }
    /// Tries to convert a `char` to `char8`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if the character can't fit in 8 bits.
    pub const fn try_from_char(c: char) -> Result<char8, DataOverflow> {
        let scalar = c as u32;
        if Char(scalar).len_bytes() == 1 {
            Ok(char8(scalar as u8))
        } else {
            Err(DataOverflow(Some(scalar as usize)))
        }
    }

    /* to_* conversions */

    /// Tries to convert this `char8` to `CharAscii`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if `self` can't fit in 7 bits.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    pub const fn try_to_char_ascii(self) -> Result<CharAscii, DataOverflow> {
        if Char(self.to_scalar()).is_ascii() {
            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            if let Some(c) = CharAscii::from_u8(self.0) {
                return Ok(c);
            } else {
                unreachable![]
            }

            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { CharAscii::from_u8_unchecked(self.0) });
        }
        Err(DataOverflow(Some(self.to_scalar() as usize)))
    }

    /// Tries to convert this `char8` to `char7`.
    ///
    /// # Errors
    /// Returns [`DataOverflow`] if `self` can't fit in 7 bits.
    pub const fn try_to_char7(self) -> Result<char7, DataOverflow> {
        char7::try_from_char8(self)
    }
    /// Converts this `char8` to `char16`.
    #[must_use]
    pub const fn to_char16(self) -> char16 {
        char16::from_char8(self)
    }
    /// Converts this `char8` to `char_utf8`.
    #[must_use]
    pub const fn to_char_utf8(self) -> char_utf8 {
        char_utf8::from_char8(self)
    }
    /// Converts this `char8` to `char`.
    #[must_use]
    pub const fn to_char(self) -> char {
        self.0 as char
    }
    /// Converts this `char8` to a `u32` Unicode scalar value.
    #[must_use]
    pub const fn to_scalar(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `char8` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 2-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[must_use]
    #[allow(clippy::unusual_byte_groupings, clippy::single_match_else)]
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
                let y = 0b10_000000 | (Char::<u8>::CONT_MASK & c);
                let x = 0b110_00000 | (c >> 6);
                [x, y]
            }
        }
    }

    //

    /* queries */

    /// Returns `true` if this Unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        Char(self.0 as u32).is_noncharacter()
    }

    /// Returns `true` if this Unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[must_use]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }

    /// Checks if the value is within the ASCII range.
    #[must_use]
    pub const fn is_ascii(self) -> bool {
        self.0 <= 0x7F
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_uppercase(self) -> char8 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_lowercase(self) -> char8 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}
