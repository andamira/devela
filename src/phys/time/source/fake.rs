// devela::phys::time::source::fake
//
//! Defines [`TimeSourceFake`].
//

#![allow(dead_code, unused_variables)]

use crate::{_TAG_FAKE, AtomicOrdering, AtomicU64, TimeScale, TimeSource};

/// Global test time source for convenience.
#[doc = _TAG_FAKE!()]
pub(crate) static TIME_SOURCE_FAKE: TimeSourceFake = TimeSourceFake::new(1_700_000_000_000);

#[doc = _TAG_FAKE!()]
/// A test-friendly time source that allows manual control.
///
/// `TimeSourceFake` provides a controlled, adjustable timestamp source for tests.
/// This enables predictable behavior when testing time-dependent systems.
///
/// # Features:
/// - **Manually set the time** with `set_time()`.
/// - **Manually advance time** with `advance_time()`.
/// - **Implements `TimeSource`**, so it works seamlessly in tests.
///
/// # Example:
/// ```
/// # use devela::TimeSourceFake;
/// let ts = TimeSourceFake::new(1_700_000_000_000);
/// assert_eq!(ts.now_millis(), 1_700_000_000_000);
/// ts.advance_time(1000);
/// assert_eq!(ts.now_millis(), 1_700_000_001_000);
/// ```
pub(crate) struct TimeSourceFake {
    /// Atomic for safe multi-threaded testing
    now: AtomicU64,
}
impl TimeSourceFake {
    /// Creates a new `TimeSourceFake` with the given starting fake time (in milliseconds).
    pub const fn new(start_time: u64) -> Self {
        Self { now: AtomicU64::new(start_time) }
    }
    /// Manually sets the fake time to a specific value (in milliseconds).
    pub fn get_time(&self, new_time: u64) {
        self.now.load(AtomicOrdering::SeqCst);
    }
    /// Manually sets the fake time to a specific value (in milliseconds).
    pub fn set_time(&self, new_time: u64) {
        self.now.store(new_time, AtomicOrdering::SeqCst);
    }
    /// Advances the fake time by a given amount (in milliseconds).
    pub fn advance_time(&self, millis: u64) {
        self.now.fetch_add(millis, AtomicOrdering::SeqCst);
    }
}

#[rustfmt::skip]
impl TimeSource for TimeSourceFake {
    fn is_monotonic() -> bool { true }
    fn time_scale() -> TimeScale { TimeScale::Millis }
    fn now_millis() -> u64 { TIME_SOURCE_FAKE.now.load(AtomicOrdering::SeqCst) }
    fn epoch_millis() -> u64 { 1_700_000_000_000 } // Default testing epoch
}
