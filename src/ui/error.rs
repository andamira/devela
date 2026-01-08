// devela::ui::error
//
//!
//

// NOTE: IoError doesn't implement Clone, PartialEq, Hash, etc.
#[cfg(feature = "sys")]
use crate::IoError;

#[doc = crate::_tags!(ui result)]
/// A user-interface result.
#[doc = crate::_doc_location!("ui")]
pub type UiResult<T> = core::result::Result<T, UiError>;

#[doc = crate::_tags!(ui error_composite)]
/// A user-interface error.
#[doc = crate::_doc_location!("ui")]
#[non_exhaustive]
#[derive(Debug)]
pub enum UiError {
    /// The requested numerical functionality is not implemented.
    ///
    /// This is the default implementation of every numeric trait method.
    NotImplemented,

    /// The requested functionality is not supported by this number type.
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

impl crate::Error for UiError {}

mod core_impls {
    use super::*;
    use crate::impl_trait;

    impl_trait! { fmt::Display for UiError |self, f| {
        use UiError as E;
        match self {
            E::NotImplemented => write!(f, "Not implemented."),
            E::NotSupported => write!(f, "Not supported."),
            #[cfg(feature = "sys")]
            E::Io(e) => write!(f, "{e:?}"),
        }
    }}

    #[cfg(feature = "sys")]
    impl From<IoError> for UiError {
        fn from(err: IoError) -> Self {
            UiError::Io(err)
        }
    }
}
