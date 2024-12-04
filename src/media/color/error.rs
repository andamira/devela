// devela::media::color::error
//
//!
//

/// A color-related result.
pub type ColorResult<T> = crate::Result<T, ColorError>;

/// A color-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)] // Hash
pub enum ColorError {
    /// The requested chromatic functionality is not implemented.
    ///
    /// This is the default implementation of every `Color` method.
    NotImplemented,

    /// The requested chromatic functionality is not supported by this color type.
    NotSupported,
}

#[allow(dead_code)]
impl ColorError {
    pub(crate) const fn ni<T>() -> ColorResult<T> {
        Err(ColorError::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> ColorResult<T> {
        Err(ColorError::NotSupported)
    }
}

mod core_impls {
    use crate::{ColorError, Display, FmtResult, Formatter};

    impl crate::Error for ColorError {}
    impl crate::ExtError for ColorError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for ColorError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            use ColorError as E;
            match self {
                E::NotImplemented => write!(f, "Not implemented."),
                E::NotSupported => write!(f, "Not supported."),
            }
        }
    }
}
