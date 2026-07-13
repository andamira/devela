// devela/ui/layout/unit.rs
//
//! Defines [`Lunit`] and metric aliases.
//

use crate::UiNum;

crate::bound_int! {
    #[doc = crate::_tags!(ui layout quant)]
    /// Fixed-point scalar used for UI layout negotiation.
    #[doc = crate::_doc_meta!{
        location("ui/layout"),
        test_size_of(Lunit = 4|32),
    }]
    /// `Lunit` is expressed in logical layout space. It is not a device pixel,
    /// physical pixel, terminal cell, typographic point, or real-world length.
    ///
    /// Backend projection maps it to physical pixels, terminal cells, or other output units.
    pub struct Lunit: repr(i32 => i32);

    value_bits(32-4);
    range(symmetric);
    ops(all);

    impl {
        /// Number of internal layout quanta per logical UI pixel.
        pub const QUANTA_PER_LOGICAL_PX: i32 = 120;

        /// Creates a layout scalar from whole logical UI pixels.
        ///
        /// Saturates if the scaled value escapes the payload range.
        pub const fn px(px: i32) -> Self {
            Self::new_saturated_up((px as i64).saturating_mul(Self::QUANTA_PER_LOGICAL_PX as i64))
        }
        /// Creates a layout scalar from a fraction of a logical UI pixel.
        ///
        /// Rounds to the nearest representable quantum and saturates on overflow.
        pub const fn px_frac(num: i32, den: i32) -> Self {
            if den == 0 {
                return if num < 0 { Self::MIN } else if num > 0 { Self::MAX } else { Self::ZERO }; }
            let n = (num as i64).saturating_mul(Self::QUANTA_PER_LOGICAL_PX as i64);
            Self::new_saturated_up(UiNum::div_nearest_i64(n, den as i64))
        }

        /// Creates a layout scalar from internal layout quanta.
        pub const fn from_quanta(quanta: i32) -> Self { Self::new_saturated(quanta) }
        /// Returns the number of internal layout quanta.
        pub const fn quanta(self) -> i32 { self.get() }

        /// Returns this value clamped below by zero.
        pub const fn positive_part(self) -> Self {
            self.max_zero()
        }
        /// Subtracts `rhs`, returning zero instead of a negative result.
        pub const fn sub_floor_zero(self, rhs: Self) -> Self {
            self.sat_sub(rhs).max_zero()
        }

        /// Divides by an integer, rounding toward negative infinity.
        ///
        /// Saturates to the signed endpoint if `rhs == 0`.
        pub const fn div_floor_i32(self, rhs: i32) -> Self {
            if rhs == 0 { return if self.is_negative() { Self::MIN } else { Self::MAX }; }
            Self::new_saturated_up(UiNum::div_floor_i64(self.get() as i64, rhs as i64))
        }
        /// Divides by an integer, rounding toward positive infinity.
        ///
        /// Saturates to the signed endpoint if `rhs == 0`.
        pub const fn div_ceil_i32(self, rhs: i32) -> Self {
            if rhs == 0 { return if self.is_negative() { Self::MIN } else { Self::MAX }; }
            Self::new_saturated_up(UiNum::div_ceil_i64(self.get() as i64, rhs as i64))
        }
        /// Divides by an integer, rounding to the nearest result.
        ///
        /// Halfway cases round away from zero. Saturates to the signed endpoint
        /// if `rhs == 0`.
        pub const fn div_round_i32(self, rhs: i32) -> Self {
            if rhs == 0 { return if self.is_negative() { Self::MIN } else { Self::MAX }; }
            Self::new_saturated_up(UiNum::div_nearest_i64(self.get() as i64, rhs as i64))
        }

        /// Returns the base part size when splitting this value into `parts`.
        ///
        /// Returns zero if `parts <= 0`.
        pub const fn split_floor(self, parts: i32) -> Self {
            if parts <= 0 { Self::ZERO } else { self.div_floor_i32(parts) }
        }
        /// Returns the quantum remainder when splitting this value into `parts`.
        ///
        /// Returns zero if `parts <= 0`.
        pub const fn split_remainder(self, parts: i32) -> i32 {
            if parts <= 0 { 0 } else { self.get() % parts }
        }
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn px_construction() {
        assert_eq!(Lunit::px(0).quanta(), 0);
        assert_eq!(Lunit::px(1).quanta(), Lunit::QUANTA_PER_LOGICAL_PX);
        assert_eq!(Lunit::px(-1).quanta(), -Lunit::QUANTA_PER_LOGICAL_PX);
    }
    #[test]
    fn px_frac_construction() {
        assert_eq!(Lunit::px_frac(1, 2).quanta(), Lunit::QUANTA_PER_LOGICAL_PX / 2);
        assert_eq!(Lunit::px_frac(1, 4).quanta(), Lunit::QUANTA_PER_LOGICAL_PX / 4);
        assert_eq!(Lunit::px_frac(-1, 2).quanta(), -Lunit::QUANTA_PER_LOGICAL_PX / 2);
    }
    #[test]
    fn sub_floor_zero() {
        assert_eq!(Lunit::px(10).sub_floor_zero(Lunit::px(3)), Lunit::px(7));
        assert_eq!(Lunit::px(3).sub_floor_zero(Lunit::px(10)), Lunit::ZERO);
    }
    #[test]
    fn split_floor_and_remainder() {
        let total = Lunit::px(10);
        let part = total.split_floor(3);
        let rem = total.split_remainder(3);
        assert_eq!(part.quanta(), total.quanta() / 3);
        assert_eq!(rem, total.quanta() % 3);
    }
    #[test]
    fn split_invalid_parts() {
        assert_eq!(Lunit::px(10).split_floor(0), Lunit::ZERO);
        assert_eq!(Lunit::px(10).split_remainder(0), 0);
        assert_eq!(Lunit::px(10).split_floor(-3), Lunit::ZERO);
        assert_eq!(Lunit::px(10).split_remainder(-3), 0);
    }
}
