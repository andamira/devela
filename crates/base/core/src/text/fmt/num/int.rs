// devela_base_core::text::fmt::num::int
//
//! Implements [`FmtNum`] for all integer primitives.
//

__impl_fmt_num_int!(signed i8, i16, i32, i64, i128, isize); // TEMP
__impl_fmt_num_int!(unsigned u8, u16, u32, u64, u128, usize); // TEMP

///
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fmt_num_int {
    () => {};
    (signed $($t:ty),+) => {$(
        impl $crate::FmtNum<$t> {
            /* write*/

            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// # Invariants
            /// - Negative values are preceded by the `'-'` sign.
            /// - The operation is atomic: on failure, nothing is written.
            #[rustfmt::skip]
            pub const fn write(self, buf: &mut [u8], mut pos: usize) -> usize {
                use $crate::{Digits, write_at};
                if self.0 < 0 {
                    let digits = Digits(self.0.wrapping_neg().cast_unsigned()); // abs
                    let needed = digits.count_digits10() as usize + 1;
                    if needed > buf.len().saturating_sub(pos) { return 0; }
                    write_at![buf, +=pos, b'-'];
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
            /// # Invariants
            /// - The emitted sign and any leading zero-padding are controlled by `conf`.
            /// - The operation is atomic: on failure, nothing is written.
            #[rustfmt::skip]
            pub const fn write_fmt(self, buf: &mut [u8], mut pos: usize, conf: $crate::FmtNumConf)
                -> usize {
                use $crate::{Cmp, Digits, is, whilst, write_at};
                let neg = self.0 < 0;
                let emit_sign = conf.sign.emit_sign(neg);
                let abs = is![neg; self.0.wrapping_neg().cast_unsigned(); self.0.cast_unsigned()];
                // digit count
                let digit_count = Digits(abs).count_digits10() as u16;
                let left_digits = if conf.pad_sign && emit_sign {
                    Cmp(digit_count + 1).max(conf.int) - 1
                } else {
                    Cmp(digit_count).max(conf.int)
                };
                // compute required space
                let needed = (emit_sign as usize) + left_digits as usize;
                if needed > buf.len().saturating_sub(pos) { return 0; }
                // emit sign, zero padding and digits
                if emit_sign { write_at![buf, +=pos, is![neg; b'-'; b'+']]; }
                whilst! { _i in 0..(left_digits - digit_count); { write_at![buf, +=pos, b'0']; }}
                let _ = Digits(abs).write_digits10(buf, pos);
                needed
            }

            /// Writes the formatted integer with optional digit grouping.
            ///
            /// Grouping is applied to the *rendered* integral digit sequence,
            /// after sign emission and zero-padding have been accounted for.
            ///
            /// # Invariants
            /// - Grouping assumes 1-byte separators and ASCII digits.
            /// - `conf.int` specifies the minimum width of the *entire* left block
            ///   (digits + zero padding + grouping separators), excluding the sign.
            /// - On failure, nothing is written (atomic operation).
            pub const fn write_group(self, buf: &mut [u8], pos: usize, conf: $crate::FmtNumConf,
                group: $crate::FmtNumGroup) -> usize {
                use $crate::{Boundary1d, Digits, is, unwrap};
                // Fast path: grouping disabled.
                if !group.has_left() { return self.write_fmt(buf, pos, conf); }

                // Determine whether a sign is emitted.
                let neg = self.0 < 0;
                let emit_sign = conf.sign.emit_sign(neg);

                // Absolute value and digit count of the numeric magnitude.
                let abs = is![neg; self.0.wrapping_neg().cast_unsigned(); self.0.cast_unsigned()];
                let digit_count = Digits(abs).count_digits10() as u16;

                // Target minimum width of the left block (excluding sign unless padded).
                let mut target_left = conf.int;
                if conf.pad_sign && emit_sign && target_left > 0 { target_left -= 1; }

                // Compute minimal digit count so that: digits + grouping separators >= target_left.
                let len = group.left_len as u16;
                let left_digits = group.digits_for_grouped_width(Boundary1d::Left,
                    digit_count, target_left);

                // Translate back to write_fmt semantics.
                let conf2_int = is![conf.pad_sign && emit_sign; left_digits + 1; left_digits];
                let conf2 = conf.with_int(conf2_int);

                // Measure final layout.
                let raw_shape = self.measure_fmt(conf2);
                let g_shape = self.measure_group(conf2, group);
                let (raw_len, final_len) = (raw_shape.total(), g_shape.total());
                if final_len > buf.len().saturating_sub(pos) { return 0; }

                // Write ungrouped number at the start.
                let written = self.write_fmt(buf, pos, conf2);
                is![written == 0; return 0];

                // Backward expansion: insert grouping separators in place.
                let (mut src, mut dst) = (pos + raw_len, pos + final_len);
                let (mut digits_left, mut group_count) = (raw_shape.left, 0u16);
                while src > pos {
                    src -= 1;
                    dst -= 1;
                    let b = buf[src];
                    buf[dst] = b;
                    // Only count digits belonging to the numeric magnitude.
                    if b >= b'0' && b <= b'9' && digits_left > 0 {
                        digits_left -= 1;
                        group_count += 1;
                        if digits_left > 0 && group_count == len {
                            dst -= 1;
                            buf[dst] = unwrap![some group.left_sep] as u8;
                            group_count = 0;
                        }
                    }
                }
                final_len
            }

            /* measure */

            /// Returns the measured shape of the integer to be formatted.
            pub const fn measure(self) -> $crate::FmtNumShape {
                use $crate::Digits;
                let (prefix, left) = if self.0 < 0 {
                    (1, Digits(self.0.wrapping_neg().cast_unsigned()).count_digits10() as u16)
                } else {
                    (0, Digits(self.0.cast_unsigned()).count_digits10() as u16)
                };
                $crate::FmtNumShape::new(prefix, left, 0)
            }
            /// Returns the measured shape of the integer
            /// when formatted with the given configuration.
            pub const fn measure_fmt(self, conf: $crate::FmtNumConf) -> $crate::FmtNumShape {
                use $crate::{Cmp, Digits, is};
                let neg = self.0 < 0;
                let prefix = conf.sign.emit_sign(neg) as u16;
                let abs = is![neg; self.0.wrapping_neg().cast_unsigned(); self.0.cast_unsigned()];
                let digits = Digits(abs).count_digits10() as u16;
                let left = if conf.pad_sign && prefix > 0 { Cmp(digits + 1).max(conf.int) - 1 }
                else { Cmp(digits).max(conf.int) };
                $crate::FmtNumShape::new(prefix, left, 0)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> $crate::FmtNumShape {}
        }
        __impl_fmt_num_int!(common $t);
    )+};
    (unsigned $($t:ty),+) => {$(
        impl $crate::FmtNum<$t> {
            /* write */

            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// # Invariants
            /// The operation is atomic: on failure, nothing is written.
            #[inline(always)]
            pub const fn write(self, buf: &mut [u8], pos: usize) -> usize {
                let digits = $crate::Digits(self.0);
                let needed = digits.count_digits10() as usize;
                if needed > buf.len().saturating_sub(pos) { return 0; }
                digits.write_digits10(buf, pos)
            }
            /// Writes the integer as ASCII decimal digits into `buf` starting at `pos`,
            /// using the given formatting configuration.
            ///
            /// Returns the number of bytes written, or `0` if the buffer is too small.
            ///
            /// # Invariants
            /// The emitted sign and any leading zero-padding are controlled by `conf`.
            /// The operation is atomic: on failure, nothing is written.
            pub const fn write_fmt(self, buf: &mut [u8], mut pos: usize, conf: $crate::FmtNumConf)
                -> usize {
                use $crate::{Cmp, Digits, FmtNumSign as Sign, whilst, write_at};
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
                if emit_sign { write_at![buf, +=pos, b'+']; }
                whilst! { _i in 0..(left_digits - digit_count); { write_at![buf, +=pos, b'0']; }}
                let _ = Digits(self.0).write_digits10(buf, pos);
                needed
            }

            /// Writes the formatted integer with optional digit grouping.
            ///
            /// Grouping is applied to the *rendered* integral digit sequence,
            /// after sign emission and zero-padding have been accounted for.
            ///
            /// # Invariants
            /// - Grouping assumes 1-byte separators and ASCII digits.
            /// - `conf.int` specifies the minimum width of the *entire* left block
            ///   (digits + zero padding + grouping separators), excluding the sign.
            /// - On failure, nothing is written (atomic operation).
            pub const fn write_group(self, buf: &mut [u8], pos: usize, conf: $crate::FmtNumConf,
                group: $crate::FmtNumGroup) -> usize {
                use $crate::{Boundary1d, Digits, is, unwrap};
                // Fast path: grouping disabled.
                if !group.has_left() { return self.write_fmt(buf, pos, conf); }

                // Digit count of the numeric magnitude.
                let digit_count = Digits(self.0).count_digits10() as u16;

                // Target minimum width of the left block.
                let target_left = conf.int;

                // Compute minimal digit count so that: digits + grouping separators >= target_left.
                let len = group.left_len as u16;
                let left_digits = group.digits_for_grouped_width(Boundary1d::Left,
                    digit_count, target_left);

                // Adjust formatting configuration.
                let conf2 = conf.with_int(left_digits);

                // Measure final layout.
                let raw_shape = self.measure_fmt(conf2);
                let g_shape = self.measure_group(conf2, group);
                let (raw_len, final_len) = (raw_shape.total(), g_shape.total());
                if final_len > buf.len().saturating_sub(pos) { return 0; }

                // Write ungrouped number at the start.
                let written = self.write_fmt(buf, pos, conf2);
                is![written == 0; return 0];

                // Backward expansion: insert grouping separators in place.
                let (mut src, mut dst) = (pos + raw_len, pos + final_len);
                let (mut digits_left, mut group_count) = (raw_shape.left, 0u16);
                while src > pos {
                    src -= 1;
                    dst -= 1;
                    let b = buf[src];
                    buf[dst] = b;
                    // Only count digits belonging to the numeric magnitude.
                    if b >= b'0' && b <= b'9' && digits_left > 0 {
                        digits_left -= 1;
                        group_count += 1;
                        if digits_left > 0 && group_count == len {
                            dst -= 1;
                            buf[dst] = unwrap![some group.left_sep] as u8;
                            group_count = 0;
                        }
                    }
                }
                final_len
            }

            /* measure */

            /// Returns the measured shape of the integer to be formatted.
            pub const fn measure(self) -> $crate::FmtNumShape {
                let left = $crate::Digits(self.0).count_digits10() as u16;
                $crate::FmtNumShape::new(0, left, 0)
            }
            /// Returns the measured shape of the integer to be formatted,
            /// using the given formatting configuration.
            pub const fn measure_fmt(self, conf: $crate::FmtNumConf) -> $crate::FmtNumShape {
                use $crate::{Cmp, Digits, FmtNumSign as Sign};
                let prefix = match conf.sign { Sign::Always | Sign::PositiveOnly => 1, _ => 0 };
                let digits = Digits(self.0).count_digits10() as u16;
                let left = Cmp(digits).max(conf.int);
                $crate::FmtNumShape::new(prefix, left, 0)
            }

            // TODO
            // pub const fn write16(self, buf: &mut [u8], pos: usize) -> usize {}
            // pub const fn measure16(self) -> $crate::FmtNumShape {}
        }
        __impl_fmt_num_int!(common $t);
    )+};
    (common $($t:ty),+) => {$(
        impl $crate::FmtNum<$t> {
            /* measure */

            /// Returns the measured shape of the number after applying digit grouping.
            ///
            /// This first measures the formatted number using `measure_fmt`, then
            /// accounts for any additional separator glyphs introduced by grouping.
            pub const fn measure_group(self, conf: $crate::FmtNumConf, group: $crate::FmtNumGroup)
                -> $crate::FmtNumShape {
                let mut shape = self.measure_fmt(conf);
                if shape.left > 0 && group.left_len != 0 && group.left_sep.is_some() {
                    let groups = (shape.left.saturating_sub(1)) / group.left_len as u16;
                    shape.left += groups;
                }
                shape
            }

            /* as_bytes */

            /// Formats the number into a provided buffer and returns it as a byte slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            #[inline(always)]
            pub const fn as_bytes_into<'b>(&self, buf: &'b mut [u8]) -> &'b [u8] {
                let len = self.write(buf, 0); $crate::Slice::range_to(buf, len)
            }
            /// Formats the number into a provided buffer and returns it as a byte slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            #[inline(always)]
            pub const fn as_bytes_into_fmt<'b>(&self, buf: &'b mut [u8], conf: $crate::FmtNumConf)
                -> &'b [u8] {
                let len = self.write_fmt(buf, 0, conf); $crate::Slice::range_to(buf, len)
            }

            /// Formats the number into a provided buffer and returns it as some byte slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked<'b>(&self, buf: &'b mut [u8]) -> Option<&'b [u8]> {
                let len = self.write(buf, 0);
                $crate::is![len == 0; None; Some($crate::Slice::range_to(buf, len))]
            }
            /// Formats the number into a provided buffer and returns it as some byte slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written
            /// and it returns `None`.
            #[inline(always)]
            pub const fn as_bytes_into_checked_fmt<'b>(&self, buf: &'b mut [u8],
                conf: $crate::FmtNumConf) -> Option<&'b [u8]> {
                let len = self.write_fmt(buf, 0, conf);
                $crate::is![len == 0; None; Some($crate::Slice::range_to(buf, len))]
            }

            /* as_str */

            #[inline(always)]
            const fn _as_str(slice: &[u8]) -> &str {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))] // safe
                return $crate::unwrap![ok_guaranteed_or_ub $crate::Str::from_utf8(slice)];
                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))] // unsafe
                // SAFETY: the ASCII bytes are always valid utf-8
                unsafe { $crate::Str::from_utf8_unchecked(slice) }
            }

            /// Formats the number into a provided buffer and returns it as a string slice.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into<'b>(&self, buf: &'b mut [u8]) -> &'b str {
                let len = self.write(buf, 0);
                Self::_as_str($crate::Slice::range_to(buf, len))
            }
            /// Formats the number into a provided buffer and returns it as a string slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_fmt<'b>(&self, buf: &'b mut [u8], conf: $crate::FmtNumConf)
                -> &'b str {
                let len = self.write_fmt(buf, 0, conf);
                Self::_as_str($crate::Slice::range_to(buf, len))
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
                if len == 0 { None } else { Some(Self::_as_str($crate::Slice::range_to(buf, len))) }
            }
            /// Formats the number into a provided buffer and returns it as some string slice,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, nothing is written,
            /// and it returns `None`.
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[inline(always)]
            pub const fn as_str_into_checked_fmt<'b>(&self, buf: &'b mut [u8],
                conf: $crate::FmtNumConf) -> Option<&'b str> {
                let len = self.write_fmt(buf, 0, conf);
                if len == 0 { None } else { Some(Self::_as_str($crate::Slice::range_to(buf, len))) }
            }

            /* as_string */

            /// Converts the number into an owned fixed-size string.
            ///
            /// The operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string<const N: usize>(&self) -> $crate::StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0);
                $crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, it returns an empty string.
            pub const fn as_string_fmt<const N: usize>(&self, conf: $crate::FmtNumConf)
                -> $crate::StringU8<N> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                $crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)
            }

            /// Converts the number into an owned fixed-size string.
            ///
            /// The operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked<const N: usize>(&self) -> Option<$crate::StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write(&mut buf, 0);
                if len == 0 { None }
                else { Some($crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)) }
            }
            /// Converts the number into an owned fixed-size string,
            /// using the given formatting configuration.
            ///
            /// The operation is atomic: if the buffer is too small, it returns `None`.
            pub const fn as_string_checked_fmt<const N: usize>(&self, conf: $crate::FmtNumConf)
                -> Option<$crate::StringU8<N>> {
                let mut buf = [0u8; N]; let len = self.write_fmt(&mut buf, 0, conf);
                if len == 0 { None }
                else { Some($crate::StringU8::<N>::_from_array_len_trusted(buf, len as u8)) }
            }
        }
    )+ };
}
pub use __impl_fmt_num_int;
