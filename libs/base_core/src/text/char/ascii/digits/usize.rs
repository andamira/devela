// devela_base_core::text::char::ascii::digits::usize

use super::*;
use crate::{Cmp, LUT_DIGITS_BASE36, LUT_POWERS10, StringU8, is};

impl AsciiDigits<usize> {
    /// The maximum number of decimal digits a `usize` can represent in the current platform.
    pub const MAX_DIGITS_10: u8 = AsciiDigits(usize::MAX).count_digits10();

    /// The maximum number of hexadecimal digits a `usize` can represent in the current platform.
    pub const MAX_DIGITS_16: u8 = AsciiDigits(usize::MAX).count_digits16();

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_usize).digit_at_power10(10), b'4');
    /// assert_eq!(AsciiDigits(12345_usize).digit_at_power10(1000), b'2');
    /// ```
    #[must_use]
    pub const fn digit_at_power10(self, divisor: usize) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    #[must_use] #[cfg(target_pointer_width = "16")] #[rustfmt::skip]
    pub const fn digit_at_power16(self, divisor: usize) -> u8 {
        AsciiDigits(self.0 as u16).digit_at_power16(divisor as u16)
    }
    #[must_use] #[cfg(target_pointer_width = "32")] #[rustfmt::skip]
    pub const fn digit_at_power16(self, divisor: usize) -> u8 {
        AsciiDigits(self.0 as u32).digit_at_power16(divisor as u32)
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(0xABCDE_usize).digit_at_power16(0x1), b'E');
    /// assert_eq!(AsciiDigits(0xABCDE_usize).digit_at_power16(0x10), b'D');
    /// ```
    #[must_use] #[cfg(target_pointer_width = "64")] #[rustfmt::skip]
    pub const fn digit_at_power16(self, divisor: usize) -> u8 {
        AsciiDigits(self.0 as u64).digit_at_power16(divisor as u64)
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= Self::MAX_DIGITS_10; return b'0'];
        let power = LUT_POWERS10[index as usize] as usize;
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = LUT_POWERS10[index as usize] as usize;
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = self.0.unbounded_shr(shift) & 0xF;
        LUT_DIGITS_BASE36[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = self.0.unbounded_shr(shift) & 0xF;
            Some(LUT_DIGITS_BASE36[digit])
        } else {
            None
        }
    }

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_usize).count_digits10()];
    /// assert_eq![4, AsciiDigits(9876_usize).count_digits10()];
    /// ```
    #[must_use]
    pub const fn count_digits10(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }
    #[doc = DOC_COUNT_DIGITS_16!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    #[must_use]
    pub const fn count_digits16(self) -> u8 {
        is![self.0 == 0; 1; ((self.0.ilog2() + 4) / 4) as u8]
    }

    /// Converts a `usize` into a byte array of `5` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use] #[cfg(target_pointer_width = "16")] #[rustfmt::skip]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        AsciiDigits(self.0 as u16).digits10()
    }
    #[must_use] #[cfg(target_pointer_width = "16")] #[rustfmt::skip]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        AsciiDigits(self.0 as u16).digits16()
    }

    /// Converts a `usize` into a byte array of `10` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use] #[cfg(target_pointer_width = "32")] #[rustfmt::skip]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        AsciiDigits(self.0 as u32).digits10()
    }
    #[must_use] #[cfg(target_pointer_width = "32")] #[rustfmt::skip]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        AsciiDigits(self.0 as u32).digits16()
    }

    /// Converts a `usize` into a byte array of `20` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use] #[cfg(target_pointer_width = "64")] #[rustfmt::skip]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        AsciiDigits(self.0 as u64).digits10()
    }
    /// Converts a `usize` into a byte array of `20` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use] #[cfg(target_pointer_width = "64")] #[rustfmt::skip]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        AsciiDigits(self.0 as u64).digits16()
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
            ::from_array_nright_unchecked(self.digits16(), width) }
    }
}
