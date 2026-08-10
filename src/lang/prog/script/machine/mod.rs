// devela/src/lang/prog/script/machine/mod.rs
//
#![doc = crate::_DOC_LANG_PROG_SCRIPT_MACHINE!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog::script; machine)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//!
//! The scripting machine is a small resumable execution kernel.
//!
//! [`ScriptMachine`] owns suspended execution state but neither the program
//! being executed nor the world affected by it. This keeps program storage,
//! execution state, and external authority independent.

//! ## Execution
//!
//! Direct and hosted execution share the same machine state and transition
//! protocol. They differ only in who resolves external calls.
//!
//! [`ScriptMachine::step`] and [`ScriptMachine::run`] return external
//! interaction as [`ScriptOutcome::HostCall`]. The caller may inspect the
//! contained [`ScriptCall`], perform the requested effect, and resume the
//! machine with [`ScriptMachine::complete_call`].
//!
//! [`ScriptMachine::step_with`] and [`ScriptMachine::run_with`] resolve the
//! same interaction through a [`ScriptHost`] instead.
//!
//! ```txt
//! program: &[ScriptOp<R>]
//!          │
//!          ▼
//!    ScriptMachine
//!          │
//!       step / run
//!          │
//!          ▼
//!    ScriptOutcome
//!      ├─ Yielded
//!      ├─ Returned
//!      ├─ BudgetExhausted
//!      └─ HostCall(call)
//!              │
//!       resolve externally
//!              │
//!              ▼
//!      complete_call(value)
//!              │
//!              └──────► resume
//!
//! step_with / run_with:
//! HostCall ──► ScriptHost::call ──► value ──► resume
//! ```
//!
//! ## Reference payloads
//!
//! `R` is the machine's opaque reference payload type. It is carried by
//! [`ScriptValue::Ref`] but is never interpreted or validated by the machine.
//! Typical payloads are compact IDs or generational handles into caller-owned state.
//!
//! `R` can remain stored after its external target becomes invalid.
//!
//! Direct execution can resolve these references explicitly. Hosted execution
//! passes the same values through [`ScriptHost`].
//!
//! # Errors
//!
//! A failing operation leaves machine state unchanged.
//! External effects are not rolled back.
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
