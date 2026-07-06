// devela/ui/view/scale/_helper.rs
//
//! Integer helpers for UI layout and projection.
//
// IMPROVE: eventually move to num

use crate::UiRound::{self, Ceil, Floor, Inward, Nearest, Outward};

pub(crate) struct UiNum;

#[rustfmt::skip]
impl UiNum {
    /// Returns the absolute magnitude as an unsigned integer.
    const fn abs_i64_u64(n: i64) -> u64 {
        if n < 0 { n.wrapping_neg() as u64 } else { n as u64 }
    }

    /// Divides two integers, rounding toward negative infinity.
    ///
    /// Requires `d != 0`.
    pub const fn div_floor_i64(n: i64, d: i64) -> i64 {
        let (q, r) = (n / d, n % d);
        if r != 0 && ((r > 0) != (d > 0)) { q - 1 } else { q }
    }
    /// Divides two integers, rounding toward positive infinity.
    ///
    /// Requires `d != 0`.
    pub const fn div_ceil_i64(n: i64, d: i64) -> i64 {
        let (q, r) = (n / d, n % d);
        if r != 0 && ((r > 0) == (d > 0)) { q + 1 } else { q }
    }
    /// Divides two integers, rounding to the nearest result.
    ///
    /// Halfway cases round away from zero. Requires `d != 0`.
    pub const fn div_nearest_i64(n: i64, d: i64) -> i64 {
        let (q, r) = (n / d, n % d);
        if r == 0 { q }
        else {
            let ar = Self::abs_i64_u64(r);
            let ad = Self::abs_i64_u64(d);
            if ar.saturating_mul(2) >= ad { if (n < 0) != (d < 0) { q - 1 } else { q + 1 } }
            else { q }
        }
    }

    /// Divides two integers using a scalar rounding policy.
    ///
    /// `Inward` and `Outward` are rectangle policies;
    /// for scalar values they behave like `Nearest`.
    pub const fn div_round_scalar_i64(n: i64, d: i64, round: UiRound) -> i64 {
        match round {
            Floor => Self::div_floor_i64(n, d),
            Ceil => Self::div_ceil_i64(n, d),
            Nearest | Inward | Outward => Self::div_nearest_i64(n, d),
        }
    }

    /// Saturates an `i64` into the `i32` range.
    pub const fn saturating_i64_to_i32(n: i64) -> i32 {
        if n > i32::MAX as i64 { i32::MAX }
        else if n < i32::MIN as i64 { i32::MIN }
        else { n as i32 }
    }

    /// Divides two integers using a scalar rounding policy and returns `i32`.
    pub const fn round_div_scalar_i64_to_i32(n: i64, d: i64, round: UiRound) -> i32 {
        Self::saturating_i64_to_i32(Self::div_round_scalar_i64(n, d, round))
    }
}
