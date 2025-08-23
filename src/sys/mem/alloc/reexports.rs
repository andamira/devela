// devela::sys::mem::alloc::reexports
//
//! Reexported allocation-related items.
//

use crate::_reexport;

/* alloc */

_reexport! { rust: alloc::alloc,
    doc: "Layout of a block of memory.",
    @Layout as MemLayout
}
_reexport! { rust: alloc::alloc,
    tag: crate::TAG_ERROR!(),
    doc: "The [`MemLayout`] parameters violated constraints.",
    @LayoutError as MemLayoutError
}
_reexport! { rust: alloc::alloc,
    doc: "A memory allocator that can be registered as the standard library’s default.",
    GlobalAlloc
}

/* std */

_reexport! { rust: std::alloc,
    doc: "The default memory allocator provided by the operating system.",
    @System as SystemAlloc
}
