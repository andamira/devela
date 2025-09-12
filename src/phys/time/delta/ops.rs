// devela::phys::time::delta::ops
//
// TOC
// - operations
// - operations involving integers
// - operations involving floats
// - impl_ops

use super::*;
use crate::unwrap;
#[allow(unused)]
use crate::{ExtFloat, Float};

/// # Operations.
impl TimeDelta {
    /// Add two time deltas together. If overflow occurs, then `None` is returned.
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

    /// Add two time deltas together. If overflow occurs, then arithmetic saturates.
    pub const fn saturating_add(self, rhs: TimeDelta) -> TimeDelta {
        let Some(sum) = self.checked_add(rhs) else {
            return if rhs.is_negative() { TimeDelta::MIN } else { TimeDelta::MAX };
        };
        sum
    }

    /// Subtract one time delta from another. If overflow occurs, then `None` is returned.
    pub const fn checked_sub(self, rhs: TimeDelta) -> Option<TimeDelta> {
        let Some(rhs) = rhs.checked_neg() else {
            return None;
        };
        self.checked_add(rhs)
    }

    /// Add two time deltas together. If overflow occurs, then arithmetic saturates.
    pub const fn saturating_sub(self, rhs: TimeDelta) -> TimeDelta {
        let Some(diff) = self.checked_sub(rhs) else {
            return if rhs.is_positive() { TimeDelta::MIN } else { TimeDelta::MAX };
        };
        diff
    }
}

/// # Operations involving integers.
impl TimeDelta {
    /// Multiply this time delta by an integer.
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

    /// Multiply this time delta by an integer.
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

    /// Returns a time delta corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// # Panics
    /// Panics if the given float overflows the minimum or maximum time delta values.
    pub fn from_secs_f64(secs: f64) -> TimeDelta {
        TimeDelta::try_from_secs_f64(secs).expect("finite and in-bounds f64")
    }
    /// Returns a time delta corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// # Panics
    /// Panics if the given float overflows the minimum or maximum time delta values.
    pub fn from_secs_f32(secs: f32) -> TimeDelta {
        TimeDelta::try_from_secs_f32(secs).expect("finite and in-bounds f32")
    }

