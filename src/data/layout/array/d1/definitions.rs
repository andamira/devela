// devela::data::layout::array::d1::definitions
//
//! 1-dimensional array definitions
//

use crate::{Bare, Storage};

#[doc = crate::_tags!(data_structure)]
/// A static 1-dimensional array backed by the core [array] primitive.
#[doc = crate::_doc_location!("data/layout/array")]
///
/// It is generic in respect to its
/// elements (`T`),
/// storage (`S`)
/// and capacity (`CAP`).
///
/// See also the related trait: [`DataArray`][crate::DataArray].
///
/// ## Methods
///
/// - Constructors:
///   [`new`][Self::new],
///   [`new_boxed`][Self::new_boxed]*(`alloc`)*,
///   [`new_bare`][Self::new_bare](*const*),
///   [`with_cloned`][Self::with_cloned],
///   [`with_copied`][Self::with_copied].
/// - Deconstructors:
///   [`as_slice`][Self::as_slice],
///   [`as_bare_slice`][Self::as_bare_slice](*const*),
///   [`as_mut_slice`][Self::as_mut_slice],
///   [`into_array`][Self::into_array]*([`copy`][Self::into_array_copy])*,
///   [`into_slice`][Self::into_slice]*(`alloc`)*,
///   [`into_vec`][Self::into_vec]*(`alloc`)*.
/// - Queries:
///   [`capacity`][Self::capacity],
///   [`contains`][Self::contains].
///
/// ---
///
/// - [Methods depending on `Option<T>`](#operations-depending-on-option-t).
///   - Over single elements:
///   [`take`][Self::take],
///   [`replace`][Self::replace],
///   [`unset`][Self::unset].
///   - Over all elements:
///   [`clear`][Self::clear],
///   [`fill_none`][Self::fill_none]*(Clone)*.
///   - Queries:
///   [`count_none`][Self::count_none],
///   [`count_some`][Self::count_some],
///   [`is_empty`][Self::is_empty],
///   [`is_full`][Self::is_full],
///   [`first_none`][Self::first_none],
///   [`first_none_mut`][Self::first_none_mut],
///   [`first_none_ref`][Self::first_none_ref],
///   [`first_some`][Self::first_some],
///   [`first_some_mut`][Self::first_some_mut],
///   [`first_some_ref`][Self::first_some_ref].
///
/// - [Methods depending on `Option<T: Copy>`](#operations-depending-on-option-t-copy).
///   - Queries:
///   [`is_bare_empty`][Self::is_bare_empty]*(const)*,
///   [`is_bare_full`][Self::is_bare_full]*(const)*.
#[must_use]
pub struct Array<T, const CAP: usize, S: Storage = Bare> {
    pub(crate) data: S::Stored<[T; CAP]>,
    // pub struct Array<T, const CAP: usize> {
    // TODO WIP
    // pub(crate) data: [T; CAP],
    // len: u8; // TODO
}
