// devela_base_std::code::panic::_reexport

use crate::_reexport;

/* structs */

_reexport! { rust: std::panic,
    doc: "Passed to `std::panic::set_hook` in `std`, where panics can have arbitrary payloads.",
    PanicHookInfo
}
