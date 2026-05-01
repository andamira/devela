// devela::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()] // public
#![doc = crate::_doc!(modules: crate::data; codec: hash)] // crypto
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
#![doc = crate::_QUO_DATA_CODEC!()]
//!
//! ## Determinism & Side Effects
//! Encoding and decoding should be **deterministic**.
//! Implementations should avoid introducing side effects where possible.
//!
//! **Potential sources of non-determinism:**
//! - Writing to or reading from external files or devices.
//! - Using randomness during encoding or decoding.
//! - Modifying or depending on global state.
//!
//! ## Examples
//! ```
//! use devela::{Encodable, CodecLenValue, IoWrite};
//!
//! # #[cfg(feature = "alloc")] { use devela::Vec;
//! let mut buf = Vec::new();
//! CodecLenValue::<_, u8>::new("hello").encode(&mut buf).unwrap();
//! assert_eq!(&buf, b"\x05hello");
//! # }
//! ```
//

mod bit;
mod encode; // encoders and decoders.
mod radix; // radix-based encodings (Base32, Base64, Base58…).
mod types;

// mod compress; // WIP compression algorithms
pub mod crypto; // Cryptographic primitives (Digest, Sha1)
// mod _wip_crc; // WIP
// pub mod frame; // WIP
pub mod hash; // hashing algorithms (Fnv, Fx, MD5)
// mod hex; // WIP Hexadecimal literals and conversions
// mod rle; // WIP Run-length encoding and similar techniques
mod deser; // WIP structured serialization/deserialization
// pub mod schema; // WIP

// #[cfg(feature = "alloc")]
// mod lempel_ziv; // WIP

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            bit::_all::*,
            // deser::_all::*,
            encode::_all::*,
            radix::_all::*,
            types::*,
        };
        // pub use super::lempel_ziv::*;
    }
    _pub_mods {
        pub use super::{
            crypto::_all::*,
            // frame::_all::*,
            hash::_all::*,
            // schema::_all::*,
        };
    }
}
