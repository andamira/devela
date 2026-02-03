// devela::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()] // public
#![doc = crate::_doc!(modules: crate::sys; mem: cell)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
//
// safety
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

mod _reexport_core; // SYMLINK to /src/base/core/src/sys/mem/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /src/base/alloc/src/sys/mem/_reexport.rs

mod alloc; // Memory allocation, arenas, and ownership-backed storage.
mod bound; // Addressing, alignment, and movement constraints over memory.
mod ext; // MemExt
mod layout; // Memory layout, bit-validity, and representation invariants.
mod size; // size_of_expr!, BitSized, ByteSized,
mod view; // Typed and byte-level views over memory.

#[cfg(feature = "std")]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod guard; // Current, CurrrentGuard

pub mod cell; // ExtCellOption, ::core::cell::*

crate::structural_mods! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        pub use super::{
            alloc::_all::*,
            bound::_all::*,
            ext::*,
            layout::_all::*,
            size::_all::*,
            view::_all::*,
        };
        #[cfg(feature = "std")]
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_layout"))))]
        pub use super::guard::{Current, CurrentGuard};
    }
    _pub_mods {
        pub use super::cell::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;

        #[doc(inline)]
        pub use devela_base_core::sys::mem::{
            CacheAlign, MaybeByte, Mem, MemAligned, cswap, define_arena,
        };

        #[doc(inline)]
        pub use crate::Sized;
    }
    _hidden {
        pub use super::size::_hidden::*;
    }
}
