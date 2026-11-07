//
//!
//

use crate::{ScriptCallId, Value64};
#[cfg(doc)]
use crate::{ScriptMachine, ScriptOutcome};

#[doc = crate::_tags!(lang)]
/// A host capability boundary for resolving scripted operations.
#[doc = crate::_doc_meta!{
    location("lang/prog/script/machine"),
}]
/// [`ScriptMachine::step_with`] and [`ScriptMachine::run_with`] resolve
/// host calls through this trait. Direct execution may instead return
/// [`ScriptOutcome::HostCall`] for explicit resolution by the caller.
///
/// Opaque reference tokens are passed as [`Value64`] values; the host
/// determines their meaning and validity.
///
/// Arguments are supplied in stack order, with the earliest argument first.
/// Every successful call returns exactly one machine value; [`Value64::NIL`]
/// represents a call with no meaningful result.
pub trait ScriptHost {
    /// An error produced while performing a host operation.
    type Error;

    /// Performs the host operation identified by `id`.
    ///
    /// Host effects performed before returning an error
    /// are not rolled back by the scripting machine.
    fn call(&mut self, id: ScriptCallId, args: &[Value64]) -> Result<Value64, Self::Error>;
}

impl<E, F> ScriptHost for F
where
    F: FnMut(ScriptCallId, &[Value64]) -> Result<Value64, E>,
{
    type Error = E;

    fn call(&mut self, id: ScriptCallId, args: &[Value64]) -> Result<Value64, E> {
        self(id, args)
    }
}
