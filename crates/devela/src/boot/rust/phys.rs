use crate::{_reexport, _tags};

pub(crate) use {time::*, time_source::*};

pub(crate) mod time {
    use super::*;
    #[cfg(all(doc, feature = "std"))]
    use crate::SystemTime;

    /* structs */

    _reexport! { rust: core::time,
        location: "phys/time" => struct Duration, tag: _tags!(time),
        doc: "A span of time, with `u64` seconds and `u32` nanoseconds.",
        Duration
    }
    _reexport! { rust: core::time,
        location: "phys/time" => struct DurationErrorTryFromFloatSecs, tag: _tags!(time error),
        doc: "Error returned from converting floating-point seconds into a [`Duration`].",
        @TryFromFloatSecsError as DurationErrorTryFromFloatSecs // MAYBE RENAME
    }

    // NOTE: replicated in phys/time/error_std.rs::SystemTimeError
    _reexport! { rust: std::time, location: "phys/time", tag: _tags!(time error),
        doc: "Error returned from [`SystemTime`] `duration_since` and `elapsed`.",
        @SystemTimeError as StdSystemTimeError
    }
}

pub(crate) mod time_source {
    use super::*;

    /* structs */

    _reexport! { rust: std::time,
        location: "phys/time/source" => struct SystemInstant, tag: _tags!(time),
        doc: "A measurement of a monotonically nondecreasing clock.",
        @Instant as SystemInstant
    }
    _reexport! { rust: std::time,
        location: "phys/time/source" => struct SystemTime, tag: _tags!(time),
        doc: "A measurement of the system clock.",
        SystemTime
    }

    /* constants */

    _reexport! { rust: std::time,
        location: "phys/time/source" => const UNIX_EPOCH, tag: _tags!(time),
        doc: "A [`SystemTime`] anchored to \"1970-01-01 00:00:00 UTC\".",
        UNIX_EPOCH
    }
}
