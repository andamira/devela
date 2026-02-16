// devela_base_core::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()] // public
#![doc = crate::_doc!(modules: crate::data; codec: hash)] // crypto
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//

mod bit; // bitfield!
mod radix; // radix-based encodings (Base32, Base64, Base58…).

// pub mod crypto;
// pub mod deser; // WIP
// pub mod frame;
pub mod hash;
// pub mod schema; // WIP

// mod zlib; // WIP

crate::structural_mods! { // _mod, _pub_mods
    _mods {
        pub use super::{
            bit::_all::*,
            radix::*,
            // zlib::*, // WIP
        };
    }
    _pub_mods {
        pub use super::{
            // crypto::_all::*,
            // deser::_all::*, // WIP
            // frame::_all::*,
            hash::_all::*,
            // schema::_all::*, // WIP
        };
    }
}
