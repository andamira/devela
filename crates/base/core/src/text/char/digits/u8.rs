// devela_base_core::text::char::digits::u8

use super::*;
use crate::{Cmp, StringU8, TextLut, is};

impl Digits<u8> {
    /// The maximum number of decimal digits a `u8` can represent.
    pub const MAX_DIGITS_10: u8 = 3;

    /// The maximum number of hexadecimal digits a `u8` can represent.
    pub const MAX_DIGITS_16: u8 = 2; // 0xFF

    #[doc = DOC_COUNT_DIGITS_10!()]
    /// # Example
    /// ```
    /// # use devela_base_core::text::Digits;
    /// assert_eq![1, Digits(0_u8).count_digits10()];
    /// assert_eq![3, Digits(123_u8).count_digits10()];
    /// ```
    #[must_use]
    pub const fn count_digits10(self) -> u8 {
        is![self.0 == 0; 1; self.0.ilog10() as u8 + 1]
    }

    #[doc = DOC_COUNT_DIGITS_16!()]
    #[must_use]
    pub const fn count_digits16(self) -> u8 {
        match self.0 {
            0 => 1,
            0x1..=0xF => 1,
            _ => 2,
        }
    }

    /* digit_at_ */

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `b'0'` if the index is beyond the number's decimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_at_index10(self, index: u8) -> u8 {
        is![index >= self.count_digits10(); return b'0'];
        const POWERS: [u8; 3] = [100, 10, 1];
        self.digit_at_power10(POWERS[index as usize])
    }

    /// Returns the ASCII decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits10(); return None];
        let power = TextLut::POWERS10[index as usize] as u8;
        Some((self.0 / power % 10) + b'0')
    }

    /// Returns the ASCII hexadecimal digit at the specified index (0 = least significant digit).
    ///
    /// For indices beyond the number's hexadecimal digits, returns the digit `b'0'`.
    #[must_use]
    pub const fn digit_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        TextLut::DIGITS_BASE36[digit]
    }
    /// Returns `Some(ASCII digit)` if the index is within the number's hexadecimal digits,
    ///
    /// For indices beyond the number's hexadecimal digits, returns `None`.
    #[must_use]
    pub const fn digit_at_index16_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits16(); return None];
        let shift = index as u32 * 4;
        let digit = (self.0.unbounded_shr(shift) & 0xF) as usize;
        Some(TextLut::DIGITS_BASE36[digit])
    }

    /* digit_value_at_ */

    /// Returns the numeric value (0-9) of the decimal digit at the specified index.
    ///
    /// Returns `0` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_value_at_index10(self, index: u8) -> u8 {
        is![index >= self.count_digits10(); return 0];
        let power = TextLut::POWERS10[index as usize] as u8;
        self.0 / power % 10
    }
    /// Returns `Some(numeric_value)` (0-9) of the decimal digit at the specified index.
    ///
    /// Returns `None` if the index is beyond the number's decimal digits.
    #[must_use]
    pub const fn digit_value_at_index10_checked(self, index: u8) -> Option<u8> {
        is![index >= self.count_digits10(); return None];
        let power = TextLut::POWERS10[index as usize] as u8;
        Some(self.0 / power % 10)
    }

    /// Returns the numeric value (0-15) of the hexadecimal digit at the specified index.
    ///
    /// Returns `0` if the index is beyond the number's hexadecimal digits.
    #[must_use]
    #[inline(always)]
    pub const fn digit_value_at_index16(self, index: u8) -> u8 {
        let shift = index as u32 * 4;
        self.0.unbounded_shr(shift) & 0xF
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
    #[inline(always)]
    pub(crate) const fn digit_at_power10(self, divisor: u8) -> u8 {
        (self.0 / divisor % 10) + b'0'
    }

    #[doc = DOC_DIGIT_AT_POWER_16!()]
    #[must_use]
    #[inline(always)]
    pub(crate) const fn digit_at_power16(self, divisor: u8) -> u8 {
        let digit = match divisor {
            0x1 => self.0 & 0xF,
            0x10 => (self.0 >> 4) & 0xF,
            _ => (self.0 / divisor) % 16,
        };
        TextLut::DIGITS_BASE36[digit as usize]
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
        // alternative with no clear advantage, should bench
        // let n = self.0;
        // let hundreds = n / 100;
        // let tens = (n % 100) / 10;
        // let ones = n % 10;
        // [hundreds + b'0', tens + b'0', ones + b'0']
    }
    /// Converts a `u8` into a byte array of `2` ASCII digits with leading zero.
    ///
    /// You can trim the leading zeros with `Slice::`[`trim_leading()`][crate::Slice::trim_leading].
    #[must_use]
    pub const fn digits16(self) -> [u8; Self::MAX_DIGITS_16 as usize] {
        [
            //                      21
            //                      FF    ← u8::MAX
            self.digit_at_power16(0x10), // 2 digits
            self.digit_at_power16(0x1),
        ]
    }

    #[inline(always)]
    const fn write_digits10_inner(self, buf: &mut [u8], offset: usize) -> usize {
        let n = self.0;
        if n < 10 {
            buf[offset] = n + b'0';
            1
        } else if n < 100 {
            buf[offset] = n / 10 + b'0';
            buf[offset + 1] = n % 10 + b'0';
            2
        } else {
            buf[offset] = n / 100 + b'0';
            buf[offset + 1] = (n / 10) % 10 + b'0';
            buf[offset + 2] = n % 10 + b'0';
            3
        }
    }

    /// Writes 1..=3 decimal digits without leading zeros starting at `offset`,
    /// returning the number of bytes written.
    ///
    /// Returns 0 and writes nothing if fewer than 3 bytes remain.
    pub const fn write_digits10(self, buf: &mut [u8], offset: usize) -> usize {
        is![offset + 3 > buf.len(); return 0];
        self.write_digits10_inner(buf, offset)
    }

    /// Writes 1..=3 decimal digits without leading zeros starting at `offset`,
    /// returning the number of bytes written.
    ///
    /// Returns 0 and writes nothing if the value is 0 or if fewer than 3 bytes remain.
    pub const fn write_digits10_omit0(self, buf: &mut [u8], offset: usize) -> usize {
        is![self.0 == 0; return 0];
        is![offset + 3 > buf.len(); return 0];
        self.write_digits10_inner(buf, offset)
    }

    /// Writes 1..=2 hexadecimal digits without leading zeros starting at `offset`,
    /// returning the number of bytes written.
    ///
    /// Returns 0 and writes nothing if fewer than 2 bytes remain.
    pub const fn write_digits16(self, buf: &mut [u8], offset: usize) -> usize {
        let n = self.0;
        is![offset + 2 > buf.len(); return 0];
        if n < 0x10 {
            buf[offset] = TextLut::DIGITS_BASE36[n as usize];
            1
        } else {
            buf[offset] = TextLut::DIGITS_BASE36[(n >> 4) as usize];
            buf[offset + 1] = TextLut::DIGITS_BASE36[(n & 0x0F) as usize];
            2
        }
    }
    /// Writes 1..=2 hexadecimal digits without leading zeros starting at `offset`,
    /// returning the number of bytes written.
    ///
    /// Returns 0 and writes nothing if the value is 0 or if fewer than 2 bytes remain.
    pub const fn write_digits16_omit0(self, buf: &mut [u8], offset: usize) -> usize {
        let n = self.0;
        is![n == 0; return 0];
        is![offset + 2 > buf.len(); return 0];
        if n < 0x10 {
            buf[offset] = TextLut::DIGITS_BASE36[n as usize];
            1
        } else {
            buf[offset] = TextLut::DIGITS_BASE36[(n >> 4) as usize];
            buf[offset + 1] = TextLut::DIGITS_BASE36[(n & 0x0F) as usize];
            2
        }
    }

    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits10_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_10 as usize}> {
        let width = Cmp(width).clamp(self.count_digits10(), Self::MAX_DIGITS_10);

        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return crate::unwrap![ok StringU8::<{Self::MAX_DIGITS_10 as usize}>
            ::from_array_nright(self.digits10(), width)];

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid UTF-8
        unsafe { StringU8::<{Self::MAX_DIGITS_10 as usize}>
            ::from_array_nright_unchecked(self.digits10(), width) }
    }
    #[doc = DOC_DIGITS_STR!()] #[rustfmt::skip]
    pub const fn digits16_str(self, width: u8) -> StringU8<{Self::MAX_DIGITS_16 as usize}> {
        let width = Cmp(width).clamp(self.count_digits16(), Self::MAX_DIGITS_16);

        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return crate::unwrap![ok StringU8::<{Self::MAX_DIGITS_16 as usize}>
            ::from_array_nright(self.digits16(), width)];

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
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
        TextLut::DIGITS_BASE36[self.0 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Digits;

    #[test]
    fn test_write_digits10() {
        let mut buf = [0u8; 10];
        assert_eq!(Digits(123_u8).write_digits10(&mut buf, 0), 3);
        assert_eq!(Digits(45_u8).write_digits10(&mut buf, 3), 2);
        assert_eq!(Digits(0_u8).write_digits10(&mut buf, 5), 1); // writes 0
        assert_eq!(Digits(6_u8).write_digits10(&mut buf, 6), 1);
        assert_eq!(&buf[0..7], b"1234506");
        assert_eq!(Digits(100_u8).write_digits10(&mut buf[0..2], 0), 0); // buffer too small
    }
    #[test]
    fn test_write_digits10_omit0() {
        let mut buf = [0u8; 10];
        assert_eq!(Digits(123_u8).write_digits10_omit0(&mut buf, 0), 3);
        assert_eq!(Digits(45_u8).write_digits10_omit0(&mut buf, 3), 2);
        assert_eq!(Digits(0_u8).write_digits10_omit0(&mut buf, 3), 0); // omits zero
        assert_eq!(Digits(6_u8).write_digits10_omit0(&mut buf, 5), 1);
        assert_eq!(&buf[0..6], b"123456");
        assert_eq!(Digits(100_u8).write_digits10_omit0(&mut buf[0..2], 0), 0); // buffer too small
    }

    #[test]
    fn test_write_digits16() {
        let mut buf = [0u8; 10];
        assert_eq!(Digits(0xA_u8).write_digits16(&mut buf, 0), 1);
        assert_eq!(Digits(0_u8).write_digits16(&mut buf, 1), 1); // writes 0
        assert_eq!(Digits(0xBC_u8).write_digits16(&mut buf, 2), 2);
        assert_eq!(&buf[0..4], b"A0BC");
        assert_eq!(Digits(0x10_u8).write_digits16(&mut buf[0..1], 0), 0); // buffer too small
    }
    #[test]
    fn test_write_digits16_omit0() {
        let mut buf = [0u8; 10];
        assert_eq!(Digits(0x1_u8).write_digits16_omit0(&mut buf, 0), 1);
        assert_eq!(Digits(0x0__u8).write_digits16_omit0(&mut buf, 1), 0); // omits zero
        assert_eq!(Digits(0x23_u8).write_digits16_omit0(&mut buf, 1), 2);
        assert_eq!(&buf[0..3], b"123");
        assert_eq!(Digits(0x10_u8).write_digits16_omit0(&mut buf[0..1], 0), 0); // buffer too small
    }
}
