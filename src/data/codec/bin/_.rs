// devela/src/data/codec/bin/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_BIN!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; bin: bit)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    pub mod_ bit; // Bit-focused items
    // mod endian // MAYBE Be, Le…
    // mod magic; // WIP MagicBytes, signatures, later
    // mod pad; // WIP Serialized padding helpers
    mod tag; // BinTag4
    // mod varint; // WIP LEB128, VLQ, MIDI variable length quantities
}
crate::mods_out! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // endian::*,
            // magic::*,
            // pad::*,
            tag::*,
            // varint::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            bit::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            bit::{bitfield, set},
        };
    }
    _hidden {
        pub use super::bit::_hidden::*;
    }
}
