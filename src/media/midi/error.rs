// devela::media::midi::error
//
//!
//

/// A midi-related result.
pub type MidiResult<T> = crate::Result<T, MidiError>;

/// A midi-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MidiError {
    /// TEMP
    MidiError,
}

mod core_impls {
    use crate::{Display, FmtResult, Formatter, MidiError};

    impl crate::Error for MidiError {}
    impl crate::ExtError for MidiError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for MidiError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "MidiError"]
        }
    }
}
