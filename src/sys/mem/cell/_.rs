// devela/src/sys/mem/cell/mod.rs
//
#![doc = crate::_DOC_SYS_MEM_CELL!()] // pulic
#![doc = crate::_doc!(modules: crate::sys::mem; cell)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: cell)]
//

crate::mods_in! {
    mod _reexport_core;

    mod_ hedge; // MemHedgeCtrl, MemHedgeError, MemHedgeRead, MemHedgeState
    mod option; // CellOptionExt

    // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
    // mod_ ghost; // WIP
    // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
    // mod_ scell; // singleton!
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            hedge::_all::*,
            option::CellOptionExt,
        };
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::ghost::_all::*; // WIP
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::scell::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
