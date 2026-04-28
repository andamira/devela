// devela::text::parse::scanner::ascii

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
}
