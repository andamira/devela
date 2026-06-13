// devela/src/lang/prog/script/shell/word/error.rs
//
//! Defines [`ShellWordError`].
//

#[doc = crate::_tags!(lang error_composite)]
/// An error while parsing or quoting shell words.
#[doc = crate::_doc_meta!{location("lang/prog/script/shell")}]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShellWordError {
    /// The command line ended before a program word was found.
    EmptyCommand,

    /// The input ended after an unescaped backslash.
    TrailingEscape,

    /// The input ended inside a single-quoted word.
    UnterminatedSingleQuote,

    /// The input ended inside a double-quoted word.
    UnterminatedDoubleQuote,

    /// The output buffer was too small.
    OutputTooSmall {
        /// Required output length.
        needed: usize,
    },

    /// The word contains a nul byte.
    Nul,

    /// The word contains an interactive-unsafe control byte.
    Control {
        /// The rejected byte.
        byte: u8,
    },

    /// The decoded word is not valid UTF-8.
    InvalidUtf8,
}
