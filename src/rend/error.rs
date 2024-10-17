// devela::rend::error
//
//!
//

use crate::rend::{AudioError, ColorError, DrawError, FontError, ImageError, LayoutError};

/// A rend media result.
pub type RendResult<T> = core::result::Result<T, RendError>;

/// A rend media error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendError {
    ///
    AudioError(AudioError),
    ///
    DrawError(DrawError),
    ///
    ColorError(ColorError),
    ///
    FontError(FontError),
    ///
    ImageError(ImageError),
    ///
    LayoutError(LayoutError),
}

mod core_impls {
    use super::{AudioError, ColorError, DrawError, FontError, ImageError, LayoutError, RendError};
    use core::fmt;

    impl crate::error::Error for RendError {}

    impl fmt::Display for RendError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RendError::AudioError(e) => write!(f, "{e:?}"),
                RendError::ColorError(e) => write!(f, "{e:?}"),
                RendError::DrawError(e) => write!(f, "{e:?}"),
                RendError::FontError(e) => write!(f, "{e:?}"),
                RendError::ImageError(e) => write!(f, "{e:?}"),
                RendError::LayoutError(e) => write!(f, "{e:?}"),
            }
        }
    }
    impl From<AudioError> for RendError {
        fn from(e: AudioError) -> Self {
            Self::AudioError(e)
        }
    }
    impl From<ColorError> for RendError {
        fn from(e: ColorError) -> Self {
            Self::ColorError(e)
        }
    }
    impl From<DrawError> for RendError {
        fn from(e: DrawError) -> Self {
            Self::DrawError(e)
        }
    }
    impl From<FontError> for RendError {
        fn from(e: FontError) -> Self {
            Self::FontError(e)
        }
    }
    impl From<ImageError> for RendError {
        fn from(e: ImageError) -> Self {
            Self::ImageError(e)
        }
    }
    impl From<LayoutError> for RendError {
        fn from(e: LayoutError) -> Self {
            Self::LayoutError(e)
        }
    }
}
