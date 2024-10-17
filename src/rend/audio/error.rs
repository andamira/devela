// devela::rend::audio::error
//
//!
//

/// An audio rendering result.
pub type AudioResult<T> = core::result::Result<T, AudioError>;

/// An audio rendering error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioError {
    /// TEMP
    AudioError,
}

impl crate::error::Error for AudioError {}

mod core_impls {
    use super::AudioError;
    use core::fmt;

    impl fmt::Display for AudioError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write![f, "AudioError"]
        }
    }
}
