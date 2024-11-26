// devela::rend::color::error
//
//!
//
// MAYBE: make it generic?

/// A chromatic result.
pub type ColorResult<T> = core::result::Result<T, ColorError>;

/// A chromatic error.
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

impl crate::error::Error for ColorError {}

mod core_impls {
    use super::ColorError;
    use core::fmt;

    impl fmt::Display for ColorError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use ColorError as E;
            match self {
                E::NotImplemented => write!(f, "Not implemented."),
                E::NotSupported => write!(f, "Not supported."),
            }
        }
    }
}
