// devela::geom::metric::angle::impl::int
//
//!
//
// IMPROVE IDEAS
// - make alternative methods that doesn't depnd on floating point operations,
//   and instead use integer scaling functions Int::scale.
// - maybe use NonExtreme for the signed representation.

// IMPROVE: remove FloatExt, replace by Float, to simplify from_float_normalized
#[cfg(not(feature = "std"))]
use crate::FloatExt;
use crate::{Angle, AngleDirection, AngleKind};
#[allow(unused_imports)]
use crate::{FloatConst, fsize};

/// impl `Angle` methods with an integer representation:
///
/// # TOC
/// - integer common methods
/// - signed integers specific methods
/// - unsigned integers specific methods
///
/// # Macro arguments
/// $t: the inner integer primitive type
/// $f: the associated floating point type
macro_rules! impl_angle {
    () => {
        impl_angle![sint i8:f32, i16:f32, i32:f32, i64:f64, i128:f64, isize:fsize];
        impl_angle![uint u8:f32, u16:f32, u32:f32, u64:f64, u128:f64, usize:fsize];
    };

    // integers common methods
    (int $($t:ty : $f:ty),+) => {
        $( impl_angle![@int $t:$f]; )+
    };
    (@int $t:ty : $f:ty) => {
        #[doc = concat!("# Methods for angles represented using `", stringify!($t), "`.")]
        impl Angle<$t> {
            /* private helpers */

            // Returns the inner value normalized as a float between -1 and 1
            const fn to_float_normalized(self) -> $f { self.turn as $f / <$t>::MAX as $f }
            // Returns the `value` associated to the full turn `unit`, scaled to the full $t range.
            fn from_float_normalized(value: $f, unit: $f) -> $t {
                ((value / unit) * <$t>::MAX as $f).round() as $t
            }

            /* construct */

            /// Creates a normalized full positive angle at 0 degrees.
            pub const fn new_full() -> Self { Self::new(0) }

            /// Creates a normalized right positive angle at 90 degrees.
            pub const fn new_right() -> Self { Self::new(<$t>::MAX / 4) }

            /// Creates a normalized straight positive angle at 180 degrees.
            pub const fn new_straight() -> Self { Self::new(<$t>::MAX / 2) }

            /// Creates a new angle from a floating-point `radians` value.
            pub fn from_rad(radians: $f) -> Self {
                Self::new(Self::from_float_normalized(radians, <$f>::TAU))
            }

            /// Creates a new angle from a floating-point `degrees` value.
            pub fn from_deg(degrees: $f) -> Self {
                Self::new(Self::from_float_normalized(degrees, 360.0))
            }

            /// Creates a new angle from a `value` in a `custom_unit` which represents a full turn.
            pub fn from_custom(value: $f, custom_unit: $f) -> Self {
                Self::new(Self::from_float_normalized(value, custom_unit))
            }

            /* convert */

            /// Converts the angle to radians.
            #[must_use]
            pub const fn to_rad(self) -> $f { self.to_float_normalized() * <$f>::TAU }

            /// Converts the angle to degrees.
            #[must_use]
            pub const fn to_deg(self) -> $f { self.to_float_normalized() * 360.0 }

            /// Converts the angle to a `custom_unit` which represents a full turn.
            #[must_use]
            pub const fn to_custom(self, custom_unit: $f) -> $f {
                self.to_float_normalized() * custom_unit
            }

            /* normalize */

            /// Always returns `true` since integer representations are always normalized.
            #[must_use]
            pub const fn is_normalized(self) -> bool { true }

            /// Returns the angle normalized (no-op for integer representation).
            pub const fn normalize(self) -> Self { self }

            /// Sets the angle normalized (no-op for integer representation).
            pub fn set_normalized(&mut self) {}
            // TODO: to_norm*

            /// Returns `true` if the angle has the given `direction`.
            #[must_use ]
            pub const fn has_direction(self, direction: AngleDirection) -> bool {
                direction as i8 == self.direction() as i8
            }

            /* kind */

            /// Returns the kind of the normalized angle.
            pub const fn kind(self) -> AngleKind {
                let angle = self.positive().turn;
                let right = <$t>::MAX / 4;
                let straight = <$t>::MAX / 2;
                use AngleKind as K;
                if angle == 0 { // 1 turn (0' or 360º)
                    K::Full
                } else if angle == right { // 1/4 turn (90º)
                    K::Right
                } else if angle == straight { // 1/2 turn (180º)
                    K::Straight
                //
                } else if angle < right { // < 1/4 turn (< 90º)
                    K::Acute
                } else if angle < straight { // < 1/2 turn (< 180º)
                    K::Obtuse
                } else { // < 1 turn (< 360º)
                    K::Reflex
                }
            }

            /// Returns `true` if the angle is of the given `kind`.
            #[must_use]
            pub const fn is_kind(self, kind: AngleKind) -> bool {
                let angle = self.positive().turn;
                let right = <$t>::MAX / 4;
                let straight = <$t>::MAX / 2;

                use AngleKind as K;
                match kind {
                    K::Full => angle == 0,
                    K::Right => angle == right,
                    K::Straight => angle == straight,
                    //
                    K::Acute => angle > 0 && angle < right,
                    K::Obtuse => angle < right && angle < straight,
                    K::Reflex => angle > right,
                }
            }
        }
    };

    // signed integers specific methods
    (sint $($t:ty : $f:ty),+) => {
        $( impl_angle![@sint $t:$f]; )+
    };
    (@sint $t:ty : $f:ty) => {
        impl_angle![int $t:$f];

        #[doc = concat!("# Methods for angles represented using `", stringify!($t), "`, signed.")]
        impl Angle<$t> {
            /* direction */

            /// Returns the angle direction.
            ///
            /// The direction will be `Undefined` if the angle kind is [`Full`][AngleKind::Full].
            pub const fn direction(self) -> AngleDirection {
                use AngleDirection as D;
                if self.turn == 0 {
                    D::Undefined
                } else if self.turn > 0 {
                    D::Positive
                } else {
                    D::Negative
                }
            }

            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            pub const fn with_direction(self, direction: AngleDirection) -> Self {
                use AngleDirection as D;
                match direction {
                    D::Positive | D::Undefined => Self::new(self.turn.saturating_abs()),
                    D::Negative => Self::new(-self.turn.saturating_abs()),
                }
            }

            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            pub fn set_direction(&mut self, direction: AngleDirection) {
                use AngleDirection as D;
                match direction {
                    D::Positive | D::Undefined => self.turn = self.turn.saturating_abs(),
                    D::Negative => self.turn = -self.turn.saturating_abs(),
                }
            }

            /// Returns a version of the angle with inverted direction.
            pub const fn invert_direction(self) -> Self {
                Self::new(self.turn.saturating_neg())
            }

            /// Returns the negative version of the angle.
            pub const fn negative(self) -> Self { Self::new(-self.turn.saturating_abs()) }

            /// Sets the angle as negative.
            pub fn set_negative(&mut self) { self.turn = -self.turn.saturating_abs(); }

            /// Returns the positive version of the angle.
            pub const fn positive(self) -> Self { Self::new(self.turn.saturating_abs()) }

            /// Sets the angle as positive.
            pub fn set_positive(&mut self) { self.turn = self.turn.saturating_abs(); }
        }
    };

    // unsigned integers specific methods
    (uint $($t:ty : $f:ty),+) => {
        $( impl_angle![@uint $t:$f]; )+
    };
    (@uint $t:ty : $f:ty) => {
        impl_angle![int $t:$f];

        #[doc = concat!("# Methods for angles represented using `", stringify!($t), "`, unsigned.")]
        impl Angle<$t> {
            /* direction */

            /// Returns the angle direction.
            ///
            /// For unsigned integers the direction is always `Positive`.
            pub const fn direction(self) -> AngleDirection { AngleDirection::Positive }

            /// Returns a version of the angle with the given `direction` (no-op for unsigned).
            ///
            /// Unsigned integers can only have `Positive` direction.
            pub const fn with_direction(self, _direction: AngleDirection) -> Self { self }

            /// Returns a version of the angle with the given `direction` (no-op for unsigned).
            ///
            /// Unsigned integers can only have `Positive` direction.
            pub const fn set_direction(self, _direction: AngleDirection) {}

            /// Returns a version of the angle with inverted direction (no-op for unsigned).
            ///
            /// Unsigned integers can only have `Positive` direction.
            pub const fn invert_direction(self) -> Self { self }

            /// Returns the negative version of the angle (no-op for unsigned).
            pub const fn negative(self) -> Self { self }

            /// Sets the angle as negative (no-op for unsigned).
            pub fn set_negative(&mut self) {}

            /// Returns the positive version of the angle (no-op for unsigned).
            pub const fn positive(self) -> Self { self }

            /// Sets the angle as positive (no-op for unsigned).
            pub fn set_positive(&mut self) {}
        }
    };
}
impl_angle!();
