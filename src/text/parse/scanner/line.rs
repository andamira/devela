// devela::text::parse::scanner::line

use crate::{TextRange, TextScanner};
use crate::{is, whilst};

/// EOL and line-oriented scanning.
impl<'a> TextScanner<'a> {
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
}
