// devela_base_core::data::codec::hash
//
#![doc = crate::_DOC_DATA_CODEC_HASH!()]
//

mod check; // Adler32
mod fx; // HasherBuildFx, HasherFx.
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            check::_all::*,
            fx::*,
            reexports::*,
        };
    }
}
