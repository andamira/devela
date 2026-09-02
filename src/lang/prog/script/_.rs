// devela/src/lang/prog/script/_.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; script: form, machine)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//!
//!
//! Scripting provides changeable behavior inside a host-owned world.
//!
//! The host retains ownership of world structure, resources, timing, and
//! authority. Scripts operate on small machine values and request external
//! effects through explicit capabilities.
//!
//! Source syntax and machine execution remain independent. A language may
//! lower into machine operations without becoming part of the execution model.
//

crate::mods_in! {
        // mod_ embed; // WIP Host-embedded scripting notation and generators
    pub mod_ form; // Scripting and programming language forms
    pub mod_ machine; // Resumable scripting execution machinery
        // mod_ source;   // MAYBE: Script source loading and lowering
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        // pub use super::{
        //     embed::_all::*,
        // };
    }
    _pub_mods {
        pub use super::{
            form::_all::*,
            machine::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::ScriptMachine;
    }
}
