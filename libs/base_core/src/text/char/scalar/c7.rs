// devela_base_core::text::char::scalar::c7
//
//!
//

use super::*;
use crate::{
    Char, CharAscii, ConstInitCore, MismatchedCapacity, NonExtremeU8, Str, TextLut, cmp, whilst,
};

#[rustfmt::skip]
impl char7 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    const fn from_char_unchecked(c: char) -> char7 { char7::new_unchecked(c as u32 as u8) }

    // SAFETY: this is not marked as unsafe because it's only used privately
    // for a few selected operations in this module and also by CharIter.
    pub(crate) const fn new_unchecked(value: u8) -> char7 {
        #[cfg(any(base_safe_text, not(feature = "unsafe_niche")))]
        if let Some(c) = NonExtremeU8::new(value) { char7(c) } else { unreachable![] }
        #[cfg(all(not(base_safe_text), feature = "unsafe_niche"))]
        unsafe { char7(NonExtremeU8::new_unchecked(value)) }
    }

    /* constants */

    /// The lowest Unicode scalar a `char7` can represent, `'\u{00}'`.
    pub const MIN: char7 = char7::new_unchecked(0x00);

    /// The highest Unicode scalar a `char7` can represent, `'\u{7F}'`.
    pub const MAX: char7 = char7::new_unchecked(0x7F);

    /* from_* conversions */

    /// Converts an `CharAscii` to `char7`.
    pub const fn from_char_ascii(c: CharAscii) -> char7 { char7::new_unchecked(c as u8) }

    /// Tries to convert a `char8` to `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the character can't fit in 7 bits.
    pub const fn try_from_char8(c: char8) -> Result<char7, MismatchedCapacity> {
        let scalar = c.to_scalar();
        if Char(scalar).is_ascii() { Ok(char7::new_unchecked(scalar as u8)) }
        else { Err(MismatchedCapacity::too_large(scalar as usize, 127)) }
    }
    /// Tries to convert a `char16` to `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the character can't fit in 7 bits.
    pub const fn try_from_char16(c: char16) -> Result<char7, MismatchedCapacity> {
        let scalar = c.to_scalar();
        if Char(scalar).is_ascii() { Ok(char7::new_unchecked(scalar as u8)) }
        else { Err(MismatchedCapacity::too_large(scalar as usize, 127)) }
    }
    /// Tries to convert a `charu` to `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the character can't fit in 7 bits.
    pub const fn try_from_charu(c: charu) -> Result<char7, MismatchedCapacity> {
        let scalar = c.to_scalar();
        if Char(scalar).is_ascii() { Ok(char7::new_unchecked(scalar as u8)) }
        else { Err(MismatchedCapacity::too_large(scalar as usize, 127)) }
    }

    /// Tries to convert a `char` to `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the character can't fit in 7 bits.
    pub const fn try_from_char(c: char) -> Result<char7, MismatchedCapacity> {
        if Char(c as u32).is_ascii() { Ok(char7::new_unchecked(c as u32 as u8)) }
        else { Err(MismatchedCapacity::too_large(c as u32 as usize, 127)) }
    }

    /* to_* conversions */

    /// Returns the byte representation.
    #[inline(always)]
    pub const fn to_byte(&self) -> u8 { self.0.get() }
    /// Returns the string slice representation.
    #[inline(always)]
    pub const fn to_str(&self) -> &'static str { TextLut::ASCII_CHARS[self.to_byte() as usize] }

    /// Converts a `char7` to `CharAscii`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[must_use]
    pub const fn to_char_ascii(c: char7) -> CharAscii {
        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return if let Some(c) = CharAscii::from_u8(c.0.get()) { c } else { unreachable!() };
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        unsafe { CharAscii::from_u8_unchecked(c.0.get()) }
    }

    /// Converts this `char7` to `char8`.
    pub const fn to_char8(self) -> char8 { char8::from_char7(self) }
    /// Converts this `char7` to `char16`.
    pub const fn to_char16(self) -> char16 { char16::from_char7(self) }
    /// Converts this `char7` to `charu`.
    pub const fn to_charu(self) -> charu { charu::from_char7(self) }
    #[must_use]
    /// Converts this `char7` to `char`.
    pub const fn to_char(self) -> char { self.0.get() as char }
    #[must_use]
    /// Converts this `char7` to a `u32` Unicode scalar value.
    pub const fn to_scalar(self) -> u32 { self.0.get() as u32 }

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

    /// Returns `true` if this Unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool { false }

    /// Returns `true` if this Unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[must_use]
    pub const fn is_character(self) -> bool { true }

    /// Checks if the value is within the ASCII range.
    #[must_use]
    pub const fn is_ascii(self) -> bool { true }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    pub const fn to_ascii_uppercase(self) -> char7 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    pub const fn to_ascii_lowercase(self) -> char7 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}

