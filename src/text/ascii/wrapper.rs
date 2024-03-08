// devela::text::ascii::wrapper
//
//! Ascii functionality wrapper struct.
//

use crate::code::iif;

/// Provides ASCII operations on `T`, most of them *const*.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Ascii<T: Copy>(pub T);

impl Ascii<usize> {
    /// Returns the ASCII byte of a specific digit in a `usize` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq!(Ascii(12345_usize).calc_digit(10), b'4');
    /// assert_eq!(Ascii(12345_usize).calc_digit(1000), b'2');
    /// ```
    #[inline]
    #[must_use]
    pub const fn calc_digit(self, divisor: usize) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`][crate::num::Int] *base* methods.
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq![1, Ascii(0_usize).count_digits()];
    /// assert_eq![4, Ascii(9876_usize).count_digits()];
    /// ```
    #[inline]
    #[must_use]
    pub const fn count_digits(self) -> usize {
        iif![self.0 == 0; 1; self.0.ilog10() as usize + 1]
    }

    /// Converts a `usize` into a byte array of `5` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[inline] #[must_use] #[cfg(target_pointer_width = "16")] #[rustfmt::skip]
    pub const fn digits(self) -> [u8; Ascii(usize::MAX).count_digits()] {
        Ascii(self.0 as u16).digits()
    }

    /// Converts a `usize` into a byte array of `10` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[inline] #[must_use] #[cfg(target_pointer_width = "32")] #[rustfmt::skip]
    pub const fn digits(self) -> [u8; Ascii(usize::MAX).count_digits()] {
        Ascii(self.0 as u32).digits()
    }

    /// Converts a `usize` into a byte array of `20` ascii digits with leading zeros.
    ///
    /// The actual array length depends on the target platform's pointer size.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[inline] #[must_use] #[cfg(target_pointer_width = "64")] #[rustfmt::skip]
    pub const fn digits(self) -> [u8; Ascii(usize::MAX).count_digits()] {
        Ascii(self.0 as u64).digits()
    }
}

impl Ascii<u8> {
    /// Returns the ASCII byte of a specific digit in a `u8` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq!(Ascii(123_u8).calc_digit(10), b'2');
    /// assert_eq!(Ascii(123_u8).calc_digit(100), b'1');
    /// ```
    #[inline]
    #[must_use]
    pub const fn calc_digit(self, divisor: u8) -> u8 {
        (self.0 / divisor % 10) + b'0'
    }

    /// Counts the number of decimal digits.
    ///
    /// For more complex needs check the [`Int`][crate::num::Int] *base* methods.
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq![1, Ascii(0_u8).count_digits()];
    /// assert_eq![3, Ascii(123_u8).count_digits()];
    /// ```
    #[inline]
    #[must_use]
    pub const fn count_digits(self) -> u8 {
        iif![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    /// Converts a `u8` into a byte array of `3` ASCII digits with leading zeros.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[inline]
    #[must_use]
    pub const fn digits(self) -> [u8; 3] {
        [
            //              321
            //              255 u8::MAX
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Converts a one-digit number to the corresponding `1` ASCII digit.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 9.
    #[inline]
    #[must_use]
    pub const fn digits_1(self) -> u8 {
        debug_assert![self.0 <= 9];
        self.0 + b'0'
    }

    /// Converts a two-digit number to the corresponding `2` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 99.
    #[inline]
    #[must_use]
    pub const fn digits_2(self) -> [u8; 2] {
        debug_assert![self.0 <= 99];
        [self.calc_digit(10), self.calc_digit(1)]
    }
}

impl Ascii<u16> {
    /// Returns the ASCII byte of a specific digit in a `u16` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq!(Ascii(12345_u16).calc_digit(10), b'4');
    /// assert_eq!(Ascii(12345_u16).calc_digit(1000), b'2');
    /// ```
    #[inline]
    #[must_use]
    pub const fn calc_digit(self, divisor: u16) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Converts a `u16` into a byte array of `5` ASCII digits, padded with zeros.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[inline]
    #[must_use]
    pub const fn digits(self) -> [u8; 5] {
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

    /// Converts a three-digit number to the corresponding `3` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 999.
    #[inline]
    #[must_use]
    pub const fn digits_3(self) -> [u8; 3] {
        debug_assert![self.0 <= 999];
        [
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }

    /// Converts a four-digit number to the corresponding `4` ASCII digits.
    ///
    /// # Panics
    /// This function panics in debug if the given number is > 9999.
    #[inline]
    #[must_use]
    pub const fn digits_4(self) -> [u8; 4] {
        debug_assert![self.0 <= 9999];
        [
            self.calc_digit(1000),
            self.calc_digit(100),
            self.calc_digit(10),
            self.calc_digit(1),
        ]
    }
}

impl Ascii<u32> {
    /// Returns the ASCII byte of a specific digit in a `u32` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq!(Ascii(12345_u32).calc_digit(10), b'4');
    /// assert_eq!(Ascii(12345_u32).calc_digit(1000), b'2');
    /// ```
    #[inline]
    #[must_use]
    pub const fn calc_digit(self, divisor: u32) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Converts a `u32` into a byte array of `10` ASCII digits, padded with zeros.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[inline]
    #[must_use]
    pub const fn digits(self) -> [u8; 10] {
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
}

impl Ascii<u64> {
    /// Returns the ASCII byte of a specific digit in a `u64` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq!(Ascii(12345_u64).calc_digit(10), b'4');
    /// assert_eq!(Ascii(12345_u64).calc_digit(1000), b'2');
    /// ```
    #[inline]
    #[must_use]
    pub const fn calc_digit(self, divisor: u64) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Converts a `u64` into a byte array of `20` ascii digits, padded with zeros.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[must_use]
    pub const fn digits(self) -> [u8; 20] {
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
}

impl Ascii<u128> {
    /// Returns the ASCII byte of a specific digit in a `u128` number.
    ///
    /// # Arguments
    /// * `divisor`: A power of 10 used to determine which digit to extract.
    ///
    /// # Examples
    /// ```
    /// # use devela::text::Ascii;
    /// assert_eq!(Ascii(12345_u128).calc_digit(10), b'4');
    /// assert_eq!(Ascii(12345_u128).calc_digit(1000), b'2');
    /// ```
    #[inline]
    #[must_use]
    pub const fn calc_digit(self, divisor: u128) -> u8 {
        (self.0 / divisor % 10) as u8 + b'0'
    }

    /// Converts a `u128` into a byte array of `39` ascii digits, padded with zeros.
    ///
    /// You can trim the leading zeros with
    /// [`slice_trim_leading_bytes`][crate::mem::slice_trim_leading_bytes].
    #[must_use]
    pub const fn digits(self) -> [u8; 39] {
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
}
