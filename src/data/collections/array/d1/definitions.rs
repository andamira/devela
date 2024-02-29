// devela::data::collections::array::d1::definitions
//
//! 1-dimensional array definitions
//
// TOC
// - define Array
// - define trait DataArray
// - implement DataCollection for Array
// - implement DataArray for Array

use crate::{
    data::{DataCollection, DataError as E, DataResult as Result},
    mem::{Bare, Storage},
};

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

/* types */

/// A const generic 1-dimensional array backed by the core [`array`] primitive.
///
/// It is generic in respect to its
/// elements (`T`),
/// storage (`S`)
/// and length (`LEN`).
///
/// See also the related trait: [`DataArray`][crate::DataArray].
///
/// ## Methods
///
/// - Constructors:
///   [`new`][Self::new],
///   [`new_boxed`][Self::new_boxed]*(`alloc`)*,
///   [`new_bare`][Self::new_bare].
///   [`with_cloned`][Self::with_cloned].
///   [`with_copied`][Self::with_copied].
/// - Deconstructors:
///   [`as_slice`][Self::as_slice],
///   [`as_mut_slice`][Self::as_mut_slice],
///   [`into_array`][Self::into_array]*([`const`][Self::into_array_const])*,
///   [`into_slice`][Self::into_slice]*(`alloc`)*,
///   [`into_vec`][Self::into_vec]*(`alloc`)*.
/// - Queries:
///   [`len`][Self::len], [`is_empty`][Self::is_empty],
///   [`contains`][Self::contains].
#[must_use]
pub struct Array<T, const LEN: usize, S: Storage = Bare> {
    pub(crate) data: S::Stored<[T; LEN]>,
}
