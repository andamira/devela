// devela::rend::audio::error
//
//!
//

/// An audio rendering result.
pub type AudioResult<T> = crate::Result<T, AudioError>;

/// An audio rendering error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioError {
    /// TEMP
    AudioError,
}

impl crate::Error for AudioError {}

mod core_impls {
    use crate::{AudioError, Display, FmtResult, Formatter};

    impl Display for AudioError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "AudioError"]
        }
    }
}
