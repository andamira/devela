// devela::math::error
//
//!
//

/// A mathematical result.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "math")))]
pub type MathResult<T> = core::result::Result<T, MathErrors>;

/// A mathematical error.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "math")))]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathErrors {
    /// The requested mathematical functionality is not implemented.
    ///
    /// This is the default implementation of every `Num` method.
    NotImplemented,

    /// The requested mathematical functionality is not supported by this number type.
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

    ///
    Overflow,

    ///
    // RETHINK
    Underflow,
}

#[allow(dead_code)]
impl MathErrors {
    pub(crate) const fn ni<T>() -> MathResult<T> {
        Err(MathErrors::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> MathResult<T> {
        Err(MathErrors::NotSupported)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// WAIT: error_in_core https://github.com/rust-lang/rust/issues/103765
impl std::error::Error for MathErrors {}

mod core_impls {
    use super::MathErrors;
    use core::fmt;

    impl fmt::Display for MathErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MathErrors::NotImplemented => write!(f, "Not implemented."),
                MathErrors::NotSupported => write!(f, "Not supported."),
                MathErrors::Unspecified => write!(f, "Unspecified."),
                MathErrors::Invalid => write!(f, "Invalid value."),
                MathErrors::MismatchedSizes => {
                    write!(f, "The provided values are not compatible in size.")
                }
                MathErrors::NonNegativeRequired => write!(f, "A non-negative value is required."),
                MathErrors::PositiveRequired => write!(f, "A positive value is required.."),
                MathErrors::Overflow => write!(f, "Overflow."),
                MathErrors::Underflow => write!(f, "Underflow"),
            }
        }
    }
}
