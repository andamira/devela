// devela::text::parse::scanner::core

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{ConstInit, Slice, Str, is, unwrap, whilst};
use crate::{TextCursor, TextIndex, TextRange, TextScanner, TextUnit};

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
    // private cursor helpers
    pub(super) const fn _cursor_bump(&mut self, n: u32) {
        self.cursor = TextCursor::new_prim(self.cursor.index.0 + n);
    }
    pub(super) const fn _cursor_set(&mut self, pos: u32) {
        self.cursor = TextCursor::new_prim(pos);
    }

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
    /// Consumes and returns the remaining unread range.
    ///
    /// After this call, the scanner is at end of input.
    pub const fn take_rest(&mut self) -> TextRange {
        let start = self.mark();
        let _ = self.advance(self.remaining_len());
        self.range_from(start)
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
