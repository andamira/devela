// devela::fig::prim::angle
//
//! Definitions related to angles.
//

#![allow(unused, dead_code)]

mod r#impl;

/// A basic angle.
///
/// It's unit-agnostic over radians or degrees, and respects directionality.
///
/// If `T` is an integer primitive the range from 0 to T::MAX represents a full
/// turn, and for signed primitives the range from 0 to T::MIN represents a full
/// negative (clockwise) turn.
///
/// For floating-point primitives the range from -1..1 (non-inclusive) represents
/// the normalized full turn, anything beyond being angles larger than a full turn.
#[must_use]
#[repr(transparent)]
pub struct Angle<T>(pub T);

/// The angle kind, based on its normalized magnitude.
#[must_use]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum AngleKind {
    /// = 0 degrees equivalent to 360 degrees (1τ or 2π radians), full turn or no turn.
    #[default]
    Full = 0,
    /// < 90 degrees and > 0 degrees.
    Acute = 45,
    /// = 90 degrees (¼τ radians), quarter turn.
    Right = 90,
    /// > 90 degrees and < 180 degrees.
    Obtuse = 135,
    /// = 180 degrees (½τ radians), half turn.
    Straight = 180,
    /// > 180 degrees and < 360 degrees.
    Reflex = u8::MAX,
}

/// The angle direction.
///
/// In mathematics and most graphics programming contexts, the default direction
/// for angle measurements is counterclockwise from a defined zero point (usually
/// the positive x-axis). This convention applies to both 2D and 3D coordinate
/// systems where:
/// - A positive angle represents a counterclockwise rotation.
/// - A negative angle represents a clockwise rotation.
///
/// It aligns with the right-hand rule used in mathematics, physics, and engineering.
#[must_use]
#[repr(i8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AngleDirection {
    /// By convention, positive angles represent a counterclockwise rotation.
    ///
    /// This is the default direction.
    #[default]
    CounterClockwise = 1,

    /// By convention, negative angles represent a counterclockwise rotation.
    Clockwise = -1,

    /// An undefined direction can happen when a full turn angle is normalized
    /// to an unsigned 0, like when using primitive signed integers.
    Undefined = 0,
}
