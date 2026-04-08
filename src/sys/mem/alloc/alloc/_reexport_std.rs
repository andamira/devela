// devela::sys::mem::alloc::_reexport_std

use crate::{_reexport, _tags};

_reexport! { rust: std::alloc, location: "sys/mem", tag: _tags!(allocation),
    doc: "The default memory allocator provided by the operating system.",
    @System as SystemAlloc
}
