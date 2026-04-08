// devela::run::cycle::cycle
//
//! Defines [`RunControl`], [`RunPhase`], [`RunCycle`].
//

#[doc = crate::_tags!(runtime)]
/// Control returned by a runtime step.
#[doc = crate::_doc_location!("run/cycle")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunControl {
    /// Continue running.
    Continue,
    /// Stop running.
    Stop,
}

#[doc = crate::_tags!(runtime)]
/// High-level phases of a running system.
#[doc = crate::_doc_location!("run/cycle")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RunPhase {
    /// The run exists but has not started normal progression yet.
    ///
    /// This is the default phase.
    #[default]
    Init = 0,
    /// The run is actively progressing.
    Running,
    /// The run exists but progression is temporarily suspended.
    Paused,
    /// The run has terminated and no longer progresses.
    Stopped,
}
impl RunPhase {
    /// Compile-time equality comparison.
    #[must_use]
    pub const fn eq(self, other: Self) -> bool {
        self as u8 == other as u8
    }
}

#[doc = crate::_tags!(runtime)]
/// Manages run phases and guards phase transitions.
#[doc = crate::_doc_location!("run/cycle")]
///
/// `RunCycle` tracks the high-level lifecycle of a running system.
///
/// It is intentionally small:
/// - it does not advance time,
/// - it does not schedule work,
/// - it does not own runtime services.
///
/// It only answers what phase the run is in and whether a transition is valid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RunCycle {
    phase: RunPhase,
}

impl RunCycle {
    /// Creates a new cycle in the [`Init`][RunPhase::Init] phase.
    pub const fn new() -> Self {
        Self { phase: RunPhase::Init }
    }

    /// Returns the current phase.
    pub const fn phase(self) -> RunPhase {
        self.phase
    }

    /// Returns `true` if the run is in the [`Running`][RunPhase::Running] phase.
    pub const fn is_running(self) -> bool {
        matches!(self.phase, RunPhase::Running)
    }

    /// Returns `true` if the run is in the [`Paused`][RunPhase::Paused] phase.
    pub const fn is_paused(self) -> bool {
        matches!(self.phase, RunPhase::Paused)
    }

    /// Returns `true` if the run is in the [`Stopped`][RunPhase::Stopped] phase.
    pub const fn is_stopped(self) -> bool {
        matches!(self.phase, RunPhase::Stopped)
    }

    /// Returns `true` if logical progression is currently allowed.
    pub const fn can_advance(self) -> bool {
        matches!(self.phase, RunPhase::Running)
    }

    /// Returns whether a transition from the current phase to `next` is valid.
    #[allow(clippy::unnested_or_patterns)]
    pub const fn can_transition(self, next: RunPhase) -> bool {
        use RunPhase as P;
        matches!(
            (self.phase, next),
            (P::Init, P::Init | P::Running | P::Stopped)
                | (P::Running, P::Running | P::Paused | P::Stopped)
                | (P::Paused, P::Paused | P::Running | P::Stopped)
                | (P::Stopped, P::Stopped)
        )
    }

    /// Transitions to `next` if valid.
    ///
    /// Returns `true` if the transition is valid.
    pub const fn transition(&mut self, next: RunPhase) -> bool {
        if self.can_transition(next) {
            self.phase = next;
            true
        } else {
            false
        }
    }
}
