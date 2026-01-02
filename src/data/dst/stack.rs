// devela::data::dst::stack
//
//! Implementation of the LIFO stack structure.
//
// TOC
// - public API
// - private API
// - core_impls

use super::{DstArray, DstBuf, check_fat_pointer, decompose_pointer, list_push_gen};
use crate::{ConstInit, MaybeUninit, MemAligned, PhantomData, Ptr};

/* public API */

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A statically allocated LIFO stack of
/// <abbr title="Dynamically sized type">DST</abbr>s with pointer alignment.
#[doc = crate::_doc_location!("data/dst")]
///
/// # Examples
/// ```
/// # use devela::data::DstStackUsize;
/// let mut stack = DstStackUsize::<[u8], 16>::new();
/// stack.push_copied(&[1]);
/// ```
// WAIT: [lazy_type_alias](https://github.com/rust-lang/rust/issues/112792) ↓DENIED
pub type DstStackUsize<DST /*: ?Sized*/, const CAP: usize> = DstStack<DST, DstArray<usize, CAP>>;

// Implementation Notes
// -----
//
// The data array is filled from the back, with the metadata stored before
// (at a lower memory address) the actual data. This so the code can use a
// single integer to track the position (using size_of_val when popping items,
// and the known size when pushing).

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A statically allocated LIFO stack of <abbr title="Dynamically sized type">DST</abbr>s.
#[doc = crate::_doc_location!("data/dst")]
///
/// Note: Each item in the stack takes at least one slot in the buffer
/// (to store the metadata)
pub struct DstStack<DST: ?Sized, BUF: DstBuf> {
    _pd: PhantomData<*const DST>,
    // Offset from the _back_ of `data` to the next free position.
    // I.e. data[data.len() - cur_ofs] is the first metadata word
    next_ofs: usize,
    data: BUF,
}

#[doc = crate::_TAG_ITERATOR!()]
/// An iterator over the elements of a [`DstStack`].
#[doc = crate::_doc_location!("data/dst")]
#[derive(Debug)]
pub struct DstStackIter<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(&'a DstStack<DST, BUF>, usize);

#[doc = crate::_TAG_ITERATOR!()]
/// A mutable iterator over the elements of a [`DstStack`].
#[doc = crate::_doc_location!("data/dst")]
#[derive(Debug)]
pub struct DstStackIterMut<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(
    &'a mut DstStack<DST, BUF>,
    usize,
);

impl<DST: ?Sized, BUF: DstBuf> DstStack<DST, BUF> {
    /// Constructs a new (empty) stack.
    #[must_use] #[rustfmt::skip]
    pub fn new() -> Self where BUF: Default { Self::with_buffer(BUF::default()) }

    /// Constructs a new (empty) stack, in compile-time.
    #[must_use] #[rustfmt::skip]
    pub const fn new_const() -> Self where BUF: ConstInit { Self::with_buffer(BUF::INIT) }

    /// Constructs a new (empty) stack using the given `buffer`.
    #[must_use] #[rustfmt::skip]
    pub const fn with_buffer(buffer: BUF) -> Self {
        DstStack { _pd: PhantomData, next_ofs: 0, data: buffer }
    }

    /// Returns `true` if the stack is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.next_ofs == 0
    }

    /// Pushes a value at the top of the stack.
    ///
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<u64, 8>>::new();
    /// stack.push([1, 2,3], |v| v);
    /// ```
    pub fn push<VAL, F: FnOnce(&VAL) -> &DST>(&mut self, v: VAL, f: F) -> Result<(), VAL>
    where
        (VAL, BUF::Inner): MemAligned,
    {
        <(VAL, BUF::Inner) as MemAligned>::assert_compatibility();

        // SAFETY: Destination address is valid.
        unsafe {
            match self.push_inner(check_fat_pointer(&v, f)) {
                Ok(pii) => {
                    Ptr::write(pii.data.as_mut_ptr() as *mut VAL, v);
                    Ok(())
                }
                Err(()) => Err(v),
            }
        }
    }

    /// Returns a shared reference to the top item on the stack.
    #[must_use]
    pub fn top(&self) -> Option<&DST> {
        self.top_raw().map(|x| unsafe { &*x })
    }

    /// Returns an exclusive reference to the top item on the stack.
    #[must_use]
    pub fn top_mut(&mut self) -> Option<&mut DST> {
        self.top_raw_mut().map(|x| unsafe { &mut *x })
    }

    /// Pops the top item off the stack.
    pub fn pop(&mut self) {
        if let Some(ptr) = self.top_raw_mut() {
            assert!(self.next_ofs > 0);
            // SAFETY: Pointer is valid, and will never be accessed after this point.
            let words = unsafe {
                let size = size_of_val(&*ptr);
                Ptr::drop_in_place(ptr);
                BUF::round_to_words(size)
            };
            self.next_ofs -= words + Self::meta_words();
        }
    }

    /// Returns an immutable iterator
    /// (yields references to items, in the order they would be popped).
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut list = DstStack::<str, DstArray<usize, 8>>::new();
    /// list.push_str("Hello");
    /// list.push_str("world");
    /// let mut it = list.iter();
    /// assert_eq!(it.next(), Some("world"));
    /// assert_eq!(it.next(), Some("Hello"));
    /// assert_eq!(it.next(), None);
    /// ```
    #[must_use]
    pub const fn iter(&self) -> DstStackIter<'_, DST, BUF> {
        DstStackIter(self, self.next_ofs)
    }

    /// Returns a mutable iterator.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut list = DstStack::<[u8], DstArray<usize, 8>>::new();
    /// list.push_copied(&[1,2,3]);
    /// list.push_copied(&[9]);
    /// for v in list.iter_mut() {
    ///     v[0] -= 1;
    /// }
    /// let mut it = list.iter();
    /// assert_eq!(it.next(), Some(&[8][..]));
    /// assert_eq!(it.next(), Some(&[0,2,3][..]));
    /// assert_eq!(it.next(), None);
    /// ```
    #[must_use]
    pub fn iter_mut(&mut self) -> DstStackIterMut<'_, DST, BUF> {
        DstStackIterMut(self, self.next_ofs)
    }
}

