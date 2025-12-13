// devela::sys::os::linux::time::instant
//
//! Defines [`LinuxInstant`], [`LinuxTime`].
//

use crate::{Linux, LinuxClock, LinuxTimespec, TimeScale, TimeSource, TimeSourceCfg};

/// A fast, monotonic Linux time source.
///
/// This type is a zero-configuration wrapper around a fixed
/// Linux monotonic clock, optimized for hot timing paths.
///
/// Internally uses `CLOCK_MONOTONIC`.
#[derive(Debug)]
pub struct LinuxInstant;

#[rustfmt::skip]
impl TimeSource for LinuxInstant {
    #[inline(always)]
    fn time_is_monotonic() -> bool { true }
    #[inline(always)]
    fn time_is_absolute() -> bool { false }
    #[inline(always)]
    fn time_scale() -> TimeScale { TimeScale::Nanoseconds }
    #[inline(always)]
    fn time_now_millis() -> u64 {
        // SAFETY: CLOCK_MONOTONIC is always valid
        let ts = unsafe {
            let mut ts = LinuxTimespec::default();
            let _ = Linux::sys_clock_gettime(LinuxClock::Monotonic, ts.as_mut_ptr());
            ts
        };

        (ts.tv_sec as u64) * 1_000 + (ts.tv_nsec as u64) / 1_000_000
    }
    #[inline(always)]
    fn time_now_micros() -> u64 {
        // SAFETY: CLOCK_MONOTONIC is always valid
        let ts = unsafe {
            let mut ts = LinuxTimespec::default();
            let _ = Linux::sys_clock_gettime(LinuxClock::Monotonic, ts.as_mut_ptr());
            ts
        };
        (ts.tv_sec as u64) * 1_000_000 + (ts.tv_nsec as u64) / 1_000
    }
    #[inline(always)]
    fn time_now_nanos() -> u64 {
        // SAFETY: CLOCK_MONOTONIC is always valid
        let ts = unsafe {
            let mut ts = LinuxTimespec::default();
            let _ = Linux::sys_clock_gettime(LinuxClock::Monotonic, ts.as_mut_ptr());
            ts
        };
        (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
    }
}

/// Configurable Linux time source.
///
/// Represents the family of Linux clocks selectable via `LinuxClock`.
/// Each configuration defines an independent numeric timeline.
#[derive(Debug)]
pub struct LinuxTime;

impl TimeSourceCfg for LinuxTime {
    type Config = LinuxClock;

    fn time_is_monotonic(clock: LinuxClock) -> bool {
        matches!(
            clock,
            LinuxClock::Monotonic
                | LinuxClock::MonotonicRaw
                | LinuxClock::Boottime
                | LinuxClock::Tai
                | LinuxClock::MonotonicCoarse
                | LinuxClock::BoottimeAlarm
        )
    }
    #[inline(always)]
    fn time_is_absolute(_: LinuxClock) -> bool {
        true
    }
    fn time_scale(_: LinuxClock) -> TimeScale {
        TimeScale::Nanoseconds
    }
    fn time_now_millis(clock: LinuxClock) -> u64 {
        let ts = Linux::clock_gettime(clock).expect("clock_gettime failed");
        (ts.tv_sec as u64) * 1_000 + (ts.tv_nsec as u64) / 1_000_000
    }
    fn time_now_micros(clock: LinuxClock) -> u64 {
        let ts = Linux::clock_gettime(clock).expect("clock_gettime failed");
        (ts.tv_sec as u64) * 1_000_000 + (ts.tv_nsec as u64) / 1_000
    }
    fn time_now_nanos(clock: LinuxClock) -> u64 {
        let ts = Linux::clock_gettime(clock).expect("clock_gettime failed");
        (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
    }
}
