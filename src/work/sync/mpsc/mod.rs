// devela::work::sync::mpsc
//
#![doc = crate::_DOC_WORK_SYNC_MPSC!()]
// #![doc = crate::_doc!(extends: mpsc)] // IMPROVE

crate::mod_path!(std _s "../../../../libs/base_std/src/work/sync/mpsc/reexports.rs");

mod namespace; // Mpsc

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::namespace::*;
        #[cfg(feature = "std")]
        pub use super::_s::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::_s::*;
    }
}
