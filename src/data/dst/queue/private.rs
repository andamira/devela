// devela::data::dst::queue::private
//
//! DstQueue private API
//
// TOC

use super::super::{decompose_pointer, make_fat_ptr, store_metadata};
use super::{DstBuf, DstQueue};
use crate::{MaybeUninit, Ptr};

pub(super) struct PushInnerInfo<'a, I> {
    // Buffer for value data.
    pub(super) data: &'a mut [MaybeUninit<I>],

    // Buffer for metadata (length/vtable).
    pub(super) meta: &'a mut [MaybeUninit<I>],

    // Memory location for resetting the push.
    pub(super) reset_slot: &'a mut usize,
    pub(super) reset_value: usize,
}

impl<DST: ?Sized, BUF: DstBuf> DstQueue<DST, BUF> {
    /// Pushes an item to the list (setting metadata based on `fat_ptr`).
    //
    // SAFETY: Caller must fill the buffer before any potential panic.
    pub(super) unsafe fn push_inner(
        &mut self,
        fat_ptr: &DST,
    ) -> Result<PushInnerInfo<'_, BUF::Inner>, ()> {
        let bytes = size_of_val(fat_ptr);
        let (_data_ptr, len, v) = decompose_pointer(fat_ptr);
        // SAFETY: caller must ensure safety
        unsafe { self.push_inner_raw(bytes, &v[..len]) }
    }

    // SAFETY: TODO
    pub(super) unsafe fn push_inner_raw(
        &mut self,
        bytes: usize,
        metadata: &[usize],
    ) -> Result<PushInnerInfo<'_, BUF::Inner>, ()> {
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
        store_metadata(meta, metadata);

        // Increment offset and return
        Ok(PushInnerInfo {
            meta,
            data: rv,
            reset_slot: &mut self.write_pos,
            reset_value: prev_write_pos,
        })
    }

    #[must_use]
    pub(super) fn meta_words() -> usize {
        BUF::round_to_words(size_of::<&DST>() - size_of::<usize>())
    }

    #[must_use]
    fn space_words(&self) -> usize {
        self.data.as_ref().len() - self.write_pos
    }

    #[must_use]
    pub(super) fn front_raw(&self) -> *mut DST {
        assert!(self.read_pos < self.write_pos);
        // SAFETY: Internal consistency maintains the metadata validity.
        unsafe { self.raw_at(self.read_pos) }
    }

    #[must_use]
    // SAFETY: Caller must ensure that `pos` is the start of an object.
    pub(super) unsafe fn raw_at(&self, pos: usize) -> *mut DST {
        assert!(pos >= self.read_pos);
        assert!(pos < self.write_pos);
        let meta = &self.data.as_ref()[pos..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at(mw);
        // SAFETY: caller must ensure safety
        unsafe { make_fat_ptr(data.as_ptr() as *mut (), meta) }
    }

    #[must_use]
    pub(super) fn front_raw_mut(&mut self) -> *mut DST {
        assert!(self.read_pos < self.write_pos);
        // SAFETY: Internal consistency maintains the metadata validity.
        unsafe { self.raw_at_mut(self.read_pos) }
    }

    #[must_use]
    // SAFETY: Caller must ensure that `pos` is the start of an object.
    pub(super) unsafe fn raw_at_mut(&mut self, pos: usize) -> *mut DST {
        assert!(pos >= self.read_pos);
        assert!(pos < self.write_pos);
        let meta = &mut self.data.as_mut()[pos..];
        let mw = Self::meta_words();
        let (meta, data) = meta.split_at_mut(mw);
        // SAFETY: caller must ensure safety
        unsafe { make_fat_ptr(data.as_mut_ptr() as *mut (), meta) }
    }

    pub(super) fn pop_front_inner(&mut self) {
        // SAFETY: `front_raw_mut` asserts that there's an item, rest is correct.
        unsafe {
            let ptr = &mut *self.front_raw_mut();
            let len = size_of_val(ptr);
            Ptr::drop_in_place(ptr);
            let words = BUF::round_to_words(len);
            self.read_pos += Self::meta_words() + words;
        }
    }
}
