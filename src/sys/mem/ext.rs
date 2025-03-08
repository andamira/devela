// devela::sys::mem::ext
//
//! Defines [`ExtMem`], An extension trait for memory management over `T`.
//

use super::Mem;

impl<T: ?Sized> ExtMem for T {}

#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait for type memory information and manipulation.
///
/// This trait is automatically implemented for every `?Sized` type,
/// although most methods are only available where `Self: Sized`.
#[rustfmt::skip]
pub trait ExtMem {
    /// Know whether dropping values of this type matters, in compile-time.
    const NEEDS_DROP: bool = Mem::needs_drop::<Self>();

    /// Returns the minimum alignment of the type in bytes.
    ///
    /// See [`Mem::align_of`].
    #[must_use]
    fn mem_align_of<T>() -> usize { Mem::align_of::<T>() }

    /// Returns the alignment of the pointed-to value in bytes.
    ///
    /// See [`Mem::align_of_val`].
    #[must_use]
    fn mem_align_of_val(&self) -> usize { Mem::align_of_val(self) }

    /// Returns the size of a type in bytes.
    ///
    /// See [`Mem::size_of`].
    #[must_use]
    fn mem_size_of<T>() -> usize { Mem::size_of::<T>() }

    /// Returns the size of the pointed-to value in bytes.
    ///
    /// See [`Mem::size_of_val`].
    #[must_use]
    fn mem_size_of_val(&self) -> usize { Mem::size_of_val(self) }

    /// Bitwise-copies a value.
    ///
    /// It is useful when you want to pass a function pointer to a combinator,
    /// rather than defining a new closure.
    ///
    /// See [`Mem::copy`].
    #[must_use]
    fn mem_copy(&self) -> Self where Self: Copy { Mem::copy(self) }

    /// Returns `true` if dropping values of this type matters.
    ///
    /// See [`Mem::needs_drop`].
    #[must_use]
    fn mem_needs_drop(&self) -> bool { Self::NEEDS_DROP }

    /// Drops `self` by running its destructor.
    ///
    /// See [`Mem::drop`].
    fn mem_drop(self) where Self: Sized { Mem::drop(self) }

    /// Forgets about `self` *without running its destructor*.
    ///
    /// See [`Mem::forget`].
    fn mem_forget(self) where Self: Sized { Mem::forget(self) }

    /// Replaces `self` with other, returning the previous value of `self`.
    ///
    /// See [`Mem::replace`].
    #[must_use]
    fn mem_replace(&mut self, other: Self) -> Self where Self: Sized { Mem::replace(self, other) }

    /// Replaces `self` with its default value, returning the previous value of `self`.
    ///
    /// See [`Mem::take`].
    #[must_use]
    fn mem_take(&mut self) -> Self where Self: Default, { Mem::take(self) }

    /// Swaps the value of `self` and `other` without deinitializing either one.
    ///
    /// See [`Mem::swap`].
    fn mem_swap(&mut self, other: &mut Self) where Self: Sized { Mem::swap(self, other); }

    /// Returns the value of type `T` represented by the all-zero byte-pattern.
    ///
    /// # Safety
    /// See [`Mem::zeroed`].
    #[must_use]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    unsafe fn mem_zeroed<T>() -> T { unsafe { Mem::zeroed::<T>() } }

    /// Returns the value of type `T` represented by the all-zero byte-pattern.
    ///
    /// # Safety
    /// See [`Mem::transmute_copy`].
    #[must_use]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    unsafe fn mem_transmute_copy<Src, Dst>(src: &Src) -> Dst {
        unsafe { Mem::transmute_copy::<Src, Dst>(src) }
    }

    /// View a `Sync + Unpin` `self` as `&[u8]`.
    ///
    /// See [`Mem::as_bytes`], and for the `const` version for sized types
    /// see [`Mem::as_bytes_sized`].
    #[must_use]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    fn mem_as_bytes(&self) -> &[u8] where Self: Sync + Unpin { Mem::as_bytes(self) }

    /// View a `Sync + Unpin` `self` as `&mut [u8]`.
    ///
    /// See [`Mem::as_bytes_mut`].
    #[must_use]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    fn mem_as_bytes_mut(&mut self) -> &mut [u8] where Self: Sync + Unpin { Mem::as_bytes_mut(self) }
}
