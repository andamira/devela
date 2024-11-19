// devela::sys::time::split
//
//! Splitting and decomposing time.
//

use crate::Duration;

/// A full time split from years, down to nanoseconds.
///
/// See also the related aliases:
/// - [`TimeSplitYearNano`]`/`[`Norm`][TimeSplitYearNanoNorm],
///   [`TimeSplitYearDay`]`/`[`Norm`][TimeSplitYearDayNorm],
///   [`TimeSplitYearSec`]`/`[`Norm`][TimeSplitYearSecNorm],
///   [`TimeSplitHourSec`]`/`[`Norm`][TimeSplitHourSecNorm],
///   [`TimeSplitHourNano`]`/`[`Norm`][TimeSplitHourNanoNorm],
///   [`TimeSplitMilliNano`]`/`[`Norm`][TimeSplitMilliNanoNorm].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplit<Y, MO, D, H, M, S, MS, US, NS> {
    /// Years.
    pub y: Y,
    /// Months.
    pub mo: MO,
    /// Days.
    pub d: D,
    /// Hours.
    pub h: H,
    /// Minutes.
    pub m: M,
    /// Seconds.
    pub s: S,
    /// Milliseconds.
    pub ms: MS,
    /// Microseconds.
    pub us: US,
    /// Nanoseconds.
    pub ns: NS,
}

/* aliases */

/// A time split from years to nanoseconds.
pub type TimeSplitYearNano<Y, MO, D, H, M, S, MS, US, NS> =
    TimeSplit<Y, MO, D, H, M, S, MS, US, NS>;
/// A time split from years to nanoseconds, normalized *(192b size, 152b payload)*.
pub type TimeSplitYearNanoNorm = TimeSplit<u64, u8, u8, u8, u8, u8, u16, u16, u16>;

/// A time split from years to days.
pub type TimeSplitYearDay<Y, MO, D> = TimeSplit<Y, MO, D, (), (), (), (), (), ()>;
/// A time split from years to days, normalized *(128b size, 80b payload)*.
pub type TimeSplitYearDayNorm = TimeSplit<u64, u8, u8, (), (), (), (), (), ()>;

/// A time split from years to seconds.
pub type TimeSplitYearSec<Y, MO, D, H, M, S> = TimeSplit<Y, MO, D, H, M, S, (), (), ()>;
/// A time split from years to seconds, normalized *(128b size, 104b payload)*.
pub type TimeSplitYearSecNorm = TimeSplit<u64, u8, u8, u8, u8, u8, (), (), ()>;

/// A time split from hours to seconds.
pub type TimeSplitHourSec<H, M, S> = TimeSplit<(), (), (), H, M, S, (), (), ()>;
/// A time split from hours to seconds, normalized *(128b size, 80b payload)*.
pub type TimeSplitHourSecNorm = TimeSplit<(), (), (), u64, u8, u8, (), (), ()>;

/// A time split from hours to nanoseconds.
pub type TimeSplitHourNano<H, M, S, MS, US, NS> = TimeSplit<(), (), (), H, M, S, MS, US, NS>;
/// A time split from hours to seconds, normalized *(128b size & payload)*.
pub type TimeSplitHourNanoNorm = TimeSplit<(), (), (), u64, u8, u8, u16, u16, u16>;

/// A time split from milliseconds to nanoseconds.
pub type TimeSplitMilliNano<MS, US, NS> = TimeSplit<(), (), (), (), (), (), MS, US, NS>;
/// A time split from milliseconds to nanoseconds, normalized *(48b size & payload)*.
pub type TimeSplitMilliNanoNorm = TimeSplit<(), (), (), (), (), (), u16, u16, u16>;

/* constructors */

impl<Y, MO, D, H, M, S, MS, US, NS> TimeSplit<Y, MO, D, H, M, S, MS, US, NS> {
    /// Returns a new [`TimeSplit`].
    #[allow(clippy::too_many_arguments)]
    pub const fn new(y: Y, mo: MO, d: D, h: H, m: M, s: S, ms: MS, us: US, ns: NS) -> Self {
        Self { y, mo, d, h, m, s, ms, us, ns }
    }

    /// Aalias of [`Self::new`]. Returns a new [`TimeSplitYearNano`].
    #[allow(clippy::too_many_arguments)]
    pub const fn new_year_nano(
        y: Y,
        mo: MO,
        d: D,
        h: H,
        m: M,
        s: S,
        ms: MS,
        us: US,
        ns: NS,
    ) -> Self {
        Self::new(y, mo, d, h, m, s, ms, us, ns)
    }
}
impl TimeSplitYearNanoNorm {
    /// Converts a `Duration` into a [`TimeSplitYearNano`].
    ///
    /// It assumes non-leap years and 30-day months for simplicity in calendar calculations.
    pub const fn from_duration(duration: Duration) -> Self {
        let secs = duration.as_secs();
        let total_days = secs / 86400;
        let y = total_days / 365;
        let days_remaining = total_days % 365;
        let mo = (days_remaining / 30) as u8;
        let d = (days_remaining % 30) as u8;

        let h = ((secs % 86400) / 3600) as u8;
        let m = ((secs % 3600) / 60) as u8;
        let s = (secs % 60) as u8;

        let nanos = duration.subsec_nanos();
        let ms = (nanos / 1_000_000) as u16;
        let us = ((nanos % 1_000_000) / 1_000) as u16;
        let ns = (nanos % 1_000) as u16;

        TimeSplitYearNano { y, mo, d, h, m, s, ms, us, ns }
    }
}

