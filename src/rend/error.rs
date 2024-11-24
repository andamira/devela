// devela::rend::error
//
//!
//

#[cfg(feature = "audio")]
use crate::rend::AudioError;
#[cfg(feature = "color")]
use crate::rend::ColorError;
#[cfg(feature = "draw")]
use crate::rend::DrawError;
#[cfg(feature = "font")]
use crate::rend::FontError;
#[cfg(feature = "image")]
use crate::rend::ImageError;
#[cfg(feature = "layout")]
use crate::rend::LayoutError;

/// A rendering media result.
pub type RendResult<T> = core::result::Result<T, RendError>;

/// A rendering media error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendError {
    ///
    #[cfg(feature = "audio")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "audio")))]
    AudioError(AudioError),
    ///
    #[cfg(feature = "color")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "color")))]
    ColorError(ColorError),
    ///
    #[cfg(feature = "draw")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "draw")))]
    DrawError(DrawError),
    ///
    #[cfg(feature = "font")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
    FontError(FontError),
    ///
    #[cfg(feature = "image")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
    ImageError(ImageError),
    ///
    #[cfg(feature = "layout")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "layout")))]
    LayoutError(LayoutError),
}

mod core_impls {
    use super::RendError;
    #[cfg(feature = "audio")]
    use crate::rend::AudioError;
    #[cfg(feature = "color")]
    use crate::rend::ColorError;
    #[cfg(feature = "draw")]
    use crate::rend::DrawError;
    #[cfg(feature = "font")]
    use crate::rend::FontError;
    #[cfg(feature = "image")]
    use crate::rend::ImageError;
    #[cfg(feature = "layout")]
    use crate::rend::LayoutError;
    use core::fmt;

    impl crate::error::Error for RendError {}

    impl fmt::Display for RendError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                #[cfg(feature = "audio")]
                RendError::AudioError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "color")]
                RendError::ColorError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "draw")]
                RendError::DrawError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "font")]
                RendError::FontError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "image")]
                RendError::ImageError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "layout")]
                RendError::LayoutError(e) => write!(f, "{e:?}"),
            }
        }
    }
    #[cfg(feature = "audio")]
    impl From<AudioError> for RendError {
        fn from(e: AudioError) -> Self {
            Self::AudioError(e)
        }
    }
    #[cfg(feature = "color")]
    impl From<ColorError> for RendError {
        fn from(e: ColorError) -> Self {
            Self::ColorError(e)
        }
    }
    #[cfg(feature = "draw")]
    impl From<DrawError> for RendError {
        fn from(e: DrawError) -> Self {
            Self::DrawError(e)
        }
    }
    #[cfg(feature = "font")]
    impl From<FontError> for RendError {
        fn from(e: FontError) -> Self {
            Self::FontError(e)
        }
    }
    #[cfg(feature = "image")]
    impl From<ImageError> for RendError {
        fn from(e: ImageError) -> Self {
            Self::ImageError(e)
        }
    }
    #[cfg(feature = "layout")]
    impl From<LayoutError> for RendError {
        fn from(e: LayoutError) -> Self {
            Self::LayoutError(e)
        }
    }
}
