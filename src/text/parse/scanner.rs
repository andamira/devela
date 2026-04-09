// devela::text::parse::scanner
//
//! Defines [`TextScanner`].
//

#![allow(unused, missing_docs)]

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{_impl_init, ConstInitCore, Slice, Str, is, unwrap, whilst};
use crate::{InvalidUtf8, TextCursor, TextIndex, TextParseError, TextRange, TextUnit};

#[must_use]
#[doc = crate::_tags!(text)]
/// A byte scanner over source text.
#[doc = crate::_doc_location!("text/parse")]
///
/// `TextScanner` provides incremental, allocation-free traversal over a borrowed
/// text source, exposing byte-oriented operations suitable for building parsers.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextScanner<'a> {
    bytes: &'a [u8],
    cursor: TextCursor,
}
_impl_init![ConstInitCore: Self::new("") => TextScanner<'_>];

// private cursor helpers
impl<'a> TextScanner<'a> {
    pub(super) const fn _cursor_bump(&mut self, n: u32) {
        self.cursor = TextCursor::new_prim(self.cursor.index.0 + n);
    }
    pub(super) const fn _cursor_set(&mut self, pos: u32) {
        self.cursor = TextCursor::new_prim(pos);
    }
}

/// Construction and views.
impl<'a> TextScanner<'a> {
    /* constructors */

    /// Creates a scanner over a string slice.
    pub const fn new(string: &'a str) -> Self {
        Self::from_bytes(string.as_bytes())
    }
    /// Creates a scanner over a byte slice.
    pub const fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: TextCursor::INIT }
    }

    /* source views */

    /// Returns the byte slice covered by `range`.
    #[must_use]
    pub const fn slice(&self, range: TextRange) -> &'a [u8] {
        Slice::range(self.bytes, range.start.as_usize(), range.end.as_usize())
    }

    /// Returns the string slice covered by `range`, if valid UTF-8.
    #[must_use]
    pub const fn slice_str(&self, range: TextRange) -> Option<&'a str> {
        unwrap![ok_some Str::from_utf8(self.slice(range))]
    }
    /// Returns the string slice covered by `range`, without checking if it's valid UTF-8.
    ///
    /// # Safety
    /// The caller must ensure the covered bytes are valid UTF-8.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    pub const unsafe fn slice_str_unchecked(&self, range: TextRange) -> &'a str {
        unsafe { Str::from_utf8_unchecked(self.slice(range)) }
    }

    /// Returns the remaining unread bytes.
    #[must_use]
    pub const fn rest(&self) -> &'a [u8] {
        let pos = self.cursor.index.as_usize();
        Slice::range_from(self.bytes, pos)
    }
}

/// Cursor state and range construction.
impl<'a> TextScanner<'a> {
    /// Returns the current cursor position.
    pub const fn pos(&self) -> TextIndex {
        self.cursor.index
    }
    /// Returns the current cursor for later range construction.
    pub const fn mark(&self) -> TextCursor {
        self.cursor
    }

    /// Returns the number of unread bytes remaining.
    #[must_use]
    pub const fn remaining_len(&self) -> TextUnit {
        let pos = self.cursor.index.as_usize();
        self.bytes.len().saturating_sub(pos) as TextUnit
    }

    /// Advances the cursor by up to `n` bytes.
    ///
    /// Returns the number of bytes actually consumed.
    ///
    /// If fewer than `n` bytes remain, advances to end of input.
    #[must_use]
    pub const fn advance(&mut self, n: TextUnit) -> TextUnit {
        let consumed = crate::Cmp(n).min(self.remaining_len());
        self._cursor_bump(consumed);
        consumed
    }

    /// Returns `true` if the scanner reached the end of input.
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.cursor.index.as_usize() >= self.bytes.len()
    }
    /// Returns the half-open range from `mark` to the current cursor.
    ///
    /// # Panics
    /// Panics if `mark` is ahead of the current cursor.
    pub const fn range_from(&self, mark: TextCursor) -> TextRange {
        TextRange::new(mark.index, self.cursor.index)
    }
}

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
        let start = self.mark();
        let Some(first) = self.peek_byte() else {
            return None;
        };
        match first {
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => self._cursor_bump(1),
            _ => return None,
        }
        whilst! { let Some(byte) = self.peek_byte(); {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' => self._cursor_bump(1),
                _ => break,
            }
        }}
        Some(self.range_from(start))
    }

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

    /// Consumes and returns the remaining unread range.
    ///
    /// After this call, the scanner is at end of input.
    pub const fn take_rest(&mut self) -> TextRange {
        let start = self.mark();
        self.advance(self.remaining_len());
        self.range_from(start)
    }
}

/// Quoted string scanning and decoding.
impl<'a> TextScanner<'a> {
    /// Consumes a double-quoted string and returns the inner byte range.
    ///
    /// The returned range excludes the surrounding quotes.
    ///
    /// Recognized escapes:
    /// `\"`, `\\`, `\n`, `\r`, `\t`, `\0`
    ///
    /// This method validates escape syntax but does not decode escapes.
    ///
    /// Returns `Ok(None)` if the next byte is not `"` .
    pub const fn take_quoted_basic(&mut self) -> Result<Option<TextRange>, TextParseError> {
        is! { !self.eat_byte(b'"'), return Ok(None) }
        let start = self.mark();
        loop {
            let Some(byte) = self.peek_byte() else {
                return Err(TextParseError::unterminated_quote(self.cursor));
            };
            match byte {
                b'"' => {
                    let range = self.range_from(start);
                    self._cursor_bump(1);
                    return Ok(Some(range));
                }
                b'\\' => {
                    self._cursor_bump(1);
                    let Some(esc) = self.peek_byte() else {
                        return Err(TextParseError::unterminated_quote(self.cursor));
                    };
                    match esc {
                        b'"' | b'\\' | b'n' | b'r' | b't' | b'0' => self._cursor_bump(1),
                        _ => return Err(TextParseError::invalid_escape(self.cursor)),
                    }
                }
                _ => self._cursor_bump(1),
            }
        }
    }

