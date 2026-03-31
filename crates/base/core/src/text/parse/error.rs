// devela_base_core::text::parse::error
//
//! Defines [`TextParseError`], [`TextParseErrorKind`].
//

use crate::{_impl_init, InvalidUtf8, TextCursor, impl_trait};

#[doc = crate::_tags!(text error)]
/// The category of a text parsing failure.
#[doc = crate::_doc_location!("text/parse")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextParseErrorKind {
    /* expectation and boundary failures */
    /// Reached end of input unexpectedly.
    UnexpectedEof,
    /// Expected a specific byte but found a different byte, or EOF.
    UnexpectedByte {
        /// The byte that was expected.
        expected: u8,
        /// The byte that was found, or `None` if EOF was reached.
        found: Option<u8>,
    },
    /* decoding and lexical-value failures */
    /// Decoded bytes were not valid UTF-8.
    InvalidUtf8(InvalidUtf8),
    /// Expected an ASCII digit but found another byte.
    InvalidDigit,
    /// Found an unsupported or malformed escape sequence.
    InvalidEscape,

    /* capacity and numeric-conversion failures */
    /// The destination buffer could not hold the parsed output.
    BufferTooSmall,
    /// The parsed value overflowed the target representation.
    Overflow,

    /* quoted-text structural failures */
    /// A quoted segment reached EOF before its closing quote.
    UnterminatedQuote,
    /// Found unexpected trailing data after a closing quote.
    UnexpectedAfterQuote,
}

_impl_init![ConstInitCore: Self::UnexpectedEof => TextParseErrorKind];
impl_trait![fmt::Display for TextParseErrorKind |self, f| {
    use TextParseErrorKind as K;
    match self {
        K::UnexpectedEof => f.write_str("unexpected EOF"),
        K::UnexpectedByte { expected, found } => match found {
            Some(found) => write!(f, "unexpected byte: found {found:?}, expected {expected:?}"),
            None => write!(f, "unexpected EOF: expected byte {expected:?}"),
        },
        K::InvalidUtf8(err) => write!(f, "invalid UTF-8 ({err})"),
        K::InvalidDigit => f.write_str("invalid digit"),
        K::BufferTooSmall => f.write_str("buffer too small"),
        K::Overflow => f.write_str("overflow"),
        //
        K::UnterminatedQuote => f.write_str("unterminated quoted string"),
        K::UnexpectedAfterQuote => f.write_str("unexpected data after closing quote"),
        K::InvalidEscape => f.write_str("invalid escape"),
    }
}];

#[doc = crate::_tags!(text error)]
/// A text parsing failure paired with its cursor location.
#[doc = crate::_doc_location!("text/parse")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextParseError {
    /// Cursor at which the failure was detected.
    pub at: TextCursor,
    /// Failure category and payload.
    pub kind: TextParseErrorKind,
}

_impl_init![ConstInitCore:
    Self { at: TextCursor::INIT, kind: TextParseErrorKind::INIT } => TextParseError];
impl_trait![fmt::Display+Error for TextParseError |self, f|
    write!(f, "text parse error at: {:?}, {}", self.at.index, self.kind)
];

impl TextParseError {
    /// Creates a parse error from its cursor and kind.
    pub const fn new(at: TextCursor, kind: TextParseErrorKind) -> Self {
        Self { at, kind }
    }

    /* expectation and boundary failures */

    /// Returns an unexpected-EOF error.
    pub const fn unexpected_eof(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::UnexpectedEof)
    }
    /// Returns an unexpected-byte error.
    pub const fn unexpected_byte(at: TextCursor, expected: u8, found: Option<u8>) -> Self {
        Self::new(at, TextParseErrorKind::UnexpectedByte { expected, found })
    }

    /* decoding and lexical-value failures */

    /// Returns an invalid-UTF-8 error.
    ///
    /// `at` marks the start of the decoded payload.
    /// `err.valid_up_to` is relative to that decoded payload.
    pub const fn invalid_utf8(at: TextCursor, err: InvalidUtf8) -> Self {
        Self::new(at, TextParseErrorKind::InvalidUtf8(err))
    }
    /// Returns an invalid-digit error.
    pub const fn invalid_digit(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::InvalidDigit)
    }
    /// Returns an invalid-escape error.
    pub const fn invalid_escape(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::InvalidEscape)
    }

    /* capacity and numeric-conversion failures */

    /// Returns a buffer-too-small error.
    pub const fn buffer_too_small(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::BufferTooSmall)
    }
    /// Returns an overflow error.
    pub const fn overflow(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::Overflow)
    }

    /* quoted-text structural failures */

    /// Returns an unterminated-quote error.
    pub const fn unterminated_quote(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::UnterminatedQuote)
    }
    /// Returns an unexpected-after-quote error.
    pub const fn unexpected_after_quote(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::UnexpectedAfterQuote)
    }
}
