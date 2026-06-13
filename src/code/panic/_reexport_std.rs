// devela/src/code/panic/_reexport_std.rs

#[allow(unused_imports, reason = "re-exported from devela")]
use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::panic, location: "code/panic", tag: _tags!(code),
    doc: "Passed to `std::panic::set_hook` where panics can have arbitrary payloads.",
    PanicHookInfo
}
