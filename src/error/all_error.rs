// devela::error::all_error
//
//! Defines the [`AllError`] enum.
//

use super::reexports::crate_errors::{DataError, *};

/// The root result type, aggregating all module-specific results.
pub type AllResult<T> = crate::Result<T, AllError>;

/// The root error type, aggregating all module-specific errors.
///
/// This error is designed to encompass all possible errors within the library's domain,
/// providing a unified interface for error handling across modules.
#[derive(Debug)]
pub enum AllError {
    /// A data error.
    Data(DataError),

    /// A numeric error.
    Num(NumError),

    /// A rendering error.
    #[cfg(_rend_·)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(_rend_·)))]
    Rend(RendError),

    /// A text error.
    Text(TextError),

    /// An I/O error.
    #[cfg(feature = "io")]
    Io(IoError),

    /// A time error.
    #[cfg(feature = "time")]
    Time(TimeError),

    /// Other static error.
    Other(&'static str),
}

mod core_impls {
    use super::*;
    use crate::impl_trait;

    impl crate::Error for AllError {}

    impl_trait! { fmt::Display for AllError |self, f| {
        use AllError as E;
        match self {
            E::Data(e) => write!(f, "{e:?}"),
            E::Num(e) => write!(f, "{e:?}"),
            #[cfg(_rend_·)]
            E::Rend(e) => write!(f, "{e:?}"),
            E::Text(e) => write!(f, "{e:?}"),
            #[cfg(feature = "io")]
            E::Io(e) => write!(f, "{e:?}"),
            #[cfg(feature = "time")]
            E::Time(e) => write!(f, "{e:?}"),
            E::Other(s) => write!(f, "{s}"),
        }
    }}
}
