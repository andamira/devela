// devela::num::geom::linear::vector
//
//! Linear algebra vectors.
//!
//! Vectors represent the difference between two positions.
//!
//! They are characterized by their *direction* and *magnitude*, and
//! their direction can be decomposed into *orientation* and *sense*.
//

use crate::Num;
#[cfg(feature = "alloc")]
use crate::{Box, NumError, NumResult as Result, Vec};

mod array;
#[cfg(feature = "alloc")]
mod vec;

/* types */

#[doc = crate::TAG_GEOM!()]
/// A static `D`-dimensional vector, backed by a primitive [`array`][prim@array].
#[repr(transparent)]
pub struct Vector<T, const D: usize> {
    /// The vector coordinates in some basis.
    pub coords: [T; D],
}

#[doc = crate::TAG_GEOM!()]
/// A static 2-dimensional vector.
pub type Vector2d<T> = Vector<T, 2>;

#[doc = crate::TAG_GEOM!()]
/// A static 3-dimensional vector.
pub type Vector3d<T> = Vector<T, 3>;

#[doc = crate::TAG_GEOM!()]
/// A dynamic vector, backed by a primitive [`Vec`].
#[repr(transparent)]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub struct VecVector<T> {
    /// The vector coordinates in some basis.
    pub coords: Vec<T>,
}

/* trait */

#[doc = crate::TAG_GEOM!()]
/// A common trait for all vectors.
pub trait NumVector: Num {
    /// The associated scalar type.
    type Scalar;
}

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T: Num + 'static, const D: usize>
    TryInto<Box<dyn NumVector<Scalar = T, Rhs = Self, Inner = [T; D], Out = Self>>>
    for Vector<T, D>
{
    type Error = NumError;

    fn try_into(
        self,
    ) -> Result<Box<dyn NumVector<Scalar = T, Rhs = Self, Inner = [T; D], Out = Self>>> {
        Ok(Box::new(self))
    }
}
