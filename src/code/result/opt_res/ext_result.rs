// devela::code::result::ext
//
//!
//

/// Marker trait to prevent downstream implementations of the [`ExtResult`] trait.
trait Sealed {}
impl<T, E> Sealed for Result<T, E> {}

#[doc = crate::TAG_RESULT!()]
/// Extension trait providing additional methods for [`Result`].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`ExtOption`][crate::ExtOption].
///
/// Based on work from:
/// - <https://github.com/rust-lang/rust/issues/62358> (contains).
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtResult<T, E>: Sealed {
    /// Returns `true` if the result is an [`Ok`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// # use devela::ExtResult;
    /// assert_eq!(Ok::<_, ()>(1).contains(&1), true);
    /// assert_eq!(Ok::<_, ()>(1).contains(&2), false);
    /// assert_eq!(Err::<u8, _>("err").contains(&1), false);
    /// ```
    #[must_use]
    fn contains<U: PartialEq<T>>(&self, x: &U) -> bool;

    /// Returns `true` if the result is an [`Err`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// # use devela::ExtResult;
    /// assert_eq!(Ok::<_, &str>(1).contains_err(&"Some error message"), false);
    /// assert_eq!(Err::<u8, _>("err").contains_err(&"err"), true);
    /// assert_eq!(Err::<u8, _>("err2").contains_err(&"err"), false);
    /// ```
    #[must_use]
    fn contains_err<F: PartialEq<E>>(&self, f: &F) -> bool;

    // WIP
    // /// Merges `self` with another `Result`.
    // ///
    // /// Returns
    // /// - `Ok(f(l, r))` if both options are `Ok(_)`.
    // /// - `Err((le, re))` if any or both options are `Err(_)`.
    // ///
    // /// # Examples
    // /// ```
    // /// # use devela::ExtOption;
    // /// # use core::{cmp::min, ops::Add};
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
    // fn reduce<F: FnOnce(T, T) -> T>(self, other: Result<T, E>, f: F) -> Result<T, E>;
}

impl<T, E> ExtResult<T, E> for Result<T, E> {
    fn contains<U: PartialEq<T>>(&self, x: &U) -> bool {
        self.as_ref().is_ok_and(|y| x == y)
    }

    fn contains_err<F: PartialEq<E>>(&self, f: &F) -> bool {
        self.as_ref().err().is_some_and(|e| f == e)
    }

    // // WIP
    // fn reduce<F: FnOnce(T, T) -> T>(self, other: Result<T, E>, f: F) -> Result<T, E> {
    //     match (self, other) {
    //         (Some(l), Some(r)) => Some(f(l, r)),
    //         (x @ Some(_), None) | (None, x @ Some(_)) => x,
    //         (None, None) => None,
    //     }
    // }
}
