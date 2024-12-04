// devela::media::error
//
//!
//

#[cfg(feature = "audio")]
use crate::media::AudioError;
#[cfg(feature = "color")]
use crate::media::ColorError;
#[cfg(feature = "draw")]
use crate::media::DrawError;
#[cfg(feature = "font")]
use crate::media::FontError;
#[cfg(feature = "image")]
use crate::media::ImageError;
#[cfg(feature = "layout")]
use crate::media::LayoutError;

/// A media-related result.
pub type MediaResult<T> = crate::Result<T, MediaError>;

/// A media-related error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaError {
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
    #[cfg(feature = "audio")]
    use crate::AudioError;
    #[cfg(feature = "color")]
    use crate::ColorError;
    #[cfg(feature = "draw")]
    use crate::DrawError;
    #[cfg(feature = "font")]
    use crate::FontError;
    #[cfg(feature = "image")]
    use crate::ImageError;
    #[cfg(feature = "layout")]
    use crate::LayoutError;
    use crate::{Display, FmtResult, Formatter, MediaError};

    impl crate::Error for MediaError {}
    impl crate::ExtError for MediaError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for MediaError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            match self {
                #[cfg(feature = "audio")]
                MediaError::AudioError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "color")]
                MediaError::ColorError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "draw")]
                MediaError::DrawError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "font")]
                MediaError::FontError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "image")]
                MediaError::ImageError(e) => write!(f, "{e:?}"),
                #[cfg(feature = "layout")]
                MediaError::LayoutError(e) => write!(f, "{e:?}"),
            }
        }
    }
    #[cfg(feature = "audio")]
    impl From<AudioError> for MediaError {
        fn from(e: AudioError) -> Self {
            Self::AudioError(e)
        }
    }
    #[cfg(feature = "color")]
    impl From<ColorError> for MediaError {
        fn from(e: ColorError) -> Self {
            Self::ColorError(e)
        }
    }
    #[cfg(feature = "draw")]
    impl From<DrawError> for MediaError {
        fn from(e: DrawError) -> Self {
            Self::DrawError(e)
        }
    }
    #[cfg(feature = "font")]
    impl From<FontError> for MediaError {
        fn from(e: FontError) -> Self {
            Self::FontError(e)
        }
    }
    #[cfg(feature = "image")]
    impl From<ImageError> for MediaError {
        fn from(e: ImageError) -> Self {
            Self::ImageError(e)
        }
    }
    #[cfg(feature = "layout")]
    impl From<LayoutError> for MediaError {
        fn from(e: LayoutError) -> Self {
            Self::LayoutError(e)
        }
    }
}
