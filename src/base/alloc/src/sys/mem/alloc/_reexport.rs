// devela_base_alloc::sys::mem::alloc::_reexport
//
//!
//

use crate::{_TAG_ALLOCATION, _TAG_ERROR, _reexport};

_reexport! { rust: alloc::alloc, location: "sys/mem", tag: _TAG_ALLOCATION!(),
    doc: "Layout of a block of memory.",
    @Layout as MemLayout
}
_reexport! { rust: alloc::alloc, location: "sys/mem", tag: _TAG_ALLOCATION!() _TAG_ERROR!(),
    doc: "The [`MemLayout`] parameters violated constraints.",
    @LayoutError as MemLayoutError
}
_reexport! { rust: alloc::alloc, location: "sys/mem", tag: _TAG_ALLOCATION!(),
    doc: "A memory allocator that can be registered as the standard library’s default.",
    GlobalAlloc
}
