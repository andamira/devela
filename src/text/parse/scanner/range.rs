// devela::text::parse::scanner::range

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{AsciiSet, TextParseError, TextRange, TextScanner, TextUnit};
use crate::{is, unwrap, whilst};

/// ASCII scanning and range-taking operations.
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
}

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

    /// Consumes and returns the range up to, but excluding, the next line ending.
    ///
    /// Stops before either `\\n` or `\\r`.
    /// If no line ending is found, consumes to end of input.
    pub const fn take_until_eol(&mut self) -> TextRange {
        let start = self.mark();
        whilst! { let Some(byte) = self.peek_byte(); {
            match byte {
                b'\n' | b'\r' => break,
                _ => self._cursor_bump(1),
            }
        }}
        self.range_from(start)
    }

    /// Consumes and returns the next line, excluding its line ending.
    ///
    /// Consumes a following `\n`, `\r`, or `\r\n` if present.
    ///
    /// Returns `None` at end of input.
    #[must_use]
    pub const fn next_line(&mut self) -> Option<TextRange> {
        is! { self.is_eof(), return None }
        let range = self.take_until_eol();
        let _ = self.eat_eol();
        Some(range)
    }

    /// Consumes and returns the next line trimmed of ASCII horizontal whitespace.
    ///
    /// Currently trims: space, tab.
    ///
    /// Returns `None` at end of input.
    #[must_use]
    pub const fn next_line_trimmed(&mut self) -> Option<TextRange> {
        let Some(line) = self.next_line() else {
            return None;
        };
        Some(self.trim_ascii_hws(line))
    }

    /// Consumes and returns the next line before `byte`, trimmed of ASCII horizontal whitespace.
    ///
    /// The delimiter byte is not consumed as part of the returned range, but the
    /// full source line and its line ending are consumed.
    ///
    /// This is useful for simple line-comment formats, for example with `b'#'`.
    /// It is byte-blind and does not understand quoted strings.
    ///
    /// Returns `None` at end of input.
    #[must_use]
    pub const fn next_line_trimmed_before(&mut self, byte: u8) -> Option<TextRange> {
        let Some(line) = self.next_line() else {
            return None;
        };
        let mut end = line.end.0;
        whilst! { i in line.start.0,..line.end.0; {
            if self.bytes[i as usize] == byte {
                end = i;
                break;
            }
        }}
        Some(self.trim_ascii_hws(TextRange::from_prim(line.start.0, end)))
    }

    /// Consumes and returns the remaining unread range.
    ///
    /// After this call, the scanner is at end of input.
    pub const fn take_rest(&mut self) -> TextRange {
        let start = self.mark();
        let _ = self.advance(self.remaining_len());
        self.range_from(start)
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
