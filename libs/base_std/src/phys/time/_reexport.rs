// devela_base_std::phys::time::_reexport

use crate::{_TAG_ERROR, _TAG_TIME, _reexport};

// NOTE: replicated in ../error
_reexport! { rust: std::time, location: "phys/time", tag: _TAG_TIME!() _TAG_ERROR!(),
    doc: "Error returned from the `duration_since` and `elapsed` methods on [`SystemTime`].",
    @SystemTimeError as StdSystemTimeError
}
