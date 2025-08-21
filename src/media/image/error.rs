// devela::media::image::error
//
//!
//

// use crate::Mismatch; use crate::IntErrorKind;
#[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
use crate::IoErrorKind;

#[doc = crate::TAG_RESULT!()]
/// An image-related result.
pub type ImageResult<T> = crate::Result<T, ImageError>;

#[doc = crate::TAG_ERROR_COMPOSITE!()]
/// An image-related error.
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
    // WAIT: [Derive Copy and Hash for IntErrorKind](https://github.com/rust-lang/rust/pull/131923)
    // InvalidParsedInteger(IntErrorKind), // Does not implement Copy
    InvalidParsedInteger,

    /// A `core::fmt::Error`.
    FmtError,

    /// An `I/O` error.
    #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
    #[cfg_attr(
        nightly_doc,
        doc(cfg(any(feature = "std", all(not(feature = "std"), feature = "io"))))
    )]
    IoError(IoErrorKind),
}

mod core_impls {
    #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
    use crate::IoError;
    use crate::{Display, FmtResult, Formatter, ImageError};
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
                #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
                E::IoError(e) => write!(f, "An I/O Error: {e:?}"),
            }
        }
    }

    // IMPROVE
    impl From<crate::ParseIntError> for ImageError {
        fn from(_: crate::ParseIntError) -> Self {
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
    #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
    impl From<IoError> for ImageError {
        fn from(e: IoError) -> Self {
            Self::IoError(e.kind())
        }
    }
}
