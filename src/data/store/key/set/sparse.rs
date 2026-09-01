// devela/src/data/store/key/set/sparse.rs
//
//! Dense/sparse integer sets.
//

use crate::{ConstInit, is, slice, whilst};

// Result alias for sparse-set operations.
pub(crate) type SparseSetResult<T> = Result<T, SparseSetError>;

// RETHINK
#[doc = crate::_tags!(data_structure error set)]
/// Sparse-set operation error.
#[doc = crate::_doc_meta!{
    location("data/store/key/set", enum SparseSetError),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SparseSetError {
    /// The value is outside the sparse universe.
    ValueOutOfBounds,

    /// The dense storage is full.
    Full,

    /// The requested backing storage is too large.
    AllocationTooLarge,

    /// The backing memory mapping failed.
    MapFailed,
}

#[doc = crate::_tags!(data_structure set)]
/// A sparse integer set with constant-time insertion, removal, lookup and clearing.
#[doc = crate::_doc_meta!{
    location("data/store/key/set", struct SparseSetArray),
    #[cfg(target_pointer_width = "32")]
    test_size_of(SparseSetArray<8, 8> = 68|544; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(SparseSetArray<8, 8> = 136|1088; niche !Option),
}]
/// Values are `usize`s in the range `0..SPARSE`.
///
/// `DENSE` is the maximum number of stored values.
/// `SPARSE` is the size of the membership universe.
#[derive(Clone, Debug)]
pub struct SparseSetArray<const DENSE: usize, const SPARSE: usize> {
    /// Stores the live values in `dense[..count]`.
    dense: [usize; DENSE],
    /// Maps each value to its candidate dense index.
    sparse: [usize; SPARSE],
    /// Number of live values in `dense`.
    count: usize,
}
#[rustfmt::skip]
impl<const DENSE: usize, const SPARSE: usize> SparseSetArray<DENSE, SPARSE> {
    /// Creates a new empty sparse set.
    #[must_use]
    pub const fn new() -> Self { Self { dense: [0; DENSE], sparse: [0; SPARSE], count: 0 } }

    /// Returns the maximum number of values this set can store.
    #[must_use]
    pub const fn capacity(&self) -> usize { DENSE }

    /// Returns the size of the value universe.
    #[must_use]
    pub const fn universe(&self) -> usize { SPARSE }

    /// Returns the number of stored values.
    #[must_use]
    pub const fn len(&self) -> usize { self.count }

    /// Returns `true` if the set contains no values.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.count == 0 }

    /// Returns `true` if the dense storage is full.
    #[must_use]
    pub const fn is_full(&self) -> bool { self.count == DENSE }

    /// Clears the set in constant time.
    ///
    /// This does not clear the dense or sparse arrays.
    pub const fn clear(&mut self) { self.count = 0; }

    /// Returns the live dense values.
    #[must_use]
    pub const fn as_slice(&self) -> &[usize] { slice![&self.dense, ..self.count] }

    /// Returns an iterator over the stored values.
    pub fn iter(&self) -> core::slice::Iter<'_, usize> { self.as_slice().iter() }

    /// Returns `true` if `value` is present.
    #[must_use]
    pub const fn contains(&self, value: usize) -> bool {
        contains_raw(&self.dense, &self.sparse, self.count, value)
    }
    /// Inserts `value`.
    ///
    /// Returns `Ok(true)` if the value was newly inserted.
    /// Returns `Ok(false)` if the value was already present.
    pub const fn insert(&mut self, value: usize) -> SparseSetResult<bool> {
        insert_raw(&mut self.dense, &mut self.sparse, &mut self.count, value)
    }
    /// Removes `value`.
    ///
    /// Returns `Ok(true)` if the value was present.
    /// Returns `Ok(false)` if the value was absent.
    pub const fn remove(&mut self, value: usize) -> SparseSetResult<bool> {
        remove_raw(&mut self.dense, &mut self.sparse, &mut self.count, value)
    }
    /// Checks the internal dense/sparse consistency.
    ///
    /// # Panics
    /// Panics if the internal invariant is broken.
    pub const fn check_consistency(&self) {
        check_consistency_raw(&self.dense, &self.sparse, self.count);
    }
}
impl<const DENSE: usize, const SPARSE: usize> ConstInit for SparseSetArray<DENSE, SPARSE> {
    const INIT: Self = Self::new();
}
impl<const DENSE: usize, const SPARSE: usize> Default for SparseSetArray<DENSE, SPARSE> {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) const fn contains_raw(
    dense: &[usize],
    sparse: &[usize],
    count: usize,
    value: usize,
) -> bool {
    is! { value >= sparse.len(), return false }
    let index = sparse[value];
    index < count && dense[index] == value
}

