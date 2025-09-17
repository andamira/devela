// devela::phys::time::split
//
//! Splitting and decomposing time.
//

use crate::{Duration, ExtAny};
use ::core::fmt;

#[doc = crate::_TAG_TIME!()]
/// A full time split from years to nanoseconds.
///
/// See also the related aliases:
/// - [`TimeSplitNorm`][TimeSplitNorm],
///   [`TimeSplitYearDay`]`/`[`Norm`][TimeSplitYearDayNorm],
///   [`TimeSplitYearSec`]`/`[`Norm`][TimeSplitYearSecNorm],
///   [`TimeSplitHourSec`]`/`[`Norm`][TimeSplitHourSecNorm],
///   [`TimeSplitHourNano`]`/`[`Norm`][TimeSplitHourNanoNorm],
///   [`TimeSplitMilliNano`]`/`[`Norm`][TimeSplitMilliNanoNorm].
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[doc = crate::_TAG_TIME!()]
/// A time split from years to nanoseconds, normalized *(192b size, 152b payload)*.
pub type TimeSplitNorm = TimeSplit<u64, u8, u8, u8, u8, u8, u16, u16, u16>;

#[doc = crate::_TAG_TIME!()]
/// A time split from years to days.
pub type TimeSplitYearDay<Y, MO, D> = TimeSplit<Y, MO, D, (), (), (), (), (), ()>;
#[doc = crate::_TAG_TIME!()]
/// A time split from years to days, normalized *(128b size, 80b payload)*.
pub type TimeSplitYearDayNorm = TimeSplit<u64, u8, u8, (), (), (), (), (), ()>;

#[doc = crate::_TAG_TIME!()]
/// A time split from years to seconds.
pub type TimeSplitYearSec<Y, MO, D, H, M, S> = TimeSplit<Y, MO, D, H, M, S, (), (), ()>;
#[doc = crate::_TAG_TIME!()]
/// A time split from years to seconds, normalized *(128b size, 104b payload)*.
pub type TimeSplitYearSecNorm = TimeSplit<u64, u8, u8, u8, u8, u8, (), (), ()>;

#[doc = crate::_TAG_TIME!()]
/// A time split from hours to seconds.
pub type TimeSplitHourSec<H, M, S> = TimeSplit<(), (), (), H, M, S, (), (), ()>;
#[doc = crate::_TAG_TIME!()]
/// A time split from hours to seconds, normalized *(128b size, 80b payload)*.
pub type TimeSplitHourSecNorm = TimeSplit<(), (), (), u64, u8, u8, (), (), ()>;

#[doc = crate::_TAG_TIME!()]
/// A time split from hours to nanoseconds.
pub type TimeSplitHourNano<H, M, S, MS, US, NS> = TimeSplit<(), (), (), H, M, S, MS, US, NS>;
#[doc = crate::_TAG_TIME!()]
/// A time split from hours to seconds, normalized *(128b size & payload)*.
pub type TimeSplitHourNanoNorm = TimeSplit<(), (), (), u64, u8, u8, u16, u16, u16>;

#[doc = crate::_TAG_TIME!()]
/// A time split from milliseconds to nanoseconds.
pub type TimeSplitMilliNano<MS, US, NS> = TimeSplit<(), (), (), (), (), (), MS, US, NS>;
#[doc = crate::_TAG_TIME!()]
/// A time split from milliseconds to nanoseconds, normalized *(48b size & payload)*.
pub type TimeSplitMilliNanoNorm = TimeSplit<(), (), (), (), (), (), u16, u16, u16>;

/* constructors */

