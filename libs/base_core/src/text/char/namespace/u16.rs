// devela_base_core::text::char::namespace::u16

use crate::Char;

/// # Methods over `u16`.
#[rustfmt::skip]
impl Char<u16> {
    /* private helpers */

    /// Bitmask for extracting the 6-bit payload from a UTF-8 continuation byte (`10xxxxxx`).
    pub(crate) const CONT_MASK: u16 = 0b0011_1111;

    /* public methods */

    #[must_use] #[inline(always)]
    /// Returns `true` if the given Unicode scalar code is a [surrogate code point][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#surrogate_code_point
    pub const fn is_surrogate(self) -> bool { matches!(self.0, 0xD800..=0xDFFF) }

    #[must_use] #[inline(always)]
    /// Returns `true` if the given Unicode scalar code is a [leading surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#high_surrogate_code_point
    pub const fn is_surrogate_high(self) -> bool { matches!(self.0, 0xD800..=0xDBFF) }

    #[must_use] #[inline(always)]
    /// Returns `true` if the given Unicode scalar code is a [trailing surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#low_surrogate_code_point
    pub const fn is_surrogate_low(self) -> bool { matches!(self.0, 0xDC00..=0xDFFF) }

    #[must_use]
    /// Decodes the given surrogate pair.
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip duplicated validation checks.
    pub const fn decode_surrogate_pair(high: u16, low: u16) -> Option<char> {
        if Char(high).is_surrogate_high() && Char(low).is_surrogate_low() {
            let code = 0x10000 + (((high as u32 - 0xD800) << 10) | (low as u32 - 0xDC00));
            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            return char::from_u32(code);
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            Some(unsafe { char::from_u32_unchecked(code) })
        } else {
            None
        }
    }
}
