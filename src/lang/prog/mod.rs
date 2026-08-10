// devela/src/lang/prog/mod.rs
//
#![doc = crate::_DOC_LANG_PROG!()] // public
#![doc = crate::_doc!(modules: crate::lang; prog: ffi, script)] //
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//!
//! Programming-language support spans executable forms, execution machinery,
//! and boundaries with external languages.
//!
//! Internal execution and foreign interoperability remain separate concerns.
//! Language forms may lower into executable representations without becoming
//! part of their runtime semantics.
//

// mod calc; // Executable semantic calculus WIP
pub mod ffi; // Foreign language interfaces
// mod ir; // Lowered program representations
// mod kernel; // Reusable computational language kernels
// mod phrase;   // Source-level program phrases/forms.
pub mod script; // Scripting machinery and language forms
// mod template; // Template and substitution language machinery

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // calc::_all::*,
            // ir::_all::*,
            // kernel::_all::*,
            // phrase::_all::*,
            // template::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            ffi::_all::*,
            script::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::ffi::_crate_internals::*;
    }
}
