// devela::num::error
//
//!
//

/// A numerical result.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
pub type NumResult<T> = core::result::Result<T, NumErrors>;

/// A numerical error.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumErrors {
    /// The requested numerical functionality is not implemented.
    ///
    /// This is the default implementation of every `Num` method.
    NotImplemented,

    /// The requested numerical functionality is not supported by this number type.
    NotSupported,

    /// Unspecified error.
    ///
    /// When no clearer error can be given.
    // RETHINK
    Unspecified,

    /// An invalid value was received for the given type or operation.
    Invalid,

    /// The provided values are not compatible in size.
    MismatchedSizes,

    /// A non-negative value is required.
    NonNegativeRequired,

    /// A positive value is required.
    PositiveRequired,

    /// An arithmetic overflow error.
    Overflow,
}

#[allow(dead_code)]
impl NumErrors {
    pub(crate) const fn ni<T>() -> NumResult<T> {
        Err(NumErrors::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> NumResult<T> {
        Err(NumErrors::NotSupported)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// WAIT: error_in_core https://github.com/rust-lang/rust/issues/103765
impl std::error::Error for NumErrors {}

mod core_impls {
    use super::NumErrors;
    use core::fmt;

    impl fmt::Display for NumErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                NumErrors::NotImplemented => write!(f, "Not implemented."),
                NumErrors::NotSupported => write!(f, "Not supported."),
                NumErrors::Unspecified => write!(f, "Unspecified."),
                NumErrors::Invalid => write!(f, "Invalid value."),
                NumErrors::MismatchedSizes => {
                    write!(f, "The provided values are not compatible in size.")
                }
                NumErrors::NonNegativeRequired => write!(f, "A non-negative value is required."),
                NumErrors::PositiveRequired => write!(f, "A positive value is required.."),
                NumErrors::Overflow => write!(f, "Overflow."),
            }
        }
    }
}
