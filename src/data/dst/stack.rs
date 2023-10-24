// devela::data::dst::stack
//
//! Implementation of the LIFO stack structure

use super::{check_fat_pointer, decompose_pointer, list_push_gen, BufSlice, DstArray, DstBuf};
use crate::mem::MemAligned;
use core::{iter, marker, mem, ops, ptr};

/// A single LIFO stack of DSTs using a `usize` aligned buffer.
///
/// # Examples
/// ```
/// let mut stack = devela::data::DstStackU::<[u8], 16>::new();
/// stack.push_copied(&[1]);
/// ```
pub type DstStackU<T /*: ?Sized*/, const N: usize /* = 16*/> = DstStack<T, DstArray<usize, N>>;

// Implementation Notes
// -----
//
// The data array is filled from the back, with the metadata stored before
// (at a lower memory address) the actual data. This so the code can use a
// single integer to track the position (using size_of_val when popping items,
// and the known size when pushing).

/// A fixed-capacity stack that can contain dynamically-sized types
///
/// Uses an array of usize as a backing store for a First-In, Last-Out stack
/// of items that can unsize to `T`.
///
/// Note: Each item in the stack takes at least one slot in the buffer
/// (to store the metadata)
pub struct DstStack<T: ?Sized, D: DstBuf> {
    _pd: marker::PhantomData<*const T>,
    // Offset from the _back_ of `data` to the next free position.
    // I.e. data[data.len() - cur_ofs] is the first metadata word
    next_ofs: usize,
    data: D,
}

impl<T: ?Sized, D: DstBuf> ops::Drop for DstStack<T, D> {
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
    }
}
impl<T: ?Sized, D: DstBuf + Default> Default for DstStack<T, D> {
    fn default() -> Self {
        DstStack::new()
    }
}

impl<T: ?Sized, D: DstBuf> DstStack<T, D> {
    /// Construct a new (empty) stack
    pub fn new() -> Self
    where
        D: Default,
    {
        Self::with_buffer(D::default())
    }
    /// Construct a new (empty) stack using the provided buffer
    pub fn with_buffer(data: D) -> Self {
        DstStack {
            _pd: marker::PhantomData,
            next_ofs: 0,
            data,
        }
    }

    /// Tests if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.next_ofs == 0
    }

    fn meta_words() -> usize {
        D::round_to_words(mem::size_of::<&T>() - mem::size_of::<usize>())
    }

    /// Push a value at the top of the stack (without using `Unsize`)
    ///
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<u64, 8>>::new();
    /// stack.push_stable([1, 2,3], |v| v);
    /// ```
    pub fn push_stable<U, F: FnOnce(&U) -> &T>(&mut self, v: U, f: F) -> Result<(), U>
    where
        (U, D::Inner): MemAligned,
    {
        <(U, D::Inner) as MemAligned>::check();

        // SAFE: Destination address is valid
        unsafe {
            match self.push_inner(check_fat_pointer(&v, f)) {
                Ok(pii) => {
                    ptr::write(pii.data.as_mut_ptr() as *mut U, v);
                    Ok(())
                }
                Err(_) => Err(v),
            }
        }
    }

    unsafe fn raw_at(&self, ofs: usize) -> *mut T {
        let dar = self.data.as_ref();
        let meta = &dar[dar.len() - ofs..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at(mw);
        super::make_fat_ptr(data.as_ptr() as *mut (), meta)
    }
    unsafe fn raw_at_mut(&mut self, ofs: usize) -> *mut T {
        let dar = self.data.as_mut();
        let ofs = dar.len() - ofs;
        let meta = &mut dar[ofs..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at_mut(mw);
        super::make_fat_ptr(data.as_mut_ptr() as *mut (), meta)
    }
    // Get a raw pointer to the top of the stack
    fn top_raw(&self) -> Option<*mut T> {
        if self.next_ofs == 0 {
            None
        } else {
            // SAFE: Internal consistency maintains the metadata validity
            Some(unsafe { self.raw_at(self.next_ofs) })
        }
    }
    // Get a raw pointer to the top of the stack
    fn top_raw_mut(&mut self) -> Option<*mut T> {
        if self.next_ofs == 0 {
            None
        } else {
            // SAFE: Internal consistency maintains the metadata validity
            Some(unsafe { self.raw_at_mut(self.next_ofs) })
        }
    }
    /// Returns a pointer to the top item on the stack
    pub fn top(&self) -> Option<&T> {
        self.top_raw().map(|x| unsafe { &*x })
    }
    /// Returns a pointer to the top item on the stack (unique/mutable)
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.top_raw_mut().map(|x| unsafe { &mut *x })
    }
    /// Pop the top item off the stack
    pub fn pop(&mut self) {
        if let Some(ptr) = self.top_raw_mut() {
            assert!(self.next_ofs > 0);
            // SAFE: Pointer is valid, and will never be accessed after this point
            let words = unsafe {
                let size = mem::size_of_val(&*ptr);
                ptr::drop_in_place(ptr);
                D::round_to_words(size)
            };
            self.next_ofs -= words + Self::meta_words();
        }
    }

    /// Obtain an immutable iterator
    /// (yields references to items, in the order they would be popped)
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
    pub fn iter(&self) -> Iter<T, D> {
        Iter(self, self.next_ofs)
    }
    /// Obtain unique/mutable iterator
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
    pub fn iter_mut(&mut self) -> IterMut<T, D> {
        IterMut(self, self.next_ofs)
    }
}

