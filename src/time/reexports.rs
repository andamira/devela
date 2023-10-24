// devela::time
//
//! Reexported items from `core`.
//

use crate::meta::reexport;

reexport! { rust: core::time, local_module: "time",
    doc: "Represents a span of time, with `u64` seconds plus and `u32` nanoseconds",
    Duration
}
reexport! { rust: "std"|std::time, local_module: "time",
    doc: "A measurement of a monotonically nondecreasing clock.",
    Instant
}
reexport! { rust: "std"|std::time, local_module: "time",
    doc: "A measurement of the system clock,",
    SystemTime
}
