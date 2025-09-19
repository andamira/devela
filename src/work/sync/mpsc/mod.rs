// devela::work::sync::mpsc
//
#![doc = crate::_DOC_WORK_SYNC_MPSC!()]
// #![doc = crate::_doc!(extends: mpsc)] // IMPROVE

crate::mod_path!(std _s "../../../../libs/base_std/src/work/sync/mpsc/reexports.rs");

mod namespace; // Mpsc

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
        #[cfg(feature = "std")]
        pub use super::_s::*;
    }
}
