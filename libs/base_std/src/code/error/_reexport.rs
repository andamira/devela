// devela_base_std::code::error::_reexport
//
//!
//

#[allow(unused_imports, reason = "symlinked from devela")]
use crate::{_TAG_DEBUG, _reexport};

_reexport! { rust: std::backtrace, location: "code/error", tag: _TAG_DEBUG!(),
    doc: "A captured OS thread stack backtrace.",
    Backtrace
}
_reexport! { rust: std::backtrace, location: "code/error", tag: _TAG_DEBUG!(),
    doc: "The current status of a backtrace.",
    BacktraceStatus
}
