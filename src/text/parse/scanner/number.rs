// devela/src/text/parse/scanner/number.rs

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{TextParseError, TextScanner, is, unwrap, whilst};

/// ASCII numeric parsing.
impl<'a> TextScanner<'a> {
    /* unsigned 64-bit */

    /// Consumes an ASCII unsigned decimal 64-bit integer prefix.
    ///
    /// Returns:
    /// - `Ok(Some(value))` if one or more digits were consumed.
    /// - `Ok(None)` if the next byte is not an ASCII decimal digit.
    /// - `Err(...)` on numeric overflow.
    ///
    /// On overflow, the scanner stops before the offending digit.
    pub const fn take_ascii_u64(&mut self) -> Result<Option<u64>, TextParseError> {
        self.take_ascii_u64_radix(10)
    }
    /// Consumes an ASCII unsigned decimal 64-bit integer, or returns an error.
    ///
    /// Returns [`InvalidDigit`][TextParseErrorKind::InvalidDigit] if no
    /// decimal integer starts at the current cursor.
    pub const fn expect_ascii_u64(&mut self) -> Result<u64, TextParseError> {
        match unwrap![ok self.take_ascii_u64()] {
            Some(value) => Ok(value),
            None => Err(TextParseError::invalid_digit(self.cursor)),
        }
    }

    /// Consumes an ASCII unsigned 64-bit integer prefix in `radix`.
    ///
    /// Radices from 2 through 36 are supported. ASCII letters are
    /// case-insensitive and represent digits 10 through 35.
    ///
    /// Base prefixes and digit separators are not recognized. For example,
    /// hexadecimal input should be `FF`, not `0xFF`.
    ///
    /// Returns:
    /// - `Ok(Some(value))` if one or more valid digits were consumed.
    /// - `Ok(None)` if the next byte is not a digit in `radix`.
    /// - `Err(...)` on numeric overflow.
    ///
    /// On overflow, the scanner stops before the offending digit.
    ///
    /// # Panics
    /// Panics if `radix` is outside `2..=36`.
    pub const fn take_ascii_u64_radix(
        &mut self,
        radix: u32,
    ) -> Result<Option<u64>, TextParseError> {
        assert_ascii_radix(radix);
        let radix_u64 = radix as u64;
        let (mut value, mut saw_digit) = (0_u64, false);
        whilst! { let Some(byte) = self.peek_byte(); {
            let digit = match ascii_digit_value(byte) {
                Some(digit) if digit < radix => digit as u64,
                _ => break,
            };
            let Some(next) = value.checked_mul(radix_u64) else {
                return Err(TextParseError::overflow(self.cursor));
            };
            let Some(next) = next.checked_add(digit) else {
                return Err(TextParseError::overflow(self.cursor));
            };
            value = next;
            self._cursor_bump(1);
            saw_digit = true;
        }}
        if saw_digit { Ok(Some(value)) } else { Ok(None) }
    }

    /// Consumes an ASCII unsigned 64-bit integer in `radix`, or returns an error.
    ///
    /// Returns [`InvalidDigit`][TextParseErrorKind::InvalidDigit] if no
    /// integer in `radix` starts at the current cursor.
    ///
    /// # Panics
    /// Panics if `radix` is outside `2..=36`.
    pub const fn expect_ascii_u64_radix(&mut self, radix: u32) -> Result<u64, TextParseError> {
        match unwrap![ok self.take_ascii_u64_radix(radix)] {
            Some(value) => Ok(value),
            None => Err(TextParseError::invalid_digit(self.cursor)),
        }
    }

    /* signed 64-bit */

    /// Consumes an ASCII signed decimal 64-bit integer prefix.
    ///
    /// An optional leading `+` or `-` is accepted.
    ///
    /// A sign not followed by a decimal digit is not consumed and produces
    /// `Ok(None)`.
    ///
    /// On overflow, the scanner stops before the offending digit.
    pub const fn take_ascii_i64(&mut self) -> Result<Option<i64>, TextParseError> {
        self.take_ascii_i64_radix(10)
    }
    /// Consumes an ASCII signed decimal 64-bit integer, or returns an error.
    ///
    /// Returns [`InvalidDigit`][TextParseErrorKind::InvalidDigit] if no
    /// signed decimal integer starts at the current cursor.
    pub const fn expect_ascii_i64(&mut self) -> Result<i64, TextParseError> {
        match unwrap![ok self.take_ascii_i64()] {
            Some(value) => Ok(value),
            None => Err(TextParseError::invalid_digit(self.cursor)),
        }
    }