struct PushInnerInfo<'a, DInner> {
    /// Buffer for value data
    data: &'a mut BufSlice<DInner>,
    /// Buffer for metadata (length/vtable)
    meta: &'a mut BufSlice<DInner>,
    /// Memory location for resetting the push
    reset_slot: &'a mut usize,
    reset_value: usize,
}
impl<T: ?Sized, D: DstBuf> DstStack<T, D> {
    /// See `push_inner_raw`
    unsafe fn push_inner(&mut self, fat_ptr: &T) -> Result<PushInnerInfo<D::Inner>, ()> {
        let bytes = mem::size_of_val(fat_ptr);
        let (_data_ptr, len, v) = decompose_pointer(fat_ptr);
        self.push_inner_raw(bytes, &v[..len])
    }

    /// Returns:
    /// - metadata slot
    /// - data slot
    /// - Total words used
    unsafe fn push_inner_raw(
        &mut self,
        bytes: usize,
        metadata: &[usize],
    ) -> Result<PushInnerInfo<D::Inner>, ()> {
        assert!(D::round_to_words(mem::size_of_val(metadata)) == Self::meta_words());
        let words = D::round_to_words(bytes) + Self::meta_words();

        let req_space = self.next_ofs + words;
        // Attempt resize (if the underlying buffer allows it)
        if req_space > self.data.as_ref().len() {
            let old_len = self.data.as_ref().len();
            if self.data.extend(req_space).is_ok() {
                let new_len = self.data.as_ref().len();
                self.data.as_mut().rotate_right(new_len - old_len);
            }
        }

        // Check if there is sufficient space for the new item
        if req_space <= self.data.as_ref().len() {
            // Get the base pointer for the new item
            let prev_next_ofs = self.next_ofs;
            self.next_ofs += words;
            let len = self.data.as_ref().len();
            let slot = &mut self.data.as_mut()[len - self.next_ofs..][..words];
            let (meta, rv) = slot.split_at_mut(Self::meta_words());

            // Populate the metadata
            super::store_metadata(meta, metadata);

            // Increment offset and return
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

impl<D: DstBuf> DstStack<str, D> {
    /// Push the contents of a string slice as an item onto the stack
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<str, DstArray<u8, 32>>::new();
    /// stack.push_str("Hello!");
    /// ```
    pub fn push_str(&mut self, v: &str) -> Result<(), ()> {
        unsafe {
            self.push_inner(v).map(|pii| {
                ptr::copy(
                    v.as_bytes().as_ptr(),
                    pii.data.as_mut_ptr() as *mut u8,
                    v.len(),
                )
            })
        }
    }
}
impl<D: DstBuf, T: Clone> DstStack<[T], D>
where
    (T, D::Inner): MemAligned,
{
    /// Pushes a set of items (cloning out of the input slice)
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<u64, 8>>::new();
    /// stack.push_cloned(&[1, 2, 3]);
    /// ```
    pub fn push_cloned(&mut self, v: &[T]) -> Result<(), ()> {
        <(T, D::Inner) as MemAligned>::check();
        self.push_from_iter(v.iter().cloned())
    }
    /// Pushes a set of items (copying out of the input slice)
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstStack};
    /// let mut stack = DstStack::<[u8], DstArray<u64, 8>>::new();
    /// stack.push_copied(&[1, 2, 3]);
    /// ```
    pub fn push_copied(&mut self, v: &[T]) -> Result<(), ()>
    where
        T: Copy,
    {
        <(T, D::Inner) as MemAligned>::check();
        // SAFE: Carefully constructed to maintain consistency
        unsafe {
            self.push_inner(v).map(|pii| {
                ptr::copy(
                    v.as_ptr() as *const u8,
                    pii.data.as_mut_ptr() as *mut u8,
                    mem::size_of_val(v),
                )
            })
        }
    }
}
impl<D: DstBuf, T> DstStack<[T], D>
where
    (T, D::Inner): MemAligned,
{
    /// Push an item, populated from an exact-sized iterator
    ///
    /// # Examples
    /// ```
    /// # extern crate core;
    /// # use devela::data::{DstArray, DstStack};
    /// # use core::fmt::Display;
    /// let mut stack = DstStack::<[u8], DstArray<usize, 8>>::new();
    /// stack.push_from_iter(0..10);
    /// assert_eq!(stack.top().unwrap(), &[0,1,2,3,4,5,6,7,8,9]);
    /// ```
    pub fn push_from_iter(&mut self, mut iter: impl ExactSizeIterator<Item = T>) -> Result<(), ()> {
        <(T, D::Inner) as MemAligned>::check();
        // SAFE: API used correctly
        unsafe {
            let pii = self.push_inner_raw(iter.len() * mem::size_of::<T>(), &[0])?;
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

/// DST DstStack iterator (immutable)
pub struct Iter<'a, T: 'a + ?Sized, D: 'a + DstBuf>(&'a DstStack<T, D>, usize);
impl<'a, T: 'a + ?Sized, D: 'a + DstBuf> iter::Iterator for Iter<'a, T, D> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.1 == 0 {
            None
        } else {
            // SAFE: Bounds checked, aliasing enforced by API
            let rv = unsafe { &*self.0.raw_at(self.1) };
            self.1 -= DstStack::<T, D>::meta_words() + D::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}

/// DstStack iterator (immutable)
pub struct IterMut<'a, T: 'a + ?Sized, D: 'a + DstBuf>(&'a mut DstStack<T, D>, usize);
impl<'a, T: 'a + ?Sized, D: 'a + DstBuf> iter::Iterator for IterMut<'a, T, D> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        if self.1 == 0 {
            None
        } else {
            // SAFE: Bounds checked, aliasing enforced by API
            let rv = unsafe { &mut *self.0.raw_at_mut(self.1) };
            self.1 -= DstStack::<T, D>::meta_words() + D::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}

mod impls {
    use super::DstBuf;
    use core::fmt;

    macro_rules! d {
        ( $t:path; $($body:tt)* ) => {
            impl<D: DstBuf, T: ?Sized> $t for super::DstStack<T, D>
            where T: $t { $( $body )* }
        }
    }

    d! { fmt::Debug;
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
}
