// devela/src/sys/mem/alloc/alloc/_reexport_alloc.rs

use crate::{_reexport, _tags};

_reexport! { rust: alloc::alloc,
    location: "sys/mem" => struct MemLayout, tag: _tags!(allocation),
    doc: "Layout of a block of memory.",
    @Layout as MemLayout
}
_reexport! { rust: alloc::alloc,
    location: "sys/mem" => struct MemLayoutError, tag: _tags!(allocation error),
    doc: "The [`MemLayout`] parameters violated constraints.",
    @LayoutError as MemLayoutError
}
_reexport! { rust: alloc::alloc,
    location: "sys/mem" => trait GlobalAlloc, tag: _tags!(allocation),
    doc: "A memory allocator that can be registered as the standard library's default.",
    GlobalAlloc
}
