// devela::phys::time
//
//! Reexported items from `core`.
//

use crate::{TAG_ERROR, TAG_TIME, reexport};

/* core */

reexport! { rust: core::time,
    tag: TAG_TIME!(),
    doc: "A span of time, with `u64` seconds and `u32` nanoseconds.",
    Duration
}
reexport! { rust: core::time,
    tag: concat![TAG_TIME!(), TAG_ERROR!()],
    doc: "Error returned from converting floating-point seconds into a [`Duration`].",
    @TryFromFloatSecsError as DurationErrorTryFromFloatSecs
}

/* std */

reexport! { rust: std::time,
    tag: TAG_TIME!(),
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
reexport! { rust: std::time,
    tag: TAG_TIME!(),
    doc: "A measurement of the system clock.",
    SystemTime
}
// reexport! { rust: std::time,
//    tag: concat![TAG_TIME!(), TAG_ERROR!()],
//     tag: crate::TAG_ERROR!(),
//     doc: "Error returned from the `duration_since` and `elapsed` methods on [`SystemTime`].",
//     SystemTimeError
// }
reexport! { rust: std::time,
    tag: TAG_TIME!(),
    doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
    UNIX_EPOCH
}
