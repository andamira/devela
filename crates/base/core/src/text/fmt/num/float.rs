// devela_base_core::text::fmt::num
//
//! Implemnts [`FmtNum`] for all floating-point primitives.
//

__impl_fmt_num_float!(impl float f32|u64, f64|u128); // TEMP

///
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fmt_num_float {
    () => {};
    // $t: floating-point primitive
    // $u: unsigned primitive to cast the integral part as
    (impl float $($t:ty | $u:ty),+) => {$(
        impl $crate::FmtNum<$t> {
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
                use $crate::{Digits, Float, is, whilst, write_at};
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
                    let written = Digits(fract).write_digits10(buf, pos); // frac. digits
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
            pub const fn write_fmt(self, buf: &mut [u8], mut pos: usize, conf: $crate::FmtNumConf)
                -> usize {
                use $crate::{Cmp, Digits, Float, is, whilst, write_at};
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let emit_sign = conf.sign.emit_sign(neg);
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
                    let written = Digits(fract).write_digits10(buf, pos); // frac. digits
                    pos += written;
                    let missing = conf.fract as usize - written;
                    whilst! { _i in 0..missing; { write_at![buf, pos, b'0']; }}
                }
                needed
            }

            /// Writes the formatted floating-point number with optional digit grouping.
            ///
            /// Grouping is applied to the *rendered* digit sequences on both sides of the radix,
            /// after sign emission, zero-padding, and precision handling have been accounted for.
            ///
            /// # Invariants
            /// - Grouping assumes 1-byte separators and ASCII digits.
            /// - `conf.int` specifies the minimum width of the *entire* left block
            ///   (integral digits + zero padding + grouping separators), excluding the sign.
            /// - `conf.fract` specifies the fractional precision before grouping is applied
            ///   to the rendered fractional digit sequence.
            /// - On failure, nothing is written (atomic operation).
            pub const fn write_group(self, buf: &mut [u8], pos: usize,
                conf: $crate::FmtNumConf, group: $crate::FmtNumGroup) -> usize {
                use $crate::{Boundary1d, Digits, Float, is};
                // Fast path: grouping disabled on both sides.
                if (group.left_len == 0 || group.left_sep.is_none())
                    && (group.right_len == 0 || group.right_sep.is_none())
                {
                    return self.write_fmt(buf, pos, conf);
                }
                let f = Float(self.0);
                let neg = f.is_sign_negative();
                let fabs = is![neg; f.abs(); f];

                /* left (integral) side */

                // Integral magnitude and digit count.
                let integ = fabs.trunc().0 as u128;
                let integ_digits = Digits(integ).count_digits10() as u16;
                let emit_sign = conf.sign.emit_sign(neg);

                // Target minimum width of the left block (excluding sign unless pad_sign is set).
                let mut target_left = conf.int;
                if conf.pad_sign && emit_sign && target_left > 0 {
                    target_left -= 1;
                }
                // Compute minimal digit count so that: digits + grouping seps >= target_left.
                let left_digits = group.digits_for_grouped_width(Boundary1d::Left,
                    integ_digits, target_left);

                /* right (fractional) side */

                // Fractional digit count is fixed by configuration.
                let fract_digits = conf.fract;
                // Compute minimal digit count so that: digits + grouping seps >= target_right.
                let right_digits = group.digits_for_grouped_width(Boundary1d::Right,
                    fract_digits, fract_digits);

                // Adjusted configuration.
                let conf2 = $crate::FmtNumConf {
                    int: is![conf.pad_sign && emit_sign; left_digits + 1; left_digits],
                    fract: right_digits,
                    ..conf
                };

                // Measure final layout.
                let raw_shape = self.measure_fmt(conf2);
                let g_shape = self.measure_group(conf2, group);
                let (raw_len, final_len) = (raw_shape.total(), g_shape.total());
                if final_len > buf.len().saturating_sub(pos) { return 0; }

                // Write ungrouped.
                let written = self.write_fmt(buf, pos, conf2);
                if written == 0 { return 0; }

                /* Backward expansion */

                let mut src = pos + raw_len;
                let mut dst = pos + final_len;
                // Counters for both sides of the radix.
                let mut left_left = raw_shape.left;
                let mut right_left = raw_shape.right;
                let mut left_group = 0u16;
                let mut right_group = 0u16;
                let left_len = group.left_len as u16;
                let right_len = group.right_len as u16;
                let has_left = group.left_len != 0 && group.left_sep.is_some();
                let has_right = group.right_len != 0 && group.right_sep.is_some();
                while src > pos {
                    src -= 1;
                    dst -= 1;
                    let b = buf[src];
                    buf[dst] = b;
                    // Fractional side (right of radix)
                    if b >= b'0' && b <= b'9' && right_left > 0 {
                        right_left -= 1;
                        if has_right {
                            right_group += 1;
                            if right_left > 0 && right_group == right_len {
                                dst -= 1;
                                buf[dst] = crate::unwrap![some group.right_sep] as u8;
                                right_group = 0;
                            }
                        }
                        continue;
                    }
                    // Integral side (left of radix)
                    if b >= b'0' && b <= b'9' && left_left > 0 {
                        left_left -= 1;
                        if has_left {
                            left_group += 1;
                            if left_left > 0 && left_group == left_len {
                                dst -= 1;
                                buf[dst] = crate::unwrap![some group.left_sep] as u8;
                                left_group = 0;
                            }
                        }
                    }
                }
                final_len
            }

            /* measure */

            /// Returns the measured shape of the floating-point to be formatted.
            pub const fn measure(self, fract_len: u16) -> $crate::FmtNumShape {
                use $crate::{Digits, Float, is};
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let int_digits = Digits(fabs.trunc().0 as $u).count_digits10() as u16;
                $crate::FmtNumShape::new(neg as u16, int_digits, fract_len)
            }
            /// Returns the measured shape of the floating-point to be formatted,
            /// using the given formatting configuration.
            pub const fn measure_fmt(self, conf: $crate::FmtNumConf) -> $crate::FmtNumShape {
                use $crate::{Cmp, Digits, Float, is};
                let f = Float(self.0);
                let (neg, fabs) = is![f.is_sign_negative(); (true, f.abs()); (false, f)];
                let prefix = conf.sign.emit_sign(neg) as u16;
                let int_digits = Digits(fabs.trunc().0 as $u).count_digits10() as u16;
                let left = if conf.pad_sign && prefix > 0 { Cmp(int_digits + 1).max(conf.int) - 1 }
                else { Cmp(int_digits).max(conf.int) };
                $crate::FmtNumShape::new(prefix, left, conf.fract)
            }

            /// Returns the measured shape of the number after applying digit grouping.
            ///
            /// This first measures the formatted number using `measure_fmt`, then
            /// accounts for any additional separator glyphs introduced by grouping.
            ///
            /// Grouping assumes 1-byte separators and ASCII digits.
            pub const fn measure_group(self, conf: $crate::FmtNumConf, group: $crate::FmtNumGroup)
                -> $crate::FmtNumShape {
                let mut shape = self.measure_fmt(conf);
                if group.left_len > 0 && group.left_sep.is_some() && shape.left > 0 {
                    let groups = (shape.left.saturating_sub(1)) / group.left_len as u16;
                    shape.left += groups;
                }
                if group.right_len > 0 && group.right_sep.is_some() && shape.right > 0 {
                    let groups = (shape.right.saturating_sub(1)) / group.right_len as u16;
                    shape.right += groups;
                }
                shape
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> $crate::FmtNumShape {}

            /* as_bytes */

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is writte.
            #[inline(always)]
            pub const fn as_bytes_into<'b>(&self, buf: &'b mut [u8], fract_len: u16) -> &'b [u8] {
                let len = self.write(buf, 0, fract_len); $crate::Slice::range_to(buf, len)
            }
            /// Formats the number into a provided buffer and returns it as a byte slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is writte.
            #[inline(always)]
            pub const fn as_bytes_into_fmt<'b>(&self, buf: &'b mut [u8], conf: $crate::FmtNumConf)
                -> &'b [u8] {
                let len = self.write_fmt(buf, 0, conf); $crate::Slice::range_to(buf, len)
            }

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked<'b>(&self, buf: &'b mut [u8], fract_len: u16)
            -> Option<&'b [u8]> {
                let len = self.write(buf, 0, fract_len);
                if len == 0 { None } else { Some($crate::Slice::range_to(buf, len)) }
            }
            /// Formats the number into a provided buffer and returns it as a byte slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked_fmt<'b>(&self, buf: &'b mut [u8],
                conf: $crate::FmtNumConf) -> Option<&'b [u8]> {
                let len = self.write_fmt(buf, 0, conf);
                if len == 0 { None } else { Some($crate::Slice::range_to(buf, len)) }
            }

            /* as_str */

            #[inline(always)]
            const fn _as_str(slice: &[u8]) -> &str {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))] // safe
                return crate::unwrap![ok_guaranteed_or_ub $crate::Str::from_utf8(slice)];
                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))] // unsafe
                // SAFETY: the ASCII bytes are always valid utf-8
                unsafe { $crate::Str::from_utf8_unchecked(slice) }
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'b>(&self, buf: &'b mut [u8], fract_len: u16) -> &'b str {
                let len = self.write(buf, 0, fract_len);
                Self::_as_str($crate::Slice::range_to(buf, len))
            }
            /// Formats the number into a provided buffer and returns it as a string slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_fmt<'b>(&self, buf: &'b mut [u8],
                conf: $crate::FmtNumConf) -> &'b str {
                let len = self.write_fmt(buf, 0, conf);
                Self::_as_str($crate::Slice::range_to(buf, len))
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
                $crate::is![len == 0; None; Some(Self::_as_str($crate::Slice::range_to(buf, len)))]
            }
            /// Formats the number into a provided buffer and returns it as a string slice,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_checked_fmt<'b>(&self, buf: &'b mut [u8],
                conf: $crate::FmtNumConf) -> Option<&'b str> {
                let len = self.write_fmt(buf, 0, conf);
                $crate::is![len == 0; None; Some(Self::_as_str($crate::Slice::range_to(buf, len)))]
            }

            /* as_string */

            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string<const N: usize>(&self, fract_len: u16) -> $crate::StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0, fract_len);
                $crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string_fmt<const N: usize>(&self, conf: $crate::FmtNumConf)
                -> $crate::StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                $crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked<const N: usize>(&self, fract_len: u16)
                -> Option<$crate::StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0, fract_len);
                if len == 0 { None }
                else { Some($crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)) }
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// This operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked_fmt<const N: usize>(&self, conf: $crate::FmtNumConf)
                -> Option<$crate::StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                if len == 0 { None }
                else { Some($crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)) }
            }
        }
    )+ };
}
pub use __impl_fmt_num_float;
