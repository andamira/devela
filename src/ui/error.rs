// devela::ui::error
//
//!
//

#[cfg(feature = "sys")]
use crate::sys::IoError;

/// A user-interface result.
pub type UiResult<T> = core::result::Result<T, UiError>;

/// A user-interface error.
#[non_exhaustive]
#[derive(Debug)]
pub enum UiError {
    /// The requested numerical functionality is not implemented.
    ///
    /// This is the default implementation of every numeric trait method.
    NotImplemented,

    /// The requested numerical functionality is not supported by this number type.
    NotSupported,

    /// An io error.
    #[cfg(feature = "sys")]
    Io(IoError),
}

#[allow(dead_code)]
impl UiError {
    pub(crate) const fn ni<T>() -> UiResult<T> {
        Err(UiError::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> UiResult<T> {
        Err(UiError::NotSupported)
    }
}

impl crate::code::Error for UiError {}

mod core_impls {
    use super::*;
    use core::fmt;

    impl fmt::Display for UiError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                UiError::NotImplemented => write!(f, "Not implemented."),
                UiError::NotSupported => write!(f, "Not supported."),
                #[cfg(feature = "sys")]
                UiError::Io(e) => fmt::Debug::fmt(e, f),
            }
        }
    }

    #[cfg(feature = "sys")]
    impl From<IoError> for UiError {
        fn from(err: IoError) -> Self {
            UiError::Io(err)
        }
    }
}
