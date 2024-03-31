// devela::num::geom::prim::angle::impl::methods_f
//
//!
//

use super::super::{Angle, AngleDirection, AngleKind};
use crate::num::{fsize, Compare, ExtFloatConst};
#[cfg(feature = "num_float")]
use crate::num::{ExtFloat, Float};

// impl Angle methods with a floating-point representation
macro_rules! impl_angle {
    // $f: the inner floating-point type
    (float $($f:ty : $cap:literal),+) => { $( impl_angle![@float $f : $cap]; )+ };
    (@float $f:ty : $cap:literal) => {
        #[doc = concat!("# Methods for angles represented using `", stringify!($f), "`.")]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Angle<$f> {
            /* construct */

            /// Creates a normalized full positive angle at 0 degrees.
            #[inline]
            pub const fn new_full() -> Self { Self(0.0) }

            /// Creates a normalized right positive angle at 90 degrees.
            #[inline]
            pub const fn new_right() -> Self { Self(0.25) }

            /// Creates a normalized straight positive angle at 180 degrees.
            #[inline]
            pub const fn new_straight() -> Self { Self(0.5) }

            /// Creates a new angle from a `radians` value.
            #[inline]
            pub fn from_rad(radians: $f) -> Self {
                Self(radians / <$f>::TAU)
            }

            /// Creates a new angle from a `degrees` value.
            #[inline]
            pub fn from_deg(degrees: $f) -> Self {
                Self(degrees / 360.0)
            }

            /// Creates a new angle from a `value` in a `custom_unit` which represents a full turn.
            #[inline]
            pub fn from_custom(value: $f, custom_unit: $f) -> Self {
                Self(value / custom_unit)
            }

            /* convert */

            /// Converts the angle to radians.
            #[inline] #[must_use]
            pub fn to_rad(self) -> $f {
                self.0 * <$f>::TAU
            }

            /// Converts the angle to degrees.
            #[inline] #[must_use]
            pub fn to_deg(self) -> $f {
                self.0 * 360.0
            }

            /// Converts the angle to a `custom_unit` which represents a full turn.
            #[inline] #[must_use]
            pub fn to_custom(self, custom_unit: $f) -> $f {
                self.0 * custom_unit
            }

            /* normalize */

            /// Returns `true` if the angle is between -1 and 1 (non-inclusive).
            #[inline]
            pub fn is_normalized(self) -> bool {
                Compare(self.0).gt(-1.0) && Compare(self.0).lt(1.0)
            }

            /// Returns the angle normalized to the non-inclusive range -1 to 1.
            // BLOCKED: const by fract
            #[inline]
            pub fn normalize(self) -> Self {
                Self(self.0.fract())
            }

            /// Sets the angle normalized to the non-inclusive range -1 to 1.
            #[inline]
            pub fn set_normalized(&mut self) {
                self.0 = self.0.fract();
            }

            /* direction */

            /// Returns the angle direction.
            ///
            /// Since the floating-point representation always maintains the sign
            /// the direction can't be undefined.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn direction(self) -> AngleDirection {
                use AngleDirection::{Clockwise, CounterClockwise};
                if Float(self.0).is_sign_negative() { Clockwise } else { CounterClockwise }
            }
            /// Returns the angle direction.
            ///
            /// Since the floating-point representation always maintains the sign
            /// the direction can't be undefined.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn direction(self) -> AngleDirection {
                use AngleDirection::{Clockwise, CounterClockwise};
                if self.0.is_sign_negative() { Clockwise } else { CounterClockwise }
            }

            /// Returns `true` if the angle has the given `direction`.
            ///
            /// Since the floating-point representation always maintains the sign
            /// the direction can't be undefined, and it will return `false` in that case.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[must_use] #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn has_direction(self, direction: AngleDirection) -> bool {
                direction as i8 == self.direction() as i8
            }
            /// Returns `true` if the angle has the given `direction`.
            ///
            /// Since the floating-point representation always maintains the sign
            /// the direction can't be undefined, and it will return `false` in that case.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn has_direction(self, direction: AngleDirection) -> bool {
                direction == self.direction()
            }

            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn with_direction(self, direction: AngleDirection) -> Self {
                use AngleDirection as D;
                match direction {
                    D::CounterClockwise | D::Undefined => Self(Float(self.0).const_abs().0),
                    D::Clockwise => Self(Float(self.0).neg_abs().0),
                }
            }
            /// Returns a version of the angle with the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn with_direction(self, direction: AngleDirection) -> Self {
                use AngleDirection as D;
                match direction {
                    D::CounterClockwise | D::Undefined => Self(self.0.abs()),
                    D::Clockwise => Self(-self.0.abs()),
                }
            }

            /// Sets the angle to the given `direction`.
            ///
            /// An `Undefined` direction will be interpreted as counter-clockwise (positive).
            #[inline]
            pub fn set_direction(&mut self, direction: AngleDirection) {
                use AngleDirection as D;
                match direction {
                    D::CounterClockwise | D::Undefined => self.0 = self.0.abs(),
                    D::Clockwise => self.0 = -self.0.abs(),
                }
            }

            /// Returns a version of the angle with inverted direction.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_float")))]
            #[cfg(feature = "num_float")]
            pub const fn invert_direction(self) -> Self {
                Self(Float(self.0).flip_sign().0)
            }
            /// Returns a version of the angle with inverted direction.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_float")))]
            #[cfg(feature = "num_float")]
            pub fn invert_direction(self) -> Self {
                Self(Float(self.0).flip_sign().0)
            }

            /// Returns the negative version of the angle.
            /// # Features
            /// This function will only be const with the `unsafe_const` and `num_float`
            /// features enabled.
            #[inline] #[cfg(all(
                not(feature = "safe_num"), feature = "unsafe_const", feature = "num_float"
            ))]
            pub const fn negative(self) -> Self { Self(Float(self.0).neg_abs().0) }
            /// Returns the negative version of the angle.
            /// # Features
            /// This function will only be const with the `unsafe_const` and `num_float`
            /// features enabled.
            #[inline] #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn negative(self) -> Self {
                #[cfg(feature = "num_float")]
                return Self(Float(self.0).neg_abs().0);
                #[cfg(not(feature = "num_float"))]
                return Self(-self.0.abs());
            }

            /// Sets the angle as negative.
            #[inline]
            pub fn set_negative(&mut self) {
                #[cfg(feature = "num_float")]
                { self.0 = Float(self.0).neg_abs().0; }
                #[cfg(not(feature = "num_float"))]
                { self.0 = -self.0.abs(); }
            }

            /// Returns the positive version of the angle.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn positive(self) -> Self { Self(Float(self.0).const_abs().0) }
            /// Returns the positive version of the angle.
            /// # Features
            /// This function will only be const with the `unsafe_const` feature enabled.
            #[inline] #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn positive(self) -> Self { Self(self.0.abs()) }

            /// Sets the angle as positive.
            #[inline]
            pub fn set_positive(&mut self) {
                self.0 = self.0.abs();
            }

            /* kind */

            /// Returns the kind of the normalized angle.
            // BLOCKED: const by normalize
            #[inline]
            pub fn kind(self) -> AngleKind {
                let angle = Compare(self.normalize().positive().0);
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

            /// Returns `true` if the angle is of the given `kind`.
            #[inline] #[must_use]
            // BLOCKED: const by normalize
            pub fn is_kind(self, kind: AngleKind) -> bool {
                let angle = Compare(self.normalize().positive().0);
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
        }
    };
}
impl_angle![float f32:"f32", f64:"f64"];
