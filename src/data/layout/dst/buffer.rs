// devela/src/data/layout/dst/buffer.rs

use crate::{ConstInit, Deref, DerefMut, MaybeUninit, MemPod};

#[doc = crate::_tags!(data_structure)]
/// Represents the backing buffer for storing dynamically sized types.
#[doc = crate::_doc_meta!{location("data/layout/dst")}]
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

#[doc = crate::_tags!(data_structure)]
/// An inline fixed-capacity buffer for storing
/// <abbr title="Dynamically sized type">DST</abbr>s.
#[doc = crate::_doc_meta!{location("data/layout/dst")}]
///
/// `CAP` is measured in `T`-sized storage words.
/// The size and alignment of `T` determine which values can be stored.
#[repr(transparent)]
#[derive(Debug)]
pub struct DstArray<T, const CAP: usize> {
    inner: [MaybeUninit<T>; CAP],
}
impl<T, const CAP: usize> DstArray<T, CAP> {
    /// The number of `T`-sized storage words.
    pub const CAPACITY: usize = CAP;

    /// Creates an uninitialized fixed-capacity DST buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: [const { MaybeUninit::uninit() }; CAP] }
    }
    /// Returns the backing native array.
    #[must_use]
    pub const fn as_array(&self) -> &[MaybeUninit<T>; CAP] {
        &self.inner
    }
    /// Returns the backing native array exclusively.
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [MaybeUninit<T>; CAP] {
        &mut self.inner
    }
    /// Returns the backing storage as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[MaybeUninit<T>] {
        &self.inner
    }
    /// Returns the backing storage as an exclusive slice.
    #[must_use]
    pub const fn as_mut_slice(&mut self) -> &mut [MaybeUninit<T>] {
        &mut self.inner
    }
    /// Consumes the buffer and returns its backing native array.
    #[must_use]
    pub fn into_array(self) -> [MaybeUninit<T>; CAP] {
        self.inner
    }
}
impl<T, const CAP: usize> Default for DstArray<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, const CAP: usize> ConstInit for DstArray<T, CAP> {
    const INIT: Self = Self::new();
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

#[doc = crate::_tags!(data_structure)]
/// A statically allocated buffer for storing <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
#[doc = crate::_doc_meta!{location("data/layout/dst")}]
pub type DstArrayUsize<const CAP: usize> = DstArray<usize, CAP>;

#[doc = crate::_tags!(data_structure)]
/// A dynamically allocated buffer for storing <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
#[doc = crate::_doc_meta!{location("data/layout/dst")}]
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
