// devela_base_core::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()] // public
#![doc = crate::_doc!(modules: crate::sys; mem: alloc, bound, cell, layout, size, view)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
#![doc = crate::_QUO_SYS_MEM!()]
//

mod _reexport; // SYMLINK from /src/sys/mem/_reexport_core.rs

mod namespace; // Mem

pub mod alloc; // Allocation strategies and ownership-backed storage abstractions.
pub mod bound; // Addressing, alignment, and movement constraints over memory.
pub mod cell;
pub mod layout; // Memory layout, bit-validity, and representation invariants.
pub mod size; // size_of_expr!, BitSized, ByteSized,
pub mod view; // Typed and byte-level views over memory.

crate::structural_mods! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        pub use super::{
            namespace::*,
        };
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
        pub use super::_reexport::*;
    }
    _hidden {
        pub use super::{
            alloc::_hidden::*,
            size::_hidden::*,
        };
    }
}
