use crate::{_reexport, _tags};

/* structs */

_reexport! { rust: std::alloc,
    location: "sys/mem/alloc" => struct AllocSystem, tag: _tags!(allocation),
    doc: "The default memory allocator provided by the operating system.",
    @System as AllocSystem
}
