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
    /// Reached end of input unexpectedly.
    UnexpectedEof,
    /// Found a different byte than expected.
    UnexpectedByte {
        /// The byte expected.
        expected: u8,
        /// The byte found. `None` means EOF.
        found: Option<u8>,
    },
    /// Expected an ASCII digit.
    InvalidDigit,
    /// The destination buffer was too small.
    BufferTooSmall,
    /// The parsed numeric value overflowed its target type.
    Overflow,
    /// A quoted string reached EOF before its closing quote.
    UnterminatedQuoted,
    /// Found an unsupported escape sequence.
    InvalidEscape,
    /// Decoded bytes were not valid UTF-8.
    InvalidUtf8(InvalidUtf8),
}

_impl_init![ConstInitCore: Self::UnexpectedEof => TextParseErrorKind];
impl_trait![fmt::Display for TextParseErrorKind |self, f| {
    use TextParseErrorKind as K;
    match self {
        K::UnexpectedEof => f.write_str("unexpected EOF"),
        K::UnexpectedByte { expected, found }
            => write!(f, "unexpected byte: {found:?}, expected: {expected}"),
        K::InvalidDigit => f.write_str("invalid digit"),
        K::BufferTooSmall => f.write_str("buffer too small"),
        K::Overflow => f.write_str("overflow"),
        K::UnterminatedQuoted => f.write_str("unterminated quoted string"),
        K::InvalidEscape => f.write_str("invalid escape"),
        K::InvalidUtf8(err) => write!(f, "invalid UTF-8 ({err})"),
    }
}];

#[doc = crate::_tags!(text error)]
/// A text parsing failure with cursor location.
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

    /* numeric and buffer failures */

    /// Returns an invalid-digit error.
    pub const fn invalid_digit(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::InvalidDigit)
    }
    /// Returns an overflow error.
    pub const fn overflow(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::Overflow)
    }
    /// Returns a buffer-too-small error.
    pub const fn buffer_too_small(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::BufferTooSmall)
    }

    /* expectation failures */

    /// Returns an unexpected-EOF error.
    pub const fn unexpected_eof(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::UnexpectedEof)
    }
    /// Returns an unexpected-byte error.
    pub const fn unexpected_byte(at: TextCursor, expected: u8, found: Option<u8>) -> Self {
        Self::new(at, TextParseErrorKind::UnexpectedByte { expected, found })
    }

    /* quoted-string and decoding failures */

    /// Returns an unterminated-quoted-string error.
    pub const fn unterminated_quoted(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::UnterminatedQuoted)
    }
    /// Returns an invalid-escape error.
    pub const fn invalid_escape(at: TextCursor) -> Self {
        Self::new(at, TextParseErrorKind::InvalidEscape)
    }
    /// Returns an invalid-UTF-8 error.
    ///
    /// `at` marks the start of the decoded payload.
    /// `err.valid_up_to` is relative to that decoded payload.
    pub const fn invalid_utf8(at: TextCursor, err: InvalidUtf8) -> Self {
        Self::new(at, TextParseErrorKind::InvalidUtf8(err))
    }
}
