// devela::media::error
//
//!
//

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

#[doc = crate::TAG_RESULT!()]
/// A media-related result.
pub type MediaResult<T> = crate::Result<T, MediaError>;

#[doc = crate::TAG_ERROR_COMPOSITE!()]
/// A media-related error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaError {
    ///
    #[cfg(feature = "audio")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "audio")))]
    Audio(AudioError),
    ///
    #[cfg(feature = "color")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "color")))]
    Color(ColorError),
    ///
    #[cfg(feature = "draw")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "draw")))]
    Draw(DrawError),
    ///
    #[cfg(feature = "font")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
    Font(FontError),
    ///
    #[cfg(feature = "image")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
    Image(ImageError),
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
                MediaError::Audio(e) => write!(f, "{e:?}"),
                #[cfg(feature = "color")]
                MediaError::Color(e) => write!(f, "{e:?}"),
                #[cfg(feature = "draw")]
                MediaError::Draw(e) => write!(f, "{e:?}"),
                #[cfg(feature = "font")]
                MediaError::Font(e) => write!(f, "{e:?}"),
                #[cfg(feature = "image")]
                MediaError::Image(e) => write!(f, "{e:?}"),
            }
        }
    }
    #[cfg(feature = "audio")]
    impl From<AudioError> for MediaError {
        fn from(e: AudioError) -> Self {
            Self::Audio(e)
        }
    }
    #[cfg(feature = "color")]
    impl From<ColorError> for MediaError {
        fn from(e: ColorError) -> Self {
            Self::Color(e)
        }
    }
    #[cfg(feature = "draw")]
    impl From<DrawError> for MediaError {
        fn from(e: DrawError) -> Self {
            Self::Draw(e)
        }
    }
    #[cfg(feature = "font")]
    impl From<FontError> for MediaError {
        fn from(e: FontError) -> Self {
            Self::Font(e)
        }
    }
    #[cfg(feature = "image")]
    impl From<ImageError> for MediaError {
        fn from(e: ImageError) -> Self {
            Self::Image(e)
        }
    }
}
