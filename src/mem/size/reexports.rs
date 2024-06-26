// devela::mem::size::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

#[doc(inline)]
pub use crate::code::Sized;

reexport! { rust: core::mem,
    doc: "Returns the size of a type in bytes.",
    @size_of as mem_size_of
}
reexport! { rust: core::mem,
    doc: "Returns the size of the pointed-to value in bytes.",
    @size_of_val as mem_size_of_val
}
