// devela_base_core::lang::prog::ffi
//
#![doc = crate::_DOC_LANG_PROG_FFI!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; ffi: c)] // glsl, js
#![doc = crate::_doc!(flat:"lang")]
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
