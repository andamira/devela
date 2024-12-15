// devela::sys::env
//
//! Inspection and manipulation of the process’s environment.
//!
#![doc = crate::doc_!(extends: env)]
//

mod reexports;

#[cfg(feature = "std")]
mod env;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;
        #[cfg(feature = "std")]
        pub use super::env::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
