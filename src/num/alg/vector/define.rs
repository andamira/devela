// devela/src/num/alg/vector/define.rs
//
//! Fixed coordinate vectors and their algebra.
//

/* types */

#[doc = crate::_tags!(lin)]
/// A fixed `D`-dimensional coordinate vector.
#[doc = crate::_doc_meta!{location("num/alg")}]
///
/// Its components are expressed in a chosen basis. Vector addition and scalar
/// multiplication are available when supported by the component type.
///
/// Geometric displacement, Euclidean length, normalization, and orientation
/// are interpretations or additional structures, not representation invariants.
#[must_use]
#[repr(transparent)]
pub struct Vector<T, const D: usize> {
    /// The vector coordinates in some basis.
    pub coords: [T; D],
}

#[doc = crate::_tags!(lin)]
/// A static 2-dimensional vector.
#[doc = crate::_doc_meta!{location("num/alg")}]
pub type Vector2d<T> = Vector<T, 2>;

#[doc = crate::_tags!(lin)]
/// A static 3-dimensional vector.
#[doc = crate::_doc_meta!{location("num/alg")}]
pub type Vector3d<T> = Vector<T, 3>;

mod impl_traits {
    use crate::{ConstInit, Debug, FmtResult, Formatter, Hash, Hasher, Vector, init_array};

    impl<T: Clone, const D: usize> Clone for Vector<T, D> {
        fn clone(&self) -> Self {
            Self { coords: self.coords.clone() }
        }
    }

    impl<T: Copy, const D: usize> Copy for Vector<T, D> {}

    impl<T: Default, const D: usize> Default for Vector<T, D> {
        /// Returns a vector filled with each component's default value.
        fn default() -> Self {
            Self::new(init_array![default [T; D], "safe_num", "unsafe_array"])
        }
    }

    impl<T: ConstInit, const D: usize> ConstInit for Vector<T, D> {
        /// A vector filled with each component's initial value.
        const INIT: Self = Self::new(init_array![INIT in ConstInit [T; D]]);
    }

    impl<T: Debug, const D: usize> Debug for Vector<T, D> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            f.debug_struct("Vector").field("D", &D).field("coords", &self.coords).finish()
        }
    }

    impl<T: Eq, const D: usize> Eq for Vector<T, D> {}
    impl<T: PartialEq, const D: usize> PartialEq for Vector<T, D> {
        fn eq(&self, other: &Self) -> bool {
            self.coords == other.coords
        }
    }

    impl<T: Hash, const D: usize> Hash for Vector<T, D> {
        fn hash<HR: Hasher>(&self, state: &mut HR) {
            self.coords.hash(state);
        }
    }
}
