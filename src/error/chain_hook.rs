// devela::error::traits
//
//! Free function chaining traits helpers.
//
// Based on the code by George Burton, Unlicense licensed.
// https://crates.io/crates/apply/0.3.0

/// Allows chaining transformations by passing values through a sequence of functions.
///
/// Use `Chain` to thread operations where each function takes ownership or
/// references the value and returns a new result.
///
/// # Examples
/// ```
/// use devela::Chain;
///
/// let x = 1
///     .chain(|x| x * 2)
///     .chain(|x| x + 1)
///     .chain(|x: i32| x.to_string());
/// assert_eq![x, 3.to_string()];
/// ```
///
/// ```compile_fail
/// use devela::Chain;
///
/// // We can sort it, but we don't receive the new vec.
/// let v: Vec<i32> = vec![3, 2, 1, 5].chain_mut(|it| it.sort());
/// ```
///
/// See also the [`Hook`][crate::Hook] trait.
pub trait Chain<R> {
    /// Chain a function which takes the parameter by value.
    #[must_use]
    fn chain<F>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
        Self: Sized,
    {
        f(self)
    }

    /// Chain a function which takes the parameter by shared reference.
    #[must_use]
    fn chain_ref<F>(&self, f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        f(self)
    }

    /// Chain a function which takes the parameter by exclusive reference.
    #[must_use]
    fn chain_mut<F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        f(self)
    }
}
impl<T: ?Sized, R> Chain<R> for T {}

/// Allows attaching operations or side effects to a value without breaking its flow.
///
/// Use `Hook` when you want to inspect, log, or modify the value in-place without
/// directly returning a new result from the function.
///
/// Useful for when a method doesn't return the receiver
/// but you want to apply several of them to the object.
///
/// It assumes that each function in the chain modifies the value by exclusive
/// reference and returns the modified value.
///
/// # Examples
/// ```
/// use devela::Hook;
///
/// let v = vec![3, 2, 1, 5]
///     .hook_mut(|v| v.sort())
///     .hook_ref(|v| assert_eq![v, &[1, 2, 3, 5]])
///     .hook_mut(|v| v.push(7));
/// assert_eq![v, vec![1, 2, 3, 5, 7]];
/// ```
///
/// See also the [`Chain`][crate::Chain] trait.
pub trait Hook: Sized {
    /// Applies a function which takes the parameter by shared reference,
    /// and then returns the (possibly) modified owned value.
    ///
    /// Similar to [`chain_ref`], but instead of returning self directly from `f`,
    /// since it has a different signature, returns it indirectly.
    ///
    /// Similar to [`chain_ref`], but the function is used for side effects
    /// and does not directly return a transformed value.
    ///
    /// [`chain_ref`]: Chain::chain_ref
    #[must_use]
    fn hook_ref<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self),
    {
        f(&self);
        self
    }

    /// Applies a function which takes the parameter by exclusive reference,
    /// and then returns the (possibly) modified owned value.
    ///
    /// Similar to [`chain_mut`], but instead of returning self directly from `f`,
    /// since it has a different signature, returns it indirectly.
    ///
    /// [`chain_mut`]: Chain::chain_mut
    #[must_use]
    fn hook_mut<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}
impl<T: Sized> Hook for T {}
