// devela::sys::net
//
//! Networking functionality.
//!
#![doc = crate::doc_!(extends: net)]
//

crate::items! {
    mod reexports;
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)] pub use super::_mods::*;
    }
}
