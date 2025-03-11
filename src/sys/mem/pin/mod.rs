// devela::sys::mem::pin
//
//! Types that pin data to a location in memory.
//!
#![doc = crate::doc_!(extends: pin)]
//

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ptr")))]
mod pinned; // Pinned
mod reexports; // ::core::pin::*

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;

        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
        pub use super::pinned::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
