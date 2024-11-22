// devela::sys::os::linux::structs::timespec

use crate::Duration;

/// Represents the [`timespec`] structure from libc.
/// Time in seconds and nanoseconds.
///
/// [`timespec`]: https://man7.org/linux/man-pages/man3/timespec.3type.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct LinuxTimespec {
    /// Number of whole seconds.
    pub tv_sec: isize,
    /// Number of nanoseconds.
    pub tv_nsec: isize,
}

impl LinuxTimespec {
    /// Returns a new `LinuxTimespec` with the given `seconds` and `nanoseconds`.
    #[must_use]
    pub const fn new(seconds: isize, nanoseconds: isize) -> Self {
        Self { tv_sec: seconds, tv_nsec: nanoseconds }
    }

    /// Returns a new `LinuxTimespec` with the given `duration`.
    #[must_use]
    pub const fn with(duration: Duration) -> Self {
        Self {
            tv_sec: duration.as_secs() as isize,
            tv_nsec: duration.subsec_nanos() as isize,
        }
    }

    /// Returns a raw pointer to self.
    #[must_use]
    pub const fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /// Returns a raw mutable pointer to self.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}

impl From<Duration> for LinuxTimespec {
    #[must_use]
    fn from(duration: Duration) -> Self {
        Self::with(duration)
    }
}
