// devela_base_core::data::codec::hash
//
#![doc = crate::_DOC_DATA_CODEC_HASH!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; hash)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//

mod _reexport; // SYMLINK from /src/data/codec/hash/_reexport_core.rs

mod check; // Adler32
mod fx; // HasherBuildFx, HasherFx.

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            check::_all::*,
            fx::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
