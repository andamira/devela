// devela::phys::time::error
//
//!
//

use crate::define_error;
#[cfg(feature = "std")]
use crate::Duration;

#[cfg(feature = "std")]
use ::std::time::SystemTimeError as StdSystemTimeError;

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
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl From<StdSystemTimeError> for SystemTimeError {
    fn from(from: StdSystemTimeError) -> Self {
        SystemTimeError(from.duration())
    }
}

#[cfg(all(feature = "error", feature = "time"))]
pub use full_composite::*;
#[cfg(all(feature = "error", feature = "time"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "error", feature = "time"))))]
mod full_composite {
    use super::*;
    use crate::{DataOverflow, DOC_DATA_OVERFLOW};

    #[doc = crate::TAG_TIME!()]
    #[doc = crate::TAG_RESULT!()]
    /// A text-related result.
    pub type TimeResult<T> = crate::Result<T, TimeError>;

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
}
