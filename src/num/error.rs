// devela::num::error
//
//!
//

/// A numeric result.
pub type NumResult<T> = core::result::Result<T, NumError>;

/// A numeric error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumError {
    /// The requested functionality is not implemented.
    ///
    /// This is the default result of every auto-implemented `Num` method.
    NotImplemented,

    /// The requested functionality is not supported by this number type.
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

impl NumError {
    pub(crate) fn notimpl<T>() -> NumResult<T> {
        Err(NumError::NotImplemented)
    }
}
