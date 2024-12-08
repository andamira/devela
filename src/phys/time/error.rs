// devela::phys::time::error
//
//!
//

use super::Duration;

/// A time-related result.
pub type TimeResult<T> = crate::Result<T, TimeError>;

/// A time-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeError {
    /// The `Duration` from a [`SystemTimeError`][std::time::SystemTimeError].
    ///
    /// Used to learn how far in the opposite direction a [`SystemTime`][super::SystemTime] lies.
    // IMPROVE: generalize.
    SystemTimeError(Duration),

    /// The given value is out of bounds.
    OutOfBounds(Option<usize>),
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
mod std_impls {
    use super::TimeError;
    use std::time::SystemTimeError;

    impl From<SystemTimeError> for TimeError {
        fn from(time: SystemTimeError) -> Self {
            TimeError::SystemTimeError(time.duration())
        }
    }
}

mod core_impls {
    use crate::{impl_trait, TimeError};

    impl crate::Error for TimeError {}
    impl crate::ExtError for TimeError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }
    impl_trait! { fmt::Display for TimeError |self, f| {
        use TimeError as E;
        match self {
            E::SystemTimeError(d) => {
                write!(f, "SystemTimeError({d:?})")
            }
            E::OutOfBounds(v) => match v {
                Some(v) => write!(f, "The given value {v} is out of bounds."),
                None => write!(f, "The given value is out of bounds."),
            },
        }
    }}
}
