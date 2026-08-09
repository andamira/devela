// devela/src/lang/prog/script/machine/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT_MACHINE!()] // private
#![doc = crate::_doc!(modules: crate::lang::prog::script; machine)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//!
//! The scripting machine is a small resumable execution kernel.
//!
//! [`ScriptMachine`] owns suspended execution state but neither the program
//! being executed nor the world affected by it. This keeps program storage,
//! execution state, and external authority independent.
//!
//! External interaction is reified as [`ScriptCall`]. Direct execution through
//! [`ScriptMachine::step`] and [`ScriptMachine::run`] returns such interactions
//! to the caller, which may resolve them explicitly and resume execution.
//! [`ScriptHost`] provides an optional adapter that resolves the same protocol
//! through [`ScriptMachine::step_with`] and [`ScriptMachine::run_with`].
//!
//! This allows the same machine semantics to serve statically wired,
//! const-capable simulations as well as dynamically hosted scripting.
//!
//! A failing operation is atomic with respect to machine state. Effects already
//! performed by an external host are outside that guarantee.
//

#[cfg(test)]
mod _test;

mod call; // Contextual host-call identities
mod error; // Script execution errors
mod host; // Host capability boundary
mod instr; // Executable script operations
mod outcome; // Non-error execution outcomes
// mod program; // Loaded immutable programs
mod state; // Machine state and execution
mod value; // Machine-level values

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            call::{ScriptCall, ScriptCallId},
            error::ScriptError,
            host::ScriptHost,
            instr::ScriptOp,
            outcome::ScriptOutcome,
            state::ScriptMachine,
            value::ScriptValue,
        };
    }
}
