// devela::phys::time::source
//
//! Defines the [`TimeSource`] trait.
//
// TOC
// - TimeSource
// - TimeSourceFake

use crate::{Enum, Ratio, TimeGranularity};

#[cfg(all(feature = "js", feature = "unsafe_ffi", not(windows)))]
use crate::JsInstant;
#[cfg(feature = "std")]
use crate::{SystemInstant, SystemTime, UNIX_EPOCH};

#[rustfmt::skip]
#[doc = crate::TAG_TIME!()]
/// A source of timestamps with a known granularity and monotonicity.
///
/// Provides a consistent API for querying timestamps at various precisions,
/// abstracting over different time sources.
pub trait TimeSource<const MONOTONIC: bool> {
    /// Returns the granularity of this time source.
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>>;

    /// Returns the current timestamp in milliseconds.
    fn now_millis() -> u64;

    /* non-required */

    /// Returns the epoch offset in milliseconds.
    ///
    /// - For absolute sources (e.g. `SystemTime`), returns the absolute epoch (e.g. `UNIX_EPOCH`).
    /// - For monotonic sources with a known meaningful reference (e.g. `JsInstant`),
    ///   returns a meaningful offset.
    /// - For monotonic sources without a meaningful absolute reference (e.g. `SystemInstant`),
    ///   returns `0` (default).
    fn epoch_millis() -> u64 { 0 }

    /// Returns the current timestamp in microseconds.
    ///
    /// Default: Uses `now_millis()`.
    fn now_micros() -> u64 { Self::now_millis() * 1_000 }

    /// Returns the epoch offset in microseconds.
    ///
    /// Default: Uses `epoch_millis()`.
    fn epoch_micros() -> u64 { Self::epoch_millis() * 1_000 }

    /// Returns the current timestamp in nanoseconds.
    ///
    /// Default: Uses `now_millis()`.
    fn now_nanos() -> u64 { Self::now_millis() * 1_000_000 }

    /// Returns the epoch offset in nanoseconds.
    ///
    /// Default: Uses `epoch_millis()`.
    fn epoch_nanos() -> u64 { Self::epoch_millis() * 1_000_000 }

    /// Returns the current timestamp as an `f64` value in milliseconds.
    ///
    /// Default: Converts `now_millis()` to `f64`.
    fn now_millis_f64() -> f64 { Self::now_millis() as f64 }

    /// Returns the current timestamp as an `f64` value in milliseconds.
    ///
    /// Default: Converts `now_millis()` to `f64`.
    fn epoch_millis_f64() -> f64 { Self::epoch_millis() as f64 }
}

#[cfg(feature = "std")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl TimeSource<false> for SystemTime {
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
        Enum::A(TimeGranularity::Nanos)
    }
    fn now_millis() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_millis() as u64
    }
    //
    fn now_micros() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_micros() as u64
    }
    fn now_nanos() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("backwards time").as_nanos() as u64
    }
}
#[cfg(feature = "std")] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl TimeSource<true> for SystemInstant {
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
        Enum::A(TimeGranularity::Nanos)
    }
    fn now_millis() -> u64 { SystemInstant::now().elapsed().as_millis() as u64 }
    //
    fn now_micros() -> u64 { SystemInstant::now().elapsed().as_micros() as u64 }
    fn now_nanos() -> u64 { SystemInstant::now().elapsed().as_nanos() as u64 }
}

#[cfg(all(feature = "js", feature = "unsafe_ffi", not(windows)))] #[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "js", feature = "unsafe_ffi"))))]
impl TimeSource<true> for JsInstant {
    fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
        Enum::A(TimeGranularity::Millis)
    }
    fn now_millis() -> u64 { JsInstant::now().as_millis_f64() as u64 }
    fn epoch_millis() -> u64 { JsInstant::origin().as_millis_f64() as u64 }
    //
    fn now_millis_f64() -> f64 { JsInstant::now().as_millis_f64() }
}

// #[cfg(all(target_arch = "arm", feature = "dep_cortex_m"))]
// impl TimeSource<true> for ::cortex_m::peripheral::DWT {
//     fn granularity() -> Enum<TimeGranularity, Ratio<u32, u32>> {
//         Enum::B(Ratio<1, 32_768>)
//     }
//     fn now_millis() -> u64 {
//         unsafe { ::cortex_m::peripheral::DWT::cycle_count() as u64 / (SystemCoreClock / 1_000) }
//     }
//     fn now_micros() -> u64 {
//         unsafe { ::cortex_m::peripheral::DWT::cycle_count() as u64 / (SystemCoreClock / 1_000_000) }
//     }
// }

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use tests::*;
#[cfg(test)]
mod tests {
    #![allow(dead_code, unused_variables)]

    use crate::{AtomicOrdering, AtomicU64, Enum, Ratio, TAG_FAKE, TimeGranularity, TimeSource};

    /// Global test time source for convenience.
    #[doc = TAG_FAKE!()]
    pub(crate) static TIME_SOURCE_FAKE: TimeSourceFake = TimeSourceFake::new(1_700_000_000_000);

    #[doc = TAG_FAKE!()]
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
    impl TimeSource<true> for TimeSourceFake {
        fn granularity() -> Enum<2, TimeGranularity, Ratio<u32, u32>> {
            Enum::A(TimeGranularity::Millis)
        }
        fn now_millis() -> u64 {
            TIME_SOURCE_FAKE.now.load(AtomicOrdering::SeqCst)
        }
        fn epoch_millis() -> u64 {
            1_700_000_000_000 // Default testing epoch
        }
    }
}
