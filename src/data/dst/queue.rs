// devela::data::dst::queue
//
//! Implementation of the FIFO queue structure

use super::{check_fat_pointer, decompose_pointer, list_push_gen, BufSlice, DstArray, DstBuf};
use crate::mem::MemAligned;
use core::{iter, marker, mem, ops, ptr};

/// A FIFO queue of DSTs using a `usize` aligned buffer
///
/// # Examples
/// ```
/// # use devela::data::{DstQueueU};
/// let mut queue = DstQueueU::<[u8], 16>::new();
/// queue.push_copied(&[1]);
/// ```
pub type DstQueueU<T /*: ?Sized*/, const N: usize /* = {8+1}*/> = DstQueue<T, DstArray<usize, N>>;

// Implementation Notes
// -----
//
/// A First-In-First-Out queue of DSTs
///
/// # Examples
/// ```
/// # use devela::data::{DstArray, DstQueue};
/// let mut queue = DstQueue::<str, DstArray<usize, 8>>::new();
/// queue.push_back_str("Hello");
/// queue.push_back_str("World");
/// assert_eq!(queue.pop_front().as_ref().map(|v| &v[..]), Some("Hello"));
/// ```
pub struct DstQueue<T: ?Sized, D: DstBuf> {
    _pd: marker::PhantomData<*const T>,
    read_pos: usize,
    write_pos: usize,
    data: D,
}
impl<T: ?Sized, D: DstBuf> DstQueue<T, D> {
    /// Construct a new (empty) list
    pub fn new() -> Self
    where
        D: Default,
    {
        Self::with_buffer(D::default())
    }
    /// Construct a new (empty) list using the provided buffer
    pub fn with_buffer(data: D) -> Self {
        DstQueue {
            _pd: marker::PhantomData,
            read_pos: 0,
            write_pos: 0,
            data,
        }
    }

    fn meta_words() -> usize {
        D::round_to_words(mem::size_of::<&T>() - mem::size_of::<usize>())
    }
    fn space_words(&self) -> usize {
        self.data.as_ref().len() - self.write_pos
    }

    /// Push a value to the end of the list (without using `Unsize`)
    pub fn push_back_stable<U, F: FnOnce(&U) -> &T>(&mut self, v: U, f: F) -> Result<(), U>
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

    /// Compact the list (moving the read position to zero)
    pub fn compact(&mut self) {
        if self.read_pos != 0 {
            self.data.as_mut().rotate_left(self.read_pos);
            self.write_pos -= self.read_pos;
            self.read_pos = 0;
        }
    }

    /// Checks if the queue is currently empty
    pub fn empty(&self) -> bool {
        self.read_pos == self.write_pos
    }

