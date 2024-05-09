// devela::num::geom::prim::angle
//
//! Definitions related to angles.
//

#![allow(unused, dead_code)]

mod r#impl;

/// A generic angle.
///
/// It's unit-agnostic over radians or degrees, and respects directionality.
///
/// When `T` is an integer primitive the angle is considered to be always
/// normalized. By virtue of the range from `[0..T::MAX]` representing a full
/// positive (counterclockwise) turn. And in the case of signed primitives,
/// the range from `[0..T::MIN]` represents a full negative (clockwise) turn.
///
/// For floating-point primitives the range from -1 to 1 (non-inclusive) represents
/// the normalized full turn, and anything beyond that are non-normalied angles.
///
/// # Features
/// It will be implemented for primitive floating point types when either the
/// `std` or `_float_f[32|64]` features are enabled.
/// And it will implemented for primitive integer types when its corresponding
/// feature capability is enabled (e.g. `_int_i32`).
///
/// # Methods
///
/// Methods are implemented the same for each main angle representation, using:
///
/// - Floating-point: E.g.: [methods for `f32`](#methods-for-angles-represented-using-f32).
/// - Signed integers. E.g: [methods for `i32`](#methods-for-angles-represented-using-i32).
/// - Unsigned integers. E.g.: [methods for `u32`](#methods-for-angles-represented-using-u32).
///
/// ## Methods for `f32`
/// - Construction:
///   - `new_`
///     [`full`](Self::new_full),
///     [`right`][Self::new_right],
///     [`straight`][Self::new_straight].
///   - `from_`
///     [`rad`](Self::from_rad),
///     [`deg`][Self::from_deg],
///     [`custom`][Self::from_custom].
/// - Conversion:
///   - `to_`
///     [`rad`](Self::to_rad),
///     [`deg`][Self::to_deg],
///     [`custom`][Self::to_custom].
/// - Normalization:
///   - [`normalize`](Self::normalize), *(
///     [`is_`][Self::is_normalized],
///     [`set_`][Self::set_normalized]. )*
/// - Direction:
///   - [`direction`](Self::direction), *(
///     [`with_`](Self::with_direction),
///     [`has_`](Self::has_direction),
///     [`set_`](Self::set_direction),
///     [`invert_`](Self::invert_direction). )*
///   - [`negative`](Self::negative), *(
///     [`set_`](Self::set_negative). )*
///   - [`positive`](Self::positive), *(
///     [`set_`](Self::set_positive). )*
/// - Kind:
///   - [`kind`](Self::kind), *(
///     [`is_`](Self::is_kind). )*
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
