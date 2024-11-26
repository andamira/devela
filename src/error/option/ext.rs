// devela::error::option::ext
//
//!
//

use super::{OptionFmt, OptionFmtOr, OptionFmtOrElse};
use core::fmt::Display;

#[doc = crate::doc_private!()]
/// Marker trait to prevent downstream implementations of the [`ExtOption`] trait.
trait Sealed {}
impl<T> Sealed for Option<T> {}

/// Extension trait providing additional methods for [`Option`].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`ExtResult`][crate::error::ExtResult],
/// [`ExtOptRes`][crate::error::ExtOptRes].
///
/// Based on work from:
/// - <https://github.com/rust-lang/rust/issues/62358> (contains).
/// - <https://github.com/rust-lang/rust/pull/87036> (reduce).
#[cfg_attr(feature = "nightly_doc", doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtOption<T>: Sealed {
    /// Returns `true` if the option is a [`Some`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// # use devela::ExtOption;
    /// assert_eq!(Some(1).contains(&1), true);
    /// assert_eq!(Some(1).contains(&2), false);
    /// assert_eq!(None::<u8>.contains(&1), false);
    /// ```
    #[must_use]
    fn contains<U: PartialEq<T>>(&self, x: &U) -> bool;

    /// Merges `self` with another `Option`.
    ///
    /// Returns
    /// - `Some(f(l, r))` if both options are `Some(_)`.
    /// - `Some(x)` if either of the options is `Some(x)` and the other is `None`.
    /// - `None` if both options are `None`.
    ///
    /// # Examples
    /// ```
    /// # use devela::ExtOption;
    /// # use core::{cmp::min, ops::Add};
    /// let (x, y) = (Some(2), Some(4));
    ///
    /// assert_eq!(x.reduce(y, Add::add), Some(6));
    /// assert_eq!(x.reduce(y, min), Some(2));
    ///
    /// assert_eq!(x.reduce(None, Add::add), x);
    /// assert_eq!(None.reduce(y, min), y);
    ///
    /// assert_eq!(None.reduce(None, i32::add), None);
    /// ```
    #[must_use]
    fn reduce<F: FnOnce(T, T) -> T>(self, other: Option<T>, f: F) -> Option<T>;

    /// Format some value, or display an empty string if it's `None`.
    ///
    /// # Examples
    /// ```
    /// # use devela::ExtOption;
    /// assert_eq!("0x42", format!("{:#x}", Some(0x42).fmt_or_empty()));
    /// assert_eq!("", format!("{:#x}", None::<u8>.fmt_or_empty()));
    /// ```
    #[must_use]
    fn fmt_or_empty(&self) -> OptionFmt<T>;

    /// Format some value, or an alternative if it's `None`.
    ///
    /// The alternative value must implement [`Display`]
    /// regardless of which formatting is used originally.
    ///
    /// # Examples
    /// ```
    /// # use devela::error::ExtOption;
    /// assert_eq!("42", format!("{}", Some(Box::new(42)).fmt_or("Nothing")));
    /// assert_eq!("Nothing", format!("{}", None::<u8>.fmt_or("Nothing")));
    /// ```
    #[must_use]
    fn fmt_or<U: Display>(&self, u: U) -> OptionFmtOr<T, U>;

    /// Format some value, or run an alternative closure if it's `None`.
    ///
    /// The value returned from the closure must implement [`Display`]
    /// regardless of which formatting is used originally.
    ///
    /// The value returned from the closure is not stored after use.
    /// Therefore, using a single [`OptionFmtOrElse`] object for multiple
    /// formatting operations will run the closure multiple times.
    ///
    /// # Examples
    /// ```
    /// # use devela::error::ExtOption;
    /// assert_eq!("42", format!("{}", Some(42).fmt_or_else(|| "Nothing")));
    /// assert_eq!("Nothing", format!("{}", None::<u8>.fmt_or_else(|| "Nothing")));
    /// ```
    #[must_use]
    fn fmt_or_else<U: Display, F: Fn() -> U>(&self, f: F) -> OptionFmtOrElse<T, F>;
}

impl<T> ExtOption<T> for Option<T> {
    fn contains<U: PartialEq<T>>(&self, x: &U) -> bool {
        self.as_ref().map_or(false, |y| x == y)
    }

    fn reduce<F: FnOnce(T, T) -> T>(self, other: Option<T>, f: F) -> Option<T> {
        match (self, other) {
            (Some(l), Some(r)) => Some(f(l, r)),
            (x @ Some(_), None) | (None, x @ Some(_)) => x,
            (None, None) => None,
        }
    }

    fn fmt_or_empty(&self) -> OptionFmt<T> {
        OptionFmt(self)
    }

    fn fmt_or<U: Display>(&self, u: U) -> OptionFmtOr<T, U> {
        OptionFmtOr(self, u)
    }

    fn fmt_or_else<U: Display, F: Fn() -> U>(&self, f: F) -> OptionFmtOrElse<T, F> {
        OptionFmtOrElse(self, f)
    }
}
