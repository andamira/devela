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
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
