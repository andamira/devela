// devela::sys::mem::borrow
//
#![doc = crate::_DOC_SYS_MEM_BORROW!()]
//

mod _reexport_core; // SYMLINK to /libs/base_core/src/sys/mem/borrow/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/sys/mem/borrow/_reexport.rs

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
