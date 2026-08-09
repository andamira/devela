// devela/src/lang/prog/script/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; script: form)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod embed; // WIP Host-embedded scripting notation and generators
pub mod form; // Scripting and programming language forms
mod machine; // Hosted scripting execution machinery
// mod source;   // MAYBE: Script source loading and lowering

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            // embed::_all::*,
            machine::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            form::_all::*,
        };
    }
}
