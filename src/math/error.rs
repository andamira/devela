// devela::math::error
//
//!
//

/// A mathematical result.
pub type MathResult<T> = core::result::Result<T, MathError>;

/// A numeric error.
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
    Unspecified,

    /// An invalid value for the given type.
    Invalid,

    ///
    Overflow,

    ///
    Underflow,
}

impl MathError {
    pub(crate) fn notimpl<T>() -> MathResult<T> {
        Err(MathError::NotImplemented)
    }
}
