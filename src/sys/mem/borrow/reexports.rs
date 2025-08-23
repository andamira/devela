// devela::sys::mem::borrow::reexports
//
//! Reexported items from `core`.
//

use crate::_reexport;

_reexport! { rust: core::borrow,
    doc: "A trait for borrowing data.",
    Borrow
}
_reexport! { rust: core::borrow,
    doc: "A trait for mutably borrowing data.",
    BorrowMut
}
_reexport! { rust: alloc::borrow,
    doc: "A clone-on-write smart pointer.",
    Cow
}
_reexport! { rust: alloc::borrow,
    doc: "A generalization of Clone to borrowed data.",
    ToOwned
}
