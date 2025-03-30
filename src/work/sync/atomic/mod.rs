// devela::work::sync::atomic
//
//! Atomic types.
//!
#![doc = crate::doc_!(extends: sync)]
//

mod reexports;

crate::items! { // structural access: _mods, _all,
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::reexports::*;
    }
    pub(super) mod _all {
        pub use super::_mods::*;
    }
}
