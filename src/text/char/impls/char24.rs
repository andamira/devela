// devela::text::char::impls::char24

use super::*;
use crate::text::{
    char_is_7bit, char_is_noncharacter, AsciiChar, TextError::CharConversion, TextResult as Result,
};

impl CharU24 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[must_use]
    const fn new_unchecked_hi(value: u8) -> NonExtremeU8 {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_niche")))]
        if let Some(c) = NonExtremeU8::new(value) {
            c
        } else {
            unreachable![]
        }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_niche"))]
        unsafe {
            NonExtremeU8::new_unchecked(value)
        }
    }

    /* constants */

    /// The highest unicode scalar a `CharU24` can represent, `'\u{10FFFF}'`.
    pub const MAX: CharU24 = CharU24::from_char('\u{10ffff}');

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: CharU24 = CharU24::from_char(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts an `AsciiChar` to `CharU24`.
    #[must_use]
    pub const fn from_ascii_char(c: AsciiChar) -> CharU24 {
        CharU24 { hi: Self::new_unchecked_hi(0), mi: 0, lo: c as u8 }
    }

    /// Converts a `CharU7` to `CharU24`.
    #[must_use]
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn from_char_u7(c: CharU7) -> CharU24 {
        CharU24 {
            hi: Self::new_unchecked_hi(0),
            mi: 0,
            lo: c.0.get(),
        }
    }
    /// Converts a `CharU8` to `CharU24`.
    #[must_use]
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn from_char_u8(c: CharU8) -> CharU24 {
        CharU24 { hi: Self::new_unchecked_hi(0), mi: 0, lo: c.0 }
    }
    /// Converts a `CharU16` to `CharU24`.
    #[must_use]
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn from_char_u16(c: CharU16) -> CharU24 {
        let mi = ((c.0.get() & 0xFF00) >> 8) as u8;
        let lo = (c.0.get() & 0x00FF) as u8;
        CharU24 { hi: Self::new_unchecked_hi(0), mi, lo }
    }
    /// Converts a `CharU32` to `CharU24`.
    #[must_use]
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn from_char_u32(c: CharU32) -> CharU24 {
        CharU24::from_char(c.0)
    }
    /// Converts a `char` to `CharU24`.
    #[must_use]
    pub const fn from_char(c: char) -> CharU24 {
        let hi = ((c as u32 & 0x001F_0000) >> 16) as u8;
        let mi = ((c as u32 & 0x0000_FF00) >> 8) as u8;
        let lo = (c as u32 & 0x000_000FF) as u8;
        CharU24 { hi: Self::new_unchecked_hi(hi), mi, lo }
    }

    //

    /// Tries to convert this `CharU24` to `AsciiChar`.
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if char_is_7bit(self.to_u32()) {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            if let Some(c) = AsciiChar::from_u8(self.lo) {
                return Ok(c);
            } else {
                unreachable![]
            }

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { AsciiChar::from_u8_unchecked(self.lo) });
        }
        Err(CharConversion)
    }

    /// Tries to convert this `CharU24` to `CharU7`.
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn try_to_char_u7(self) -> Result<CharU7> {
        CharU7::try_from_char_u24(self)
    }
    /// Tries to convert this `CharU24` to `CharU8`.
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn try_to_char_u8(self) -> Result<CharU8> {
        CharU8::try_from_char_u24(self)
    }
    /// Tries to convert this `CharU24` to `CharU16`.
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn try_to_char_u16(self) -> Result<CharU16> {
        CharU16::try_from_char_u24(self)
    }
    /// Converts this `CharU24` to `CharU32`.
    #[must_use]
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn to_char_u32(self) -> CharU32 {
        CharU32(self.to_char())
    }
    /// Converts this `CharU24` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        (self.hi.get() as u32) << 16 | (self.mi as u32) << 8 | (self.lo as u32)
    }
    /// Converts this `CharU24` to `char`.
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        // #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        if let Some(c) = char::from_u32(self.to_u32()) { c } else { unreachable![] }

        // WAIT: [stable const](https://github.com/rust-lang/rust/issues/89259)
        // #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: we've already checked we contain a valid char.
        // return unsafe { char::from_u32_unchecked(code_point) };
    }

    /// Converts this `CharU24` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        let c = self.to_u32();
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (0b0011_1111 & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (0b0011_1111 & (c as u8));
                let z = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let y = 0b10_000000 | ((c >> 12) & 0b0011_1111) as u8;
                let x = 0b11110_000 | ((c >> 18) as u8);
                [x, y, z, w]
            }
        }
    }

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> CharU24 {
        Self::from_char(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> CharU24 {
        Self::from_char(char::to_ascii_lowercase(&self.to_char()))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        char_is_noncharacter(self.to_u32())
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
        self.to_u32() <= 0x7F
    }
}
