// devela/src/geom/space/topol/point_segment.rs

use crate::{_impl_init, Turn};

#[doc = crate::_tags!(geom topol)]
/// The relation of a point to a directed planar segment.
#[doc = crate::_doc_meta!{location("geom/space")}]
///
/// The reference segment is directed from its **origin** to its
/// **destination**.
///
/// ```text
///                         Left
///                           •
///
/// Behind    Origin       Between       Destination    Beyond
///   •─────────•─────────────•──────────────•────────────•
///
///                           •
///                         Right
/// ```
///
/// The collinear relations distinguish whether the point lies:
///
/// - before the segment origin;
/// - at either endpoint;
/// - strictly between the endpoints;
/// - or beyond the destination.
///
/// A degenerate segment whose endpoints coincide has no such relation.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointSegmentRelation {
    /// The point lies to the left of the directed segment.
    Left,

    /// The point lies to the right of the directed segment.
    Right,

    /// The point is collinear and lies before the origin.
    Behind,

    /// The point coincides with the segment origin.
    Origin,

    /// The point is collinear and lies strictly between the endpoints.
    Between,

    /// The point coincides with the segment destination.
    Destination,

    /// The point is collinear and lies beyond the destination.
    Beyond,
}
_impl_init! { Self::Origin => PointSegmentRelation }

impl PointSegmentRelation {
    /// Returns the relation for the oppositely directed segment.
    ///
    /// Reversing the segment exchanges its sides, endpoints, and outward
    /// directions while preserving its interior.
    pub const fn reversed(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Behind => Self::Beyond,
            Self::Origin => Self::Destination,
            Self::Between => Self::Between,
            Self::Destination => Self::Origin,
            Self::Beyond => Self::Behind,
        }
    }
    /// Returns the corresponding planar turn.
    pub const fn as_turn(self) -> Turn {
        match self {
            Self::Left => Turn::Left,
            Self::Right => Turn::Right,
            Self::Behind | Self::Origin | Self::Between | Self::Destination | Self::Beyond => {
                Turn::Collinear
            }
        }
    }
    /// Whether the point lies to the left of the directed segment.
    pub const fn is_left(self) -> bool {
        matches!(self, Self::Left)
    }
    /// Whether the point lies to the right of the directed segment.
    pub const fn is_right(self) -> bool {
        matches!(self, Self::Right)
    }
    /// Whether the point is collinear with the segment.
    pub const fn is_collinear(self) -> bool {
        !matches!(self, Self::Left | Self::Right)
    }
    /// Whether the point belongs to the closed segment.
    pub const fn is_on_segment(self) -> bool {
        matches!(self, Self::Origin | Self::Between | Self::Destination)
    }
    /// Whether the point coincides with either endpoint.
    pub const fn is_endpoint(self) -> bool {
        matches!(self, Self::Origin | Self::Destination)
    }
}
