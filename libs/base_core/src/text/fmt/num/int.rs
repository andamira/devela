// devela_base_core::text::fmt::num::int
//
//! Implements [`FmtNum`] for all integer primitives.
//

use super::{FmtNum, FmtNumConf as Conf, FmtNumShape as Shape, FmtNumSign as Sign};
use crate::{Cmp, Digits, Slice, Str, StringU8, is, whilst, write_at};

macro_rules! impl_fmtnum_int {
    () => {
        impl_fmtnum_int!(signed i8, i16, i32, i64, i128, isize);
        impl_fmtnum_int!(unsigned u8, u16, u32, u64, u128, usize);
    };
    (signed $($t:ty),+) => {$(
        impl FmtNum<$t> {
            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            /// Negative values are preceded by the `'-'` sign.
            #[rustfmt::skip]
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

            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`,
            /// using the given formatting configuration.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            ///
            /// The emitted sign and any leading zero-padding are controlled by `conf`.
            #[rustfmt::skip]
            pub const fn write_fmt(self, buf: &mut [u8], mut pos: usize, conf: Conf) -> usize {
                let neg = self.0 < 0;
                let emit_sign = match conf.sign {
                    Sign::NegativeOnly => neg,
                    Sign::Always => true,
                    Sign::Never => false,
                    Sign::PositiveOnly => !neg,
                };
                let abs = is![neg; self.0.wrapping_neg().cast_unsigned(); self.0.cast_unsigned()];
                // digit count
                let digit_count = Digits(abs).count_digits10() as u16;
                let left_digits = Cmp(digit_count).max(conf.int);
                // compute required space
                let needed = (emit_sign as usize) + left_digits as usize;
                if needed > buf.len().saturating_sub(pos) { return 0; }
                // emit sign, zero padding and digits
                if emit_sign { write_at![buf, pos, is![neg; b'-'; b'+']]; }
                whilst! { _i in 0..(left_digits - digit_count); { write_at![buf, pos, b'0']; }}
                let _ = Digits(abs).write_digits10(buf, pos);
                needed
            }

            /// Returns the measured shape of the integer to be formatted.
            pub const fn measure(self) -> Shape {
                let (prefix, left) = if self.0 < 0 {
                    (1, Digits(self.0.wrapping_neg().cast_unsigned()).count_digits10() as u16)
                } else {
                    (0, Digits(self.0.cast_unsigned()).count_digits10() as u16)
                };
                Shape::new(prefix, left, 0)
            }

            /// Returns the measured shape of the number
            /// when formatted with the given configuration.
            pub const fn measure_fmt(self, conf: Conf) -> Shape {
                let neg = self.0 < 0;
                let prefix = match conf.sign {
                    Sign::NegativeOnly => neg as u16,
                    Sign::Always => 1,
                    Sign::Never => 0,
                    Sign::PositiveOnly => (!neg) as u16,
                };
                let abs = is![neg; self.0.wrapping_neg().cast_unsigned(); self.0.cast_unsigned()];
                let digits = Digits(abs).count_digits10() as u16;
                let left = Cmp(digits).max(conf.int);
                Shape::new(prefix, left, 0)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> Shape {}
        }
        impl_fmtnum_int!(common $t);
    )+};
    (unsigned $($t:ty),+) => {$(
        impl FmtNum<$t> {
            /* write */

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
            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`,
            /// using the given formatting configuration.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            ///
            /// The emitted sign and any leading zero-padding are controlled by `conf`.
            pub const fn write_fmt(self, buf: &mut [u8], mut pos: usize, conf: Conf) -> usize {
                let emit_sign = match conf.sign {
                    Sign::Always | Sign::PositiveOnly => true,
                    _ => false
                };
                // digit count
                let digit_count = Digits(self.0).count_digits10() as u16;
                let left_digits = Cmp(digit_count).max(conf.int);
                // compute required space
                let needed = (emit_sign as usize) + left_digits as usize;
                if needed > buf.len().saturating_sub(pos) { return 0; }
                // emit sign, zero padding and digits
                if emit_sign { write_at![buf, pos, b'+']; }
                whilst! { _i in 0..(left_digits - digit_count); { write_at![buf, pos, b'0']; }}
                let _ = Digits(self.0).write_digits10(buf, pos);
                needed
            }

            /* measure */

            /// Returns the measured shape of the integer to be formatted.
            pub const fn measure(self) -> Shape {
                let left = Digits(self.0).count_digits10() as u16;
                Shape::new(0, left, 0)
            }
            /// Returns the measured shape of the integer to be formatted,
            /// using the given formatting configuration.
            pub const fn measure_fmt(self, conf: Conf) -> Shape {
                let prefix = match conf.sign { Sign::Always | Sign::PositiveOnly => 1, _ => 0 };
                let digits = Digits(self.0).count_digits10() as u16;
                let left = Cmp(digits).max(conf.int);
                Shape::new(prefix, left, 0)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> Shape {}
        }
        impl_fmtnum_int!(common $t);
    )+};
    (common $($t:ty),+) => {$(
        impl FmtNum<$t> {
            /* as_bytes */

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            #[inline(always)]
            pub const fn as_bytes_into<'b>(&self, buf: &'b mut [u8]) -> &'b [u8] {
                let len = self.write(buf, 0); Slice::range_to(buf, len)
            }
            /// Formats the number into a provided buffer and returns it as a byte slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            #[inline(always)]
            pub const fn as_bytes_into_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf) -> &'b [u8] {
                let len = self.write_fmt(buf, 0, conf); Slice::range_to(buf, len)
            }

            /// Formats the number into a provided buffer and returns it as some byte slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked<'b>(&self, buf: &'b mut [u8]) -> Option<&'b [u8]> {
                let len = self.write(buf, 0);
                is![len == 0; None; Some(Slice::range_to(buf, len))]
            }
            /// Formats the number into a provided buffer and returns it as some byte slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf)
                -> Option<&'b [u8]> {
                let len = self.write_fmt(buf, 0, conf);
                is![len == 0; None; Some(Slice::range_to(buf, len))]
            }

            /* as_str */

            #[inline(always)]
            const fn _as_str(slice: &[u8]) -> &str {
                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))] // safe
                return crate::unwrap![ok_guaranteed_or_ub Str::from_utf8(slice)];
                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))] // unsafe
                // SAFETY: the ASCII bytes are always valid utf-8
                unsafe { Str::from_utf8_unchecked(slice) }
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'b>(&self, buf: &'b mut [u8]) -> &'b str {
                let len = self.write(buf, 0); Self::_as_str(Slice::range_to(buf, len))
            }
            /// Formats the number into a provided buffer and returns it as a string slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf) -> &'b str {
                let len = self.write_fmt(buf, 0, conf); Self::_as_str(Slice::range_to(buf, len))
            }

            /// Formats the number into a provided buffer and returns it as some string slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_checked<'b>(&self, buf: &'b mut [u8]) -> Option<&'b str> {
                let len = self.write(buf, 0);
                if len == 0 { None } else { Some(Self::_as_str(Slice::range_to(buf, len))) }
            }
            /// Formats the number into a provided buffer and returns it as some string slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written,
            /// and it returns `None`.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_checked_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf)
                -> Option<&'b str> {
                let len = self.write_fmt(buf, 0, conf);
                if len == 0 { None } else { Some(Self::_as_str(Slice::range_to(buf, len))) }
            }

            /* as_string */

            /// Converts the number into an owned fixed-size string.
            ///
            /// The operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string<const N: usize>(&self) -> StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string_fmt<const N: usize>(&self, conf: Conf) -> StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// The operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked<const N: usize>(&self) -> Option<StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0);
                is![len == 0; None; Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))]
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked_fmt<const N: usize>(&self, conf: Conf)
                -> Option<StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                is![len == 0; None; Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))]
            }
        }
    )+ };
}
impl_fmtnum_int!();
