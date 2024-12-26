// devela::media::audio::error
//
//!
//

#[doc = crate::TAG_RESULT!()]
/// An audio-related result.
pub type AudioResult<T> = crate::Result<T, AudioError>;

#[doc = crate::TAG_ERROR_COMPOSITE!()]
/// An audio-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioError {
    /// TEMP
    AudioError,
}

mod core_impls {
    use crate::{AudioError, Display, FmtResult, Formatter};

    impl crate::Error for AudioError {}
    impl crate::ExtError for AudioError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for AudioError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "AudioError"]
        }
    }
}
