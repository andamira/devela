// devela_base_std::code::error::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: std::backtrace,
    tag: crate::_TAG_DEBUG!(),
    doc: "A captured OS thread stack backtrace.",
    Backtrace
}
_reexport! { rust: std::backtrace,
    tag: crate::_TAG_DEBUG!(),
    doc: "The current status of a backtrace.",
    BacktraceStatus
}
