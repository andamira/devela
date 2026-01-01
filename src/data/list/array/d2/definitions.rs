// devela::data::list::array::d2::definitions
//
//! 2-dimensional array definitions
//

use crate::{Array, Bare, Storage};
// #[cfg(feature = "dep_rkyv")] // DEP_DISABLED
// use rkyv::{Archive, Deserialize, Serialize};

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A static 2-dimensional [`Array`].
#[doc = crate::_doc!(location: "data/list/array")]
///
/// It is generic in respect to its:
/// - elements (`T`),
/// - number of columns (`C`),
/// - number of rows (`R`),
/// - total capacity (`CR`),
/// - storage order (`RMAJ`)
/// - storage abstraction (`S`).
///
/// The total lenght `CR` must be equal to the product `C` * `R`.
///
/// The
/// [*storage order*](https://en.wikipedia.org/wiki/Row-_and_column-major_order)
/// is row-major by default (`RMAJ = true`). It can be column-major if set to false.
///
/// ## Methods
///
/// - Construct:
///   [`with_cloned`][Self::with_cloned],
///   [`with_copied`][Self::with_copied].
/// - Deconstruct:
///   [`as_slice`][Self::as_slice],
///   [`as_mut_slice`][Self::as_mut_slice],
///   [`into_array`][Self::into_array]*([`const`][Self::into_array_copy])*,
///   [`into_slice`][Self::into_slice]*(`alloc`)*,
///   [`into_vec`][Self::into_vec]*(`alloc`)*.
/// - Query:
///   [`capacity`][Self::capacity],
///   [`cap_col`][Self::cap_col],
///   [`cap_row`][Self::cap_row],
///   [`contains`][Self::contains].
/// - Indexing and coordinates (for the current order):
///   - [`get_ref`][Self::get_ref]*([`uc`][Self::get_ref_unchecked])*,
///   [`get_mut`][Self::get_mut]*([`uc`][Self::get_mut_unchecked])*,
///   [`set`][Self::set]*([`uc`][Self::set_unchecked])*,
///   [`get_index`][Self::get_index]*([`uc`][Self::get_index_unchecked])*,
///   [`get_coords`][Self::get_coords]*([`uc`][Self::get_coords_unchecked])*.
/// - Indexing and coordinates (specific for the **opposite** order):
///   - row-major:
///   [`get_ref_cmaj`][Self::get_ref_cmaj]*([`uc`][Self::get_ref_cmaj_unchecked])*,
///   [`get_mut_cmaj`][Self::get_mut_cmaj]*([`uc`][Self::get_mut_cmaj_unchecked])*,
///   [`set_cmaj`][Self::set_cmaj]*([`uc`][Self::set_cmaj_unchecked])*,
///   [`get_index_cmaj`][Self::get_index_cmaj]*([`uc`][Self::get_index_cmaj_unchecked])*,
///   [`get_coords_cmaj`][Self::get_coords_cmaj]*([`uc`][Self::get_coords_cmaj_unchecked])*.
///   - column-major:
///   [`get_ref_rmaj`][Self::get_ref_rmaj]*([`uc`][Self::get_ref_rmaj_unchecked])*,
///   [`get_mut_rmaj`][Self::get_mut_rmaj]*([`uc`][Self::get_mut_rmaj_unchecked])*,
///   [`set_rmaj`][Self::set_rmaj]*([`uc`][Self::set_rmaj_unchecked])*,
///   [`get_index_rmaj`][Self::get_index_rmaj]*([`uc`][Self::get_index_rmaj_unchecked])*,
///   [`get_coords_rmaj`][Self::get_coords_rmaj]*([`uc`][Self::get_coords_rmaj_unchecked])*.
///
/// ## Panics
/// Note that the `Default` and `ConstInit` constructors will panic if `C * R != CR`.
//
// WAIT: [adt_const_params](https://github.com/rust-lang/rust/issues/95174)
//       would allow to use enums and arrays as const-generic parameters.
// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560)
//       would allow calculating CR automatically from C and R.
// #[cfg_attr(feature = "dep_rkyv", derive(Archive, Serialize, Deserialize))]
// rkyv(archived = Array2dArchived, attr(doc = crate::_TAG_RKYV![])))]
// #[cfg_attr(
//     all(feature = "dep_rkyv", nightly_doc),
//     rkyv(attr(doc(cfg(feature = "dep_rkiv"))))
// )]
pub struct Array2d<
    T,
    const C: usize,
    const R: usize,
    const CR: usize,
    const RMAJ: bool = true,
    S: Storage = Bare,
> {
    pub(super) data: Array<T, CR, S>,
}
