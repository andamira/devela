// devela::text::error
//
//!
//

/// A text-related result.
pub type TextResult<T> = core::result::Result<T, TextError>;

/// A text-related result.
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
                E::OutOfBounds(i) => match i {
                    Some(i) => write!(f, "The given index or capacity {i} is out of bounds."),
                    None => write!(f, "The given index or capacity is out of bounds."),
                },
            }
        }
    }
}
