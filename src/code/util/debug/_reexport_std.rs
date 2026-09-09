// devela/src/code/util/debug/_reexport_std.rs

#[allow(unused_imports, reason = "symlinked from devela")]
use crate::{_reexport, _tags};

_reexport! { rust: std::backtrace,
    location: "code/util/debug" => struct Backtrace, tag: _tags!(code debug),
    doc: "A captured OS thread stack backtrace.",
    Backtrace
}

_reexport! { rust: std::backtrace,
    location: "code/util/debug" => enum BacktraceStatus, tag: _tags!(code debug),
    doc: "The current status of a backtrace.",
    BacktraceStatus
}
