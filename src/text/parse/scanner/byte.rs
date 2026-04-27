// devela::text::parse::scanner::byte

use crate::{TextCursor, TextParseError, TextScanner, TextUnit};
use crate::{is, whilst};

/// Byte inspection and exact consumption.
impl<'a> TextScanner<'a> {
    /// Returns the next byte without advancing.
    #[must_use]
    pub const fn peek_byte(&self) -> Option<u8> {
        let pos = self.cursor.index.as_usize();
        is! { pos < self.bytes.len(), Some(self.bytes[pos]), None }
    }
    /// Returns the byte at `offset` from the current cursor, without advancing.
    ///
    /// `offset == 0` returns the same value as [`peek_byte`][Self::peek_byte].
    #[must_use]
    pub const fn peek_byte_at(&self, offset: TextUnit) -> Option<u8> {
        let start = self.cursor.index.as_usize();
        let off = offset as usize;
        match start.checked_add(off) {
            Some(pos) if pos < self.bytes.len() => Some(self.bytes[pos]),
            _ => None,
        }
    }

    /// Returns `true` if the unread input starts with `bytes`.
    ///
    /// This method does not advance the scanner.
    #[must_use]
    pub const fn starts_with(&self, bytes: &[u8]) -> bool {
        let start = self.cursor.index.as_usize();
        let end = start + bytes.len();
        is! { end > self.bytes.len(), return false }
        whilst! { i in 0..bytes.len(); {
            is! { self.bytes[start + i] != bytes[i], return false }
        }}
        true
    }

    /// Returns the next byte and advances the scanner.
    #[must_use]
    pub const fn next_byte(&mut self) -> Option<u8> {
        let pos = self.cursor.index.as_usize();
        if pos < self.bytes.len() {
            let byte = self.bytes[pos];
            self._cursor_bump(1);
            Some(byte)
        } else {
            None
        }
    }

    #[must_use]
    /// Consumes `byte` if it is next.
    pub const fn eat_byte(&mut self, byte: u8) -> bool {
        match self.peek_byte() {
            Some(found) if found == byte => {
                self._cursor_bump(1);
                true
            }
            _ => false,
        }
    }
    /// Consumes `bytes` if they are next.
    #[must_use]
    pub const fn eat_bytes(&mut self, bytes: &[u8]) -> bool {
        let start = self.cursor.index.as_usize();
        let end = start + bytes.len();
        is! { end > self.bytes.len(), return false }
        whilst! { i in 0..bytes.len(); {
            is! { self.bytes[start + i] != bytes[i], return false }
        }}
        self._cursor_bump(bytes.len() as u32);
        true
    }

    /// Consumes a single line ending.
    ///
    /// Accepts `\n`, `\r`, or `\r\n`.
    ///
    /// Returns `true` if a line ending was consumed.
    #[must_use]
    pub const fn eat_eol(&mut self) -> bool {
        let pos = self.cursor.index.as_usize();
        is! { pos >= self.bytes.len(), return false }
        match self.bytes[pos] {
            b'\n' => {
                self._cursor_bump(1);
                true
            }
            b'\r' => {
                if pos + 1 < self.bytes.len() && self.bytes[pos + 1] == b'\n' {
                    self._cursor_bump(2);
                } else {
                    self._cursor_bump(1);
                }
                true
            }
            _ => false,
        }
    }

    /// Consumes `byte` if it is next, or returns an error.
    pub const fn expect_byte(&mut self, byte: u8) -> Result<(), TextParseError> {
        match self.peek_byte() {
            Some(found) if found == byte => {
                self._cursor_bump(1);
                Ok(())
            }
            found => Err(TextParseError::unexpected_byte(self.cursor, byte, found)),
        }
    }
    /// Consumes `bytes` if they are next, or returns an error.
    pub const fn expect_bytes(&mut self, bytes: &[u8]) -> Result<(), TextParseError> {
        let start = self.cursor.index.as_usize();
        let end = start + bytes.len();
        is! { end > self.bytes.len(), return Err(TextParseError::unexpected_eof(self.cursor)) }
        whilst! { i in 0..bytes.len(); {
            let found = self.bytes[start + i];
            let expected = bytes[i];
            if found != expected {
                let at = TextCursor::new_prim(self.cursor.index.0 + i as u32);
                return Err(TextParseError::unexpected_byte(at, expected, Some(found)));
            }
        }}
        self._cursor_bump(bytes.len() as u32);
        Ok(())
    }
}
