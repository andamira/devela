// devela::sys::mem::cell::option
//
//! Defines [`CellOptionExt`].
//

use crate::Cell;

/// Marker trait to prevent downstream implementations of the [`ExtCell`] trait.
trait Sealed {}
impl<T> Sealed for Cell<Option<T>> {}

#[doc = crate::_TAG_NAMESPACE!()]
/// Extension trait providing additional methods for `Cell<Option>`.
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait CellOptionExt<T>: Sealed {
    /// Modifies the value inside the `Cell<Option<T>>` by applying the provided closure
    /// to a mutable reference of the current value if present.
    ///
    /// This method extracts the value, applies the function, and stores the result back.
    ///
    /// # Example
    /// ```
    /// use devela::{Cell, CellOptionExt};
    ///
    /// let cell = Cell::new(Some(10));
    /// cell.modify(|x| x + 5);
    /// assert_eq![cell.get(), Some(15)];
    /// ```
    // WAIT: [cell_update](https://github.com/rust-lang/rust/issues/50186)
    fn modify<F: FnOnce(T) -> T>(&self, func: F);

    /// Modifies the value inside the `Cell<Option<T>>` by applying the provided function
    /// and returns the old contained value.
    ///
    /// # Example
    /// ```
    /// use devela::{Cell, CellOptionExt};
    ///
    /// let cell = Cell::new(Some(10));
    /// let old = cell.modify_ret(|x| x + 5);
    /// assert_eq![old, Some(10)];
    /// assert_eq![cell.get(), Some(15)];
    /// ```
    fn modify_ret<F: FnOnce(T) -> T>(&self, func: F) -> Option<T>
    where
        T: Clone;

    /// Modifies the value inside the `Cell<Option<T>>` by applying the provided closure
    /// to a mutable reference of the current value if present, and returns a result.
    ///
    /// This method allows in-place modification via a mutable reference, returning any value.
    ///
    /// # Example
    /// ```
    /// use devela::{Cell, CellOptionExt};
    ///
    /// let cell = Cell::new(Some(10));
    /// let result = cell.modify_mut(|x| {
    ///     let old = *x;
    ///     *x *= 2;
    ///     old
    /// });
    ///
    /// assert_eq![result, Some(10)];
    /// assert_eq![cell.get(), Some(20)];
    /// ```
    fn modify_mut<R, F: FnOnce(&mut T) -> R>(&self, func: F) -> Option<R>;
}

impl<T> CellOptionExt<T> for Cell<Option<T>> {
    fn modify<F: FnOnce(T) -> T>(&self, func: F) {
        let mut value = self.take();
        // If the value exists, apply the function and store it back
        if let Some(v) = value.take() {
            self.set(Some(func(v)));
        }
    }

    fn modify_ret<F: FnOnce(T) -> T>(&self, func: F) -> Option<T>
    where
        T: Clone,
    {
        self.replace(None).inspect(|v| {
            let new_value = func(v.clone());
            self.set(Some(new_value));
        })
    }

    fn modify_mut<R, F: FnOnce(&mut T) -> R>(&self, func: F) -> Option<R> {
        if let Some(mut v) = self.take() {
            // Apply the function to a mutable reference and capture the return value
            let result = func(&mut v);
            // Store the possibly modified value back in the Cell
            self.set(Some(v));
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify() {
        let cell = Cell::new(Some(10));
        cell.modify(|x| x + 5);
        assert_eq!(cell.get(), Some(15));
    }

    #[test]
    fn test_modify_with_none() {
        let cell: Cell<Option<i32>> = Cell::new(None);
        cell.modify(|x| x + 5);
        assert_eq!(cell.get(), None);
    }

    #[test]
    fn test_modify_ret() {
        let cell = Cell::new(Some(10));

        let previous_value = cell.modify_ret(|x| x * 2);

        assert_eq!(previous_value, Some(10));
        assert_eq!(cell.get(), Some(20));
    }

    #[test]
    fn test_modify_ret_with_none() {
        let cell: Cell<Option<i32>> = Cell::new(None);

        let previous_value = cell.modify_ret(|x| x * 2);

        assert_eq!(previous_value, None);
        assert_eq!(cell.get(), None);
    }

    #[test]
    fn test_modify_mut_previous() {
        let cell = Cell::new(Some(10));

        let result = cell.modify_mut(|x| {
            let prev = *x;
            *x *= 2;
            prev
        });

        assert_eq!(result, Some(10)); // previous value
        assert_eq!(cell.get(), Some(20));
    }
    #[test]
    fn test_modify_mut() {
        let cell = Cell::new(Some(10));

        let result = cell.modify_mut(|x| {
            *x *= 2;
            *x + 5
        });

        assert_eq!(result, Some(25));
        assert_eq!(cell.get(), Some(20));
    }

    #[test]
    fn test_modify_mut_with_none() {
        let cell: Cell<Option<i32>> = Cell::new(None);

        let result = cell.modify_mut(|x| {
            *x *= 2;
            *x + 5
        });

        assert_eq!(result, None);
        assert_eq!(cell.get(), None);
    }

    #[test]
    fn test_modify_mut_side_effect() {
        let cell = Cell::new(Some(10));

        let result = cell.modify_mut(|x| {
            *x += 3;
            "Done"
        });

        assert_eq!(result, Some("Done"));
        assert_eq!(cell.get(), Some(13));
    }
}
