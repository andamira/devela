// devela_base_core::text::fmt::num
//
//! Defines [`FmtNum`], [`FmtNumShape`].
//

use crate::{_TAG_FMT, Digits, Float, Slice, Str, StringU8, is, unwrap, whilst, write_at};

#[doc = _TAG_FMT!()]
/// Describes the structural shape of a formatted number.
///
/// The shape captures the lengths of the numeric regions and prefixes,
/// independent of padding, alignment, or styling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumShape {
    /// Number of prefix glyphs (e.g. sign or base prefix).
    pub prefix: u16,

    /// Number of digits in the integral (left) part.
    pub left: u16,

    /// Number of digits in the fractional (right) part.
    pub right: u16,
}
impl FmtNumShape {
    /// Creates a numeric shape from prefix and digit counts.
    ///
    /// - `prefix` counts leading non-digit glyphs (e.g. sign or base prefix).
    /// - `left` counts digits in the integral (left) part.
    /// - `right` counts digits in the fractional (right) part.
    #[inline(always)]
    pub const fn new(prefix: u16, left: u16, right: u16) -> Self {
        Self { prefix, left, right }
    }

    /// Returns whether the number has a radix separator.
    ///
    /// This is true when there is a fractional part (`right > 0`).
    #[inline(always)]
    pub const fn has_radix(&self) -> bool {
        self.right > 0
    }

    /// Returns the total number of glyphs required to render the number.
    ///
    /// This includes:
    /// - the prefix (e.g. sign),
    /// - integral digits,
    /// - the radix separator (if present),
    /// - fractional digits.
    #[inline(always)]
    pub const fn total(&self) -> usize {
        self.prefix as usize
            + self.left as usize
            + is![self.has_radix(); 1; 0]
            + self.right as usize
    }

    /// Returns the size of the left alignment block.
    ///
    /// This includes the prefix and all integral digits, but excludes
    /// the radix separator and fractional digits.
    #[inline(always)]
    pub const fn left_block(self) -> usize {
        (self.prefix + self.left) as usize
    }
}

#[doc = _TAG_FMT!()]
/// Const number formatter.
///
/// Provides a lightweight, allocation-free interface
/// for writing numeric values into an existing byte buffer.
///
/// # Example
/// ```
/// # use devela_base_core::FmtNum;
/// let mut buf = [0u8; 8];
/// let len = FmtNum(-123i32).write(&mut buf, 0);
/// assert_eq!(&buf[..len], b"-123");
///
/// let len = FmtNum(42u8).write(&mut buf, 0);
/// assert_eq!(&buf[..len], b"42");
///
/// let float_str = FmtNum(-4.2_f32).as_str_into(&mut buf, 2);
/// assert_eq!(float_str, "-4.19"); // be aware of floating-point inexactitudes
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNum<T>(pub T);

