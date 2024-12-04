// devela::data::collections::array::d2::definitions
//
//! 2-dimensional array definitions
//

use crate::{Array, Bare, Storage};

/// A static 2-dimensional [`Array`].
///
/// It is generic in respect to its
/// elements (`T`),
/// columns (`C`),
/// rows (`R`),
/// size (`CR`),
/// storage order (`RMAJ`)
/// and storage (`S`).
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
///   [`len`][Self::len]
///   [`x_len`][Self::x_len]
///   [`y_len`][Self::y_len]
///   [`is_empty`][Self::is_empty],
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
/// Note that the `Default` and `ConstDefault` constructors will panic if `C * R != CR`.
//
// WAIT: [adt_const_params](https://github.com/rust-lang/rust/issues/95174)
//       will allow to use enums and arrays as const-generic parameters.
// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560)
//       will allow calculating CR automatically from C and R. WAIT: Depends on:
// DONE:1.78: [allow constants refer to statics](https://github.com/rust-lang/rust/pull/119614)
// DONE:1.77: [const-eval interning improvements](https://github.com/rust-lang/rust/pull/119044)
// DONE:1.76: [compile-time evaluation improvements](https://github.com/rust-lang/rust/pull/118324)
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
