// devela::phys::time::source::impl_source
//
//!
//
// TOC
// - blanket bridge
// - SystemTime
// - SystemInstant
// - JsInstant
//
// NOTE: other impls in:
// - phys::time::source::fake::{TimeFake, TimeFakeRef}
// - sys::os::linux::time::instant::{LinuxInstant, LinuxTime}

use crate::{TimePoint, TimeScale, TimeSource, TimeSourceCfg};

/// Blanket bridge that lifts any non-configurable `TimeSource` into
/// `TimeSourceCfg` by using `()` as its trivial configuration.
#[rustfmt::skip]
impl<T, P> TimeSourceCfg<P> for T
where
    T: TimeSource<P>,
    P: TimePoint,
{
    type Config = ();

    fn time_is_monotonic((): ()) -> bool { T::time_is_monotonic() }
    fn time_is_absolute((): ()) -> bool { T::time_is_absolute() }
    fn time_scale((): ()) -> TimeScale { T::time_scale() }

    fn time_now((): ()) -> P { T::time_now() }
    fn time_point_value((): (), point: P) -> u64 { T::time_point_value(point) }
    fn time_elapsed_value((): (), elapsed: P::Elapsed) -> u64 { T::time_elapsed_value(elapsed) }

    fn time_now_millis((): ()) -> u64 { T::time_now_millis() }
    fn time_now_micros((): ()) -> u64 { T::time_now_micros() }
    fn time_now_nanos((): ()) -> u64 { T::time_now_nanos() }
    fn time_now_millis_f64((): ()) -> f64 { T::time_now_millis_f64() }
}

#[rustfmt::skip]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod impl_std {
    use crate::{Duration, UNIX_EPOCH, OnceLock, SystemInstant, SystemTime};
    use super::*;
    impl TimeSource<SystemTime> for SystemTime { fn time_is_monotonic() -> bool { false }
        fn time_is_absolute() -> bool { true }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn time_now() -> SystemTime { SystemTime::now() }
        fn time_point_value(point: SystemTime) -> u64 {
            point.duration_since(UNIX_EPOCH).expect("backwards system time").as_nanos() as u64
        }
        fn time_elapsed_value(elapsed: Duration) -> u64 { elapsed.as_nanos() as u64 }
    }
    /// Unix-time `u64` projection of [`SystemTime`] in nanoseconds.
    ///
    /// This is the canonical wide numeric projection:
    /// At nanosecond resolution, `u64` spans about 584 years.
    impl TimeSource<u64> for SystemTime {
        fn time_is_monotonic() -> bool { false }
        fn time_is_absolute() -> bool { true }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn time_now() -> u64 { SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("backwards system time").as_nanos() as u64
        }
        fn time_point_value(point: u64) -> u64 { point }
        fn time_elapsed_value(elapsed: u64) -> u64 { elapsed }
    }
    /// Unix-time `u32` projection of [`SystemTime`] in seconds.
    ///
    /// This gives a compact absolute projection with a range of about 136 years
    /// from the Unix epoch, so it remains valid until February 2106.
    impl TimeSource<u32> for SystemTime {
        fn time_is_monotonic() -> bool { false }
        fn time_is_absolute() -> bool { true }
        fn time_scale() -> TimeScale { TimeScale::Seconds }
        fn time_now() -> u32 {
            u32::try_from(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("backwards system time")
                    .as_secs()
            ).expect("SystemTime u32 second projection overflow")
        }
        fn time_point_value(point: u32) -> u64 { point.into() }
        fn time_elapsed_value(elapsed: u32) -> u64 { elapsed.into() }
    }

    /// Process-local base instant used to derive a numeric timeline for `SystemInstant`.
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

    impl TimeSource<SystemInstant> for SystemInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn time_now() -> SystemInstant { SystemInstant::now() }
        fn time_point_value(point: SystemInstant) -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            point.duration_since(*base).as_nanos() as u64
        }
        fn time_elapsed_value(elapsed: Duration) -> u64 { elapsed.as_nanos() as u64 }
    }
    /// Process-local monotonic `u64` projection of [`SystemInstant`] in nanoseconds.
    ///
    /// This is the canonical wide numeric projection:
    /// At nanosecond resolution, `u64` spans about 584 years.
    impl TimeSource<u64> for SystemInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Nanos }
        fn time_now() -> u64 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            base.elapsed().as_nanos() as u64
        }
        fn time_point_value(point: u64) -> u64 { point }
        fn time_elapsed_value(elapsed: u64) -> u64 { elapsed }
    }
    /// Process-local monotonic `u32` projection of [`SystemInstant`] in microseconds.
    ///
    /// This compact projection trades range for storage size while keeping
    /// sub-millisecond precision useful for short and medium-lived runtimes.
    ///
    /// At microsecond resolution, `u32` spans about 71.6 minutes,
    impl TimeSource<u32> for SystemInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Micros }
        fn time_now() -> u32 {
            let base = SYSTEM_INSTANT_BASE.get_or_init(SystemInstant::now);
            u32::try_from(base.elapsed().as_micros())
                .expect("SystemInstant u32 projection overflow")
        }
        fn time_point_value(point: u32) -> u64 { point.into() }
        fn time_elapsed_value(elapsed: u32) -> u64 { elapsed.into() }
    }
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "js", feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "js", feature = "unsafe_ffi"))))]
mod impl_js {
    use crate::JsInstant;
    use super::*;

    /// Relative `u64` projection of [`JsInstant`] in milliseconds.
    ///
    /// This is the canonical wide numeric projection for the browser high-resolution
    /// time origin. Milliseconds are the native scale exposed by the type's public API.
    ///
    /// At millisecond resolution, `u64` spans about 584 million years.
    impl TimeSource<u64> for JsInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Millis }
        fn time_now() -> u64 { JsInstant::now().as_millis_f64() as u64 }
        fn time_point_value(point: u64) -> u64 { point }
        fn time_elapsed_value(elapsed: u64) -> u64 { elapsed }
        fn time_now_millis_f64() -> f64 { JsInstant::now().as_millis_f64() }
    }
    /// Relative `u32` projection of [`JsInstant`] in milliseconds.
    ///
    /// This compact projection favors storage size while keeping a practical range
    /// for browser sessions and medium-lived applications.
    ///
    /// At millisecond resolution, `u32` spans about 49.7 days.
    impl TimeSource<u32> for JsInstant {
        fn time_is_monotonic() -> bool { true }
        fn time_is_absolute() -> bool { false }
        fn time_scale() -> TimeScale { TimeScale::Millis }
        fn time_now() -> u32 {
            u32::try_from(JsInstant::now().as_millis_f64() as u64)
                .expect("JsInstant u32 millisecond projection overflow")
        }
        fn time_point_value(point: u32) -> u64 { point.into() }
        fn time_elapsed_value(elapsed: u32) -> u64 { elapsed.into() }
        fn time_now_millis_f64() -> f64 { JsInstant::now().as_millis_f64() }
    }
}
