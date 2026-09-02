// devela/src/lang/prog/_.rs
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

crate::mods_in! {
        // mod_ calc; // Executable semantic calculus WIP
    pub mod_ ffi; // Foreign language interfaces
        // mod_ ir; // Lowered program representations
        // mod_ kernel; // Reusable computational language kernels
        // mod_ phrase;   // Source-level program phrases/forms.
    pub mod_ script; // Scripting machinery and language forms
        // mod_ template; // Template and substitution language machinery
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals
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
        pub(crate) use super::{
            ffi::_crate_internals::*,
        };
    }
}
