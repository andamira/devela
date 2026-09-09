// devela/src/error/_reexport_core.rs

use crate::{_reexport, _tags};

_reexport! { rust: core::error,
    location: "error" => trait Error, tag: _tags!(code error),
    doc: "A trait representing the basic expectations for error values.",
    Error
}
