// devela::phys::time::tick
//
//! Defines [`TimeTick`].
//

use crate::Ordering;

#[doc = crate::_TAG_TIME!()]
/// A deterministic logical time counter.
///
/// `TimeTick` represents time as an explicitly advanced,
/// monotonically increasing tick count.
///
/// It is **constructed**, not observed: ticks are advanced explicitly by
/// program logic rather than sampled from external clocks.
///
/// ## Semantics
/// - The origin (`0`) is synthetic and application-defined.
/// - Ticks have no inherent unit; their meaning is determined by context
///   (frames, cycles, steps, events, etc.).
/// - Advancement is explicit and deterministic.
///
/// ## Intended use
/// - simulation and game loops
/// - schedulers and state machines
/// - compile-time–friendly time tracking
/// - deterministic testing and replay
///
/// ## Non-goals
/// - querying wall-clock or system time
/// - representing civil or absolute time
/// - implicit progression
///
/// For sampled or system-backed time, use a [`TimeSource`][crate::TimeSource] instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeTick {
    ticks: u64,
}

impl TimeTick {
    /// Creates a new logical time value with the given tick count.
    pub const fn new(ticks: u64) -> Self {
        Self { ticks }
    }

    /// Returns the underlying tick count.
    pub const fn ticks(self) -> u64 {
        self.ticks
    }

    /// Advances by one tick in place.
    pub const fn tick(&mut self) {
        self.ticks += 1;
    }

    /// Returns the next tick value.
    pub const fn next(self) -> Self {
        Self { ticks: self.ticks + 1 }
    }

    /// Advances this time by `delta` ticks in place.
    pub const fn advance_mut(&mut self, delta: u64) {
        self.ticks += delta;
    }

    /// Returns a new `TimeTick` advanced by `delta` ticks.
    pub const fn advanced(self, delta: u64) -> Self {
        Self { ticks: self.ticks + delta }
    }

    /// Returns the difference in ticks between two times.
    ///
    /// The result is saturating and never negative.
    pub const fn delta(self, earlier: Self) -> u64 {
        self.ticks.saturating_sub(earlier.ticks)
    }

    /// Returns whether this time is strictly after `other`.
    pub const fn is_after(self, other: Self) -> bool {
        self.ticks > other.ticks
    }

    /// Returns whether this time is strictly before `other`.
    pub const fn is_before(self, other: Self) -> bool {
        self.ticks < other.ticks
    }

    /// Returns whether this time is equal to `other`.
    pub const fn eq(self, other: Self) -> bool {
        self.ticks == other.ticks
    }

    /// Compares two tick times.
    pub const fn cmp(self, other: Self) -> core::cmp::Ordering {
        if self.ticks < other.ticks {
            Ordering::Less
        } else if self.ticks > other.ticks {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
