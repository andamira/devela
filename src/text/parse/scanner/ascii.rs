// devela/src/text/parse/scanner/ascii.rs

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{AsciiSet, TextRange, TextScanner, TextUnit};
use crate::{is, unwrap, whilst};

/// ASCII whitespace, identifiers.
impl<'a> TextScanner<'a> {
    /// Skips ASCII horizontal and vertical whitespace.
    ///
    /// Currently skips: space, tab, line feed, carriage return,
    /// form feed, and vertical tab.
    pub const fn skip_ascii_ws(&mut self) {
        whilst! { let Some(byte) = self.peek_byte(); {
            match byte {
                b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B => self._cursor_bump(1),
                _ => break,
            }
        }}
    }
    /// Skips ASCII horizontal whitespace.
    ///
    /// Currently skips: space, tab.
    pub const fn skip_ascii_hws(&mut self) {
        whilst! { let Some(byte) = self.peek_byte(); {
            match byte {
                b' ' | b'\t' => self._cursor_bump(1),
                _ => break,
            }
        }}
    }

    /// Consumes and returns an ASCII identifier range.
    ///
    /// Identifier syntax:
    /// - first byte: `A..=Z`, `a..=z`, or `_`
    /// - following bytes: ASCII alphanumeric or `_`
    ///
    /// Returns `None` if the next byte is not a valid identifier start.
    #[must_use]
    pub const fn take_ascii_ident(&mut self) -> Option<TextRange> {
        self.take_ascii_run(AsciiSet::IDENT_HEAD, AsciiSet::IDENT_TAIL)
    }

    /// Consumes and returns an ASCII identifier-tail range.
    ///
    /// Identifier-tail syntax:
    /// - bytes: ASCII alphanumeric or `_`
    ///
    /// Unlike [`take_ascii_ident`][Self::take_ascii_ident],
    /// this accepts digits as the first consumed byte.
    ///
    /// Returns `None` if no identifier-tail byte was consumed.
    #[must_use]
    pub const fn take_ascii_ident_tail(&mut self) -> Option<TextRange> {
        self.take_ascii_set(AsciiSet::IDENT_TAIL)
    }

    /// Returns `range` trimmed of leading and trailing ASCII whitespace.
    ///
    /// Currently trims: space, tab, line feed, carriage return,
    /// form feed, and vertical tab.
    pub const fn trim_ascii_ws(&self, range: TextRange) -> TextRange {
        let mut start = range.start.0;
        let mut end = range.end.0;

        whilst! { start < end; {
            match self.bytes[start as usize] {
                b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B => start += 1,
                _ => break,
            }
        }}
        whilst! { start < end; {
            match self.bytes[(end - 1) as usize] {
                b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B => end -= 1,
                _ => break,
            }
        }}
        TextRange::from_prim(start, end)
    }

    /// Returns `range` trimmed of leading and trailing ASCII horizontal whitespace.
    ///
    /// Currently trims: space, tab.
    pub const fn trim_ascii_hws(&self, range: TextRange) -> TextRange {
        let mut start = range.start.0;
        let mut end = range.end.0;

        whilst! { start < end; {
            match self.bytes[start as usize] {
                b' ' | b'\t' => start += 1,
                _ => break,
            }
        }}
        whilst! { start < end; {
            match self.bytes[(end - 1) as usize] {
                b' ' | b'\t' => end -= 1,
                _ => break,
            }
        }}
        TextRange::from_prim(start, end)
    }
}

/// `AsciiSet` scanning.
impl<'a> TextScanner<'a> {
    /// Consumes the next byte if it belongs to `set`.
    ///
    /// Returns `true` if a byte was consumed.
    ///
    /// Non-ASCII bytes never match.
    #[must_use]
    pub const fn eat_ascii_set(&mut self, set: AsciiSet) -> bool {
        match self.peek_byte() {
            Some(byte) if set.contains_byte(byte) => {
                self._cursor_bump(1);
                true
            }
            _ => false,
        }
    }

