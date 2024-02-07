// devela::mem::storage
//
//! The [`Storage`] trait allows the data structure implementations to have
//! specialized methods by storage type (specially useful for constructors).
//!
//! It is already implemented for the [`Bare`] and [`Boxed`] type markers,
//! which wraps their data in a [`BareBox`] and a [`Box`], respectively.
//

#[cfg(all(doc, feature = "alloc"))]
use crate::_deps::alloc::boxed::Box;
use core::ops::DerefMut;

mod bare;
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod boxed;

pub use bare::*;
#[cfg(feature = "alloc")]
pub use boxed::*;

/// Allows to be generic in respect of the data storage.
///
/// There are two reference implementations:
/// - [`Bare`][super::Bare], which wraps the data in a [`BareBox`].
/// - [`Boxed`], which wraps the data in a [`Box`].
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
    type Stored<T>: DerefMut<Target = T> + From<T>;

    /// Returns the static name of the storage implementation.
    ///
    /// This can be useful for debugging.
    fn name() -> &'static str;

    // WAIT: [box_into_inner](https://github.com/rust-lang/rust/issues/80437)
    // fn unstore(self) -> T;
}
