// devela/src/lang/prog/script/machine/outcome.rs
//
//!
//

use crate::{ConstInit, ScriptCall, ScriptValue};
#[cfg(doc)]
use crate::{ScriptMachine, ScriptOp};

#[doc = crate::_tags!(lang result)]
/// The non-error outcome of running a [`ScriptMachine`].
#[doc = crate::_doc_meta!{
    location("lang/prog/script/machine"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: ScriptOutcome<u32> = 16|128; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: ScriptOutcome<u32> = 32|256; niche Option),
}]
/// An outcome explains why control returned to the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScriptOutcome<R> {
    /// Execution reached a host operation awaiting resolution.
    HostCall(ScriptCall),

    /// Execution explicitly suspended with [`ScriptOp::Yield`].
    Yielded,

    /// Execution finished with an optional returned value.
    Returned(Option<ScriptValue<R>>),

    /// Execution reached the caller-provided operation budget.
    BudgetExhausted,
}

impl<R> ConstInit for ScriptOutcome<R> {
    const INIT: Self = Self::Returned(None);
}
