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
//! program: &[ScriptOp]
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
//! Machine values are compact [`Value64`] words. Their [`ValueKind4::Ref`]
//! payloads can carry scalar-packed IDs or generational handles, but
//! the machine never resolves or validates external references.
//!
//! The host owns packing, decoding, and validation. A token may remain
//! stored even after its referenced object has been removed.
//!
//! [`Value64`]: crate::Value64
//! [`ValueKind4::Ref`]: crate::ValueKind4::Ref
//!
//! # Errors
//!
//! A failing operation leaves machine state unchanged.
//! External effects are not rolled back.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod call; // Contextual host-call identities
    mod error; // Script execution errors
    mod host; // Host capability boundary
    mod instr; // Executable script operations
    mod outcome; // Non-error execution outcomes
    // mod program; // Loaded immutable programs
    mod state; // Machine state and execution
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            call::{ScriptCall, ScriptCallId},
            error::ScriptError,
            host::ScriptHost,
            instr::ScriptOp,
            outcome::ScriptOutcome,
            state::ScriptMachine,
        };
    }
}
