// devela::sys::os::linux::consts::clock
//
//! Defines [`LINUX_CLOCK`].
//

use crate::{LINUX_ERRNO, Linux, LinuxError, LinuxResult as Result, LinuxTimespec, c_int};

/// [`Linux`][crate::Linux] clock identifiers for
/// [`sys_clock_gettime`][crate::Linux::sys_clock_gettime] and related time functions.
///
/// See [clock_gettime(2)] for detailed information about each clock type.
///
/// [clock_gettime(2)]: https://www.man7.org/linux/man-pages/man2/clock_gettime.2.html
#[repr(i32)] // == c_int
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LinuxClock {
    /// System-wide realtime clock (wall clock). Adjustable via NTP.
    Realtime = 0,

    /// Monotonic clock (steady since boot). Unaffected by NTP.
    Monotonic = 1,

    /// A clock that measures CPU time consumed by this process.
    // since Linux 2.6.12
    ProcessCpuTimeId = 2,

    /// A clock that measures CPU time consumed by this thread.
    // since Linux 2.6.12
    ThreadCpuTimeId = 3,

    /// Monotonic, raw hardware time (TSC-based where available).
    // since Linux 2.6.28
    MonotonicRaw = 4,

    /// A faster but less precise version of [`Realtime`][Self::Realtime].
    // since Linux 2.6.32
    RealtimeCoarse = 5,

    /// A faster but less precise version of [`Monotonic`][Self::Monotonic].
    // since Linux 2.6.32
    MonotonicCoarse = 6,

    /// Like [`Monotonic`][Self::Monotonic] but includes suspend time.
    // since Linux 2.6.39
    Boottime = 7,

    /// Like [`Realtime`][Self::Realtime], but not settable.
    // since Linux 3.0
    RealtimeAlarm = 8,

    /// Like [`Boottime`][Self::Boottime], but not settable.
    BoottimeAlarm = 9,

    // The driver implementing this got removed
    // SgiCycle = 10,
    /// International Atomic Time (ignores leap seconds).
    // since Linux 3.10
    Tai = 11,
}

impl TryFrom<c_int> for LinuxClock {
    type Error = LinuxError;
    fn try_from(value: c_int) -> Result<Self> {
        Self::from_raw(value).ok_or(LinuxError::Sys(LINUX_ERRNO::EINVAL))
    }
}
impl From<LinuxClock> for c_int {
    fn from(clock: LinuxClock) -> Self {
        clock.as_raw()
    }
}

impl LinuxClock {
    /// Is this clock adjusted by NTP or system time changes?
    pub fn is_ntp_adjusted(self) -> bool {
        matches!(self, Self::Realtime | Self::RealtimeCoarse)
        // Note: TAI is not NTP-adjusted but can be manually set
    }

    /// Is this clock based on raw hardware (completely unadjusted)?
    pub fn is_raw_hardware(self) -> bool {
        matches!(self, Self::MonotonicRaw)
        // TAI is also "raw" in a sense but can be set, so excluded
    }

    /// Is this clock settable via `clock_settime()`?
    ///
    /// Note that only alarm clocks are specifically NOT settable.
    pub fn is_settable(self) -> bool {
        !self.is_alarm()
    }

    /// Is this a monotonic clock? (never goes backwards)
    ///
    /// Note that CPU time clocks are monotonic for their process/thread.
    pub fn is_monotonic(self) -> bool {
        matches!(
            self,
            Self::Monotonic
                | Self::MonotonicRaw
                | Self::MonotonicCoarse
                | Self::Boottime
                | Self::BoottimeAlarm
        )
    }

    /// Is this a realtime (wall clock) type? (represents calendar time)
    pub fn is_realtime(self) -> bool {
        matches!(self, Self::Realtime | Self::RealtimeCoarse | Self::RealtimeAlarm | Self::Tai)
    }

    /// Is this a CPU time measurement clock? (measures actual CPU usage)
    pub fn is_cpu_time(self) -> bool {
        matches!(self, Self::ProcessCpuTimeId | Self::ThreadCpuTimeId)
    }

    /// Is this an alarm clock? (for timerfd/signals, not settable)
    pub fn is_alarm(self) -> bool {
        matches!(self, Self::RealtimeAlarm | Self::BoottimeAlarm)
    }

    /// Does this clock include time during system suspend?
    pub fn includes_suspend_time(self) -> bool {
        matches!(self, Self::Boottime | Self::BoottimeAlarm)
    }
}

impl LinuxClock {
    /// Gets the current time for this clock
    pub fn get_time(self) -> Result<LinuxTimespec> {
        Linux::clock_gettime(self)
    }

    /// Gets the resolution for this clock
    pub fn get_resolution(self) -> Result<LinuxTimespec> {
        Linux::clock_getres(self)
    }

    /// Converts to the raw integer value for syscalls.
    pub const fn as_raw(self) -> c_int {
        self as c_int
    }
    /// Converts from a raw integer (for FFI or error handling).
    pub const fn from_raw(value: c_int) -> Option<Self> {
        match value {
            0 => Some(Self::Realtime),
            1 => Some(Self::Monotonic),
            2 => Some(Self::ProcessCpuTimeId),
            3 => Some(Self::ThreadCpuTimeId),
            4 => Some(Self::MonotonicRaw),
            5 => Some(Self::RealtimeCoarse),
            6 => Some(Self::MonotonicCoarse),
            7 => Some(Self::Boottime),
            8 => Some(Self::RealtimeAlarm),
            9 => Some(Self::BoottimeAlarm),
            11 => Some(Self::Tai),
            _ => None,
        }
    }
}

/// Traditional UPPERCASE constants with the same naming convention as Linux kernel headers.
#[allow(missing_docs)]
impl LinuxClock {
    pub const REALTIME: Self = Self::Realtime;
    pub const MONOTONIC: Self = Self::Monotonic;
    pub const PROCESS_CPUTIME_ID: Self = Self::ProcessCpuTimeId;
    pub const THREAD_CPUTIME_ID: Self = Self::ThreadCpuTimeId;
    pub const MONOTONIC_RAW: Self = Self::MonotonicRaw;
    pub const REALTIME_COARSE: Self = Self::RealtimeCoarse;
    pub const MONOTONIC_COARSE: Self = Self::MonotonicCoarse;
    pub const BOOTTIME: Self = Self::Boottime;
    pub const REALTIME_ALARM: Self = Self::RealtimeAlarm;
    pub const BOOTTIME_ALARM: Self = Self::BoottimeAlarm;
    // pub const SGI_CYCLE: Self = Self::SgiCycle;
    pub const TAI: Self = Self::Tai;
}