    /// Consumes an ASCII signed 64-bit integer prefix in `radix`.
    ///
    /// An optional leading `+` or `-` is accepted. A sign is consumed only
    /// when immediately followed by a valid digit in `radix`.
    ///
    /// Radices from 2 through 36 are supported. ASCII letters are
    /// case-insensitive and represent digits 10 through 35.
    ///
    /// Base prefixes and digit separators are not recognized.
    ///
    /// On overflow, the scanner stops before the offending digit.
    ///
    /// # Panics
    /// Panics if `radix` is outside `2..=36`.
    pub const fn take_ascii_i64_radix(
        &mut self,
        radix: u32,
    ) -> Result<Option<i64>, TextParseError> {
        assert_ascii_radix(radix);
        let negative = match self.peek_byte() {
            Some(b'-') => {
                is! { !is_ascii_digit_in_radix(self.peek_byte_at(1), radix), return Ok(None) }
                self._cursor_bump(1);
                true
            }
            Some(b'+') => {
                is! { !is_ascii_digit_in_radix(self.peek_byte_at(1), radix), return Ok(None) }
                self._cursor_bump(1);
                false
            }
            byte if is_ascii_digit_in_radix(byte, radix) => false,
            _ => return Ok(None),
        };
        let radix_i64 = radix as i64;
        let mut value = 0_i64;
        whilst! { let Some(byte) = self.peek_byte(); {
            let digit = match ascii_digit_value(byte) {
                Some(digit) if digit < radix => digit as i64,
                _ => break,
            };
            let Some(next) = value.checked_mul(radix_i64) else {
                return Err(TextParseError::overflow(self.cursor));
            };
            // Accumulating negative values directly allows i64::MIN without
            // requiring an intermediate positive magnitude that cannot fit.
            let next = if negative { next.checked_sub(digit) } else { next.checked_add(digit) };
            let Some(next) = next else {
                return Err(TextParseError::overflow(self.cursor));
            };
            value = next;
            self._cursor_bump(1);
        }}
        Ok(Some(value))
    }
    /// Consumes an ASCII signed 64-bit integer in `radix`, or returns an error.
    ///
    /// Returns [`InvalidDigit`][TextParseErrorKind::InvalidDigit] if no
    /// signed integer in `radix` starts at the current cursor.
    ///
    /// # Panics
    /// Panics if `radix` is outside `2..=36`.
    pub const fn expect_ascii_i64_radix(&mut self, radix: u32) -> Result<i64, TextParseError> {
        match unwrap![ok self.take_ascii_i64_radix(radix)] {
            Some(value) => Ok(value),
            None => Err(TextParseError::invalid_digit(self.cursor)),
        }
    }

    /* pointer-sized unsigned */

    /// Consumes an ASCII unsigned decimal pointer-sized integer prefix.
    ///
    /// Returns:
    /// - `Ok(Some(value))` if one or more digits were consumed.
    /// - `Ok(None)` if the next byte is not an ASCII decimal digit.
    /// - `Err(...)` on numeric overflow.
    ///
    /// On overflow, the scanner stops before the offending digit.
    pub const fn take_ascii_usize(&mut self) -> Result<Option<usize>, TextParseError> {
        let (mut value, mut saw_digit) = (0_usize, false);
        whilst! { let Some(byte) = self.peek_byte(); {
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as usize,
                _ => break,
            };
            let Some(next) = value.checked_mul(10) else {
                return Err(TextParseError::overflow(self.cursor));
            };
            let Some(next) = next.checked_add(digit) else {
                return Err(TextParseError::overflow(self.cursor));
            };
            value = next;
            self._cursor_bump(1);
            saw_digit = true;
        }}
        if saw_digit { Ok(Some(value)) } else { Ok(None) }
    }

    /// Consumes an ASCII unsigned decimal pointer-sized integer.
    ///
    /// Returns [`InvalidDigit`][TextParseErrorKind::InvalidDigit] if no
    /// decimal integer starts at the current cursor.
    pub const fn expect_ascii_usize(&mut self) -> Result<usize, TextParseError> {
        match unwrap![ok self.take_ascii_usize()] {
            Some(value) => Ok(value),
            None => Err(TextParseError::invalid_digit(self.cursor)),
        }
    }
}

