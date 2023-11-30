// devela::data::dst::queue
//
//! Implementation of the FIFO queue structure.

use super::{check_fat_pointer, decompose_pointer, list_push_gen, DstArray, DstBuf};
use crate::mem::MemAligned;
use core::{iter, marker, mem, ops, ptr};

/// A statically allocated FIFO queue of <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
///
/// # Examples
/// ```
/// use devela::data::{DstQueueU};
///
/// let mut queue = DstQueueU::<[u8], 16>::new();
/// queue.push_copied(&[1]);
/// ```
pub type DstQueueU<DST /*: ?Sized*/, const N: usize> = DstQueue<DST, DstArray<usize, N>>;

// Implementation Notes
// -----
//
/// A statically allocated FIFO queue of <abbr title="Dynamically sized type">DST</abbr>s.
///
/// # Examples
/// ```
/// use devela::data::{DstArray, DstQueue};
///
/// let mut queue = DstQueue::<str, DstArray<usize, 8>>::new();
/// queue.push_back_str("Hello");
/// queue.push_back_str("World");
/// assert_eq!(queue.pop_front().as_ref().map(|v| &v[..]), Some("Hello"));
/// ```
pub struct DstQueue<DST: ?Sized, BUF: DstBuf> {
    _pd: marker::PhantomData<*const DST>,
    read_pos: usize,
    write_pos: usize,
    data: BUF,
}
impl<DST: ?Sized, BUF: DstBuf> DstQueue<DST, BUF> {
    /// Constructs a new (empty) queue.
    pub fn new() -> Self
    where
        BUF: Default,
    {
        Self::with_buffer(BUF::default())
    }
    /// Constructs a new (empty) queue using the given `buffer`.
    pub fn with_buffer(data: BUF) -> Self {
        DstQueue {
            _pd: marker::PhantomData,
            read_pos: 0,
            write_pos: 0,
            data,
        }
    }

    fn meta_words() -> usize {
        BUF::round_to_words(mem::size_of::<&DST>() - mem::size_of::<usize>())
    }
    fn space_words(&self) -> usize {
        self.data.as_ref().len() - self.write_pos
    }

    /// Pushes a value to the end of the queue.
    pub fn push_back<VAL, F: FnOnce(&VAL) -> &DST>(&mut self, v: VAL, f: F) -> Result<(), VAL>
    where
        (VAL, BUF::Inner): MemAligned,
    {
        <(VAL, BUF::Inner) as MemAligned>::check();

        // SAFETY: Destination address is valid.
        unsafe {
            match self.push_inner(check_fat_pointer(&v, f)) {
                Ok(pii) => {
                    ptr::write(pii.data.as_mut_ptr() as *mut VAL, v);
                    Ok(())
                }
                Err(_) => Err(v),
            }
        }
    }

    /// Compacts the queue (moving the read position to zero).
    pub fn compact(&mut self) {
        if self.read_pos != 0 {
            self.data.as_mut().rotate_left(self.read_pos);
            self.write_pos -= self.read_pos;
            self.read_pos = 0;
        }
    }

    /// Returns `true` if the queue is empty.
    pub const fn empty(&self) -> bool {
        self.read_pos == self.write_pos
    }

