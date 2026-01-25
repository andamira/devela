// devela::lang::prog
//
#![doc = crate::_DOC_LANG_PROG!()]
#![doc = crate::_doc!(modules: crate::lang; prog: ffi)] // dsl, ir
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(br+lf)]
#![doc = crate::_doc!(extends: ffi)]
//!
//! Languages that describe computation, behavior, and execution.
//

mod dsl; // embedded or domain-specific languages
// mod ir; // intermediate representations

pub mod ffi; // foreign language interfaces

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            dsl::_all::*,
            // ir::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::ffi::_crate_internals::*;
    }
}
