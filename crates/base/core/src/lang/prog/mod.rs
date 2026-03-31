// devela_base_core::lang::prog
//
#![doc = crate::_DOC_LANG_PROG!()] // public
#![doc = crate::_doc!(modules: crate::lang; prog: ffi)] // dsl, ir
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//!
//! Languages that describe computation, behavior, and execution.
//

// mod dsl; // Embedded and domain-specific languages
pub mod ffi; // foreign language interfaces
// mod ir; // Intermediate representations
// mod script; // Command and scripting languages
// mod template; // Template and substitution languages

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     // dls::_all::*,
        //     // ir::_all::*,
        //     // script::_all::*,
        //     // template::_all::*,
        // };
    }
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
}
