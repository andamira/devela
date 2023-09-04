// devela::os::linux::syscall::structs
//
//!
//

use core::time::Duration;

/// Represents a time interval measured in seconds and nanoseconds.
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[repr(C)]
pub struct SysTimeSpec {
    /// Number of whole seconds.
    pub tv_sec: isize,
    /// Number of nanoseconds.
    pub tv_nsec: isize,
}
impl SysTimeSpec {
    /// Returns a new empty `SysTimeSpec`.
    pub const fn new() -> Self {
        Self {
            tv_sec: 0,
            tv_nsec: 0,
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

impl Default for SysTimeSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Duration> for SysTimeSpec {
    fn from(duration: Duration) -> Self {
        Self::with(duration)
    }
}
