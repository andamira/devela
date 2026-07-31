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

#[cfg(test)]
mod _test {
    use crate::{TextScanner, charu};

    #[test]
    fn scanner_utf8_char_peek_next_take_and_eat() {
        let mut s = TextScanner::new("aé€🐛z");
        assert_eq!(s.pos().as_usize(), 0);
        assert_eq!(s.peek_char(), Some('a'));
        assert_eq!(s.peek_char(), Some('a'));
        assert_eq!(s.next_char(), Some('a'));
        assert_eq!(s.pos().as_usize(), 1);
        let r = s.take_char().unwrap();
        assert_eq!(s.str_at(r), "é");
        assert_eq!(s.pos().as_usize(), 3);
        assert!(s.eat_char('€'));
        assert_eq!(s.pos().as_usize(), 6);
        let r = s.take_char().unwrap();
        assert_eq!(s.str_at(r), "🐛");
        assert_eq!(s.pos().as_usize(), 10);
        assert_eq!(s.next_char(), Some('z'));
        assert_eq!(s.next_char(), None);
        assert!(s.is_eof());
    }
    #[test]
    fn scanner_utf8_char_predicates() {
        let mut s = TextScanner::new("αβγ123");
        let r = s.take_char_if(|ch| ch == 'α').unwrap();
        assert_eq!(s.str_at(r), "α");
        assert!(s.take_char_if(|ch| ch.is_ascii_digit()).is_none());
        assert_eq!(s.peek_char(), Some('β'));
        let letters = s.take_char_while(|ch| ch.is_alphabetic());
        assert_eq!(s.str_at(letters), "βγ");
        assert_eq!(s.skip_char_while(|ch| ch.is_ascii_digit()), 3);
        assert!(s.is_eof());
    }
    #[test]
    fn scanner_utf8_invalid_input_does_not_advance() {
        let mut s = TextScanner::from_bytes(b"a\xFFz");
        assert_eq!(s.next_char(), Some('a'));
        assert_eq!(s.pos().as_usize(), 1);
        assert_eq!(s.peek_char(), None);
        assert_eq!(s.next_char(), None);
        assert!(s.take_char().is_none());
        assert!(!s.eat_char('z'));
        assert!(s.take_char_if(|_| true).is_none());
        // Strict UTF-8 scalar methods stop before the invalid byte.
        assert_eq!(s.pos().as_usize(), 1);
        // Byte-level methods remain available for recovery.
        assert_eq!(s.next_byte(), Some(0xFF));
        assert_eq!(s.next_char(), Some('z'));
        assert!(s.is_eof());
    }
    /* Unicode scalar scanning, with UTF-8 representation */
    #[test]
    fn scanner_utf8_charu_peek_next_and_eat() {
        let mut s = TextScanner::new("aé€🐛z");
        assert_eq!(s.peek_charu(), Some(charu::from_char('a')));
        assert_eq!(s.peek_charu(), Some(charu::from_char('a')));
        assert_eq!(s.next_charu(), Some(charu::from_char('a')));
        assert_eq!(s.pos().as_usize(), 1);
        assert!(s.eat_charu(charu::from_char('é')));
        assert_eq!(s.pos().as_usize(), 3);
        assert!(s.eat_charu(charu::from_char('€')));
        assert_eq!(s.pos().as_usize(), 6);
        assert_eq!(s.next_charu(), Some(charu::from_char('🐛')));
        assert_eq!(s.pos().as_usize(), 10);
        assert!(!s.eat_charu(charu::from_char('x')));
        assert_eq!(s.next_charu(), Some(charu::from_char('z')));
        assert_eq!(s.next_charu(), None);
        assert!(s.is_eof());
    }
    #[test]
    fn scanner_utf8_charu_predicates() {
        let mut s = TextScanner::new("αβγ123");
        let r = s.take_charu_if(|ch| ch == charu::from_char('α')).unwrap();
        assert_eq!(s.str_at(r), "α");
        assert!(s.take_charu_if(|ch| ch == charu::from_char('1')).is_none());
        assert_eq!(s.peek_charu(), Some(charu::from_char('β')));
        let letters = s.take_charu_while(|ch| !ch.to_char().is_ascii_digit());
        assert_eq!(s.str_at(letters), "βγ");
        assert_eq!(s.skip_charu_while(|ch| ch.to_char().is_ascii_digit()), 3);
        assert!(s.is_eof());
    }
    #[test]
    fn scanner_utf8_charu_invalid_input_does_not_advance() {
        let mut s = TextScanner::from_bytes(b"a\xFFz");
        assert_eq!(s.next_charu(), Some(charu::from_char('a')));
        assert_eq!(s.pos().as_usize(), 1);
        assert_eq!(s.peek_charu(), None);
        assert_eq!(s.next_charu(), None);
        assert!(s.take_charu_if(|_| true).is_none());
        assert_eq!(s.skip_charu_while(|_| true), 0);
        // Strict UTF-8 scalar methods stop before the invalid byte.
        assert_eq!(s.pos().as_usize(), 1);
        // Byte-level recovery remains possible.
        assert_eq!(s.next_byte(), Some(0xFF));
        assert_eq!(s.next_charu(), Some(charu::from_char('z')));
        assert!(s.is_eof());
    }
}
