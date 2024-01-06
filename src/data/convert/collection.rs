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

/// Converts `&[T]` to `Vec<U>` when `U` implements `From<T>`.
///
/// # Examples
/// ```
/// use devela::data::slice_into_vec;
///
/// let a: Vec<u32> = slice_into_vec(&[1_u8, 2, 3]);
/// assert_eq![a, vec![1_u32, 2, 3]];
/// ```
#[inline]
#[must_use]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn slice_into_vec<T: Clone, U: From<T>>(slice: &[T]) -> Vec<U> {
    slice
        .iter()
        .map(|t| U::from(t.clone()))
        .collect::<Vec<_>>()
        .into_iter()
        .collect()
}

/// Converts `&[T]` to `[U; N]` when `U` implements `From<T>`.
///
/// # Panics
/// Panics if the length of the slice is less than the length of the array.
///
/// # Examples
/// ```
/// use devela::data::slice_into_array;
///
/// let a: [u32; 3] = slice_into_array(&[1_u8, 2, 3, 4, 5]);
/// assert_eq![a, [1_u32, 2, 3]];
/// ```
///
/// # Features
/// When either the `unsafe_data` or `unsafe_mem` features are enabled it uses
/// `MaybeUninit` to improve performance.
///
// IMPROVE make a try_slice_into_array version:
// WAITING https://doc.rust-lang.org/nightly/core/array/fn.try_from_fn.html
#[inline]
#[must_use]
pub fn slice_into_array<T: Clone, U: From<T>, const N: usize>(slice: &[T]) -> [U; N] {
    if slice.len() >= N {
        #[cfg(not(feature = "unsafe_data"))]
        {
            let mut array: [U; N] = core::array::from_fn(|i| U::from(slice[i].clone()));

            for (i, item) in slice.iter().take(N).enumerate() {
                array[i] = U::from(item.clone());
            }
            array
        }

        // SAFETY: we make sure of initializing every array element
        #[cfg(feature = "unsafe_data")]
        {
            use core::mem::MaybeUninit;

            let mut array: [MaybeUninit<U>; N] = unsafe { MaybeUninit::uninit().assume_init() };
            for i in 0..N {
                array[i] = MaybeUninit::new(U::from(slice[i].clone()));
            }
            array.map(|x| unsafe { x.assume_init() })
        }
    } else {
        panic!("Input slice length is less than the array size")
    }
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

/// Tries to convert `&[T]` to `Vec<U>` when `U` implements `TryFrom<T>`.
///
/// # Examples
/// ```
/// use devela::data::try_slice_into_vec;
///
/// let a: Result<Vec<u32>, _> = try_slice_into_vec(&[1_u64, 2, 3]);
/// assert_eq![a, Ok(vec![1_u32, 2, 3])];
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn try_slice_into_vec<T: Clone, E, U: TryFrom<T, Error = E>>(slice: &[T]) -> Result<Vec<U>, E> {
    slice
        // 1. Vec<Result<_>>:
        .iter()
        .map(|t| U::try_from(t.clone()))
        .collect::<Vec<_>>()
        // 2. Result<Vec<_>>:
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
}
