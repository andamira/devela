// devela::num::geom::algebra::vector
//
//! Linear algebra vectors.
//!
//! Vectors represent the difference between two positions.
//!
//! They are characterized by their *direction* and *magnitude*, and
//! their direction can be decomposed into *orientation* and *sense*.
//

use crate::num::Num;
#[cfg(feature = "alloc")]
use crate::{
    _liballoc::{boxed::Box, vec::Vec},
    num::{NumError, NumResult as Result},
};

mod impl_array;
#[cfg(feature = "alloc")]
mod impl_vec;

/* types */

/// A static `D`-dimensional vector, backed by a primitive [`array`].
#[repr(transparent)]
pub struct Vector<T, const D: usize> {
    /// The vector coordinates in some basis.
    pub array: [T; D],
}

/// A static 2-dimensional vector.
pub type Vector2d<T> = Vector<T, 2>;

/// A static 3-dimensional vector.
pub type Vector3d<T> = Vector<T, 3>;

/// A dynamic vector, backed by a primitive [`Vec`].
#[repr(transparent)]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub struct VecVector<T> {
    /// The vector coordinates in some basis.
    pub vec: Vec<T>,
}

/* trait */

/// A common trait for all vectors.
pub trait NumVector: Num {
    /// The associated scalar type.
    type Scalar;
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
