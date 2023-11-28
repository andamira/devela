// devela::math::error
//
//!
//

/// A mathematical result.
pub type MathResult<T> = core::result::Result<T, MathError>;

/// A mathematical error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathError {
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

    /// An invalid value for the given type.
    Invalid,

    ///
    Overflow,

    ///
    // RETHINK
    Underflow,
}

impl MathError {
    pub(crate) fn notimpl<T>() -> MathResult<T> {
        Err(MathError::NotImplemented)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// WAIT: error_in_core https://github.com/rust-lang/rust/issues/103765
impl std::error::Error for MathError {}

mod core_impls {
    use super::MathError;
    use core::fmt;

    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MathError::NotImplemented => write!(f, "Not implemented."),
                MathError::NotSupported => write!(f, "Not supported."),
                MathError::Unspecified => write!(f, "Unspecified."),
                MathError::Invalid => write!(f, "Invalid."),
                MathError::Overflow => write!(f, "Overflow."),
                MathError::Underflow => write!(f, "Underflow"),
            }
        }
    }
}
