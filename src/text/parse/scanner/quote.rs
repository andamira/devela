// devela::text::parse::scanner::quote

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{Slice, is, whilst};
use crate::{TextCursor, TextParseError, TextRange, TextScanner};

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

    /// Consumes a basic double-quoted string, or the remaining unread input.
    ///
    /// If the next byte is `"`, returns the inner quoted range, validating basic
    /// escapes as [`take_quoted_basic`][Self::take_quoted_basic] does.
    ///
    /// Otherwise consumes and returns the rest of the input.
    pub const fn take_quoted_basic_or_rest(&mut self) -> Result<TextRange, TextParseError> {
        match self.take_quoted_basic() {
            Ok(Some(range)) => Ok(range),
            Ok(None) => Ok(self.take_rest()),
            Err(err) => Err(err),
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
