// devela_base_alloc::sys::mem::borrow::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: alloc::borrow,
    doc: "A clone-on-write smart pointer.",
    Cow
}
_reexport! { rust: alloc::borrow,
    doc: "A generalization of Clone to borrowed data.",
    ToOwned
}
