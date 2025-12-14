// devela_base_std::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//

/// Time sources.
pub mod source {
    use crate::{_TAG_TIME, _reexport};

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
    _reexport! { rust: std::time,
        tag: _TAG_TIME!(),
        doc: "A [`SystemTime`] anchored to “1970-01-01 00:00:00 UTC”.",
        UNIX_EPOCH
    }
}

mod errors;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            errors::*,
        };
    }
    _pub_mods {
        pub use super::{
            source::*,
        };
    }
}
