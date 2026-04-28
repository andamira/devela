// devela::text::parse::scanner::number
//
// FUTURE TODO
// - take_ascii_i64
// - take_ascii_usize
// - take_ascii_radix_u64

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{TextParseError, TextScanner};
use crate::{unwrap, whilst};

/// ASCII numeric parsing.
impl<'a> TextScanner<'a> {
    /// Consumes an ASCII unsigned 64-bit integer prefix.
    ///
    /// Returns:
    /// - `Ok(Some(value))` if one or more digits were consumed
    /// - `Ok(None)` if the next byte is not an ASCII digit
    /// - `Err(...)` on numeric overflow
    ///
    /// On overflow, the scanner stops at the offending digit.
    pub const fn take_ascii_u64(&mut self) -> Result<Option<u64>, TextParseError> {
        let mut value = 0u64;
        let mut saw_digit = false;
        whilst! { let Some(byte) = self.peek_byte(); {
            match byte {
                b'0'..=b'9' => {
                    let digit = (byte - b'0') as u64;
                    let Some(v) = value.checked_mul(10) else {
                        return Err(TextParseError::overflow(self.cursor));
                    };
                    let Some(v) = v.checked_add(digit) else {
                        return Err(TextParseError::overflow(self.cursor));
                    };
                    value = v;
                    self._cursor_bump(1);
                    saw_digit = true;
                }
                _ => break,
            }
        }}
        if saw_digit { Ok(Some(value)) } else { Ok(None) }
    }
    /// Consumes an ASCII unsigned integer prefix, or returns an error.
    ///
    /// Returns [`InvalidDigit`][TextParseErrorKind::InvalidDigit] if the next
    /// byte is not an ASCII digit.
    pub const fn expect_ascii_u64(&mut self) -> Result<u64, TextParseError> {
        match unwrap![ok self.take_ascii_u64()] {
            Some(value) => Ok(value),
            None => Err(TextParseError::invalid_digit(self.cursor)),
        }
    }
}
