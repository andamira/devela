// devela::mem::trait
//
//! Functionality related to byte sizes.
//

use super::{mem_needs_drop, mem_swap, *};

impl<T: ?Sized> Mem for T {}

/// A trait for type memory information and manipulation.
///
/// This trait is automatically implemented for every `?Sized` type,
/// although most methods are only available where `Self: Sized`.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub trait Mem {
    /// Whether dropping values of this type matters.
    const NEEDS_DROP: bool = mem_needs_drop::<Self>();

    /// Returns `true` if dropping values of this type matters.
    fn mem_needs_drop(&self) -> bool {
        Self::NEEDS_DROP
    }

    /// Drops `self` by running its destructor.
    fn mem_drop(self)
    where
        Self: Sized,
    {
        mem_drop(self)
    }

    /// Forgets about `self` *without running its destructor*.
    fn mem_forget(self)
    where
        Self: Sized,
    {
        mem_forget(self)
    }

    /// Replaces `self` with other, returning the previous value of `self`.
    fn mem_replace(&mut self, other: Self) -> Self
    where
        Self: Sized,
    {
        mem_replace(self, other)
    }

    /// Replaces `self` with its default value, returning the previous value of `self`.
    fn mem_take(&mut self) -> Self
    where
        Self: Default,
    {
        mem_take(self)
    }

    /// Swaps the value of `self` and `other` without deinitializing either one.
    fn mem_swap(&mut self, other: &mut Self)
    where
        Self: Sized,
    {
        mem_swap(self, other);
    }

    /// View a `Sync + Unpin` `self` as `&[u8]`.
    ///
    /// For the `const` version for sized types see [`mem_as_bytes_sized`].
    #[cfg(feature = "unsafe_mem")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_mem")))]
    fn mem_as_bytes(&self) -> &[u8]
    where
        Self: Sync + Unpin,
    {
        mem_as_bytes(self)
    }

    /// View a `Sync + Unpin` `self` as `&mut [u8]`.
    #[cfg(feature = "unsafe_mem")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_mem")))]
    fn mem_as_bytes_mut(&mut self) -> &mut [u8]
    where
        Self: Sync + Unpin,
    {
        mem_as_bytes_mut(self)
    }
}
