// devela::data::dst

use crate::{Array, ConstInit, Deref, DerefMut, MaybeUninit, MemPod};

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// Represents the backing buffer for storing dynamically sized types.
#[doc = crate::_doc!(location: "data/dst")]
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
    type Inner: MemPod;

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
unsafe impl<T, U> DstBuf for &mut T where U: MemPod, T: DstBuf<Inner = U> {
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
unsafe impl<T: MemPod, const CAP: usize> DstBuf for [MaybeUninit<T>; CAP] {
    type Inner = T;

    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>] {
        self
    }
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>] {
        self
    }
    fn extend(&mut self, len: usize) -> Result<(), ()> {
        if len > CAP { Err(()) } else { Ok(()) }
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
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
unsafe impl<T: MemPod> DstBuf for crate::Vec<MaybeUninit<T>> {
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

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A static array for storing <abbr title="Dynamically sized type">DST</abbr>s.
#[doc = crate::_doc!(location: "data/dst")]
#[derive(Debug)]
pub struct DstArray<T, const CAP: usize> {
    inner: Array<MaybeUninit<T>, CAP>,
}
impl<T, const CAP: usize> Deref for DstArray<T, CAP> {
    type Target = Array<MaybeUninit<T>, CAP>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T, const CAP: usize> DerefMut for DstArray<T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<T: MemPod, const CAP: usize> Default for DstArray<T, CAP> {
    fn default() -> Self {
        Self { inner: Array::new([MaybeUninit::uninit(); CAP]) }
    }
}
impl<T: MemPod, const CAP: usize> ConstInit for DstArray<T, CAP> {
    const INIT: Self = Self {
        inner: Array::new_bare([MaybeUninit::uninit(); CAP]),
    };
}
#[rustfmt::skip]
unsafe impl<T: MemPod, const CAP: usize> DstBuf for DstArray<T, CAP> {
    type Inner = T;
    fn as_ref(&self) -> &[MaybeUninit<Self::Inner>] {
        &self.inner
    }
    fn as_mut(&mut self) -> &mut [MaybeUninit<Self::Inner>] {
        &mut self.inner
    }
    fn extend(&mut self, len: usize) -> Result<(), ()> {
        if len > CAP { Err(()) } else { Ok(()) }
    }
}

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A statically allocated buffer for storing <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
#[doc = crate::_doc!(location: "data/dst")]
pub type DstArrayUsize<const CAP: usize> = DstArray<usize, CAP>;

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A dynamically allocated buffer for storing <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
#[doc = crate::_doc!(location: "data/dst")]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub type DstVecUsize = crate::Vec<MaybeUninit<usize>>;

// MAYBE
// /// A DST buffer backing onto a Vec.
// #[cfg(feature = "alloc")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
// pub struct DstVec<T: MemPod>(crate::Vec<MaybeUninit<T>>);
// impl<T: MemPod> Deref for DstVec<T> {
//     type Target = Vec<MaybeUninit<T>>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<T: MemPod> DerefMut for DstVec<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
