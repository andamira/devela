// devela::phys::time::_reexport

use crate::{_reexport, _tags};

_reexport! { rust: core::time, location: "phys/time", tag: _tags!(time),
    doc: "A span of time, with `u64` seconds and `u32` nanoseconds.",
    Duration
}
_reexport! { rust: core::time, location: "phys/time", tag: _tags!(time error),
    doc: "Error returned from converting floating-point seconds into a [`Duration`].",
    @TryFromFloatSecsError as DurationErrorTryFromFloatSecs // MAYBE RENAME
}
