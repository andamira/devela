use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: alloc::alloc,
    location: "sys/mem/alloc" => trait AllocGlobal, tag: _tags!(allocation),
    doc: "An allocator that forwards to the configured global allocator.",
    @Global as AllocGlobal
}
