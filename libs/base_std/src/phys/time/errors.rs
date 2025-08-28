// devela_base_std::phys::time::errors
//
//! Time-related errors.
//
// TOC
// - individual errors:
//   - SystemTimeError
// - partial composite errors:
// - full composite errors:
//   - TimeError
//   - TimeResult

use crate::{Duration, define_error, StdSystemTimeError};
// use ::std::time::SystemTimeError as StdSystemTimeError;

/* individual errors */

define_error! { individual:
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub struct SystemTimeError(Duration);
    +tag: crate::TAG_TIME!(),
    DOC_SYSTEM_TIME_ERROR =
    "Returned from the `duration_since` and `elapsed` methods on `SystemTime`.\n\n
This is basically a replication of `std::time::`[`SystemTimeError`][StdSystemTimeError].",
    self+f => write!(f, "SystemTimeError difference: {:?}", self.0)
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))] // RETHINK these
impl From<StdSystemTimeError> for SystemTimeError {
    fn from(from: StdSystemTimeError) -> Self { SystemTimeError(from.duration()) }
}

// NOTE: can't implement this conversion here because it needs std…
// but Timeout is maybe important enough to be defined here.
// MAYBE we could do alternative versions for no_std and std.
//
// use crate::{MpscRecvTimeoutError, Timeout};
// impl From<MpscRecvTimeoutError> for Timeout {
//     fn from(_from: MpscRecvTimeoutError) -> Self { Timeout }
// }

pub use full_composite::*;
mod full_composite {
    use super::*;
    use crate::{CONST, DataOverflow};

    CONST! {
        DOC_DATA_OVERFLOW = "The value has surpassed the bounds of the representable data space.";
    }

    define_error! { composite: fmt(f)
        #[doc = crate::TAG_TIME!()]
        /// A time-related composite error.
        #[non_exhaustive]
        pub enum TimeError {
            DOC_DATA_OVERFLOW: +const
                DataOverflow(o|0: Option<usize>) => DataOverflow(*o),

            DOC_SYSTEM_TIME_ERROR: +const
                SystemTime(d|0: Duration) => SystemTimeError(*d),
        }
    }
    impl From<StdSystemTimeError> for TimeError {
        fn from(from: StdSystemTimeError) -> Self {
            TimeError::SystemTime(from.duration())
        }
    }
}
