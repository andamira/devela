// devela_base_core::text::char::char
//
//! Define the [`UnicodeScalar`] trait.
//

#[rustfmt::skip]
#[doc = crate::_TAG_TEXT!()]
/// Common trait for Unicode scalar types.
///
#[doc = crate::_doc!(location: "text/char")]
///
/// It's implemented for: [`char7`], [`char8`], [`char16`],
/// and [`char`][crate::char].
pub trait UnicodeScalar {
    /// The lowest Unicode scalar that can be represented.
    const MIN: Self;

    /// The highest Unicode scalar that can be represented.
    const MAX: Self;

    /* encode */

    #[must_use]
    /// Returns the number of bytes needed to represent the scalar value.
    fn len_bytes(self) -> usize;

    #[must_use]
    /// Returns the number of bytes needed to encode in UTF-8.
    fn len_utf8(self) -> usize;

    #[must_use]
    /// Returns the number of bytes needed to encode in UTF-16.
    fn len_utf16(self) -> usize;

    #[must_use]
    /// Encodes this scalar as UTF-8 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length four is large enough to encode any char.
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str;

    #[must_use]
    /// Converts this `scalar` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    fn to_utf8_bytes(self) -> [u8; 4];

    #[must_use]
    /// Encodes this scalar as UTF-16 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length 2 is large enough to encode any char.
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];

    #[must_use]
    /// Converts the scalar to a digit in the given radix.
    ///
    /// ‘Digit’ is defined to be only the following characters:
    /// `0-9`, `a-z`, `A-Z`.
    ///
    /// # Errors
    /// Returns None if the char does not refer to a digit in the given radix.
    ///
    /// # Panics
    /// Panics if given a radix larger than 36.
    fn to_digit(self, radix: u32) -> Option<u32>;

    #[must_use]
    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    fn to_ascii_uppercase(self) -> Self where Self: Sized;

    #[must_use]
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    fn to_ascii_lowercase(self) -> Self where Self: Sized;

    /* escape */

    /* queries */

    // basic ASCII checks

    #[must_use]
    /// Checks if the value is within the ASCII range.
    fn is_ascii(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar is the nul character (`0x00`).
    fn is_nul(self) -> bool;

    // character type categories (general classification)

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `Alphabetic` property.
    fn is_alphabetic(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar has one of the general categories for numbers.
    ///
    /// If you want to parse ASCII decimal digits (0-9) or ASCII base-N,
    /// use [`is_ascii_digit`][Self#method.is_ascii_digit] or
    /// [`is_digit`][Self#method.is_digit] instead.
    fn is_numeric(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar satisfies either
    /// [`is_alphabetic()`][Self#method.is_alphabetic] or
    /// [`is_numeric()`][Self#method.is_numeric].
    fn is_alphanumeric(self) -> bool;

    #[must_use]
    /// Checks if the Unicode scalar is a digit in the given radix.
    ///
    /// See also [`to_digit`][Self#method.to_digit].
    fn is_digit(self, radix: u32) -> bool;

    // casing (related to alphabetic)

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `Lowercase` property.
    fn is_lowercase(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `Uppercase` property.
    fn is_uppercase(self) -> bool;

    // whitespace and controls (structural characters)

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `White_Space` property.
    fn is_whitespace(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar has the general category for control codes.
    fn is_control(self) -> bool;

    // special Unicode categories (edge cases, least common)

    #[must_use]
    /// Returns `true` if this Unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    fn is_noncharacter(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    fn is_character(self) -> bool where Self: Sized { !self.is_noncharacter() }

    #[must_use]
    /// Returns `true` if this Unicode scalar is a [combining character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#combining_character
    fn is_combining(self) -> bool;
}
