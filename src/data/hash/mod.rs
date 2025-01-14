// devela::data::hash
//
//! Generic hashing support.
//!
#![doc = crate::doc_!(extends: hash)]
//

mod reexports;

#[cfg(feature = "hash")]
crate::items! {
    mod fx; // HasherBuildFx, HasherFx
    mod pengy; // hash_pengy

    #[cfg(feature = "cast")]
    mod fnv; // HasherBuildFnv, HasherFnv
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;

        #[cfg(feature = "hash")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "hash")))]
        pub use super::{fx::*, pengy::*};

        #[cfg(all(feature = "hash", feature = "cast"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(all(feature = "hash", feature = "cast"))))]
        pub use super::fnv::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
