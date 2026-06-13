// devela/src/text/parse/scanner/scalar.rs

use crate::{Char, TextRange, TextScanner, TextUnit, charu, is, slice, unwrap, whilst};

/// Unicode scalar scanning.
impl<'a> TextScanner<'a> {
    /// Returns the next UTF-8 scalar without advancing.
    ///
    /// Returns `None` at EOF or if the unread input does not start with valid UTF-8.
    #[must_use]
    pub const fn peek_char(&self) -> Option<char> {
        let pos = self.cursor.index.as_usize();
        let Some((ch, _len)) = Char(self.bytes).to_char(pos) else { return None };
        Some(ch)
    }

    /// Returns the next UTF-8 scalar and advances past it.
    ///
    /// Returns `None` at EOF or if the unread input does not start with valid UTF-8.
    #[must_use]
    pub const fn next_char(&mut self) -> Option<char> {
        let pos = self.cursor.index.as_usize();
        let Some((ch, len)) = Char(self.bytes).to_char(pos) else { return None };
        self._cursor_bump(len as u32);
        Some(ch)
    }

    /// Consumes and returns the range of the next UTF-8 scalar.
    ///
    /// Returns `None` at EOF or if the unread input does not start with valid UTF-8.
    #[must_use]
    pub const fn take_char(&mut self) -> Option<TextRange> {
        let start = self.mark();
        let pos = self.cursor.index.as_usize();
        let Some((_ch, len)) = Char(self.bytes).to_char(pos) else { return None };
        self._cursor_bump(len as u32);
        Some(self.range_from(start))
    }

    /// Consumes `ch` if it is the next UTF-8 scalar.
    ///
    /// Returns `true` if the scalar was consumed.
    #[must_use]
    pub const fn eat_char(&mut self, ch: char) -> bool {
        let pos = self.cursor.index.as_usize();
        let Some((found, len)) = Char(self.bytes).to_char(pos) else { return false };
        if found == ch {
            self._cursor_bump(len as u32);
            true
        } else {
            false
        }
    }

    /// Consumes and returns the range of the next UTF-8 scalar if `f` accepts it.
    ///
    /// Returns `None` at EOF, on invalid UTF-8, or if `f` returns `false`.
    #[must_use]
    pub fn take_char_if<F: FnOnce(char) -> bool>(&mut self, f: F) -> Option<TextRange> {
        let start = self.mark();
        let pos = self.cursor.index.as_usize();
        let (ch, len) = unwrap![some? Char(self.bytes).to_char(pos)];
        if f(ch) {
            self._cursor_bump(len as u32);
            Some(self.range_from(start))
        } else {
            None
        }
    }

    /// Advances while the next UTF-8 scalar matches `f`.
    ///
    /// Returns the number of consumed bytes.
    pub fn skip_char_while<F: FnMut(char) -> bool>(&mut self, mut f: F) -> u32 {
        let start = self.cursor.index.0;
        loop {
            let pos = self.cursor.index.as_usize();
            let (ch, len) = unwrap![some_or Char(self.bytes).to_char(pos), break];
            is! { !f(ch), break }
            self._cursor_bump(len as u32);
        }
        self.cursor.index.0 - start
    }

    /// Consumes and returns the byte range of UTF-8 scalars matching `f`.
    pub fn take_char_while<F: FnMut(char) -> bool>(&mut self, mut f: F) -> TextRange {
        let start = self.mark();
        loop {
            let pos = self.cursor.index.as_usize();
            let Some((ch, len)) = Char(self.bytes).to_char(pos) else { break };
            is! { !f(ch), break }
            self._cursor_bump(len as u32);
        }
        self.range_from(start)
    }
}

