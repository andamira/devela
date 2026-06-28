// devela/src/num/signal/phase.rs
//
//!
//

use crate::SignalNext;

#[doc = crate::_tags!(num signal wave primitive)]
/// A normalized cycle phase represented as wrapping `u32`.
#[repr(transparent)]
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Phase(pub u32);

#[rustfmt::skip]
impl Phase {
    /// The beginning of the cycle, equal to `0` turns.
    pub const ZERO: Self = Self(0);
    /// One quarter of a cycle, equal to `1/4` turn.
    pub const QUARTER: Self = Self(0x4000_0000);
    /// One half of a cycle, equal to `1/2` turn.
    pub const HALF: Self = Self(0x8000_0000);
    /// Three quarters of a cycle, equal to `3/4` turn.
    pub const THREE_QUARTERS: Self = Self(0xC000_0000);

    /// Creates a phase from its raw wrapping representation.
    pub const fn new(raw: u32) -> Self { Self(raw) }
    /// Returns the raw wrapping representation.
    pub const fn raw(self) -> u32 { self.0 }

    /// Advances this phase by `step`, wrapping at the end of the cycle.
    pub const fn advance(self, step: PhaseStep) -> Self {
        Self(self.0.wrapping_add(step.0))
    }

    /// Returns this phase as normalized `f32` turns in `[0, 1)`.
    pub const fn as_f32(self) -> f32 { self.0 as f32 * (1.0 / (u32::MAX as f32 + 1.0)) }
    /// Returns this phase as normalized `f64` turns in `[0, 1)`.
    pub const fn as_f64(self) -> f64 { self.0 as f64 * (1.0 / (u32::MAX as f64 + 1.0)) }
}

#[doc = crate::_tags!(num signal wave primitive)]
/// A wrapping phase increment.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhaseStep(pub u32);

#[rustfmt::skip]
impl PhaseStep {
    /// A zero phase increment.
    pub const ZERO: Self = Self(0);

    /// Creates a phase step from its raw wrapping representation.
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }
    /// Returns the raw wrapping representation.
    pub const fn raw(self) -> u32 {
        self.0
    }
}

#[doc = crate::_tags!(num signal wave)]
/// A stateful phase accumulator.
#[doc = crate::_doc_meta!{
    location("num/signal"),
    test_size_of(PhaseAccum = 8|64),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhaseAccum {
    /// The current phase.
    pub phase: Phase,
    /// The phase increment applied at each step.
    pub step: PhaseStep,
}
impl PhaseAccum {
    /// Creates a phase accumulator from an initial `phase` and a `step`.
    pub const fn new(phase: Phase, step: PhaseStep) -> Self {
        Self { phase, step }
    }
    /// Returns the current phase, then advances by one step.
    pub const fn next_phase(&mut self) -> Phase {
        let phase = self.phase;
        self.phase = self.phase.advance(self.step);
        phase
    }
    /// Advances by one step, then returns the new phase.
    pub const fn advance_phase(&mut self) -> Phase {
        self.phase = self.phase.advance(self.step);
        self.phase
    }
}

impl SignalNext for PhaseAccum {
    type Sample = Phase;
    fn next(&mut self) -> Phase {
        self.next_phase()
    }
}
