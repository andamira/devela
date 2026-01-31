// devela_base_alloc::lang::prog
//
#![doc = crate::_DOC_LANG_PROG!()] // public
#![doc = crate::_doc!(modules: crate::lang; prog: ffi)] // dsl, ir
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//!
//! Languages that describe computation, behavior, and execution.
//

// mod dsl; // embedded or domain-specific languages
// mod ir; // intermediate representations

pub mod ffi; // foreign language interfaces

crate::structural_mods! { // , _pub_mods
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
}
