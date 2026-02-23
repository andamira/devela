// devela_base_alloc::data::layout::sort::impl_generic
//
//! Implements sorting algorithms for exclusive generic arrays `[T: Ord; N]`.
//

use crate::{BTreeMap, SortAlloc, Vec};

impl<T: Ord> SortAlloc<&mut [T]> {
    /// Sorts a slice using counting sort, and returns the ordered frequencies.
    ///
    /// Counting sort is particularly efficient when the range of input values is
    /// small compared to the number of elements to be sorted.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_alloc::SortAlloc;
    /// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
    /// let freq = SortAlloc::new(&mut data[..]).counting();
    /// assert_eq![data, [2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 64, 64]];
    /// assert_eq![freq, [3, 7, 5, 2]];
    /// ```
    pub fn counting(self) -> Vec<usize>
    where
        T: Clone,
    {
        let mut counts = BTreeMap::new();
        // Calculate the frequencies and save them
        for item in self.0.0.iter() {
            let count = counts.entry(item.clone()).or_insert(0);
            *count += 1;
        }
        let freq: Vec<usize> = counts.values().copied().collect();
        // Reconstruct the sorted slice
        let mut i = 0;
        for (item, &count) in counts.iter() {
            for _ in 0..count {
                self.0.0[i] = item.clone();
                i += 1;
            }
        }
        freq
    }

    /// Sorts a `slice` using merge sort.
    ///
    /// It allocates one vector for the entire sort operation.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_alloc::SortAlloc;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// SortAlloc::new(&mut arr[..]).merge();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn merge(self)
    where
        T: Copy,
    {
        let len = self.0.0.len();
        let mut buffer = Vec::with_capacity(len);
        buffer.resize(len, self.0.0[0]);
        helper::sort_merge_internal(self.0.0, &mut buffer);
    }
}

// private helper fns
mod helper {
    use crate::is;

    pub(super) fn sort_merge_internal<T: Ord + Copy>(slice: &mut [T], buffer: &mut [T]) {
        let len = slice.len();
        is![len <= 1, return];
        let mid = len / 2;
        sort_merge_internal(&mut slice[..mid], buffer);
        sort_merge_internal(&mut slice[mid..], buffer);
        sort_merge_merge(&slice[..mid], &slice[mid..], &mut buffer[..len]);
        slice.copy_from_slice(&buffer[..len]);
    }
    pub(super) fn sort_merge_merge<T: Ord + Copy>(left: &[T], right: &[T], slice: &mut [T]) {
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            is![
                left[i] < right[j],
                {
                    slice[k] = left[i];
                    i += 1;
                },
                {
                    slice[k] = right[j];
                    j += 1;
                }
            ];
            k += 1;
        }
        is![i < left.len(), slice[k..].copy_from_slice(&left[i..])];
        is![j < right.len(), slice[k..].copy_from_slice(&right[j..])];
    }
}
