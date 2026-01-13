// devela::phys::time::unix
//
// # LINKS
// - https://en.wikipedia.org/wiki/Unix_time
// - https://doc.rust-lang.org/std/time/struct.SystemTime.html
// - https://www.gnu.org/software/libc/manual/html_node/Getting-the-Time.html
// - https://www.gnu.org/software/libc/manual/html_node/Time-Functions-Example.html
//
//! Defines [`TimeUnixI64`], [`TimeUnixU32`].
//

use crate::{
    Debug, Display, FmtResult, Formatter, Month, TimeSplit, TimeSplitYearSec, TryFromIntError, is,
    is_leap_year,
};

#[doc = crate::_tags!(time)]
/// 64-bit Unix time, supporting negative values.
#[doc = crate::_doc_location!("phys/time")]
///
/// Stores the number of seconds relative to the Unix Epoch (`1970-01-01 00:00:00 UTC`).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeUnixI64 {
    /// The number of seconds relative the Unix Epoch.
    pub seconds: i64,
}

#[doc = crate::_tags!(time)]
/// 32-bit Unix time, supporting only non-negative values.
#[doc = crate::_doc_location!("phys/time")]
///
/// Stores the number of seconds since the Unix Epoch (`1970-01-01 00:00:00 UTC`).
///
/// It can represent time from `1970-01-01_00:00:00` to `2106-02-07_06:28:15`.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeUnixU32 {
    /// The number of seconds since the Unix Epoch.
    pub seconds: u32,
}

impl TimeUnixI64 {
    /// Returns a new `TimeUnixI64` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// # use devela::TimeUnixI64;
    /// assert_eq!["1970-01-01_00:00:01", TimeUnixI64::new(1).to_string()];
    /// assert_eq!["1969-12-31_23:59:59", TimeUnixI64::new(-1).to_string()];
    /// assert_eq!["2038-01-19_03:14:07", TimeUnixI64::new(i32::MAX as i64).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", TimeUnixI64::new(u32::MAX as i64).to_string()];
    /// assert_eq!["1833-11-24_17:31:45", TimeUnixI64::new(u32::MAX as i64 * -1).to_string()];
    /// ```
    pub const fn new(seconds: i64) -> Self {
        Self { seconds }
    }

    /// Returns a new `TimeUnixI64` anchored to the current second.
    #[cfg(feature = "std")]
    // all(not(feature = "std"), feature = "unsafe", feature = "libc")
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    // all(feature = "no_std", feature = "unsafe", feature = "libc")
    pub fn now() -> Self {
        Self { seconds: Self::unix_time_64() }
    }

    /// Splits the `TimeUnixI64` into `{ y, mo, d, h, m, s }`.
    ///
    /// # Examples
    /// ```
    /// # use devela::TimeUnixI64;
    /// assert_eq![(1970, 1, 1, 0, 0, 1), TimeUnixI64::new(1).split().to_tuple_y_s().unwrap()];
    /// assert_eq![(1969, 12, 31, 23, 59, 59),
    ///     TimeUnixI64::new(-1).split().to_tuple_y_s().unwrap()];
    /// ```
    // 72b
    pub const fn split(&self) -> TimeSplitYearSec<i32, u8, u8, u8, u8, u8> {
        let secs_per_min: u32 = 60;
        let mins_per_hour: u32 = 60;
        let hours_per_day: u32 = 24;
        let days_per_year: u32 = 365;

        let mut secs_left = self.seconds.abs();
        let mut year = if self.seconds >= 0 { 1970 } else { 1969 };
        let mut leap = is_leap_year(year);

        while secs_left >= (hours_per_day * mins_per_hour * secs_per_min * days_per_year) as i64 {
            leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            secs_left -= (hours_per_day * mins_per_hour * secs_per_min * days_in_year) as i64;
            is![self.seconds >= 0; year += 1; year -= 1];
        }
        let mut month = Month::January;
        while secs_left
            >= (hours_per_day * mins_per_hour * secs_per_min * month.len(leap) as u32) as i64
        {
            secs_left -=
                (hours_per_day * mins_per_hour * secs_per_min * month.len(leap) as u32) as i64;
            month = month.next();
        }

        let day = (secs_left / (hours_per_day * mins_per_hour * secs_per_min) as i64) as u8 + 1;
        secs_left %= (hours_per_day * mins_per_hour * secs_per_min) as i64;

        let hour = secs_left / (mins_per_hour * secs_per_min) as i64;
        secs_left %= (mins_per_hour * secs_per_min) as i64;

        let minute = secs_left / secs_per_min as i64;
        let second = secs_left % secs_per_min as i64;

        if self.seconds >= 0 {
            TimeSplit::new_year_sec(
                year,
                month.number(),
                day,
                hour as u8,
                minute as u8,
                second as u8,
            )
        } else {
            TimeSplit::new_year_sec(
                year,
                13 - month.number(),
                Month::December.previous_nth(month.index()).len(leap) - day + 1,
                23 - hour as u8,
                59 - minute as u8,
                60 - second as u8,
            )
        }
    }
}

