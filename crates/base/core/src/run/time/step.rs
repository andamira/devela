// devela_base_core::run::time::step
//
//! Defines [`RunStep`].
//

use crate::{RunPhase, RuntimeTick};

#[doc = crate::_tags!(runtime time)]
/// A per-step snapshot passed to app logic.
#[doc = crate::_doc_location!("run/time")]
///
/// `RunStep` is the minimal envelope of one runtime iteration.
///
/// It exposes:
/// - the current logical tick,
/// - the current run phase,
/// - and the events gathered for this step.
///
/// It does not own the runtime, control pacing, or store long-lived state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RunStep<'a, E = (), C = ()> {
    tick: RuntimeTick,
    phase: RunPhase,
    // real_dt: Option<Duration>,
    events: &'a [E],
    context: C,
}

impl<'a, E, C> RunStep<'a, E, C> {
    /// Creates a new step snapshot.
    pub const fn new(tick: RuntimeTick, phase: RunPhase, events: &'a [E], context: C) -> Self {
        Self { tick, phase, events, context }
    }

    /// Returns the logical tick of this step.
    pub const fn tick(&self) -> RuntimeTick {
        self.tick
    }

    /// Returns the run phase of this step.
    pub const fn phase(&self) -> RunPhase {
        self.phase
    }

    /// Returns the events gathered for this step.
    pub const fn events(&self) -> &'a [E] {
        self.events
    }

    /// Returns `true` if this step is in the [`Running`][RunPhase::Running] phase.
    pub const fn is_running(&self) -> bool {
        matches!(self.phase, RunPhase::Running)
    }
}
