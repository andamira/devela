// devela_base_core::text::char::namespace
//
//! Defines the [`Char`] namespace.
//
// TOC
// - methods over char
// - methods over u32
// - methods over u16
// - methods over u8
// - methods over &[u8]

#[cfg(test)]
mod tests;

mod char; // Char<char>
mod u32; // Char<u32>
mod bytes; // Char<u8 | &[u8] | &[u8; N]>

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Unicode scalars-related *const* operations.
///
/// # Methods
/// - [over `char`](#methods-over-char)
/// - [over `u32`](#methods-over-u32)
/// - [over `u16`](#methods-over-u16)
/// - [over `u8`](#methods-over-u8)
/// - [over `&[u8]`](#methods-over-u8-slice)
///
/// See also [`Str`][crate::Str].
#[derive(Clone, Copy, Debug)]
pub struct Char<T>(pub T);

/// # Methods over `u16`.
#[rustfmt::skip]
impl Char<u16> {
    #[must_use] #[inline(always)]
    /// Returns `true` if the given unicode scalar code is a [surrogate code point][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#surrogate_code_point
    pub const fn is_surrogate(self) -> bool { matches!(self.0, 0xD800..=0xDFFF) }

    #[must_use] #[inline(always)]
    /// Returns `true` if the given unicode scalar code is a [leading surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#high_surrogate_code_point
    pub const fn is_surrogate_high(self) -> bool { matches!(self.0, 0xD800..=0xDBFF) }

    #[must_use] #[inline(always)]
    /// Returns `true` if the given unicode scalar code is a [trailing surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#low_surrogate_code_point
    pub const fn is_surrogate_low(self) -> bool { matches!(self.0, 0xDC00..=0xDFFF) }

    #[must_use]
    /// Decodes the given surrogate pair.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled to avoid double validation.
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
