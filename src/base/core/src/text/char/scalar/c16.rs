// devela_base_core::text::char::scalar::c16
//
//!
//

use super::*;
use crate::{Char, CharAscii, MismatchedCapacity, text::char::NonSurrogateU16};

impl char16 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // for a few selected operations in this module involving ASCII.
    const fn from_char_unchecked(c: char) -> char16 {
        char16::new_unchecked(c as u32 as u16)
    }

    // SAFETY: this is not marked as unsafe because it's only used privately
    // for a few selected operations in this module and also by CharIter.
    pub(crate) const fn new_unchecked(value: u16) -> char16 {
        #[cfg(any(base_safe_text, not(feature = "unsafe_niche")))]
        return Self(crate::unwrap![some NonSurrogateU16::new(value)]);

        #[cfg(all(not(base_safe_text), feature = "unsafe_niche"))]
        unsafe {
            char16(NonSurrogateU16::new_unchecked(value))
        }
    }

    /* constants */

    /// The lowest Unicode scalar a `char16` can represent, `'\u{00}'`.
    pub const MIN: char16 = char16::new_unchecked(0x0000);

    /// The highest Unicode scalar a `char16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that `'\u{FFFF}'` is a *noncharacter*.
    pub const MAX: char16 = char16::new_unchecked(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: char16 =
        char16::new_unchecked(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* from_* conversions */

    /// Converts an `CharAscii` to `char16`.
    pub const fn from_char_ascii(c: CharAscii) -> char16 {
        char16::new_unchecked(c as u8 as u16)
    }

    /// Converts a `char7` to `char16`.
    pub const fn from_char7(c: char7) -> char16 {
        char16::new_unchecked(c.0.get() as u16)
    }
    /// Converts a `char8` to `char16`.
    pub const fn from_char8(c: char8) -> char16 {
        char16::new_unchecked(c.0 as u16)
    }
    /// Tries to convert a `charu` to `char16`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the character can't fit in 16 bits.
    pub const fn try_from_charu(c: charu) -> Result<char16, MismatchedCapacity> {
        let scalar = c.to_scalar();
        if Char(scalar).len_bytes() == 1 {
            Ok(char16::new_unchecked(scalar as u16))
        } else {
            Err(MismatchedCapacity::too_large(scalar as usize, u16::MAX as usize))
        }
    }
    /// Tries to convert a `char` to `char16`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the character can't fit in 16 bits.
    pub const fn try_from_char(c: char) -> Result<char16, MismatchedCapacity> {
        if Char(c as u32).len_bytes() <= 2 {
            Ok(char16::new_unchecked(c as u32 as u16))
        } else {
            Err(MismatchedCapacity::too_large(c as u32 as usize, u16::MAX as usize))
        }
    }

    /* to_* conversions */

    /// Tries to convert this `char16` to `CharAscii`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `self` can't fit in 7 bits.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    pub const fn try_to_char_ascii(self) -> Result<CharAscii, MismatchedCapacity> {
        if Char(self.to_scalar()).is_ascii() {
            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            if let Some(c) = CharAscii::from_u8(self.0.get() as u8) {
                return Ok(c);
            } else {
                unreachable![]
            }

            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { CharAscii::from_u8_unchecked(self.0.get() as u8) });
        }
        Err(MismatchedCapacity::too_large(self.to_scalar() as usize, 127))
    }

    /// Tries to convert this `char16` to `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `self` can't fit in 7 bits.
    pub const fn try_to_char7(self) -> Result<char7, MismatchedCapacity> {
        char7::try_from_char16(self)
    }
    /// Tries to convert this `char16` to `char8`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `self` can't fit in 8 bits.
    pub const fn try_to_char8(self) -> Result<char8, MismatchedCapacity> {
        char8::try_from_char16(self)
    }
    /// Converts this `char8` to `charu`.
    pub const fn to_charu(self) -> charu {
        charu::from_char16(self)
    }
    /// Converts this `char16` to `char`.
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to avoid duplicated validation.
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        if let Some(c) = char::from_u32(self.to_scalar()) { c } else { unreachable![] }

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: we guarantee we contain a valid scalar value
        return unsafe { char::from_u32_unchecked(self.to_scalar()) };
    }
    /// Converts this `char16` to a `u32` Unicode scalar value.
    #[must_use]
    pub const fn to_scalar(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `char16` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 3-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 3] {
        let c = self.0.get();
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c as u8, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (Char::<u8>::CONT_MASK & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (Char::<u8>::CONT_MASK & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & Char::<u16>::CONT_MASK) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z]
            }
        }
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    pub const fn to_ascii_uppercase(self) -> char16 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    pub const fn to_ascii_lowercase(self) -> char16 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }

    /* queries */

    /// Returns `true` if this Unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        Char(self.0.get() as u32).is_noncharacter()
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
        self.0.get() <= 0x7F
    }
}
