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
    fn is_monotonic(_: ()) -> bool { T::is_monotonic() }
    fn time_scale(_: ()) -> TimeScale { T::time_scale() }
    fn now_millis(_: ()) -> u64 { T::now_millis() }
    //
    fn epoch_millis(_: ()) -> u64 { T::epoch_millis() }
    fn now_micros(_: ()) -> u64 { T::now_micros() }
    fn epoch_micros(_: ()) -> u64 { T::epoch_micros() }
    fn now_nanos(_: ()) -> u64 { T::now_nanos() }
    fn epoch_nanos(_: ()) -> u64 { T::epoch_nanos() }
    fn now_millis_f64(_: ()) -> f64 { T::now_millis_f64() }
    fn epoch_millis_f64(_: ()) -> f64 { T::epoch_millis_f64() }
}

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
    #[allow(unused_variables)]
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
