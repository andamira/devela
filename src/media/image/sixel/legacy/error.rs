// devela::media::image::sixel::error

use devela::Error;

/// A sixel-related result.
pub(crate) type LegacySixelResult<T> = Result<T, LegacySixelError>;

/// A sixel-related error.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LegacySixelError {
    /// Bad argument detected.
    BadArgument,
    /// Bad input detected.
    BadInput,
    /// Integer overflow.
    BadIntegerOverflow,
}

mod _core_impls {
    use super::{Error, LegacySixelError};
    use core::fmt;

    impl Error for LegacySixelError {}

    impl fmt::Display for LegacySixelError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LegacySixelError::BadArgument => write!(f, "bad argument detected"),
                LegacySixelError::BadInput => write!(f, "bad input detected"),
                LegacySixelError::BadIntegerOverflow => write!(f, "integer overflow"),
            }
        }
    }
}
