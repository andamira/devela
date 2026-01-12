// devela_base_core::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()]
//

mod radix; // radix-based encodings (Base32, Base64, Base58…).

// pub mod frame;
pub mod hash;
// pub mod zlib; // WIP
// pub mod serde; // WIP

crate::structural_mods! { // _mod, _pub_mods
    _mods {
        pub use super::radix::*;
    }
    _pub_mods {
        pub use super::{
            // frame::_all::*,
            hash::_all::*,
            // zlib::*, // WIP
            // serde::_all::*, // WIP
        };
    }
}
