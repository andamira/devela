// devela::rend::image::error
//
//!
//

// use crate::Mismatch; use crate::IntErrorKind;
use crate::IoErrorKind;

/// An image rendering result.
pub type ImageResult<T> = crate::Result<T, ImageError>;

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
    use crate::{Display, FmtResult, Formatter, ImageError, IoError};
    use core::fmt;

    impl crate::Error for ImageError {}

    impl Display for ImageError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            use ImageError as E;
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

    // IMPROVE
    impl From<core::num::ParseIntError> for ImageError {
        fn from(_: core::num::ParseIntError) -> Self {
            // Self::InvalidParsedInteger(e.kind().clone())
            Self::InvalidParsedInteger
        }
    }
    // IMPROVE
    impl From<fmt::Error> for ImageError {
        fn from(_: fmt::Error) -> Self {
            Self::FmtError
        }
    }
    impl From<IoError> for ImageError {
        fn from(e: IoError) -> Self {
            Self::IoError(e.kind())
        }
    }
}
