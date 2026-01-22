// devela_base_core::phys::time::_reexport

use crate::{_TAG_ERROR, _TAG_TIME, _reexport};

_reexport! { rust: core::time, location: "phys/time", tag: _TAG_TIME!(),
    doc: "A span of time, with `u64` seconds and `u32` nanoseconds.",
    Duration
}
_reexport! { rust: core::time, location: "phys/time", tag: _TAG_TIME!() _TAG_ERROR!(),
    doc: "Error returned from converting floating-point seconds into a [`Duration`].",
    @TryFromFloatSecsError as DurationErrorTryFromFloatSecs // MAYBE RENAME
}