/// Implements [`FmtNum::write`] for all integer primitives.
///
/// Signed versions prepend `'-'` when negative and use `cast_unsigned` to obtain the absolute
/// unsigned value before delegating to [`Digits::write_digits10`].
macro_rules! impl_fmtnum {
    () => {
        impl_fmtnum!(signed i8, i16, i32, i64, i128, isize);
        impl_fmtnum!(unsigned u8, u16, u32, u64, u128, usize);
        impl_fmtnum!(float f32|u64, f64|u128);
    };
    (signed $($t:ty),+) => {$(
        impl_fmtnum!(common $t);

        impl FmtNum<$t> {
            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            /// Negative values are preceded by the `'-'` sign.
            #[inline(always)] #[rustfmt::skip]
            pub const fn write(self, buf: &mut [u8], mut pos: usize) -> usize {
                if self.0 < 0 {
                    let digits = Digits(self.0.wrapping_neg().cast_unsigned()); // abs
                    let needed = digits.count_digits10() as usize + 1;
                    if needed > buf.len().saturating_sub(pos) { return 0; }
                    write_at![buf, pos, b'-'];
                    digits.write_digits10(buf, pos) + 1
                } else {
                    let digits = Digits(self.0.cast_unsigned());
                    let needed = digits.count_digits10() as usize;
                    if needed > buf.len().saturating_sub(pos) { return 0; }
                    digits.write_digits10(buf, pos)
                }
            }

            /// Returns the measured shape of the integer to be formatted.
            pub const fn measure(self) -> FmtNumShape {
                let (prefix, left) = if self.0 < 0 {
                    (1, Digits(self.0.wrapping_neg().cast_unsigned()).count_digits10() as u16)
                } else {
                    (0, Digits(self.0.cast_unsigned()).count_digits10() as u16)
                };
                FmtNumShape::new(prefix, left, 0)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> FmtNumShape {}
            //
            // pub const fn write_fmt(self, buf: &mut [u8], pos: usize, fmt: FmtInt) -> usize {}
            // pub const fn measure_fmt(self, fmt: IntFmt) -> FmtNumShape {}
        }
    )+};
    (unsigned $($t:ty),+) => {$(
        impl_fmtnum!(common $t);

        impl FmtNum<$t> {
            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            #[inline(always)]
            pub const fn write(self, buf: &mut [u8], pos: usize) -> usize {
                let digits = Digits(self.0);
                let needed = digits.count_digits10() as usize;
                if needed > buf.len().saturating_sub(pos) { return 0; }
                digits.write_digits10(buf, pos)
            }

            /// Returns the measured shape of the integer to be formatted.
            pub const fn measure(self) -> FmtNumShape {
                let left = Digits(self.0).count_digits10() as u16;
                FmtNumShape::new(0, left, 0)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> FmtNumShape {}
            //
            // pub const fn write_fmt(self, buf: &mut [u8], pos: usize, fmt: FmtInt) -> usize {}
            // pub const fn measure_fmt(self, fmt: IntFmt) -> FmtNumShape {}
        }
    )+};
    // convenience methods for integers
    (common $($t:ty),+) => {$(
        impl FmtNum<$t> {
            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written.
            #[inline(always)]
            pub const fn as_bytes_into<'buf>(&self, buf: &'buf mut [u8]) -> &'buf [u8] {
                let len = self.write(buf, 0);
                Slice::range_to(buf, len)
            }
            /// Formats the number into a provided buffer and returns it as some byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked<'buf>(&self, buf: &'buf mut [u8])
                -> Option<&'buf [u8]> {
                let len = self.write(buf, 0);
                if len == 0 { return None; }
                Some(Slice::range_to(buf, len))
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'buf>(&self, buf: &'buf mut [u8]) -> &'buf str {
                let len = self.write(buf, 0);
                let slice = Slice::range_to(buf, len);

                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok Str::from_utf8(slice)];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: the ASCII bytes are always valid utf-8
                unsafe { Str::from_utf8_unchecked(slice) }
            }
            /// Formats the number into a provided buffer and returns it as some string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_checked<'buf>(&self, buf: &'buf mut [u8])
                -> Option<&'buf str> {
                let len = self.write(buf, 0);
                if len == 0 { return None; }
                let slice = Slice::range_to(buf, len);

                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok_some Str::from_utf8(slice)];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: the ASCII bytes are always valid utf-8
                Some(unsafe { Str::from_utf8_unchecked(slice) })
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            #[inline(always)]
            pub const fn as_string<const N: usize>(&self) -> StringU8<N> {
                let mut buf = [0u8; N];
                let len = self.write(&mut buf, 0);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            #[inline(always)]
            pub const fn as_string_checked<const N: usize>(&self) -> Option<StringU8<N>> {
                let mut buf = [0u8; N];
                let len = self.write(&mut buf, 0);
                if len == 0 { return None; }
                Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))
            }
        }
    )+ };
    // $t: floating-point primitive
    // $u: unsigned primitive to cast the integral part as
    (float $($t:ty | $u:ty),+) => {$(
        impl FmtNum<$t> {
            /// Writes the floating-point number in fixed-point decimal form into `buf`
            /// starting at `pos`, using exactly `fract_len` fractional digits.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            /// Negative values are preceded by the `'-'` sign.
            ///
            /// Fractional digits are truncated and zero-padded as needed.
            #[rustfmt::skip]
            pub const fn write(self, buf: &mut [u8], mut pos: usize, fract_len: u16) -> usize {
                let f = Float(self.0);
                let (neg, fabs) = if f.is_sign_negative() { (true, f.abs()) } else { (false, f) };
                // integral part
                let integ = fabs.trunc().0 as $u;
                let integ_digits = Digits(integ).count_digits10() as usize;
                // compute required space
                let (sign_len, dot_len) = (neg as usize, (fract_len > 0) as usize);
                let needed = sign_len + integ_digits + dot_len + fract_len as usize;
                if needed > buf.len().saturating_sub(pos) { return 0; }

                // write:
                if neg { write_at![buf, pos, b'-']; } // sign
                pos += Digits(integ).write_digits10(buf, pos); // integral digits
                if fract_len > 0 {
                    write_at![buf, pos, b'.']; // dot
                    let multiplier = Float(10.0 as $t).powi(fract_len as i32).0;
                    let fract = (fabs.fract().0 * multiplier) as $u;
                    let written = Digits(fract).write_digits10(buf, pos); // fractional digits
                    let missing = fract_len as usize - written;
                    whilst! { _i in 0..missing; { write_at![buf, pos, b'0']; }}
                }
                needed
            }

            /// Returns the measured shape of the floating-point to be formatted.
            pub const fn measure(self, fract_len: u16) -> FmtNumShape {
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let integ_digits = Digits(fabs.trunc().0 as $u).count_digits10() as u16;
                FmtNumShape::new(neg as u16, integ_digits, fract_len)
            }

            //

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is writte.
            #[inline(always)]
            pub const fn as_bytes_into<'buf>(&self, buf: &'buf mut [u8], fract_len: u16)
            -> &'buf [u8] {
                let len = self.write(buf, 0, fract_len);
                Slice::range_to(buf, len)
            }
            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked<'buf>(&self, buf: &'buf mut [u8], fract_len: u16)
            -> Option<&'buf [u8]> {
                let len = self.write(buf, 0, fract_len);
                if len == 0 { return None; }
                Some(Slice::range_to(buf, len))
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'buf>(&self, buf: &'buf mut [u8], fract_len: u16)
            -> &'buf str {
                let len = self.write(buf, 0, fract_len);
                let slice = Slice::range_to(buf, len);

                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok Str::from_utf8(slice)];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: the bytes are valid utf-8
                unsafe { Str::from_utf8_unchecked(slice) }
            }
            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            pub const fn as_str_into_checked<'buf>(&self, buf: &'buf mut [u8], fract_len: u16)
            -> Option<&'buf str> {
                let len = self.write(buf, 0, fract_len);
                if len == 0 { return None; }
                let slice = Slice::range_to(buf, len);

                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok_some Str::from_utf8(slice)];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: the ASCII bytes are always valid utf-8
                Some(unsafe { Str::from_utf8_unchecked(slice) })
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            #[inline(always)]
            pub const fn as_string<const N: usize>(&self, fract_len: u16) -> StringU8<N> {
                let mut buf = [0u8; N];
                let len = self.write(&mut buf, 0, fract_len);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            #[inline(always)]
            pub const fn as_string_checked<const N: usize>(&self, fract_len: u16)
                -> Option<StringU8<N>> {
                let mut buf = [0u8; N];
                let len = self.write(&mut buf, 0, fract_len);
                if len == 0 { return None; }
                Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))
            }
        }
    )+ };
}
impl_fmtnum!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[rustfmt::skip]
    fn fmtnum_float() {
        let mut buf = [0u8; 8];

        let f = FmtNum(-1.2f32); assert_eq!(f.measure(1).total(), 4);
        let s = f.as_str_into(&mut buf, 1); assert_eq!(s, "-1.2");

        let f = FmtNum(1.2f32); assert_eq!(f.measure(0).total(), 1);
        let s = f.as_str_into(&mut buf, 0); assert_eq!(s, "1");

        let f = FmtNum(1.2f32); assert_eq!(f.measure(1).total(), 3);
        let s = f.as_str_into(&mut buf, 1); assert_eq!(s, "1.2");

        let f = FmtNum(1.2f32); assert_eq!(f.measure(2).total(), 4);
        let s = f.as_str_into(&mut buf, 2); assert_eq!(s, "1.20");
    }

    #[test]
    fn fmtnum_signed() {
        let mut buf = [0u8; 8];

        // unsigned
        let len = FmtNum(0i32).write(&mut buf, 0);
        assert_eq!(&buf[..len], b"0");

        let len = FmtNum(u8::MAX).write(&mut buf, 0);
        assert_eq!(&buf[..len], b"255");

        // signed
        let len = FmtNum(-12_i32).write(&mut buf, 0);
        assert_eq!(&buf[..len], b"-12");

        let len = FmtNum(i8::MIN).write(&mut buf, 0);
        assert_eq!(&buf[..len], b"-128");
    }

    #[test]
    fn fmtnum_unsigned() {
        let mut buf = [0u8; 8];
        let len = FmtNum(255u8).write(&mut buf, 0);
        assert_eq!(&buf[..len], b"255");
    }

    #[test]
    fn fmtnum_truncation() {
        let mut buf = [0u8; 2];
        let len = FmtNum(1234u32).write(&mut buf, 0);
        // digits not written, since there's not enough space for all four digits
        assert_eq!(len, 0);
        assert_eq!(&buf, b"\0\0");

        // For negative numbers the sign should not be written wither
        let len = FmtNum(-123i32).write(&mut buf, 0);
        assert_eq!(len, 0);
        assert_eq!(&buf, b"\0\0");
    }
}
