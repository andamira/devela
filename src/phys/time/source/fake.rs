// devela::phys::time::source::fake
//
//! Defines [`TimeFake`], [`TimeFakeRef`].
//

#![allow(dead_code, unused_variables)]

use crate::{AtomicOrdering, AtomicU64, TimeScale, TimeSourceCfg};

#[doc = crate::_tags!(time fake)]
/// A test-friendly time source that allows manual control.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// `TimeFake` provides a controlled, adjustable timestamp source for tests.
/// This enables predictable behavior when testing time-dependent systems.
///
/// Although TimeFake is commonly advanced monotonically, it allows arbitrary
/// time jumps and therefore does not provide a monotonic timeline guarantee.
///
/// This form is ideal for:
/// - tests that need explicit control
/// - deterministic state machines
/// - simulations
///
/// # Example:
/// ```
/// # use devela::{TimeFake, TimeScale};
/// // Create an independent fake clock starting at t = 100 ms
/// let clock = TimeFake::new(100, TimeScale::Millis);
///
/// // Inspect the current fake time
/// assert_eq!(clock.get_time(), 100);
///
/// // Advance time explicitly
/// clock.advance_time(25);
/// assert_eq!(clock.get_time(), 125);
///
/// // Set time to an exact value
/// clock.set_time(1_000);
/// assert_eq!(clock.get_time(), 1_000);
/// ```
#[derive(Debug, Default)]
pub struct TimeFake {
    now: AtomicU64,
    scale: TimeScale,
}
impl TimeFake {
    /// Creates a new `TimeFake` with the given starting fake time and scale.
    pub const fn new(start: u64, scale: TimeScale) -> Self {
        Self { now: AtomicU64::new(start), scale }
    }
    /// Creates a new `TimeFake` with the given starting fake time in milliseconds.
    pub const fn new_millis(start: u64) -> Self {
        Self {
            now: AtomicU64::new(start),
            scale: TimeScale::Millis,
        }
    }

    /// Returns the time scale.
    pub fn time_scale(&self) -> TimeScale {
        self.scale
    }

    /// Gets the time value.
    pub fn get_time(&self) -> u64 {
        self.now.load(AtomicOrdering::SeqCst)
    }

    /// Manually sets the time to a specific value.
    pub fn set_time(&self, value: u64) {
        self.now.store(value, AtomicOrdering::SeqCst);
    }

    /// Advances the fake time by a given amount.
    pub fn advance_time(&self, delta: u64) {
        self.now.fetch_add(delta, AtomicOrdering::SeqCst);
    }

    /// Gets the current time value in milliseconds.
    pub fn get_millis(&self) -> u64 {
        self.scale.convert_simulated(self.get_time(), TimeScale::Milliseconds)
    }

    /// Sets the current time to the given value in milliseconds.
    pub fn set_millis(&self, time: u64) {
        let value = TimeScale::Milliseconds.convert_simulated(time, self.scale);
        self.set_time(value);
    }

    /// Advances the current time by the given number of milliseconds.
    pub fn advance_millis(&self, delta: u64) {
        let value = TimeScale::Milliseconds.convert_simulated(delta, self.scale);
        self.advance_time(value);
    }

    /// Returns a borrowed view of this fake time source.
    pub fn view(&self) -> TimeFakeRef<'_> {
        TimeFakeRef { src: self }
    }
}

#[doc = crate::_tags!(time fake)]
/// A borrowed configuration handle selecting a specific `TimeFake` timeline.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// This type does not own time state; it exists solely to satisfy
/// [`TimeSourceCfg`] and model timeline selection explicitly.
///
/// This form is ideal for:
/// - code generic over `TimeSourceCfg`
/// - schedulers and timers
/// - components that must not own the clock
///
/// # Example
/// ```
/// # use devela::{TimeFake, TimeFakeRef, TimeSourceCfg};
/// // Create a fake clock
/// let clock = TimeFake::new_millis(500);
///
/// // Obtain a configuration handle selecting this timeline
/// let view = clock.view();
///
/// // Read time through the generic time-source interface
/// let t1 = <TimeFakeRef as TimeSourceCfg>::time_now_millis(view);
/// // or just:
/// let t1 = TimeFakeRef::time_now_millis(view);
/// // or:
/// let t1 = view.now_millis();
///
/// assert_eq!(t1, 500);
///
/// // Advance the underlying clock
/// clock.advance_time(200);
///
/// // Observe the updated time through the same config
/// let t2 = TimeFakeRef::time_now_millis(view);
/// assert_eq!(t2, 700);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TimeFakeRef<'a> {
    src: &'a TimeFake,
}

impl<'a> TimeFakeRef<'a> {
    /// Returns the current time snapshot in milliseconds.
    ///
    /// Equivalent to calling `TimeSourceCfg::time_now_millis`.
    pub fn now_millis(self) -> u64 {
        <Self as TimeSourceCfg>::time_now_millis(self)
    }
}

/// The trait is implemented for `TimeFakeRef` rather than `TimeFake` itself
/// because `TimeSourceCfg::Config` represents a *selection of a timeline*,
/// not ownership of the time source. This allows multiple independent fake
/// clocks to coexist without relying on global state.
#[rustfmt::skip]
impl<'a> TimeSourceCfg for TimeFakeRef<'a> {
    type Config = Self;
    fn time_is_monotonic(_: Self::Config) -> bool { false }
    fn time_is_absolute(_: Self::Config) -> bool { false }
    fn time_scale(cfg: Self::Config) -> TimeScale { cfg.src.scale }
    fn time_now_millis(cfg: Self) -> u64 {
        cfg.src.scale.convert_simulated(cfg.src.get_time(), TimeScale::Millis)
    }
    fn time_now_micros(cfg: Self) -> u64 {
        cfg.src.scale.convert_simulated(cfg.src.get_time(), TimeScale::Micros)
    }
    fn time_now_nanos(cfg: Self) -> u64 {
        cfg.src.scale.convert_simulated(cfg.src.get_time(), TimeScale::Nanos)
    }
}
