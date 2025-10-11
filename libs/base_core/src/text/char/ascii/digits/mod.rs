// devela_base_core::text::char::ascii::digits
//
//! Defines [`AsciiDigits`].
//
// TOC
// - const DOC_*
// - struct AsciiDigits
// - impls for:
//   - usize
//   - u8
//   - u16
//   - u32
//   - u64
//   - u128

#[allow(unused, reason = "±unsafe in digits*_str methods")]
use crate::unwrap;
use crate::{CONST, Cmp, StringU8, is};

#[cfg(test)]
mod tests;

CONST! {
DOC_DIGIT_AT_POWER_10 =
"Extracts the ASCII byte for a specific digit position using a power-of-10 divisor.\n\n
# Arguments\n\n
* `divisor` - A power of 10 that selects the digit position (1, 10, 100, …)";
DOC_DIGIT_AT_POWER_16 =
"Extracts the ASCII byte for a specific digit position using a power-of-16 divisor.\n\n
# Arguments\n\n
* `divisor` - A power of 16 that selects the digit position (1, F, FF, …)";

DOC_COUNT_DIGITS_10 = "Counts the number of decimal digits in the number.\n\n
Returns 1 for zero, and the base-10 logarithm plus one for other numbers.\n\n
For more advanced needs check the [`Int`] *base* methods.\n\n";
DOC_COUNT_DIGITS_16 = "Counts the number of hexadecimal digits in the number.\n\n
Returns 1 for zero, and the base-16 logarithm plus one for other numbers.\n\n
For more advanced needs check the [`Int`] *base* methods.\n\n";

DOC_DIGITS_STR = "Returns a static string with zero-padded digits with minimum `width`.\n\n
The given `width` will be clamped betweeen the actual number of digits
and the maximum number of digits.
# Features
- Makes use of the `unsafe_str` feature if enabled.\n\n";
}

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Provides ASCII digit operations and conversions for unsigned integer primitives.
///
#[doc = crate::_doc!(location: "text/char")]
///
/// Enables efficient ASCII digit extraction, counting, and conversion
/// for unsigned integer types. All operations are `const` and designed for
/// performance-critical scenarios like number formatting.
///
/// It converts **numbers → digits** for display/formatting.
///
/// # Example
/// ```
/// # use devela_base_core::{AsciiDigits, Slice};
/// let ascii_num = AsciiDigits(12345_u32);
/// assert_eq!(ascii_num.digit_at_power10(100), b'3');
/// assert_eq!(ascii_num.count_digits10(), 5);
/// assert_eq!(ascii_num.digits10(), *b"0000012345");
/// assert_eq!(Slice::trim_leading(&ascii_num.digits10(), b'0'), b"12345");
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiDigits<T: Copy>(pub T);

impl<T: Copy> AsciiDigits<T> {
    /// Lookup table for digit characters in bases 2 through 36.
    const DIGITS: [u8; 36] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
}

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
        let power = POWERS10[index as usize] as usize;
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = POWERS10[index as usize] as usize;
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = self.0.unbounded_shr(shift) & 0xF;
        Self::DIGITS[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = self.0.unbounded_shr(shift) & 0xF;
            Some(Self::DIGITS[digit])
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright_unchecked(self.digits16(), width) }
    }
}

impl AsciiDigits<u8> {
    /// The maximum number of decimal digits a `u8` can represent.
    pub const MAX_DIGITS_10: u8 = 3;

