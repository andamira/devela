// devela/src/data/codec/_.rs
//
#![doc = crate::_DOC_DATA_CODEC!()] // public
#![doc = crate::_doc!(modules: crate::data; codec: bin, crypto, hash, integrity, pack, symbol)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//!
//! Codecs act on the representation of data.
//!
//! Some transformations are reversible encodings and decodings.
//! Others derive fingerprints, integrity values, authenticated forms,
//! symbolic forms, or packed structures without preserving a direct inverse.
//

crate::mods_in! {
    pub mod_ bin; // Binary representation atoms
    pub mod_ crypto; // Cryptographic primitives
    // mod_ detect; // WIP Format detection
    mod_ encode; // Composable codecs for reading and writing values
    // pub mod_ frame; // WIP Framing codecs for bounded byte sequences
    pub mod_ integrity; // Integrity codecs (Adler32, Crc32…)
    pub mod_ hash; // Hashing algorithms (Fnv, Fx…)
    pub mod_ pack; // Packed data representations
    mod_ radix; // Radix-based encodings (Base32, Base64, Base58…)
    pub mod_ symbol; // Symbolic codes that encode data into visual marks
}

crate::mods_out! { // _mods, _pub_mods, _reexports, _crate_internals, _hidden
    _mods {
        pub use super::{
            // detect::_all::*,
            encode::_all::*,
            radix::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            bin::_all::*,
            crypto::_all::*,
            // frame::_all::*,
            hash::_all::*,
            integrity::_all::*,
            pack::_all::*,
            symbol::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            bin::{bitfield, set},
            hash::HasherFx,
            integrity::Crc,
        };
    }
    _crate_internals {
        pub(crate) use super::crypto::_crate_internals::*;
    }
    _hidden {
        pub use super::{
            bin::_hidden::*,
            crypto::_hidden::*,
        };
    }
}
