// devela::data::codec::bin
//
#![doc = crate::_DOC_DATA_CODEC_BIN!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; bin)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod bit; // Bit-focused items
// mod endian // MAYBE Be, Le…
// mod magic; // WIP MagicBytes, signatures, later
// mod pad; // WIP Serialized padding helpers
mod tag; // BinTag4
// mod varint; // WIP LEB128, VLQ, MIDI variable length quantities

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            bit::_all::*,
            // endian::*,
            // magic::*,
            // pad::*,
            tag::*,
            // varint::_all::*,
        };
    }
}
