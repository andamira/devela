// devela::num::geom::prim::angle::impl::methods_iu
//
//!
//

use super::super::{Angle, AngleDirection, AngleKind};
#[cfg(all(not(feature = "std"), _some_float))]
use crate::num::ExtFloat;
use crate::{
    code::compile,
    num::{fsize, ExtFloatConst},
};

// impl Angle methods with an integer representation
//
// TOC
// - integer common methods
// - signed integers specific methods
// - unsigned integers specific methods
//
// $t: the inner integer primitive type
// $f: the associated floating point type
// $tcap: the capability feature that enables the given integer implementation. E.g "_int_i8".
// $fcap: the capability feature that enables the given floating implementation. E.g "_float_f32".
macro_rules! impl_angle {
    () => {
        impl_angle![sint
            i8:f32;"_int_i8":"_float_f32",
            i16:f32;"_int_i16":"_float_f32",
            i32:f32;"_int_i32":"_float_f32",
            i64:f64;"_int_i64":"_float_f64",
            i128:f64;"_int_i128":"_float_f64"
        ];
        #[cfg(target_pointer_width = "32")]
        impl_angle![sint isize:fsize;"_int_isize":"_float_f32"];
        #[cfg(target_pointer_width = "64")]
        impl_angle![sint isize:fsize;"_int_isize":"_float_f64"];

        impl_angle![uint
            u8:f32;"_int_u8":"_float_f32",
            u16:f32;"_int_u16":"_float_f32",
            u32:f32;"_int_u32":"_float_f32",
            u64:f64;"_int_u64":"_float_f64",
            u128:f64;"_int_u128":"_float_f64"
        ];
        #[cfg(target_pointer_width = "32")]
        impl_angle![uint usize:fsize;"_int_usize":"_float_f32"];
        #[cfg(target_pointer_width = "64")]
        impl_angle![uint usize:fsize;"_int_usize":"_float_f64"];
    };

    // integers common methods
    (int $($t:ty : $f:ty ; $tcap:literal : $fcap:literal),+) => {
        $( impl_angle![@int $t:$f ; $tcap:$fcap]; )+
    };
    (@int $t:ty : $f:ty ; $tcap:literal : $fcap:literal) => {
        #[doc = concat!("# Methods for angles represented using `", stringify!($t), "`.")]
        #[cfg(feature = $tcap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $tcap)))]
        impl Angle<$t> {
            /* private helpers */

            // Returns the inner value normalized as a float between -1 and 1
            #[inline]
            fn to_float_normalized(self) -> $f { self.0 as $f / <$t>::MAX as $f }
            // Returns the `value` associated to the full turn `unit`, scaled to the full $t range.
            #[inline]
            #[cfg(any(feature = "std", feature = $fcap))]
            fn from_float_normalized(value: $f, unit: $f) -> $t {
                ((value / unit) * <$t>::MAX as $f).round() as $t
            }

            /* construct */

            /// Creates a normalized full positive angle at 0 degrees.
            #[inline]
            pub const fn new_full() -> Self { Self(0) }

            /// Creates a normalized right positive angle at 90 degrees.
            #[inline]
            pub const fn new_right() -> Self { Self(<$t>::MAX / 4) }

            /// Creates a normalized straight positive angle at 180 degrees.
            #[inline]
            pub const fn new_straight() -> Self { Self(<$t>::MAX / 2) }

            /// Creates a new angle from a floating-point `radians` value.
            #[inline]
            #[cfg(any(feature = "std", feature = $fcap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $fcap))))]
            pub fn from_rad(radians: $f) -> Self {
                Self(Self::from_float_normalized(radians, <$f>::TAU))
            }

            /// Creates a new angle from a floating-point `degrees` value.
            #[inline]
            #[cfg(any(feature = "std", feature = $fcap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $fcap))))]
            pub fn from_deg(degrees: $f) -> Self {
                Self(Self::from_float_normalized(degrees, 360.0))
            }

            /// Creates a new angle from a `value` in a `custom_unit` which represents a full turn.
            #[inline]
            #[cfg(any(feature = "std", feature = $fcap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $fcap))))]
            pub fn from_custom(value: $f, custom_unit: $f) -> Self {
                Self(Self::from_float_normalized(value, custom_unit))
            }

            /* convert */

            /// Converts the angle to radians.
            #[inline] #[must_use]
            #[cfg(any(feature = "std", _some_float))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", _some_float))))]
            pub fn to_rad(self) -> $f { self.to_float_normalized() * <$f>::TAU }
            /// Converts the angle to degrees.
            #[inline] #[must_use]
            pub fn to_deg(self) -> $f { self.to_float_normalized() * 360.0 }
            /// Converts the angle to a `custom_unit` which represents a full turn.
            #[inline] #[must_use]
            pub fn to_custom(self, custom_unit: $f) -> $f { self.to_float_normalized() * custom_unit }

            /* normalize */

            /// Always returns `true` since integer representations are always normalized.
            #[inline] #[must_use]
            pub const fn is_normalized(self) -> bool { true }

            /// Returns the angle normalized (no-op for integer representation).
            #[inline]
            pub const fn normalize(self) -> Self { self }

            /// Sets the angle normalized (no-op for integer representation).
            #[inline]
            pub fn set_normalized(&mut self) {}

            /// Returns `true` if the angle has the given `direction`.
            #[inline] #[must_use ]
            pub const fn has_direction(self, direction: AngleDirection) -> bool {
                direction as i8 == self.direction() as i8
            }

            /* kind */

            /// Returns the kind of the normalized angle.
            pub const fn kind(self) -> AngleKind {
                let this = self.positive().0;
                let right = <$t>::MAX / 4;
                let straight = <$t>::MAX / 2;

                use AngleKind::{Full, Acute, Right, Obtuse, Straight, Reflex};
                if this == 0 { // 1 turn (0' or 360º)
                    Full
                } else if this == right { // 1/4 turn (90º)
                    Right
                } else if this == straight { // 1/2 turn (180º)
                    Straight
                } else if this < right { // < 1/4 turn (< 90º)
                    Acute
                } else if this < straight { // < 1/2 turn (< 180º)
                    Obtuse
                } else { // < 1 turn (< 360º)
                    Reflex
                }
            }

            /// Returns `true` if the angle is of the given `kind`.
            #[inline] #[must_use]
            pub const fn is_kind(self, kind: AngleKind) -> bool {
                let angle = self.positive().0;
                let right = <$t>::MAX / 4;
                let straight = <$t>::MAX / 2;

                use AngleKind::{Full, Acute, Right, Obtuse, Straight, Reflex};
                match kind {
                    Full => angle == 0,
                    Right => angle == right,
                    Straight => angle == straight,
                    Acute => angle > 0 && angle < right,
                    Obtuse => angle < right && angle < straight,
                    Reflex => angle > right,
                }
            }
        }
    };

    // signed integers specific methods
    (sint $($t:ty : $f:ty ; $tcap:literal : $fcap:literal),+) => {
        $( impl_angle![@sint $t:$f ; $tcap:$fcap]; )+
    };
    (@sint $t:ty : $f:ty ; $tcap:literal : $fcap:literal) => {
        impl_angle![int $t:$f ; $tcap:$fcap];

        #[doc = concat!("# Methods for angles represented using `", stringify!($t), "`, signed.")]
        #[cfg(feature = $tcap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $tcap)))]
        impl Angle<$t> {
            /* direction */

            /// Returns the angle direction.
            ///
            /// The direction will be `Undefined` if the angle kind is [`Full`][AngleKind::Full].
            #[inline]
            pub const fn direction(self) -> AngleDirection {
                if self.0 == 0 {
                    AngleDirection::Undefined
                } else if self.0 > 0 {
                    AngleDirection::CounterClockwise
                } else {
                    AngleDirection::Clockwise
                }
            }

            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            #[inline]
            pub const fn with_direction(self, direction: AngleDirection) -> Self {
                use AngleDirection as D;
                match direction {
                    D::CounterClockwise | D::Undefined => Self(self.0.saturating_abs()),
                    D::Clockwise => Self(-self.0.saturating_abs()),
                }
            }

            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            #[inline]
            pub fn set_direction(&mut self, direction: AngleDirection) {
                use AngleDirection as D;
                match direction {
                    D::CounterClockwise | D::Undefined => self.0 = self.0.saturating_abs(),
                    D::Clockwise => self.0 = -self.0.saturating_abs(),
                }
            }

            /// Returns a version of the angle with inverted direction.
            #[inline]
            pub const fn invert_direction(self) -> Self {
                Self(self.0.saturating_neg())
            }

            /// Returns the negative version of the angle.
            #[inline]
            pub const fn negative(self) -> Self { Self(-self.0.saturating_abs()) }

            /// Sets the angle as negative.
            #[inline]
            pub fn set_negative(&mut self) { self.0 = -self.0.saturating_abs(); }

            /// Returns the positive version of the angle.
            #[inline]
            pub const fn positive(self) -> Self { Self(self.0.saturating_abs()) }

            /// Sets the angle as positive.
            #[inline]
            pub fn set_positive(&mut self) { self.0 = self.0.saturating_abs(); }
        }
    };

    // unsigned integers specific methods
    (uint $($t:ty : $f:ty ; $tcap:literal : $fcap:literal),+) => {
        $( impl_angle![@uint $t:$f ; $tcap:$fcap]; )+
    };
    (@uint $t:ty : $f:ty ; $tcap:literal : $fcap:literal) => {
        impl_angle![int $t:$f ; $tcap:$fcap];

        #[doc = concat!("# Methods for angles represented using `", stringify!($t), "`, unsigned.")]
        #[cfg(feature = $tcap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $tcap)))]
        impl Angle<$t> {
            /* direction */

            /// Returns the angle direction.
            ///
            /// For unsigned integers the direction is always `CounterClockwise`.
            #[inline]
            pub const fn direction(self) -> AngleDirection { AngleDirection::CounterClockwise }

            /// Returns a version of the angle with the given `direction` (no-op for unsigned).
            ///
            /// Unsigned integers can only have `CounterClockwise` direction.
            #[inline]
            pub const fn with_direction(self, _direction: AngleDirection) -> Self { self }

            /// Returns a version of the angle with the given `direction` (no-op for unsigned).
            ///
            /// Unsigned integers can only have `CounterClockwise` direction.
            #[inline]
            pub fn set_direction(self, _direction: AngleDirection) {}

            /// Returns a version of the angle with inverted direction (no-op for unsigned).
            ///
            /// Unsigned integers can only have `CounterClockwise` direction.
            #[inline]
            pub const fn invert_direction(self) -> Self { self }

            /// Returns the negative version of the angle (no-op for unsigned).
            #[inline]
            pub const fn negative(self) -> Self { self }

            /// Sets the angle as negative (no-op for unsigned).
            #[inline]
            pub fn set_negative(&mut self) {}

            /// Returns the positive version of the angle (no-op for unsigned).
            #[inline]
            pub const fn positive(self) -> Self { self }

            /// Sets the angle as positive (no-op for unsigned).
            #[inline]
            pub fn set_positive(&mut self) {}

        }
    };
}
impl_angle!();
