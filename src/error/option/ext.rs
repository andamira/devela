// devela::error::option::ext
//
//!
//

use super::{OptionFmt, OptionFmtOr, OptionFmtOrElse};
use core::fmt::Display;

// Marker trait to prevent downstream implementations of the `OptionExt` trait.
mod private {
    pub trait Sealed {}
}
impl<T> private::Sealed for Option<T> {}

/// Extension trait providing additional methods for [`Option`].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`ResultExt`][crate::error::ResultExt].
//
// Based on work from:
// - https://github.com/rust-lang/rust/issues/62358 (closed proposal).
// - https://crates.io/crates/option-ext/0.2.0 by Simon Ochsenreither.
// - https://crates.io/crates/opt_reduce/1.0.0 by Waffle Lapkin.
pub trait OptionExt<T>: private::Sealed {
    /// Returns `true` if the option is a [`Some`] value containing the given value.
    ///
    /// # Examples
    /// ```
    /// use devela::error::OptionExt;
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
    /// use devela::error::OptionExt;
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
    #[must_use]
    fn reduce<F>(self, other: Option<T>, f: F) -> Option<T>
    where
        F: FnOnce(T, T) -> T;

    /// Format some value, or display an empty string if it's `None`.
    ///
    /// # Examples
    /// ```
    /// use devela::error::OptionExt;
    ///
    /// let foo: Option<u32> = Some(0x42);
    /// let bar: Option<u32> = None;
    ///
    /// assert_eq!("0x42", format!("{:#x}", foo.fmt_or_empty()));
    /// assert_eq!("", format!("{:#x}", bar.fmt_or_empty()));
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
    /// use devela::error::OptionExt;
    ///
    /// let foo: Option<Box<u32>> = Some(Box::new(42));
    /// let bar: Option<Box<u32>> = None;
    ///
    /// assert_eq!("42", format!("{}", foo.fmt_or("Nothing")));
    /// assert_eq!("Nothing", format!("{}", bar.fmt_or("Nothing")));
    /// ```
    #[must_use]
    fn fmt_or<U>(&self, u: U) -> OptionFmtOr<T, U>
    where
        U: Display;

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
    /// use devela::error::OptionExt;
    ///
    /// let foo: Option<u32> = Some(42);
    /// let bar: Option<u32> = None;
    ///
    /// assert_eq!("42", format!("{}", foo.fmt_or_else(|| "Nothing")));
    /// assert_eq!("Nothing", format!("{}", bar.fmt_or_else(|| "Nothing")));
    /// ```
    #[must_use]
    fn fmt_or_else<U, F>(&self, f: F) -> OptionFmtOrElse<T, F>
    where
        U: Display,
        F: Fn() -> U;
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

    #[inline]
    fn fmt_or_empty(&self) -> OptionFmt<T> {
        OptionFmt(self)
    }

    #[inline]
    fn fmt_or<U>(&self, u: U) -> OptionFmtOr<T, U>
    where
        U: Display,
    {
        OptionFmtOr(self, u)
    }

    #[inline]
    fn fmt_or_else<U, F>(&self, f: F) -> OptionFmtOrElse<T, F>
    where
        U: Display,
        F: Fn() -> U,
    {
        OptionFmtOrElse(self, f)
    }
}
