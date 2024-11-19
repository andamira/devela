// devela::sys::time::split
//
//! Splitting and decomposing time.
//

use crate::Duration;

/* decomposed */

/// A time split in years, months and days.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplitYearDay<Y, MO, D> {
    /// Years.
    pub y: Y,
    /// Months.
    pub mo: MO,
    /// Days.
    pub d: D,
}
// 80b
impl TimeSplitYearDay<u64, u8, u8> {
    /// Converts a `Duration` into a time split representation in years, months, and days.
    ///
    /// This method assumes 365 days per year and 30 days per month for simplicity.
    pub const fn from_duration(duration: Duration) -> Self {
        let days = duration.as_secs() / 86400;
        let y = days / 365;
        let days_rem = days % 365;
        let mo = (days_rem / 30) as u8;
        let d = (days_rem % 30) as u8;
        TimeSplitYearDay { y, mo, d }
    }
}

/// A time split in hours, minutes and seconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplitHourSec<H, M, S> {
    /// Hours.
    pub h: H,
    /// Minutes.
    pub m: M,
    /// Seconds.
    pub s: S,
}
// 80b
impl TimeSplitHourSec<u64, u8, u8> {
    /// Converts a `Duration` into a time split representation of hours, minutes and seconds.
    ///
    /// Excess days or longer periods are converted into additional hours.
    pub const fn from_duration(duration: Duration) -> Self {
        let secs = duration.as_secs();
        let h = secs / 3600;
        let m = ((secs % 3600) / 60) as u8;
        let s = (secs % 60) as u8;
        TimeSplitHourSec { h, m, s }
    }
}

/// A time split in milliseconds, microseconds, and nanoseconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplitMilliNano<MS, US, NS> {
    /// Milliseconds.
    pub ms: MS,
    /// Microseconds.
    pub us: US,
    /// Nanoseconds.
    pub ns: NS,
}
// 48 b
impl TimeSplitMilliNano<u16, u16, u16> {
    /// Converts a `Duration`'s sub-second component into a compact time split representation.
    ///
    /// Extracts and segments the nanosecond portion of a `Duration`
    /// into milliseconds, microseconds, and nanoseconds.
    pub const fn from_duration(duration: Duration) -> Self {
        let nanos = duration.subsec_nanos();
        let ms = (nanos / 1_000_000) as u16;
        let us = ((nanos % 1_000_000) / 1_000) as u16;
        let ns = (nanos % 1_000) as u16;
        TimeSplitMilliNano { ms, us, ns }
    }
}

/* composed */

/// A time split in years, months, days, hours, minutes, seconds,
/// milliseconds, microseconds, and nanoseconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplitYearNano<Y, MO, D, H, M, S, MS, US, NS> {
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
// 152b
impl TimeSplitYearNano<u64, u8, u8, u8, u8, u8, u16, u16, u16> {
    /// Converts a `Duration` into a time split representation from years down to nanoseconds.
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

/// A time split in years, months, days, hours, minutes and seconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplitYearSec<Y, MO, D, H, M, S> {
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
}
// 104b
impl TimeSplitYearSec<u64, u8, u8, u8, u8, u8> {
    /// Converts a `Duration` into a time split representation from years down to seconds.
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

        TimeSplitYearSec { y, mo, d, h, m, s }
    }
}

/// A time split in hours, minutes, seconds, milliseconds, microseconds, and nanoseconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSplitHourNano<H, M, S, MS, US, NS> {
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
// 128b
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

        TimeSplitHourNano { h, m, s, ms, us, ns }
    }
}

// ----------

// TEMP
/// A time split in seconds, milliseconds, microseconds, and nanoseconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecNanoSplit<S, MS, US, NS> {
    /// Seconds.
    pub s: S,
    /// Milliseconds.
    pub ms: MS,
    /// Microseconds.
    pub us: US,
    /// Nanoseconds.
    pub ns: NS,
}

// TEMP
/// A time split in hours, minutes, seconds and milliseconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HourMilliSplit<H, M, S, MS> {
    /// Hours.
    pub h: H,
    /// Minutes.
    pub m: M,
    /// Seconds.
    pub s: S,
    /// Milliseconds.
    pub ms: MS,
}

// # Arguments
// - $name: the type name
// - $LEN:  the number of generics
// - $T:    the generic T (repeated type)
// - $G:    the generic type (different), and the field name (in lower case)
macro_rules! impl_as_to {
    ($name:ident: $LEN:literal, <$($T:ident+$G:ident),+>) => { $crate::paste! {
        impl<$($G),+> $name<$($G),+> {
            /// Returns the fields as a tuple.
            #[inline] #[must_use]
            pub fn as_tuple(self) -> ( $($G),+ ) {
                ( $( self.[<$G:lower>] ),+ )
            }
        }
        impl<$($G: Copy),+> $name<$($G),+> {
            /// Returns the fields as a tuple in compile-time.
            #[inline] #[must_use]
            pub const fn to_tuple(self) -> ( $($G),+ ) {
                ( $( self.[<$G:lower>] ),+ )
            }
        }
        impl<T> $name<$($T),+> {
            /// Returns the fields as an array.
            #[inline] #[must_use]
            pub fn as_array(self) -> [T; $LEN] {
                [ $( self.[<$G:lower>] ),+ ]
            }
        }
        impl<T: Copy> $name<$($T),+> {
            /// Returns the fields as an array in compile-time.
            #[inline] #[must_use]
            pub const fn to_array(self) -> [T; $LEN] {
                [ $( self.[<$G:lower>] ),+ ]
            }
        }

    }};
}
impl_as_to![TimeSplitYearDay: 3, <T+Y, T+MO, T+D>];
impl_as_to![TimeSplitHourSec: 3, <T+H, T+M, T+S>];
impl_as_to![TimeSplitMilliNano: 3, <T+MS, T+US, T+NS>];
//
impl_as_to![TimeSplitYearNano: 9, <T+Y, T+MO, T+D, T+H, T+M, T+S, T+MS, T+US, T+NS>];
impl_as_to![TimeSplitYearSec: 6, <T+Y, T+MO, T+D, T+H, T+M, T+S>];
impl_as_to![TimeSplitHourNano: 6, <T+H, T+M, T+S, T+MS, T+US, T+NS>];

// TEMP
impl_as_to![SecNanoSplit: 4, <T+S, T+MS, T+US, T+NS>];
impl_as_to![HourMilliSplit: 4, <T+H, T+M, T+S, T+MS>];
