// devela_base_core::text::fmt::num
//
//! Defines [`FmtNum`], [`FmtNumShape`].
//

use crate::{_TAG_FMT, Digits, Float, Slice, Str, StringU8, unwrap, whilst, write_at};

#[doc = _TAG_FMT!()]
/// Describes the measured shape of a formatted number.
///
/// The shape captures the number of digits on each side of the radix point,
/// independent of field width, padding, or alignment.
///
/// For formats with additional notation (e.g. scientific),
/// the radix refers to the primary numeric mantissa.
///
/// This information is useful for higher-level formatting tasks
/// such as decimal alignment and column layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumShape {
    /// Number of digits to the left of the radix point,
    /// including any sign or leading numeric zeros.
    pub left_len: usize,

    /// Number of digits to the right of the radix point.
    ///
    /// This is always `0` for integers.
    ///
    pub right_len: usize,
}
impl FmtNumShape {
    /// Creates a new numeric shape from left and right digit counts.
    #[inline(always)] #[rustfmt::skip]
    pub const fn new(left_len: usize, right_len: usize) -> Self { Self { left_len, right_len } }
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
        impl_fmtnum!(float f32, f64);
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
                let left = if self.0 < 0 {
                    Digits(self.0.wrapping_neg().cast_unsigned()).count_digits10() as usize + 1
                } else {
                    Digits(self.0.cast_unsigned()).count_digits10() as usize
                };
                FmtNumShape::new(left, 0)
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
                let left = Digits(self.0).count_digits10() as usize;
                FmtNumShape::new(left, 0)
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
            pub const fn as_bytes_into<'a>(&self, buf: &'a mut [u8]) -> &'a [u8] {
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
    (float $($t:ty),+) => {$(
        impl FmtNum<$t> {
            /// Formats the number into a provided buffer and returns it as a byte slice.
            #[inline(always)]
            pub const fn as_bytes_into<'a>(&self, buf: &'a mut [u8], fract_len: u16) -> &'a [u8] {
                let len = self.write(buf, 0, fract_len);
                Slice::range_to(buf, len)
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// The string will be empty if there isn't enough space for the full number.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'buf>(&self, buf: &'buf mut [u8], fract_len: u16) -> &'buf str {
                let len = self.write(buf, 0, fract_len);
                let slice = Slice::range_to(buf, len);

                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return crate::unwrap![ok Str::from_utf8(slice)];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: the bytes are valid utf-8
                unsafe { Str::from_utf8_unchecked(slice) }
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// The string will be empty if there isn't enough space for the full number.
            #[inline(always)]
            pub const fn as_string<const N: usize>(&self, fract_len: u16) -> StringU8<N> {
                let mut buf = [0u8; N];
                let len = self.write(&mut buf, 0, fract_len);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }

            /// Writes the floating point as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// - It writes the integral part, followed by a dot, and the given number of decimals.
            /// - For negative values, it also writes a leading `'-'`.
            /// - Returns the number of bytes written.
            #[rustfmt::skip]
            pub const fn write(self, buf: &mut [u8], mut pos: usize, fract_len: u16) -> usize {
                let f = Float(self.0);
                let (integ, fract);

                if f.is_sign_negative() {
                    if pos >= buf.len() { return 0; } // no room for the sign
                    write_at![buf, pos, b'-'];
                    let fabs = f.abs();
                    (integ, fract) = (fabs.trunc().0 as u32, fabs.fract().0);
                } else {
                    (integ, fract) = (f.trunc().0 as u32, f.fract().0);
                }
                // integral part
                pos += Digits(integ).write_digits10(buf, pos);

                // fractional part
                if fract_len > 0 && pos < buf.len() {
                    write_at![buf, pos, b'.'];
                    let multiplier = Float(10.0 as $t).powi(fract_len as i32).0;
                    let fract = (fract * multiplier) as u32;
                    let len = Digits(fract).write_digits10(buf, pos);
                    pos += len;
                    let diff_fract_len = fract_len as usize - len;
                    whilst! { d in 0..diff_fract_len; { write_at![buf, pos, b'0']; }}
                }
                pos
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
        let s = FmtNum(-1.2f32).as_str_into(&mut buf, 1); assert_eq!(s, "-1.2");
        let s = FmtNum(1.2f32).as_str_into(&mut buf, 0); assert_eq!(s, "1");
        let s = FmtNum(1.2f32).as_str_into(&mut buf, 1); assert_eq!(s, "1.2");
        let s = FmtNum(1.2f32).as_str_into(&mut buf, 2); assert_eq!(s, "1.20");
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
