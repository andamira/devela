// devela_base_core::text::ascii::wrapper
//
//! Defines [`AsciiDigits`].
//

use crate::is;

#[allow(unused, reason = "±unsafe in digits_str methods")]
use crate::unwrap;
use crate::{Compare, StringU8};

#[doc = crate::TAG_TEXT!()]
#[doc = crate::TAG_NAMESPACE!()]
/// Provides ASCII operations on `T`, most of them *const*.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct AsciiDigits<T: Copy>(pub T);

impl AsciiDigits<usize> {
    /// The maximum number of decimal digits a `usize` can represent in the current platform.
    pub const MAX_DIGITS: usize = AsciiDigits(usize::MAX).count_digits() as usize;

    /// Returns the ASCII byte of a specific digit in a `usize` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_usize).calc_digit(10), b'4');
    /// assert_eq!(AsciiDigits(12345_usize).calc_digit(1000), b'2');
    /// ```
    #[must_use]
    pub const fn calc_digit(self, divisor: usize) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`] *base* methods.
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_usize).count_digits()];
    /// assert_eq![4, AsciiDigits(9876_usize).count_digits()];
    /// ```
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `usize` into a byte array of `5` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use] #[cfg(target_pointer_width = "16")] #[rustfmt::skip]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        AsciiDigits(self.0 as u16).digits()
    }

    /// Converts a `usize` into a byte array of `10` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use] #[cfg(target_pointer_width = "32")] #[rustfmt::skip]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        AsciiDigits(self.0 as u32).digits()
    }

    /// Converts a `usize` into a byte array of `20` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use] #[cfg(target_pointer_width = "64")] #[rustfmt::skip]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        AsciiDigits(self.0 as u64).digits()
    }

    /// Returns a static string with zero-padded digits with minimum `width`.
    ///
    /// The given `width` will be clamped betweeen the actual number of digits
    /// and the maximum number of digits.
    ///
    /// # Features
    /// - Makes use of the `unsafe_str` feature if enabled.
    pub const fn digits_str(self, width: u8) -> StringU8<{ Self::MAX_DIGITS }> {
        let width = Compare(width).clamp(self.count_digits(), Self::MAX_DIGITS as u8);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS}>::from_bytes_nright(self.digits(), width)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            StringU8::<{ Self::MAX_DIGITS }>::from_bytes_nright_unchecked(self.digits(), width)
        }
    }
}

impl AsciiDigits<u8> {
    /// The maximum number of decimal digits a `u8` can represent.
    pub const MAX_DIGITS: usize = 3;

    /// Returns the ASCII byte of a specific digit in a `u8` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(123_u8).calc_digit(10), b'2');
    /// assert_eq!(AsciiDigits(123_u8).calc_digit(100), b'1');
    /// ```
    #[must_use]
    pub const fn calc_digit(self, divisor: u8) -> u8 {
        (self.0 / divisor % 10) + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`] *base* methods.
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u8).count_digits()];
    /// assert_eq![3, AsciiDigits(123_u8).count_digits()];
    /// ```
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `u8` into a byte array of `3` ASCII digits with leading zeros.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        [
            //              321
            //              255 u8::MAX
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Returns a static string with zero-padded digits with minimum `width`.
    ///
    /// The given `width` will be clamped betweeen the actual number of digits
    /// and the maximum number of digits.
    ///
    /// # Features
    /// - Makes use of the `unsafe_str` feature if enabled.
    pub const fn digits_str(self, width: u8) -> StringU8<{ Self::MAX_DIGITS }> {
        let width = Compare(width).clamp(self.count_digits(), Self::MAX_DIGITS as u8);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS}>::from_bytes_nright(self.digits(), width)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            StringU8::<{ Self::MAX_DIGITS }>::from_bytes_nright_unchecked(self.digits(), width)
        }
    }

    /// Converts a one-digit number to the corresponding `1` ASCII digit.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 9.
    #[must_use]
    pub const fn digits_1(self) -> u8 {
        debug_assert![self.0 <= 9];
        self.0 + b'0'
    }

    /// Converts a two-digit number to the corresponding `2` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 99.
    #[must_use]
    pub const fn digits_2(self) -> [u8; 2] {
        debug_assert![self.0 <= 99];
        [self.calc_digit(10), self.calc_digit(1)]
    }
}

impl AsciiDigits<u16> {
    /// The maximum number of decimal digits a `u16` can represent.
    pub const MAX_DIGITS: usize = 5;

