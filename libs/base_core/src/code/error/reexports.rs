// devela_base_core::code::error::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
}

_reexport! { rust: core,
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}
