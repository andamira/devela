// devela_base_core::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()]
//

// pub mod frame;
pub mod hash;
// pub mod zlib; // codec WIP

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            // frame::_all::*,
            hash::_all::*,
            // zlib::*, // WIP
        };
    }
}
