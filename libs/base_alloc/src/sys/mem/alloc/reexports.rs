// devela_base_alloc::sys::mem::alloc::reexports
//
//!
//

use crate::_reexport;

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