    /// Returns the ASCII byte of a specific digit in a `u16` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u16).calc_digit(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u16).calc_digit(1000), b'2');
    /// ```
    #[must_use]
    pub const fn calc_digit(self, divisor: u16) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`] *base* methods.
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u16).count_digits()];
    /// assert_eq![4, AsciiDigits(9876_u16).count_digits()];
    /// ```
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `u16` into a byte array of `5` ASCII digits with leading zeros.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        [
            //              54321
            //              65535    ← u16::MAX
            self.calc_digit(10000), // 5 digits
            self.calc_digit(1000),
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Returns a static string with zero-padded digits with minimum `width`.
    ///
    /// The given `width` will be clamped betweeen the actual number of digits
    /// and the maximum number of digits.
    ///
    /// # Features
    /// - Makes use of the `unsafe_str` feature if enabled.
    pub const fn digits_str(self, width: u8) -> StringU8<{ Self::MAX_DIGITS }> {
        let width = Compare(width).clamp(self.count_digits(), Self::MAX_DIGITS as u8);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS}>::from_bytes_nright(self.digits(), width)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            StringU8::<{ Self::MAX_DIGITS }>::from_bytes_nright_unchecked(self.digits(), width)
        }
    }

    /// Converts a three-digit number to the corresponding `3` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 999.
    #[must_use]
    pub const fn digits_3(self) -> [u8; 3] {
        debug_assert![self.0 <= 999];
        [self.calc_digit(100), self.calc_digit(10), self.calc_digit(1)]
    }

    /// Converts a four-digit number to the corresponding `4` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 9999.
    #[must_use]
    pub const fn digits_4(self) -> [u8; 4] {
        debug_assert![self.0 <= 9999];
        [self.calc_digit(1000), self.calc_digit(100), self.calc_digit(10), self.calc_digit(1)]
    }
}

impl AsciiDigits<u32> {
    /// The maximum number of decimal digits a `u32` can represent.
    pub const MAX_DIGITS: usize = 10;

    /// Returns the ASCII byte of a specific digit in a `u32` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u32).calc_digit(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u32).calc_digit(1000), b'2');
    /// ```
    #[must_use]
    pub const fn calc_digit(self, divisor: u32) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`] *base* methods.
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u32).count_digits()];
    /// assert_eq![4, AsciiDigits(9876_u32).count_digits()];
    /// ```
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `u32` into a byte array of `10` ASCII digits with leading zeros.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        [
            //              0987654321
            //              4294967295    ← u32::MAX
            self.calc_digit(1000000000), // 10 digits
            self.calc_digit(100000000),
            self.calc_digit(10000000),
            self.calc_digit(1000000),
            self.calc_digit(100000),
            self.calc_digit(10000), // 5 digits
            self.calc_digit(1000),
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Returns a static string with zero-padded digits with minimum `width`.
    ///
    /// The given `width` will be clamped betweeen the actual number of digits
    /// and the maximum number of digits.
    ///
    /// # Features
    /// - Makes use of the `unsafe_str` feature if enabled.
    pub const fn digits_str(self, width: u8) -> StringU8<{ Self::MAX_DIGITS }> {
        let width = Compare(width).clamp(self.count_digits(), Self::MAX_DIGITS as u8);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS}>::from_bytes_nright(self.digits(), width)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            StringU8::<{ Self::MAX_DIGITS }>::from_bytes_nright_unchecked(self.digits(), width)
        }
    }
}

impl AsciiDigits<u64> {
    /// The maximum number of decimal digits a `u64` can represent.
    pub const MAX_DIGITS: usize = 20;

    /// Returns the ASCII byte of a specific digit in a `u64` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u64).calc_digit(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u64).calc_digit(1000), b'2');
    /// ```
    #[must_use]
    pub const fn calc_digit(self, divisor: u64) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`] *base* methods.
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u64).count_digits()];
    /// assert_eq![4, AsciiDigits(9876_u64).count_digits()];
    /// ```
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `u64` into a byte array of `20` ascii digits with leading zeros.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        [
            //              0987654321_987654321
            //              18446744073709551615    ← u64::MAX
            self.calc_digit(10000000000000000000), // 20 digits
            self.calc_digit(1000000000000000000),
            self.calc_digit(100000000000000000),
            self.calc_digit(10000000000000000),
            self.calc_digit(1000000000000000),
            self.calc_digit(100000000000000),
            self.calc_digit(10000000000000),
            self.calc_digit(1000000000000),
            self.calc_digit(100000000000),
            self.calc_digit(10000000000),
            self.calc_digit(1000000000), // 10 digits
            self.calc_digit(100000000),
            self.calc_digit(10000000),
            self.calc_digit(1000000),
            self.calc_digit(100000),
            self.calc_digit(10000),
            self.calc_digit(1000),
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Returns a static string with zero-padded digits with minimum `width`.
    ///
    /// The given `width` will be clamped betweeen the actual number of digits
    /// and the maximum number of digits.
    ///
    /// # Features
    /// - Makes use of the `unsafe_str` feature if enabled.
    pub const fn digits_str(self, width: u8) -> StringU8<{ Self::MAX_DIGITS }> {
        let width = Compare(width).clamp(self.count_digits(), Self::MAX_DIGITS as u8);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS}>::from_bytes_nright(self.digits(), width)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            StringU8::<{ Self::MAX_DIGITS }>::from_bytes_nright_unchecked(self.digits(), width)
        }
    }
}

