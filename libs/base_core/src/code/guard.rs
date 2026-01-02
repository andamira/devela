// devela_base_core::code::guard
//
//! Defines [`ScopeGuard`].
//

use crate::{Deref, DerefMut};

#[doc = crate::_TAG_GUARD!()]
/// A general-purpose RAII guard that executes a callback on drop.
#[doc = crate::_doc!(location: "code")]
///
/// - The callback can take both a value and a state.
/// - The state can be updated dynamically during the guard's lifetime.
/// - The guard can be dismissed, preventing the callback from executing on drop.
///
/// # Features
/// Uses `unsafe_layout` to avoid redundant unwrapping checks.
#[doc = crate::_doc!(vendor: "stated-scope-guard")]
#[derive(Debug)]
pub struct ScopeGuard<T, F: FnOnce(T, &S), S> {
    /// The guarded value,
    /// wrapped in an Option to allow taking ownership during drop.
    value: Option<T>,
    /// A callback to process the value and state on drop,
    /// wrapped in an Option for safe ownership transfer.
    callback: Option<F>,
    /// The associated state passed to the callback.
    state: S,
}

impl<T> ScopeGuard<T, fn(T, &bool), bool> {
    /// Constructs a scope guard with a boolean state (defaulting to true).
    ///
    /// The callback is executed on drop unless dismissed.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cell, ScopeGuard};
    /// let result = Cell::new(0);
    /// {
    ///     let _guard = ScopeGuard::new(10, |value| {
    ///         result.set(value + 5);
    ///     });
    /// }
    /// assert_eq!(result.get(), 15);
    /// ```
    pub fn new<F: FnOnce(T)>(value: T, callback: F) -> ScopeGuard<T, impl FnOnce(T, &bool), bool> {
        ScopeGuard::with(value, true, move |value, state: &bool| {
            if *state {
                callback(value);
            }
        })
    }
}
impl<T, F: FnOnce(T, &bool)> ScopeGuard<T, F, bool> {
    /// Dismisses the callback for a boolean state guard.
    ///
    /// Once dismissed, the callback won’t be executed on drop.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cell, ScopeGuard};
    /// let result = Cell::new(0);
    /// {
    ///     let mut guard = ScopeGuard::new(10, |value| {
    ///         result.set(value + 5);
    ///     });
    ///     guard.dismiss();
    /// }
    /// assert_eq!(result.get(), 0);
    /// ```
    pub fn dismiss(&mut self) {
        self.set_state(false);
    }
}
impl<T, F: FnOnce(T, &S), S> ScopeGuard<T, F, S> {
    /// Creates a scope guard with a custom state.
    ///
    /// The guarded value is accessible via `Deref` and `DerefMut`.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cell, ScopeGuard};
    /// // A simple resource that requires cleanup.
    /// struct Resource;
    /// impl Resource {
    ///     fn new() -> Self { Resource }
    ///     /// Cleans up the resource using the given strategy, updating the flag accordingly.
    ///     fn cleanup(&self, strategy: &Cleanup, flag: &Cell<&'static str>) {
    ///         match strategy {
    ///             Cleanup::Standard => flag.set("standard cleanup"),
    ///             Cleanup::Alternate => flag.set("alternate cleanup"),
    ///         }
    ///     }
    /// }
    /// // Define different cleanup strategies.
    /// enum Cleanup {
    ///     Standard,
    ///     Alternate,
    /// }
    /// let cleanup_flag = Cell::new("not cleaned");
    /// {
    ///     let mut guard = ScopeGuard::with(
    ///         Resource::new(),
    ///         Cleanup::Standard,
    ///         |res, strategy| { res.cleanup(strategy, &cleanup_flag) }
    ///     );
    ///     // Perform operations that require changing the cleanup strategy.
    ///     guard.set_state(Cleanup::Alternate);
    /// } // When the guard goes out of scope, it triggers the cleanup callback.
    /// assert_eq!(cleanup_flag.get(), "alternate cleanup");
    /// ```
    pub fn with(value: T, state: S, callback: F) -> Self {
        Self {
            value: Some(value),
            state,
            callback: Some(callback),
        }
    }
    /// Updates the current state.
    pub fn set_state(&mut self, state: S) {
        self.state = state;
    }
}
#[rustfmt::skip]
impl<T, F: FnOnce(T, &S), S> Deref for ScopeGuard<T, F, S> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        #[cfg(any(feature = "safe_code", not(feature = "unsafe_layout")))]
        { self.value.as_ref().unwrap() }
        #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
        // SAFETY: `value` is always `Some` until dropped
        unsafe { self.value.as_ref().unwrap_unchecked() }
    }
}
#[rustfmt::skip]
impl<T, F: FnOnce(T, &S), S> DerefMut for ScopeGuard<T, F, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(any(feature = "safe_code", not(feature = "unsafe_layout")))]
        { self.value.as_mut().unwrap() }
        #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
        // SAFETY: `value` is always `Some` until dropped
        unsafe { self.value.as_mut().unwrap_unchecked() }
    }
}
impl<T, F: FnOnce(T, &S), S> Drop for ScopeGuard<T, F, S> {
    /// On drop, invokes the callback with the guarded value and a reference to the current state.
    fn drop(&mut self) {
        let (value, callback) = {
            #[cfg(any(feature = "safe_code", not(feature = "unsafe_layout")))]
            {
                let value = self.value.take().unwrap();
                let callback = self.callback.take().unwrap();
                (value, callback)
            }
            #[cfg(all(not(feature = "safe_code"), feature = "unsafe_layout"))]
            {
                // SAFETY: `value` is always `Some` until dropped
                let value = unsafe { self.value.take().unwrap_unchecked() };
                // SAFETY: `callback` is always `Some` until dropped
                let callback = unsafe { self.callback.take().unwrap_unchecked() };
                (value, callback)
            }
        };
        callback(value, &self.state);
    }
}
