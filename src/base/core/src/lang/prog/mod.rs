// devela_base_core::lang::prog
//
#![doc = crate::_DOC_LANG_PROG!()]
#![doc = crate::_doc!(modules: crate::lang; prog: ffi)] // dsl
//!
#![doc = crate::_doc!(extends: ffi)]
//!
//! Languages that describe computation, behavior, and execution.
//

// mod dsl; // embedded or domain-specific languages
// mod ir; // intermediate representations

pub mod ffi; // foreign language interfaces

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     // dls::_all::*,
        //     // ir::_all::*,
        // };
    }
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
}