impl<Y, MO, D, H, M, S, MS, US, NS> TimeSplit<Y, MO, D, H, M, S, MS, US, NS> {
    /// Returns a new [`TimeSplit`].
    #[allow(clippy::too_many_arguments)]
    pub const fn new(y: Y, mo: MO, d: D, h: H, m: M, s: S, ms: MS, us: US, ns: NS) -> Self {
        Self { y, mo, d, h, m, s, ms, us, ns }
    }
}
impl TimeSplitNorm {
    /// Converts a `Duration` into a full [`TimeSplit`].
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
        TimeSplit { y, mo, d, h, m, s, ms, us, ns }
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

/// # Structural introspection
impl<Y, MO, D, H, M, S, MS, US, NS> TimeSplit<Y, MO, D, H, M, S, MS, US, NS> {
    /// Indicates whether the [`y`][Self::y] field is enabled.
    pub const Y: bool = size_of::<Y>() != 0;
    /// Indicates whether the [`mo`][Self::mo] field is enabled.
    pub const MO: bool = size_of::<MO>() != 0;
    /// Indicates whether the [`d`][Self::d] field is enabled.
    pub const D: bool = size_of::<D>() != 0;
    /// Indicates whether the [`h`][Self::h] field is enabled.
    pub const H: bool = size_of::<H>() != 0;
    /// Indicates whether the [`m`][Self::m] field is enabled.
    pub const M: bool = size_of::<M>() != 0;
    /// Indicates whether the [`s`][Self::s] field is enabled.
    pub const S: bool = size_of::<S>() != 0;
    /// Indicates whether the [`ms`][Self::ms] field is enabled.
    pub const MS: bool = size_of::<MS>() != 0;
    /// Indicates whether the [`us`][Self::us] field is enabled.
    pub const US: bool = size_of::<US>() != 0;
    /// Indicates whether the [`ns`][Self::ns] field is enabled.
    pub const NS: bool = size_of::<NS>() != 0;

    /// Indicates whether the 3 fields from [`y`][Self::y] to [`d`][Self::d] are enabled.
    pub const Y_D: bool = Self::Y && Self::MO && Self::D;
    /// Indicates whether the 3 fields from [`h`][Self::h] to [`s`][Self::s] are enabled.
    pub const H_S: bool = Self::H && Self::M && Self::S;
    /// Indicates whether the 3 fields from [`ms`][Self::ms] to [`ns`][Self::ns] are enabled.
    pub const MS_NS: bool = Self::MS && Self::US && Self::NS;

    /// Indicates whether the 6 fields from [`y`][Self::y] to [`s`][Self::s] are enabled.
    pub const Y_S: bool = Self::Y_D && Self::H_S;
    /// Indicates whether the 6 fields from [`h`][Self::h] to [`ns`][Self::ns] are enabled.
    pub const H_NS: bool = Self::H_S && Self::MS_NS;

    /// Indicates whether all the 9 fields from [`y`][Self::y] to [`ns`][Self::ns] are enabled.
    pub const Y_NS: bool = Self::Y_D && Self::H_S && Self::MS_NS;

    /// Indicates whether *only* the 3 fields from [`y`][Self::y] to [`d`][Self::d] are enabled.
    pub const IS_YEAR_DAY: bool = Self::Y_D && !Self::H_NS;
    /// Indicates whether *only* the 3 fields from [`h`][Self::h] to [`s`][Self::s] are enabled.
    pub const IS_HOUR_SEC: bool = Self::H_S && !Self::Y_D && !Self::MS_NS;
    /// Indicates whether the 3 fields from [`ms`][Self::ms] to [`ns`][Self::ns] are enabled.
    pub const IS_MILLI_NANO: bool = Self::MS_NS && !Self::Y_S;

    /// Indicates whether *only* the 6 fields from [`y`][Self::y] to [`s`][Self::s] are enabled.
    pub const IS_YEAR_SEC: bool = Self::Y_S && !Self::MS_NS;
    /// Indicates whether *only* the 6 fields from [`h`][Self::h] to [`ns`][Self::ns] are enabled.
    pub const IS_HOUR_NANO: bool = Self::H_NS && !Self::Y_D;

