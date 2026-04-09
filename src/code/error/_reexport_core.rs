// devela::code::error::reexport_core

use crate::{_reexport, _tags};

_reexport! { rust: core::error, location: "code/error", tag: _tags!(code error),
    doc: "A trait representing the basic expectations for error values.",
    Error
}

_reexport! { rust: core, location: "code/error", tag: _tags!(code error),
    doc: "Causes compilation to fail with the given message when encountered.",
    compile_error
}
