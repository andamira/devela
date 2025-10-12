// devela_base_core::text::char::digits::u64

use super::*;
use crate::{Cmp, LUT_DIGITS_BASE36, LUT_POWERS10, StringU8, is};

impl AsciiDigits<u64> {
    /// The maximum number of decimal digits a `u64` can represent.
    pub const MAX_DIGITS_10: u8 = 20;

    /// The maximum number of hexadecimal digits a `u64` can represent.
    pub const MAX_DIGITS_16: u8 = 16;

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u64).count_digits10()];
    /// assert_eq![4, AsciiDigits(9876_u64).count_digits10()];
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

    /* digit_at_ */

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= self.count_digits10(); return b'0'];
        let power = LUT_POWERS10[index as usize] as u64;
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits10(); return None];
        let power = LUT_POWERS10[index as usize] as u64;
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        LUT_DIGITS_BASE36[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits16(); return None];
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Some(LUT_DIGITS_BASE36[digit])
    }

    /* digit_value_at_ */

    /// Returns the numeric value (0-9) of the decimal digit at the specified index.
    ///
    /// Returns `0` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_value_at_index10(self, index: u8) -> u8 {
        is![index >= self.count_digits10(); return 0];
        let power = LUT_POWERS10[index as usize] as u64;
        (self.0 / power % 10) as u8
    }
    /// Returns `Some(numeric_value)` (0-9) of the decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_value_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits10(); return None];
        let power = LUT_POWERS10[index as usize] as u64;
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
    pub(crate) const fn digit_at_power10(self, divisor: u64) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub(crate) const fn digit_at_power16(self, divisor: u64) -> u8 {
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
        LUT_DIGITS_BASE36[digit as usize]
    }

    /// Converts a `u64` into a byte array of `20` ASCII decimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        [
            //                    0987654321_987654321
            //                    18446744073709551615    ← u64::MAX
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
    /// Converts a `u64` into a byte array of `16` ASCII hexadecimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[allow(clippy::unreadable_literal)]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        [
            //                      0FEDCBA987654321
            //                      FFFFFFFFFFFFFFFF    ← u64::MAX
            self.digit_at_power16(0x1000000000000000), // 16 digits
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
