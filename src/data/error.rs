// devela::data::error
//
//!
//

/// A data-related result.
pub type DataResult<T> = core::result::Result<T, DataErrors>;

/// A data-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataErrors {
    /// The requested data-related functionality is not implemented.
    NotImplemented,

    /// The requested data-related functionality is not supported by this data type.
    NotSupported,

    /// Value above maximum representable.
    Overflow,

    /// Value below minimum representable.
    Underflow,

    /// The given `index` is out of bounds.
    OutOfBounds(Option<usize>),

    /// The given indices does not match the expected order.
    MismatchedIndices,

    /// The dimensions given did not match the elements provided
    DimensionMismatch,

    /// There are not enough elements for the operation.
    ///
    /// Optionally contains the minimum number of elements needed.
    NotEnoughElements(Option<usize>),

    /// There is not enough free space for the operation.
    ///
    /// Optionally contains the number of free spaces needed.
    NotEnoughSpace(Option<usize>),
}

#[allow(dead_code)]
impl DataErrors {
    pub(crate) const fn ni<T>() -> DataResult<T> {
        Err(DataErrors::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> DataResult<T> {
        Err(DataErrors::NotSupported)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// WAIT: error_in_core https://github.com/rust-lang/rust/issues/103765
impl std::error::Error for DataErrors {}

mod core_impls {
    use super::DataErrors as E;
    use core::fmt;

    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                E::NotImplemented => write!(f, "Not implemented."),
                E::NotSupported => write!(f, "Not supported."),
                E::Overflow => write!(f, "Value above maximum representable."),
                E::Underflow => write!(f, "Value aboce minimum representable."),
                E::MismatchedIndices => {
                    write!(f, "The given indices does not match the expected order.")
                }
                E::OutOfBounds(i) => {
                    if let Some(i) = i {
                        write!(f, "The given index {i} is out of bounds.")
                    } else {
                        write!(f, "The given index is out of bounds.")
                    }
                }
                E::DimensionMismatch => write!(f, "Dimension Mismatch."),
                E::NotEnoughElements(n) => {
                    if let Some(n) = n {
                        write!(f, "Not enough elements. Needs at least `{n}` elements.")
                    } else {
                        write!(f, "Not enough elements.")
                    }
                }
                E::NotEnoughSpace(n) => {
                    if let Some(n) = n {
                        write!(
                            f,
                            "Not enough space. Needs at least `{n}` free space for elements."
                        )
                    } else {
                        write!(f, "Not enough space.")
                    }
                }
            }
        }
    }
}