#[rustfmt::skip]
impl char7 {
    /* byte arrays */

    /// Tries to convert a `[u8; N]` into `[char7; N]`, ensuring ASCII.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if any byte is not valid ASCII (greater than `0x7F`).
    pub const fn try_from_u8_array<const N: usize>(bytes: [u8; N])
        -> Result<[char7; N], MismatchedCapacity> {
        let mut out = [char7::INIT; N];
        whilst! { i in 0..N; {
            if bytes[i] <= 0x7F { out[i] = char7::new_unchecked(bytes[i]); }
            else { return Err(MismatchedCapacity::too_large(bytes[i] as usize, 0x7F)); }
        }}
        Ok(out)
    }

    /// Converts a `[u8; N]` into `[char7; N]` without checks.
    ///
    /// # Safety
    /// All bytes must be ASCII (`<= 0x7F`).
    pub const unsafe fn from_u8_array_unchecked<const N: usize>(bytes: [u8; N]) -> [char7; N] {
        let mut out = [char7::INIT; N];
        whilst! { i in 0..N; { out[i] = char7::new_unchecked(bytes[i]); }}
        out
    }

    /* byte slices */

    /// Reads ASCII bytes from `bytes` into `out`.
    ///
    /// Converts up to `min(bytes.len(), out.len())` elements and returns
    /// the number of characters written.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if a non-ASCII byte (greater than `0x7F`) is encountered.
    pub const fn read_u8_slice(bytes: &[u8], out: &mut [char7])
        -> Result<usize, MismatchedCapacity> {
        let len = cmp!(min bytes.len(), out.len());
        whilst! { i in 0..len; {
            let b = bytes[i];
            if b <= 0x7F { out[i] = char7::new_unchecked(b); }
            else { return Err(MismatchedCapacity::too_large(b as usize, 0x7F)); }
        }}
        Ok(len)
    }

    /// Reads ASCII bytes from `bytes` into `out`, stopping at the first non-ASCII byte.
    ///
    /// Converts up to `min(bytes.len(), out.len())` elements and returns
    /// the number of characters written.
    ///
    /// Invalid (non-ASCII) bytes terminate the conversion but are not reported as an error.
    pub const fn read_u8_slice_trunc(bytes: &[u8], out: &mut [char7]) -> usize {
        let len = cmp!(min bytes.len(), out.len());
        whilst! { i in 0..len; {
            let b = bytes[i];
            if b > 0x7F { break; }
            out[i] = char7::new_unchecked(b);
        }}
        i
    }

    /// Writes ASCII bytes into `out`.
    ///
    /// Writes up to `min(chars.len(), out.len())` bytes and returns the number written.
    pub const fn write_u8_slice(chars: &[char7], out: &mut [u8]) -> usize {
        let len = cmp!(min chars.len(), out.len());
        whilst! { i in 0..len; { out[i] = chars[i].to_byte(); }}
        len
    }

    /* string slices */

    /// Writes the UTF-8 bytes into `out` and returns a `&str` view into it.
    ///
    /// The returned string slice borrows `out`.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    pub const fn write_str<'s, const N: usize>(chars: &[char7; N], out: &'s mut [u8; N])
        -> &'s str {
        whilst! { i in 0..N; { out[i] = chars[i].to_byte(); }}

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return crate::unwrap![ok Str::from_utf8(out)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: all bytes are ASCII by construction
        unsafe { Str::from_utf8_unchecked(out) }
    }

    /* ASCII normalization / comparison */

    /// Converts all characters in the array to their ASCII lowercase form.
    pub const fn to_ascii_lowercase_array<const N: usize>(mut a: [char7; N]) -> [char7; N] {
        whilst! { i in 0..N; { a[i] = a[i].to_ascii_lowercase(); }}
        a
    }

    /// Compares two arrays for equality, ignoring ASCII case differences.
    pub const fn eq_ignore_ascii_case<const N: usize>(a: &[char7; N], b: &[char7; N]) -> bool {
        whilst! { i in 0..N; {
            // if !a[i].to_ascii_lowercase().eq(b[i].to_ascii_lowercase()) { return false; }
            if !a[i].to_byte().eq_ignore_ascii_case(&b[i].to_byte()) { return false; }
        }}
        true
    }
}
