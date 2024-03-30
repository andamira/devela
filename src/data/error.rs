// devela::data::error
//
//!
//

use crate::code::Mismatch;

/// A data-related result.
pub type DataResult<T> = core::result::Result<T, DataError>;

/// A data-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataError {
    /// The requested data-related functionality is not implemented.
    NotImplemented,

    /// The requested data-related functionality is not supported by this data type.
    NotSupported,

    /// Value above maximum representable.
    Overflow,

    /// The given `index`, `length` or `capacity` is out of bounds.
    ///
    /// Optionally contains given magnitude.
    OutOfBounds(Option<usize>),

    /// The given axis has 0 length, which is not allowed.
    ///
    /// Optionally contains the given axis number.
    InvalidAxisLength(Option<usize>),

    /// The given indices does not match the expected order.
    MismatchedIndices,

    /// The dimensions given did not match the elements provided
    MismatchedDimensions(Mismatch<usize, usize>),

    /// The given length or capacity did not match the required constraints.
    MismatchedLength(Mismatch<usize, usize>),

    /// There are not enough elements for the operation.
    ///
    /// Optionally contains the minimum number of elements needed.
    NotEnoughElements(Option<usize>),

    /// There is not enough free space for the operation.
    ///
    /// Optionally contains the number of free spaces needed.
    NotEnoughSpace(Option<usize>),

    /// The operation could only add a subset of the elements.
    ///
    /// Optionally contains the number of elements added.
    PartiallyAdded(Option<usize>),

    /// The key already exists.
    KeyAlreadyExists,

    /// The node is empty.
    EmptyNode,
}

#[allow(dead_code)]
impl DataError {
    pub(crate) const fn ni<T>() -> DataResult<T> {
        Err(DataError::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> DataResult<T> {
        Err(DataError::NotSupported)
    }
}

mod core_impls {
    use super::DataError as E;
    use core::fmt;

    impl crate::code::Error for E {}

    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                E::NotImplemented => write!(f, "Not implemented."),
                E::NotSupported => write!(f, "Not supported."),
                E::Overflow => write!(f, "Value above maximum representable."),
                E::InvalidAxisLength(n) => match n {
                    Some(n) => write!(f, "Axis number {n} has 0 length, which is not allowed."),
                    None => write!(f, "One ore more axis have 0 length, which is not allowed."),
                },
                E::MismatchedIndices => {
                    write!(f, "The given indices does not match the expected order.")
                }
                E::OutOfBounds(i) => match i {
                    Some(i) => write!(f, "The given index {i} is out of bounds."),
                    None => write!(f, "The given index is out of bounds."),
                },
                E::MismatchedDimensions(m) => {
                    write!(f, "Mismatched dimensions: {m:?}.")
                }
                E::MismatchedLength(m) => {
                    write!(f, "Mismatched length or capacity: {m:?}.")
                }
                E::NotEnoughElements(n) => match n {
                    Some(n) => write!(f, "Not enough elements. Needs at least `{n}` elements."),
                    None => write!(f, "Not enough elements."),
                },
                E::NotEnoughSpace(n) => match n {
                    Some(n) => write!(
                        f,
                        "Not enough space. Needs at least `{n}` free space for elements."
                    ),
                    None => write!(f, "Not enough space."),
                },
                E::PartiallyAdded(n) => match n {
                    Some(n) => write!(f, "Only `{n}` elements could be added."),
                    None => write!(f, "Only a subset of elements could be added."),
                },
                E::KeyAlreadyExists => write!(f, "The key already exists."),
                E::EmptyNode => write!(f, "The node is empty."),
            }
        }
    }
}
