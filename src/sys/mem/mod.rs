// devela::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()] // public
#![doc = crate::_doc!(modules: crate::sys; mem: alloc, bound, cell, layout, size, view)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
//
// safety
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

mod _reexport_core;
#[cfg(feature = "alloc")]
mod _reexport_alloc;

mod ext; // MemExt
#[cfg(feature = "std")]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod guard; // Current, CurrrentGuard
mod namespace; // Mem

pub mod alloc; // Memory allocation, arenas, and ownership-backed storage.
pub mod bound; // Addressing, alignment, and movement constraints over memory.
pub mod cell; // ExtCellOption, ::core::cell::*
pub mod layout; // Memory layout, bit-validity, and representation invariants.
pub mod size; // size_of_expr!, BitSized, ByteSized,
pub mod view; // Typed and byte-level views over memory.

crate::structural_mods! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        pub use super::{
            ext::*,
            namespace::*,
        };
        #[cfg(feature = "std")]
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_layout"))))]
        pub use super::guard::{Current, CurrentGuard};
    }
    _pub_mods {
        pub use super::{
            alloc::_all::*,
            bound::_all::*,
            cell::_all::*,
            layout::_all::*,
            size::_all::*,
            view::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
    }
    _hidden {
        pub use super::{
            alloc::_hidden::*,
            size::_hidden::*,
        };
    }
}
