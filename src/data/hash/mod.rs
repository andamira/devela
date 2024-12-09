// devela::data::hash
//
//! Generic hashing support.
//!
#![doc = crate::doc_!(extends: hash)]
//

mod reexports;

#[cfg(feature = "hash")]
crate::items! {
    mod fnv; // HasherBuildFnv, HasherFnv
    mod fx; // HasherBuildFx, HasherFx
    mod pengy; // hash_pengy
}

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(feature = "hash")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "hash")))]
        pub use super::{fnv::*, fx::*, pengy::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
