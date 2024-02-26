// devela::data::collections::array::d2::definitions
//
//! 2-dimensional array definitions
//

#[cfg(feature = "alloc")]
use crate::{_deps::alloc::vec::Vec, mem::Boxed};
use crate::{
    data::{Array, DataResult as Result},
    mem::{Bare, Storage},
};

/// A const-generic 2-dimensional [`Array`].
///
/// It is generic in respect to its
/// elements (`T`),
/// storage (`S`),
/// x-length (`X`),
/// y-length (`Y`),
/// total length (`LEN`)
/// and storage order (`XMAJ`).
///
/// The lenght `LEN` must be equal to `X` * `Y`.
///
/// The
/// [*storage order*](https://en.wikipedia.org/wiki/Row-_and_column-major_order)
/// is X-major by default (`XMAJ = true`) and can be made Y-major if set to false.
///
/// ## Methods
///
/// - Construct:
///   [`with_cloned`][Self::with_cloned],
///   [`with_copied`][Self::with_copied].
/// - Deconstruct:
///   [`as_slice`][Self::as_slice],
///   [`as_mut_slice`][Self::as_mut_slice],
///   [`into_array`][Self::into_array]*([`const`][Self::into_array_const])*,
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
///   - X-major:
///   [`get_ref_ymaj`][Self::get_ref_ymaj]*([`uc`][Self::get_ref_ymaj_unchecked])*,
///   [`get_mut_ymaj`][Self::get_mut_ymaj]*([`uc`][Self::get_mut_ymaj_unchecked])*,
///   [`set_ymaj`][Self::set_ymaj]*([`uc`][Self::set_ymaj_unchecked])*,
///   [`get_index_ymaj`][Self::get_index_ymaj]*([`uc`][Self::get_index_ymaj_unchecked])*,
///   [`get_coords_ymaj`][Self::get_coords_ymaj]*([`uc`][Self::get_coords_ymaj_unchecked])*.
///   - Y-major:
///   [`get_ref_xmaj`][Self::get_ref_xmaj]*([`uc`][Self::get_ref_xmaj_unchecked])*,
///   [`get_mut_xmaj`][Self::get_mut_xmaj]*([`uc`][Self::get_mut_xmaj_unchecked])*,
///   [`set_xmaj`][Self::set_xmaj]*([`uc`][Self::set_xmaj_unchecked])*,
///   [`get_index_xmaj`][Self::get_index_xmaj]*([`uc`][Self::get_index_xmaj_unchecked])*,
///   [`get_coords_xmaj`][Self::get_coords_xmaj]*([`uc`][Self::get_coords_xmaj_unchecked])*.
///
/// ## Panics
/// Note that the `Default` and `ConstDefault` constructors will panic if `X * Y != LEN`.
//
// WAIT: [adt_const_params](https://github.com/rust-lang/rust/issues/95174)
//       will allow to use enums and arrays as const-generic parameters.
// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560)
//       will allow calculating LEN automatically from X and Y. WAIT: Depends on:
// WAIT:1.78: [allow constants refer to statics](https://github.com/rust-lang/rust/pull/119614)
// WAIT:1.77: [const-eval interning improvements](https://github.com/rust-lang/rust/pull/119044)
// DONE:1.76: [compile-time evaluation improvements](https://github.com/rust-lang/rust/pull/118324)
pub struct Array2d<
    T,
    S: Storage,
    const X: usize,
    const Y: usize,
    const LEN: usize,
    const XMAJ: bool = true,
> {
    pub(super) array: Array<T, S, LEN>,
}

/// An [`Array2d`] stored in the stack.
pub type BareArray2d<T, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool = true> =
    Array2d<T, Bare, X, Y, LEN, XMAJ>;

/// An [`Array2d`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub type BoxedArray2d<
    T,
    const X: usize,
    const Y: usize,
    const LEN: usize,
    const XMAJ: bool = true,
> = Array2d<T, Boxed, X, Y, LEN, XMAJ>;
