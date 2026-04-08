// devela::run::time::run
//
//! Defines [`Runtime`].
//

use crate::{RunCycle, RunPhase, RuntimeTick};

#[doc = crate::_tags!(runtime)]
/// A handle to the live execution context of a running system.
#[doc = crate::_doc_location!("run/time")]
///
/// `Runtime` stores the long-lived state that defines an active run:
/// - its committed regime,
/// - its logical tick,
/// - and its lifecycle phase.
///
/// It is a context object, not a scheduler or loop controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Runtime<R> {
    regime: R,
    tick: RuntimeTick,
    cycle: RunCycle,
}

impl<R> Runtime<R> {
    /// Creates a new runtime from the given regime.
    pub const fn new(regime: R) -> Self {
        Self {
            regime,
            tick: RuntimeTick::new(0),
            cycle: RunCycle::new(),
        }
    }

    /// Returns a shared reference to the committed regime.
    pub const fn regime(&self) -> &R {
        &self.regime
    }

    /// Returns a mutable reference to the committed regime.
    pub const fn regime_mut(&mut self) -> &mut R {
        &mut self.regime
    }

    /// Returns the current logical tick.
    pub const fn tick(&self) -> RuntimeTick {
        self.tick
    }

    /// Returns the current run cycle.
    pub const fn cycle(&self) -> RunCycle {
        self.cycle
    }

    /// Returns the current run phase.
    pub const fn phase(&self) -> RunPhase {
        self.cycle.phase()
    }

    /// Returns `true` if logical progression is currently allowed.
    pub const fn can_advance(&self) -> bool {
        self.cycle.can_advance()
    }

    /// Advances the logical tick by one.
    ///
    /// This does not check the current phase.
    pub const fn tick_once(&mut self) {
        self.tick.tick();
    }

    /// Advances the logical tick by `delta`.
    ///
    /// This does not check the current phase.
    pub const fn advance_mut(&mut self, delta: u64) {
        self.tick.advance_mut(delta);
    }

    /// Attempts to transition to `next`.
    ///
    /// Returns `true` if the transition is valid.
    pub const fn transition(&mut self, next: RunPhase) -> bool {
        self.cycle.transition(next)
    }
}
