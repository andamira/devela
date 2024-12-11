// devela::sys::env
//
//! Inspection and manipulation of the process’s environment.
//!
#![doc = crate::doc_!(extends: env)]
//

mod reexports;

#[cfg(feature = "std")]
mod env;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(feature = "std")]
        pub use super::env::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