/* private helpers */

const fn assert_ascii_radix(radix: u32) {
    assert!(radix >= 2 && radix <= 36, "ASCII integer radix must be in 2..=36");
}
const fn ascii_digit_value(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some((byte - b'0') as u32),
        b'A'..=b'Z' => Some((byte - b'A') as u32 + 10),
        b'a'..=b'z' => Some((byte - b'a') as u32 + 10),
        _ => None,
    }
}
const fn is_ascii_digit_in_radix(byte: Option<u8>, radix: u32) -> bool {
    match byte {
        Some(byte) => match ascii_digit_value(byte) {
            Some(digit) => digit < radix,
            None => false,
        },
        None => false,
    }
}

#[cfg(test)]
mod _test {
    use crate::{TextParseErrorKind, TextScanner, assert_matches};

    #[test]
    fn ascii_u64_edge_cases() {
        let mut s = TextScanner::new("abc");
        assert_eq!(s.take_ascii_u64(), Ok(None));
        assert!(s.expect_ascii_u64().is_err());
        assert_eq!(s.pos().as_usize(), 0);
        let mut s = TextScanner::new("18446744073709551615!");
        assert_eq!(s.take_ascii_u64(), Ok(Some(u64::MAX)));
        assert_eq!(s.peek_byte(), Some(b'!'));
        let mut s = TextScanner::new("18446744073709551616!");
        assert!(s.take_ascii_u64().is_err());
        assert_eq!(s.pos().as_usize(), 19); // stops before the offending digit
        assert_eq!(s.peek_byte(), Some(b'6'));
    }
    #[test]
    fn ascii_i64_edge_cases() {
        let mut s = TextScanner::new("abc");
        assert_eq!(s.take_ascii_i64(), Ok(None));
        assert_eq!(s.pos().as_usize(), 0);
        let err = s.expect_ascii_i64().unwrap_err();
        assert_matches!(err.kind, TextParseErrorKind::InvalidDigit);
        assert_eq!(err.at.index.as_usize(), 0);
        assert_eq!(s.pos().as_usize(), 0);
        let mut s = TextScanner::new("9223372036854775807!");
        assert_eq!(s.take_ascii_i64(), Ok(Some(i64::MAX)));
        assert_eq!(s.peek_byte(), Some(b'!'));
        let mut s = TextScanner::new("-9223372036854775808!");
        assert_eq!(s.take_ascii_i64(), Ok(Some(i64::MIN)));
        assert_eq!(s.peek_byte(), Some(b'!'));
        let mut s = TextScanner::new("+42!");
        assert_eq!(s.take_ascii_i64(), Ok(Some(42)));
        assert_eq!(s.peek_byte(), Some(b'!'));
        let mut s = TextScanner::new("-0!");
        assert_eq!(s.take_ascii_i64(), Ok(Some(0)));
        assert_eq!(s.peek_byte(), Some(b'!'));
    }
    #[test]
    fn ascii_i64_bare_sign_does_not_advance() {
        for text in ["+", "-", "+x", "-x"] {
            let mut s = TextScanner::new(text);
            assert_eq!(s.take_ascii_i64(), Ok(None), "{text:?}");
            assert_eq!(s.pos().as_usize(), 0, "{text:?}");
            let err = s.expect_ascii_i64().unwrap_err();
            assert_matches!(err.kind, TextParseErrorKind::InvalidDigit);
            assert_eq!(err.at.index.as_usize(), 0);
            assert_eq!(s.pos().as_usize(), 0);
        }
    }
    #[test]
    fn ascii_i64_overflow_stops_before_offending_digit() {
        let mut positive = TextScanner::new("9223372036854775808!");
        let err = positive.take_ascii_i64().unwrap_err();
        assert_matches!(err.kind, TextParseErrorKind::Overflow);
        assert_eq!(err.at.index.as_usize(), 18);
        assert_eq!(positive.pos().as_usize(), 18);
        assert_eq!(positive.peek_byte(), Some(b'8'));
        let mut negative = TextScanner::new("-9223372036854775809!");
        let err = negative.take_ascii_i64().unwrap_err();
        assert_matches!(err.kind, TextParseErrorKind::Overflow);
        assert_eq!(err.at.index.as_usize(), 19);
        assert_eq!(negative.pos().as_usize(), 19);
        assert_eq!(negative.peek_byte(), Some(b'9'));
    }
    #[test]
    fn ascii_u64_radix() {
        let mut hex = TextScanner::new("fFa0!");
        assert_eq!(hex.take_ascii_u64_radix(16), Ok(Some(0xFFA0)));
        assert_eq!(hex.peek_byte(), Some(b'!'));
        let mut binary = TextScanner::new("1011012");
        assert_eq!(binary.take_ascii_u64_radix(2), Ok(Some(0b101101)));
        assert_eq!(binary.peek_byte(), Some(b'2'));
        let mut octal = TextScanner::new("8");
        assert_eq!(octal.take_ascii_u64_radix(8), Ok(None));
        assert_eq!(octal.pos().as_usize(), 0);
    }
    #[test]
    fn ascii_u64_radix_overflow() {
        let mut max = TextScanner::new("FFFFFFFFFFFFFFFF!");
        assert_eq!(max.take_ascii_u64_radix(16), Ok(Some(u64::MAX)));
        assert_eq!(max.peek_byte(), Some(b'!'));
        let mut overflow = TextScanner::new("10000000000000000!");
        let err = overflow.take_ascii_u64_radix(16).unwrap_err();
        assert_matches!(err.kind, TextParseErrorKind::Overflow);
        assert_eq!(err.at.index.as_usize(), 16);
        assert_eq!(overflow.pos().as_usize(), 16);
        assert_eq!(overflow.peek_byte(), Some(b'0'));
    }
    #[test]
    fn ascii_i64_radix() {
        let mut max = TextScanner::new("7FFFFFFFFFFFFFFF!");
        assert_eq!(max.take_ascii_i64_radix(16), Ok(Some(i64::MAX)));
        assert_eq!(max.peek_byte(), Some(b'!'));
        let mut min = TextScanner::new("-8000000000000000!");
        assert_eq!(min.take_ascii_i64_radix(16), Ok(Some(i64::MIN)));
        assert_eq!(min.peek_byte(), Some(b'!'));
        let mut signed = TextScanner::new("+7f!");
        assert_eq!(signed.take_ascii_i64_radix(16), Ok(Some(127)));
        assert_eq!(signed.peek_byte(), Some(b'!'));
    }
    #[test]
    fn ascii_usize_edge_cases() {
        let mut none = TextScanner::new("x");
        assert_eq!(none.take_ascii_usize(), Ok(None));
        assert_eq!(none.pos().as_usize(), 0);
        #[cfg(target_pointer_width = "16")]
        let (max, overflow, overflow_at) = ("65535!", "65536!", 4);
        #[cfg(target_pointer_width = "32")]
        let (max, overflow, overflow_at) = ("4294967295!", "4294967296!", 9);
        #[cfg(target_pointer_width = "64")]
        let (max, overflow, overflow_at) = ("18446744073709551615!", "18446744073709551616!", 19);
        let mut s = TextScanner::new(max);
        assert_eq!(s.take_ascii_usize(), Ok(Some(usize::MAX)));
        assert_eq!(s.peek_byte(), Some(b'!'));
        let mut s = TextScanner::new(overflow);
        let err = s.take_ascii_usize().unwrap_err();
        assert_matches!(err.kind, TextParseErrorKind::Overflow);
        assert_eq!(err.at.index.as_usize(), overflow_at);
        assert_eq!(s.pos().as_usize(), overflow_at);
    }
    #[test]
    #[should_panic(expected = "ASCII integer radix must be in 2..=36")]
    fn ascii_integer_rejects_invalid_radix() {
        let mut s = TextScanner::new("10");
        let _ = s.take_ascii_u64_radix(1);
    }
}
