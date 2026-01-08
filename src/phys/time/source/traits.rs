// devela::phys::time::source::traits
//
//! Defines [`TimeSource`] and [`TimeSourceCfg`].
//

use crate::TimeScale;

/// Blanket bridge that lifts any non-configurable `TimeSource` into
/// `TimeSourceCfg` by using `()` as its trivial configuration.
#[rustfmt::skip]
impl<T: TimeSource> TimeSourceCfg for T {
    type Config = ();
    fn time_is_monotonic((): ()) -> bool { T::time_is_monotonic() }
    fn time_is_absolute((): ()) -> bool { T::time_is_absolute() }
    fn time_scale((): ()) -> TimeScale { T::time_scale() }
    fn time_now_millis((): ()) -> u64 { T::time_now_millis() }
    //
    fn time_now_micros((): ()) -> u64 { T::time_now_micros() }
    fn time_now_nanos((): ()) -> u64 { T::time_now_nanos() }
    fn time_now_millis_f64((): ()) -> f64 { T::time_now_millis_f64() }
}

#[rustfmt::skip]
#[doc = crate::_tags!(time)]
/// A numeric time source.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// A `TimeSource` provides timestamps as numeric values (`u64`) on a well-defined timeline
/// with a known scale. Returned values are suitable for computing time deltas by subtraction.
///
/// ## Timeline model
/// - The timeline may be **absolute** (e.g. UNIX time),
///   **relative** (e.g. boot time, JS origin),
///   or **synthetic** (process-local).
/// - For monotonic sources, successive calls to `time_now_*` will never go backwards.
/// - For non-monotonic sources (e.g. wall clocks), values may jump.
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
    fn time_is_monotonic() -> bool;

    /// Returns whether this source provides an absolute civil timeline.
    ///
    /// If `true`, `time_now_*` returns a timestamp aligned to the Unix epoch.
    /// If `false`, `time_now_*` returns a relative or synthetic timeline.
    fn time_is_absolute() -> bool;

    /// Returns the time scale of this time source.
    fn time_scale() -> TimeScale;

    /* current time */

    /// Returns the current timestamp in milliseconds.
    ///
    /// All other `time_now_*` methods must be consistent with this timeline.
    ///
    /// By default, sub-millisecond methods derive from this value
    /// and assume a millisecond-based timeline.
    fn time_now_millis() -> u64;

    /* # provided # */
    /* current time */

    /// Returns the current timestamp in microseconds.
    fn time_now_micros() -> u64 { Self::time_now_millis() * 1_000 }
    /// Returns the current timestamp in nanoseconds.
    fn time_now_nanos() -> u64 { Self::time_now_millis() * 1_000_000 }

    /* float helpers */

    /// Returns the current timestamp as an `f64` value in milliseconds.
    fn time_now_millis_f64() -> f64 { Self::time_now_millis() as f64 }
}

#[rustfmt::skip]
#[doc = crate::_tags!(time)]
/// A configurable numeric time source.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// `TimeSourceCfg` generalizes [`TimeSource`] to sources whose behavior depends
/// on a runtime configuration value (for example, selecting a specific Linux clock).
///
/// Each configuration value defines its own numeric timeline with a known scale
/// and monotonicity. Returned timestamps are suitable for computing
/// deltas by subtraction *within the same configuration*.
///
/// `Config` is expected to be a small, copyable selector, not a stateful handle.
///
/// This trait exists to support families of clocks that
/// cannot be modeled as a single, fixed time source.
pub trait TimeSourceCfg {
    /// Runtime configuration used to select a concrete timeline.
    type Config;

    /* # required # */

    /// Returns whether the source selected by `cfg` is monotonic.
    ///
    /// A monotonic source never goes backwards on its timeline.
    fn time_is_monotonic(cfg: Self::Config) -> bool;

    /// Returns whether the source selected by `cfg` provides an absolute civil timeline.
    ///
    /// If `true`, `time_now_*` returns a timestamp aligned to the Unix epoch.
    /// If `false`, `time_now_*` returns a relative or synthetic timeline.
    fn time_is_absolute(cfg: Self::Config) -> bool;

    /// Returns the time scale of the timeline selected by `cfg`.
    fn time_scale(cfg: Self::Config) -> TimeScale;

    /// Returns the current timestamp in milliseconds for the timeline selected by `cfg`.
    ///
    /// By default, sub-millisecond methods derive from this value
    /// and assume a millisecond-based timeline.
    fn time_now_millis(cfg: Self::Config) -> u64;

    /* # provided # */

    /// Returns the current timestamp in microseconds.
    fn time_now_micros(cfg: Self::Config) -> u64 { Self::time_now_millis(cfg) * 1_000 }
    /// Returns the current timestamp in nanoseconds.
    fn time_now_nanos(cfg: Self::Config) -> u64 { Self::time_now_millis(cfg) * 1_000_000 }

    /// Returns the current timestamp as an `f64` value in milliseconds.
    fn time_now_millis_f64(cfg: Self::Config) -> f64 { Self::time_now_millis(cfg) as f64 }
}
