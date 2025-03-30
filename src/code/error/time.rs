// devela::code::error::time
//
//! Time-related errors.
//
// TOC
// - individual data-related error types:
//   - SystemTimeError
//   - Timeout
// - partial composite errors:
// - full composite errors:
//   - TimeError
//   - TimeResult

use crate::define_error;
#[cfg(feature = "std")]
use {
    crate::{Duration, MpscRecvTimeoutError},
    ::std::time::SystemTimeError as StdSystemTimeError,
};

/* individual errors */

define_error! { individual:
    +tag: crate::TAG_TIME!(),
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub struct SystemTimeError(Duration);
    DOC_SYSTEM_TIME_ERROR =
    "Returned from the `duration_since` and `elapsed` methods on `SystemTime`.\n\n
This is basically a replication of `std::time::`[`SystemTimeError`][StdSystemTimeError].",
    self+f => write!(f, "SystemTimeError difference: {:?}", self.0)
}
#[cfg(feature = "std")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl From<StdSystemTimeError> for SystemTimeError {
    fn from(from: StdSystemTimeError) -> Self { SystemTimeError(from.duration()) }
}

define_error! { individual: pub struct Timeout;
    DOC_KEY_ALREADY_EXISTS = "The operation has exceeded the allowed execution time.",
    self+f => write!(f, "The operation has exceeded the allowed execution time.")
}
#[cfg(feature = "std")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl From<MpscRecvTimeoutError> for Timeout {
    fn from(_from: MpscRecvTimeoutError) -> Self { Timeout }
}

#[cfg(all(feature = "error", feature = "time"))]
pub use full_composite::*;
#[cfg(all(feature = "error", feature = "time"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "error", feature = "time"))))]
mod full_composite {
    use super::*;
    use crate::{DOC_DATA_OVERFLOW, DataOverflow};

    define_error! { composite: fmt(f)
        #[doc = crate::TAG_TIME!()]
        /// A time-related composite error.
        #[non_exhaustive]
        pub enum TimeError {
            DOC_DATA_OVERFLOW:
                DataOverflow(o|0: Option<usize>) => DataOverflow(*o),

            // FIXME
            #[cfg(feature = "std")]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
            DOC_SYSTEM_TIME_ERROR:
                SystemTime(d|0: Duration) => SystemTimeError(*d),
        }
    }
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    impl From<StdSystemTimeError> for TimeError {
        fn from(from: StdSystemTimeError) -> Self {
            TimeError::SystemTime(from.duration())
        }
    }

    #[doc = crate::TAG_TIME!()]
    #[doc = crate::TAG_RESULT!()]
    /// A time-related result.
    pub type TimeResult<T> = crate::Result<T, TimeError>;
}
