// devela::sys::mem::cell
//
#![doc = crate::_DOC_SYS_MEM_CELL!()]
//!
#![doc = crate::_doc!(extends: cell)]
//

mod option; // CellOptionExt

// WIPZONE
// #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
// mod scell; // define_singleton!
// #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
// mod ghost; // WIP

mod _reexport_core; // NOTE: symlink to /libs/base_core/src/sys/mem/cell/_reexport.rs

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            option::*,
        };
        // WIPZONE
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::scell::*;
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::ghost::*; // WIP
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
