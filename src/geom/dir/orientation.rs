// devela::geom::dir::orientation
//
//! Defines [`Orientation`][1|2|3], [`ori!`].
//

#[cfg(doc)]
use crate::{Distance, Position};

#[doc = crate::_tags!(geom_dir)]
/// A unitless directional vector in `D`-dimensional space.
#[doc = crate::_doc_location!("geom/dir")]
///
/// Represents **only the direction of movement**, without an absolute
/// reference point or inherent magnitude. It is **typically normalized**
/// to remove scale dependence.
///
/// - Unlike [`Position`], `Orientation` **does not describe a fixed location**.
/// - Unlike [`Distance`], `Orientation` **does not measure separation**.
///
/// This type does **not enforce normalization**, but it is expected
/// to be normalized in most use cases.
///
/// See also: [`Orientation1`], [`Orientation2`], [`Orientation3`], [`ori!`].
#[must_use]
#[repr(transparent)]
pub struct Orientation<T, const D: usize> {
    /// The directional components in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom_dir)]
/// A 1-dimensional [`Orientation`].
#[doc = crate::_doc_location!("geom/dir")]
pub type Orientation1<T> = Orientation<T, 1>;

#[doc = crate::_tags!(geom_dir)]
/// A 2-dimensional [`Orientation`].
#[doc = crate::_doc_location!("geom/dir")]
pub type Orientation2<T> = Orientation<T, 2>;

#[doc = crate::_tags!(geom_dir)]
/// A 3-dimensional [`Orientation`].
#[doc = crate::_doc_location!("geom/dir")]
pub type Orientation3<T> = Orientation<T, 3>;

crate::_define_geom_dim_macro![($) ori, "an", Orientation, geom_dir, "geom/dir"];

crate::_impl_geom_dim![common_methods: Orientation];
crate::_impl_geom_dim![common_traits: Orientation];

#[cfg(test)]
mod tests {
    use crate::{Orientation, Orientation2, ori};

    #[test]
    fn test_macro_surface() {
        assert_eq![Orientation::<i32, 1>::new([2]), ori!(2)];
        assert_eq![Orientation2::<i32>::new([2, 5]), ori!(2, 5)];
        assert_eq![ori!([3; 6]).dim, [3, 3, 3, 3, 3, 3]];

        let p = ori!(2_i32, 5_i32);
        assert_eq![ori!(checked p => u8), Ok(Orientation2::<u8>::new([2, 5]))];
        assert_eq![ori!(saturating => u8; 300_i32, -5_i32), Orientation2::<u8>::new([255, 0])];
    }
}
