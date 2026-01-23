// devela_base_alloc::lang::prog::ffi
//
#![doc = crate::_DOC_LANG_PROG_FFI!()]
#![doc = crate::_doc!(modules: crate::lang::prog; ffi: c)]
//!
#![doc = crate::_doc!(extends: ffi)]
//

pub mod c;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            c::_all::*,
        };
    }
}
