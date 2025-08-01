// devela::sys::mem::borrow::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::borrow,
    doc: "A trait for borrowing data.",
    Borrow
}
reexport! { rust: core::borrow,
    doc: "A trait for mutably borrowing data.",
    BorrowMut
}
reexport! { rust: alloc::borrow,
    doc: "A clone-on-write smart pointer.",
    Cow
}
reexport! { rust: alloc::borrow,
    doc: "A generalization of Clone to borrowed data.",
    ToOwned
}
