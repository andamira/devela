// devela::sys::mem::cell
//
#![doc = crate::_DOC_SYS_MEM_CELL!()]
//!
#![doc = crate::_doc!(extends: cell)]
//

mod option; // ExtCellOption
mod reexports; // ::core::cell::*

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{option::*, reexports::*};
        // WIPZONE
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::scell::*;
        // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
        // pub use super::ghost::*; // WIP
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
// WIPZONE
// #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
// mod scell; // define_singleton!
// #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
// mod ghost; // WIP
