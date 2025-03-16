// devela::num::geom::metric::angle::kind
//
//! Defines [`AngleKind`].
//

use crate::{FloatConst, Interval};

#[doc = crate::TAG_GEOM!()]
/// The kind of [`Angle`], based on its normalized turn.
///
/// The variant values are normalized to the full range of an u8.
#[must_use]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum AngleKind {
    /// = 0º = 360º (1τ = 2π), a full turn, or no turn.
    #[default]
    Full,

    /// > 0° and < 90°
    Acute,

    /// = 90° (¼τ), a quarter turn.
    Right,

    /// > 90° and < 180°.
    Obtuse,

    /// = 180° (½τ), a half turn.
    Straight,

    /// > 180° and < 360°.
    Reflex,
}

impl AngleKind {
    /// Returns an interval representing the angle range in degrees.
    pub fn interval_deg(self) -> Interval<u16> {
        use AngleKind as K;
        match self {
            K::Full => Interval::closed(0, 0),         // Full turn as [0, 0]
            K::Acute => Interval::open(0, 90),         // Acute: (0, 90)
            K::Right => Interval::closed(90, 90),      // Right: [90, 90]
            K::Obtuse => Interval::open(90, 180),      // Obtuse: (90, 180)
            K::Straight => Interval::closed(180, 180), // Straight: [180, 180]
            K::Reflex => Interval::open(180, 360),     // Reflex: (180, 360)
        }
    }

    /// Returns an interval representing the angle range in gradians.
    pub fn interval_gra(self) -> Interval<u16> {
        use AngleKind as K;
        match self {
            K::Full => Interval::closed(0, 0),
            K::Acute => Interval::open(0, 100),
            K::Right => Interval::closed(100, 100),
            K::Obtuse => Interval::open(100, 200),
            K::Straight => Interval::closed(200, 200),
            K::Reflex => Interval::open(200, 400),
        }
    }

    /// Returns an interval representing the angle range in radians.
    pub fn interval_rad(self) -> Interval<f32> {
        const PI: f32 = f32::PI;
        use AngleKind as K;
        match self {
            K::Full => Interval::closed(0.0, 0.0),
            K::Acute => Interval::open(0.0, PI / 2.0),
            K::Right => Interval::closed(PI / 2.0, PI / 2.0),
            K::Obtuse => Interval::open(PI / 2.0, PI),
            K::Straight => Interval::closed(PI, PI),
            K::Reflex => Interval::open(PI, 2.0 * PI),
        }
    }

    /// Returns an interval representing the angle range using 256 as a full turn.
    pub fn interval_u8(self) -> Interval<u8> {
        use AngleKind as K;
        match self {
            K::Full => Interval::closed(0, 0),
            K::Acute => Interval::open(0, 64),
            K::Right => Interval::closed(64, 64),
            K::Obtuse => Interval::open(64, 128),
            K::Straight => Interval::closed(128, 128),
            K::Reflex => Interval::open(128, 255),
        }
    }
}
