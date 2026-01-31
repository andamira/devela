// devela::num::lin::vector
//
//! Linear algebra vectors.
//!
//! Vectors represent the difference between two positions.
//!
//! They are characterized by their *direction* and *magnitude*, and
//! their direction can be decomposed into *orientation* and *sense*.
//

use crate::Num;
// #[cfg(feature = "alloc")]
// use crate::{Box, NumError, NumResult as Result, Vec};

/* types */

#[doc = crate::_tags!(lin)]
/// A static `D`-dimensional vector, backed by a primitive [`array`][prim@array].
#[doc = crate::_doc_location!("num/lin")]
#[repr(transparent)]
pub struct Vector<T, const D: usize> {
    /// The vector coordinates in some basis.
    pub coords: [T; D],
}

#[doc = crate::_tags!(lin)]
/// A static 2-dimensional vector.
#[doc = crate::_doc_location!("num/lin")]
pub type Vector2d<T> = Vector<T, 2>;

#[doc = crate::_tags!(lin)]
/// A static 3-dimensional vector.
#[doc = crate::_doc_location!("num/lin")]
pub type Vector3d<T> = Vector<T, 3>;

// #[doc = crate::_tags!(lin)]
// /// A dynamic vector, backed by a primitive [`Vec`].
// #[doc = crate::_doc_location!("num/lin")]
// #[repr(transparent)]
// #[cfg(feature = "alloc")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
// pub struct VecVector<T> {
//     /// The vector coordinates in some basis.
//     pub coords: Vec<T>,
// }

/* trait */

#[doc = crate::_tags!(wip)]
#[doc = crate::_tags!(lin)]
/// A common trait for all vectors.
#[doc = crate::_doc_location!("num/lin")]
pub trait NumVector: Num {
    /// The associated scalar type.
    type Scalar;

    // fn dot() -> Self::Scalar;
    // fn cross() -> Self::Scalar;
}

// #[cfg(feature = "alloc")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
// impl<T: Num + 'static, const D: usize>
//     TryInto<Box<dyn NumVector<Scalar = T, Rhs = Self, Inner = [T; D], Out = Self>>>
//     for Vector<T, D>
// {
//     type Error = NumError;
//
//     fn try_into(
//         self,
//     ) -> Result<Box<dyn NumVector<Scalar = T, Rhs = Self, Inner = [T; D], Out = Self>>> {
//         Ok(Box::new(self))
//     }
// }
