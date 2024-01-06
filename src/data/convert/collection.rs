// devela::data::convert::collection
//
//! Helpers for converting between collections.
//

#[cfg(feature = "alloc")]
use _alloc::vec::Vec;

/// Converts `Vec<T>` to `Vec<U>` when `U` implements `From<T>`.
///
/// # Examples
/// ```
/// use devela::data::vec_into_vec;
///
/// let a: Vec<u32> = vec_into_vec(vec![1_u8, 2, 3]);
/// assert_eq![a, vec![1_u32, 2, 3]];
/// ```
#[inline]
#[must_use]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn vec_into_vec<T, U: From<T>>(vec: Vec<T>) -> Vec<U> {
    vec.into_iter()
        .map(|t| U::from(t))
        .collect::<Vec<_>>()
        .into_iter()
        .collect()
}

/// Tries to convert `Vec<T>` to `Vec<U>` when `U` implements `TryFrom<T>`.
///
/// # Examples
/// ```
/// use devela::data::try_vec_into_vec;
///
/// let a: Result<Vec<u32>, _> = try_vec_into_vec(vec![1_u64, 2, 3]);
/// assert_eq![a, Ok(vec![1_u32, 2, 3])];
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn try_vec_into_vec<T, E, U: TryFrom<T, Error = E>>(vec: Vec<T>) -> Result<Vec<U>, E> {
    vec.into_iter()
        // 1. Vec<Result<_>>:
        .map(U::try_from)
        .collect::<Vec<_>>()
        // 2. Result<Vec<_>>:
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
}
