// devela::text::parse::scanner::until

use crate::{TextScanner, TextRange};
use crate::{is, whilst};

/// Byte-delimited range scanning
impl<'a> TextScanner<'a> {
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