/// Unicode scalar scanning with UTF-8 representation.
impl<'a> TextScanner<'a> {
    /// Returns the next UTF-8 scalar in UTF-8-backed scalar form without advancing.
    ///
    /// Returns `None` at EOF or if the unread input does not start
    /// with valid UTF-8.
    #[must_use]
    pub const fn peek_charu(&self) -> Option<charu> {
        let pos = self.cursor.index.as_usize();
        // let Some((_scalar, len)) = Char(self.bytes).to_scalar(pos) else { return None };
        // Some(charu::_from_utf8_prefix_trusted(slice![self.bytes, pos, ..], len))
        let Some((ch, _)) = charu::from_utf8_prefix(slice![self.bytes, pos, ..]) else {
            return None;
        };
        Some(ch)
    }

    /// Returns the next UTF-8 scalar in UTF-8-backed scalar form and advances past it.
    ///
    /// Returns `None` at EOF or if the unread input does not start
    /// with valid UTF-8.
    #[must_use]
    pub const fn next_charu(&mut self) -> Option<charu> {
        let pos = self.cursor.index.as_usize();
        // let Some((_scalar, len)) = Char(self.bytes).to_scalar(pos) else { return None };
        // let ch = charu::_from_utf8_prefix_trusted(slice![self.bytes, pos, ..], len);
        let Some((ch, len)) = charu::from_utf8_prefix(slice![self.bytes, pos, ..]) else {
            return None;
        };
        self._cursor_bump(len);
        Some(ch)
    }

    /// Consumes `expected` if it is the next UTF-8 scalar.
    ///
    /// This compares the encoded UTF-8 bytes directly.
    #[must_use]
    pub const fn eat_charu(&mut self, expected: charu) -> bool {
        let bytes = expected.to_utf8_bytes();
        let len = expected.len_utf8();
        let pos = self.cursor.index.as_usize();
        is![self.bytes.len().saturating_sub(pos) < len, return false];
        whilst! { i in 0..len; {
            is! { self.bytes[pos + i] != bytes[i], return false }
        }}
        self._cursor_bump(len as u32);
        true
    }

    /// Consumes and returns the range of the next UTF-8 scalar if `f` accepts it.
    ///
    /// Returns `None` at EOF, on invalid UTF-8, or if `f` returns `false`.
    #[must_use]
    pub fn take_charu_if<F: FnOnce(charu) -> bool>(&mut self, f: F) -> Option<TextRange> {
        let start = self.mark();
        let pos = self.cursor.index.as_usize();
        let (_scalar, len) = unwrap![some? Char(self.bytes).to_char(pos)];
        let ch = charu::_from_utf8_prefix_trusted(slice![self.bytes, pos, ..], len);
        if f(ch) {
            self._cursor_bump(len as u32);
            Some(self.range_from(start))
        } else {
            None
        }
    }

    /// Advances while the next UTF-8 scalar matches `f`.
    ///
    /// Returns the number of consumed bytes.
    pub fn skip_charu_while<F: FnMut(charu) -> bool>(&mut self, mut f: F) -> TextUnit {
        let start = self.cursor.index.0;
        loop {
            let pos = self.cursor.index.as_usize();
            let (_scalar, len) = unwrap![some_or Char(self.bytes).to_scalar(pos), break];
            let ch = charu::_from_utf8_prefix_trusted(slice![self.bytes, pos, ..], len);
            is![!f(ch), break];
            self._cursor_bump(len as u32);
        }
        self.cursor.index.0 - start
    }

    /// Consumes and returns the byte range of UTF-8 scalars matching `f`.
    pub fn take_charu_while<F: FnMut(charu) -> bool>(&mut self, mut f: F) -> TextRange {
        let start = self.mark();
        loop {
            let pos = self.cursor.index.as_usize();
            let (_scalar, len) = unwrap![some_or Char(self.bytes).to_scalar(pos), break];
            let ch = charu::_from_utf8_prefix_trusted(slice![self.bytes, pos, ..], len);
            is![!f(ch), break];
            self._cursor_bump(len as u32);
        }
        self.range_from(start)
    }
}
