// devela::phys::time::source::_reexport_std

use crate::{_reexport, _tags};

_reexport! { rust: std::time, location: "phys/time/source", tag: _tags!(time),
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
_reexport! { rust: std::time, location: "phys/time/source", tag: _tags!(time),
    doc: "A measurement of the system clock.",
    SystemTime
}
_reexport! { rust: std::time, location: "phys/time/source", tag: _tags!(time),
    doc: "A [`SystemTime`] anchored to \"1970-01-01 00:00:00 UTC\".",
    UNIX_EPOCH
}
