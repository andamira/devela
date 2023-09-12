// devela::os::linux::syscall::structs
//
//!
//

use core::time::Duration;

/// Represents a time interval measured in seconds and nanoseconds.
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
#[repr(C)]
pub struct SysTimeSpec {
    /// Number of whole seconds.
    pub tv_sec: isize,
    /// Number of nanoseconds.
    pub tv_nsec: isize,
}
impl SysTimeSpec {
    /// Returns a new `SysTimeSpec` with the given `seconds` and `nanoseconds`.
    pub const fn new(seconds: isize, nanoseconds: isize) -> Self {
        Self {
            tv_sec: seconds,
            tv_nsec: nanoseconds,
        }
    }

    /// Returns a new `SysTimeSpec` with the given `duration`.
    pub const fn with(duration: Duration) -> Self {
        Self {
            tv_sec: duration.as_secs() as isize,
            tv_nsec: duration.subsec_nanos() as isize,
        }
    }
}
impl From<Duration> for SysTimeSpec {
    fn from(duration: Duration) -> Self {
        Self::with(duration)
    }
}