// private functions
impl TimeUnixI64 {
    // Returns the number of seconds since `1970-01-01 00:00:00 UTC`.
    #[cfg(feature = "std")]
    fn unix_time_64() -> i64 {
        use crate::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .min(i64::MAX as u64) as i64
    }

    // // Returns the number of seconds since 1970-01-01 00:00:00 UTC.
    // //
    // // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2106`.
    // #[cfg(all(not(feature = "std"), feature = "unsafe", feature = "libc"))]
    // fn unix_time_64() -> i64 {
    //     // https://docs.rs/libc/latest/libc/fn.time.html
    //     #[allow(clippy::unnecessary_cast)] // could be i32 in other platforms?
    //     // SAFETY: safe since we pass a null pointer and do not dereference anything.
    //     unsafe {
    //         Libc::time(Ptr::null_mut()) as i64
    //     }
    // }
}

impl TimeUnixU32 {
    /// Returns a new `TimeUnixU32` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// # use devela::TimeUnixU32;
    /// assert_eq!["1970-01-01_00:00:00", TimeUnixU32::new(0).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", TimeUnixU32::new(u32::MAX).to_string()];
    /// ```
    pub const fn new(seconds: u32) -> Self {
        Self { seconds }
    }

    /// Returns a new `TimeUnixU32` anchored to the current second.
    #[cfg(feature = "std")]
    // all(not(feature = "std"), feature = "unsafe", feature = "libc")
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    // all(feature = "no_std", feature = "unsafe", feature = "libc")
    pub fn now() -> Self {
        Self { seconds: Self::unix_time_32() }
    }

    /// Splits the `TimeUnixU32` into `{ y, mo, d, h, m, s }`.
    ///
    /// # Examples
    /// ```
    /// # use devela::TimeUnixU32;
    /// assert_eq![(1970, 1, 1, 0, 0, 1), TimeUnixU32::new(1).split().to_tuple_y_s().unwrap()];
    /// assert_eq![(2038, 1, 19, 3, 14, 7),
    ///     TimeUnixU32::new(i32::MAX as u32).split().to_tuple_y_s().unwrap()];
    /// ```
    pub const fn split(&self) -> TimeSplitYearSec<u16, u8, u8, u8, u8, u8> {
        let secs_per_min: u32 = 60;
        let mins_per_hour: u32 = 60;
        let hours_per_day: u32 = 24;
        let days_per_year: u32 = 365;

        let mut secs_left = self.seconds;
        let mut year = 1970;
        let mut leap = is_leap_year(year);

        while secs_left >= (hours_per_day * mins_per_hour * secs_per_min * days_per_year) {
            year += 1;
            leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            secs_left -= hours_per_day * mins_per_hour * secs_per_min * days_in_year;
        }

        let mut month = Month::January;
        while secs_left >= hours_per_day * mins_per_hour * secs_per_min * month.len(leap) as u32 {
            secs_left -= hours_per_day * mins_per_hour * secs_per_min * month.len(leap) as u32;
            month = month.next();
        }

        let day = (secs_left / (hours_per_day * mins_per_hour * secs_per_min)) + 1;
        secs_left %= hours_per_day * mins_per_hour * secs_per_min;

        let hour = secs_left / (mins_per_hour * secs_per_min);
        secs_left %= mins_per_hour * secs_per_min;

        let minute = secs_left / secs_per_min;
        let second = secs_left % secs_per_min;

        TimeSplit::new_year_sec(
            year as u16,
            month.number(),
            day as u8,
            hour as u8,
            minute as u8,
            second as u8,
        )
    }
}

