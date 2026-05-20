// devela::text::parse::scanner::byte

use crate::{TextCursor, TextParseError, TextRange, TextScanner, TextUnit};
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

    /// Advances the scanner by one byte.
    ///
    /// Returns `true` if a byte was consumed.
    pub const fn skip_byte(&mut self) -> bool {
        self.advance(1) == 1
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

/// Byte-delimited range scanning
impl<'a> TextScanner<'a> {
    // MAYBE add: skip_until_byte
    /// Consumes and returns the range up to, but excluding, the next `byte`.
    ///
    /// If `byte` is not found, consumes to end of input.
    pub const fn take_until_byte(&mut self, byte: u8) -> TextRange {
        let start = self.mark();
        whilst! { let Some(found) = self.peek_byte(); {
            is! { found == byte, break }
            self._cursor_bump(1);
        }}
        self.range_from(start)
    }
    /// Consumes and returns the range up to, but excluding, the next occurrence of `bytes`.
    ///
    /// If `bytes` is not found, consumes to end of input.
    ///
    /// If `bytes` is empty, returns an empty range and does not advance.
    pub const fn take_until_bytes(&mut self, bytes: &[u8]) -> TextRange {
        let start = self.mark();
        is! { bytes.is_empty(), return self.range_from(start) }
        let hay = self.bytes;
        let needle_len = bytes.len();
        let pos0 = self.cursor.index.as_usize();
        if needle_len > hay.len().saturating_sub(pos0) {
            self._cursor_set(hay.len() as u32);
            return self.range_from(start);
        }
        whilst! { pos in pos0, ..=(hay.len() - needle_len); {
            let mut matched = true;
            whilst!{ i in 0..needle_len; {
                if hay[pos + i] != bytes[i] {
                    matched = false;
                    break;
                }
            }}
            if matched {
                self._cursor_set(pos as u32);
                return self.range_from(start);
            }
        }}
        self._cursor_set(hay.len() as u32);
        self.range_from(start)
    }

    /// Consumes and returns the range up to, but excluding, the next
    /// occurrence of either `byte1` or `byte2`.
    ///
    /// If neither byte is found, consumes to end of input.
    pub const fn take_until_any2(&mut self, byte1: u8, byte2: u8) -> TextRange {
        let start = self.mark();
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { byte == byte1 || byte == byte2, break }
            self._cursor_bump(1);
        }}
        self.range_from(start)
    }
    /// Consumes and returns the range up to, but excluding, the next
    /// occurrence of `byte1`, `byte2`, or `byte3`.
    ///
    /// If none of the bytes is found, consumes to end of input.
    pub const fn take_until_any3(&mut self, byte1: u8, byte2: u8, byte3: u8) -> TextRange {
        let start = self.mark();
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { byte == byte1 || byte == byte2 || byte == byte3, break }
            self._cursor_bump(1);
        }}
        self.range_from(start)
    }
}
