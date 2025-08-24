// devela_base_std::phys::time::reexports
//
//!
//

use crate::{_reexport, TAG_TIME};

_reexport! { rust: std::time,
    tag: TAG_TIME!(),
    doc: "A measurement of a monotonically nondecreasing clock.",
    @Instant as SystemInstant
}
_reexport! { rust: std::time,
    tag: TAG_TIME!(),
    doc: "A measurement of the system clock.",
    SystemTime
}
// _reexport! { rust: std::time,
//    tag: concat![TAG_TIME!(), TAG_ERROR!()],
//     tag: crate::TAG_ERROR!(),
//     doc: "Error returned from the `duration_since` and `elapsed` methods on [`SystemTime`].",
//     SystemTimeError
// }
_reexport! { rust: std::time,
    tag: TAG_TIME!(),
    doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
    UNIX_EPOCH
}