    /// Removes an item from the front of the queue.
    pub fn pop_front(&mut self) -> Option<DstQueuePopHandle<DST, BUF>> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(DstQueuePopHandle { parent: self })
        }
    }
    /// Returns an exclusive reference to the front element.
    pub fn front_mut(&mut self) -> Option<&mut DST> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(unsafe { &mut *self.front_raw_mut() })
        }
    }
    /// Returns a shared reference to the front element.
    pub fn front(&self) -> Option<&DST> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(unsafe { &*self.front_raw() })
        }
    }

    /// Returns an immutable iterator
    /// (yields references to items, in insertion order).
    ///
    /// # Examples
    /// ```
    /// use devela::data::{DstArray, DstQueue};
    ///
    /// let mut list = DstQueue::<str, DstArray<usize, 8>>::new();
    /// list.push_back_str("Hello");
    /// list.push_back_str("world");
    /// let mut it = list.iter();
    /// assert_eq!(it.next(), Some("Hello"));
    /// assert_eq!(it.next(), Some("world"));
    /// assert_eq!(it.next(), None);
    /// ```
    pub const fn iter(&self) -> DstQueueIter<DST, BUF> {
        DstQueueIter(self, self.read_pos)
    }

    /// Returns a mutable iterator.
    ///
    /// # Examples
    /// ```
    /// use devela::data::{DstArray, DstQueue};
    ///
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
    pub fn iter_mut(&mut self) -> DstQueueIterMut<DST, BUF> {
        DstQueueIterMut(self, self.read_pos)
    }
    // NOTE: No into_iter, not possible due to unsized types.
    // Could make a `drain` that returns read handles (pops as it goes).

    fn front_raw(&self) -> *mut DST {
        assert!(self.read_pos < self.write_pos);

        // SAFETY: Internal consistency maintains the metadata validity.
        unsafe { self.raw_at(self.read_pos) }
    }
    // SAFETY:UNSAFE Caller must ensure that `pos` is the start of an object.
    unsafe fn raw_at(&self, pos: usize) -> *mut DST {
        assert!(pos >= self.read_pos);
        assert!(pos < self.write_pos);
        let meta = &self.data.as_ref()[pos..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at(mw);
        super::make_fat_ptr(data.as_ptr() as *mut (), meta)
    }
    fn front_raw_mut(&mut self) -> *mut DST {
        assert!(self.read_pos < self.write_pos);

        // SAFETY: Internal consistency maintains the metadata validity.
        unsafe { self.raw_at_mut(self.read_pos) }
    }
    // SAFETY:UNSAFE Caller must ensure that `pos` is the start of an object.
    unsafe fn raw_at_mut(&mut self, pos: usize) -> *mut DST {
        assert!(pos >= self.read_pos);
        assert!(pos < self.write_pos);
        let meta = &mut self.data.as_mut()[pos..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at_mut(mw);
        super::make_fat_ptr(data.as_mut_ptr() as *mut (), meta)
    }
    fn pop_front_inner(&mut self) {
        // SAFETY: `front_raw_mut` asserts that there's an item, rest is correct.
        unsafe {
            let ptr = &mut *self.front_raw_mut();
            let len = mem::size_of_val(ptr);
            ptr::drop_in_place(ptr);
            let words = BUF::round_to_words(len);
            self.read_pos += Self::meta_words() + words;
        }
    }

    /// Removes any items that don't meet a predicate.
    ///
    /// # Examples
    /// ```
    /// use devela::data::{DstArray, DstQueue};
    /// use core::{any::Any, fmt::Debug};
    ///
    /// trait DebugAny: 'static + Any + Debug { fn as_any(&self) -> &dyn Any; }
    /// impl<DST: Debug + Any + 'static> DebugAny for DST { fn as_any(&self) -> &dyn Any { self } }
    /// let mut list = {
    ///     let mut list: DstQueue<dyn DebugAny, DstArray<usize, 8>> = DstQueue::new();
    ///     list.push_back(1234, |v| v);
    ///     list.push_back(234.5f32, |v| v);
    ///     list.push_back(5678, |v| v);
    ///     list.push_back(0.5f32, |v| v);
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
        Cb: FnMut(&mut DST) -> bool,
    {
        let orig_write_pos = self.write_pos;
        self.write_pos = self.read_pos;
        let mut ofs = self.read_pos;
        let mut writeback_pos = ofs;
        while ofs < orig_write_pos {
            let v: &mut DST = unsafe {
                let meta = &mut self.data.as_mut()[ofs..];
                let mw = Self::meta_words();
                let (meta, data) = meta.split_at_mut(mw);
                &mut *super::make_fat_ptr(data.as_mut_ptr() as *mut (), meta)
            };
            let words = Self::meta_words() + BUF::round_to_words(mem::size_of_val(v));
            if cb(v) {
                if writeback_pos != ofs {
                    let d = self.data.as_mut();
                    // writeback is always before `ofs`, so this ordering is correct.
                    for i in 0..words {
                        let (a, b) = d.split_at_mut(ofs + i);
                        a[writeback_pos + i] = b[0];
                    }
                }
                writeback_pos += words;
            } else {
                // Don't update `writeback_pos`.
                // SAFETY: Valid pointer, won't be accessed again.
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
    // Buffer for value data.
    data: &'a mut [mem::MaybeUninit<DInner>],

    // Buffer for metadata (length/vtable).
    meta: &'a mut [mem::MaybeUninit<DInner>],

    // Memory location for resetting the push.
    reset_slot: &'a mut usize,
    reset_value: usize,
}

impl<DST: ?Sized, BUF: DstBuf> DstQueue<DST, BUF> {
    // Pushes an item to the list (setting metadata based on `fat_ptr`).
    //
    // # Safety
    // Unsafe. Caller must fill the buffer before any potential panic.
    unsafe fn push_inner(&mut self, fat_ptr: &DST) -> Result<PushInnerInfo<BUF::Inner>, ()> {
        let bytes = mem::size_of_val(fat_ptr);
        let (_data_ptr, len, v) = decompose_pointer(fat_ptr);
        self.push_inner_raw(bytes, &v[..len])
    }

    unsafe fn push_inner_raw(
        &mut self,
        bytes: usize,
        metadata: &[usize],
    ) -> Result<PushInnerInfo<BUF::Inner>, ()> {
        let words = BUF::round_to_words(bytes) + Self::meta_words();

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

impl<BUF: DstBuf> DstQueue<str, BUF> {
    /// Pushes the contents of a `string` slice as an item onto the stack.
    pub fn push_back_str(&mut self, string: &str) -> Result<(), ()> {
        unsafe {
            self.push_inner(string).map(|pii| {
                ptr::copy(
                    string.as_bytes().as_ptr(),
                    pii.data.as_mut_ptr() as *mut u8,
                    string.len(),
                )
            })
        }
    }
}

impl<BUF: DstBuf, DST: Clone> DstQueue<[DST], BUF>
where
    (DST, BUF::Inner): MemAligned,
{
    /// Pushes a set of items (cloning out of the input `slice`).
    ///
    /// # Examples
    /// ```
    /// use devela::data::{DstArray, DstQueue};
    ///
    /// let mut queue = DstQueue::<[String], DstArray<usize, 8>>::new();
    /// queue.push_cloned(&["1".to_owned()]);
    /// ```
    pub fn push_cloned(&mut self, slice: &[DST]) -> Result<(), ()> {
        <(DST, BUF::Inner) as MemAligned>::check();
        self.push_from_iter(slice.iter().cloned())
    }

    /// Pushes a set of items (copying out of the input `slice`).
    ///
    /// # Examples
    /// ```
    /// use devela::data::{DstArray, DstQueue};
    ///
    /// let mut queue = DstQueue::<[usize], DstArray<usize, 8>>::new();
    /// queue.push_copied(&[1]);
    /// ```
    pub fn push_copied(&mut self, slice: &[DST]) -> Result<(), ()>
    where
        DST: Copy,
    {
        <(DST, BUF::Inner) as MemAligned>::check();
        // SAFETY: Carefully constructed to maintain consistency.
        unsafe {
            self.push_inner(slice).map(|pii| {
                ptr::copy(
                    slice.as_ptr() as *const u8,
                    pii.data.as_mut_ptr() as *mut u8,
                    mem::size_of_val(slice),
                )
            })
        }
    }
}
impl<BUF: DstBuf, DST> DstQueue<[DST], BUF>
where
    (DST, BUF::Inner): MemAligned,
{
    /// Pushes an item, populated from an exact-sized iterator.
    ///
    /// # Examples
    /// ```
    /// use devela::data::{DstArray, DstQueue};
    ///
    /// let mut stack = DstQueue::<[u8], DstArray<usize, 8>>::new();
    /// stack.push_from_iter(0..10);
    /// assert_eq!(stack.front().unwrap(), &[0,1,2,3,4,5,6,7,8,9]);
    /// ```
    pub fn push_from_iter(
        &mut self,
        mut iter: impl ExactSizeIterator<Item = DST>,
    ) -> Result<(), ()> {
        <(DST, BUF::Inner) as MemAligned>::check();
        // SAFETY: API used correctly.
        unsafe {
            let pii = self.push_inner_raw(iter.len() * mem::size_of::<DST>(), &[0])?;
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

impl<DST: ?Sized, BUF: DstBuf> ops::Drop for DstQueue<DST, BUF> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
impl<DST: ?Sized, BUF: DstBuf + Default> Default for DstQueue<DST, BUF> {
    fn default() -> Self {
        DstQueue::new()
    }
}

/// Handle returned by [`DstQueue::pop`][DstQueue#method.pop]
/// (does the actual pop on drop).
pub struct DstQueuePopHandle<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> {
    parent: &'a mut DstQueue<DST, BUF>,
}
impl<'a, DST: ?Sized, BUF: DstBuf> ops::Deref for DstQueuePopHandle<'a, DST, BUF> {
    type Target = DST;
    fn deref(&self) -> &DST {
        unsafe { &*self.parent.front_raw() }
    }
}
impl<'a, DST: ?Sized, BUF: DstBuf> ops::DerefMut for DstQueuePopHandle<'a, DST, BUF> {
    fn deref_mut(&mut self) -> &mut DST {
        unsafe { &mut *self.parent.front_raw_mut() }
    }
}
impl<'a, DST: ?Sized, BUF: DstBuf> ops::Drop for DstQueuePopHandle<'a, DST, BUF> {
    fn drop(&mut self) {
        self.parent.pop_front_inner();
    }
}

/// An iterator over the elements of a [`DstQueue`].
pub struct DstQueueIter<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(&'a DstQueue<DST, BUF>, usize);
impl<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> iter::Iterator for DstQueueIter<'a, DST, BUF> {
    type Item = &'a DST;
    fn next(&mut self) -> Option<&'a DST> {
        if self.1 == self.0.write_pos {
            None
        } else {
            // SAFETY: Bounds checked, aliasing enforced by API.
            let rv = unsafe { &*self.0.raw_at(self.1) };
            self.1 +=
                DstQueue::<DST, BUF>::meta_words() + BUF::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}
/// A mutable iterator over the elements of a [`DstQueue`].
pub struct DstQueueIterMut<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(
    &'a mut DstQueue<DST, BUF>,
    usize,
);
impl<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> iter::Iterator for DstQueueIterMut<'a, DST, BUF> {
    type Item = &'a mut DST;
    fn next(&mut self) -> Option<&'a mut DST> {
        if self.1 == self.0.write_pos {
            None
        } else {
            // SAFETY: Bounds checked, aliasing enforced by API
            let rv = unsafe { &mut *self.0.raw_at_mut(self.1) };
            self.1 +=
                DstQueue::<DST, BUF>::meta_words() + BUF::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}

mod impls {
    use super::DstBuf;
    use core::fmt;

    macro_rules! d {
        ( $t:path; $($body:tt)* ) => {
            impl<BUF: DstBuf, DST: ?Sized> $t for super::DstQueue<DST, BUF>
                where DST: $t { $( $body )* }
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