    /// Skips bytes while they belong to `set`.
    ///
    /// Returns the number of skipped bytes.
    pub const fn skip_ascii_set(&mut self, set: AsciiSet) -> TextUnit {
        let start = self.cursor.index.0;
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { set.contains_byte(byte), self._cursor_bump(1), break }
        }}
        self.cursor.index.0 - start
    }

    /// Skips bytes until the next byte belonging to `set`.
    ///
    /// Stops before the matching byte.
    ///
    /// If no byte from `set` is found, skips to the end of input.
    ///
    /// Non-ASCII bytes never match.
    pub const fn skip_until_ascii_set(&mut self, set: AsciiSet) -> TextUnit {
        let start = self.cursor.index.0;
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { set.contains_byte(byte), break }
            self._cursor_bump(1);
        }}
        self.cursor.index.0 - start
    }

    /// Consumes and returns a range of bytes belonging to `set`.
    ///
    /// Returns `None` if no byte was consumed.
    #[must_use]
    pub const fn take_ascii_set(&mut self, set: AsciiSet) -> Option<TextRange> {
        let start = self.mark();
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { set.contains_byte(byte), self._cursor_bump(1), break }
        }}
        is! { self.cursor.index.0 == start.index.0, None, Some(self.range_from(start)) }
    }

    /// Consumes and returns an ASCII run with distinct head and tail sets.
    ///
    /// The first byte must belong to `head`.
    /// Following bytes may belong to `tail`.
    ///
    /// Returns `None` if the first byte does not belong to `head`.
    #[must_use]
    pub const fn take_ascii_run(&mut self, head: AsciiSet, tail: AsciiSet) -> Option<TextRange> {
        let start = self.mark();
        // let Some(byte) = self.peek_byte() else { return None; };
        // let byte = is![let Some(byte) = self.peek_byte(), byte, return None];
        let byte = unwrap![some? self.peek_byte()];
        is! { !head.contains_byte(byte), return None }
        self._cursor_bump(1);
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { tail.contains_byte(byte), self._cursor_bump(1), break }
        }}
        Some(self.range_from(start))
    }

    /// Consumes and returns the range up to, but excluding, the next byte belonging to `set`.
    ///
    /// If no byte from `set` is found, consumes to the end of input.
    ///
    /// Non-ASCII bytes never match.
    pub const fn take_until_ascii_set(&mut self, set: AsciiSet) -> TextRange {
        let start = self.mark();
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { set.contains_byte(byte), break }
            self._cursor_bump(1);
        }}
        self.range_from(start)
    }
}

#[cfg(test)]
mod _test {
    use crate::{AsciiSet, TextScanner};

    #[test]
    fn ascii_scanners() {
        let mut s = TextScanner::new(" \t\r\nfoo_12 123bar");
        s.skip_ascii_ws();
        assert_eq!(s.pos().as_usize(), 4);
        let ident = s.take_ascii_ident().unwrap();
        assert_eq!(s.slice_str(ident), Some("foo_12"));
        assert_eq!(s.peek_byte(), Some(b' '));
        s.skip_ascii_ws();
        assert_eq!(s.expect_ascii_u64(), Ok(123));
        assert_eq!(s.rest(), b"bar");
    }
    #[test]
    fn scanner_ascii_set_consumes_runs_and_delimiters() {
        let mut s = TextScanner::from_bytes(b"abc=123;\xC3\xB1!");
        let alpha = s.take_ascii_set(AsciiSet::ALPHA).unwrap();
        assert_eq!(s.str_at(alpha), "abc");
        assert!(s.eat_ascii_set(AsciiSet::PUNCT));
        assert_eq!(s.peek_byte(), Some(b'1'));
        let digits = s.take_ascii_set(AsciiSet::DIGIT).unwrap();
        assert_eq!(s.slice(digits), b"123");
        assert_eq!(s.skip_until_ascii_set(AsciiSet::PUNCT), 0);
        assert!(s.eat_ascii_set(AsciiSet::PUNCT));
        // Non-ASCII bytes do not match the ASCII set, but are still skipped over.
        assert_eq!(s.skip_until_ascii_set(AsciiSet::PUNCT), 2);
        assert_eq!(s.peek_byte(), Some(b'!'));
        assert!(s.eat_ascii_set(AsciiSet::PUNCT));
        assert!(s.is_eof());
    }
    #[test]
    fn scanner_ascii_set_takes_until_and_runs() {
        let mut s = TextScanner::new("name:value rest");
        let head = s.take_until_ascii_set(AsciiSet::PUNCT);
        assert_eq!(s.str_at(head), "name");
        assert_eq!(s.peek_byte(), Some(b':'));
        assert!(s.eat_ascii_set(AsciiSet::PUNCT));
        let tail = s.take_until_ascii_set(AsciiSet::PUNCT);
        assert_eq!(s.str_at(tail), "value rest");
        assert!(s.is_eof());
        let mut id = TextScanner::new("9bad _good");
        assert!(id.take_ascii_run(AsciiSet::IDENT_HEAD, AsciiSet::IDENT_TAIL).is_none());
        assert_eq!(id.pos().as_usize(), 0);
        let tail = id.take_ascii_set(AsciiSet::IDENT_TAIL).unwrap();
        assert_eq!(id.str_at(tail), "9bad");
        id.skip_ascii_hws();
        let ident = id.take_ascii_run(AsciiSet::IDENT_HEAD, AsciiSet::IDENT_TAIL).unwrap();
        assert_eq!(id.str_at(ident), "_good");
        assert!(id.is_eof());
    }
}
