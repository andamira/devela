// devela::phys::time::source
//
//! # Time sources
//!
//! This module defines **numeric time sources** used for profiling,
//! instrumentation, and elapsed-time measurement.
//!
//! ## Core model
//!
//! A [`TimeSource`] provides timestamps as numeric values (`u64`) on a
//! well-defined timeline with a known scale. Returned values are suitable
//! for computing time deltas by subtraction.
//!
//! - The timeline may be **absolute** (e.g. UNIX time),
//!   **relative** (e.g. boot time, JS origin),
//!   or **synthetic** (process-local).
//! - A source may be **monotonic** or **non-monotonic**.
//! - An exposed epoch is **optional** and informational.
//!
//! ## Numeric vs opaque time
//!
//! Some APIs (such as `SystemInstant`) expose *opaque instants* that
//! can only be compared by duration. To fit the numeric timeline model,
//! such sources use a synthetic, process-local origin.
//!
//! ## Configurable sources
//!
//! [`TimeSourceCfg`] extends this model to families of clocks whose behavior
//! depends on a runtime configuration (for example, Linux clock IDs).
//!
//! Non-configurable sources automatically lift into `TimeSourceCfg` using
//! a trivial `()` configuration.
//!
//! ## Source comparison
//!
//! | Source              | Monotonic | Numeric epoch | Epoch meaning          |
//! |---------------------|-----------|---------------|------------------------|
//! | [`SystemTime`]      | No        | Yes           | UNIX epoch             |
//! | [`SystemInstant`]     | Yes       | Synthetic     | Process-local          |
//! | Linux `CLOCK_REALTIME` | No     | Yes           | UNIX epoch             |
//! | Linux `CLOCK_MONOTONIC` | Yes   | Relative      | Boot time              |
//! | [`JsInstant`]       | Yes       | Relative      | JS time origin         |
//!
#![cfg_attr(feature = "std", doc = "[`SystemTime`]: crate::SystemTime")]
#![cfg_attr(not(feature = "std"), doc = "[`SystemTime`]: #")]
#![cfg_attr(feature = "std", doc = "[`SystemInstant`]: crate::SystemInstant")]
#![cfg_attr(not(feature = "std"), doc = "[`SystemInstant`]: #")]
#![cfg_attr(all(feature = "js", feature = "unsafe_ffi"), doc = "[`JsInstant`]: crate::JsInstant")]
#![cfg_attr(not(all(feature = "js", feature = "unsafe_ffi")), doc = "[`JsInstant`]: #")]
//
// TOC
// - definitions
//   - trait TimeSource
//   - trait TimeSourceCfg
// - impls
//   - blanket
//   - SystemTime
//   - SystemInstant
//   - JsInstant
// - tests
//   - struct TimeSourceFake

use crate::TimeScale;

/* definitions */

#[rustfmt::skip]
#[doc = crate::_TAG_TIME!()]
/// A numeric time source.
///
/// A `TimeSource` provides timestamps as numeric values (`u64`) on a well-defined timeline
/// with a known scale. Returned values are suitable for computing time deltas by subtraction.
///
/// ## Timeline model
/// - The timeline may be **absolute** (e.g. UNIX time),
///   **relative** (e.g. boot time, JS origin),
///   or **synthetic** (process-local).
/// - For monotonic sources, successive calls to `now_*` will never go backwards.
/// - For non-monotonic sources (e.g. wall clocks), values may jump.
///
/// ## Epoch
/// Some sources expose a meaningful epoch via `epoch_*`. Others (notably [`SystemInstant`])
/// use a synthetic, process-local origin and therefore return `0` as their epoch.
///
/// ## Intended use
/// This trait is designed for:
/// - profiling and logging
/// - measuring elapsed time
/// - diagnostics and instrumentation
///
/// It is **not** intended to expose raw timestamp types.
pub trait TimeSource {
    /* # required # */
    /* properties */

    /// Returns whether this source is monotonic.
    ///
    /// A monotonic source never goes backwards on its timeline.
    fn is_monotonic() -> bool;

    /// Returns the time scale of this time source.
    fn time_scale() -> TimeScale;

    /* current time */

    /// Returns the current timestamp in milliseconds.
    ///
    /// All other `now_*` methods must be consistent with this timeline.
    fn now_millis() -> u64;

    /* # provided # */
    /* current time */

    /// Returns the current timestamp in microseconds.
    fn now_micros() -> u64 { Self::now_millis() * 1_000 }
    /// Returns the current timestamp in nanoseconds.
    fn now_nanos() -> u64 { Self::now_millis() * 1_000_000 }

    /* epoch / origin */

    /// Returns the epoch offset in milliseconds.
    ///
    /// It is provided for inspection and logging only;
    /// it must not be relied upon for ordering or delta computation.
    ///
    /// Returns `0` if this source does not expose a meaningful epoch.
    fn epoch_millis() -> u64 { 0 }
    /// Returns the epoch offset in microseconds.
    fn epoch_micros() -> u64 { Self::epoch_millis() * 1_000 }
    /// Returns the epoch offset in nanoseconds.
    fn epoch_nanos() -> u64 { Self::epoch_millis() * 1_000_000 }

    /* float helpers */

    /// Returns the current timestamp as an `f64` value in milliseconds.
    fn now_millis_f64() -> f64 { Self::now_millis() as f64 }
    /// Returns the current timestamp as an `f64` value in milliseconds.
    fn epoch_millis_f64() -> f64 { Self::epoch_millis() as f64 }
}

