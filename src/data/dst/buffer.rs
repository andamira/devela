// devela::data::dst

use crate::_deps::bytemuck::Pod;
use crate::data::Array;
use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

/// Represents the backing buffer for storing dynamically sized types.
///
/// # Safety
/// Must conform to the following rules:
/// - The `as_ref`/`as_mut` methods must return pointers to the same data.
/// - The pointer returned by `as_mut` must be stable until either a call to
///   `extend` or the value is moved (i.e. `let a = foo.as_mut().as_ptr();
///   let b = foo.as_mut().as_ptr(); assert!(a == b)` always holds).
/// - `extend` must not change any contained data
///   (but may extend with unspecified values).
pub unsafe trait DstBuf {
    /// Inner type of the buffer
    type Inner: Pod;

    /// Get the buffer slice as shared reference.
    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>];

    /// Get the buffer slice as an exclusive reference.
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>];

    /// Extend the buffer (fallible).
    fn extend(&mut self, len: usize) -> Result<(), ()>;

    /// Convert a byte count to a word count (rounding up).
    fn round_to_words(bytes: usize) -> usize {
        super::round_to_words::<Self::Inner>(bytes)
    }
}

// impl for an exclusive reference
#[rustfmt::skip]
unsafe impl<T, U> DstBuf for &mut T where U: Pod, T: DstBuf<Inner = U> {
    type Inner = T::Inner;

    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>] {
        (**self).as_ref()
    }
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>] {
        (**self).as_mut()
    }
    fn extend(&mut self, len: usize) -> Result<(), ()> {
        (**self).extend(len)
    }
}

// impl for array
unsafe impl<T: Pod, const N: usize> DstBuf for [MaybeUninit<T>; N] {
    type Inner = T;

    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>] {
        self
    }
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>] {
        self
    }
    fn extend(&mut self, len: usize) -> Result<(), ()> {
        if len > N {
            Err(())
        } else {
            Ok(())
        }
    }
}

/// Vector backed structures, can be used to auto-grow the allocation
///
/// # Examples
/// ```
/// # use {devela::data::DstQueue, core::mem::MaybeUninit};
/// let mut buf = DstQueue::<str, Vec<MaybeUninit<u8>>>::new();
/// buf.push_back_str("Hello world!");
/// buf.push_back_str("This is a very long string");
/// buf.push_back_str("The buffer should keep growing as it needs to");
/// for line in buf.iter() {
///   println!("{}", line);
/// }
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
unsafe impl<T: Pod> DstBuf for crate::_alloc::vec::Vec<MaybeUninit<T>> {
    type Inner = T;
    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>] {
        self
    }
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>] {
        self
    }
    fn extend(&mut self, len: usize) -> Result<(), ()> {
        if len > self.len() {
            self.resize(len, MaybeUninit::uninit());
            let cap = self.capacity();
            self.resize(cap, MaybeUninit::uninit());
        }
        Ok(())
    }
}

/// A static array for storing <abbr title="Dynamically sized
/// type">DST</abbr>s.
pub struct DstArray<T, const N: usize> {
    inner: Array<MaybeUninit<T>, N>,
}
impl<T, const N: usize> Deref for DstArray<T, N> {
    type Target = Array<MaybeUninit<T>, N>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T, const N: usize> DerefMut for DstArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
#[rustfmt::skip]
impl<T: Pod, const N: usize> Default for DstArray<T, N> {
    fn default() -> Self { Self { inner: Array::new([MaybeUninit::uninit(); N]) } }
}
#[rustfmt::skip]
unsafe impl<T: Pod, const N: usize> DstBuf for DstArray<T, N> {
    type Inner = T;
    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>] {
        &self.inner
    }
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>] {
        &mut self.inner
    }
    fn extend(&mut self, len: usize) -> Result<(), ()> {
        if len > N { Err(()) } else { Ok(()) }
    }
}

/// A statically allocated buffer for storing <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
pub type DstArrayUsize<const N: usize> = DstArray<usize, N>;

/// A dynamically allocated buffer for storing <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub type DstVecUsize = crate::_alloc::vec::Vec<MaybeUninit<usize>>;

// MAYBE
// /// A DST buffer backing onto a Vec.
// #[cfg(feature = "alloc")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
// pub struct DstVec<T: Pod>(crate::_alloc::vec::Vec<MaybeUninit<T>>);
// impl<T: Pod> Deref for DstVec<T> {
//     type Target = Vec<MaybeUninit<T>>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<T: Pod> DerefMut for DstVec<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
