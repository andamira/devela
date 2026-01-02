// devela_base_core::code::error::reexport
//
//!
//

use crate::{_TAG_CODE, _TAG_ERROR, _reexport};

_reexport! { rust: core::error, location: "code/error", tag: _TAG_CODE!() _TAG_ERROR!(),
    doc: "A trait representing the basic expectations for error values.",
    Error
}

_reexport! { rust: core, location: "code/error", tag: _TAG_CODE!() _TAG_ERROR!(),
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}
