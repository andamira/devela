// devela_base_core::run::time::frame
//
//! Defines [`RunFrame`].
//

use crate::{RunPhase, RunStep, RuntimeTick};

#[doc = crate::_tags!(runtime time)]
/// A per-frame snapshot passed to rendering or other backend-facing work.
#[doc = crate::_doc_location!("run/time")]
///
/// `RunFrame` pairs the logical step snapshot with short-lived backend context.
///
/// It exposes:
/// - the current logical tick,
/// - the current run phase,
/// - the events gathered for this step,
/// - and an ephemeral context value tied to this frame.
///
/// The context is intentionally *per-frame*:
/// - it may borrow backend state,
/// - it may expose host handles or drawing surfaces,
/// - but it should not own the runtime or become a long-lived service container.
///
/// `RunFrame` is the rendering-facing sibling of [`RunStep`].
/// Use [`RunStep`] for pure logical progression, and `RunFrame` when step
/// information must be paired with backend access.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RunFrame<'a, E = (), C = ()> {
    step: RunStep<'a, E>,
    context: C,
}

impl<'a, E, C> RunFrame<'a, E, C> {
    /// Creates a new frame from an existing logical step and a backend context.
    pub const fn new(step: RunStep<'a, E>, context: C) -> Self {
        Self { step, context }
    }

    /// Creates a new frame directly from its logical parts and backend context.
    pub const fn from_parts(
        tick: RuntimeTick,
        phase: RunPhase,
        events: &'a [E],
        context: C,
    ) -> Self {
        Self { step: RunStep::new(tick, phase, events), context }
    }

    /// Returns the underlying logical step snapshot.
    #[must_use]
    pub fn step(&self) -> &RunStep<'a, E> {
        &self.step
    }

    /// Returns the logical tick of this frame.
    pub const fn tick(&self) -> RuntimeTick {
        self.step.tick()
    }

    /// Returns the run phase of this frame.
    pub const fn phase(&self) -> RunPhase {
        self.step.phase()
    }

    /// Returns the events gathered for this frame.
    pub const fn events(&self) -> &'a [E] {
        self.step.events()
    }

    /// Returns `true` if this frame is in the running phase.
    pub const fn is_running(&self) -> bool {
        self.step.is_running()
    }

    /// Returns a shared reference to the per-frame backend context.
    #[must_use]
    pub fn context(&self) -> &C {
        &self.context
    }

    /// Returns an exclusive reference to the per-frame backend context.
    #[must_use]
    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }

    /// Splits the frame into its logical step and backend context.
    #[must_use]
    pub fn into_parts(self) -> (RunStep<'a, E>, C) {
        (self.step, self.context)
    }

    /// Maps the backend context while preserving the logical step.
    #[must_use]
    pub fn map_context<T, F: FnOnce(C) -> T>(self, f: F) -> RunFrame<'a, E, T> {
        let (step, context) = self.into_parts();
        RunFrame { step, context: f(context) }
    }
}
