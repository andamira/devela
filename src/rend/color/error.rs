// devela::rend::color::error
//
//!
//

/// A chromatic result.
pub type ColorResult<T> = core::result::Result<T, ColorErrors>;

/// A chromatic error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorErrors {
    /// The requested chromatic functionality is not implemented.
    ///
    /// This is the default implementation of every `Color` method.
    NotImplemented,

    /// The requested chromatic functionality is not supported by this color type.
    NotSupported,
}

#[allow(dead_code)]
impl ColorErrors {
    pub(crate) const fn ni<T>() -> ColorResult<T> {
        Err(ColorErrors::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> ColorResult<T> {
        Err(ColorErrors::NotSupported)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
// WAIT: [error_in_core](https://github.com/rust-lang/rust/issues/103765)
impl std::error::Error for ColorErrors {}

mod core_impls {
    use super::ColorErrors;
    use core::fmt;

    impl fmt::Display for ColorErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ColorErrors::NotImplemented => write!(f, "Not implemented."),
                ColorErrors::NotSupported => write!(f, "Not supported."),
            }
        }
    }
}
