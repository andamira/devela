// devela::mem::storage
//
//! The [`Storage`] trait allows the data structure implementations to have
//! specialized methods by storage type (specially useful for constructors).
//!
//! It is already implemented for the [`Boxed`] type and the [`()`][unit] unit
//! type, which wraps their data in a [`Box`] and a [`Direct`], respectively.
//

use core::ops;

mod direct;
pub use direct::*;

/// Allows to be generic in respect of the data storage.
///
/// There are two reference implementations:
/// - [`Boxed`], which wraps the data in a [`Box`].
/// - [`()`][unit], which wraps the data in a [`Direct`].
///
/// # Examples
/// ```
/// use core::{array, mem::size_of};
/// use devela::mem::Storage;
///
/// /// Generically store a generic array of generic size.
/// pub struct MyStructure<T, S: Storage, const L: usize> {
///     data: S::Stored<[T; L]>,
/// }
///
/// impl<T, S: Storage, const L: usize> MyStructure<T, S, L> {
///     pub fn new() -> Self
///     where
///         T: Default,
///     {
///         Self {
///             data: S::Stored::from(array::from_fn(|_| T::default())),
///         }
///     }
/// }
///
/// // The array is stored in the stack
/// assert_eq![100, size_of::<MyStructure::<u8, (), 100>>()];
///
/// // The array is stored in the heap.
/// #[cfg(feature = "alloc")]
/// assert_eq![8, size_of::<MyStructure::<u8, devela::mem::Boxed, 100>>()];
///
/// ```
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mem")))]
pub trait Storage {
    /// The stored associated type.
    ///
    /// Any type `T` that is to be stored must be able to be dereferenced to a
    /// mutable reference of `T` and to be constructed from a value of type `T`.
    type Stored<T>: ops::DerefMut<Target = T> + From<T>;

    /// Returns the static name of the storage implementation.
    ///
    /// This can be useful for debugging.
    fn name() -> &'static str;

    // WAIT: [box_into_inner](https://github.com/rust-lang/rust/issues/80437)
    // fn unstore(self) -> T;
}

/// A storage type that wraps its data in a [`Box`].
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mem")))]
pub struct Boxed;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl Storage for Boxed {
    type Stored<T> = crate::_deps::alloc::boxed::Box<T>;

    fn name() -> &'static str {
        "Boxed"
    }
}

/// A storage type that wraps its data in a [`Direct`].
impl Storage for () {
    type Stored<T> = Direct<T>;

    fn name() -> &'static str {
        "Direct"
    }
}
