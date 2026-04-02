// devela::sys::os::linux::time::instant
//
//! Defines [`LinuxInstant`], [`LinuxTime`].
//

use crate::{Linux, LinuxClock, LinuxTimespec, TimeScale, TimeSource, TimeSourceCfg};

#[doc = crate::_tags!(linux time)]
/// A fast monotonic Linux time source.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// This is the canonical fixed Linux numeric source.
/// It uses `CLOCK_MONOTONIC` and projects time as `u64` nanoseconds.
#[derive(Debug)]
pub struct LinuxInstant;

#[inline(always)]
fn linux_ts_nanos(ts: LinuxTimespec) -> u64 {
    (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
}

#[rustfmt::skip]
/// Relative `u64` projection of [`LinuxInstant`] in nanoseconds.
///
/// This is the canonical wide numeric projection for `CLOCK_MONOTONIC`.
/// At nanosecond resolution, `u64` spans about 584 years.
impl TimeSource<u64> for LinuxInstant {
    fn time_is_monotonic() -> bool { true }
    fn time_is_absolute() -> bool { false }
    fn time_scale() -> TimeScale { TimeScale::Nanos }
    fn time_now() -> u64 {
        // SAFETY: CLOCK_MONOTONIC is always valid.
        let ts = unsafe {
            let mut ts = LinuxTimespec::default();
            let _ = Linux::sys_clock_gettime(LinuxClock::Monotonic, ts.as_mut_ptr());
            ts
        };
        linux_ts_nanos(ts)
    }
    fn time_point_value(point: u64) -> u64 { point }
    fn time_elapsed_value(elapsed: u64) -> u64 { elapsed }
}

/// Relative `u32` projection of [`LinuxInstant`] in microseconds.
///
/// This compact projection trades range for storage size while keeping
/// sub-millisecond precision useful for short and medium-lived runtimes.
///
/// At microsecond resolution, `u32` spans about 71.6 minutes.
#[rustfmt::skip]
impl TimeSource<u32> for LinuxInstant {
    fn time_is_monotonic() -> bool { true }
    fn time_is_absolute() -> bool { false }
    fn time_scale() -> TimeScale { TimeScale::Micros }
    fn time_now() -> u32 {
        // SAFETY: CLOCK_MONOTONIC is always valid.
        let ts = unsafe {
            let mut ts = LinuxTimespec::default();
            let _ = Linux::sys_clock_gettime(LinuxClock::Monotonic, ts.as_mut_ptr());
            ts
        };
        let ts_micros = (ts.tv_sec as u64) * 1_000_000 + (ts.tv_nsec as u64) / 1_000;
        u32::try_from(ts_micros).expect("LinuxInstant u32 microsecond projection overflow")
    }
    fn time_point_value(point: u32) -> u64 { point.into() }
    fn time_elapsed_value(elapsed: u32) -> u64 { elapsed.into() }
}

#[doc = crate::_tags!(linux time)]
/// A configurable family of Linux clocks.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// Each [`LinuxClock`] selects a concrete Linux timeline and exposes it
/// through the common `TimeSourceCfg<u64>` interface using nanosecond values.
#[derive(Debug)]
pub struct LinuxTime;

#[rustfmt::skip]
/// Configurable `u64` projection of [`LinuxTime`] in nanoseconds.
///
/// This is the canonical configurable wide numeric projection for Linux clocks.
/// At nanosecond resolution, `u64` spans about 584 years.
///
/// Whether the selected timeline is monotonic or absolute depends on the chosen [`LinuxClock`].
impl TimeSourceCfg<u64> for LinuxTime {
    type Config = LinuxClock;

    fn time_is_monotonic(clock: LinuxClock) -> bool {
        matches!(
            clock,
            LinuxClock::Monotonic
                | LinuxClock::MonotonicRaw
                | LinuxClock::MonotonicCoarse
                | LinuxClock::Boottime
                | LinuxClock::BoottimeAlarm
                | LinuxClock::Tai
        )
    }
    fn time_is_absolute(clock: LinuxClock) -> bool {
        matches!(
            clock,
            LinuxClock::Realtime
                | LinuxClock::RealtimeCoarse
                | LinuxClock::RealtimeAlarm
                | LinuxClock::Tai
        )
    }
    #[inline(always)]
    fn time_scale(_: LinuxClock) -> TimeScale { TimeScale::Nanos }
    #[inline(always)]
    fn time_now(clock: LinuxClock) -> u64 {
        let ts = Linux::clock_gettime(clock).expect("clock_gettime failed");
        linux_ts_nanos(ts)
    }
    #[inline(always)]
    fn time_point_value(_: LinuxClock, point: u64) -> u64 { point }
    #[inline(always)]
    fn time_elapsed_value(_: LinuxClock, elapsed: u64) -> u64 { elapsed }
}
