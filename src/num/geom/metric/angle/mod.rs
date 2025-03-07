// devela::num::geom::metric::angle
//
//! Defines [`Angle`], [`AngleDirection`], [`AngleKind`].
//!
//! [`Angle`]s and [`Cycle`][crate::Cycle]s are closely related:
//! - An angle represents a fraction of a full rotation.
//! - A cycle represents a repeating pattern over a period.
//! - A full-turn normalized angle (0.0 to 1.0 or 0..256) is directly analogous to phase in a cycle.
//
// TOC
// - struct Angle
// - struct AngleDirection

mod r#impl;

mod kind;
pub use kind::AngleKind;

#[doc = crate::TAG_GEOM!()]
/// An angle represents a fraction of a full rotation.
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
///
/// This type does **not enforce normalization**, but it is expected
/// to be normalized in most use cases.
#[must_use]
#[repr(transparent)]
pub struct Angle<T> {
    /// A unit-agnostic rotation.
    pub turn: T,
}

impl<T> Angle<T> {
    /// Creates a new angle.
    pub const fn new(turn: T) -> Self {
        Self { turn }
    }
}

#[doc = crate::TAG_GEOM!()]
/// The direction of rotation of an angle.
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
    /// By convention, **positive** angles represent a **counterclockwise** rotation.
    ///
    /// Also known as the Right-Handed Rule.
    ///
    /// This is the default direction of rotation.
    #[default]
    Positive = 1,

    /// By convention, **negative** angles represent a **clockwise** rotation.
    ///
    /// Also known as the Left-Hand Rule.
    Negative = -1,

    /// An undefined direction can occur when a full-turn angle is normalized
    /// to an unsigned `0`, such as when working with primitive signed integers.
    Undefined = 0,
}

#[allow(missing_docs, non_upper_case_globals)]
impl AngleDirection {
    /// Alias of **positive** angle direction.
    pub const CounterClockwise: AngleDirection = AngleDirection::Positive;
    pub const CCW: AngleDirection = AngleDirection::Positive;
    pub const RightHandRule: AngleDirection = AngleDirection::Positive;
    pub const RHR: AngleDirection = AngleDirection::Positive;

    /// Alias of **negative** angle direction.
    pub const Clockwise: AngleDirection = AngleDirection::Negative;
    pub const CW: AngleDirection = AngleDirection::Negative;
    pub const LeftHandRule: AngleDirection = AngleDirection::Positive;
    pub const LHR: AngleDirection = AngleDirection::Positive;
}
