// devela::lang::prog
//
#![doc = crate::_DOC_LANG_PROG!()] // public
#![doc = crate::_doc!(modules: crate::lang; prog: ffi)] // dsl, ir
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//!
//! > Languages that describe computation, behavior, and execution.
//

// mod calc; // Executable semantic calculus WIP
mod dsl; // Embedded or domain-specific languages WIP
pub mod ffi; // Foreign language interfaces
// mod ir; // Intermediate representations
// mod script; // Command and scripting languages
// mod template; // Template and substitution languages

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // calc::_all::*,
            dsl::_all::*,
            // ir::_all::*,
            // script::_all::*,
            // template::_all::*,
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