impl AsciiDigits<u128> {
    /// The maximum number of decimal digits a `u128` can represent.
    pub const MAX_DIGITS: usize = 39;

    /// Returns the ASCII byte of a specific digit in a `u128` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq!(AsciiDigits(12345_u128).calc_digit(10), b'4');
    /// assert_eq!(AsciiDigits(12345_u128).calc_digit(1000), b'2');
    /// ```
    #[must_use]
    pub const fn calc_digit(self, divisor: u128) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`] *base* methods.
    #[doc = crate::doclink!(custom devela_base_num "[`Int`]" "num/struct.Int.html")]
    /// # Example
    /// ```
    /// # use devela_base_core::text::AsciiDigits;
    /// assert_eq![1, AsciiDigits(0_u128).count_digits()];
    /// assert_eq![19, AsciiDigits(9876543210987654321_u128).count_digits()];
    /// ```
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `u128` into a byte array of `39` ascii digits with leading zeros.
    ///
    /// You can trim the leading zeros with
    /// [`trim_leading_bytes`][crate::Slice::trim_leading_bytes].
    #[must_use]
    #[allow(clippy::unreadable_literal)]
    pub const fn digits(self) -> [u8; Self::MAX_DIGITS] {
        [
            //              987654321_987654321_987654321_987654321
            //              340282366920938463463374607431768211455    ← u128::MAX
            self.calc_digit(100000000000000000000000000000000000000), // 39 digits
            self.calc_digit(10000000000000000000000000000000000000),
            self.calc_digit(1000000000000000000000000000000000000),
            self.calc_digit(100000000000000000000000000000000000),
            self.calc_digit(10000000000000000000000000000000000),
            self.calc_digit(1000000000000000000000000000000000),
            self.calc_digit(100000000000000000000000000000000),
            self.calc_digit(10000000000000000000000000000000),
            self.calc_digit(1000000000000000000000000000000),
            self.calc_digit(100000000000000000000000000000), // 30 digits
            self.calc_digit(10000000000000000000000000000),
            self.calc_digit(1000000000000000000000000000),
            self.calc_digit(100000000000000000000000000),
            self.calc_digit(10000000000000000000000000),
            self.calc_digit(1000000000000000000000000),
            self.calc_digit(100000000000000000000000),
            self.calc_digit(10000000000000000000000),
            self.calc_digit(1000000000000000000000),
            self.calc_digit(100000000000000000000),
            self.calc_digit(10000000000000000000), // 20 digits
            self.calc_digit(1000000000000000000),
            self.calc_digit(100000000000000000),
            self.calc_digit(10000000000000000),
            self.calc_digit(1000000000000000),
            self.calc_digit(100000000000000),
            self.calc_digit(10000000000000),
            self.calc_digit(1000000000000),
            self.calc_digit(100000000000),
            self.calc_digit(10000000000),
            self.calc_digit(1000000000), // 10 digits
            self.calc_digit(100000000),
            self.calc_digit(10000000),
            self.calc_digit(1000000),
            self.calc_digit(100000),
            self.calc_digit(10000),
            self.calc_digit(1000),
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Returns a static string with zero-padded digits with minimum `width`.
    ///
    /// The given `width` will be clamped betweeen the actual number of digits
    /// and the maximum number of digits.
    ///
    /// # Features
    /// - Makes use of the `unsafe_str` feature if enabled.
    pub const fn digits_str(self, width: u8) -> StringU8<{ Self::MAX_DIGITS }> {
        let width = Compare(width).clamp(self.count_digits(), Self::MAX_DIGITS as u8);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok StringU8::<{Self::MAX_DIGITS}>::from_bytes_nright(self.digits(), width)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            StringU8::<{ Self::MAX_DIGITS }>::from_bytes_nright_unchecked(self.digits(), width)
        }
    }
}
