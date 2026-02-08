// devela::sys::mem::alloc::_reexport

use crate::{_TAG_ALLOCATION, _reexport};

_reexport! { rust: std::alloc, location: "sys/mem", tag: _TAG_ALLOCATION!(),
    doc: "The default memory allocator provided by the operating system.",
    @System as SystemAlloc
}
