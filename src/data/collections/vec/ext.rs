// devela::data::collections::vec::ext
//
//!
//

use crate::_deps::alloc::vec::Vec;

// Marker trait to prevent downstream implementations of the `ExtVec` trait.
#[rustfmt::skip] mod private { pub trait Sealed {} }
impl<T> private::Sealed for Vec<T> {}

/// Extension trait providing additional methods for [`Vec`].
///
/// This trait is sealed and cannot be implemented for any other type.
pub trait ExtVec<T>: private::Sealed {
    /* convert */

    /// Converts `Vec<T>` to `Vec<U>` when `U` implements `From<T>`.
    /// # Examples
    /// ```
    /// # use devela::data::ExtVec;
    /// assert_eq![vec![1_u16, 2, 3], vec![1_u8, 2, 3].vec_into_vec()];
    /// assert_eq![vec![1_u16, 2, 3], vec![1_u8, 2, 3].vec_into_vec::<u16>()];
    /// ```
    #[must_use]
    fn vec_into_vec<U>(self) -> Vec<U>
    where
        U: From<T>;

    /// Tries to convert `Vec<T>` to `Vec<U>` when `U` implements `TryFrom<T>`.
    /// # Examples
    /// ```
    /// # use devela::data::ExtVec;
    /// assert_eq![Ok(vec![1_i32, 2, 3]), vec![1_i64, 2, 3].vec_try_into_vec()];
    /// assert_eq![Ok(vec![1_i32, 2, 3]), vec![1_i64, 2, 3].vec_try_into_vec::<_, i32>()];
    /// ```
    fn vec_try_into_vec<E, U>(self) -> Result<Vec<U>, E>
    where
        U: TryFrom<T, Error = E>;
}

impl<T> ExtVec<T> for Vec<T> {
    /* convert */

    #[inline] #[rustfmt::skip]
    fn vec_into_vec<U>(self) -> Vec<U> where U: From<T> {
        self.into_iter().map(|t| U::from(t)).collect::<Vec<_>>().into_iter().collect()
    }
    #[inline] #[rustfmt::skip]
    fn vec_try_into_vec<E, U>(self) -> Result<Vec<U>, E> where U: TryFrom<T, Error = E> {
        self.into_iter()
            // 1. Vec<Result<_>>:
            .map(U::try_from)
            .collect::<Vec<_>>()
            // 2. Result<Vec<_>>:
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }
}
