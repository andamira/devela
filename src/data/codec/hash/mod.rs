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

#[cfg(feature = "hash")]
crate::items! {
    mod pengy; // hash_pengy
    mod fnv; // HasherBuildFnv, HasherFnv
}

// re-exports
crate::mod_path!(_c "../../../../libs/base_core/src/data/codec/hash/reexports.rs");
crate::mod_path!(std _s "../../../../libs/base_std/src/data/codec/hash/reexports.rs");

crate::structural_mods! { // _mods
    _mods {

        #[cfg(feature = "hash")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "hash")))]
        pub use super::{fnv::*, pengy::*};

        // re-exports
        pub use super::_c::*;
        #[cfg(feature = "std")]
        pub use super::_s::*;
        pub use devela_base_core::data::codec::hash::{
            Adler32, HasherBuildFx, HasherFx,
        };
    }
}
