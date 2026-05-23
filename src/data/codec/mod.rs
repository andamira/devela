// devela::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()] // public
#![doc = crate::_doc!(modules: crate::data; codec: crypto, hash)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
#![doc = crate::_QUO_DATA_CODEC!()]
//

mod bin; // Binary representation atoms.
pub mod crypto; // Cryptographic primitives
// mod detect; // WIP Format detection
mod encode; // Composable codecs for reading and writing values
// pub mod frame; // WIP Framing codecs for bounded byte sequences
pub mod hash; // Hashing algorithms (Fnv, Fx…)
mod integrity; // Integrity codecs (Adler32, Crc32…)
mod pack; // Packed data representations
mod radix; // Radix-based encodings (Base32, Base64, Base58…)
// mod symbol; // WIP Symbolic codes that encode data into visual marks.

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            bin::_all::*,
            // detect::_all::*,
            encode::_all::*,
            integrity::_all::*,
            pack::_all::*,
            radix::_all::*,
            // symbol::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            crypto::_all::*,
            // frame::_all::*,
            hash::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::crypto::_crate_internals::*;
    }
    _hidden {
        pub use super::crypto::_hidden::*;
    }
}
