// devela::phys::time::delta
//
//! Defines the [`TimeDelta`] struct.
//
// TOC
// - basic methods
// - Operations involving integers
// - Operations involving floats
// - additional methods
// - conversions

#[cfg(feature = "js")]
use crate::JsInstant;
#[cfg(feature = "std")]
use crate::SystemInstant;
use crate::{unwrap, Duration};
#[allow(unused)]
#[cfg(_float··)]
use crate::{ExtFloat, Float};

/// A signed duration of time, stored as an `(i64, i32)` pair of seconds and nanoseconds.
///
/// Supports negative values, allowing representation of both past and future offsets.
//
// TODO: comparison with Duration.
#[doc = crate::doc_!(vendor: "jiff")]
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelta {
    secs: i64,
    nanos: i32,
}

const NANOS_PER_SEC: i32 = 1_000_000_000;
const NANOS_PER_MILLI: i32 = 1_000_000;
const NANOS_PER_MICRO: i32 = 1_000;
const MILLIS_PER_SEC: i64 = 1_000;
const MICROS_PER_SEC: i64 = 1_000_000;
const SECS_PER_MINUTE: i64 = 60;
const MINS_PER_HOUR: i64 = 60;

/// # Basic methods
impl TimeDelta {
    /// A duration of zero time.
    pub const ZERO: TimeDelta = TimeDelta { secs: 0, nanos: 0 };

    /// The minimum possible duration. Or the "most negative" duration.
    pub const MIN: TimeDelta = TimeDelta { secs: i64::MIN, nanos: -(NANOS_PER_SEC - 1) };

    /// The maximum possible duration.
    pub const MAX: TimeDelta = TimeDelta { secs: i64::MAX, nanos: NANOS_PER_SEC - 1 };

    /// Creates a new `TimeDelta` from the given number of whole seconds and additional nanoseconds.
    ///
    /// If the absolute value of the nanoseconds is greater than or equal to
    /// 1 second, then the excess balances into the number of whole seconds.
    ///
    /// # Panics
    /// When the absolute value of the nanoseconds is greater than or equal
    /// to 1 second and the excess that carries over to the number of whole
    /// seconds overflows `i64`.
    ///
    /// This never panics when `nanos` is less than `1_000_000_000`.
    pub const fn new(mut secs: i64, mut nanos: i32) -> TimeDelta {
        // When |nanos| exceeds 1 second, we balance the excess up to seconds.
        if !(-NANOS_PER_SEC < nanos && nanos < NANOS_PER_SEC) {
            // Never wraps or panics because NANOS_PER_SEC!={0,-1}.
            let addsecs = nanos / NANOS_PER_SEC;
            secs = match secs.checked_add(addsecs as i64) {
                Some(secs) => secs,
                None => panic!("nanoseconds overflowed seconds in TimeDelta::new"),
            };
            // Never wraps or panics because NANOS_PER_SEC!={0,-1}.
            nanos %= NANOS_PER_SEC;
        }
        // At this point, we're done if either unit is zero or if they have the same sign.
        if nanos == 0 || secs == 0 || secs.signum() == (nanos.signum() as i64) {
            return TimeDelta::new_unchecked(secs, nanos);
        }
        // Otherwise, the only work we have to do is to balance negative nanos
        // into positive seconds, or positive nanos into negative seconds.
        if secs < 0 {
            debug_assert!(nanos > 0);
            // Never wraps because adding +1 to a negative i64 never overflows.
            //
            // MSRV(1.79): Consider using `unchecked_add` here.
            secs += 1;
            // Never wraps because subtracting +1_000_000_000 from a positive i32 never overflows.
            //
            // MSRV(1.79): Consider using `unchecked_sub` here.
            nanos -= NANOS_PER_SEC;
        } else {
            debug_assert!(secs > 0);
            debug_assert!(nanos < 0);
            // Never wraps because subtracting +1 from a positive i64 never
            // overflows.
            //
            // MSRV(1.79): Consider using `unchecked_add` here.
            secs -= 1;
            // Never wraps because adding +1_000_000_000 to a negative i32 never overflows.
            //
            // MSRV(1.79): Consider using `unchecked_add` here.
            nanos += NANOS_PER_SEC;
        }
        TimeDelta::new_unchecked(secs, nanos)
    }

    /// Creates a new signed duration without handling nanosecond overflow.
    ///
    /// This might produce tighter code in some cases.
    ///
    /// In debug mode only, when `|nanos|` is greater than or equal to 1 second.
    ///
    /// This is not exported so that code outside this module can rely on
    /// `|nanos|` being less than a second for purposes of memory safety.
    const fn new_unchecked(secs: i64, nanos: i32) -> TimeDelta {
        debug_assert!(nanos <= 999_999_999);
        debug_assert!(nanos >= -999_999_999);
        TimeDelta { secs, nanos }
    }

