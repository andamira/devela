// devela::code::result::ext
//
//!
//

// Marker trait to prevent downstream implementations of the `ExtResult` trait.
impl<T, E> private::Sealed for Result<T, E> {}
mod private {
    pub trait Sealed {}
}

/// Extension trait providing additional methods for [`Result`].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`ExtOption`][crate::code::ExtOption].
//
// Based on work from:
// - https://github.com/rust-lang/rust/issues/62358 (closed proposal).
// - https://crates.io/crates/result-ext/0.2.0 by Simon Ochsenreither
pub trait ExtResult<T, E>: private::Sealed {
    /// Returns `true` if the result is an [`Ok`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// use devela::code::ExtResult;
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.contains(&2), true);
    ///
    /// let x: Result<u32, &str> = Ok(3);
    /// assert_eq!(x.contains(&2), false);
    ///
    /// let x: Result<u32, &str> = Err("Some error message");
    /// assert_eq!(x.contains(&2), false);
    /// ```
    #[must_use]
    fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>;

    /// Returns `true` if the result is an [`Err`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// use devela::code::ExtResult;
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.contains_err(&"Some error message"), false);
    ///
    /// let x: Result<u32, &str> = Err("Some error message");
    /// assert_eq!(x.contains_err(&"Some error message"), true);
    ///
    /// let x: Result<u32, &str> = Err("Some other error message");
    /// assert_eq!(x.contains_err(&"Some error message"), false);
    /// ```
    #[must_use]
    fn contains_err<F>(&self, f: &F) -> bool
    where
        F: PartialEq<E>;

    // WIP
    // /// Merges `self` with another `Result`.
    // ///
    // /// Returns
    // /// - `Ok(f(l, r))` if both options are `Ok(_)`.
    // /// - `Err((le, re))` if any or both options are `Err(_)`.
    // ///
    // /// # Examples
    // /// ```
    // /// use devela::code::ExtOption;
    // /// use core::{cmp::min, ops::Add};
    // ///
    // /// let x = Some(2);
    // /// let y = Some(4);
    // ///
    // /// assert_eq!(x.reduce(y, Add::add), Some(6));
    // /// assert_eq!(x.reduce(y, min), Some(2));
    // ///
    // /// assert_eq!(x.reduce(None, Add::add), x);
    // /// assert_eq!(None.reduce(y, min), y);
    // ///
    // /// assert_eq!(None.reduce(None, i32::add), None);
    // /// ```
    // fn reduce<F>(self, other: Result<T, E>, f: F) -> Result<T, E>
    // where
    //     F: FnOnce(T, T) -> T;
}

impl<T, E> ExtResult<T, E> for Result<T, E> {
    #[inline]
    fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match *self {
            Ok(ref y) => x == y,
            Err(_) => false,
        }
    }

    #[inline]
    fn contains_err<F>(&self, f: &F) -> bool
    where
        F: PartialEq<E>,
    {
        match *self {
            Ok(_) => false,
            Err(ref e) => f == e,
        }
    }

    // // WIP
    // #[inline]
    // fn reduce<F>(self, other: Result<T, E>, f: F) -> Result<T, E>
    // where
    //     F: FnOnce(T, T) -> T,
    // {
    //     match (self, other) {
    //         (Some(l), Some(r)) => Some(f(l, r)),
    //         (x @ Some(_), None) | (None, x @ Some(_)) => x,
    //         (None, None) => None,
    //     }
    // }
}