    /// Consumes a single-quoted literal string and returns the inner byte range.
    ///
    /// The returned range excludes the surrounding quotes.
    ///
    /// Backslashes have no special meaning here.
    ///
    /// Returns `Ok(None)` if the next byte is not `'`.
    pub const fn take_quoted_literal(&mut self) -> Result<Option<TextRange>, TextParseError> {
        is! { !self.eat_byte(b'\''), return Ok(None) }
        let start = self.mark();
        loop {
            let Some(byte) = self.peek_byte() else {
                return Err(TextParseError::unterminated_quote(self.cursor));
            };
            match byte {
                b'\'' => {
                    let range = self.range_from(start);
                    self._cursor_bump(1);
                    return Ok(Some(range));
                }
                _ => self._cursor_bump(1),
            }
        }
    }

    /// Decodes a basic quoted string payload into `out`.
    ///
    /// The input `range` is expected to be the inner range returned by
    /// [`take_quoted_basic`][Self::take_quoted_basic], excluding the surrounding quotes.
    ///
    /// Recognized escapes:
    /// `\"`, `\\`, `\n`, `\r`, `\t`, `\0`
    ///
    /// Returns the number of bytes written into `out`.
    ///
    /// # Errors
    /// Returns:
    /// - [`InvalidEscape`][TextParseErrorKind::InvalidEscape] for an unknown escape
    /// - [`UnterminatedQuote`][TextParseErrorKind::UnterminatedQuote] if the range ends
    ///   with a trailing backslash
    /// - [`BufferTooSmall`][TextParseErrorKind::BufferTooSmall] if `out` is too small
    pub const fn decode_quoted_basic_into(
        &self,
        range: TextRange,
        out: &mut [u8],
    ) -> Result<usize, TextParseError> {
        let src = self.slice(range);
        let mut j = 0;
        whilst! { i in 0..src.len(); {
            if j >= out.len() {
                let at = TextCursor::new_prim(range.start.0 + i as u32);
                return Err(TextParseError::buffer_too_small(at));
            }
            let byte = src[i];
            if byte != b'\\' {
                out[j] = byte;
                i += 1;
                j += 1;
                continue;
            }
            // backslash escape
            i += 1;
            if i >= src.len() {
                let cursor = TextCursor::new_prim(range.start.0 + i as u32);
                return Err(TextParseError::unterminated_quote(cursor));
            }
            let decoded = match src[i] {
                b'"' => b'"',
                b'\\' => b'\\',
                b'n' => b'\n',
                b'r' => b'\r',
                b't' => b'\t',
                b'0' => b'\0',
                _ => {
                    let cursor = TextCursor::new_prim(range.start.0 + i as u32);
                    return Err(TextParseError::invalid_escape(cursor));
                }
            };
            out[j] = decoded;
            j += 1;
        }}
        Ok(j)
    }

    /// Decodes a basic quoted string payload into `out` and returns it as `&str`.
    ///
    /// This is a convenience wrapper over
    /// [`decode_quoted_basic_into`][Self::decode_quoted_basic_into].
    ///
    /// # Errors
    /// Returns the same decoding errors as `decode_quoted_basic_into`.
    /// Returns [`InvalidEscape`][TextParseErrorKind::InvalidEscape] as a fallback
    /// if the decoded bytes are not valid UTF-8.
    pub const fn decode_quoted_basic_str_into<'str>(
        &self,
        range: TextRange,
        out: &'str mut [u8],
    ) -> Result<&'str str, TextParseError> {
        let len = match self.decode_quoted_basic_into(range, out) {
            Ok(len) => len,
            Err(err) => return Err(err),
        };
        match crate::Str::from_utf8(Slice::range_to(out, len)) {
            Ok(s) => Ok(s),
            Err(err) => Err(TextParseError::invalid_utf8(TextCursor::new(range.start), err)),
        }
    }
}

/// Predicate-driven scanning adapters.
impl<'a> TextScanner<'a> {
    /// Consumes the next byte if it satisfies `f`.
    ///
    /// Returns `true` if a byte was consumed.
    ///
    /// This is the predicate-based counterpart to
    /// [`eat_byte`][Self::eat_byte].
    #[must_use]
    pub fn eat_if<F: FnOnce(u8) -> bool>(&mut self, f: F) -> bool {
        match self.peek_byte() {
            Some(found) if f(found) => {
                self._cursor_bump(1);
                true
            }
            _ => false,
        }
    }

    /// Advances while `byte` matches the predicate.
    ///
    /// Returns the number of consumed bytes.
    pub fn skip_while<F: FnMut(u8) -> bool>(&mut self, mut f: F) -> TextUnit {
        let start = self.cursor.index.0;
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { !f(byte), break }
            self._cursor_bump(1);
        }}
        self.cursor.index.0 - start
    }

    /// Consumes and returns the half-open range of bytes matching the predicate.
    pub fn take_while<F: FnMut(u8) -> bool>(&mut self, mut f: F) -> TextRange {
        let start = self.mark();
        whilst! { let Some(byte) = self.peek_byte(); {
            is! { !f(byte), break }
            self._cursor_bump(1);
        }}
        self.range_from(start)
    }
}
