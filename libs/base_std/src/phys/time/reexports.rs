// devela_base_std::phys::time::reexports
//
//!
//

use crate::{_TAG_ERROR, _TAG_TIME, _reexport};

_reexport! { rust: std::time,
    tag: _TAG_TIME!(),
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
_reexport! { rust: std::time,
    tag: _TAG_TIME!(),
    doc: "A measurement of the system clock.",
    SystemTime
}
// NOTE: replicated in phys::time:errors
_reexport! { rust: std::time,
    tag: concat![_TAG_TIME!(), _TAG_ERROR!()],
    doc: "Error returned from the `duration_since` and `elapsed` methods on [`SystemTime`].",
    @SystemTimeError as StdSystemTimeError
}
_reexport! { rust: std::time,
    tag: _TAG_TIME!(),
    doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
    UNIX_EPOCH
}
