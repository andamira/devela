// devela::data::codec
//
//! Encoding/Decoding data into compact, transportable or interoperable formats.
#![doc = crate::doc_!(modules: crate::data; codec: hash)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: hash)]
//

mod bit; // bitfield handling and binary transformations.
mod radix; // radix-based encodings (Base32, Base64, Base58…).
mod serde; // structured serialization/deserialization.
mod types;

pub mod hash; // Hashing algorithms (Fnv, Fx, MD5).

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods { #![allow(unused)]
        pub use super::{bit::_all::*, radix::_all::*, serde::_all::*, types::*};
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            hash::_all::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{_mods::*, _pub_mods::*};
        pub use super::hash::_always::*;
    }
}