    /// Returns a time delta corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// If the given float overflows the minimum or maximum time delta values,
    /// then an error is returned.
    //
    // TODO: RETHINK
    // # Features
    // - Uses `std` or `_float_f64` when available, leveraging
    //   `trunc`, `fract`, and `round` for precise, bias-free conversion.
    // - In strict `no_std` mode, manually rounds using integer arithmetic,
    //   ensuring correctness while lacking exact fractional nanosecond precision.
    #[rustfmt::skip]
    pub fn try_from_secs_f64(secs: f64) -> Result<TimeDelta, &'static str> {
        if !secs.is_finite() {
            return Err("could not convert non-finite seconds {secs} to time delta"); }
        if secs < (i64::MIN as f64) {
            return Err("floating point seconds {secs} overflows TimeDelta::MIN"); }
        if secs > (i64::MAX as f64) {
            return Err("floating point seconds {secs} overflows TimeDelta::MAX"); }
        let (isecs, nanos);
        // #[cfg(any(feature = "std", feature = "_float_f64"))] {
        isecs = secs.trunc() as i64;
        nanos = (secs.fract() * NANOS_PER_SEC as f64).round() as i32;
        // }
        // #[cfg(not(any(feature = "std", feature = "_float_f64")))] {
        //     let secs_rounded = if secs >= 0.0 { secs + 0.5 }  // Round normally
        //     else { secs - 0.5 };  // Round away from zero for negatives
        //     isecs = secs_rounded as i64;
        //     nanos = ((secs_rounded - isecs as f64) * NANOS_PER_SEC as f64) as i32;
        // }
        Ok(TimeDelta::new_unchecked(isecs, nanos))
    }

    /// Returns a time delta corresponding to the number of seconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// If the given float overflows the minimum or maximum time delta
    /// values, then an error is returned.
    #[rustfmt::skip]
    pub fn try_from_secs_f32(secs: f32) -> Result<TimeDelta, &'static str> {
        if !secs.is_finite() {
            return Err("could not convert non-finite seconds {secs} to time delta"); }
        if secs < (i64::MIN as f32) {
            return Err("floating point seconds {secs} overflows TimeDelta::MIN"); }
        if secs > (i64::MAX as f32) {
            return Err("floating point seconds {secs} overflows TimeDelta::MAX"); }
        let (isecs, nanos);
        isecs = secs.trunc() as i64;
        nanos = (secs.fract() * NANOS_PER_SEC as f32).round() as i32;
        // #[cfg(not(any(feature = "std", feature = "_float_f32")))] {
        //     let secs_rounded = if secs >= 0.0 { secs + 0.5 }  // Round normally
        //     else { secs - 0.5 };  // Round away from zero for negatives
        //     isecs = secs_rounded as i64;
        //     nanos = ((secs_rounded - isecs as f32) * NANOS_PER_SEC as f32) as i32;
        // }
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

    /// Returns a time delta corresponding to the number of milliseconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// # Panics
    /// Panics if the given float overflows the minimum or maximum time delta values.
    pub fn from_millis_f64(millis: f64) -> TimeDelta {
        TimeDelta::try_from_millis_f64(millis).expect("finite and in-bounds f64")
    }
    /// Compile-time friendly version of `try_from_millis_f64`.
    pub const fn const_from_millis_f64(millis: f64) -> TimeDelta {
        unwrap![ok_expect TimeDelta::const_try_from_millis_f64(millis), "finite and in-bounds f64"]
    }

    /// Returns a time delta corresponding to the number of milliseconds.
    ///
    /// The number given may have a fractional nanosecond component.
    ///
    /// If the given float overflows the minimum or maximum time delta
    /// values, then an error is returned.
    //
    // TODO: RETHINK
    // # Features
    // - Uses `std` or `_float_f64` when available, leveraging `round`, `div_euclid`,
    //   and `rem_euclid` for precise, bias-free conversion.
    // - In strict `no_std` mode, manually rounds using integer arithmetic, ensuring correctness
    //   while lacking exact Euclidean division for negatives.
    #[rustfmt::skip]
    pub fn try_from_millis_f64(millis: f64) -> Result<TimeDelta, &'static str> {
        if !millis.is_finite() {
            return Err("could not convert non-finite milliseconds {millis} to time delta"); }
        if millis < (i64::MIN as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MIN"); }
        if millis > (i64::MAX as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MAX"); }
        let (millis_rounded, secs, nanos);
        // #[cfg(any(feature = "std", feature = "_float_f64"))]
        // {
        millis_rounded = millis.round();
        secs = millis_rounded.div_euclid(1_000.0) as i64;
        nanos = (millis_rounded.rem_euclid(1_000.0) * 1_000_000.0) as i32;
        // }
        // #[cfg(not(any(feature = "std", feature = "_float_f64")))]
        // {
        //     millis_rounded = if millis >= 0.0 { millis + 0.5 }  // Round normally
        //     else { millis - 0.5 };  // Round away from zero for negatives
        //     let millis_i64 = millis_rounded as i64;
        //     secs = millis_i64 / MILLIS_PER_SEC;
        //     nanos = ((millis_i64 % MILLIS_PER_SEC) * NANOS_PER_MILLI as i64) as i32;
        // }
        Ok(TimeDelta::new_unchecked(secs, nanos))
    }
    /// Compile-time friendly version of `try_from_millis_f64`.
    //
    // TODO: RETHINK
    // # Features
    // - Uses `_float_f64` if enabled, leveraging [`Float`]'s `const_round`, `div_euclid`,
    //   and `rem_euclid` for precise, bias-free conversion.
    // - Without `_float_f64`, rounds manually using integer arithmetic, preventing
    //   systematic underestimation but lacking exact Euclidean division for negatives.
    #[rustfmt::skip]
    pub const fn const_try_from_millis_f64(millis: f64) -> Result<TimeDelta, &'static str> {
        if !millis.is_finite() {
            return Err("could not convert non-finite milliseconds {millis} to time delta"); }
        if millis < (i64::MIN as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MIN"); }
        if millis > (i64::MAX as f64) {
            return Err("floating point milliseconds {millis} overflows TimeDelta::MAX"); }
        let (millis_rounded, secs, nanos);
        millis_rounded = Float(millis).const_round().0 as i64;
        secs = Float(millis_rounded as f64).div_euclid(MILLIS_PER_SEC as f64).0 as i64;
        nanos = Float(millis_rounded as f64).rem_euclid(MILLIS_PER_SEC as f64).0 as i64
            * NANOS_PER_MILLI as i64;
        // #[cfg(not(feature = "_float_f64"))]
        // {
        //     // millis_rounded = millis as i64; // slight systematic underestimation
        //     millis_rounded = if millis >= 0.0 { (millis + 0.5) as i64 }  // Round normally
        //     else { (millis - 0.5) as i64 };  // Round away from zero for negatives
        //     secs = millis_rounded / MILLIS_PER_SEC;
        //     nanos = (millis_rounded % MILLIS_PER_SEC) * NANOS_PER_MILLI as i64;
        // }
        Ok(TimeDelta::new_unchecked(secs, nanos as i32))
    }

    /* ops */

    /// Returns the result of multiplying this duration by the given 64-bit float.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    pub fn mul_f64(self, rhs: f64) -> TimeDelta {
        TimeDelta::from_secs_f64(rhs * self.as_secs_f64())
    }
    /// Returns the result of multiplying this duration by the given 32-bit float.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    pub fn mul_f32(self, rhs: f32) -> TimeDelta {
        TimeDelta::from_secs_f32(rhs * self.as_secs_f32())
    }

    /// Returns the result of dividing this duration by the given `f64`.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    pub fn div_f64(self, rhs: f64) -> TimeDelta {
        TimeDelta::from_secs_f64(self.as_secs_f64() / rhs)
    }
    /// Returns the result of dividing this duration by the given `f32`.
    ///
    /// # Panics
    /// This panics if the result is not finite or overflows a `TimeDelta`.
    pub fn div_f32(self, rhs: f32) -> TimeDelta {
        TimeDelta::from_secs_f32(self.as_secs_f32() / rhs)
    }

    /// Divides this time delta by another time delta.
    pub const fn div_delta_f64(self, rhs: TimeDelta) -> f64 {
        let lhs_nanos = (self.secs as f64) * (NANOS_PER_SEC as f64) + (self.nanos as f64);
        let rhs_nanos = (rhs.secs as f64) * (NANOS_PER_SEC as f64) + (rhs.nanos as f64);
        lhs_nanos / rhs_nanos
    }
    /// Divides this time delta by another time delta.
    pub const fn div_delta_f32(self, rhs: TimeDelta) -> f32 {
        let lhs_nanos = (self.secs as f32) * (NANOS_PER_SEC as f32) + (self.nanos as f32);
        let rhs_nanos = (rhs.secs as f32) * (NANOS_PER_SEC as f32) + (rhs.nanos as f32);
        lhs_nanos / rhs_nanos
    }
}

