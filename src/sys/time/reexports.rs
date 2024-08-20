// devela::sys::time
//
//! Reexported items from `core`.
//

use crate::code::reexport;

/* core */

// IMPROVE
reexport! { rust: core::time,
    doc: "Represents a span of time, with `u64` seconds and `u32` nanoseconds.",
    @Duration as Duration
}
reexport! { rust: core::time,
    doc: "Can be returned when converting a floating-point value of seconds into a Duration.",
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
    doc: "An error returned from the duration_since and elapsed methods on [`SystemTime`].",
    SystemTimeError
}
reexport! { rust: std::time,
    doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
    UNIX_EPOCH
}
