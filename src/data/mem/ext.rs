// devela::data::mem::trait
//
//! Functionality related to byte sizes.
//

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_slice"))]
use super::{mem_as_bytes, mem_as_bytes_mut};
use super::{mem_drop, mem_forget, mem_needs_drop, mem_replace, mem_swap, mem_take};

impl<T: ?Sized> ExtMem for T {}

/// A trait for type memory information and manipulation.
///
/// This trait is automatically implemented for every `?Sized` type,
/// although most methods are only available where `Self: Sized`.
#[rustfmt::skip]
pub trait ExtMem {
    /// Know whether dropping values of this type matters, in compile-time.
    const NEEDS_DROP: bool = mem_needs_drop::<Self>();

    /// Returns `true` if dropping values of this type matters.
    ///
    /// See [`mem_needs_drop`].
    fn mem_needs_drop(&self) -> bool { Self::NEEDS_DROP }

    /// Drops `self` by running its destructor.
    ///
    /// See [`mem_drop`].
    fn mem_drop(self) where Self: Sized { mem_drop(self) }

    /// Forgets about `self` *without running its destructor*.
    ///
    /// See [`mem_forget`].
    fn mem_forget(self) where Self: Sized { mem_forget(self) }

    /// Replaces `self` with other, returning the previous value of `self`.
    ///
    /// See [`mem_replace`].
    fn mem_replace(&mut self, other: Self) -> Self where Self: Sized { mem_replace(self, other) }

    /// Replaces `self` with its default value, returning the previous value of `self`.
    ///
    /// See [`mem_take`].
    fn mem_take(&mut self) -> Self where Self: Default, { mem_take(self) }

    /// Swaps the value of `self` and `other` without deinitializing either one.
    ///
    /// See [`mem_swap`].
    fn mem_swap(&mut self, other: &mut Self) where Self: Sized { mem_swap(self, other); }

    /// View a `Sync + Unpin` `self` as `&[u8]`.
    ///
    /// For the `const` version for sized types see [`mem_as_bytes_sized`].
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    fn mem_as_bytes(&self) -> &[u8] where Self: Sync + Unpin { mem_as_bytes(self) }

    /// View a `Sync + Unpin` `self` as `&mut [u8]`.
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    fn mem_as_bytes_mut(&mut self) -> &mut [u8] where Self: Sync + Unpin { mem_as_bytes_mut(self) }
}
