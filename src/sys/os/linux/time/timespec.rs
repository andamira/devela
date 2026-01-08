// devela::sys::os::linux::time::timespec
//
//! Time value representation.
//!
//! Defines the [`LinuxTimespec`] structure used to represent time values
//! with second and nanosecond precision.
//

#![cfg_attr(not(feature = "unsafe_syscall"), allow(dead_code))]

use crate::{
    Cast, ConstInit, Display, Duration, FmtResult, Formatter, Overflow, format_buf, unwrap,
};

use crate::TimeDelta;

#[doc = crate::_tags!(linux time)]
/// Represents the [`timespec`] structure from libc. Time in seconds and nanoseconds.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// [`timespec`]: https://man7.org/linux/man-pages/man3/timespec.3type.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
#[must_use]
pub struct LinuxTimespec {
    /// Number of whole seconds.
    pub tv_sec: isize,
    /// Number of nanoseconds.
    pub tv_nsec: isize,
}
impl ConstInit for LinuxTimespec {
    const INIT: Self = Self::new(0, 0);
}
impl Display for LinuxTimespec {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        // write!(f, "{}s {}ns", self.tv_sec, self.tv_nsec)
        let mut buf = [0u8; 64];
        let s = format_buf![&mut buf, "{}s {}ns", self.tv_sec, self.tv_nsec].unwrap();
        f.write_str(s)
    }
}

impl LinuxTimespec {
    /// Returns a new `LinuxTimespec` with the given `seconds` and `nanoseconds`.
    pub const fn new(seconds: isize, nanoseconds: isize) -> Self {
        Self { tv_sec: seconds, tv_nsec: nanoseconds }
    }

    /// Returns a new `LinuxTimespec` with the given `duration`.
    pub const fn try_with_duration(duration: Duration) -> Result<Self, Overflow> {
        Ok(Self {
            tv_sec: unwrap![ok? Cast(duration.as_secs()).checked_cast_to_isize()],
            tv_nsec: unwrap![ok? Cast(duration.subsec_nanos()).checked_cast_to_isize()],
        })
    }
    /// Returns a new `LinuxTimespec` with the given `duration`.
    pub const fn with_saturating_duration(duration: Duration) -> Self {
        Self {
            tv_sec: duration.as_secs() as isize,
            tv_nsec: duration.subsec_nanos() as isize,
        }
    }

    /// Returns a new `LinuxTimespec` with the given `time_delta`.
    pub const fn try_with_time_delta(time_delta: TimeDelta) -> Result<Self, Overflow> {
        Ok(Self {
            tv_sec: unwrap![ok? Cast(time_delta.as_secs()).checked_cast_to_isize()],
            tv_nsec: unwrap![ok? Cast(time_delta.subsec_nanos()).checked_cast_to_isize()],
        })
    }
    /// Returns a new `LinuxTimespec` with the given `time_delta`.
    pub const fn with_saturating_time_delta(time_delta: TimeDelta) -> Self {
        Self {
            tv_sec: time_delta.as_secs() as isize,
            tv_nsec: time_delta.subsec_nanos() as isize,
        }
    }

    /// Returns a raw pointer to self.
    pub const fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /// Returns a raw mutable pointer to self.
    pub const fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    /// Converts to a `Duration`.
    pub const fn try_to_duration(&self) -> Result<Duration, Overflow> {
        Ok(Duration::new(
            unwrap![ok? Cast(self.tv_sec).checked_cast_to_u64()],
            unwrap![ok? Cast(self.tv_nsec).checked_cast_to_u32()],
        ))
    }
    #[must_use]
    /// Converts to a `Duration`, saturating on overflow.
    pub const fn to_saturating_duration(&self) -> Duration {
        Duration::new(
            Cast(self.tv_sec).saturating_cast_to_u64(),
            Cast(self.tv_nsec).saturating_cast_to_u32(),
        )
    }

    /// Converts to a `TimeDelta`.
    pub const fn try_to_time_delta(&self) -> Result<TimeDelta, Overflow> {
        Ok(TimeDelta::new(
            unwrap![ok? Cast(self.tv_sec).checked_cast_to_i64()],
            unwrap![ok? Cast(self.tv_nsec).checked_cast_to_i32()],
        ))
    }
    #[must_use]
    /// Converts to a `TimeDelta`, saturating on overflow.
    pub const fn to_saturating_time_delta(&self) -> TimeDelta {
        TimeDelta::new(
            Cast(self.tv_sec).saturating_cast_to_i64(),
            Cast(self.tv_nsec).saturating_cast_to_i32(),
        )
    }
}

impl TryFrom<Duration> for LinuxTimespec {
    type Error = Overflow;
    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        Self::try_with_duration(duration)
    }
}
impl TryFrom<LinuxTimespec> for Duration {
    type Error = Overflow;
    fn try_from(timespec: LinuxTimespec) -> Result<Self, Self::Error> {
        timespec.try_to_duration()
    }
}
impl TryFrom<TimeDelta> for LinuxTimespec {
    type Error = Overflow;
    fn try_from(time_delta: TimeDelta) -> Result<Self, Self::Error> {
        Self::try_with_time_delta(time_delta)
    }
}
impl TryFrom<LinuxTimespec> for TimeDelta {
    type Error = Overflow;
    fn try_from(timespec: LinuxTimespec) -> Result<Self, Self::Error> {
        timespec.try_to_time_delta()
    }
}
