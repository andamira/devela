use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: core::alloc,
    location: "sys/mem/alloc" => struct AllocError, tag: _tags!(allocation error),
    doc: "An allocation failure.",
    AllocError
}
_reexport! { rust: core::alloc,
    location: "sys/mem/alloc" => struct MemLayout, tag: _tags!(allocation),
    doc: "Layout of a block of memory.",
    @Layout as MemLayout
}
_reexport! { rust: core::alloc,
    location: "sys/mem/alloc" => struct MemLayoutError, tag: _tags!(allocation error),
    doc: "The [`MemLayout`] parameters violated constraints.",
    @LayoutError as MemLayoutError
}

/* traits */

_reexport! { rust: core::alloc,
    location: "sys/mem/alloc" => trait Allocator, tag: _tags!(allocation),
    doc: "An allocator for arbitrary memory blocks.",
    Allocator
}
_reexport! { rust: core::alloc,
    location: "sys/mem/alloc" => trait GlobalAlloc, tag: _tags!(allocation),
    doc: "A memory allocator that can be registered as the standard library's default.",
    GlobalAlloc
}
