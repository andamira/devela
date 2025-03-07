// devela::text::error
//
//!
//
// TOC
// - individual text-related error types:
//   - InvalidChar
//   - InvalidUtf8
// - full composite errors:
//   - TextResult
//   - TextError

use crate::{
    _core::str::Utf8Error, impl_error, Interval, Mismatch, MismatchedCapacity,
    DOC_MISMATCHED_CAPACITY,
};

/* individual errors */

impl_error! { individual: pub struct InvalidChar(char);
    DOC_INVALID_CHAR = "An invalid given character was found.",
    self+f => write!(f, "An invalid {:?} character was found.", self.0)
}

impl_error! { individual:
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

impl_error! { composite: fmt(f)
    #[doc = crate::TAG_TEXT!()]
    /// An error composite of [`InvalidChar`] + [`InvalidUtf8`] + [`MismatchedCapacity`].
    ///
    /// Used in methods of:
    /// [`StringNonul`][crate::StringNonul], and `StringU*`.
    pub enum InvalidText {
        DOC_INVALID_CHAR: Char(c|0: char) => InvalidChar(*c),
        DOC_INVALID_UTF8: Utf8 {
            #[doc = ""] valid_up_to: usize,
            #[doc = ""] error_len: Option<usize>
        } => InvalidUtf8 { valid_up_to: *valid_up_to, error_len: *error_len },
        DOC_MISMATCHED_CAPACITY:
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

#[cfg(all(feature = "error", text··))]
pub use full_composite::*;
#[cfg(all(feature = "error", text··))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(all(feature = "error", text··))))]
mod full_composite {
    use super::*;
    use crate::{ElementNotFound, MismatchedCapacity, DOC_ELEMENT_NOT_FOUND};

    #[doc = crate::TAG_RESULT!()]
    /// A text-related result.
    pub type TextResult<T> = crate::Result<T, TextError>;

    impl_error! { composite: fmt(f)
        /// A text-related composite error.
        #[non_exhaustive]
        pub enum TextError {
            DOC_ELEMENT_NOT_FOUND:
                ElementNotFound => ElementNotFound,

            DOC_INVALID_CHAR:
                InvalidChar(c|0: char) => InvalidChar(*c),

            DOC_INVALID_UTF8:
            InvalidUtf8 {
                /// The index in the given string up to which valid UTF-8 was verified.
                valid_up_to: usize,
                /// The length of the error in bytes, if known.
                error_len: Option<usize>
            }
            => InvalidUtf8 { valid_up_to: *valid_up_to, error_len: *error_len },

            DOC_MISMATCHED_CAPACITY:
                MismatchedCapacity(c|0: Mismatch<Interval<usize>, usize>) => MismatchedCapacity(*c),
        }
    }
    impl_error! { composite: from(f): InvalidText, for: TextError {
        Char(c) => InvalidChar(c),
        Utf8 { valid_up_to, error_len } => InvalidUtf8 { valid_up_to, error_len },
        Capacity(i) => MismatchedCapacity(i),
    }}
}
