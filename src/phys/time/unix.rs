// devela::phys::time::unix
//
// # LINKS
// - https://en.wikipedia.org/wiki/Unix_time
// - https://doc.rust-lang.org/std/time/struct.SystemTime.html
// - https://www.gnu.org/software/libc/manual/html_node/Getting-the-Time.html
// - https://www.gnu.org/software/libc/manual/html_node/Time-Functions-Example.html
//
//! Unix time.
//

use crate::{
    Debug, Display, FmtResult, Formatter, Month, TimeSplit, TimeSplitYearSec, TryFromIntError,
    is_leap_year,
};

#[doc = crate::TAG_TIME!()]
/// 64-bit Unix time, supporting negative values.
///
/// Stores the number of seconds relative to the Unix Epoch (`1970-01-01 00:00:00 UTC`).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTimeI64 {
    /// The number of seconds relative the Unix Epoch.
    pub seconds: i64,
}

#[doc = crate::TAG_TIME!()]
/// 32-bit Unix time, supporting only non-negative values.
///
/// Stores the number of seconds since the Unix Epoch (`1970-01-01 00:00:00 UTC`).
///
/// It can represent time from `1970-01-01_00:00:00` to `2106-02-07_06:28:15`.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTimeU32 {
    /// The number of seconds since the Unix Epoch.
    pub seconds: u32,
}

impl UnixTimeI64 {
    /// Returns a new `UnixTimeI64` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// # use devela::UnixTimeI64;
    /// assert_eq!["1970-01-01_00:00:01", UnixTimeI64::new(1).to_string()];
    /// assert_eq!["1969-12-31_23:59:59", UnixTimeI64::new(-1).to_string()];
    /// assert_eq!["2038-01-19_03:14:07", UnixTimeI64::new(i32::MAX as i64).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", UnixTimeI64::new(u32::MAX as i64).to_string()];
    /// assert_eq!["1833-11-24_17:31:45", UnixTimeI64::new(u32::MAX as i64 * -1).to_string()];
    /// ```
    pub fn new(seconds: i64) -> Self {
        Self { seconds }
    }

    /// Returns a new `UnixTimeI64` anchored to the current second.
    #[cfg(feature = "std")]
    // all(not(feature = "std"), feature = "unsafe", feature = "libc")
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    // all(feature = "no_std", feature = "unsafe", feature = "libc")
    pub fn now() -> Self {
        Self { seconds: Self::unix_time_64() }
    }

    /// Splits the `UnixTimeI64` into `{ y, mo, d, h, m, s }`.
    ///
    /// # Examples
    /// ```
    /// # use devela::UnixTimeI64;
    /// assert_eq![(1970, 1, 1, 0, 0, 1), UnixTimeI64::new(1).split().to_tuple_y_s().unwrap()];
    /// assert_eq![(1969, 12, 31, 23, 59, 59),
    ///     UnixTimeI64::new(-1).split().to_tuple_y_s().unwrap()];
    /// ```
    // 72b
    pub const fn split(&self) -> TimeSplitYearSec<i32, u8, u8, u8, u8, u8> {
        let seconds_per_minute: u32 = 60;
        let minutes_per_hour: u32 = 60;
        let hours_per_day: u32 = 24;
        let days_per_year: u32 = 365;

        let mut seconds_left = self.seconds.abs();
        let mut year = if self.seconds >= 0 { 1970 } else { 1969 };
        let mut leap = is_leap_year(year);

        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * days_per_year) as i64
        {
            leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            seconds_left -=
                (hours_per_day * minutes_per_hour * seconds_per_minute * days_in_year) as i64;

            if self.seconds >= 0 {
                year += 1;
            } else {
                year -= 1;
            }
        }

        let mut month = Month::January;
        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32)
                as i64
        {
            seconds_left -=
                (hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32)
                    as i64;
            month = month.next();
        }

        let day = (seconds_left / (hours_per_day * minutes_per_hour * seconds_per_minute) as i64)
            as u8
            + 1;
        seconds_left %= (hours_per_day * minutes_per_hour * seconds_per_minute) as i64;

        let hour = seconds_left / (minutes_per_hour * seconds_per_minute) as i64;
        seconds_left %= (minutes_per_hour * seconds_per_minute) as i64;

        let minute = seconds_left / seconds_per_minute as i64;
        let second = seconds_left % seconds_per_minute as i64;

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
impl UnixTimeI64 {
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
    //         libc::time(core::ptr::null_mut()) as i64
    //     }
    // }
}

impl UnixTimeU32 {
    /// Returns a new `UnixTimeU32` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// # use devela::UnixTimeU32;
    /// assert_eq!["1970-01-01_00:00:00", UnixTimeU32::new(0).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", UnixTimeU32::new(u32::MAX).to_string()];
    /// ```
    pub fn new(seconds: u32) -> Self {
        Self { seconds }
    }

    /// Returns a new `UnixTimeU32` anchored to the current second.
    #[cfg(feature = "std")]
    // all(not(feature = "std"), feature = "unsafe", feature = "libc")
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    // all(feature = "no_std", feature = "unsafe", feature = "libc")
    pub fn now() -> Self {
        Self { seconds: Self::unix_time_32() }
    }

