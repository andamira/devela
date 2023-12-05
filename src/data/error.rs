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
    IndexOutOfBounds(Option<usize>),

    /// The given indices does not match the expected order.
    MismatchedIndices,

    /// The dimensions given did not match the elements provided
    DimensionMismatch,
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
                E::IndexOutOfBounds(i) => {
                    if let Some(i) = i {
                        write!(f, "The given index {i} is out of bounds.")
                    } else {
                        write!(f, "The given index is out of bounds.")
                    }
                }
                E::DimensionMismatch => write!(f, "Dimension Mismatch."),
            }
        }
    }
}
