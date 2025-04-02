// devela::sys::mem::alloc
//
//! Memory allocation.
// #![doc = crate::doc_!(extends: alloc)]

mod reexports;
mod namespace;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{namespace::*, reexports::*};
        // WIPZONE
        // #[cfg(feature = "unsafe_layout")]
        // pub use super::mini::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
// #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "unsafe_layout"))))]
// mod mini;