pub(super) const fn insert_raw(
    dense: &mut [usize],
    sparse: &mut [usize],
    count: &mut usize,
    value: usize,
) -> SparseSetResult<bool> {
    is! { value >= sparse.len(), return Err(SparseSetError::ValueOutOfBounds) }
    is! { contains_raw(dense, sparse, *count, value), return Ok(false) }
    is! { *count == dense.len(), return Err(SparseSetError::Full) }
    dense[*count] = value;
    sparse[value] = *count;
    *count += 1;
    Ok(true)
}
pub(super) const fn remove_raw(
    dense: &mut [usize],
    sparse: &mut [usize],
    count: &mut usize,
    value: usize,
) -> SparseSetResult<bool> {
    is! { value >= sparse.len(), return Err(SparseSetError::ValueOutOfBounds) }
    is! { !contains_raw(dense, sparse, *count, value), return Ok(false) }
    let index = sparse[value];
    let last_index = *count - 1;
    let last_value = dense[last_index];
    dense[index] = last_value;
    sparse[last_value] = index;
    *count = last_index;
    Ok(true)
}
pub(super) const fn check_consistency_raw(dense: &[usize], sparse: &[usize], count: usize) {
    assert!(count <= dense.len());
    whilst! { index in 0..count; {
        let value = dense[index];
        assert!(value < sparse.len());
        assert!(sparse[value] == index);
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sparse_set() {
        let mut set = SparseSetArray::<5, 10_000>::new();
        assert_eq!(set.insert(3), Ok(true));
        assert_eq!(set.insert(5), Ok(true));
        assert_eq!(set.insert(5), Ok(false));
        assert!(set.contains(3));
        assert!(set.contains(5));
        assert!(!set.contains(2));
        assert!(!set.contains(11_000));
        assert_eq!(set.as_slice().len(), 2);
        set.check_consistency();
        assert_eq!(set.remove(3), Ok(true));
        assert!(!set.contains(3));
        assert!(set.contains(5));
        assert_eq!(set.remove(500), Ok(false));
        assert_eq!(set.insert(20_000), Err(SparseSetError::ValueOutOfBounds));
        assert_eq!(set.remove(20_000), Err(SparseSetError::ValueOutOfBounds));
        set.check_consistency();
    }
    #[test]
    fn clear_is_constant_time_and_stale_sparse_is_harmless() {
        let mut set = SparseSetArray::<4, 16>::new();
        assert_eq!(set.insert(3), Ok(true));
        assert_eq!(set.insert(7), Ok(true));
        assert!(set.contains(3));
        assert!(set.contains(7));
        set.clear();
        assert_eq!(set.len(), 0);
        assert!(!set.contains(3));
        assert!(!set.contains(7));
        assert_eq!(set.insert(7), Ok(true));
        assert!(set.contains(7));
        assert!(!set.contains(3));
        set.check_consistency();
    }
    #[test]
    fn dense_capacity_is_checked() {
        let mut set = SparseSetArray::<2, 16>::new();
        assert_eq!(set.insert(1), Ok(true));
        assert_eq!(set.insert(2), Ok(true));
        assert_eq!(set.insert(3), Err(SparseSetError::Full));
    }
}
