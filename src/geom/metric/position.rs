// devela::geom::metric::position
//
//! Defines [`Position`][1|2|3], [`pos!`].
//!
//! > A geometrical position.
//

#[cfg(doc)]
use crate::{Distance, Orientation};

#[doc = crate::_tags!(geom)]
/// A location in `D`-dimensional space.
#[doc = crate::_doc_location!("geom/metric")]
///
/// Represents an absolute position in a coordinate system.
///
/// - Unlike [`Distance`], `Position` is **not relative**, it describes
///   a fixed location rather than a displacement.
/// - Unlike [`Orientation`], `Position` is a **location**, not a direction.
///
/// See also: [`Position1`], [`Position2`], [`Position3`], [`pos!`].
#[must_use]
#[repr(transparent)]
pub struct Position<T, const D: usize> {
    /// The coordinate values in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Position`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Position1<T> = Position<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Position`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Position2<T> = Position<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Position`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Position3<T> = Position<T, 3>;

crate::_define_geom_dim_macro![($) pos, "a", Position, geom, "geom/metric"];

crate::_impl_geom_dim![common_methods: Position];
crate::_impl_geom_dim![common_traits: Position];

#[cfg(test)]
mod tests {
    use crate::{Position, Position2, pos};

    #[test]
    fn test_macro_surface() {
        assert_eq![Position::<i32, 1>::new([2]), pos!(2)];
        assert_eq![Position2::<i32>::new([2, 5]), pos!(2, 5)];
        assert_eq![pos!([3; 6]).dim, [3, 3, 3, 3, 3, 3]];

        let p = pos!(2_i32, 5_i32);
        assert_eq![pos!(checked p => u8), Ok(Position2::<u8>::new([2, 5]))];
        assert_eq![pos!(saturating => u8; 300_i32, -5_i32), Position2::<u8>::new([255, 0])];
    }
}
