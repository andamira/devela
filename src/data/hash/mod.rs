// devela::data::hash
//
//! Generic hashing support.
//!
#![doc = crate::doc_!(extends: hash)]
//

mod reexports;

#[cfg(feature = "hash")]
crate::items! {
    #[cfg(feature = "cast")]
    mod fnv; // HasherBuildFnv, HasherFnv
    mod fx; // HasherBuildFx, HasherFx
    mod pengy; // hash_pengy
}

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::reexports::*;

        #[cfg(feature = "hash")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "hash")))]
        pub use super::{fx::*, pengy::*};

        #[cfg(all(feature = "hash", feature = "cast"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(all(feature = "hash", feature = "cast"))))]
        pub use super::fnv::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
