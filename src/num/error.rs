// devela::num::error
//
//!
//

use super::Sign;

/// A numerical result.
pub type NumResult<T> = core::result::Result<T, NumError>;

/// A numerical error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumError {
    /// The requested numerical functionality is not implemented.
    ///
    /// This is the default implementation of every numeric trait method.
    NotImplemented,

    /// The requested numerical functionality is not supported by this number type.
    NotSupported,

    /// Unspecified error.
    ///
    /// When no clearer error can be given.
    // RETHINK
    Unspecified,

    /// An invalid value was received for the given type or operation.
    Invalid,

    /// The provided values are not compatible in size.
    MismatchedSizes,

    /// A non-negative value is required.
    NonNegativeRequired,

    /// A positive value is required.
    PositiveRequired,

    /// An arithmetic overflow error, with an optional associated sign.
    Overflow(Option<Sign>),
}

#[allow(dead_code)]
impl NumError {
    pub(crate) const fn ni<T>() -> NumResult<T> {
        Err(NumError::NotImplemented)
    }
    pub(crate) const fn ns<T>() -> NumResult<T> {
        Err(NumError::NotSupported)
    }
}

impl crate::result::Error for NumError {}

mod core_impls {
    use super::{NumError as E, Sign};
    use core::fmt;

    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                E::NotImplemented => write!(f, "Not implemented."),
                E::NotSupported => write!(f, "Not supported."),
                E::Unspecified => write!(f, "Unspecified."),
                E::Invalid => write!(f, "Invalid value."),
                E::MismatchedSizes => {
                    write!(f, "The provided values are not compatible in size.")
                }
                E::NonNegativeRequired => write!(f, "A non-negative value is required."),
                E::PositiveRequired => write!(f, "A positive value is required.."),
                E::Overflow(sign) => {
                    if let Some(sign) = sign {
                        match sign {
                            Sign::Positive => write!(f, "Positive overflow."),
                            Sign::Negative => write!(f, "Negative overflow."),
                            Sign::None => write!(f, "Unsigned overflow."), // not meaninful
                        }
                    } else {
                        write!(f, "Overflow.")
                    }
                }
            }
        }
    }
}
