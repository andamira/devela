use core::time::Duration;

/// Represents the [`timespec`] structure from libc.
/// Time in seconds and nanoseconds.
///
/// [`timespec`]: https://man7.org/linux/man-pages/man3/timespec.3type.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SysTimespec {
    /// Number of whole seconds.
    pub tv_sec: isize,
    /// Number of nanoseconds.
    pub tv_nsec: isize,
}

impl SysTimespec {
    /// Returns a new `SysTimespec` with the given `seconds` and `nanoseconds`.
    pub const fn new(seconds: isize, nanoseconds: isize) -> Self {
        Self {
            tv_sec: seconds,
            tv_nsec: nanoseconds,
        }
    }

    /// Returns a new `SysTimespec` with the given `duration`.
    pub const fn with(duration: Duration) -> Self {
        Self {
            tv_sec: duration.as_secs() as isize,
            tv_nsec: duration.subsec_nanos() as isize,
        }
    }

    /// Returns a raw pointer to self.
    pub const fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /// Returns a raw mutable pointer to self.
    pub fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}

impl From<Duration> for SysTimespec {
    fn from(duration: Duration) -> Self {
        Self::with(duration)
    }
}
