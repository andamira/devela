// devela::result
//
//! Result, extends [`std::result`].
//

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
    /// use devela::result::ResultExt;
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
    /// use devela::result::ResultExt;
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
