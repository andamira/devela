// devela::data::hash
//
//! Generic hashing support.
//!
#![doc = crate::doc_!(extends: hash)]
//

mod fnv; // HasherBuildFnv, HasherFnv
mod fx; // HasherBuildFx, HasherFx
mod pengy; // hash_pengy
mod reexports;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{fnv::*, fx::*, pengy::*, reexports::*};
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
