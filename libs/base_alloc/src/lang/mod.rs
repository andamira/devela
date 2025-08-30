// devela_base_alloc::lang
//
#![doc = crate::_DOC_LANG!()]
//

pub mod ffi;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
}
