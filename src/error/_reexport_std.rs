// devela/src/error/_reexport_std.rs

#[allow(unused_imports, reason = "symlinked from devela")]
use crate::{_reexport, _tags};

_reexport! { rust: std::backtrace, location: "error", tag: _tags!(debug),
    doc: "A captured OS thread stack backtrace.",
    Backtrace
}
_reexport! { rust: std::backtrace, location: "error", tag: _tags!(debug),
    doc: "The current status of a backtrace.",
    BacktraceStatus
}