    /// Creates a new `TimeDelta` from the given number of whole seconds.
    pub const fn from_secs(secs: i64) -> TimeDelta {
        TimeDelta::new_unchecked(secs, 0)
    }

    /// Creates a new `TimeDelta` from the given number of whole milliseconds.
    ///
    /// Note that since this accepts an `i64`, this method cannot be used
    /// to construct the full range of possible signed duration values. In
    /// particular, [`TimeDelta::as_millis`] returns an `i128`, and this
    /// may be a value that would otherwise overflow an `i64`.
    pub const fn from_millis(millis: i64) -> TimeDelta {
        // OK because MILLIS_PER_SEC!={-1,0}.
        let secs = millis / MILLIS_PER_SEC;
        // OK because MILLIS_PER_SEC!={-1,0} and because millis % MILLIS_PER_SEC
        // can be at most 999, and 999 * 1_000_000 never overflows i32.
        let nanos = (millis % MILLIS_PER_SEC) as i32 * NANOS_PER_MILLI;
        TimeDelta::new_unchecked(secs, nanos)
    }

    /// Creates a new `TimeDelta` from the given number of whole microseconds.
    ///
    /// Note that since this accepts an `i64`, this method cannot be used
    /// to construct the full range of possible signed duration values. In
    /// particular, [`TimeDelta::as_micros`] returns an `i128`, and this
    /// may be a value that would otherwise overflow an `i64`.
    pub const fn from_micros(micros: i64) -> TimeDelta {
        // OK because MICROS_PER_SEC!={-1,0}.
        let secs = micros / MICROS_PER_SEC;
        // OK because MICROS_PER_SEC!={-1,0} and because millis % MICROS_PER_SEC
        // can be at most 999, and 999 * 1_000_000 never overflows i32.
        let nanos = (micros % MICROS_PER_SEC) as i32 * NANOS_PER_MICRO;
        TimeDelta::new_unchecked(secs, nanos)
    }

    /// Creates a new `TimeDelta` from the given number of whole nanoseconds.
    ///
    /// Note that since this accepts an `i64`, this method cannot be used
    /// to construct the full range of possible signed duration values. In
    /// particular, [`TimeDelta::as_nanos`] returns an `i128`, which may
    /// be a value that would otherwise overflow an `i64`.
    pub const fn from_nanos(nanos: i64) -> TimeDelta {
        // OK because NANOS_PER_SEC!={-1,0}.
        let secs = nanos / (NANOS_PER_SEC as i64);
        // OK because NANOS_PER_SEC!={-1,0}.
        let nanos = (nanos % (NANOS_PER_SEC as i64)) as i32;
        TimeDelta::new_unchecked(secs, nanos)
    }

