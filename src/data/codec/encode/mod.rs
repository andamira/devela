// devela/src/data/codec/encode/mod.rs
//
#![doc = crate::_DOC_DATA_CODEC_ENCODE!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; encode)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

// mod deser; // WIP Format-neutral value (de)serialization
mod enums; // EncodingMode

// WIP
// mod combinator; // CodecIf, CodecJoin, CodecLenValue...
// mod error; // CodecError
// mod endian; // CodecBe, CodecLe
// mod len; // CodecLen
// mod traits; // Encode, Decode, EncodedLen
// mod generator; // MAYBE codec! or encode!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // deser::_all::*,
            enums::*,
        };
    }
}
