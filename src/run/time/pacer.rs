// devela::run::time::pacer
//
//! Defines [`RunPacer`].
//

use crate::TimeSpan;

#[doc = crate::_tags!(runtime time)]
/// Controls presentation cadence independently of simulation.
#[doc = crate::_doc_location!("run/time")]
///
/// `RunPacer` accumulates elapsed span and decides when presentation is due.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunPacer<T: TimeSpan> {
    interval: T,
    accum: T,
}

impl<T: TimeSpan> RunPacer<T> {
    /// Creates a pacer targeting a fixed presentation interval.
    ///
    /// Returns `None` if `interval` is zero.
    pub fn new(interval: T) -> Option<Self> {
        if interval.time_is_zero() {
            None
        } else {
            Some(Self { interval, accum: T::TIME_ZERO })
        }
    }

    /// Returns the fixed presentation interval.
    pub const fn interval(&self) -> T {
        self.interval
    }

    /// Returns the currently accumulated remainder.
    ///
    /// This is the elapsed span retained after prior interval consumption.
    pub const fn accum(&self) -> T {
        self.accum
    }

    /// Returns `true` if at least one presentation interval became due.
    ///
    /// This is the cheaper path. It consumes at most one interval.
    ///
    /// # Panics
    /// May panic if span arithmetic is not representable.
    pub fn allow(&mut self, dt: T) -> bool {
        self.allow_checked(dt).expect("RunPacer span arithmetic overflowed or underflowed")
    }

    /// Returns `Some(true)` if at least one presentation interval became due.
    ///
    /// This is the cheaper fallible path. It consumes at most one interval.
    pub fn allow_checked(&mut self, dt: T) -> Option<bool> {
        self.accum = self.accum.time_add_checked(dt)?;
        if self.accum >= self.interval {
            self.accum = self.accum.time_sub_checked(self.interval)?;
            Some(true)
        } else {
            Some(false)
        }
    }

    /// Returns how many whole presentation intervals became due.
    ///
    /// This is the fuller path. It repeatedly consumes intervals until the
    /// remainder is less than `interval`.
    ///
    /// # Panics
    /// May panic if span arithmetic or cycle counting is not representable.
    pub fn cycles(&mut self, dt: T) -> u64 {
        self.cycles_checked(dt).expect("RunPacer span arithmetic overflowed or underflowed")
    }

    /// Returns how many whole presentation intervals became due.
    ///
    /// This is the fuller fallible path. It repeatedly consumes intervals until
    /// the remainder is less than `interval`.
    pub fn cycles_checked(&mut self, dt: T) -> Option<u64> {
        self.accum = self.accum.time_add_checked(dt)?;

        let mut cycles = 0_u64;
        while self.accum >= self.interval {
            self.accum = self.accum.time_sub_checked(self.interval)?;
            cycles = cycles.checked_add(1)?;
        }
        Some(cycles)
    }
}