    /// Creates a new `TimeDelta` from the given number of hours.
    /// Every hour is exactly `3,600` seconds.
    ///
    /// # Panics
    /// Panics if the number of hours, after being converted to nanoseconds,
    /// overflows the minimum or maximum `TimeDelta` values.
    pub const fn from_hours(hours: i64) -> TimeDelta {
        // OK because (SECS_PER_MINUTE*MINS_PER_HOUR)!={-1,0}.
        const MIN_HOUR: i64 = i64::MIN / (SECS_PER_MINUTE * MINS_PER_HOUR);
        // OK because (SECS_PER_MINUTE*MINS_PER_HOUR)!={-1,0}.
        const MAX_HOUR: i64 = i64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR);
        // OK because (SECS_PER_MINUTE*MINS_PER_HOUR)!={-1,0}.
        if hours < MIN_HOUR {
            panic!("hours overflowed minimum number of TimeDelta seconds")
        }
        // OK because (SECS_PER_MINUTE*MINS_PER_HOUR)!={-1,0}.
        if hours > MAX_HOUR {
            panic!("hours overflowed maximum number of TimeDelta seconds")
        }
        TimeDelta::from_secs(hours * MINS_PER_HOUR * SECS_PER_MINUTE)
    }

    /// Creates a new `TimeDelta` from the given number of minutes.
    /// Every minute is exactly `60` seconds.
    ///
    /// # Panics
    /// Panics if the number of minutes, after being converted to nanoseconds,
    /// overflows the minimum or maximum `TimeDelta` values.
    pub const fn from_mins(minutes: i64) -> TimeDelta {
        // OK because SECS_PER_MINUTE!={-1,0}.
        const MIN_MINUTE: i64 = i64::MIN / SECS_PER_MINUTE;
        // OK because SECS_PER_MINUTE!={-1,0}.
        const MAX_MINUTE: i64 = i64::MAX / SECS_PER_MINUTE;
        // OK because SECS_PER_MINUTE!={-1,0}.
        if minutes < MIN_MINUTE {
            panic!("minutes overflowed minimum number of TimeDelta seconds")
        }
        // OK because SECS_PER_MINUTE!={-1,0}.
        if minutes > MAX_MINUTE {
            panic!("minutes overflowed maximum number of TimeDelta seconds")
        }
        TimeDelta::from_secs(minutes * SECS_PER_MINUTE)
    }

    /// Returns true if this duration spans no time.
    pub const fn is_zero(&self) -> bool {
        self.secs == 0 && self.nanos == 0
    }

    /// Returns the number of whole seconds in this duration.
    ///
    /// The value returned is negative when the duration is negative.
    ///
    /// This does not include any fractional component corresponding to units
    /// less than a second. To access those, use one of the `subsec` methods
    /// such as [`TimeDelta::subsec_nanos`].
    pub const fn as_secs(&self) -> i64 {
        self.secs
    }

    /// Returns the fractional part of this duration in whole milliseconds.
    ///
    /// The value returned is negative when the duration is negative. It is
    /// guaranteed that the range of the value returned is in the inclusive
    /// range `-999..=999`.
    ///
    /// To get the length of the total duration represented in milliseconds,
    /// use [`TimeDelta::as_millis`].
    pub const fn subsec_millis(&self) -> i32 {
        // OK because NANOS_PER_MILLI!={-1,0}.
        self.nanos / NANOS_PER_MILLI
    }

    /// Returns the fractional part of this duration in whole microseconds.
    ///
    /// The value returned is negative when the duration is negative. It is
    /// guaranteed that the range of the value returned is in the inclusive
    /// range `-999_999..=999_999`.
    ///
    /// To get the length of the total duration represented in microseconds,
    /// use [`TimeDelta::as_micros`].
    pub const fn subsec_micros(&self) -> i32 {
        // OK because NANOS_PER_MICRO!={-1,0}.
        self.nanos / NANOS_PER_MICRO
    }

    /// Returns the fractional part of this duration in whole nanoseconds.
    ///
    /// The value returned is negative when the duration is negative. It is
    /// guaranteed that the range of the value returned is in the inclusive
    /// range `-999_999_999..=999_999_999`.
    ///
    /// To get the length of the total duration represented in nanoseconds,
    /// use [`TimeDelta::as_nanos`].
    pub const fn subsec_nanos(&self) -> i32 {
        self.nanos
    }

    /// Returns the total duration in units of whole milliseconds.
    ///
    /// The value returned is negative when the duration is negative.
    ///
    /// To get only the fractional component of this duration in units of
    /// whole milliseconds, use [`TimeDelta::subsec_millis`].
    pub const fn as_millis(&self) -> i128 {
        // OK because 1_000 times any i64 will never overflow i128.
        let millis = (self.secs as i128) * (MILLIS_PER_SEC as i128);
        // OK because NANOS_PER_MILLI!={-1,0}.
        let subsec_millis = (self.nanos / NANOS_PER_MILLI) as i128;
        // OK because subsec_millis maxes out at 999, and adding that to
        // i64::MAX*1_000 will never overflow a i128.
        millis + subsec_millis
    }

    /// Returns the total duration in units of whole microseconds.
    ///
    /// The value returned is negative when the duration is negative.
    ///
    /// To get only the fractional component of this duration in units of
    /// whole microseconds, use [`TimeDelta::subsec_micros`].
    pub const fn as_micros(&self) -> i128 {
        // OK because 1_000_000 times any i64 will never overflow i128.
        let micros = (self.secs as i128) * (MICROS_PER_SEC as i128);
        // OK because NANOS_PER_MICRO!={-1,0}.
        let subsec_micros = (self.nanos / NANOS_PER_MICRO) as i128;
        // OK because subsec_micros maxes out at 999_999, and adding that to
        // i64::MAX*1_000_000 will never overflow a i128.
        micros + subsec_micros
    }

    /// Returns the total duration in units of whole nanoseconds.
    ///
    /// The value returned is negative when the duration is negative.
    ///
    /// To get only the fractional component of this duration in units of
    /// whole nanoseconds, use [`TimeDelta::subsec_nanos`].
    pub const fn as_nanos(&self) -> i128 {
        // OK because 1_000_000_000 times any i64 will never overflow i128.
        let nanos = (self.secs as i128) * (NANOS_PER_SEC as i128);
        // OK because subsec_nanos maxes out at 999_999_999, and adding that to
        // i64::MAX*1_000_000_000 will never overflow a i128.
        nanos + (self.nanos as i128)
    }

    // NOTE: We don't provide `abs_diff` here because we can't represent the
    // difference between all possible durations. For example,
    // `abs_diff(TimeDelta::MAX, TimeDelta::MIN)`. It therefore seems
    // like we should actually return a `std::time::Duration` here, but I'm
    // trying to be conservative when diverging from std.
}

