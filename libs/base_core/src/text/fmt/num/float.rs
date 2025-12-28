// devela_base_core::text::fmt::num
//
//! Implemnts [`FmtNum`] for all floating-point primitives.
//

use super::{FmtNum, FmtNumConf as Conf, FmtNumShape as Shape, FmtNumSign as Sign};
use crate::{Cmp, Digits, Float, Slice, Str, StringU8, is, whilst, write_at};

macro_rules! impl_fmtnum_float {
    () => {
        impl_fmtnum_float!(float f32|u64, f64|u128);
    };
    // $t: floating-point primitive
    // $u: unsigned primitive to cast the integral part as
    (float $($t:ty | $u:ty),+) => {$(
        impl FmtNum<$t> {
            /* write */

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
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
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
                    pos += written;
                    let missing = fract_len as usize - written;
                    whilst! { _i in 0..missing; { write_at![buf, pos, b'0']; }}
                }
                needed
            }

            /// Writes the floating-point number in fixed-point decimal form into `buf`
            /// starting at `pos`, using the given formatting configuration.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// The operation is atomic: on failure, nothing is written.
            ///
            /// The emitted sign, any leading zero-padding and the number of fractional digits
            /// are controlled by `conf`.
            pub const fn write_fmt(self, buf: &mut [u8], mut pos: usize, conf: Conf) -> usize {
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let emit_sign = match conf.sign {
                    Sign::NegativeOnly => neg,
                    Sign::Always => true,
                    Sign::Never => false,
                    Sign::PositiveOnly => !neg,
                };
                // digit count of integral part
                let integ = fabs.trunc().0 as $u;
                let digit_count = Digits(integ).count_digits10() as u16;
                let left_digits = if conf.pad_sign && emit_sign {
                    Cmp(digit_count + 1).max(conf.int) - 1
                } else {
                    Cmp(digit_count).max(conf.int)
                };
                // compute required space
                let (sign_len, dot_len) = (emit_sign as usize, (conf.fract > 0) as usize);
                let needed = sign_len + (left_digits as usize) + dot_len + (conf.fract as usize);
                // emit sign, zero padding, int digits, dot and fract digits
                if emit_sign { write_at![buf, pos, is![neg; b'-'; b'+']]; } // ←sign ↓zero-padding
                whilst! { _i in 0..(left_digits - digit_count); { write_at![buf, pos, b'0']; }}
                pos += Digits(integ).write_digits10(buf, pos); // integral digits
                if conf.fract > 0 {
                    write_at![buf, pos, b'.']; // dot
                    let multiplier = Float(10.0 as $t).powi(conf.fract as i32).0;
                    let fract = (fabs.fract().0 * multiplier) as $u;
                    let written = Digits(fract).write_digits10(buf, pos); // fractional digits
                    pos += written;
                    let missing = conf.fract as usize - written;
                    whilst! { _i in 0..missing; { write_at![buf, pos, b'0']; }}
                }
                needed
            }

            /* measure */

            /// Returns the measured shape of the floating-point to be formatted.
            pub const fn measure(self, fract_len: u16) -> Shape {
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let int_digits = Digits(fabs.trunc().0 as $u).count_digits10() as u16;
                Shape::new(neg as u16, int_digits, fract_len)
            }
            /// Returns the measured shape of the floating-point to be formatted,
            /// using the given formatting configuration.
            pub const fn measure_fmt(self, conf: Conf) -> Shape {
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let prefix = match conf.sign {
                    Sign::NegativeOnly => neg as u16,
                    Sign::Always => 1,
                    Sign::Never => 0,
                    Sign::PositiveOnly => (!neg) as u16,
                };
                let int_digits = Digits(fabs.trunc().0 as $u).count_digits10() as u16;
                let left = if conf.pad_sign && prefix > 0 { Cmp(int_digits + 1).max(conf.int) - 1 }
                else { Cmp(int_digits).max(conf.int) };
                Shape::new(prefix, left, conf.fract)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> Shape {}

            /* as_bytes */

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is writte.
            #[inline(always)]
            pub const fn as_bytes_into<'b>(&self, buf: &'b mut [u8], fract_len: u16) -> &'b [u8] {
                let len = self.write(buf, 0, fract_len); Slice::range_to(buf, len)
            }
            /// Formats the number into a provided buffer and returns it as a byte slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is writte.
            #[inline(always)]
            pub const fn as_bytes_into_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf) -> &'b [u8] {
                let len = self.write_fmt(buf, 0, conf); Slice::range_to(buf, len)
            }

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked<'b>(&self, buf: &'b mut [u8], fract_len: u16)
            -> Option<&'b [u8]> {
                let len = self.write(buf, 0, fract_len);
                if len == 0 { None } else { Some(Slice::range_to(buf, len)) }
            }
            /// Formats the number into a provided buffer and returns it as a byte slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf)
            -> Option<&'b [u8]> {
                let len = self.write_fmt(buf, 0, conf);
                if len == 0 { None } else { Some(Slice::range_to(buf, len)) }
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
            /// This operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'b>(&self, buf: &'b mut [u8], fract_len: u16) -> &'b str {
                let len = self.write(buf, 0, fract_len); Self::_as_str(Slice::range_to(buf, len))
            }
            /// Formats the number into a provided buffer and returns it as a string slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_fmt<'b>(&self, buf: &'b mut [u8], conf: Conf) -> &'b str {
                let len = self.write_fmt(buf, 0, conf); Self::_as_str(Slice::range_to(buf, len))
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_checked<'b>(&self, buf: &'b mut [u8], fract_len: u16)
            -> Option<&'b str> {
                let len = self.write(buf, 0, fract_len);
                if len == 0 { None } else { Some(Self::_as_str(Slice::range_to(buf, len))) }
            }
            /// Formats the number into a provided buffer and returns it as a string slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
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
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string<const N: usize>(&self, fract_len: u16) -> StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0, fract_len);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string_fmt<const N: usize>(&self, conf: Conf) -> StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked<const N: usize>(&self, fract_len: u16)
                -> Option<StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0, fract_len);
                is![len == 0; None; Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))]
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked_fmt<const N: usize>(&self, conf: Conf)
                -> Option<StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                is![len == 0; None; Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))]
            }
        }
    )+ };
}
impl_fmtnum_float!();