// private functions
impl TimeUnixU32 {
    // Returns the number of seconds since `1970-01-01 00:00:00 UTC`.
    //
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2106`.
    #[cfg(feature = "std")]
    fn unix_time_32() -> u32 {
        use crate::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .min(u32::MAX as u64) as u32
    }

    // // Returns the number of seconds since 1970-01-01 00:00:00 UTC.
    // //
    // // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2106`.
    // #[cfg(all(not(feature = "std"), feature = "unsafe", feature = "libc"))]
    // fn unix_time_32() -> u32 {
    //     // SAFETY: safe since we pass a null pointer and do not dereference anything.
    //     unsafe { libc::time(core::ptr::null_mut()).clamp(0, u32::MAX as i64) as u32 }
    // }
}

impl Display for TimeUnixI64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "{y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02}"]
    }
}
impl Debug for TimeUnixI64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "TimeUnixI64 {{ {y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02} }}"]
    }
}
impl Display for TimeUnixU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "{y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02}"]
    }
}
impl Debug for TimeUnixU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "TimeUnixU32 {{ {y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02} }}"]
    }
}

impl From<TimeUnixU32> for TimeUnixI64 {
    fn from(ut: TimeUnixU32) -> TimeUnixI64 {
        TimeUnixI64 { seconds: ut.seconds.into() }
    }
}
impl TryFrom<TimeUnixI64> for TimeUnixU32 {
    type Error = TryFromIntError;

    fn try_from(ut: TimeUnixI64) -> Result<TimeUnixU32, Self::Error> {
        Ok(TimeUnixU32 { seconds: u32::try_from(ut.seconds)? })
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use crate::{CapacityMismatch, Cast, TimeUnixU32};
    use crate::{SystemTime, SystemTimeError, TimeUnixI64};

    impl TryFrom<SystemTime> for TimeUnixI64 {
        type Error = SystemTimeError;

        fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
            let duration = time.duration_since(SystemTime::UNIX_EPOCH)?;
            Ok(TimeUnixI64 {
                seconds: duration.as_secs() as i64, // Assuming the range is acceptable for i64
            })
        }
    }

    impl TryFrom<SystemTime> for TimeUnixU32 {
        type Error = crate::TimeError;

        fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
            let since = time.duration_since(SystemTime::UNIX_EPOCH)?;
            let seconds = u32::try_from(since.as_secs()).map_err(|_| {
                CapacityMismatch::too_large_try(
                    Cast(since.as_secs()).checked_cast_to_usize(),
                    u32::MAX as usize,
                )
            })?;
            Ok(TimeUnixU32 { seconds })
        }
    }

    // impl From<SystemTime> for TimeUnixI64 {
    //     fn from(time: SystemTime) -> Self {
    //         match time.duration_since(SystemTime::UNIX_EPOCH) {
    //             Ok(duration) => TimeUnixI64 { seconds: duration.as_secs() as i64 },
    //             Err(e) => TimeUnixI64 { seconds: -(e.duration().as_secs() as i64) },
    //         }
    //     }
    // }
    // impl From<SystemTime> for TimeUnixU32 {
    //     fn from(time: SystemTime) -> Self {
    //         let duration = time.duration_since(SystemTime::UNIX_EPOCH)
    //             .expect("Time is before Unix epoch, which is unsupported for `TimeUnixU32`.");
    //         TimeUnixU32 {
    //             seconds: duration.as_secs() as u32, // Safe cast, as u32 covers the valid range for TimeUnixU32
    //         }
    //     }
    // }
}

// Implements From<primitive> for TimeUnix*
macro_rules! impl_from_prim {
    // for many
    ($ut:ty, $($prim:ty),+) => { $( impl_from_prim![@ $ut, $prim]; )+ };
    (@ $ut:ty, $prim:ty) => {
        impl From<$prim> for $ut {
            fn from(seconds: $prim) -> $ut {
                Self { seconds: seconds.into() }
            }
        }
    };
}
impl_from_prim![TimeUnixI64, i64, i32, i16, i8, u32, u16, u8];
impl_from_prim![TimeUnixU32, u32, u16, u8];

// Implements TryFrom<primitive> for TimeUnix*
macro_rules! impl_try_from_prim {
    ($ut:ty, $($prim:ty),+) => { $( impl_try_from_prim![@ $ut, $prim]; )+ };
    (@ $ut:ty, $prim:ty) => {
        impl TryFrom<$prim> for $ut {
            type Error = TryFromIntError;
            fn try_from(seconds: $prim) -> Result<$ut, Self::Error> {
                Ok(Self { seconds: seconds.try_into()? })
            }
        }
    };
}
impl_try_from_prim![TimeUnixI64, u64, u128, usize, i128, isize];
#[rustfmt::skip]
impl_try_from_prim![TimeUnixU32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
