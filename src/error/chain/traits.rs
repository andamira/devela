// devela::traits::chain
//
//! Free function chaining traits helpers.
//

/// Allows to chain free functions into method call chains.
///
/// # Examples
/// ```
/// use devela::error::Apply;
///
/// let s = 1
///     .apply(|s| s * 2)
///     .apply(|s| s + 1)
///     .apply(|s: i32| s.to_string());
/// assert_eq![s, 3.to_string()];
/// ```
///
/// ```compile_fail
/// use devela::error::Apply;
///
/// // We can sort it, but we don't receive the new vec.
/// let v: Vec<i32> = vec![3, 2, 1, 5].apply_mut(|it| it.sort());
/// ```
// Based on the code by George Burton, Unlicense licensed.
// https://crates.io/crates/apply/0.3.0
pub trait Apply<Res> {
    /// Apply a function which takes the parameter by value.
    #[inline]
    #[must_use]
    fn apply<F: FnOnce(Self) -> Res>(self, f: F) -> Res
    where
        Self: Sized,
    {
        f(self)
    }

    /// Apply a function which takes the parameter by shared reference.
    #[inline]
    #[must_use]
    fn apply_ref<F: FnOnce(&Self) -> Res>(&self, f: F) -> Res {
        f(self)
    }

    /// Apply a function which takes the parameter by exclusive reference.
    #[inline]
    #[must_use]
    fn apply_mut<F: FnOnce(&mut Self) -> Res>(&mut self, f: F) -> Res {
        f(self)
    }
}

impl<T: ?Sized, Res> Apply<Res> for T {}

/// Represents a type that you can apply arbitrary functions to.
///
/// Useful for when a method doesn't return the receiver
/// but you want to apply several of them to the object.
///
/// It assumes that each function in the chain modifies the value by exclusive
/// reference and returns the modified value.
///
/// # Examples
/// ```
/// use devela::error::Also;
///
/// let v = vec![3, 2, 1, 5]
///     .also_mut(|v| v.sort())
///     .also_ref(|v| assert_eq![v, &[1, 2, 3, 5]])
///     .also_mut(|v| v.push(7));
/// assert_eq![v, vec![1, 2, 3, 5, 7]];
/// ```
///
// Based on the code by George Burton, Unlicense licensed.
// https://crates.io/crates/apply/0.3.0
pub trait Also: Sized {
    /// Applies a function which takes the parameter by exclusive reference,
    /// and then returns the (possibly) modified owned value.
    ///
    /// Similar to [`apply`], but instead of returning self directly from `f`,
    /// since it has a different signature, returns it indirectly.
    ///
    /// [`apply`]: Apply::apply
    #[inline]
    #[must_use]
    fn also_mut<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }

    /// Applies a function which takes the parameter by shared reference,
    /// and then returns the (possibly) modified owned value.
    ///
    /// Similar to [`apply`], but instead of returning self directly from `f`,
    /// since it has a different signature, returns it indirectly.
    ///
    /// [`apply`]: Apply::apply
    #[inline]
    #[must_use]
    fn also_ref<F: FnOnce(&Self)>(self, f: F) -> Self {
        f(&self);
        self
    }
}
impl<T: Sized> Also for T {}
