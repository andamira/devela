// devela::geom::metric::distance
//
//! Defines [`Distance`][1|2|3], [`dis!`].
//

#[cfg(doc)]
use crate::{Orientation, Position};

#[doc = crate::_tags!(geom)]
/// A separation between two locations in `D`-dimensional space.
#[doc = crate::_doc_location!("geom/metric")]
///
/// Represents a displacement vector **without an absolute origin**.
/// It describes the magnitude of separation between positions.
///
/// - Unlike [`Position`], `Distance` is **relative**,
///   and represents how far apart two positions are.
/// - Unlike [`Orientation`], `Distance` has **magnitude** but no defined direction.
///
/// See also: [`Distance1`], [`Distance2`], [`Distance3`], [`dis!`].
#[must_use]
#[repr(transparent)]
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Distance`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Distance1<T> = Distance<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Distance`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Distance2<T> = Distance<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Distance`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Distance3<T> = Distance<T, 3>;

crate::_define_geom_dim_macro![($) dis, "a", Distance, geom, "geom/metric"];

crate::_impl_geom_dim![common_methods: Distance];
crate::_impl_geom_dim![common_traits: Distance];

#[cfg(test)]
mod tests {
    use crate::{Distance, Distance2, dis};

    #[test]
    fn test_macro_surface() {
        assert_eq![Distance::<i32, 1>::new([2]), dis!(2)];
        assert_eq![Distance2::<i32>::new([2, 5]), dis!(2, 5)];
        assert_eq![dis!([3; 6]).dim, [3, 3, 3, 3, 3, 3]];

        let p = dis!(2_i32, 5_i32);
        assert_eq![dis!(checked p => u8), Ok(Distance2::<u8>::new([2, 5]))];
        assert_eq![dis!(saturating => u8; 300_i32, -5_i32), Distance2::<u8>::new([255, 0])];
    }
}
