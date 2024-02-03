// devela::result::reexport_std
//
//! Reexported items from `std`.
//

use crate::code::reexport;

reexport! { rust: no_std|std::error, local_module: "error",
    doc: "A trait representing the basic expectations for error values.",
    Error
}
