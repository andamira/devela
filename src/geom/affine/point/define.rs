// devela/src/geom/affine/point/define.rs
//
//!
//

/* definitions */

#[doc = crate::_tags!(geom)]
/// A coordinate point in `D`-dimensional affine space.
#[doc = crate::_doc_meta!{location("geom/affine")}]
///
/// A point represents a position, not a displacement.
///
/// [`Vector`][crate::Vector] acts on a point by translation:
/// - `point + vector` produces another point,
/// - `point - vector` produces another point,
/// - `destination - origin` produces the displacement vector.
///
/// Points do not themselves form a vector space: two points cannot be added,
/// and a point has no intrinsic negation or scalar multiplication.
#[must_use]
#[repr(transparent)]
pub struct Point<T, const D: usize> {
    /// The D-dimensional coordinates.
    pub coords: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 2-dimensional affine point.
#[doc = crate::_doc_meta!{location("geom/affine")}]
pub type Point2d<T> = Point<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional affine point.
#[doc = crate::_doc_meta!{location("geom/affine")}]
pub type Point3d<T> = Point<T, 3>;

/* implementations */

#[rustfmt::skip]
impl<T, const D: usize> Point<T, D> {
    /// Returns a new `Point` from the given `coords` array.
    pub const fn new(coords: [T; D]) -> Self { Self { coords } }
}

/* manual impls for specific dimensionalities */

#[rustfmt::skip]
impl<T> Point2d<T> {
    /// Returns a copy of the first dimension `x`.
    #[must_use] pub const fn x(&self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[must_use] pub const fn y(&self) -> T where T: Copy { self.coords[1] }

    /// Returns a shared reference to the first dimension `x`.
    #[must_use] pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[must_use] pub const fn y_ref(&self) -> &T { &self.coords[1] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[must_use] pub const fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[must_use] pub const fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
}

#[rustfmt::skip]
impl<T> Point3d<T> {
    /// Returns a copy of the first dimension `x`.
    #[must_use] pub const fn x(&self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[must_use] pub const fn y(&self) -> T where T: Copy { self.coords[1] }
    /// Returns a copy of the third dimension `z`.
    #[must_use] pub const fn z(&self) -> T where T: Copy { self.coords[2] }

    /// Returns a shared reference to the first dimension `x`.
    #[must_use] pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[must_use] pub const fn y_ref(&self) -> &T { &self.coords[1] }
    /// Returns a shared reference to the third dimension `z`.
    #[must_use] pub const fn z_ref(&self) -> &T { &self.coords[2] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[must_use] pub const fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[must_use] pub const fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
    /// Returns an exclusive reference to the third dimension `z`.
    #[must_use] pub const fn z_mut(&mut self) -> &mut T { &mut self.coords[2] }
}

mod impl_traits {
    use crate::{ArrayExt, Point, init_array};
    use crate::{ConstInit, Debug, Display, FmtResult, Formatter, Hash, Hasher};

    impl<T: Clone, const D: usize> Clone for Point<T, D> {
        fn clone(&self) -> Self {
            Self::new(self.coords.clone())
        }
    }
    impl<T: Copy, const D: usize> Copy for Point<T, D> {}

    impl<T: Default, const D: usize> Default for Point<T, D> {
        fn default() -> Self {
            Self::new(init_array![default [T; D], "safe_geom", "unsafe_array"])
        }
    }
    impl<T: ConstInit, const D: usize> ConstInit for Point<T, D> {
        const INIT: Self = Self::new(init_array![INIT in ConstInit [T; D]]);
    }

    impl<T: Debug, const D: usize> Debug for Point<T, D> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            f.debug_tuple("Point").field(&self.coords).finish()
        }
    }
    impl<T: Display, const D: usize> Display for Point<T, D> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write!(f, "Point {{ coords: {} }}", self.coords.fmt())
        }
    }

    impl<T: PartialEq, const D: usize> PartialEq for Point<T, D> {
        fn eq(&self, other: &Self) -> bool {
            self.coords == other.coords
        }
    }
    impl<T: Eq, const D: usize> Eq for Point<T, D> {}

    impl<T: Hash, const D: usize> Hash for Point<T, D> {
        fn hash<HR: Hasher>(&self, state: &mut HR) {
            self.coords.hash(state);
        }
    }
}
