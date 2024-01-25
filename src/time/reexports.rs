// devela::time
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { rust: core::time, local_module: "time",
    doc: "Represents a span of time, with `u64` seconds and `u32` nanoseconds",
    @Duration as SystemDuration
}
reexport! { rust: std::time, local_module: "time",
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
reexport! { rust: std::time, local_module: "time",
    doc: "A measurement of the system clock,",
    SystemTime
}
