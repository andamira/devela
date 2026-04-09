// devela::phys::time::_reexport_std

#[cfg(doc)]
use crate::SystemTime;
use crate::{_reexport, _tags};

// NOTE: replicated in ../error
_reexport! { rust: std::time, location: "phys/time", tag: _tags!(time error),
    doc: "Error returned from the `duration_since` and `elapsed` methods on [`SystemTime`].",
    @SystemTimeError as StdSystemTimeError
}
