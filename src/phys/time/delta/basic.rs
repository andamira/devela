// devela::phys::time::delta::basic
//
// TOC
// - basic methods
// - additional methods

use super::*;

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

    /// Creates a new time delta without handling nanosecond overflow.
    ///
    /// This might produce tighter code in some cases.
    ///
    /// In debug mode only, when `|nanos|` is greater than or equal to 1 second.
    ///
    /// This is not exported so that code outside this module can rely on
    /// `|nanos|` being less than a second for purposes of memory safety.
    pub(super) const fn new_unchecked(secs: i64, nanos: i32) -> TimeDelta {
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
    /// to construct the full range of possible time delta values. In
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
    /// to construct the full range of possible time delta values. In
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
    /// to construct the full range of possible time delta values. In
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

    /// Returns the absolute value of this time delta.
    ///
    /// If this duration is positive, then this returns the original duration unchanged.
    ///
    /// # Panics
    /// This panics when the seconds component of this time delta is equal to `i64::MIN`.
    pub const fn abs(self) -> TimeDelta {
        TimeDelta::new_unchecked(self.secs.abs(), self.nanos.abs())
    }

    /// Returns the absolute value of this time delta as a [`Duration`].
    ///
    /// This method cannot panic because the absolute value of `TimeDelta::MIN`
    /// is always representable in a `Duration`.
    pub const fn abs_duration(self) -> Duration {
        Duration::new(self.secs.unsigned_abs(), self.nanos.unsigned_abs())
    }

    /// Returns the negative absolute value of this time delta.
    ///
    /// If this duration is negative, then this returns the original duration unchanged.
    ///
    /// # Panics
    /// This panics when the seconds component of this time delta is equal to `i64::MIN`.
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
