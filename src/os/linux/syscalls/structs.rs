// devela::os::linux::syscall::structs
//
//!
//

use core::time::Duration;

/// Represents a time interval measured in seconds and nanoseconds.
#[derive(Clone, Copy)]
#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
pub struct timespec {
    /// Number of whole seconds.
    pub tv_sec: isize,
    /// Number of nanoseconds.
    pub tv_nsec: isize,
}
impl timespec {
    /// Returns a new empty `timespec`.
    pub const fn new() -> Self {
        timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }

    /// Returns a new `timespec` with the given `duration`.
    pub const fn with(duration: Duration) -> Self {
        timespec {
            tv_sec: duration.as_secs() as isize,
            tv_nsec: duration.subsec_nanos() as isize,
        }
    }
}

impl Default for timespec {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Duration> for timespec {
    fn from(duration: Duration) -> Self {
        Self::with(duration)
    }
}
