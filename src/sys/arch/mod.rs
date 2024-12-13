// devela::sys::arch
//
//! Architecture-specific intrinsics.
//!
#![doc = crate::doc_!(extends: arch)]
#![doc = crate::doc_!(newline)]
//

mod reexports;

#[cfg(feature = "dep_safe_arch")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_safe_arch")))]
mod namespace;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::reexports::*;

        #[cfg(feature = "dep_safe_arch")]
        pub use super::namespace::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