    /// The maximum number of hexadecimal digits a `u8` can represent.
    pub const MAX_DIGITS_16: u8 = 2; // 0xFF

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= Self::MAX_DIGITS_10; return b'0'];
        let power = POWERS10[index as usize] as u8;
        (self.0 / power % 10) + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = POWERS10[index as usize] as u8;
        Some((self.0 / power % 10) + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Self::DIGITS[digit]
    }
    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
            Some(Self::DIGITS[digit])
        } else {
            None
        }
    }

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(123_u8).digit_at_power10(10), b'2');
    /// assert_eq!(AsciiDigits(123_u8).digit_at_power10(100), b'1');
    /// ```
    #[must_use]
    pub const fn digit_at_power10(self, divisor: u8) -> u8 {
        (self.0 / divisor % 10) + b'0'
    }

    #[doc = DOC_DIGIT_AT_POWER_16!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(0xAB_u8).digit_at_power16(0x1), b'B');
    /// assert_eq!(AsciiDigits(0xAB_u8).digit_at_power16(0x10), b'A');
    /// ```
    #[must_use]
    pub const fn digit_at_power16(self, divisor: u8) -> u8 {
        let digit = match divisor {
            0x1 => self.0 & 0xF,
            0x10 => (self.0 >> 4) & 0xF,
            _ => (self.0 / divisor) % 16,
        };
        Self::DIGITS[digit as usize]
    }

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u8).count_digits10()];
    /// assert_eq![3, AsciiDigits(123_u8).count_digits10()];
    /// ```
    #[must_use]
    pub const fn count_digits10(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    #[doc = DOC_COUNT_DIGITS_16!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    #[must_use]
    pub const fn count_digits16(self) -> u8 {
        match self.0 {
            0 => 1,
            0x1..=0xF => 1,
            _ => 2,
        }
    }

    /// Converts a `u8` into a byte array of `3` ASCII decimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        [
            //                    321
            //                    255    ← u8::MAX
            self.digit_at_power10(100), // 3 digits
            self.digit_at_power10(10),
            self.digit_at_power10(1),
        ]
    }
    /// Converts a `u8` into a byte array of `2` ASCII digits with leading zero.
    ///
    /// You can trim the leading zerowith `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        [
            //                      21
            //                      FF    ← u8::MAX
            self.digit_at_power16(0x10), // 2 digits
            self.digit_at_power16(0x1),
        ]
    }

    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits10_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_10 as usize}> {
        let width = Cmp(width).clamp(self.count_digits10(), Self::MAX_DIGITS_10);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright_unchecked(self.digits16(), width) }
    }

    /// Converts a one-digit decimal number to the corresponding `1` ASCII digit.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 9.
    #[must_use]
    pub const fn digits10_1(self) -> u8 {
        debug_assert![self.0 <= 9];
        self.0 + b'0'
    }
    /// Converts a two-digit decimal number to the corresponding `2` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 99.
    #[must_use]
    pub const fn digits10_2(self) -> [u8; 2] {
        debug_assert![self.0 <= 99];
        [self.digit_at_power10(10), self.digit_at_power10(1)]
    }

    /// Converts a one-digit hexadecimal number to the corresponding `1` ASCII digit.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > `0xF`.
    pub const fn digits16_1(self) -> u8 {
        debug_assert![self.0 <= 0xF];
        Self::DIGITS[self.0 as usize]
    }
}

impl AsciiDigits<u16> {
    /// The maximum number of decimal digits a `u16` can represent.
    pub const MAX_DIGITS_10: u8 = 5;

