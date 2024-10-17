// devela::rend::error
//
//!
//

use crate::rend::image::ImageError;

/// A rend media result.
pub type RendResult<T> = core::result::Result<T, RendError>;

/// A rend media error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendError {
    ImageError(ImageError)
}

mod core_impls {
    use super::{RendError, ImageError};
    use core::fmt;

    impl crate::error::Error for RendError {}

    impl fmt::Display for RendError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RendError::ImageError(e) => write!(f, "{e:?}"),
            }
        }
    }

    impl From<ImageError> for RendError {
        fn from(e: ImageError) -> Self {
            Self::ImageError(e)
        }
    }
}