    /// Splits the `UnixTimeU32` into `{ y, mo, d, h, m, s }`.
    ///
    /// # Examples
    /// ```
    /// # use devela::UnixTimeU32;
    /// assert_eq![(1970, 1, 1, 0, 0, 1), UnixTimeU32::new(1).split().to_tuple_y_s().unwrap()];
    /// assert_eq![(2038, 1, 19, 3, 14, 7),
    ///     UnixTimeU32::new(i32::MAX as u32).split().to_tuple_y_s().unwrap()];
    /// ```
    pub const fn split(&self) -> TimeSplitYearSec<u16, u8, u8, u8, u8, u8> {
        let seconds_per_minute: u32 = 60;
        let minutes_per_hour: u32 = 60;
        let hours_per_day: u32 = 24;
        let days_per_year: u32 = 365;

        let mut seconds_left = self.seconds;
        let mut year = 1970;
        let mut leap = is_leap_year(year);

        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * days_per_year)
        {
            year += 1;
            leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            seconds_left -= hours_per_day * minutes_per_hour * seconds_per_minute * days_in_year;
        }

        let mut month = Month::January;
        while seconds_left
            >= hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32
        {
            seconds_left -=
                hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32;
            month = month.next();
        }

        let day = (seconds_left / (hours_per_day * minutes_per_hour * seconds_per_minute)) + 1;
        seconds_left %= hours_per_day * minutes_per_hour * seconds_per_minute;

        let hour = seconds_left / (minutes_per_hour * seconds_per_minute);
        seconds_left %= minutes_per_hour * seconds_per_minute;

        let minute = seconds_left / seconds_per_minute;
        let second = seconds_left % seconds_per_minute;

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
impl UnixTimeU32 {
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

impl Display for UnixTimeI64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "{y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02}"]
    }
}
impl Debug for UnixTimeI64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "UnixTimeI64 {{ {y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02} }}"]
    }
}

impl Display for UnixTimeU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "{y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02}"]
    }
}

impl Debug for UnixTimeU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let TimeSplit { y, mo, d, h, m, s, .. } = self.split();
        write![f, "UnixTimeU32 {{ {y:04}-{mo:02}-{d:02}_{h:02}:{m:02}:{s:02} }}"]
    }
}

impl From<UnixTimeU32> for UnixTimeI64 {
    fn from(ut: UnixTimeU32) -> UnixTimeI64 {
        UnixTimeI64 { seconds: ut.seconds.into() }
    }
}

impl TryFrom<UnixTimeI64> for UnixTimeU32 {
    type Error = TryFromIntError;

    fn try_from(ut: UnixTimeI64) -> Result<UnixTimeU32, Self::Error> {
        Ok(UnixTimeU32 { seconds: u32::try_from(ut.seconds)? })
    }
}

#[cfg(feature = "std")]
mod std_impls {
    #[cfg(feature = "error")]
    use crate::{Cast, DataOverflow, UnixTimeU32};
    use crate::{SystemTime, SystemTimeError, UnixTimeI64};

    impl TryFrom<SystemTime> for UnixTimeI64 {
        type Error = SystemTimeError;

        fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
            let duration = time.duration_since(SystemTime::UNIX_EPOCH)?;
            Ok(UnixTimeI64 {
                seconds: duration.as_secs() as i64, // Assuming the range is acceptable for i64
            })
        }
    }

    #[cfg(feature = "error")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "error")))]
    impl TryFrom<SystemTime> for UnixTimeU32 {
        type Error = crate::TimeError;

        fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
            let since = time.duration_since(SystemTime::UNIX_EPOCH)?;
            let seconds = u32::try_from(since.as_secs())
                .map_err(|_| DataOverflow(Cast(since.as_secs()).checked_cast_to_usize().ok()))?;
            Ok(UnixTimeU32 { seconds })
        }
    }

    // impl From<SystemTime> for UnixTimeI64 {
    //     fn from(time: SystemTime) -> Self {
    //         match time.duration_since(SystemTime::UNIX_EPOCH) {
    //             Ok(duration) => UnixTimeI64 { seconds: duration.as_secs() as i64 },
    //             Err(e) => UnixTimeI64 { seconds: -(e.duration().as_secs() as i64) },
    //         }
    //     }
    // }
    // impl From<SystemTime> for UnixTimeU32 {
    //     fn from(time: SystemTime) -> Self {
    //         let duration = time.duration_since(SystemTime::UNIX_EPOCH)
    //             .expect("Time is before Unix epoch, which is unsupported for `UnixTimeU32`.");
    //         UnixTimeU32 {
    //             seconds: duration.as_secs() as u32, // Safe cast, as u32 covers the valid range for UnixTimeU32
    //         }
    //     }
    // }
}

// Implements From<primitive> for UnixTime*
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
impl_from_prim![UnixTimeI64, i64, i32, i16, i8, u32, u16, u8];
impl_from_prim![UnixTimeU32, u32, u16, u8];

// Implements TryFrom<primitive> for UnixTime*
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
impl_try_from_prim![UnixTimeI64, u64, u128, usize, i128, isize];
#[rustfmt::skip]
impl_try_from_prim![UnixTimeU32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