    /// The maximum number of hexadecimal digits a `u16` can represent.
    pub const MAX_DIGITS_16: u8 = 4;

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= Self::MAX_DIGITS_10; return b'0'];
        let power = POWERS10[index as usize] as u16;
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = POWERS10[index as usize] as u16;
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Self::DIGITS[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
            Some(Self::DIGITS[digit])
        } else {
            None
        }
    }

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u16).digit_at_power10(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u16).digit_at_power10(1000), b'2');
    /// ```
    #[must_use]
    pub const fn digit_at_power10(self, divisor: u16) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    pub const fn digit_at_power16(self, divisor: u16) -> u8 {
        let digit = match divisor {
            0x1 => self.0 & 0xF,
            0x10 => (self.0 >> 4) & 0xF,
            0x100 => (self.0 >> 8) & 0xF,
            0x1000 => (self.0 >> 12) & 0xF,
            _ => (self.0 / divisor) % 16,
        };
        Self::DIGITS[digit as usize]
    }

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u16).count_digits10()];
    /// assert_eq![4, AsciiDigits(9876_u16).count_digits10()];
    /// ```
    #[must_use]
    pub const fn count_digits10(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    #[doc = DOC_COUNT_DIGITS_16!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    pub const fn count_digits16(self) -> u8 {
        match self.0 {
            0 => 1,
            0x1..=0xF => 1,
            0x10..=0xFF => 2,
            0x100..=0xFFF => 3,
            _ => 4,
        }
    }

    /// Converts a `u16` into a byte array of `5` ASCII decimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        [
            //                    54321
            //                    65535    ← u16::MAX
            self.digit_at_power10(10000), // 5 digits
            self.digit_at_power10(1000),
            self.digit_at_power10(100),
            self.digit_at_power10(10),
            self.digit_at_power10(1),
        ]
    }
    /// Converts a `u16` into a byte array of `4` ASCII digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        [
            //                      4321
            //                      FFFF    ← u16::MAX
            self.digit_at_power16(0x1000), // 4 digits
            self.digit_at_power16(0x100),
            self.digit_at_power16(0x10),
            self.digit_at_power16(0x1),
        ]
    }

    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits10_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_10 as usize}> {
        let width = Cmp(width).clamp(self.count_digits10(), Self::MAX_DIGITS_10);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
            ::from_array_nright(self.digits10(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_10 as usize}>
            ::from_array_nright_unchecked(self.digits10(), width)}
    }
    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits16_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_16 as usize}> {
        let width = Cmp(width).clamp(self.count_digits16(), Self::MAX_DIGITS_16);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{ Self::MAX_DIGITS_16 as usize }>
            ::from_array_nright_unchecked(self.digits16(), width)
        }
    }

    /// Converts a three-digit decimal number to the corresponding `3` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 999.
    #[must_use]
    pub const fn digits10_3(self) -> [u8; 3] {
        debug_assert![self.0 <= 999];
        [self.digit_at_power10(100), self.digit_at_power10(10), self.digit_at_power10(1)]
    }

    /// Converts a four-digit decimal number to the corresponding `4` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 9999.
    #[must_use]
    pub const fn digits10_4(self) -> [u8; 4] {
        debug_assert![self.0 <= 9999];
        [
            self.digit_at_power10(1000),
            self.digit_at_power10(100),
            self.digit_at_power10(10),
            self.digit_at_power10(1),
        ]
    }
}

impl AsciiDigits<u32> {
    /// The maximum number of decimal digits a `u32` can represent.
    pub const MAX_DIGITS_10: u8 = 10;

    /// The maximum number of hexadecimal digits a `u32` can represent.
    pub const MAX_DIGITS_16: u8 = 8;

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= Self::MAX_DIGITS_10; return b'0'];
        let power = POWERS10[index as usize] as u32;
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = POWERS10[index as usize] as u32;
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Self::DIGITS[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
            Some(Self::DIGITS[digit])
        } else {
            None
        }
    }

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u32).digit_at_power10(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u32).digit_at_power10(1000), b'2');
    /// ```
    #[must_use]
    pub const fn digit_at_power10(self, divisor: u32) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digit_at_power16(self, divisor: u32) -> u8 {
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
        Self::DIGITS[digit as usize]
    }

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u32).count_digits10()];
    /// assert_eq![4, AsciiDigits(9876_u32).count_digits10()];
    /// ```
    #[must_use]
    pub const fn count_digits10(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    #[doc = DOC_COUNT_DIGITS_16!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    pub const fn count_digits16(self) -> u8 {
        // from u32 and up, the match gets too large and this bit math is more efficient
        is![self.0 == 0; 1; ((self.0.ilog2() + 4) / 4) as u8]
    }

    /// Converts a `u32` into a byte array of `10` ASCII decimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits10(self) -> [u8; Self::MAX_DIGITS_10 as usize] {
        [
            //                    0987654321
            //                    4294967295    ← u32::MAX
            self.digit_at_power10(1000000000), // 10 digits
            self.digit_at_power10(100000000),
            self.digit_at_power10(10000000),
            self.digit_at_power10(1000000),
            self.digit_at_power10(100000),
            self.digit_at_power10(10000), // 5 digits
            self.digit_at_power10(1000),
            self.digit_at_power10(100),
            self.digit_at_power10(10),
            self.digit_at_power10(1),
        ]
    }
    /// Converts a `u32` into a byte array of `8` ASCII hexadecimal digits with leading zeros.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        [
            //                      87654321
            //                      FFFFFFFF    ← u32::MAX
            self.digit_at_power16(0x10000000), // 8 digits
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright_unchecked(self.digits16(), width)
        }
    }
}

