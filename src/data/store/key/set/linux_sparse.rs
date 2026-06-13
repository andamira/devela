// devela/src/data/store/key/set/linux_sparse.rs
//
//! Linux mmap-backed sparse sets.
//

use super::sparse::{check_consistency_raw, contains_raw, insert_raw, remove_raw};
use crate::{Libc, PtrNonNull, Slice, c_void, is};
use crate::{SparseSetError, SparseSetResult};
use core::slice::Iter as IterSlice;

#[doc = crate::_tags!(data_structure platform set)]
/// A Linux mmap-backed sparse set with lazily zeroed storage.
#[doc = crate::_doc_meta!{location("data/store/key")}]
///
/// The backing arrays are anonymous memory mappings, so Linux provides
/// zero-initialized virtual pages without Rust clearing every element first.
#[derive(Debug)]
pub struct LinuxSparseSet {
    dense: LinuxMmapUsize,
    sparse: LinuxMmapUsize,
    count: usize,
}
#[rustfmt::skip]
impl LinuxSparseSet {
    /// Creates a new Linux mmap-backed sparse set.
    ///
    /// `dense` is the maximum number of stored values.
    /// `sparse` is the size of the value universe.
    pub fn new(dense: usize, sparse: usize) -> SparseSetResult<Self> {
        Ok(Self {
            dense: LinuxMmapUsize::new_zeroed(dense, false)?,
            sparse: LinuxMmapUsize::new_zeroed(sparse, false)?,
            count: 0,
        })
    }
    /// Creates a new sparse set using `MAP_NORESERVE`.
    ///
    /// This can reserve a large virtual range, but writes may fail at the OS
    /// level if memory cannot be committed later.
    pub fn new_noreserve(dense: usize, sparse: usize) -> SparseSetResult<Self> {
        Ok(Self {
            dense: LinuxMmapUsize::new_zeroed(dense, true)?,
            sparse: LinuxMmapUsize::new_zeroed(sparse, true)?,
            count: 0,
        })
    }
    /// Returns the maximum number of values this set can store.
    #[must_use]
    pub const fn capacity(&self) -> usize { self.dense.len() }

    /// Returns the size of the value universe.
    #[must_use]
    pub const fn universe(&self) -> usize { self.sparse.len() }

    /// Returns the number of stored values.
    #[must_use]
    pub const fn len(&self) -> usize { self.count }

    /// Returns `true` if the set contains no values.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.count == 0 }

    /// Returns `true` if the dense storage is full.
    #[must_use]
    pub const fn is_full(&self) -> bool { self.count == self.dense.len() }

    /// Clears the set in constant time.
    pub const fn clear(&mut self) { self.count = 0; }

    /// Returns the live dense values.
    #[must_use]
    pub fn as_slice(&self) -> &[usize] { &self.dense.as_slice()[..self.count] }

    /// Returns an iterator over the stored values.
    pub fn iter(&self) -> IterSlice<'_, usize> { self.as_slice().iter() }

    /// Returns `true` if `value` is present.
    #[must_use]
    pub fn contains(&self, value: usize) -> bool {
        contains_raw(self.dense.as_slice(), self.sparse.as_slice(), self.count, value)
    }
    /// Inserts `value`.
    pub fn insert(&mut self, value: usize) -> SparseSetResult<bool> {
        insert_raw(self.dense.as_mut_slice(), self.sparse.as_mut_slice(), &mut self.count, value)
    }
    /// Removes `value`.
    pub fn remove(&mut self, value: usize) -> SparseSetResult<bool> {
        remove_raw(self.dense.as_mut_slice(), self.sparse.as_mut_slice(), &mut self.count, value)
    }
    /// Checks the internal dense/sparse consistency.
    pub fn check_consistency(&self) {
        check_consistency_raw(self.dense.as_slice(), self.sparse.as_slice(), self.count);
    }
}

/// Owned Linux mmap storage for `usize` values.
#[derive(Debug)]
struct LinuxMmapUsize {
    ptr: PtrNonNull<usize>,
    len: usize,
    bytes: usize,
}

impl LinuxMmapUsize {
    fn new_zeroed(len: usize, noreserve: bool) -> SparseSetResult<Self> {
        let bytes =
            len.checked_mul(size_of::<usize>()).ok_or(SparseSetError::AllocationTooLarge)?;
        is! { bytes == 0, return Ok(Self { ptr: PtrNonNull::dangling(), len, bytes }) }
        let mut flags = Libc::MAP_PRIVATE | Libc::MAP_ANONYMOUS;
        is! { noreserve, flags |= Libc::MAP_NORESERVE }
        // SAFETY:
        // - `addr = null` lets the kernel choose the address.
        // - `bytes > 0`.
        // - `fd = -1` and `offset = 0` are correct for anonymous mappings.
        // - On success, the returned mapping is readable, writable, and suitably aligned.
        let raw = unsafe {
            Libc::mmap(
                core::ptr::null_mut(),
                bytes,
                Libc::PROT_READ | Libc::PROT_WRITE,
                flags,
                -1,
                0,
            )
        };
        is! { Libc::is_map_failed(raw), return Err(SparseSetError::MapFailed) }
        let ptr = PtrNonNull::new(raw.cast::<usize>()).ok_or(SparseSetError::MapFailed)?;
        Ok(Self { ptr, len, bytes })
    }
    const fn len(&self) -> usize {
        self.len
    }
    fn as_slice(&self) -> &[usize] {
        // SAFETY:
        // - `ptr` points to a valid mmap allocation for `len * size_of::<usize>()` bytes,
        //   or is dangling with `len == 0`.
        // - The mapping is initialized to zero by Linux anonymous mmap.
        unsafe { Slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    fn as_mut_slice(&mut self) -> &mut [usize] {
        // SAFETY:
        // - same as `as_slice`.
        // - `&mut self` guarantees exclusive access to this owner.
        unsafe { Slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl Drop for LinuxMmapUsize {
    fn drop(&mut self) {
        if self.bytes != 0 {
            // SAFETY:
            // - `ptr` and `bytes` come from a successful `mmap`.
            // - this owner calls `munmap` at most once, on drop.
            let _ = unsafe { Libc::munmap(self.ptr.as_ptr().cast::<c_void>(), self.bytes) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_sparse_set_basic() {
        let mut set = LinuxSparseSet::new(5, 10_000).unwrap();
        assert_eq!(set.insert(3), Ok(true));
        assert_eq!(set.insert(5), Ok(true));
        assert_eq!(set.insert(5), Ok(false));
        assert!(set.contains(3));
        assert!(set.contains(5));
        assert!(!set.contains(2));
        assert!(!set.contains(11_000));
        assert_eq!(set.remove(3), Ok(true));
        assert!(!set.contains(3));
        assert_eq!(set.remove(500), Ok(false));
        set.check_consistency();
    }
    #[test]
    fn linux_sparse_set_clear() {
        let mut set = LinuxSparseSet::new(8, 1024).unwrap();
        assert_eq!(set.insert(10), Ok(true));
        assert_eq!(set.insert(20), Ok(true));
        set.clear();
        assert!(!set.contains(10));
        assert!(!set.contains(20));
        assert_eq!(set.len(), 0);
        assert_eq!(set.insert(20), Ok(true));
        assert!(set.contains(20));
        set.check_consistency();
    }
}