    /// Indicates whether all the 9 fields from [`y`][Self::y] to [`ns`][Self::ns] are enabled.
    pub const IS_YEAR_NANO: bool = Self::Y_NS;
}

/// # Instance introspection
#[rustfmt::skip]
impl<Y, MO, D, H, M, S, MS, US, NS> TimeSplit<Y, MO, D, H, M, S, MS, US, NS> {
    /// Indicates whether the [`y`][Self::y] field is enabled.
    pub const fn has_y(&self) -> bool { Self::Y }
    /// Indicates whether the [`mo`][Self::mo] field is enabled.
    pub const fn has_mo(&self) -> bool { Self::MO }
    /// Indicates whether the [`d`][Self::d] field is enabled.
    pub const fn has_d(&self) -> bool { Self::D }
    /// Indicates whether the [`h`][Self::h] field is enabled.
    pub const fn has_h(&self) -> bool { Self::H }
    /// Indicates whether the [`s`][Self::s] field is enabled.
    pub const fn has_s(&self) -> bool { Self::S }
    /// Indicates whether the [`ms`][Self::ms] field is enabled.
    pub const fn has_ms(&self) -> bool { Self::MS }
    /// Indicates whether the [`us`][Self::us] field is enabled.
    pub const fn has_us(&self) -> bool { Self::US }
    /// Indicates whether the [`ns`][Self::ns] field is enabled.
    pub const fn has_ns(&self) -> bool { Self::NS }

    /// Indicates whether the 3 fields from [`y`][Self::y] to [`d`][Self::d] are enabled.
    pub const fn has_y_d(&self) -> bool { Self::Y_D }
    /// Indicates whether the 3 fields from [`h`][Self::h] to [`s`][Self::s] are enabled.
    pub const fn has_h_s(&self) -> bool { Self::H_S }
    /// Indicates whether the 3 fields from [`ms`][Self::ms] to [`ns`][Self::ns] are enabled.
    pub const fn has_ms_ns(&self) -> bool { Self::MS_NS }

    /// Indicates whether the 6 fields from [`y`][Self::y] to [`s`][Self::s] are enabled.
    pub const fn has_y_s(&self) -> bool { Self::Y_S }
    /// Indicates whether the 6 fields from [`h`][Self::h] to [`ns`][Self::ns] are enabled.
    pub const fn has_h_ns(&self) -> bool { Self::H_NS }

    /// Indicates whether all the 9 fields from [`y`][Self::y] to [`ns`][Self::ns] are enabled.
    pub const fn has_y_ns(&self) -> bool { Self::Y_NS }

    /// Indicates whether *only* the 3 fields from [`y`][Self::y] to [`d`][Self::d] are enabled.
    pub const fn is_year_day(&self) -> bool { Self::IS_YEAR_DAY }
    /// Indicates whether *only* the 3 fields from [`h`][Self::h] to [`s`][Self::s] are enabled.
    pub const fn is_hour_sec(&self) -> bool { Self::IS_HOUR_SEC }
    /// Indicates whether the 3 fields from [`ms`][Self::ms] to [`ns`][Self::ns] are enabled.
    pub const fn is_milli_nano(&self) -> bool { Self::IS_MILLI_NANO }

    /// Indicates whether *only* the 6 fields from [`y`][Self::y] to [`s`][Self::s] are enabled.
    pub const fn is_year_sec(&self) -> bool { Self::IS_YEAR_SEC }
    /// Indicates whether *only* the 6 fields from [`h`][Self::h] to [`ns`][Self::ns] are enabled.
    pub const fn is_hour_nano(&self) -> bool { Self::IS_HOUR_NANO }