impl AsciiDigits<u64> {
    /// The maximum number of decimal digits a `u64` can represent.
    pub const MAX_DIGITS_10: u8 = 20;

    /// The maximum number of hexadecimal digits a `u64` can represent.
    pub const MAX_DIGITS_16: u8 = 16;

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u64).digit_at_power10(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u64).digit_at_power10(1000), b'2');
    /// ```
    #[must_use]
    pub const fn digit_at_power10(self, divisor: u64) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digit_at_power16(self, divisor: u64) -> u8 {
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
        Self::DIGITS[digit as usize]
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= Self::MAX_DIGITS_10; return b'0'];
        let power = POWERS10[index as usize] as u64;
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = POWERS10[index as usize] as u64;
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Self::DIGITS[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
            Some(Self::DIGITS[digit])
        } else {
            None
        }
    }

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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright_unchecked(self.digits16(), width)
        }
    }
}

impl AsciiDigits<u128> {
    /// The maximum number of decimal digits a `u128` can represent.
    pub const MAX_DIGITS_10: u8 = 39;

    /// The maximum number of hexadecimal digits a `u128` can represent.
    pub const MAX_DIGITS_16: u8 = 32;

    #[doc = DOC_DIGIT_AT_POWER_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u128).digit_at_power10(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u128).digit_at_power10(1000), b'2');
    /// ```
    #[must_use]
    pub const fn digit_at_power10(self, divisor: u128) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }
    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digit_at_power16(self, divisor: u128) -> u8 {
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
        Self::DIGITS[digit as usize]
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= Self::MAX_DIGITS_10; return b'0'];
        let power = POWERS10[index as usize];
        (self.0 / power % 10) as u8 + b'0'
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= Self::MAX_DIGITS_10; return None];
        let power = POWERS10[index as usize];
        Some((self.0 / power % 10) as u8 + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Self::DIGITS[digit]
    }

    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        if index < self.count_digits16() {
            let shift = index as u32 * 4;
            let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
            Some(Self::DIGITS[digit])
        } else {
            None
        }
    }

    #[doc = DOC_COUNT_DIGITS_10!()]
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u128).count_digits10()];
    /// assert_eq![19, AsciiDigits(9876543210987654321_u128).count_digits10()];
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

    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits10_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_10 as usize}> {
        let width = Cmp(width).clamp(self.count_digits10(), Self::MAX_DIGITS_10);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
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
        return unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright_unchecked(self.digits16(), width)
        }
    }
}

/// Precomputed powers of 10 for u128: `[10^0, 10^1, ..., 10^38]`.
///
/// Used for efficient digit extraction without runtime exponentiation.
const POWERS10: [u128; 39] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
    100_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000_000_000_000,
];
#[cfg(test)]
const POWERS10_ASSERT: () = const {
    assert![size_of_val(&POWERS10) == size_of::<u128>() * 39];
    assert![size_of_val(&POWERS10) == 624];
};
