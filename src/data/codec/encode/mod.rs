// devela::data::codec::encode
//
#![doc = crate::_DOC_DATA_CODEC_ENCODE!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; encode)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: encode)]
//

mod impls;
#[cfg(test)]
mod tests;

mod combinators; // Codec*
mod enums; // EncodingMode
mod traits; // Decodable, Encodable, EncodableLen

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            combinators::*,
            enums::*,
            traits::*,
        };
    }
}
