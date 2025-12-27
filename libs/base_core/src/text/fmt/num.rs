// devela_base_core::text::fmt::num
//
//! Defines [`FmtNum`].
//
// TODO: hexadecimal
// TODO: binary
// TODO: octal
// TODO: floating-point

use crate::{Digits, Float, Slice, Str, StringU8, whilst, write_at};

#[doc = crate::_TAG_FMT!()]
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
            /// - For negative values, writes a leading `'-'` followed by the absolute digits.
            /// - Returns the number of bytes written.
            #[inline(always)] #[rustfmt::skip]
            pub const fn write(self, buf: &mut [u8], mut pos: usize) -> usize {
                let n = self.0;
                if n < 0 {
                    if pos >= buf.len() { return 0; } // no room for the sign
                    write_at![buf, pos, b'-'];
                    let abs = n.wrapping_neg().cast_unsigned();
                    let written = Digits(abs).write_digits10(buf, pos);
                    1 + written
                } else {
                    Digits(n.cast_unsigned()).write_digits10(buf, pos)
                }
            }
        }
    )+};
    (unsigned $($t:ty),+) => {$(
        impl_fmtnum!(common $t);

        impl FmtNum<$t> {
            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// - Returns the number of bytes written.
            /// - Writes nothing if there isn't enough space for the full number.
            #[inline(always)]
            pub const fn write(self, buf: &mut [u8], off: usize) -> usize {
                Digits(self.0).write_digits10(buf, off)
            }
        }
    )+};

    // only for integers
    (common $($t:ty),+) => {$(
        impl FmtNum<$t> {
            /// Formats the number into a provided buffer and returns it as a byte slice.
            #[inline(always)]
            pub const fn as_bytes_into<'a>(&self, buf: &'a mut [u8]) -> &'a [u8] {
                let len = self.write(buf, 0);
                Slice::range_to(buf, len)
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// The string will be empty if there isn't enough space for the full number.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'buf>(&self, buf: &'buf mut [u8]) -> &'buf str {
                let len = self.write(buf, 0);
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
            pub const fn as_string<const N: usize>(&self) -> StringU8<N> {
                let mut buf = [0u8; N];
                let len = self.write(&mut buf, 0);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
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

        // For negative numbers the sign is written if it fits, and taken into account
        let len = FmtNum(-123i32).write(&mut buf, 0);
        assert_eq!(len, 1);
        assert_eq!(&buf, b"-\0");
    }
}