impl<BUF: DstBuf> DstStack<str, BUF> {
    /// Pushes the contents of a `string` slice as an item onto the stack.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<str, DstArray<u8, 32>>::new();
    /// stack.push_str("Hello!");
    /// ```
    pub fn push_str(&mut self, string: &str) -> Result<(), ()> {
        unsafe {
            self.push_inner(string).map(|pii| {
                Ptr::copy(
                    string.as_bytes().as_ptr(),
                    pii.data.as_mut_ptr() as *mut u8,
                    string.len(),
                );
            })
        }
    }
}

impl<BUF: DstBuf, DST: Clone> DstStack<[DST], BUF>
where
    (DST, BUF::Inner): MemAligned,
{
    /// Pushes a set of items (cloning out of the input `slice`).
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<u64, 8>>::new();
    /// stack.push_cloned(&[1, 2, 3]);
    /// ```
    pub fn push_cloned(&mut self, slice: &[DST]) -> Result<(), ()> {
        <(DST, BUF::Inner) as MemAligned>::assert_compatibility();
        self.push_from_iter(slice.iter().cloned())
    }
    /// Pushes a set of items (copying out of the input `slice`).
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<u64, 8>>::new();
    /// stack.push_copied(&[1, 2, 3]);
    /// ```
    pub fn push_copied(&mut self, slice: &[DST]) -> Result<(), ()>
    where
        DST: Copy,
    {
        <(DST, BUF::Inner) as MemAligned>::assert_compatibility();
        // SAFETY: Carefully constructed to maintain consistency.
        unsafe {
            self.push_inner(slice).map(|pii| {
                Ptr::copy(
                    slice.as_ptr() as *const u8,
                    pii.data.as_mut_ptr() as *mut u8,
                    size_of_val(slice),
                );
            })
        }
    }
}

impl<BUF: DstBuf, DST> DstStack<[DST], BUF>
where
    (DST, BUF::Inner): MemAligned,
{
    /// Pushes an item, populated from an exact-sized iterator.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<usize, 8>>::new();
    /// stack.push_from_iter(0..10);
    /// assert_eq!(stack.top().unwrap(), &[0,1,2,3,4,5,6,7,8,9]);
    /// ```
    pub fn push_from_iter(
        &mut self,
        mut iter: impl ExactSizeIterator<Item = DST>,
    ) -> Result<(), ()> {
        <(DST, BUF::Inner) as MemAligned>::assert_compatibility();
        // SAFETY: API used correctly.
        unsafe {
            let pii = self.push_inner_raw(iter.len() * size_of::<DST>(), &[0])?;
            list_push_gen(
                pii.meta,
                pii.data,
                iter.len(),
                |_| iter.next().unwrap(),
                pii.reset_slot,
                pii.reset_value,
            );
            Ok(())
        }
    }
}

/* private API */

struct PushInnerInfo<'a, DInner> {
    // Buffer for value data
    data: &'a mut [MaybeUninit<DInner>],

    // Buffer for metadata (length/vtable)
    meta: &'a mut [MaybeUninit<DInner>],

    // Memory location for resetting the push
    reset_slot: &'a mut usize,
    reset_value: usize,
}

impl<DST: ?Sized, BUF: DstBuf> DstStack<DST, BUF> {
    #[must_use]
    fn meta_words() -> usize {
        BUF::round_to_words(size_of::<&DST>() - size_of::<usize>())
    }

