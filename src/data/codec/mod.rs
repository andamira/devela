// devela/src/data/codec/mod.rs
//
#![doc = crate::_DOC_DATA_CODEC!()] // public
#![doc = crate::_doc!(modules: crate::data; codec: bin, crypto, hash)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//!
//! Some transformations are reversible encodings and decodings. Others derive
//! fingerprints, integrity values, authenticated forms, or packed structures
//! without preserving a direct inverse.
//!
//! Codecs do not determine what a value means, where it is stored, or which
//! identity it carries. They operate on the representation crossing those
//! boundaries.
//!
//! - [`Cryptography`](crypto) provides cryptographic transforms,
//!   authentication mechanisms, and secure digests.
//! - [`Hashing`](hash) derives compact fingerprints from data.
//! - Binary atoms, general encoders, integrity checks, packed forms,
//!   and radix encodings are re-exported directly from this module.
//

pub mod bin; // Binary representation atoms.
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
            bin::_all::*,
            crypto::_all::*,
            // frame::_all::*,
            hash::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            bin::bitfield,
            hash::HasherFx,
        };
    }
    _crate_internals {
        pub(crate) use super::crypto::_crate_internals::*;
    }
    _hidden {
        pub use super::crypto::_hidden::*;
    }
}
