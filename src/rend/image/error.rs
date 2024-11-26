// devela::rend::image::error
//
//!
//

// use crate::Mismatch; use crate::IntErrorKind;
use crate::IoErrorKind;

/// An image rendering result.
pub type ImageResult<T> = core::result::Result<T, ImageError>;

/// An image rendering error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageError {
    /// Invalid image size, with an optional width and height.
    // InvalidImageSize(Mismatch<SizeUsize, SizeUsize>), // TODO
    InvalidImageSize(Option<(usize, usize)>), // TEMP

    /// Invalid pixel value.
    InvalidPixel, // IMPROVE add optional data

    /// Invalid magic number
    InvalidMagicNumber,

    /* from std */
    ///
    // InvalidParsedInteger(IntErrorKind), // Does not implement Copy
    InvalidParsedInteger,

    /// A `core::fmt::Error`.
    FmtError,

    /// An `I/O` error.
    IoError(IoErrorKind),
}

mod core_impls {
    use super::ImageError as E;
    use crate::error::IoError;
    use core::fmt;

    impl crate::error::Error for E {}

    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                E::InvalidImageSize(o) => write!(f, "InvalidImageSize: {o:?}"),
                E::InvalidMagicNumber => write!(f, "Invalid magic number."),
                E::InvalidPixel => write!(f, "Invalid pixel."),
                //
                // E::InvalidParsedInteger(k) => write!(f, "Invalid parsed integer: {k:?}."),
                E::InvalidParsedInteger => write!(f, "Invalid parsed integer."),
                E::FmtError => write!(f, "A core::fmt::Error."),
                E::IoError(e) => write!(f, "An I/O Error: {e:?}"),
            }
        }
    }

    impl From<core::num::ParseIntError> for E {
        fn from(_: core::num::ParseIntError) -> Self {
            // Self::InvalidParsedInteger(e.kind().clone())
            Self::InvalidParsedInteger
        }
    }
    impl From<fmt::Error> for E {
        fn from(_: fmt::Error) -> Self {
            Self::FmtError
        }
    }
    impl From<IoError> for E {
        fn from(e: IoError) -> Self {
            Self::IoError(e.kind())
        }
    }
}
