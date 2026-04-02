// devela::phys::time::source::source
//
//! Defines [`TimeSource`] and [`TimeSourceCfg`].
//
// TOC
// - trait TimeSource
// - trait TimeSourceCfg

use crate::{TimePoint, TimeScale};

#[rustfmt::skip]
#[doc = crate::_tags!(time)]
/// A source of time points with a canonical numeric projection.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// `TimeSource<P>` separates:
/// - the **point representation** `P`,
/// - the source's **timeline properties**,
/// - and a canonical `u64` projection of points and elapsed values.
///
/// The chosen point type `P` may be opaque (for example, `SystemInstant`)
/// or numeric (for example, `u64` or `u32`).
///
/// The numeric projection is expressed in [`time_scale`][Self::time_scale] units
/// and is intended for diagnostics, profiling, logging, and generic timestamp handling.
///
/// ## Timeline model
/// - The timeline may be **absolute** (for example, Unix time),
///   **relative** (for example, boot time or JS origin),
///   or **synthetic** (for example, process-local).
/// - For monotonic sources, successive calls to [`time_now`][Self::time_now]
///   will never go backwards on the source timeline.
/// - For non-monotonic sources, values may jump.
///
/// ## Intended use
/// This trait is designed for:
/// - storing time points in different representations,
/// - measuring forward elapsed time,
/// - profiling and logging,
/// - diagnostics and instrumentation.
///
/// It does not require the point type itself to be numeric.
pub trait TimeSource<P: TimePoint> {
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

    /// Returns the unit used by [`time_point_value`][Self::time_point_value]
    /// and [`time_elapsed_value`][Self::time_elapsed_value].
    fn time_scale() -> TimeScale;

    /* current point */

    /// Returns the current time point in the chosen representation `P`.
    fn time_now() -> P;

    /* numeric projection */

    /// Converts a time point into a `u64` value in [`time_scale`][Self::time_scale] units.
    fn time_point_value(point: P) -> u64;

    /// Converts an elapsed value into a `u64` value in [`time_scale`][Self::time_scale] units.
    fn time_elapsed_value(elapsed: P::Elapsed) -> u64;

    /* # provided # */

    /// Returns the current time as a `u64` value in [`time_scale`][Self::time_scale] units.
    fn time_now_value() -> u64 { Self::time_point_value(Self::time_now()) }

    /// Returns the forward elapsed value from `point` to now.
    fn time_elapsed_since(point: P) -> P::Elapsed { P::time_elapsed(Self::time_now(), point) }

    /// Returns the forward elapsed value from `point` to now,
    /// or `None` if it is not valid or not representable.
    fn time_elapsed_since_checked(point: P) -> Option<P::Elapsed> {
        P::time_elapsed_checked(Self::time_now(), point)
    }
    /// Returns the forward elapsed value from `point` to now
    /// as a `u64` in [`time_scale`][Self::time_scale] units.
    fn time_elapsed_since_value(point: P) -> u64 {
        Self::time_elapsed_value(Self::time_elapsed_since(point))
    }

    /// Converts `point` to seconds.
    fn time_point_seconds(point: P) -> u64 {
        Self::time_scale().convert_simulated(Self::time_point_value(point), TimeScale::Seconds)
    }
    /// Converts `point` to milliseconds.
    fn time_point_millis(point: P) -> u64 {
        Self::time_scale().convert_simulated(Self::time_point_value(point), TimeScale::Millis)
    }
    /// Converts `point` to microseconds.
    fn time_point_micros(point: P) -> u64 {
        Self::time_scale().convert_simulated(Self::time_point_value(point), TimeScale::Micros)
    }
    /// Converts `point` to nanoseconds.
    fn time_point_nanos(point: P) -> u64 {
        Self::time_scale().convert_simulated(Self::time_point_value(point), TimeScale::Nanos)
    }

    /// Converts `elapsed` to seconds.
    fn time_elapsed_seconds(elapsed: P::Elapsed) -> u64 {
        Self::time_scale().convert_simulated(Self::time_elapsed_value(elapsed), TimeScale::Seconds)
    }
    /// Converts `elapsed` to milliseconds.
    fn time_elapsed_millis(elapsed: P::Elapsed) -> u64 {
        Self::time_scale().convert_simulated(Self::time_elapsed_value(elapsed), TimeScale::Millis)
    }
    /// Converts `elapsed` to microseconds.
    fn time_elapsed_micros(elapsed: P::Elapsed) -> u64 {
        Self::time_scale().convert_simulated(Self::time_elapsed_value(elapsed), TimeScale::Micros)
    }
    /// Converts `elapsed` to nanoseconds.
    fn time_elapsed_nanos(elapsed: P::Elapsed) -> u64 {
        Self::time_scale().convert_simulated(Self::time_elapsed_value(elapsed), TimeScale::Nanos)
    }

    /// Returns the current timestamp in seconds.
    fn time_now_seconds() -> u64 { Self::time_point_seconds(Self::time_now()) }
    /// Returns the current timestamp in milliseconds.
    fn time_now_millis() -> u64 { Self::time_point_millis(Self::time_now()) }
    /// Returns the current timestamp in microseconds.
    fn time_now_micros() -> u64 { Self::time_point_micros(Self::time_now()) }
    /// Returns the current timestamp in nanoseconds.
    fn time_now_nanos() -> u64 { Self::time_point_nanos(Self::time_now()) }

    /// Returns the current timestamp as milliseconds in `f64`.
    fn time_now_millis_f64() -> f64 { Self::time_now_millis() as f64 }
}