    #[must_use]
    unsafe fn raw_at(&self, ofs: usize) -> *mut DST {
        let dar = self.data.as_ref();
        let meta = &dar[dar.len() - ofs..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at(mw);
        // SAFETY: caller must ensure safety
        unsafe { super::make_fat_ptr(data.as_ptr() as *mut (), meta) }
    }
    #[must_use]
    unsafe fn raw_at_mut(&mut self, ofs: usize) -> *mut DST {
        let dar = self.data.as_mut();
        let ofs = dar.len() - ofs;
        let meta = &mut dar[ofs..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at_mut(mw);
        // SAFETY: caller must ensure safety
        unsafe { super::make_fat_ptr(data.as_mut_ptr() as *mut (), meta) }
    }
    // Get a raw pointer to the top of the stack.
    #[must_use]
    fn top_raw(&self) -> Option<*mut DST> {
        if self.next_ofs == 0 {
            None
        } else {
            // SAFETY: Internal consistency maintains the metadata validity.
            Some(unsafe { self.raw_at(self.next_ofs) })
        }
    }
    // Get a raw pointer to the top of the stack
    #[must_use]
    fn top_raw_mut(&mut self) -> Option<*mut DST> {
        if self.next_ofs == 0 {
            None
        } else {
            // SAFETY: Internal consistency maintains the metadata validity.
            Some(unsafe { self.raw_at_mut(self.next_ofs) })
        }
    }

    // See `push_inner_raw`.
    unsafe fn push_inner(&mut self, fat_ptr: &DST) -> Result<PushInnerInfo<'_, BUF::Inner>, ()> {
        let bytes = size_of_val(fat_ptr);
        let (_data_ptr, len, v) = decompose_pointer(fat_ptr);
        // SAFETY: caller must ensure safety
        unsafe { self.push_inner_raw(bytes, &v[..len]) }
    }

    // Returns:
    // - metadata slot
    // - data slot
    // - Total words used
    unsafe fn push_inner_raw(
        &mut self,
        bytes: usize,
        metadata: &[usize],
    ) -> Result<PushInnerInfo<'_, BUF::Inner>, ()> {
        assert!(BUF::round_to_words(size_of_val(metadata)) == Self::meta_words());
        let words = BUF::round_to_words(bytes) + Self::meta_words();

        let req_space = self.next_ofs + words;
        // Attempt to resize (if the underlying buffer allows it).
        if req_space > self.data.as_ref().len() {
            let old_len = self.data.as_ref().len();
            if self.data.extend(req_space).is_ok() {
                let new_len = self.data.as_ref().len();
                self.data.as_mut().rotate_right(new_len - old_len);
            }
        }

        // Check if there is sufficient space for the new item.
        if req_space <= self.data.as_ref().len() {
            // Get the base pointer for the new item
            let prev_next_ofs = self.next_ofs;
            self.next_ofs += words;
            let len = self.data.as_ref().len();
            let slot = &mut self.data.as_mut()[len - self.next_ofs..][..words];
            let (meta, rv) = slot.split_at_mut(Self::meta_words());

            // Populate the metadata.
            super::store_metadata(meta, metadata);

            // Increment offset and return.
            Ok(PushInnerInfo {
                meta,
                data: rv,
                reset_slot: &mut self.next_ofs,
                reset_value: prev_next_ofs,
            })
        } else {
            Err(())
        }
    }
}

mod core_impls {
    use super::{DstBuf, DstStack, DstStackIter, DstStackIterMut};
    use core::{fmt, iter, ops};

    impl<DST: ?Sized, BUF: DstBuf> ops::Drop for DstStack<DST, BUF> {
        fn drop(&mut self) {
            while !self.is_empty() {
                self.pop();
            }
        }
    }
    impl<DST: ?Sized, BUF: DstBuf + Default> Default for DstStack<DST, BUF> {
        fn default() -> Self {
            DstStack::new()
        }
    }

    macro_rules! impl_trait {
        ( $t:path; $($body:tt)* ) => {
            impl<BUF: DstBuf, DST: ?Sized> $t for DstStack<DST, BUF> where DST: $t { $( $body )* }
        }
    }
    // FIXME
    impl_trait! { fmt::Debug;
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("[")?;
            for v in self.iter() {
                v.fmt(f)?;
                f.write_str(",")?;
            }
            f.write_str("]")?;
            Ok( () )
        }
    }

    /* iter */

    impl<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> iter::Iterator for DstStackIter<'a, DST, BUF> {
        type Item = &'a DST;
        fn next(&mut self) -> Option<&'a DST> {
            if self.1 == 0 {
                None
            } else {
                // SAFETY: Bounds checked, aliasing enforced by API.
                let rv = unsafe { &*self.0.raw_at(self.1) };
                self.1 -= DstStack::<DST, BUF>::meta_words() + BUF::round_to_words(size_of_val(rv));
                Some(rv)
            }
        }
    }

    impl<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> iter::Iterator for DstStackIterMut<'a, DST, BUF> {
        type Item = &'a mut DST;
        fn next(&mut self) -> Option<&'a mut DST> {
            if self.1 == 0 {
                None
            } else {
                // SAFETY: Bounds checked, aliasing enforced by API.
                let rv = unsafe { &mut *self.0.raw_at_mut(self.1) };
                self.1 -= DstStack::<DST, BUF>::meta_words() + BUF::round_to_words(size_of_val(rv));
                Some(rv)
            }
        }
    }
}