#[rustfmt::skip]
#[doc = crate::_TAG_TIME!()]
/// A configurable numeric time source.
///
/// `TimeSourceCfg` generalizes [`TimeSource`] to sources whose behavior depends
/// on a runtime configuration value (for example, selecting a specific Linux clock).
///
/// Each configuration value defines its own numeric timeline with a known scale
/// and monotonicity. Returned timestamps are suitable for computing
/// deltas by subtraction *within the same configuration*.
///
/// This trait exists to support families of clocks that
/// cannot be modeled as a single, fixed time source.
pub trait TimeSourceCfg {
    /// Runtime configuration used to select a concrete timeline.
    type Config;

    /* # required # */

    /// Returns whether the timeline selected by `cfg` is monotonic.
    ///
    /// A monotonic timeline never goes backwards.
    fn is_monotonic(cfg: Self::Config) -> bool;

    /// Returns the time scale of the timeline selected by `cfg`.
    fn time_scale(cfg: Self::Config) -> TimeScale;

    /// Returns the current timestamp in milliseconds for the timeline selected by `cfg`.
    fn now_millis(cfg: Self::Config) -> u64;

    /* # provided # */

    /// Returns the current timestamp in microseconds.
    fn now_micros(cfg: Self::Config) -> u64 { Self::now_millis(cfg) * 1_000 }
    /// Returns the current timestamp in nanoseconds.
    fn now_nanos(cfg: Self::Config) -> u64 { Self::now_millis(cfg) * 1_000_000 }

    /// Returns the epoch offset in milliseconds for `cfg`.
    ///
    /// Returns `0` if the selected timeline does not expose a meaningful epoch.
    fn epoch_millis(cfg: Self::Config) -> u64 { 0 }
    /// Returns the epoch offset in microseconds.
    fn epoch_micros(cfg: Self::Config) -> u64 { Self::epoch_millis(cfg) * 1_000 }
    /// Returns the epoch offset in nanoseconds.
    fn epoch_nanos(cfg: Self::Config) -> u64 { Self::epoch_millis(cfg) * 1_000_000 }

    /// Returns the current timestamp as an `f64` value in milliseconds.
    fn now_millis_f64(cfg: Self::Config) -> f64 { Self::now_millis(cfg) as f64 }
    /// Returns the current timestamp as an `f64` value in milliseconds.
    fn epoch_millis_f64(cfg: Self::Config) -> f64 { Self::epoch_millis(cfg) as f64 }
}

/* impls */

#[rustfmt::skip]
/// Blanket bridge that lifts any non-configurable `TimeSource` into
/// `TimeSourceCfg` by using `()` as its trivial configuration.
impl<T: TimeSource> TimeSourceCfg for T {
    type Config = ();
    fn is_monotonic(_: ()) -> bool { T::is_monotonic() }
    fn time_scale(_: ()) -> TimeScale { T::time_scale() }
    fn now_millis(_: ()) -> u64 { T::now_millis() }
    //
    fn epoch_millis(cfg: Self::Config) -> u64 { T::epoch_millis() }
    fn now_micros(cfg: Self::Config) -> u64 { T::now_micros() }
    fn epoch_micros(cfg: Self::Config) -> u64 { T::epoch_micros() }
    fn now_nanos(cfg: Self::Config) -> u64 { T::now_nanos() }
    fn epoch_nanos(cfg: Self::Config) -> u64 { T::epoch_nanos() }
    fn now_millis_f64(cfg: Self::Config) -> f64 { T::now_millis_f64() }
    fn epoch_millis_f64(cfg: Self::Config) -> f64 { T::epoch_millis_f64() }
}

#[rustfmt::skip]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::{TimeSource, TimeScale, UNIX_EPOCH, OnceLock, SystemInstant, SystemTime};

    impl TimeSource for SystemTime {
        fn is_monotonic() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
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

    /// Process-local base instant used to derive a numeric timeline for `std::time::Instant`.
    ///
    /// `SystemInstant` does not expose a global or absolute epoch;
    /// it only supports measuring durations between two instants.
    /// To fit the `TimeSource` numeric timeline model, a synthetic,
    /// process-local origin is established on first use.
    ///
    /// All timestamps produced by the `SystemInstant` time source
    /// are measured as the elapsed duration since this base instant.
    ///
    /// This base is initialized lazily and exactly once.
    static SYSTEM_INSTANT_BASE: OnceLock<SystemInstant> = OnceLock::new();

    impl TimeSource for SystemInstant {
        fn is_monotonic() -> bool { true }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn now_millis() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_millis() as u64
        }
        //
        fn now_micros() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_micros() as u64
        }
        fn now_nanos() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_nanos() as u64
        }
    }
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "js", feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "js", feature = "unsafe_ffi"))))]
impl TimeSource for crate::JsInstant {
    fn is_monotonic() -> bool { true }
    fn time_scale() -> TimeScale { TimeScale::Millis }
    fn now_millis() -> u64 { crate::JsInstant::now().as_millis_f64() as u64 }
    //
    /// Returns the JS time origin, not UNIX time.
    fn epoch_millis() -> u64 { crate::JsInstant::origin().as_millis_f64() as u64 }
    fn now_millis_f64() -> f64 { crate::JsInstant::now().as_millis_f64() }
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use tests::*;
#[cfg(test)]
mod tests {
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
}
