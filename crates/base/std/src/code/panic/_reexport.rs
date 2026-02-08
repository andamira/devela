// devela_base_std::code::panic::_reexport

#[allow(unused_imports, reason = "re-exported from devela")]
use crate::{_TAG_CODE, _reexport};

/* structs */

_reexport! { rust: std::panic,
    location: "code/panic",
    tag: _TAG_CODE!(),
    doc: "Passed to `std::panic::set_hook` where panics can have arbitrary payloads.",
    PanicHookInfo
}