// IMPROVE use num error types
#[rustfmt::skip]
mod impl_ops {
    use crate::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, TimeDelta};
    impl Neg for TimeDelta {
        type Output = TimeDelta;
        fn neg(self) -> TimeDelta {
            self.checked_neg().expect("overflow when negating time delta")
        }
    }
    impl Add for TimeDelta {
        type Output = TimeDelta;
        fn add(self, rhs: TimeDelta) -> TimeDelta {
            self.checked_add(rhs).expect("overflow when adding time deltas")
        }
    }
    impl AddAssign for TimeDelta {
        fn add_assign(&mut self, rhs: TimeDelta) { *self = *self + rhs; }
    }
    impl Sub for TimeDelta {
        type Output = TimeDelta;
        fn sub(self, rhs: TimeDelta) -> TimeDelta {
            self.checked_sub(rhs).expect("overflow when subtracting time deltas")
        }
    }
    impl SubAssign for TimeDelta {
        fn sub_assign(&mut self, rhs: TimeDelta) { *self = *self - rhs; }
    }
    impl Mul<i32> for TimeDelta {
        type Output = TimeDelta;
        fn mul(self, rhs: i32) -> TimeDelta {
            self.checked_mul(rhs).expect("overflow when multiplying time delta by scalar")
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
            self.checked_div(rhs).expect("overflow when dividing time delta by scalar")
        }
    }
    impl DivAssign<i32> for TimeDelta {
        fn div_assign(&mut self, rhs: i32) { *self = *self / rhs; }
    }
}
