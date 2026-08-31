// devela/src/data/codec/hash/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_HASH!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; hash)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//

crate::mods_in! {
    mod _reexport_core;
    #[cfg(feature = "std")]
    mod _reexport_std;

    mod fx; // HasherBuildFx, HasherFx

    #[cfg(feature = "hash")]
    mod fnv; // HasherBuildFnv, HasherFnv
    #[cfg(feature = "hash")]
    mod pengy; // hash_pengy
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            fx::{HasherBuildFx, HasherFx},
        };
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
    }
}
