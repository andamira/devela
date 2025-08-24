// devela::sys::mem::alloc::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: std::alloc,
    doc: "The default memory allocator provided by the operating system.",
    @System as SystemAlloc
}
