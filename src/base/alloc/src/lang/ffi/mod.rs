// devela_base_alloc::lang::ffi
//
#![doc = crate::_DOC_LANG_FFI!()]
//

pub mod c;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            c::_all::*,
        };
    }
}
