// devela::phys::time
//
//! Reexported items from `core`.
//

use crate::reexport;

/* core */

reexport! { rust: core::time,
    doc: "A span of time, with `u64` seconds and `u32` nanoseconds.",
    @Duration as Duration
}
reexport! { rust: core::time,
    tag: crate::TAG_ERROR!(),
    doc: "Error returned from converting floating-point seconds into a [`Duration`].",
    @TryFromFloatSecsError as DurationErrorTryFromFloatSecs
}

/* std */

reexport! { rust: std::time,
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
reexport! { rust: std::time,
    doc: "A measurement of the system clock.",
    SystemTime
}
reexport! { rust: std::time,
    tag: crate::TAG_ERROR!(),
    doc: "Error returned from the `duration_since` and `elapsed` methods on [`SystemTime`].",
    SystemTimeError
}
reexport! { rust: std::time,
    doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
    UNIX_EPOCH
}
