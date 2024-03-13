// devela::text::error
//
//!
//

use core::str::Utf8Error;

/// A text-related result.
pub type TextResult<T> = core::result::Result<T, TextError>;

/// A text-related error.
#[non_exhaustive]
#[derive(Debug)]
pub enum TextError {
    /// Not enough capacity for the attempted operation.
    ///
    /// Returns the needed capacity.
    NotEnoughCapacity(usize),

    /// Not enough elements for the attempted operation.
    ///
    /// Returns the needed number of elements.
    NotEnoughElements(usize),

    /// The error type returned when a conversion to a unicode scalar fails.
    CharConversion,

    /// An invalid NUL character was found.
    InvalidNul,

    /// An error which can occur when attempting to interpret
    /// a sequence of [`u8`] as a string.
    // Consider:
    // - https://doc.rust-lang.org/stable/std/str/struct.Utf8Error.html
    // - https://doc.rust-lang.org/std/string/struct.FromUtf8Error.html
    InvalidUtf8(Option<Utf8Error>),

    /// The given `index`, `length` or `capacity` is out of bounds.
    ///
    /// Optionally contains given magnitude.
    OutOfBounds(Option<usize>),
}

mod core_impls {
    use super::TextError as E;
    use core::fmt;

    impl crate::result::Error for E {}

    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                E::NotEnoughCapacity(c) => write!(f, "Not enough capacity. Needed: {c}"),
                E::NotEnoughElements(e) => write!(f, "Not enough elements. Needed: {e}"),
                E::CharConversion => write!(f, "Unicode code point out of range."),
                E::InvalidUtf8(err) => match err {
                    None => write!(f, "Invalid UTF-8."),
                    Some(err) => match err.error_len() {
                        None => write!(f, "Invalid UTF-8, valid up to: {}", err.valid_up_to()),
                        Some(len) => write!(
                            f,
                            "Invalid UTF-8, valid up to: {}, len: {len}",
                            err.valid_up_to()
                        ),
                    },
                },
                E::InvalidNul => write!(f, "An invalid NUL character was found."),
                E::OutOfBounds(i) => match i {
                    None => write!(f, "The given index or capacity is out of bounds."),
                    Some(i) => write!(f, "The given index or capacity {i} is out of bounds."),
                },
            }
        }
    }
}
