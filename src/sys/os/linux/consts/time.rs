// devela::sys::os::linux::consts::time
//
//! Linux time-related constants.
//

use crate::c_int;

/// [`Linux`][crate::Linux] clock identifiers for
/// [`sys_clock_gettime`][crate::Linux::sys_clock_gettime] and related time functions.
///
/// See [clock_gettime(2)] for detailed information about each clock type.
///
/// [clock_gettime(2)]: https://www.man7.org/linux/man-pages/man2/clock_gettime.2.html
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct LINUX_CLOCK;
impl LINUX_CLOCK {
    /// System-wide realtime clock (wall clock). Adjustable via NTP.
    pub const REALTIME: c_int = 0;

    /// Monotonic clock (steady since boot). Unaffected by NTP.
    pub const MONOTONIC: c_int = 1;

    /// A clock that measures CPU time consumed by this process.
    // since Linux 2.6.12
    pub const PROCESS_CPUTIME_ID: c_int = 2;

    /// A clock that measures CPU time consumed by this thread.
    // since Linux 2.6.12
    pub const THREAD_CPUTIME_ID: c_int = 3;

    /// Monotonic, raw hardware time (TSC-based where available).
    // since Linux 2.6.28
    pub const MONOTONIC_RAW: c_int = 4;

    /// A faster but less precise version of [`REALTIME`][Self::REALTIME].
    // since Linux 2.6.32
    pub const REALTIME_COARSE: c_int = 5;

    /// A faster but less precise version of [`MONOTONIC`][Self::MONOTONIC].
    // since Linux 2.6.32
    pub const MONOTONIC_COARSE: c_int = 6;

    /// Like [`MONOTONIC`][Self::MONOTONIC] but includes suspend time.
    // since Linux 2.6.39
    pub const BOOTTIME: c_int = 7;

    /// Like [`REALTIME`][Self::REALTIME], but not settable.
    // since Linux 3.0
    pub const REALTIME_ALARM: c_int = 8;

    /// Like [`BOOTTIME`][Self::BOOTTIME], but not settable.
    pub const BOOTTIME_ALARM: c_int = 9;

    // The driver implementing this got removed
    // pub const SGI_CYCLE: c_int = 10;

    /// International Atomic Time (ignores leap seconds).
    // since Linux 3.10
    pub const TAI: c_int = 11;
}
