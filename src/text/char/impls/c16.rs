// devela::text::char::impls::char16
//
//!
//

use super::*;
use crate::{
    text::char::NonSurrogateU16, AsciiChar, Char, TextError::CharConversion, TextResult as Result,
};

impl char16 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn from_char_unchecked(c: char) -> char16 {
        char16::new_unchecked(c as u32 as u16)
    }

    // useful because Option::<T>::unwrap is not yet stable as const fn
    #[must_use]
    const fn new_unchecked(value: u16) -> char16 {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        if let Some(c) = NonSurrogateU16::new(value) {
            char16(c)
        } else {
            unreachable![]
        }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            char16(NonSurrogateU16::new_unchecked(value))
        }
    }

    /* constants */

    /// The highest unicode scalar a `char16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that `'\u{FFFF}'` is a *noncharacter*.
    pub const MAX: char16 = char16::new_unchecked(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: char16 =
        char16::new_unchecked(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* conversions */

    /// Converts an `AsciiChar` to `char16`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> char16 {
        char16::new_unchecked(c as u8 as u16)
    }

    /// Converts a `char7` to `char16`.
    #[must_use]
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char7")))]
    pub const fn from_char7(c: char7) -> char16 {
        char16::new_unchecked(c.0.get() as u16)
    }
    /// Converts a `char8` to `char16`.
    #[must_use]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    pub const fn from_char8(c: char8) -> char16 {
        char16::new_unchecked(c.0 as u16)
    }
    /// Tries to convert a `char24` to `char16`.
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    pub const fn try_from_char24(c: char24) -> Result<char16> {
        let c = c.to_u32();
        if Char::byte_len(c) == 1 {
            Ok(char16::new_unchecked(c as u16))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `char` to `char16`.
    pub const fn try_from_char(c: char) -> Result<char16> {
        if Char::byte_len(c as u32) <= 2 {
            Ok(char16::new_unchecked(c as u32 as u16))
        } else {
            Err(CharConversion)
        }
    }

    //

    /// Tries to convert this `char16` to `AsciiChar`.
    /// # Features
    /// Makes use of the `unsafe_niche` feature if enabled.
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if Char::is_7bit(self.to_u32()) {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
            if let Some(c) = AsciiChar::from_u8(self.0.get() as u8) {
                return Ok(c);
            } else {
                unreachable![]
            }

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { AsciiChar::from_u8_unchecked(self.0.get() as u8) });
        }
        Err(CharConversion)
    }

    /// Tries to convert this `char16` to `char7`.
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char7")))]
    pub const fn try_to_char7(self) -> Result<char7> {
        char7::try_from_char16(self)
    }
    /// Tries to convert this `char16` to `char8`.
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    pub const fn try_to_char8(self) -> Result<char8> {
        char8::try_from_char16(self)
    }
    /// Converts this `char16` to `char24`.
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    pub const fn to_char24(self) -> char24 {
        char24::from_char16(self)
    }
    /// Converts this `char16` to `char`.
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        // #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        if let Some(c) = char::from_u32(self.0.get() as u32) { c } else { unreachable![] }

        // WAIT: [stable const](https://github.com/rust-lang/rust/issues/89259)
        // #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        // SAFETY: we've already checked we contain a valid char.
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }
    /// Converts this `char16` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
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
                let y = 0b10_000000 | (0b0011_1111 & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z]
            }
        }
    }

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_uppercase(self) -> char16 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_lowercase(self) -> char16 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        Char::is_noncharacter(self.0.get() as u32)
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
        self.0.get() <= 0x7F
    }
}
