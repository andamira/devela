// devela_base_core::text::char::digits::u128

use super::*;
use crate::{Cmp, Lut, StringU8, is};

impl Digits<u128> {
    /// The maximum number of decimal digits a `u128` can represent.
    pub const MAX_DIGITS_10: u8 = 39;

    /// The maximum number of hexadecimal digits a `u128` can represent.
    pub const MAX_DIGITS_16: u8 = 32;

    /* digit_at_ */

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::Digits;
    /// assert_eq![1, Digits(0_u128).count_digits10()];
    /// assert_eq![19, Digits(9876543210987654321_u128).count_digits10()];
    /// ```
    #[must_use]
    pub const fn count_digits10(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    #[doc = DOC_COUNT_DIGITS_16!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    pub const fn count_digits16(self) -> u8 {
        is![self.0 == 0; 1; ((self.0.ilog2() + 4) / 4) as u8]
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= self.count_digits10(); return b'0'];
        let power = Lut::POWERS10[index as usize];
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits10(); return None];
        let power = Lut::POWERS10[index as usize];
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Lut::DIGITS_BASE36[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits16(); return None];
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Some(Lut::DIGITS_BASE36[digit])
    }

    /* digit_value_at_ */

    /// Returns the numeric value (0-9) of the decimal digit at the specified index.
    ///
    /// Returns `0` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_value_at_index10(self, index: u8) -> u8 {
        is![index >= self.count_digits10(); return 0];
        let power = Lut::POWERS10[index as usize];
        (self.0 / power % 10) as u8
    }
    /// Returns `Some(numeric_value)` (0-9) of the decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_value_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits10(); return None];
        let power = Lut::POWERS10[index as usize];
        Some((self.0 / power % 10) as u8)
    }

    /// Returns the numeric value (0-15) of the hexadecimal digit at the specified index.
    ///
    /// Returns `0` if the index is beyond the number's hexadecimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_value_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        (self.0.unbounded_shr(shift) & 0xF) as u8
    }
    /// Returns `Some(numeric_value)` (0-15) if the index is within bounds.
    ///
    /// Returns `None` if the index is beyond the number's hexadecimal digits.
    #[must_use]
    pub const fn digit_value_at_index16_checked(self, index: u8) -> Option<u8> {
        is![index < self.count_digits16(); Some(self.digit_value_at_index16(index)); None]
    }

    //

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    #[must_use]
    pub(crate) const fn digit_at_power10(self, divisor: u128) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub(crate) const fn digit_at_power16(self, divisor: u128) -> u8 {
        // only optimize the most common cases
        let digit = match divisor {
            0x1 => self.0 & 0xF,
            0x10 => (self.0 >> 4) & 0xF,
            0x100 => (self.0 >> 8) & 0xF,
            0x1000 => (self.0 >> 12) & 0xF,
            0x10000 => (self.0 >> 16) & 0xF,
            0x100000 => (self.0 >> 20) & 0xF,
            0x1000000 => (self.0 >> 24) & 0xF,
            0x10000000 => (self.0 >> 28) & 0xF,
            _ => (self.0 / divisor) % 16,
        };
        Lut::DIGITS_BASE36[digit as usize]
    }

    /// Converts a `u128` into a byte array of `39` ASCII decimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        [
            //                    987654321_987654321_987654321_987654321
            //                    340282366920938463463374607431768211455    ← u128::MAX
            self.digit_at_power10(100000000000000000000000000000000000000), // 39 digits
            self.digit_at_power10(10000000000000000000000000000000000000),
            self.digit_at_power10(1000000000000000000000000000000000000),
            self.digit_at_power10(100000000000000000000000000000000000),
            self.digit_at_power10(10000000000000000000000000000000000),
            self.digit_at_power10(1000000000000000000000000000000000),
            self.digit_at_power10(100000000000000000000000000000000),
            self.digit_at_power10(10000000000000000000000000000000),
            self.digit_at_power10(1000000000000000000000000000000),
            self.digit_at_power10(100000000000000000000000000000), // 30 digits
            self.digit_at_power10(10000000000000000000000000000),
            self.digit_at_power10(1000000000000000000000000000),
            self.digit_at_power10(100000000000000000000000000),
            self.digit_at_power10(10000000000000000000000000),
            self.digit_at_power10(1000000000000000000000000),
            self.digit_at_power10(100000000000000000000000),
            self.digit_at_power10(10000000000000000000000),
            self.digit_at_power10(1000000000000000000000),
            self.digit_at_power10(100000000000000000000),
            self.digit_at_power10(10000000000000000000), // 20 digits
            self.digit_at_power10(1000000000000000000),
            self.digit_at_power10(100000000000000000),
            self.digit_at_power10(10000000000000000),
            self.digit_at_power10(1000000000000000),
            self.digit_at_power10(100000000000000),
            self.digit_at_power10(10000000000000),
            self.digit_at_power10(1000000000000),
            self.digit_at_power10(100000000000),
            self.digit_at_power10(10000000000),
            self.digit_at_power10(1000000000), // 10 digits
            self.digit_at_power10(100000000),
            self.digit_at_power10(10000000),
            self.digit_at_power10(1000000),
            self.digit_at_power10(100000),
            self.digit_at_power10(10000),
            self.digit_at_power10(1000),
            self.digit_at_power10(100),
            self.digit_at_power10(10),
            self.digit_at_power10(1),
        ]
    }
    /// Converts a `u128` into a byte array of `32` ASCII hexadecimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[allow(clippy::unreadable_literal)]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        [
            //                      0FEDCBA9876543210FEDCBA987654321
            //                      FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF    ← u128::MAX
            self.digit_at_power16(0x10000000000000000000000000000000), // 32 digits
            self.digit_at_power16(0x1000000000000000000000000000000),
            self.digit_at_power16(0x100000000000000000000000000000),
            self.digit_at_power16(0x10000000000000000000000000000),
            self.digit_at_power16(0x1000000000000000000000000000),
            self.digit_at_power16(0x100000000000000000000000000),
            self.digit_at_power16(0x10000000000000000000000000),
            self.digit_at_power16(0x1000000000000000000000000),
            self.digit_at_power16(0x100000000000000000000000),
            self.digit_at_power16(0x10000000000000000000000),
            self.digit_at_power16(0x1000000000000000000000),
            self.digit_at_power16(0x100000000000000000000),
            self.digit_at_power16(0x10000000000000000000),
            self.digit_at_power16(0x1000000000000000000),
            self.digit_at_power16(0x100000000000000000),
            self.digit_at_power16(0x10000000000000000),
            self.digit_at_power16(0x1000000000000000),
            self.digit_at_power16(0x100000000000000),
            self.digit_at_power16(0x10000000000000),
            self.digit_at_power16(0x1000000000000),
            self.digit_at_power16(0x100000000000),
            self.digit_at_power16(0x10000000000),
            self.digit_at_power16(0x1000000000),
            self.digit_at_power16(0x100000000),
            self.digit_at_power16(0x10000000),
            self.digit_at_power16(0x1000000),
            self.digit_at_power16(0x100000),
            self.digit_at_power16(0x10000),
            self.digit_at_power16(0x1000),
            self.digit_at_power16(0x100),
            self.digit_at_power16(0x10),
            self.digit_at_power16(0x1),
        ]
    }

    #[doc = DOC_WRITE_DIGITS_10!(39)]
    pub const fn write_digits10(self, buf: &mut [u8], offset: usize) -> usize {
        let mut n = self.0;
        if n == 0 {
            is![offset < buf.len(); { buf[offset] = b'0'; return 1 }];
            return 0;
        }
        let digits = n.ilog10() as usize + 1;
        is![offset + digits > buf.len(); return 0];
        // builds the digits in reverse using the LUT
        let mut pos = offset + digits;
        while n >= 100 {
            pos -= 2;
            let idx = ((n % 100) * 2) as usize;
            buf[pos] = Lut::DECIMAL_PAIRS[idx];
            buf[pos + 1] = Lut::DECIMAL_PAIRS[idx + 1];
            n /= 100;
        }
        if n < 10 {
            pos -= 1;
            buf[pos] = n as u8 + b'0';
        } else {
            pos -= 2;
            let idx = (n * 2) as usize;
            buf[pos] = Lut::DECIMAL_PAIRS[idx];
            buf[pos + 1] = Lut::DECIMAL_PAIRS[idx + 1];
        }
        digits
    }

    #[doc = DOC_WRITE_DIGITS_10_FAST!(39)]
    pub fn write_digits10_fast(self, buf: &mut [u8], offset: usize) -> usize {
        const MAX: usize = Digits::<u128>::MAX_DIGITS_10 as usize;
        debug_assert!(offset + MAX <= buf.len(), "buffer < 39 bytes");
        let mut n = self.0;
        is![n == 0; { buf[offset] = b'0'; return 1 }];
        let mut pos = offset + MAX;
        while n >= 100 {
            pos -= 2;
            let idx = ((n % 100) * 2) as usize;
            buf[pos] = Lut::DECIMAL_PAIRS[idx];
            buf[pos + 1] = Lut::DECIMAL_PAIRS[idx + 1];
            n /= 100;
        }
        if n < 10 {
            pos -= 1;
            buf[pos] = n as u8 + b'0';
        } else {
            pos -= 2;
            let idx = (n * 2) as usize;
            buf[pos] = Lut::DECIMAL_PAIRS[idx];
            buf[pos + 1] = Lut::DECIMAL_PAIRS[idx + 1];
        }
        let written_len = (offset + MAX) - pos;
        buf.copy_within(pos..(offset + MAX), offset); // WAIT: non-const
        written_len
    }

    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits10_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_10 as usize}> {
        let width = Cmp(width).clamp(self.count_digits10(), Self::MAX_DIGITS_10);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return crate::unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
            ::from_array_nright(self.digits10(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_10 as usize}>
            ::from_array_nright_unchecked(self.digits10(), width) }
    }
    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits16_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_16 as usize}> {
        let width = Cmp(width).clamp(self.count_digits16(), Self::MAX_DIGITS_16);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return crate::unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright_unchecked(self.digits16(), width)
        }
    }
}
