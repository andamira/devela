// devela/src/lang/prog/script/machine/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT_MACHINE!()] // private
#![doc = crate::_doc!(modules: crate::lang::prog::script; machine)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod call; // Host-call identifiers and invocation metadata
// mod error; // Execution errors
// mod host; // Host capability boundary
mod instr; // Executable script operations
mod outcome; // Non-error execution outcomes
// mod program; // Loaded immutable programs
mod state; // Suspended machine state
mod value; // Machine-level values

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            instr::ScriptOp,
            outcome::ScriptOutcome,
            state::ScriptMachine,
            value::ScriptValue,
        };
    }
}
