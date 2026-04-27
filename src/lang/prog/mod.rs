// devela::lang::prog
//
#![doc = crate::_DOC_LANG_PROG!()] // public
#![doc = crate::_doc!(modules: crate::lang; prog: ffi)] //
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//!
//! > Languages that describe computation, behavior, and execution.
//

// mod calc; // Executable semantic calculus WIP
// mod embed; // Host-embedded notation and generator macros WIP
pub mod ffi; // Foreign language interfaces
// mod ir; // Lowered program representations
// mod kernel; // Reusable computational language kernels
// mod phrase;   // Source-level program phrases/forms.
// mod script; // Command and scripting language surfaces
// mod template; // Template and substitution language machinery

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // calc::_all::*,
            // embed::_all::*,
            // ir::_all::*,
            // kernel::_all::*,
            // phrase::_all::*,
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
