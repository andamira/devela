// devela_base_std::phys::time::source::_reexport

use crate::{_TAG_TIME, _reexport};

_reexport! { rust: std::time, location: "phys/time/source", tag: _TAG_TIME!(),
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
_reexport! { rust: std::time, location: "phys/time/source", tag: _TAG_TIME!(),
    doc: "A measurement of the system clock.",
    SystemTime
}
_reexport! { rust: std::time, location: "phys/time/source", tag: _TAG_TIME!(),
    doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
    UNIX_EPOCH
}
