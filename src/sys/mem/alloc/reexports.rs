// devela::sys::mem::alloc::reexports
//
//! Reexported allocation-related items.
//

use crate::reexport;

/* alloc */

reexport! { rust: alloc::alloc,
    doc: "Layout of a block of memory.",
    Layout
}
reexport! { rust: alloc::alloc,
    doc: "The parameters given to a `Layout` constructor did not satisfy the constraints.",
    LayoutError
}
reexport! { rust: alloc::alloc,
    doc: "",
    GlobalAlloc
}
