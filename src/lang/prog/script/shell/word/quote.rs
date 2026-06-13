// devela/src/lang/prog/script/shell/word/quote.rs
//
//! Shell word parsing and quoting.
//

use crate::{ShellWordError, is, whilst, write_at};

#[doc = crate::_tags!(lang text)]
/// Quotes shell words for command strings.
#[doc = crate::_doc_meta!{location("lang/prog/script/shell")}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShellQuote {
    reject_control: bool,
}
impl ShellQuote {
    /// Creates a shell word quoter.
    pub const fn new() -> Self {
        Self { reject_control: false }
    }
    /// Rejects interactive-unsafe control bytes.
    pub const fn reject_control(mut self, reject: bool) -> Self {
        self.reject_control = reject;
        self
    }

    /// Returns the number of bytes required to quote `word`.
    ///
    /// # Errors
    /// Returns [`ShellWordError::Nul`] if `word` contains a nul byte.
    /// Returns [`ShellWordError::Control`] if control bytes are rejected.
    pub const fn quoted_len(self, word: &[u8]) -> Result<usize, ShellWordError> {
        is! { word.is_empty(), return Ok(2) } // ''
        let mut needs_quote = false;
        let mut quoted_len = 2; // opening + closing single quote
        whilst! { i in 0..word.len(); {
            let byte = word[i];
            is! { byte == b'\0', return Err(ShellWordError::Nul) }
            if self.reject_control && is_control_byte(byte) {
                return Err(ShellWordError::Control { byte });
            }
            is! { !is_unquoted_shell_byte(byte), needs_quote = true; }
            quoted_len += is![byte == b'\'', 4,  1];
        }}
        is! { needs_quote, Ok(quoted_len), Ok(word.len()) }
    }

    /// Writes `word` as one shell word into `out`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns [`ShellWordError::OutputTooSmall`] if `out` is too small.
    /// Also returns the validation errors from [`quoted_len`][Self::quoted_len].
    pub const fn quote_to(self, word: &[u8], out: &mut [u8]) -> Result<usize, ShellWordError> {
        let needed = match self.quoted_len(word) {
            Ok(needed) => needed,
            Err(err) => return Err(err),
        };
        is! { out.len() < needed, return Err(ShellWordError::OutputTooSmall { needed }) }
        is! { word.is_empty(), return Ok(write_at![out, 0, b'\'', b'\'']) }
        // Fast path: no quoting needed.
        if needed == word.len() {
            whilst! { i in 0..word.len(); { out[i] = word[i]; }}
            return Ok(needed);
        }
        // Single-quote the whole word, replacing ' with '\''.
        let mut pos = 0;
        write_at![out, +=pos, b'\''];
        whilst! { i in 0..word.len(); {
            let byte = word[i];
            if byte == b'\'' {
                write_at![out, +=pos, b'\'', b'\\', b'\'', b'\''];
            } else {
                write_at![out, +=pos, byte];
            }
        }}
        out[pos] = b'\'';
        Ok(needed)
    }
}
const fn is_control_byte(byte: u8) -> bool {
    byte <= 0x1F || byte == 0x7F
}
const fn is_unquoted_shell_byte(byte: u8) -> bool {
    matches!(byte, b'+' | b'-' | b'.' | b'/' | b':' | b'@' | b']' | b'_'
        | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z')
}
