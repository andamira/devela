// devela::rend::error
//
//!
//

#[cfg(feature = "rend_audio")]
use crate::rend::AudioError;
#[cfg(feature = "rend_color")]
use crate::rend::ColorError;
#[cfg(feature = "rend_draw")]
use crate::rend::DrawError;
#[cfg(feature = "rend_font")]
use crate::rend::FontError;
#[cfg(feature = "rend_image")]
use crate::rend::ImageError;
#[cfg(feature = "rend_layout")]
use crate::rend::LayoutError;

/// A rend media result.
pub type RendResult<T> = core::result::Result<T, RendError>;

/// A rend media error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendError {
    ///
    #[cfg(feature = "rend_audio")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_audio")))]
    AudioError(AudioError),
    ///
    #[cfg(feature = "rend_color")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_color")))]
    ColorError(ColorError),
    ///
    #[cfg(feature = "rend_draw")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_draw")))]
    DrawError(DrawError),
    ///
    #[cfg(feature = "rend_font")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_font")))]
    FontError(FontError),
    ///
    #[cfg(feature = "rend_image")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_font")))]
    ImageError(ImageError),
    ///
    #[cfg(feature = "rend_layout")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_layout")))]
    LayoutError(LayoutError),
}

mod core_impls {
    use super::RendError;
    #[cfg(feature = "rend_audio")]
    use crate::rend::AudioError;
    #[cfg(feature = "rend_color")]
    use crate::rend::ColorError;
    #[cfg(feature = "rend_draw")]
    use crate::rend::DrawError;
    #[cfg(feature = "rend_font")]
    use crate::rend::FontError;
    #[cfg(feature = "rend_image")]
    use crate::rend::ImageError;
    #[cfg(feature = "rend_layout")]
    use crate::rend::LayoutError;
    use core::fmt;

    impl crate::error::Error for RendError {}

    impl fmt::Display for RendError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                #[cfg(feature = "rend_audio")]
                RendError::AudioError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "rend_color")]
                RendError::ColorError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "rend_draw")]
                RendError::DrawError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "rend_font")]
                RendError::FontError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "rend_image")]
                RendError::ImageError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "rend_layout")]
                RendError::LayoutError(e) => write!(f, "{e:?}"),
            }
        }
    }
    #[cfg(feature = "rend_audio")]
    impl From<AudioError> for RendError {
        fn from(e: AudioError) -> Self {
            Self::AudioError(e)
        }
    }
    #[cfg(feature = "rend_color")]
    impl From<ColorError> for RendError {
        fn from(e: ColorError) -> Self {
            Self::ColorError(e)
        }
    }
    #[cfg(feature = "rend_draw")]
    impl From<DrawError> for RendError {
        fn from(e: DrawError) -> Self {
            Self::DrawError(e)
        }
    }
    #[cfg(feature = "rend_font")]
    impl From<FontError> for RendError {
        fn from(e: FontError) -> Self {
            Self::FontError(e)
        }
    }
    #[cfg(feature = "rend_image")]
    impl From<ImageError> for RendError {
        fn from(e: ImageError) -> Self {
            Self::ImageError(e)
        }
    }
    #[cfg(feature = "rend_layout")]
    impl From<LayoutError> for RendError {
        fn from(e: LayoutError) -> Self {
            Self::LayoutError(e)
        }
    }
}
