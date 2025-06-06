// devela::num::geom::metric:angle::impl::test_int

#[allow(unused, reason = "±_int_*")]
use crate::{Angle, AngleDirection, AngleKind, assert_approx_eq_all};

#[cfg(feature = "_int_i16")]
mod angle_i16 {
    use super::*;

    /// Test constructors for specific normalized angles.
    #[test]
    fn angle_construction() {
        let full = Angle::<i16>::new_full();
        let right = Angle::<i16>::new_right();
        let straight = Angle::<i16>::new_straight();
        assert_eq!(full.turn, 0);
        assert_eq!(right.turn, i16::MAX / 4);
        assert_eq!(straight.turn, i16::MAX / 2);

        let full = Angle::<u16>::new_full();
        let right = Angle::<u16>::new_right();
        let straight = Angle::<u16>::new_straight();
        assert_eq!(full.turn, 0);
        assert_eq!(right.turn, u16::MAX / 4);
        assert_eq!(straight.turn, u16::MAX / 2);
    }

    /// Integer representations should always be normalized.
    #[test]
    fn angle_normalization() {
        let angle: Angle<i16> = Angle::new(12345);
        assert!(angle.is_normalized());

        let mut angle = Angle::new(12345_i16);
        // Should have no effect on normalized representation.
        angle.set_normalized();
        assert_eq!(angle.turn, 12345);
    }

    /// Test kind determination for an i16 representation.
    #[test]
    fn angle_kind_i16() {
        assert_eq!(Angle::<i16>::new_full().kind(), AngleKind::Full);
        assert_eq!(Angle::<i16>::new_right().kind(), AngleKind::Right);
        assert_eq!(Angle::<i16>::new_straight().kind(), AngleKind::Straight);
        assert_eq!(Angle::new(i16::MAX / 8).kind(), AngleKind::Acute);
        assert_eq!(Angle::new(i16::MAX / 3).kind(), AngleKind::Obtuse);
        assert_eq!(Angle::new((3 * i16::MAX as i32 / 4) as i16).kind(), AngleKind::Reflex);
    }

    /// Test direction handling for signed integers.
    #[test]
    fn signed_angle_direction() {
        let angle = Angle::new(-i16::MAX / 4);
        assert_eq!(angle.direction(), AngleDirection::Negative);

        let positive_angle = angle.with_direction(AngleDirection::Positive);
        assert_eq!(positive_angle.direction(), AngleDirection::Positive);

        let inverted = positive_angle.invert_direction();
        assert_eq!(inverted.direction(), AngleDirection::Negative);
    }

    /// Test conversion of angle to degrees using integer scaling.
    #[test]
    fn angle_conversion_to_deg() {
        let right = Angle::<i16>::new_right();
        assert_approx_eq_all!(tolerance: 0.01, right.to_deg(), 90.0); // 89.99176

        let straight = Angle::<i16>::new_straight();
        assert_approx_eq_all!(tolerance: 0.01, straight.to_deg(), 180.0); // 179.9945
    }

    /// Test boundaries for minimum and maximum values.
    #[test]
    fn angle_boundary_conditions() {
        let zero_angle = Angle::<i16>::new(0_i16);
        let min_angle = Angle::<i16>::new(i16::MIN);
        let max_angle = Angle::<i16>::new(i16::MAX);

        // Even though `min_angle` represents a negative full turn,
        // it should still be considered normalized due to integer mapping.
        assert!(zero_angle.is_normalized());
        assert!(min_angle.is_normalized());
        assert!(max_angle.is_normalized());

        // Kind determination should reflect the magnitude, not the sign.
        assert_eq!(zero_angle.kind(), AngleKind::Full);
        assert_eq!(min_angle.kind(), AngleKind::Reflex);
        assert_eq!(max_angle.kind(), AngleKind::Reflex);
    }

    /// Verify that direction is correctly set for signed types.
    #[test]
    fn angle_set_direction() {
        let mut angle = Angle::<i16>::new(-i16::MAX / 3);
        assert_eq!(angle.direction(), AngleDirection::Negative);

        angle.set_direction(AngleDirection::Positive);
        assert_eq!(angle.direction(), AngleDirection::Positive);

        // Undefined should be treated as counterclockwise.
        angle.set_direction(AngleDirection::Undefined);
        assert_eq!(angle.direction(), AngleDirection::Positive);
    }
}

#[cfg(feature = "_int_u16")]
mod angle_u16 {
    use super::*;

    /// Test constructors for specific normalized angles.
    #[test]
    fn angle_construction() {
        let full = Angle::<u16>::new_full();
        let right = Angle::<u16>::new_right();
        let straight = Angle::<u16>::new_straight();
        assert_eq!(full.turn, 0);
        assert_eq!(right.turn, u16::MAX / 4);
        assert_eq!(straight.turn, u16::MAX / 2);
    }

    /// Test direction handling for unsigned angles.
    #[test]
    fn unsigned_angle_direction() {
        let unsigned_angle = Angle::<u16>::new(u16::MAX / 4);
        assert_eq!(unsigned_angle.direction(), AngleDirection::Positive);

        // Setting direction should have no effect.
        let same_angle = unsigned_angle.with_direction(AngleDirection::Negative);
        assert_eq!(same_angle.direction(), AngleDirection::Positive);
    }
}
