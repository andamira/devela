// devela::ext
//
//! Extension traits for core types.
//
// TOC
// - OptionExt
// - ResultExt

/* Option */

/// Extension trait providing additional methods for `Option`.
//
// Based on work from:
// - https://github.com/rust-lang/rust/issues/62358 closed proposal.
// - https://crates.io/crates/option-ext by Simon Ochsenreither.
// - https://crates.io/crates/opt_reduce by Waffle Lapkin.
pub trait OptionExt<T> {
    /// Returns `true` if the option is a [`Some`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// use devela::OptionExt;
    ///
    /// let x: Option<u32> = Some(2);
    /// assert_eq!(x.contains(&2), true);
    ///
    /// let x: Option<u32> = Some(3);
    /// assert_eq!(x.contains(&2), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.contains(&2), false);
    /// ```
    #[must_use]
    fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>;

    /// Merges `self` with another `Option`.
    ///
    /// Returns
    /// - `Some(f(l, r))` if both options are `Some(_)`.
    /// - `Some(x)` if either of the options is `Some(x)` and the other is `None`.
    /// - `None` if both options are `None`.
    ///
    /// # Examples
    /// ```
    /// use devela::OptionExt;
    /// use core::{cmp::min, ops::Add};
    ///
    /// let x = Some(2);
    /// let y = Some(4);
    ///
    /// assert_eq!(x.reduce(y, Add::add), Some(6));
    /// assert_eq!(x.reduce(y, min), Some(2));
    ///
    /// assert_eq!(x.reduce(None, Add::add), x);
    /// assert_eq!(None.reduce(y, min), y);
    ///
    /// assert_eq!(None.reduce(None, i32::add), None);
    /// ```
    fn reduce<F>(self, other: Option<T>, f: F) -> Option<T>
    where
        F: FnOnce(T, T) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match *self {
            Some(ref y) => x == y,
            None => false,
        }
    }

    #[inline]
    fn reduce<F>(self, other: Option<T>, f: F) -> Option<T>
    where
        F: FnOnce(T, T) -> T,
    {
        match (self, other) {
            (Some(l), Some(r)) => Some(f(l, r)),
            (x @ Some(_), None) | (None, x @ Some(_)) => x,
            (None, None) => None,
        }
    }
}

/* Result */

/// Extension trait providing additional methods for `Result`.
//
// Based on work from:
// - https://github.com/rust-lang/rust/issues/62358 closed proposal.
// - https://crates.io/crates/result-ext by Simon Ochsenreither
pub trait ResultExt<T, E> {
    /// Returns `true` if the result is an [`Ok`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// use devela::ResultExt;
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
    /// use devela::ResultExt;
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
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
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
}
