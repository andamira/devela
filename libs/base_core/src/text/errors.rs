// devela_base_core::text::error
//
//! Text-related errors.
//
// TOC
// - individual text-related error types:
//   - InvalidChar
//   - InvalidUtf8
// - full composite errors:
//   - TextResult
//   - TextError

use crate::{DOC_MISMATCHED_CAPACITY, Interval, Mismatch, MismatchedCapacity, define_error};
use ::core::str::Utf8Error; // replaced with InvalidUtf8

/* individual errors */

define_error! { individual: pub struct InvalidChar(char);
    DOC_INVALID_CHAR = "An invalid given character was found.",
    self+f => write!(f, "An invalid {:?} character was found.", self.0)
}

define_error! { individual:
    pub struct InvalidUtf8 {
        /// The index in the given string up to which valid UTF-8 was verified.
        pub valid_up_to: usize,
        /// The length of the error in bytes, if known.
        pub error_len: Option<usize>
    }
    DOC_INVALID_UTF8 = "Invalid Utf-8 found while interpreting a byte sequence.\n\n
This is basically a replication of `core::str::`[`Utf8Error`]`.",
    self+f => if let Some(len) = self.error_len {
        write!(f, "Invalid UTF-8, valid up to: {}, len: {len}", self.valid_up_to)
    } else { write!(f, "Invalid UTF-8, valid up to: {}", self.valid_up_to) }
}
#[rustfmt::skip]
impl From<Utf8Error> for InvalidUtf8 { fn from(f: Utf8Error) -> Self { Self::from_utf8_error(f) } }
impl InvalidUtf8 {
    /// Converts `core::str`[`Utf8Error`] to [`InvalidUtf8`] in compile-time.
    #[must_use]
    pub const fn from_utf8_error(from: Utf8Error) -> InvalidUtf8 {
        let (valid_up_to, error_len) = (from.valid_up_to(), from.error_len());
        InvalidUtf8 { valid_up_to, error_len }
    }
}

/* composite errors */

define_error! { composite: fmt(f)
    #[doc = crate::_TAG_TEXT!()]
    /// An error composite of [`InvalidChar`] + [`InvalidUtf8`] + [`MismatchedCapacity`].
    ///
    /// Used in methods of:
    /// [`StringNonul`][crate::StringNonul], and `StringU*`.
    pub enum InvalidText {
        DOC_INVALID_CHAR: +const
            Char(c|0: char) => InvalidChar(*c),
        DOC_INVALID_UTF8: +const
            Utf8 {
                #[doc = ""] valid_up_to: usize,
                #[doc = ""] error_len: Option<usize>
            } => InvalidUtf8 { valid_up_to: *valid_up_to, error_len: *error_len },
        DOC_MISMATCHED_CAPACITY: +const
            Capacity(c|0: Mismatch<Interval<usize>, usize>) => MismatchedCapacity(*c),
    }
}
#[rustfmt::skip]
impl From<Utf8Error> for InvalidText { fn from(f: Utf8Error) -> Self { Self::from_utf8_error(f) } }
impl InvalidText {
    /// Converts `core::str`[`Utf8Error`] to [`InvalidText::Utf8`] in compile-time.
    #[must_use]
    pub const fn from_utf8_error(from: Utf8Error) -> InvalidText {
        let (valid_up_to, error_len) = (from.valid_up_to(), from.error_len());
        InvalidText::Utf8 { valid_up_to, error_len }
    }
}

// TODO:
pub use full_composite::*;
mod full_composite {
    use super::*;
    use crate::{DOC_ELEMENT_NOT_FOUND, ElementNotFound, MismatchedCapacity};

    #[doc = crate::_TAG_RESULT!()]
    /// A text-related result.
    pub type TextResult<T> = crate::Result<T, TextError>;

    define_error! { composite: fmt(f)
        /// A text-related composite error.
        #[non_exhaustive]
        pub enum TextError {
            DOC_ELEMENT_NOT_FOUND: +const
                ElementNotFound => ElementNotFound,

            DOC_INVALID_CHAR: +const
                InvalidChar(c|0: char) => InvalidChar(*c),

            DOC_INVALID_UTF8: +const
                InvalidUtf8 {
                    /// The index in the given string up to which valid UTF-8 was verified.
                    valid_up_to: usize,
                    /// The length of the error in bytes, if known.
                    error_len: Option<usize>
                }
                => InvalidUtf8 { valid_up_to: *valid_up_to, error_len: *error_len },

            DOC_MISMATCHED_CAPACITY: +const
                MismatchedCapacity(c|0: Mismatch<Interval<usize>, usize>) => MismatchedCapacity(*c),
        }
    }
    define_error! { composite: from(f): InvalidText, for: TextError {
        Char(c) => InvalidChar(c),
        Utf8 { valid_up_to, error_len } => InvalidUtf8 { valid_up_to, error_len },
        Capacity(i) => MismatchedCapacity(i),
    }}
}
