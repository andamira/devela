// devela_base_core::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//

mod errors {
    use crate::define_error;

    define_error! { individual: pub struct Timeout;
        DOC_KEY_ALREADY_EXISTS = "The operation has exceeded the allowed execution time.",
        self+f => write!(f, "The operation has exceeded the allowed execution time.")
    }
}

mod reexports {
    use crate::{_TAG_ERROR, _TAG_TIME, _reexport};

    _reexport! { rust: core::time,
        tag: _TAG_TIME!(),
        doc: "A span of time, with `u64` seconds and `u32` nanoseconds.",
        Duration
    }
    _reexport! { rust: core::time,
        tag: _TAG_TIME!() _TAG_ERROR!(),
        doc: "Error returned from converting floating-point seconds into a [`Duration`].",
        @TryFromFloatSecsError as DurationErrorTryFromFloatSecs
    }
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            errors::*,
            reexports::*,
        };
    }
}