/// # Operations.
impl TimeDelta {
    /// Add two signed durations together. If overflow occurs, then `None` is returned.
    pub const fn checked_add(self, rhs: TimeDelta) -> Option<TimeDelta> {
        let Some(mut secs) = self.secs.checked_add(rhs.secs) else {
            return None;
        };
        // OK because `-999_999_999 <= nanos <= 999_999_999`, and so adding
        // them together will never overflow an i32.
        let mut nanos = self.nanos + rhs.nanos;
        // The below is effectively TimeDelta::new, but with checked
        // arithmetic. My suspicion is that there is probably a better way
        // to do this. The main complexity here is that 1) `|nanos|` might
        // now exceed 1 second and 2) the signs of `secs` and `nanos` might
        // not be the same. The other difference from TimeDelta::new is
        // that we know that `-1_999_999_998 <= nanos <= 1_999_999_998` since
        // `|TimeDelta::nanos|` is guaranteed to be less than 1 second. So
        // we can skip the div and modulus operations.

        // When |nanos| exceeds 1 second, we balance the excess up to seconds.
        if nanos != 0 {
            if nanos >= NANOS_PER_SEC {
                nanos -= NANOS_PER_SEC;
                secs = match secs.checked_add(1) {
                    None => return None,
                    Some(secs) => secs,
                };
            } else if nanos <= -NANOS_PER_SEC {
                nanos += NANOS_PER_SEC;
                secs = match secs.checked_sub(1) {
                    None => return None,
                    Some(secs) => secs,
                };
            }
            if secs != 0 && nanos != 0 && secs.signum() != (nanos.signum() as i64) {
                if secs < 0 {
                    debug_assert!(nanos > 0);
                    // OK because secs<0.
                    secs += 1;
                    // OK because nanos>0.
                    nanos -= NANOS_PER_SEC;
                } else {
                    debug_assert!(secs > 0);
                    debug_assert!(nanos < 0);
                    // OK because secs>0.
                    secs -= 1;
                    // OK because nanos<0.
                    nanos += NANOS_PER_SEC;
                }
            }
        }
        Some(TimeDelta::new_unchecked(secs, nanos))
    }

    /// Add two signed durations together. If overflow occurs, then arithmetic saturates.
    pub const fn saturating_add(self, rhs: TimeDelta) -> TimeDelta {
        let Some(sum) = self.checked_add(rhs) else {
            return if rhs.is_negative() { TimeDelta::MIN } else { TimeDelta::MAX };
        };
        sum
    }

    /// Subtract one signed duration from another. If overflow occurs, then `None` is returned.
    pub const fn checked_sub(self, rhs: TimeDelta) -> Option<TimeDelta> {
        let Some(rhs) = rhs.checked_neg() else {
            return None;
        };
        self.checked_add(rhs)
    }

    /// Add two signed durations together. If overflow occurs, then arithmetic saturates.
    pub const fn saturating_sub(self, rhs: TimeDelta) -> TimeDelta {
        let Some(diff) = self.checked_sub(rhs) else {
            return if rhs.is_positive() { TimeDelta::MIN } else { TimeDelta::MAX };
        };
        diff
    }
}

/// # Operations involving integers.
impl TimeDelta {
    /// Multiply this signed duration by an integer.
    /// If the multiplication overflows, then `None` is returned.
    pub const fn checked_mul(self, rhs: i32) -> Option<TimeDelta> {
        let rhs = rhs as i64;
        // Multiplying any two i32 values never overflows an i64.
        let nanos = (self.nanos as i64) * rhs;
        // OK since NANOS_PER_SEC!={-1,0}.
        let addsecs = nanos / (NANOS_PER_SEC as i64);
        // OK since NANOS_PER_SEC!={-1,0}.
        let nanos = (nanos % (NANOS_PER_SEC as i64)) as i32;
        let Some(secs) = self.secs.checked_mul(rhs) else {
            return None;
        };
        let Some(secs) = secs.checked_add(addsecs) else {
            return None;
        };
        Some(TimeDelta::new_unchecked(secs, nanos))
    }

    /// Multiply this signed duration by an integer.
    ///
    /// If the multiplication overflows, then the result saturates to either
    /// the minimum or maximum duration depending on the sign of the product.
    pub const fn saturating_mul(self, rhs: i32) -> TimeDelta {
        let Some(product) = self.checked_mul(rhs) else {
            let sign = (self.signum() as i64) * (rhs as i64).signum();
            return if sign.is_negative() { TimeDelta::MIN } else { TimeDelta::MAX };
        };
        product
    }

    /// Divide this duration by an integer. If the division overflows, then `None` is returned.
    pub const fn checked_div(self, rhs: i32) -> Option<TimeDelta> {
        if rhs == 0 || (self.secs == i64::MIN && rhs == -1) {
            return None;
        }
        // OK since rhs!={-1,0}.
        let secs = self.secs / (rhs as i64);
        // OK since rhs!={-1,0}.
        let addsecs = self.secs % (rhs as i64);
        // OK since rhs!=0 and self.nanos>i32::MIN.
        let mut nanos = self.nanos / rhs;
        // OK since rhs!=0 and self.nanos>i32::MIN.
        let addnanos = self.nanos % rhs;
        let leftover_nanos = (addsecs * (NANOS_PER_SEC as i64)) + (addnanos as i64);
        nanos += (leftover_nanos / (rhs as i64)) as i32;
        debug_assert!(nanos < NANOS_PER_SEC);
        Some(TimeDelta::new_unchecked(secs, nanos))
    }
}

