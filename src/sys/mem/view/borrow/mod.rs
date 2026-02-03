// devela::sys::mem::view::borrow
//
#![doc = crate::_DOC_SYS_MEM_VIEW_BORROW!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; borrow)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/sys/mem/view/borrow/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /src/base/alloc/src/sys/mem/view/borrow/_reexport.rs

mod backing; // Backing
mod maybe; // MaybeOwned
mod ownership; // Ownership, BackingChoice

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            backing::*,
            maybe::*,
            ownership::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
    }
}