    /// Remove an item from the front of the list
    pub fn pop_front(&mut self) -> Option<PopHandle<T, D>> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(PopHandle { parent: self })
        }
    }
    /// Peek the front of the queue
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(unsafe { &mut *self.front_raw_mut() })
        }
    }
    /// Peek the front of the queue
    pub fn front(&self) -> Option<&T> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(unsafe { &*self.front_raw() })
        }
    }

    /// Obtain an immutable iterator
    /// (yields references to items, in insertion order)
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstQueue};
    /// let mut list = DstQueue::<str, DstArray<usize, 8>>::new();
    /// list.push_back_str("Hello");
    /// list.push_back_str("world");
    /// let mut it = list.iter();
    /// assert_eq!(it.next(), Some("Hello"));
    /// assert_eq!(it.next(), Some("world"));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T, D> {
        Iter(self, self.read_pos)
    }
    /// Obtain a mutable iterator
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstQueue};
    /// let mut list = DstQueue::<[u8], DstArray<usize, 8>>::new();
    /// list.push_copied(&[1,2,3]);
    /// list.push_copied(&[9]);
    /// for v in list.iter_mut() {
    ///     v[0] -= 1;
    /// }
    /// let mut it = list.iter();
    /// assert_eq!(it.next(), Some(&[0,2,3][..]));
    /// assert_eq!(it.next(), Some(&[8][..]));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T, D> {
        IterMut(self, self.read_pos)
    }
    // Note: No into_iter, not possible due to unsized types
    // Could make a `drain` that returns read handles (pops as it goes)

    fn front_raw(&self) -> *mut T {
        assert!(self.read_pos < self.write_pos);

        // SAFE: Internal consistency maintains the metadata validity
        unsafe { self.raw_at(self.read_pos) }
    }
    // UNSAFE: Caller must ensure that `pos` is the start of an object
    unsafe fn raw_at(&self, pos: usize) -> *mut T {
        assert!(pos >= self.read_pos);
        assert!(pos < self.write_pos);
        let meta = &self.data.as_ref()[pos..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at(mw);
        super::make_fat_ptr(data.as_ptr() as *mut (), meta)
    }
    fn front_raw_mut(&mut self) -> *mut T {
        assert!(self.read_pos < self.write_pos);

        // SAFE: Internal consistency maintains the metadata validity
        unsafe { self.raw_at_mut(self.read_pos) }
    }
    // UNSAFE: Caller must ensure that `pos` is the start of an object
    unsafe fn raw_at_mut(&mut self, pos: usize) -> *mut T {
        assert!(pos >= self.read_pos);
        assert!(pos < self.write_pos);
        let meta = &mut self.data.as_mut()[pos..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at_mut(mw);
        super::make_fat_ptr(data.as_mut_ptr() as *mut (), meta)
    }
    fn pop_front_inner(&mut self) {
        // SAFE: `front_raw_mut` asserts that there's an item, rest is correct
        unsafe {
            let ptr = &mut *self.front_raw_mut();
            let len = mem::size_of_val(ptr);
            ptr::drop_in_place(ptr);
            let words = D::round_to_words(len);
            self.read_pos += Self::meta_words() + words;
        }
    }

    /// Remove any items that don't meet a predicate
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstQueue};
    /// # use core::{any::Any, fmt::Debug};
    /// trait DebugAny: 'static + Any + Debug { fn as_any(&self) -> &dyn Any; }
    /// impl<T: Debug + Any + 'static> DebugAny for T { fn as_any(&self) -> &dyn Any { self } }
    /// let mut list = {
    ///     let mut list: DstQueue<dyn DebugAny, DstArray<usize, 8>> = DstQueue::new();
    ///     list.push_back_stable(1234, |v| v);
    ///     list.push_back_stable(234.5f32, |v| v);
    ///     list.push_back_stable(5678, |v| v);
    ///     list.push_back_stable(0.5f32, |v| v);
    ///     list
    /// };
    /// list.retain(|v| (*v).as_any().downcast_ref::<f32>().is_some());
    /// let mut it = list.iter().map(|v| format!("{:?}", v));
    /// assert_eq!(it.next(), Some("234.5".to_owned()));
    /// assert_eq!(it.next(), Some("0.5".to_owned()));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn retain<Cb>(&mut self, mut cb: Cb)
    where
        Cb: FnMut(&mut T) -> bool,
    {
        let orig_write_pos = self.write_pos;
        self.write_pos = self.read_pos;
        let mut ofs = self.read_pos;
        let mut writeback_pos = ofs;
        while ofs < orig_write_pos {
            let v: &mut T = unsafe {
                let meta = &mut self.data.as_mut()[ofs..];
                let mw = Self::meta_words();
                let (meta, data) = meta.split_at_mut(mw);
                &mut *super::make_fat_ptr(data.as_mut_ptr() as *mut (), meta)
            };
            let words = Self::meta_words() + D::round_to_words(mem::size_of_val(v));
            if cb(v) {
                if writeback_pos != ofs {
                    let d = self.data.as_mut();
                    // writeback is always before `ofs`, so this ordering is correct
                    for i in 0..words {
                        let (a, b) = d.split_at_mut(ofs + i);
                        a[writeback_pos + i] = b[0];
                    }
                }
                writeback_pos += words;
            } else {
                // Don't update `writeback_pos`
                // SAFE: Valid pointer, won't be accessed again
                unsafe {
                    ptr::drop_in_place(v);
                }
            }
            ofs += words;
        }
        assert!(ofs == orig_write_pos);
        self.write_pos = writeback_pos;
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

impl<T: ?Sized, D: DstBuf> DstQueue<T, D> {
    /// Push an item to the list (setting metadata based on `fat_ptr`)
    /// UNSAFE: Caller must fill the buffer before any potential panic
    unsafe fn push_inner(&mut self, fat_ptr: &T) -> Result<PushInnerInfo<D::Inner>, ()> {
        let bytes = mem::size_of_val(fat_ptr);
        let (_data_ptr, len, v) = decompose_pointer(fat_ptr);
        self.push_inner_raw(bytes, &v[..len])
    }
    unsafe fn push_inner_raw(
        &mut self,
        bytes: usize,
        metadata: &[usize],
    ) -> Result<PushInnerInfo<D::Inner>, ()> {
        let words = D::round_to_words(bytes) + Self::meta_words();

        // 1. Check if there's space for the item
        if self.space_words() < words {
            // 2. If not, check if compaction would help
            if self.space_words() + self.read_pos >= words {
                self.compact();
            }
            // 3. Then, try expanding
            if self.space_words() < words && self.data.extend(self.write_pos + words).is_err() {
                // if expansion fails, return error
                return Err(());
            }
        }
        assert!(self.space_words() >= words);

        // Get the base pointer for the new item
        let slot = &mut self.data.as_mut()[self.write_pos..][..words];
        let prev_write_pos = self.write_pos;
        self.write_pos += words;
        let (meta, rv) = slot.split_at_mut(Self::meta_words());

        // Populate the metadata
        super::store_metadata(meta, metadata);

        // Increment offset and return
        Ok(PushInnerInfo {
            meta,
            data: rv,
            reset_slot: &mut self.write_pos,
            reset_value: prev_write_pos,
        })
    }
}

impl<D: DstBuf> DstQueue<str, D> {
    /// Push the contents of a string slice as an item onto the stack
    pub fn push_back_str(&mut self, v: &str) -> Result<(), ()> {
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

impl<D: DstBuf, T: Clone> DstQueue<[T], D>
where
    (T, D::Inner): MemAligned,
{
    /// Pushes a set of items (cloning out of the input slice)
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstQueue};
    /// let mut queue = DstQueue::<[String], DstArray<usize, 8>>::new();
    /// queue.push_cloned(&["1".to_owned()]);
    /// ```
    pub fn push_cloned(&mut self, v: &[T]) -> Result<(), ()> {
        <(T, D::Inner) as MemAligned>::check();
        self.push_from_iter(v.iter().cloned())
    }
    /// Pushes a set of items (copying out of the input slice)
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstQueue};
    /// let mut queue = DstQueue::<[usize], DstArray<usize, 8>>::new();
    /// queue.push_copied(&[1]);
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
impl<D: DstBuf, T> DstQueue<[T], D>
where
    (T, D::Inner): MemAligned,
{
    /// Push an item, populated from an exact-sized iterator
    ///
    /// # Examples
    /// ```
    /// # use devela::data::{DstArray, DstQueue};
    /// # use core::fmt::Display;
    /// let mut stack = DstQueue::<[u8], DstArray<usize, 8>>::new();
    /// stack.push_from_iter(0..10);
    /// assert_eq!(stack.front().unwrap(), &[0,1,2,3,4,5,6,7,8,9]);
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

impl<T: ?Sized, D: DstBuf> ops::Drop for DstQueue<T, D> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
impl<T: ?Sized, D: DstBuf + Default> Default for DstQueue<T, D> {
    fn default() -> Self {
        DstQueue::new()
    }
}

/// Handle returned by `DstQueue::pop` (does the actual pop on drop)
pub struct PopHandle<'a, T: 'a + ?Sized, D: 'a + DstBuf> {
    parent: &'a mut DstQueue<T, D>,
}
impl<'a, T: ?Sized, D: DstBuf> ops::Deref for PopHandle<'a, T, D> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.parent.front_raw() }
    }
}
impl<'a, T: ?Sized, D: DstBuf> ops::DerefMut for PopHandle<'a, T, D> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.parent.front_raw_mut() }
    }
}
impl<'a, T: ?Sized, D: DstBuf> ops::Drop for PopHandle<'a, T, D> {
    fn drop(&mut self) {
        self.parent.pop_front_inner();
    }
}

