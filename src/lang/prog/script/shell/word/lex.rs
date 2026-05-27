// devela::lang::prog::script::shell::lex
//
//! Defines [`ShellLex`].
//

use crate::{ShellWordError, TextScanner, is, unwrap};

#[doc = crate::_tags!(lang parser)]
/// Splits shell-like input into command words.
#[doc = crate::_doc_meta!{location("lang/prog/script/shell")}]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShellLex<'a> {
    scanner: TextScanner<'a>,
    line_no: usize,
}
impl<'a> ShellLex<'a> {
    /// Creates a shell word lexer over bytes.
    #[must_use]
    pub const fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            scanner: TextScanner::from_bytes(bytes),
            line_no: 1,
        }
    }
    /// Creates a shell word lexer over UTF-8 text.
    #[must_use]
    pub const fn new(s: &'a str) -> Self {
        Self::from_bytes(s.as_bytes())
    }
    /// Returns the current one-based line number.
    #[must_use]
    pub const fn line_no(&self) -> usize {
        self.line_no
    }
}
impl<'a> ShellLex<'a> {
    /// Decodes the next shell word into `out`.
    ///
    /// Returns `Ok(None)` at end of input.
    ///
    /// # Errors
    /// Returns an error for unterminated quotes, trailing escapes,
    /// or insufficient output space.
    pub const fn next_word_to(&mut self, out: &mut [u8]) -> Result<Option<usize>, ShellWordError> {
        self.skip_gap();
        is! { self.scanner.peek_byte().is_none(), return Ok(None) }
        let mut len = 0;
        loop {
            let Some(byte) = self.scanner.peek_byte() else {
                return Ok(Some(len));
            };
            match byte {
                b' ' | b'\t' | b'\n' => return Ok(Some(len)),
                b'#' if len == 0 => {
                    self.skip_comment();
                    self.skip_gap();
                    is! { self.scanner.peek_byte().is_none(), return Ok(None) }
                }
                b'\'' => {
                    self.scanner.skip_byte();
                    len = unwrap![ok? self.take_single_to(out, len)];
                }
                b'"' => {
                    self.scanner.skip_byte();
                    len = unwrap![ok? self.take_double_to(out, len)];
                }
                b'\\' => {
                    self.scanner.skip_byte();
                    let Some(next) = self.scanner.next_byte() else {
                        return Err(ShellWordError::TrailingEscape);
                    };
                    self.bump_line(next);
                    is! { next != b'\n', len = unwrap![ok? Self::push_byte(out, len, next)] }
                }
                _ => {
                    self.scanner.skip_byte();
                    self.bump_line(byte);
                    len = unwrap![ok? Self::push_byte(out, len, byte)];
                }
            }
        }
    }
}
impl<'a> ShellLex<'a> {
    const fn skip_gap(&mut self) {
        loop {
            match self.scanner.peek_byte() {
                Some(b' ' | b'\t') => {
                    self.scanner.skip_byte();
                }
                Some(b'\n') => {
                    self.scanner.skip_byte();
                    self.line_no += 1;
                }
                _ => break,
            }
        }
    }
    const fn skip_comment(&mut self) {
        loop {
            match self.scanner.next_byte() {
                Some(b'\n') => {
                    self.line_no += 1;
                    break;
                }
                Some(_) => {}
                None => break,
            }
        }
    }
    const fn bump_line(&mut self, byte: u8) {
        is! { byte == b'\n', self.line_no += 1; }
    }
    const fn take_single_to(
        &mut self,
        out: &mut [u8],
        mut len: usize,
    ) -> Result<usize, ShellWordError> {
        loop {
            let Some(byte) = self.scanner.next_byte() else {
                return Err(ShellWordError::UnterminatedSingleQuote);
            };
            if byte == b'\'' {
                return Ok(len);
            } else {
                self.bump_line(byte);
                len = unwrap![ok? Self::push_byte(out, len, byte)];
            }
        }
    }
    const fn take_double_to(
        &mut self,
        out: &mut [u8],
        mut len: usize,
    ) -> Result<usize, ShellWordError> {
        loop {
            let Some(byte) = self.scanner.next_byte() else {
                return Err(ShellWordError::UnterminatedDoubleQuote);
            };
            match byte {
                b'"' => return Ok(len),
                b'\\' => {
                    let Some(next) = self.scanner.next_byte() else {
                        return Err(ShellWordError::UnterminatedDoubleQuote);
                    };
                    match next {
                        b'$' | b'`' | b'"' | b'\\' => {
                            len = unwrap![ok? Self::push_byte(out, len, next)];
                        }
                        b'\n' => {
                            self.line_no += 1;
                        }
                        _ => {
                            len = unwrap![ok? Self::push_byte(out, len, b'\\')];
                            len = unwrap![ok? Self::push_byte(out, len, next)];
                            self.bump_line(next);
                        }
                    }
                }
                _ => {
                    self.bump_line(byte);
                    len = unwrap![ok? Self::push_byte(out, len, byte)];
                }
            }
        }
    }
    const fn push_byte(out: &mut [u8], len: usize, byte: u8) -> Result<usize, ShellWordError> {
        if len == out.len() {
            Err(ShellWordError::OutputTooSmall { needed: len + 1 })
        } else {
            out[len] = byte;
            Ok(len + 1)
        }
    }
}
