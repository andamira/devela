//
//
//!
//
// TOC

use super::super::{check_fat_pointer, list_push_gen, make_fat_ptr};
use super::{DstBuf, DstQueue, DstQueueIter, DstQueueIterMut, DstQueuePopHandle};
use crate::{
    _core::{marker, ptr},
    MemAligned,
};

impl<DST: ?Sized, BUF: DstBuf> DstQueue<DST, BUF> {
    /// Constructs a new (empty) queue.
    #[must_use] #[rustfmt::skip]
    pub fn new() -> Self where BUF: Default { Self::with_buffer(BUF::default()) }

    /// Constructs a new (empty) queue using the given `buffer`.
    #[must_use] #[rustfmt::skip]
    pub fn with_buffer(data: BUF) -> Self {
        DstQueue { _pd: marker::PhantomData, read_pos: 0, write_pos: 0, data }
    }

    /// Pushes a value to the end of the queue.
    pub fn push_back<VAL, F>(&mut self, value: VAL, f: F) -> Result<(), VAL>
    where
        F: FnOnce(&VAL) -> &DST,
        (VAL, BUF::Inner): MemAligned,
    {
        <(VAL, BUF::Inner) as MemAligned>::assert_compatibility();

        // SAFETY: Destination address is valid.
        unsafe {
            match self.push_inner(check_fat_pointer(&value, f)) {
                Ok(pii) => {
                    ptr::write(pii.data.as_mut_ptr() as *mut VAL, value);
                    Ok(())
                }
                Err(()) => Err(value),
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
    #[must_use]
    pub const fn empty(&self) -> bool {
        self.read_pos == self.write_pos
    }

    /// Removes an item from the front of the queue.
    #[must_use]
    pub fn pop_front(&mut self) -> Option<DstQueuePopHandle<'_, DST, BUF>> {
        if self.read_pos == self.write_pos {
            None
        } else {
            Some(DstQueuePopHandle { parent: self })
        }
    }

    /// Returns an exclusive reference to the front element.
    #[must_use]
    pub fn front_mut(&mut self) -> Option<&mut DST> {
        if self.read_pos == self.write_pos {
            None
        } else {
            // SAFETY: TODO
            Some(unsafe { &mut *self.front_raw_mut() })
        }
    }

    /// Returns a shared reference to the front element.
    #[must_use]
    pub fn front(&self) -> Option<&DST> {
        if self.read_pos == self.write_pos {
            None
        } else {
            // SAFETY: TODO
            Some(unsafe { &*self.front_raw() })
        }
    }

    /// Returns an immutable iterator
    /// (yields references to items, in insertion order).
    ///
    /// # Examples
    /// ```
    /// # use devela::{DstArray, DstQueue};
    /// let mut list = DstQueue::<str, DstArray<usize, 8>>::new();
    /// list.push_back_str("Hello");
    /// list.push_back_str("world");
    /// let mut it = list.iter();
    /// assert_eq!(it.next(), Some("Hello"));
    /// assert_eq!(it.next(), Some("world"));
    /// assert_eq!(it.next(), None);
    /// ```
    #[must_use]
    pub const fn iter(&self) -> DstQueueIter<'_, DST, BUF> {
        DstQueueIter(self, self.read_pos)
    }

    /// Returns a mutable iterator.
    ///
    /// # Examples
    /// ```
    /// # use devela::{DstArray, DstQueue};
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
    #[must_use]
    pub fn iter_mut(&mut self) -> DstQueueIterMut<'_, DST, BUF> {
        DstQueueIterMut(self, self.read_pos)
    }
    // NOTE: No into_iter, not possible due to unsized types.
    // Could make a `drain` that returns read handles (pops as it goes).

    /// Removes any items that don't meet a predicate.
    ///
    /// # Examples
    /// ```
    /// # use {devela::{DstArray, DstQueue}, core::{any::Any, fmt::Debug}};
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
            // SAFETY: TODO
            let v: &mut DST = unsafe {
                let meta = &mut self.data.as_mut()[ofs..];
                let mw = Self::meta_words();
                let (meta, data) = meta.split_at_mut(mw);
                &mut *make_fat_ptr(data.as_mut_ptr() as *mut (), meta)
            };
            let words = Self::meta_words() + BUF::round_to_words(size_of_val(v));
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

impl<BUF: DstBuf, DST> DstQueue<[DST], BUF>
where
    (DST, BUF::Inner): MemAligned,
{
    /// Pushes a set of items (cloning out of the input `slice`).
    ///
    /// # Examples
    /// ```
    /// # use devela::{DstArray, DstQueue};
    /// let mut queue = DstQueue::<[String], DstArray<usize, 8>>::new();
    /// queue.push_cloned(&["1".to_owned()]);
    /// ```
    pub fn push_cloned(&mut self, slice: &[DST]) -> Result<(), ()>
    where
        DST: Clone,
    {
        <(DST, BUF::Inner) as MemAligned>::assert_compatibility();
        self.push_from_iter(slice.iter().cloned())
    }

    /// Pushes a set of items (copying out of the input `slice`).
    ///
    /// # Examples
    /// ```
    /// # use devela::{DstArray, DstQueue};
    /// let mut queue = DstQueue::<[usize], DstArray<usize, 8>>::new();
    /// queue.push_copied(&[1]);
    /// ```
    pub fn push_copied(&mut self, slice: &[DST]) -> Result<(), ()>
    where
        DST: Copy,
    {
        <(DST, BUF::Inner) as MemAligned>::assert_compatibility();
        // SAFETY: Carefully constructed to maintain consistency.
        unsafe {
            self.push_inner(slice).map(|pii| {
                ptr::copy(
                    slice.as_ptr() as *const u8,
                    pii.data.as_mut_ptr() as *mut u8,
                    size_of_val(slice),
                );
            })
        }
    }

    /// Pushes an item, populated from an exact-sized iterator.
    ///
    /// # Examples
    /// ```
    /// # use devela::{DstArray, DstQueue};
    /// let mut stack = DstQueue::<[u8], DstArray<usize, 8>>::new();
    /// stack.push_from_iter(0..10);
    /// assert_eq!(stack.front().unwrap(), &[0,1,2,3,4,5,6,7,8,9]);
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

impl<BUF: DstBuf> DstQueue<str, BUF> {
    /// Pushes the contents of a `string` slice as an item onto the stack.
    pub fn push_back_str(&mut self, string: &str) -> Result<(), ()> {
        // SAFETY TODO
        unsafe {
            self.push_inner(string).map(|pii| {
                ptr::copy(
                    string.as_bytes().as_ptr(),
                    pii.data.as_mut_ptr() as *mut u8,
                    string.len(),
                );
            })
        }
    }
}
