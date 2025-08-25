// devela::data::codec::hash
//
#![doc = crate::_DOC_DATA_CODEC_HASH!()]
//!
#![doc = crate::doc_!(extends: hash)]
//!
#![cfg_attr(
    not(feature = "hash"),
    doc = "## Features\nTo compile the missing items, enable the `hash` feature."
)]
//

mod fx; // HasherBuildFx, HasherFx (not feature-gated).
mod reexports;

#[cfg(feature = "hash")]
crate::items! {
    mod pengy; // hash_pengy
    mod fnv; // HasherBuildFnv, HasherFnv
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{fx::*, reexports::*};

        #[cfg(feature = "hash")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "hash")))]
        pub use super::{fnv::*, pengy::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{fx::*, reexports::*};
    }
}
