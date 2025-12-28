// devela_base_core::text::fmt::num
//
//! Implemnts [`FmtNum`] for all floating-point primitives.
//

use super::{FmtNum, FmtNumShape};
use crate::{Digits, Float, Slice, Str, StringU8, is, whilst, write_at};

macro_rules! impl_fmtnum_float {
    () => {
        impl_fmtnum_float!(float f32|u64, f64|u128);
    };
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

            /* as_bytes */

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is writte.
            #[inline(always)]
            pub const fn as_bytes_into<'b>(&self, buf: &'b mut [u8], fract_len: u16) -> &'b [u8] {
                let len = self.write(buf, 0, fract_len); Slice::range_to(buf, len)
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
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'b>(&self, buf: &'b mut [u8], fract_len: u16) -> &'b str {
                let len = self.write(buf, 0, fract_len); Self::_as_str(Slice::range_to(buf, len))
            }
            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            pub const fn as_str_into_checked<'b>(&self, buf: &'b mut [u8], fract_len: u16)
            -> Option<&'b str> {
                let len = self.write(buf, 0, fract_len);
                if len == 0 { None } else { Some(Self::_as_str(Slice::range_to(buf, len))) }
            }

            /* as_string */

            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            #[inline(always)]
            pub const fn as_string<const N: usize>(&self, fract_len: u16) -> StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0, fract_len);
                StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            #[inline(always)]
            pub const fn as_string_checked<const N: usize>(&self, fract_len: u16)
                -> Option<StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0, fract_len);
                is![len == 0; None; Some(StringU8::<N>::_from_array_len_trusted(buf, len as u8))]
            }
        }
    )+ };
}
impl_fmtnum_float!();
