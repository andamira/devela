// devela::text::char::impls::char16
//
//!
//

use super::*;
use crate::{
    text::char::NonSurrogateU16, AsciiChar, Char, TextError::CharConversion, TextResult as Result,
};

impl CharU16 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn from_char_unchecked(c: char) -> CharU16 {
        CharU16::new_unchecked(c as u32 as u16)
    }

    // useful because Option::<T>::unwrap is not yet stable as const fn
    #[must_use]
    const fn new_unchecked(value: u16) -> CharU16 {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        if let Some(c) = NonSurrogateU16::new(value) {
            CharU16(c)
        } else {
            unreachable![]
        }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            CharU16(NonSurrogateU16::new_unchecked(value))
        }
    }

    /* constants */

    /// The highest unicode scalar a `CharU16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that `'\u{FFFF}'` is a *noncharacter*.
    pub const MAX: CharU16 = CharU16::new_unchecked(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: CharU16 =
        CharU16::new_unchecked(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* conversions */

    /// Converts an `AsciiChar` to `CharU16`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> CharU16 {
        CharU16::new_unchecked(c as u8 as u16)
    }

    /// Converts a `CharU7` to `CharU16`.
    #[must_use]
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn from_char_u7(c: CharU7) -> CharU16 {
        CharU16::new_unchecked(c.0.get() as u16)
    }
    /// Converts a `CharU8` to `CharU16`.
    #[must_use]
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn from_char_u8(c: CharU8) -> CharU16 {
        CharU16::new_unchecked(c.0 as u16)
    }
    /// Tries to convert a `CharU24` to `CharU16`.
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn try_from_char_u24(c: CharU24) -> Result<CharU16> {
        let c = c.to_u32();
        if Char::byte_len(c) == 1 {
            Ok(CharU16::new_unchecked(c as u16))
        } else {
            Err(CharConversion)
        }
    }
    /// Tries to convert a `CharU32` to `CharU16`.
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn try_from_char_u32(c: CharU32) -> Result<CharU16> {
        Self::try_from_char(c.to_char())
    }
    /// Tries to convert a `char` to `CharU16`.
    pub const fn try_from_char(c: char) -> Result<CharU16> {
        if Char::byte_len(c as u32) <= 2 {
            Ok(CharU16::new_unchecked(c as u32 as u16))
        } else {
            Err(CharConversion)
        }
    }

    //

    /// Tries to convert this `CharU16` to `AsciiChar`.
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

    /// Tries to convert this `CharU16` to `CharU7`.
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn try_to_char_u7(self) -> Result<CharU7> {
        CharU7::try_from_char_u16(self)
    }
    /// Tries to convert this `CharU16` to `CharU8`.
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn try_to_char_u8(self) -> Result<CharU8> {
        CharU8::try_from_char_u16(self)
    }
    /// Converts this `CharU16` to `CharU24`.
    #[must_use]
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn to_char_u24(self) -> CharU24 {
        CharU24::from_char_u16(self)
    }
    /// Converts this `CharU16` to `CharU32`.
    #[must_use]
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn to_char_u32(self) -> CharU32 {
        CharU32::from_char_u16(self)
    }
    /// Converts this `CharU16` to `char`.
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
    /// Converts this `CharU16` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `CharU16` to an UTF-8 encoded sequence of bytes.
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
    pub const fn to_ascii_uppercase(self) -> CharU16 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    pub const fn to_ascii_lowercase(self) -> CharU16 {
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