#[rustfmt::skip]
#[doc = crate::_tags!(time)]
/// A configurable source of time points with a canonical numeric projection.
#[doc = crate::_doc_location!("phys/time/source")]
///
/// `TimeSourceCfg<P>` generalizes [`TimeSource`] to source families whose behavior
/// depends on a runtime configuration value.
///
/// Each configuration selects a concrete timeline for the chosen point representation `P`.
/// The numeric projection is expressed in [`time_scale`][Self::time_scale] units.
///
/// `Config` is expected to be a small copyable selector, not a stateful owner.
pub trait TimeSourceCfg<P: TimePoint> {
    /// Runtime configuration used to select a concrete timeline.
    type Config: Copy;

    /* # required # */
    /* properties */

    /// Returns whether the selected source timeline is monotonic.
    ///
    /// A monotonic source never goes backwards on its timeline.
    fn time_is_monotonic(cfg: Self::Config) -> bool;

    /// Returns whether the selected source timeline is absolute.
    ///
    /// If `true`, `time_now_*` returns a timestamp aligned to the Unix epoch.
    /// If `false`, `time_now_*` returns a relative or synthetic timeline.
    fn time_is_absolute(cfg: Self::Config) -> bool;

    /// Returns the unit used by `time_point_value` and `time_elapsed_value`.
    fn time_scale(cfg: Self::Config) -> TimeScale;

    /* current point */

    /// Returns the current time point for the selected configuration.
    fn time_now(cfg: Self::Config) -> P;

    /* numeric projection */

    /// Converts a time point into a `u64` value in `time_scale(cfg)` units.
    fn time_point_value(cfg: Self::Config, point: P) -> u64;

    /// Converts an elapsed value into a `u64` value in `time_scale(cfg)` units.
    fn time_elapsed_value(cfg: Self::Config, elapsed: P::Elapsed) -> u64;

    /* # provided # */

    /// Returns the current time as a `u64` value in [`time_scale`][Self::time_scale] units.
    fn time_now_value(cfg: Self::Config) -> u64 {
        Self::time_point_value(cfg, Self::time_now(cfg))
    }

    /// Returns the forward elapsed value from `point` to now.
    fn time_elapsed_since(cfg: Self::Config, point: P) -> P::Elapsed {
        P::time_elapsed(Self::time_now(cfg), point)
    }
    /// Returns the forward elapsed value from `point` to now,
    /// or `None` if it is not valid or not representable.
    fn time_elapsed_since_checked(cfg: Self::Config, point: P) -> Option<P::Elapsed> {
        P::time_elapsed_checked(Self::time_now(cfg), point)
    }
    /// Returns the forward elapsed value from `point` to now
    /// as a `u64` in [`time_scale`][Self::time_scale] units.
    fn time_elapsed_since_value(cfg: Self::Config, point: P) -> u64 {
        Self::time_elapsed_value(cfg, Self::time_elapsed_since(cfg, point))
    }

    /// Converts `point` to seconds.
    fn time_point_seconds(cfg: Self::Config, point: P) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_point_value(cfg, point), TimeScale::Seconds)
    }
    /// Converts `point` to milliseconds.
    fn time_point_millis(cfg: Self::Config, point: P) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_point_value(cfg, point), TimeScale::Millis)
    }
    /// Converts `point` to microseconds.
    fn time_point_micros(cfg: Self::Config, point: P) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_point_value(cfg, point), TimeScale::Micros)
    }
    /// Converts `point` to nanoseconds.
    fn time_point_nanos(cfg: Self::Config, point: P) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_point_value(cfg, point), TimeScale::Nanos)
    }

    /// Converts `elapsed` to seconds.
    fn time_elapsed_seconds(cfg: Self::Config, elapsed: P::Elapsed) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_elapsed_value(cfg, elapsed), TimeScale::Seconds)
    }
    /// Converts `elapsed` to milliseconds.
    fn time_elapsed_millis(cfg: Self::Config, elapsed: P::Elapsed) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_elapsed_value(cfg, elapsed), TimeScale::Millis)
    }
    /// Converts `elapsed` to microseconds.
    fn time_elapsed_micros(cfg: Self::Config, elapsed: P::Elapsed) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_elapsed_value(cfg, elapsed), TimeScale::Micros)
    }
    /// Converts `elapsed` to nanoseconds.
    fn time_elapsed_nanos(cfg: Self::Config, elapsed: P::Elapsed) -> u64 {
        Self::time_scale(cfg)
            .convert_simulated(Self::time_elapsed_value(cfg, elapsed), TimeScale::Nanos)
    }

    /// Returns the current timestamp in seconds.
    fn time_now_seconds(cfg: Self::Config) -> u64 {
        Self::time_point_seconds(cfg, Self::time_now(cfg))
    }
    /// Returns the current timestamp in milliseconds.
    fn time_now_millis(cfg: Self::Config) -> u64 {
        Self::time_point_millis(cfg, Self::time_now(cfg))
    }
    /// Returns the current timestamp in microseconds.
    fn time_now_micros(cfg: Self::Config) -> u64 {
        Self::time_point_micros(cfg, Self::time_now(cfg))
    }
    /// Returns the current timestamp in nanoseconds.
    fn time_now_nanos(cfg: Self::Config) -> u64 {
        Self::time_point_nanos(cfg, Self::time_now(cfg))
    }

    /// Returns the current timestamp as milliseconds in `f64`.
    fn time_now_millis_f64(cfg: Self::Config) -> f64 { Self::time_now_millis(cfg) as f64 }
}
