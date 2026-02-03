// devela::sys::mem::alloc::storage
//
//! The [`Storage`] trait allows data structures to abstract over how data is stored,
//! enabling specialization by storage strategy (e.g. stack vs heap).
//!
//! It is already implemented for the [`Bare`] and [`Boxed`] type markers,
//! which wraps their data in a [`BareBox`] and a [`Box`], respectively.
//

#[cfg(all(doc, feature = "alloc"))]
use crate::Box;
use crate::DerefMut;

mod bare;
#[cfg(feature = "alloc")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    mod boxed;
    pub use boxed::*;
}
pub use bare::*;

#[doc = crate::_tags!(mem)]
/// Allows data structures to be generic over their storage strategy.
#[doc = crate::_doc_location!("sys/mem")]
///
/// There are two reference implementations:
/// - [`Bare`], storing data inline via [`BareBox`].
/// - [`Boxed`], storing data on the heap via [`Box`].
///
/// # Examples
/// ```
/// use core::array::from_fn;
/// use devela::Storage;
///
/// /// Generically store a generic array of generic size.
/// pub struct MyStructure<T, S: Storage, const L: usize> {
///     data: S::Stored<[T; L]>,
/// }
///
/// impl<T: Default, S: Storage, const L: usize> MyStructure<T, S, L> {
///     pub fn new() -> Self {
///         Self {
///             data: S::Stored::from(from_fn(|_| T::default())),
///         }
///     }
/// }
///
/// // The array is stored inline (stack-allocated).
/// assert_eq![100, size_of::<MyStructure::<u8, (), 100>>()];
///
/// // The array is stored in the heap.
/// #[cfg(feature = "alloc")]
/// assert_eq![8, size_of::<MyStructure::<u8, devela::Boxed, 100>>()];
/// ```
pub trait Storage {
    /// The stored associated type.
    ///
    /// Any stored type must support mutable dereferencing and construction from `T`.
    type Stored<T>: DerefMut<Target = T> + From<T>;

    /// Returns the static name of the storage implementation.
    ///
    /// This can be useful for debugging.
    fn name() -> &'static str;

    // WAIT: [box_into_inner](https://github.com/rust-lang/rust/issues/80437)
    // fn unstore(self) -> T;
}