impl<Y, MO, D> TimeSplitYearDay<Y, MO, D> {
    /// Returns a new [`TimeSplitYearDay`].
    pub const fn new_year_day(y: Y, mo: MO, d: D) -> Self {
        Self::new(y, mo, d, (), (), (), (), (), ())
    }
}
impl TimeSplitYearDayNorm {
    /// Converts a `Duration` into a [`TimeSplitYearDay`].
    ///
    /// This method assumes 365 days per year and 30 days per month for simplicity.
    pub const fn from_duration(duration: Duration) -> Self {
        let days = duration.as_secs() / 86400;
        let y = days / 365;
        let days_rem = days % 365;
        let mo = (days_rem / 30) as u8;
        let d = (days_rem % 30) as u8;

        Self::new_year_day(y, mo, d)
    }
}

impl<Y, MO, D, H, M, S> TimeSplitYearSec<Y, MO, D, H, M, S> {
    /// Returns a new [`TimeSplitYearSec`].
    pub const fn new_year_sec(y: Y, mo: MO, d: D, h: H, m: M, s: S) -> Self {
        Self::new(y, mo, d, h, m, s, (), (), ())
    }
}
impl TimeSplitYearSecNorm {
    /// Converts a `Duration` into a [`TimeSplitYearSec`].
    ///
    /// It assumes non-leap years and 30-day months for simplicity in calendar calculations.
    pub const fn from_duration(duration: Duration) -> Self {
        let secs = duration.as_secs();
        let total_days = secs / 86400;
        let y = total_days / 365;
        let days_remaining = total_days % 365;
        let mo = (days_remaining / 30) as u8;
        let d = (days_remaining % 30) as u8;

        let h = ((secs % 86400) / 3600) as u8;
        let m = ((secs % 3600) / 60) as u8;
        let s = (secs % 60) as u8;

        Self::new_year_sec(y, mo, d, h, m, s)
    }
}

impl<H, M, S> TimeSplitHourSec<H, M, S> {
    /// Returns a new [`TimeSplitHourSec`].
    pub const fn new_hour_sec(h: H, m: M, s: S) -> Self {
        Self::new((), (), (), h, m, s, (), (), ())
    }
}
impl TimeSplitHourSecNorm {
    /// Converts a `Duration` into a [`TimeSplitHourSec`].
    ///
    /// Excess days or longer periods are converted into additional hours.
    pub const fn from_duration(duration: Duration) -> Self {
        let secs = duration.as_secs();
        let h = secs / 3600;
        let m = ((secs % 3600) / 60) as u8;
        let s = (secs % 60) as u8;
        Self::new_hour_sec(h, m, s)
    }
}

impl<MS, US, NS> TimeSplitMilliNano<MS, US, NS> {
    /// Returns a new [`TimeSplitMilliNano`].
    pub const fn new_milli_nano(ms: MS, us: US, ns: NS) -> Self {
        Self::new((), (), (), (), (), (), ms, us, ns)
    }
}
impl TimeSplitMilliNanoNorm {
    /// Converts a `Duration`'s sub-second component into a compact time split representation.
    ///
    /// Extracts and segments the nanosecond portion of a `Duration`
    /// into milliseconds, microseconds, and nanoseconds.
    pub const fn from_duration(duration: Duration) -> Self {
        let nanos = duration.subsec_nanos();
        let ms = (nanos / 1_000_000) as u16;
        let us = ((nanos % 1_000_000) / 1_000) as u16;
        let ns = (nanos % 1_000) as u16;
        Self::new_milli_nano(ms, us, ns)
    }
}

impl<H, M, S, MS, US, NS> TimeSplitHourNano<H, M, S, MS, US, NS> {
    /// Returns a new `TimeSplitHourNano`.
    pub const fn new_hour_nano(h: H, m: M, s: S, ms: MS, us: US, ns: NS) -> Self {
        Self::new((), (), (), h, m, s, ms, us, ns)
    }
}
impl TimeSplitHourNano<u64, u8, u8, u16, u16, u16> {
    /// Converts a `Duration` into a time split representation from Hours down to nanoseconds.
    pub const fn from_duration(duration: Duration) -> Self {
        let secs = duration.as_secs();
        let h = (secs % 86400) / 3600;
        let m = ((secs % 3600) / 60) as u8;
        let s = (secs % 60) as u8;

        let nanos = duration.subsec_nanos();
        let ms = (nanos / 1_000_000) as u16;
        let us = ((nanos % 1_000_000) / 1_000) as u16;
        let ns = (nanos % 1_000) as u16;

        Self::new_hour_nano(h, m, s, ms, us, ns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_size() {
        assert_eq!(0, size_of::<TimeSplit<(), (), (), (), (), (), (), (), ()>>());

        /* normalized inner reprs */

        assert_eq!(16, size_of::<TimeSplitYearDayNorm>()); // 6 bytes padded
        assert_eq![16, size_of::<TimeSplitHourSecNorm>()]; // 6 bytes padded
        assert_eq![6, size_of::<TimeSplitMilliNanoNorm>()]; // 0 padding
        assert_eq!(24, size_of::<TimeSplitYearNanoNorm>()); // 5 bytes padded:
        assert_eq!(16, size_of::<TimeSplitYearSecNorm>()); // 3 bytes padded
        assert_eq!(16, size_of::<TimeSplitHourNanoNorm>()); // 0 padding
    }
}
