// devela::data::codec::hash
//
#![doc = crate::_DOC_DATA_CODEC_HASH!()]
//!
#![doc = crate::_doc!(extends: hash)]
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

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{fx::*, reexports::*};

        #[cfg(feature = "hash")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "hash")))]
        pub use super::{fnv::*, pengy::*};
    }
    _always {
        pub use super::{fx::*, reexports::*};
    }
}