/// DST FIFO iterator (immutable)
pub struct Iter<'a, T: 'a + ?Sized, D: 'a + DstBuf>(&'a DstQueue<T, D>, usize);
impl<'a, T: 'a + ?Sized, D: 'a + DstBuf> iter::Iterator for Iter<'a, T, D> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.1 == self.0.write_pos {
            None
        } else {
            // SAFE: Bounds checked, aliasing enforced by API
            let rv = unsafe { &*self.0.raw_at(self.1) };
            self.1 += DstQueue::<T, D>::meta_words() + D::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}
/// DST FIFO iterator (mutable)
pub struct IterMut<'a, T: 'a + ?Sized, D: 'a + DstBuf>(&'a mut DstQueue<T, D>, usize);
impl<'a, T: 'a + ?Sized, D: 'a + DstBuf> iter::Iterator for IterMut<'a, T, D> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        if self.1 == self.0.write_pos {
            None
        } else {
            // SAFE: Bounds checked, aliasing enforced by API
            let rv = unsafe { &mut *self.0.raw_at_mut(self.1) };
            self.1 += DstQueue::<T, D>::meta_words() + D::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}

mod impls {
    use super::DstBuf;
    use core::fmt;

    macro_rules! d {
        ( $t:path; $($body:tt)* ) => {
            impl<D: DstBuf, T: ?Sized> $t for super::DstQueue<T, D> where T: $t { $( $body )* }
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