/// # Operations involving floating-point numbers.
impl TimeDelta {
    /// Returns the number of seconds, with a possible fractional nanosecond component.
    pub const fn as_secs_f64(&self) -> f64 {
        (self.secs as f64) + ((self.nanos as f64) / (NANOS_PER_SEC as f64))
    }

    /// Returns the number of seconds, with a possible fractional nanosecond component.
    pub const fn as_secs_f32(&self) -> f32 {
        (self.secs as f32) + ((self.nanos as f32) / (NANOS_PER_SEC as f32))
    }

    /// Returns a signed duration corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// # Panics
    /// Panics if the given float overflows the minimum or maximum signed duration values.
    pub fn from_secs_f64(secs: f64) -> TimeDelta {
        TimeDelta::try_from_secs_f64(secs).expect("finite and in-bounds f64")
    }
    /// Returns a signed duration corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// # Panics
    /// Panics if the given float overflows the minimum or maximum signed duration values.
    pub fn from_secs_f32(secs: f32) -> TimeDelta {
        TimeDelta::try_from_secs_f32(secs).expect("finite and in-bounds f32")
    }

    /// Returns a signed duration corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// If the given float overflows the minimum or maximum signed duration
    /// values, then an error is returned.
    ///
    /// # Features
    /// - Uses `std` or `_float_f64` when available, leveraging `trunc`, `fract`, and `round`
    ///   for precise, bias-free conversion.
    /// - In strict `no_std` mode, manually rounds using integer arithmetic, ensuring correctness
    ///   while lacking exact fractional nanosecond precision.
    #[rustfmt::skip]
    pub fn try_from_secs_f64(secs: f64) -> Result<TimeDelta, &'static str> {
        if !secs.is_finite() {
            return Err("could not convert non-finite seconds {secs} to signed duration"); }
        if secs < (i64::MIN as f64) {
            return Err("floating point seconds {secs} overflows TimeDelta::MIN"); }
        if secs > (i64::MAX as f64) {
            return Err("floating point seconds {secs} overflows TimeDelta::MAX"); }
        let (isecs, nanos);
        #[cfg(any(feature = "std", feature = "_float_f64"))] {
            isecs = secs.trunc() as i64;
            nanos = (secs.fract() * NANOS_PER_SEC as f64).round() as i32;
        }
        #[cfg(not(any(feature = "std", feature = "_float_f64")))] {
            let secs_rounded = if secs >= 0.0 { secs + 0.5 }  // Round normally
            else { secs - 0.5 };  // Round away from zero for negatives
            isecs = secs_rounded as i64;
            nanos = ((secs_rounded - isecs as f64) * NANOS_PER_SEC as f64) as i32;
        }
        Ok(TimeDelta::new_unchecked(isecs, nanos))
    }

    /// Returns a signed duration corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// If the given float overflows the minimum or maximum signed duration
    /// values, then an error is returned.
    #[rustfmt::skip]
    pub fn try_from_secs_f32(secs: f32) -> Result<TimeDelta, &'static str> {
        if !secs.is_finite() {
            return Err("could not convert non-finite seconds {secs} to signed duration"); }
        if secs < (i64::MIN as f32) {
            return Err("floating point seconds {secs} overflows TimeDelta::MIN"); }
        if secs > (i64::MAX as f32) {
            return Err("floating point seconds {secs} overflows TimeDelta::MAX"); }
        let (isecs, nanos);
        #[cfg(any(feature = "std", feature = "_float_f32"))] {
            isecs = secs.trunc() as i64;
            nanos = (secs.fract() * NANOS_PER_SEC as f32).round() as i32;
        }
        #[cfg(not(any(feature = "std", feature = "_float_f32")))] {
            let secs_rounded = if secs >= 0.0 { secs + 0.5 }  // Round normally
            else { secs - 0.5 };  // Round away from zero for negatives
            isecs = secs_rounded as i64;
            nanos = ((secs_rounded - isecs as f32) * NANOS_PER_SEC as f32) as i32;
        }
        Ok(TimeDelta::new_unchecked(isecs, nanos))
    }

    /// Returns the number of milliseconds, with a possible fractional nanosecond component.
    pub const fn as_millis_f64(&self) -> f64 {
        ((self.secs as f64) * (MILLIS_PER_SEC as f64))
            + ((self.nanos as f64) / (NANOS_PER_MILLI as f64))
    }
    /// Returns the number of milliseconds, with a possible fractional nanosecond component.
    pub const fn as_millis_f32(&self) -> f32 {
        ((self.secs as f32) * (MILLIS_PER_SEC as f32))
            + ((self.nanos as f32) / (NANOS_PER_MILLI as f32))
    }

    /// Returns a signed duration corresponding to the number of milliseconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// # Panics
    /// Panics if the given float overflows the minimum or maximum signed duration values.
    pub fn from_millis_f64(millis: f64) -> TimeDelta {
        TimeDelta::try_from_millis_f64(millis).expect("finite and in-bounds f64")
    }
    /// Compile-time friendly version of `try_from_millis_f64`.
    pub const fn const_from_millis_f64(millis: f64) -> TimeDelta {
        unwrap![ok_expect TimeDelta::const_try_from_millis_f64(millis), "finite and in-bounds f64"]
    }

    /// Returns a signed duration corresponding to the number of milliseconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// If the given float overflows the minimum or maximum signed duration
    /// values, then an error is returned.
    ///
    /// # Features
    /// - Uses `std` or `_float_f64` when available, leveraging `round`, `div_euclid`,
    ///   and `rem_euclid` for precise, bias-free conversion.
    /// - In strict `no_std` mode, manually rounds using integer arithmetic, ensuring correctness
    ///   while lacking exact Euclidean division for negatives.
    #[rustfmt::skip]
    pub fn try_from_millis_f64(millis: f64) -> Result<TimeDelta, &'static str> {
        if !millis.is_finite() {
            return Err("could not convert non-finite milliseconds {millis} to signed duration"); }
        if millis < (i64::MIN as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MIN"); }
        if millis > (i64::MAX as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MAX"); }
        let (millis_rounded, secs, nanos);
        #[cfg(any(feature = "std", feature = "_float_f64"))]
        {
            millis_rounded = millis.round();
            secs = millis_rounded.div_euclid(1_000.0) as i64;
            nanos = (millis_rounded.rem_euclid(1_000.0) * 1_000_000.0) as i32;
        }
        #[cfg(not(any(feature = "std", feature = "_float_f64")))]
        {
            millis_rounded = if millis >= 0.0 { millis + 0.5 }  // Round normally
            else { millis - 0.5 };  // Round away from zero for negatives
            let millis_i64 = millis_rounded as i64;
            secs = millis_i64 / MILLIS_PER_SEC;
            nanos = ((millis_i64 % MILLIS_PER_SEC) * NANOS_PER_MILLI as i64) as i32;
        }
        Ok(TimeDelta::new_unchecked(secs, nanos))
    }
    /// Compile-time friendly version of `try_from_millis_f64`.
    ///
    /// # Features
    /// - Uses `_float_f64` if enabled, leveraging [`Float`]'s `const_round`, `div_euclid`,
    ///   and `rem_euclid` for precise, bias-free conversion.
    /// - Without `_float_f64`, rounds manually using integer arithmetic, preventing
    ///   systematic underestimation but lacking exact Euclidean division for negatives.
    #[rustfmt::skip]
    pub const fn const_try_from_millis_f64(millis: f64) -> Result<TimeDelta, &'static str> {
        if !millis.is_finite() {
            return Err("could not convert non-finite milliseconds {millis} to signed duration"); }
        if millis < (i64::MIN as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MIN"); }
        if millis > (i64::MAX as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MAX"); }
        let (millis_rounded, secs, nanos);
        #[cfg(feature = "_float_f64")]
        {
            millis_rounded = Float(millis).const_round().0 as i64;
            secs = Float(millis_rounded as f64).div_euclid(MILLIS_PER_SEC as f64).0 as i64;
            nanos = Float(millis_rounded as f64).rem_euclid(MILLIS_PER_SEC as f64).0 as i64
            * NANOS_PER_MILLI as i64;
        }
        #[cfg(not(feature = "_float_f64"))]
        {
            // millis_rounded = millis as i64; // slight systematic underestimation
            millis_rounded = if millis >= 0.0 { (millis + 0.5) as i64 }  // Round normally
            else { (millis - 0.5) as i64 };  // Round away from zero for negatives
            secs = millis_rounded / MILLIS_PER_SEC;
            nanos = (millis_rounded % MILLIS_PER_SEC) * NANOS_PER_MILLI as i64;
        }
        Ok(TimeDelta::new_unchecked(secs, nanos as i32))
    }

    /* ops */

    /// Returns the result of multiplying this duration by the given 64-bit float.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f64"))))]
    pub fn mul_f64(self, rhs: f64) -> TimeDelta {
        TimeDelta::from_secs_f64(rhs * self.as_secs_f64())
    }
    /// Returns the result of multiplying this duration by the given 32-bit float.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    #[cfg(any(feature = "std", feature = "_float_f32"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f32"))))]
    pub fn mul_f32(self, rhs: f32) -> TimeDelta {
        TimeDelta::from_secs_f32(rhs * self.as_secs_f32())
    }

    /// Returns the result of dividing this duration by the given `f64`.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f64"))))]
    pub fn div_f64(self, rhs: f64) -> TimeDelta {
        TimeDelta::from_secs_f64(self.as_secs_f64() / rhs)
    }
    /// Returns the result of dividing this duration by the given `f32`.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    #[cfg(any(feature = "std", feature = "_float_f32"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f32"))))]
    pub fn div_f32(self, rhs: f32) -> TimeDelta {
        TimeDelta::from_secs_f32(self.as_secs_f32() / rhs)
    }

    /// Divides this signed duration by another signed duration.
    pub const fn div_delta_f64(self, rhs: TimeDelta) -> f64 {
        let lhs_nanos = (self.secs as f64) * (NANOS_PER_SEC as f64) + (self.nanos as f64);
        let rhs_nanos = (rhs.secs as f64) * (NANOS_PER_SEC as f64) + (rhs.nanos as f64);
        lhs_nanos / rhs_nanos
    }
    /// Divides this signed duration by another signed duration.
    pub const fn div_delta_f32(self, rhs: TimeDelta) -> f32 {
        let lhs_nanos = (self.secs as f32) * (NANOS_PER_SEC as f32) + (self.nanos as f32);
        let rhs_nanos = (rhs.secs as f32) * (NANOS_PER_SEC as f32) + (rhs.nanos as f32);
        lhs_nanos / rhs_nanos
    }
}

/// Additional methods not found in the standard library.
///
/// In most cases, these APIs exist as a result of the fact that this duration is signed.
#[rustfmt::skip]
impl TimeDelta {
    /// Returns the number of whole hours in this duration.
    ///
    /// The value returned is negative when the duration is negative.
    ///
    /// This does not include any fractional component corresponding to units
    /// less than an hour.
    pub const fn as_hours(&self) -> i64 { self.as_secs() / (MINS_PER_HOUR * SECS_PER_MINUTE) }

    /// Returns the number of whole minutes in this duration.
    ///
    /// The value returned is negative when the duration is negative.
    ///
    /// This does not include any fractional component corresponding to units less than a minute.
    pub const fn as_mins(&self) -> i64 { self.as_secs() / SECS_PER_MINUTE }

    /// Returns the absolute value of this signed duration.
    ///
    /// If this duration is positive, then this returns the original duration unchanged.
    ///
    /// # Panics
    /// This panics when the seconds component of this signed duration is equal to `i64::MIN`.
    pub const fn abs(self) -> TimeDelta {
        TimeDelta::new_unchecked(self.secs.abs(), self.nanos.abs())
    }

    /// Returns the absolute value of this signed duration as a [`Duration`].
    ///
    /// This method cannot panic because the absolute value of `TimeDelta::MIN`
    /// is always representable in a `Duration`.
    pub const fn abs_duration(self) -> Duration {
        Duration::new(self.secs.unsigned_abs(), self.nanos.unsigned_abs())
    }

    /// Returns the negative absolute value of this signed duration.
    ///
    /// If this duration is negative, then this returns the original duration unchanged.
    ///
    /// # Panics
    /// This panics when the seconds component of this signed duration is equal to `i64::MIN`.
    pub const fn neg_abs(self) -> TimeDelta {
        TimeDelta::new_unchecked(-self.secs.abs(), -self.nanos.abs())
    }

    /// Returns this duration with its sign flipped.
    ///
    /// If this duration is zero, then this returns the duration unchanged.
    ///
    /// This returns none if the negation does not exist. This occurs in
    /// precisely the cases when [`TimeDelta::as_secs`] is equal to `i64::MIN`.
    pub const fn checked_neg(self) -> Option<TimeDelta> {
        let Some(secs) = self.secs.checked_neg() else { return None; };
        // -self.nanos always OK because `-999_999_999 <= self.nanos <= 999_999_999`.
        Some(TimeDelta::new_unchecked(secs, -self.nanos))
    }

    /// Returns a number that represents the sign of this duration.
    ///
    /// * When [`TimeDelta::is_zero`] is true, this returns `0`.
    /// * When [`TimeDelta::is_positive`] is true, this returns `1`.
    /// * When [`TimeDelta::is_negative`] is true, this returns `-1`.
    ///
    /// The above cases are mutually exclusive.
    pub const fn signum(self) -> i8 {
        if self.is_zero() { 0 }
        else if self.is_positive() { 1 }
        else { debug_assert!(self.is_negative()); -1 }
    }

    /// Returns true when this duration is positive. That is, greater than [`TimeDelta::ZERO`].
    pub const fn is_positive(&self) -> bool { self.secs.is_positive() || self.nanos.is_positive() }
    /// Returns true when this duration is negative. That is, less than [`TimeDelta::ZERO`].
    pub const fn is_negative(&self) -> bool { self.secs.is_negative() || self.nanos.is_negative() }
}

/* conversions */

impl TryFrom<Duration> for TimeDelta {
    type Error = &'static str;

    fn try_from(d: Duration) -> Result<TimeDelta, Self::Error> {
        let secs = i64::try_from(d.as_secs())
            .map_err(|_| "seconds in unsigned duration {d:?} overflowed i64")?;
        // Guaranteed to succeed since 0<=nanos<=999,999,999.
        let nanos = i32::try_from(d.subsec_nanos()).unwrap();
        Ok(TimeDelta::new_unchecked(secs, nanos))
    }
}

impl TryFrom<TimeDelta> for Duration {
    type Error = &'static str;

    fn try_from(sd: TimeDelta) -> Result<Duration, Self::Error> {
        // This isn't needed, but improves error messages.
        if sd.is_negative() {
            return Err("cannot convert negative duration `{sd:?}` to \
                 unsigned `std::time::Duration`");
        }
        let secs = u64::try_from(sd.as_secs())
            .map_err(|_| "seconds in signed duration {sd:?} overflowed u64")?;
        // Guaranteed to succeed because the above only succeeds
        // when `sd` is non-negative. And when `sd` is non-negative,
        // we are guaranteed that 0<=nanos<=999,999,999.
        let nanos = u32::try_from(sd.subsec_nanos()).unwrap();
        Ok(Duration::new(secs, nanos))
    }
}

#[rustfmt::skip]
#[cfg(feature = "js")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "js")))]
impl TimeDelta {
    /// Converts a `JsInstant` into a `TimeDelta` relative to the time origin.
    pub fn from_js(js: JsInstant) -> Self { Self::from_millis_f64(js.ms_timestamp) }
    /// Converts a `JsInstant` into a `TimeDelta` relative to the time origin.
    pub const fn const_from_js(js: JsInstant) -> Self {
        Self::const_from_millis_f64(js.ms_timestamp)
    }
    /// Converts a `TimeDelta` into a `JsInstant`, interpreting it as an absolute timestamp.
    pub const fn to_js(self) -> JsInstant { JsInstant::new(self.as_millis_f64()) }
}
#[rustfmt::skip]
#[cfg(feature = "dep_jiff")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_jiff")))]
mod impl_jiff {
    use {super::TimeDelta, ::jiff::SignedDuration};
    impl TimeDelta {
        /// Converts [`SignedDuration`] into [`TimeDelta`].
        pub const fn from_jiff(from: SignedDuration) -> TimeDelta {
            TimeDelta::new(from.as_secs(), from.subsec_nanos())
        }
        /// Converts [`TimeDelta`] into [`SignedDuration`].
        pub const fn to_jiff(self) -> SignedDuration { SignedDuration::new(self.secs, self.nanos) }
    }
    impl From<SignedDuration> for TimeDelta {
        fn from(from: SignedDuration) -> TimeDelta { Self::from_jiff(from) }
    }
    impl From<TimeDelta> for SignedDuration {
        fn from(from: TimeDelta) -> SignedDuration { from.to_jiff() }
    }
}

#[rustfmt::skip]
mod impl_ops {
    use crate::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, TimeDelta};
    impl Neg for TimeDelta {
        type Output = TimeDelta;
        fn neg(self) -> TimeDelta {
            self.checked_neg().expect("overflow when negating signed duration")
        }
    }
    impl Add for TimeDelta {
        type Output = TimeDelta;
        fn add(self, rhs: TimeDelta) -> TimeDelta {
            self.checked_add(rhs).expect("overflow when adding signed durations")
        }
    }
    impl AddAssign for TimeDelta {
        fn add_assign(&mut self, rhs: TimeDelta) { *self = *self + rhs; }
    }
    impl Sub for TimeDelta {
        type Output = TimeDelta;
        fn sub(self, rhs: TimeDelta) -> TimeDelta {
            self.checked_sub(rhs).expect("overflow when subtracting signed durations")
        }
    }
    impl SubAssign for TimeDelta {
        fn sub_assign(&mut self, rhs: TimeDelta) { *self = *self - rhs; }
    }
    impl Mul<i32> for TimeDelta {
        type Output = TimeDelta;
        fn mul(self, rhs: i32) -> TimeDelta {
            self.checked_mul(rhs).expect("overflow when multiplying signed duration by scalar")
        }
    }
    impl Mul<TimeDelta> for i32 {
        type Output = TimeDelta;
        fn mul(self, rhs: TimeDelta) -> TimeDelta { rhs * self }
    }
    impl MulAssign<i32> for TimeDelta {
        fn mul_assign(&mut self, rhs: i32) { *self = *self * rhs; }
    }
    impl Div<i32> for TimeDelta {
        type Output = TimeDelta;
        fn div(self, rhs: i32) -> TimeDelta {
            self.checked_div(rhs).expect("overflow when dividing signed duration by scalar")
        }
    }
    impl DivAssign<i32> for TimeDelta {
        fn div_assign(&mut self, rhs: i32) { *self = *self / rhs; }
    }
}
