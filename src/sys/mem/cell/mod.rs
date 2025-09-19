// devela::sys::mem::cell
//
#![doc = crate::_DOC_SYS_MEM_CELL!()]
//!
#![doc = crate::_doc!(extends: cell)]
//

crate::mod_path!(_c "../../../../libs/base_core/src/sys/mem/cell/reexports.rs");

mod option; // ExtCellOption

// WIPZONE
// #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
// mod scell; // define_singleton!
// #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
// mod ghost; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{option::*, _c::*};
        // WIPZONE
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::scell::*;
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::ghost::*; // WIP
    }
}
