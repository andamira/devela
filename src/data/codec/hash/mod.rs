// devela::data::codec::hash
//
#![doc = crate::_DOC_DATA_CODEC_HASH!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; hash)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/data/codec/hash/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /crates/base/std/src/data/codec/hash/_reexport.rs

#[cfg(feature = "hash")]
crate::items! {
    mod fnv; // HasherBuildFnv, HasherFnv
    mod pengy; // hash_pengy
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "hash")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "hash")))]
        pub use super::{
            fnv::*,
            pengy::*,
        };
    }
    _reexports{
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
        pub use devela_base_core::data::codec::hash::{
            Adler32, HasherBuildFx, HasherFx,
        };
    }
}
