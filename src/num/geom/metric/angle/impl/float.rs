// devela::num::geom::metric::angle::impl::float
//
//!
//

#[cfg(_float··)]
#[allow(unused_imports)]
use crate::{Angle, AngleDirection, AngleKind, ExtFloat, ExtFloatConst, Float};

/// impl `Angle` methods with a floating-point representation.
///
/// # Macro arguments
/// $f: the inner floating-point type
/// $cap: the capability that enables the implementation. E.g "_float_f32".
/// $cmp: the capability associated to some methods. E.g. _cmp_f32.
macro_rules! impl_angle {
    () => {
        impl_angle![float
            f32:"_float_f32":"_cmp_f32",
            f64:"_float_f64":"_cmp_f64"
        ];
    };
    (float $($f:ty : $cap:literal : $cmp:literal),+) => {
        $( impl_angle![@float $f:$cap:$cmp]; )+
    };
    (@float $f:ty : $cap:literal : $cmp:literal) => {
        #[doc = concat!("# Methods for angles represented using `", stringify!($f), "`.")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap )]
        impl Angle<$f> {
            /* construct */

            /// Creates a normalized full positive angle at 0 degrees.
            pub const fn new_full() -> Self { Self::new(0.0) }

            /// Creates a normalized right positive angle at 90 degrees (0.25).
            pub const fn new_right() -> Self { Self::new(0.25) }

            /// Creates a normalized straight positive angle at 180 degrees (0.5).
            pub const fn new_straight() -> Self { Self::new(0.5) }

            /// Creates a new angle from a `radians` value.
            pub const fn from_rad(radians: $f) -> Self { Self::new(radians / <$f>::TAU) }

            /// Creates a new angle from a `degrees` value.
            pub const fn from_deg(degrees: $f) -> Self { Self::new(degrees / 360.0) }

            /// Creates a new angle from a `value` in a `custom_unit` which represents a full turn.
            pub const fn from_custom(value: $f, custom_unit: $f) -> Self {
                Self::new(value / custom_unit)
            }

            /* convert */

            /// Converts the angle to radians.
            #[must_use]
            pub const fn to_rad(self) -> $f { self.turn * <$f>::TAU }

            /// Converts the angle to degrees.
            #[must_use]
            pub const fn to_deg(self) -> $f { self.turn * 360.0 }

            /// Converts the angle to a `custom_unit` which represents a full turn.
            #[must_use]
            pub const fn to_custom(self, custom_unit: $f) -> $f { self.turn * custom_unit }

            /* normalize */

            /// Returns `true` if the angle is between -1 and 1 (non-inclusive).
            #[cfg(feature = $cmp)]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cmp)))]
            pub const fn is_normalized(self) -> bool {
                crate::Compare(self.turn).gt(-1.0) && crate::Compare(self.turn).lt(1.0)
            }

            /// Returns the angle normalized to the non-inclusive range -1 to 1.
            // BLOCKED: const by fract
            pub fn normalize(self) -> Self { Self::new(self.turn.fract()) }

            /// Sets the angle normalized to the non-inclusive range -1 to 1.
            // BLOCKED: const by fract
            pub fn set_normalized(&mut self) { self.turn = self.turn.fract(); }

            /* direction */

            /// Returns the angle direction.
            ///
            /// Since the floating-point representation always maintains the sign
            /// the direction can't be undefined.
            pub const fn direction(self) -> AngleDirection {
                use AngleDirection::{Negative, Positive};
                if Float(self.turn).is_sign_negative() { Negative } else { Positive }
            }

            /// Returns `true` if the angle has the given `direction`.
            ///
            /// Since the floating-point representation always maintains the sign
            /// the direction can't be undefined, and it will return `false` in that case.
            #[must_use]
            pub const fn has_direction(self, direction: AngleDirection) -> bool {
                direction as i8 == self.direction() as i8
            }

            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            pub const fn with_direction(self, direction: AngleDirection) -> Self {
                use AngleDirection as D;
                match direction {
                    D::Positive | D::Undefined => Self::new(Float(self.turn).abs().0),
                    D::Negative => Self::new(Float(self.turn).neg_abs().0),
                }
            }

            /// Sets the angle to the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            pub const fn set_direction(&mut self, direction: AngleDirection) {
                use AngleDirection as D;
                match direction {
                    D::Positive | D::Undefined => self.turn = Float(self.turn).abs().0,
                    D::Negative => self.turn = Float(self.turn).neg_abs().0,
                }
            }

            /// Returns a version of the angle with inverted direction.
            pub const fn invert_direction(self) -> Self {
                Self::new(Float(self.turn).flip_sign().0)
            }

            /// Returns the negative version of the angle.
            pub const fn negative(self) -> Self { Self::new(Float(self.turn).neg_abs().0) }

            /// Sets the angle as negative.
            pub const fn set_negative(&mut self) { { self.turn = Float(self.turn).neg_abs().0; } }

            /// Returns the positive version of the angle.
            pub const fn positive(self) -> Self { Self::new(Float(self.turn).abs().0) }

            /// Sets the angle as positive.
            pub const fn set_positive(&mut self) { self.turn = Float(self.turn).abs().0; }

            /* kind */

            /// Returns the kind of the normalized angle.
            // BLOCKED: const by normalize
            #[cfg(feature = $cmp)]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cmp)))]
            pub fn kind(self) -> AngleKind {
                let angle = crate::Compare(self.normalize().positive().turn);
                use AngleKind::{Full, Acute, Right, Obtuse, Straight, Reflex};
                if angle.eq(0.0) { // 1 turn (0' or 360º)
                    Full
                } else if angle.eq(0.25) { // 1/4 turn (90º)
                    Right
                } else if angle.eq(0.5) { // 1/2 turn (180º)
                    Straight
                } else if angle.lt(0.25) { // < 1/4 turn (< 90º)
                    Acute
                } else if angle.lt(0.5) { // < 1/2 turn (< 180º)
                    Obtuse
                } else { // < 1 turn (< 360º)
                    Reflex
                }
            }
            /// Returns the kind of the angle using a custom tolerance for approximate matching.
            // BLOCKED: const by normalize
            pub fn kind_approx(self, tolerance: $f) -> AngleKind {
                let angle = self.normalize().positive().turn;
                use AngleKind::{Full, Acute, Right, Obtuse, Straight, Reflex};
                if (angle - 0.0).abs() <= tolerance {
                    Full
                } else if (angle - 0.25).abs() <= tolerance {
                    Right
                } else if (angle - 0.5).abs() <= tolerance {
                    Straight
                } else if angle < 0.25 {
                    Acute
                } else if angle < 0.5 {
                    Obtuse
                } else {
                    Reflex
                }
            }

            /// Returns `true` if the angle is of the given `kind`.
            #[must_use]
            #[cfg(feature = $cmp)]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cmp)))]
            // BLOCKED: const by normalize
            pub fn is_kind(self, kind: AngleKind) -> bool {
                let angle = crate::Compare(self.normalize().positive().turn);
                use AngleKind::{Full, Acute, Right, Obtuse, Straight, Reflex};
                match kind {
                    Full => angle.eq(0.0),
                    Right => angle.eq(0.25),
                    Straight => angle.eq(0.5),
                    Acute => angle.gt(0.0) && angle.lt(0.25),
                    Obtuse => angle.gt(0.25) && angle.lt(0.5),
                    Reflex => angle.gt(0.5) && angle.lt(1.0),
                }
            }

            /// Returns `true` if the angle is of the given `kind` using a custom tolerance.
            #[must_use]
            // BLOCKED: const by normalize
            pub fn is_kind_approx(self, kind: AngleKind, tolerance: $f) -> bool {
                let angle = self.normalize().positive().turn;
                match kind {
                    AngleKind::Full => (angle - 0.0).abs() <= tolerance,
                    AngleKind::Right => (angle - 0.25).abs() <= tolerance,
                    AngleKind::Straight => (angle - 0.5).abs() <= tolerance,
                    AngleKind::Acute => angle > 0.0 && angle < 0.25,
                    AngleKind::Obtuse => angle > 0.25 && angle < 0.5,
                    AngleKind::Reflex => angle > 0.5 && angle < 1.0,
                }
            }
        }
    };
}
impl_angle!();