    /// Indicates whether all the 9 fields from [`y`][Self::y] to [`ns`][Self::ns] are enabled.
    pub const fn is_year_nano(&self) -> bool { Self::IS_YEAR_NANO }
}

#[rustfmt::skip]
#[allow(clippy::if_then_some_else_none)]
impl<Y, MO, D, H, M, S, MS, US, NS> TimeSplit<Y, MO, D, H, M, S, MS, US, NS> {
    /// Returns a (9) tuple with all the elements.
    pub fn as_tuple(self) -> (Y, MO, D, H, M, S, MS, US, NS) {
        (self.y, self.mo, self.d, self.h, self.m, self.s, self.ms, self.us, self.ns)
    }
    /// Returns a (9) tuple with all the elements.
    pub const fn to_tuple(&self) -> (Y, MO, D, H, M, S, MS, US, NS)
        where Y: Copy, MO: Copy, D: Copy, H: Copy, M: Copy, S: Copy, MS: Copy, US: Copy, NS: Copy {
        (self.y, self.mo, self.d, self.h, self.m, self.s, self.ms, self.us, self.ns)
    }

    /// Returns a (3) tuple if the 3 fields from [`y`][Self::y] to [`d`][Self::d] are enabled.
    pub fn as_tuple_y_d(self) -> Option<(Y, MO, D)> {
        if self.has_y_d() { Some((self.y, self.mo, self.d)) } else { None }
    }
    /// Returns a (3) tuple if the 3 fields from [`y`][Self::y] to [`d`][Self::d] are enabled.
    pub const fn to_tuple_y_d(&self) -> Option<(Y, MO, D)>
        where Y: Copy, MO: Copy, D: Copy {
        if self.has_y_d() { Some((self.y, self.mo, self.d)) } else { None }
    }

    /// Returns a (3) tuple if the 3 fields from [`h`][Self::h] to [`s`][Self::s] are enabled.
    pub fn as_tuple_h_s(self) -> Option<(H, M, S)> {
        if self.has_h_s() { Some((self.h, self.m, self.s)) } else { None }
    }
    /// Returns a (3) tuple if the 3 fields from [`h`][Self::h] to [`s`][Self::s] are enabled.
    pub const fn to_tuple_h_s(&self) -> Option<(H, M, S)>
        where H: Copy, M: Copy, S: Copy {
        if self.has_h_s() { Some((self.h, self.m, self.s)) } else { None }
    }

    /// Returns a (3) tuple if the 3 fields from [`ms`][Self::ms] to [`ns`][Self::ns] are enabled.
    pub fn as_tuple_ms_ns(self) -> Option<(MS, US, NS)> {
        if self.has_ms_ns() { Some((self.ms, self.us, self.ns)) } else { None }
    }
    /// Returns a (3) tuple if the 3 fields from [`ms`][Self::ms] to [`ns`][Self::ns] are enabled.
    pub const fn to_tuple_ms_ns(&self) -> Option<(MS, US, NS)>
        where MS: Copy, US: Copy, NS: Copy {
        if self.has_ms_ns() { Some((self.ms, self.us, self.ns)) } else { None }
    }

    /// Returns a (6) tuple if the 6 fields from [`y`][Self::y] to [`s`][Self::s] are enabled.
    pub fn as_tuple_y_s(self) -> Option<(Y, MO, D, H, M, S)> {
        if self.has_y_s() { Some((self.y, self.mo, self.d, self.h, self.m, self.s)) } else { None }
    }
    /// Returns a (6) tuple if the 6 fields from [`y`][Self::y] to [`s`][Self::s] are enabled.
    pub const fn to_tuple_y_s(&self) -> Option<(Y, MO, D, H, M, S)>
        where Y: Copy, MO: Copy, D: Copy, H: Copy, M: Copy, S: Copy {
        if self.has_y_s() { Some((self.y, self.mo, self.d, self.h, self.m, self.s)) } else { None }
    }

    /// Returns a (6) tuple if the 6 fields from [`h`][Self::h] to [`ns`][Self::ns] are enabled.
    pub fn as_tuple_h_ns(self) -> Option<(H, M, S, MS, US, NS)> {
        if self.has_h_ns() { Some((self.h, self.m, self.s, self.ms, self.us, self.ns))
        } else { None }
    }
    /// Returns a (6) tuple if the 6 fields from [`h`][Self::h] to [`ns`][Self::ns] are enabled.
    pub const fn to_tuple_h_ns(&self) -> Option<(H, M, S, MS, US, NS)>
        where H: Copy, M: Copy, S: Copy, MS: Copy, US: Copy, NS: Copy {
        if self.has_h_ns() { Some((self.h, self.m, self.s, self.ms, self.us, self.ns))
        } else { None }
    }

