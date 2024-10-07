// devela::rend::error
//
//!
//

// use crate::error::Mismatch;

/// A rend media result.
pub type RendResult<T> = core::result::Result<T, RendError>;

/// A rend media error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendError {
    /// Invalid image size, with an optional width and height.
    // InvalidImageSize(Mismatch<SizeUsize, SizeUsize>), // TODO
    InvalidImageSize(Option<(usize, usize)>), // TEMP

    /// Invalid pixel value.
    InvalidPixel, // IMPROVE add optional data

    /// A `core::fmt::Error`.
    CoreFmt, // MAYBE RETHINK
}

mod core_impls {
    use super::RendError as E;
    use core::fmt;

    impl crate::error::Error for E {}

    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                E::InvalidImageSize(o) => write!(f, "InvalidImageSize: {o:?}"),
                E::InvalidPixel => write!(f, "Invalid pixel."),
                E::CoreFmt => write!(f, "A core::fmt::Error."),
            }
        }
    }

    impl From<fmt::Error> for E {
        fn from(_: fmt::Error) -> Self {
            Self::CoreFmt
        }
    }
}
