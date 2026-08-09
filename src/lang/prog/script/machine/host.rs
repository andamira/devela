// devela/src/lang/prog/script/machine/host.rs
//
//!
//

use crate::{ScriptCallId, ScriptValue};
#[cfg(doc)]
use crate::{ScriptMachine, ScriptOutcome};

#[doc = crate::_tags!(lang)]
/// A host capability boundary for resolving scripted operations.
#[doc = crate::_doc_meta!{ location("lang/prog/script") }]
///
/// [`ScriptMachine::step_with`] and [`ScriptMachine::run_with`] resolve
/// host calls through this trait. Direct execution may instead return
/// [`ScriptOutcome::HostCall`] for explicit resolution by the caller.
///
/// Arguments are supplied in stack order, with the earliest argument first.
/// Every successful call returns exactly one machine value; [`ScriptValue::Nil`]
/// represents a call with no meaningful result.
pub trait ScriptHost<R> {
    /// An error produced while performing a host operation.
    type Error;

    /// Performs the host operation identified by `id`.
    ///
    /// Host effects performed before returning an error
    /// are not rolled back by the scripting machine.
    fn call(
        &mut self,
        id: ScriptCallId,
        args: &[ScriptValue<R>],
    ) -> Result<ScriptValue<R>, Self::Error>;
}

impl<R, E, F> ScriptHost<R> for F
where
    F: FnMut(ScriptCallId, &[ScriptValue<R>]) -> Result<ScriptValue<R>, E>,
{
    type Error = E;

    fn call(&mut self, id: ScriptCallId, args: &[ScriptValue<R>]) -> Result<ScriptValue<R>, E> {
        self(id, args)
    }
}