    /// Returns a (9) tuple if the 9 fields from [`y`][Self::y] to [`ns`][Self::ns] are enabled.
    #[allow(clippy::type_complexity)]
    pub fn as_tuple_y_ns(self) -> Option<(Y, MO, D, H, M, S, MS, US, NS)> {
        if self.has_y_ns() {
            Some((self.y, self.mo, self.d, self.h, self.m, self.s, self.ms, self.us, self.ns))
        } else { None }
    }
    /// Returns a (9) tuple if the 9 fields from [`y`][Self::y] to [`ns`][Self::ns] are enabled.
    #[allow(clippy::type_complexity)]
    pub const fn to_tuple_y_ns(&self) -> Option<(Y, MO, D, H, M, S, MS, US, NS)>
        where Y: Copy, MO: Copy, D: Copy, H: Copy, M: Copy, S: Copy, MS: Copy, US: Copy, NS: Copy {
        if self.has_y_ns() {
            Some((self.y, self.mo, self.d, self.h, self.m, self.s, self.ms, self.us, self.ns))
        } else { None }
    }
}

#[rustfmt::skip]
impl<Y, MO, D, H, M, S, MS, US, NS> fmt::Debug for TimeSplit<Y, MO, D, H, M, S, MS, US, NS>
where
    Y: 'static + fmt::Debug, MO: 'static + fmt::Debug, D: 'static + fmt::Debug,
    H: 'static + fmt::Debug, M: 'static + fmt::Debug, S: 'static + fmt::Debug,
    MS: 'static + fmt::Debug, US: 'static + fmt::Debug, NS: 'static + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notime = ().type_of();
        let mut debug_struct = f.debug_struct("TimeSplit");
        if Y::type_id() != notime { debug_struct.field("y", &self.mo); }
        if MO::type_id() != notime { debug_struct.field("mo", &self.mo); }
        if D::type_id() != notime { debug_struct.field("d", &self.d); }
        if H::type_id() != notime { debug_struct.field("h", &self.h); }
        if M::type_id() != notime { debug_struct.field("m", &self.m); }
        if S::type_id() != notime { debug_struct.field("s", &self.s); }
        if MS::type_id() != notime { debug_struct.field("ms", &self.ms); }
        if US::type_id() != notime { debug_struct.field("us", &self.us); }
        if NS::type_id() != notime { debug_struct.field("ns", &self.ns); }
        debug_struct.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format_buf;

    #[test]
    fn split_debug() {
        let mut buf = [0; 32];
        let _str = format_buf![&mut buf, "{:?}", TimeSplit::new_hour_sec(1, 2, 3)].unwrap();
        assert_eq!["TimeSplit { h: 1, m: 2, s: 3 }", _str];
    }

    #[test]
    fn split_size() {
        assert_eq!(0, size_of::<TimeSplit<(), (), (), (), (), (), (), (), ()>>());

        /* normalized inner reprs */

        assert_eq!(24, size_of::<TimeSplitNorm>()); // 5 bytes padded
        assert_eq!(16, size_of::<TimeSplitYearDayNorm>()); // 6 bytes padded
        assert_eq![16, size_of::<TimeSplitHourSecNorm>()]; // 6 bytes padded
        assert_eq![6, size_of::<TimeSplitMilliNanoNorm>()]; // 0 padding
        assert_eq!(16, size_of::<TimeSplitYearSecNorm>()); // 3 bytes padded
        assert_eq!(16, size_of::<TimeSplitHourNanoNorm>()); // 0 padding
    }
}
