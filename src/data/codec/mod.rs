// devela::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()]
#![doc = crate::_doc!(modules: crate::data; codec: hash)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: hash)]
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
//! ## Example
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

mod encode; // encoders and decoders.
mod radix; // radix-based encodings (Base32, Base64, Base58…).
mod types;

pub mod crypto; // cryptography
pub mod hash; // hashing algorithms (Fnv, Fx, MD5).

// WIP ZONE
// mod compress; // compression algorithms
// mod _wip_crc;
// mod hex; // Hexadecimal literals and conversions.
// mod rle; // Run-length encoding and similar techniques.
mod serde; // structured serialization/deserialization.
// #[cfg(feature = "alloc")]
// mod lempel_ziv;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{crypto::_all::*, encode::_all::*, radix::_all::*, types::*};
        // WIPZONE
        pub use super::serde::_all::*;
        // pub use serde::lempel_ziv::*;
    }
    _pub_mods {
        pub use super::{
            hash::_all::*,
        };
    }
}
