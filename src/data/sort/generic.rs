// devela::data::sort::impl_generic
//
//! Implements sorting algorithms for exclusive generic arrays `[T: Ord; N]`.
//

use crate::{iif, Sort};
#[cfg(feature = "alloc")]
use crate::{BTreeMap, Vec};

impl<T: Ord> Sort<&mut [T]> {
    /// Sorts a slice using bubble sort.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut data = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut data[..]).bubble();
    /// assert_eq![data, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn bubble(self) {
        for i in 0..self.0.len() {
            for j in 0..self.0.len() - i - 1 {
                iif![self.0[j] > self.0[j + 1]; self.0.swap(j, j + 1)];
            }
        }
    }

    /// Sorts a slice using counting sort, and returns the ordered frequencies.
    ///
    /// Counting sort is particularly efficient when the range of input values is
    /// small compared to the number of elements to be sorted.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
    /// let freq = Sort(&mut data[..]).counting();
    /// assert_eq![data, [2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 64, 64]];
    /// assert_eq![freq, [3, 7, 5, 2]];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn counting(self) -> Vec<usize>
    where
        T: Clone,
    {
        let mut counts = BTreeMap::new();
        // Calculate the frequencies and save them
        for item in self.0.iter() {
            let count = counts.entry(item.clone()).or_insert(0);
            *count += 1;
        }
        let freq: Vec<usize> = counts.values().copied().collect();
        // Reconstruct the sorted slice
        let mut i = 0;
        for (item, &count) in counts.iter() {
            for _ in 0..count {
                self.0[i] = item.clone();
                i += 1;
            }
        }
        freq
    }

    /// Sorts a slice using counting sort, and writes the frequencies, without allocating.
    ///
    /// Counting sort is particularly efficient when the range of input values is
    /// small compared to the number of elements to be sorted.
    ///
    /// This implementation makes the following assumptions:
    /// - `values` contains all distinct values present in `self`.
    /// - `freq` and `values` are of the same length.
    /// - `freq` only contains zeros.
    ///
    /// Returns `None` if `values` does not contain a value present in `self`,
    /// or if `self` has more elements than `freq` can accommodate.
    ///
    /// Note that the frequencies in `freq` will be in the order of the sorted
    /// distinct elements in `values`.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
    /// let values = [64, 4, 2, 8];
    /// let mut freq = [0; 4];
    /// Sort(&mut data[..]).counting_buf(&mut freq, &values);
    /// assert_eq![data, [64, 64, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 8, 8, 8, 8, 8]];
    /// assert_eq![freq, [2, 7, 3, 5]];
    /// ```
    /// # Panics
    /// Panics in debug if the length of `freq` and `values` is not the same.
    pub fn counting_buf(self, freq: &mut [T], values: &[T]) -> Option<()>
    where
        T: Clone + TryInto<usize> + TryFrom<usize>,
    {
        debug_assert_eq![freq.len(), values.len()];
        // Calculate the frequencies
        for item in self.0.iter() {
            let index = values.iter().position(|x| x == item)?;
            let count: usize = freq[index].clone().try_into().ok()?;
            freq[index] = T::try_from(count + 1).ok()?;
        }
        // Reconstruct the sorted slice
        let mut i = 0;
        for (index, count) in freq.iter().enumerate() {
            for _ in 0_usize..(*count).clone().try_into().ok()? {
                if i >= self.0.len() {
                    return None; // Out of bounds
                }
                self.0[i] = values[index].clone();
                i += 1;
            }
        }
        Some(())
    }

    /// Sorts a slice using insertion sort.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).insertion();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn insertion(self) {
        for i in 1..self.0.len() {
            let mut j = i;
            while j > 0 && self.0[j - 1] > self.0[j] {
                self.0.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    /// Sorts a `slice` using merge sort.
    ///
    /// It allocates one vector for the entire sort operation.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).merge();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn merge(self)
    where
        T: Copy,
    {
        let len = self.0.len();
        let mut buffer = Vec::with_capacity(len);
        buffer.resize(len, self.0[0]);
        helper::sort_merge_internal(self.0, &mut buffer);
    }

    /// Sorts a slice using selection sort.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).selection();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn selection(self) {
        let len = self.0.len();
        for i in 0..len - 1 {
            let mut min_index = i;
            for j in (i + 1)..len {
                iif![self.0[j] < self.0[min_index]; min_index = j];
            }
            self.0.swap(min_index, i);
        }
    }

    /// Sorts a slice using shaker sort.
    ///
    /// Also known as cocktail sort and double quicksort.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).shaker();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn shaker(self)
    where
        T: Clone,
    {
        let (mut swapped, mut start, mut end) = (true, 0, self.0.len());
        while swapped {
            swapped = false;
            for i in start..end - 1 {
                iif![self.0[i] > self.0[i + 1]; { self.0.swap(i, i + 1); swapped = true; }];
            }
            iif![!swapped; break];
            swapped = false;
            end -= 1;
            for i in (start..end - 1).rev() {
                iif![self.0[i] > self.0[i + 1]; { self.0.swap(i, i + 1); swapped = true; }];
            }
            start += 1;
        }
    }
}

impl<'a, T: Ord + 'a> Sort<&'a mut [T]> {
    /// Sorts a `slice` using quick sort with the Lomuto partition scheme.
    ///
    /// It performs more swaps compared to the Hoare partition scheme.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// // Sort(&mut arr[..]).quick_lomuto();
    /// Sort::quick_lomuto(&mut arr[..]);
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn quick_lomuto(slice: &mut [T]) {
        iif![slice.len() < 2; return];
        let ipivot = helper::sort_quick_lomuto_partition(slice);
        Self::quick_lomuto(&mut slice[0..ipivot]);
        Self::quick_lomuto(&mut slice[ipivot + 1..]);
    }
    // NOTE: can't use self because of multiple mutable borrows
    // pub fn quick_lomuto(self) {
    //     iif![self.0.len() < 2; return];
    //     let ipivot = helper::sort_quick_lomuto_partition(self.0);
    //     Self(&mut self.0[0..ipivot]).quick_lomuto();
    //     Self(&mut self.0[ipivot + 1..]).quick_lomuto();
    // }

    /// Sorts a `slice` using quick sort with the Three way partition scheme.
    ///
    /// It is more efficient when dealing with duplicate elements.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort::quick_3way(&mut arr);
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn quick_3way(slice: &mut [T])
    where
        T: Clone,
    {
        let len = slice.len();
        iif![len < 2; return];
        let (lt, gt) = helper::sort_quick_3way_partition(slice);
        Self::quick_3way(&mut slice[0..lt]);
        iif![gt < len; Self::quick_3way(&mut slice[gt..])];
    }
    // NOTE: can't use self because of multiple mutable borrows
    // pub fn quick_3way(self) where T: Clone {
    //     let len = self.0.len();
    //     iif![len < 2; return];
    //     let (lt, gt) = helper::sort_quick_3way_partition(self.0);
    //     Self(&mut self.0[0..lt]).quick_3way();
    //     iif![gt < len; Self(&mut self.0[gt..]).quick_3way()];
    // }

    /// Sorts a `slice` using quick sort with the Hoare partition scheme.
    ///
    /// It performs fewer swaps compared to the Lomuto partition scheme.
    ///
    /// # Examples
    /// ```
    /// # use devela::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort::quick_hoare(&mut arr);
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn quick_hoare(slice: &mut [T])
    where
        T: Clone,
    {
        let len = slice.len();
        iif![len < 2; return];
        let ipivot = helper::sort_quick_hoare_partition(slice);
        iif![ipivot > 0; Self::quick_hoare(&mut slice[0..ipivot])];
        iif![ipivot + 1 < len; Self::quick_hoare(&mut slice[ipivot + 1..])];
    }
    // NOTE: can't use self because of multiple mutable borrows
    // pub fn quick_hoare(self) where T: Clone {
    //     let len = self.0.len();
    //     iif![len < 2; return];
    //     let ipivot = helper::sort_quick_hoare_partition(self.0);
    //     iif![ipivot > 0; Self(&mut self.0[0..ipivot]).quick_hoare()];
    //     iif![ipivot + 1 < len; Self(&mut self.0[ipivot + 1..]).quick_hoare()];
    // }
}

// private helper fns
mod helper {
    use crate::{iif, sf, Ordering};

    // MAYBE WAIT:1.87 [const_copy_from_slice](https://github.com/rust-lang/rust/issues/131415)
    #[cfg(feature = "alloc")]
    pub(super) fn sort_merge_internal<T: Ord + Copy>(slice: &mut [T], buffer: &mut [T]) {
        let len = slice.len();
        iif![len <= 1; return];
        let mid = len / 2;
        sort_merge_internal(&mut slice[..mid], buffer);
        sort_merge_internal(&mut slice[mid..], buffer);
        sort_merge_merge(&slice[..mid], &slice[mid..], &mut buffer[..len]);
        slice.copy_from_slice(&buffer[..len]);
    }
    #[cfg(feature = "alloc")]
    pub(super) fn sort_merge_merge<T: Ord + Copy>(left: &[T], right: &[T], slice: &mut [T]) {
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            iif![ left[i] < right[j] ;
                { slice[k] = left[i]; i += 1; } ;
                { slice[k] = right[j]; j += 1; }
            ];
            k += 1;
        }
        iif![i < left.len(); slice[k..].copy_from_slice(&left[i..])];
        iif![j < right.len(); slice[k..].copy_from_slice(&right[j..])];
    }
    pub(super) fn sort_quick_lomuto_partition<T: Ord>(slice: &mut [T]) -> usize {
        let len = slice.len();
        let ipivot = len / 2;
        slice.swap(ipivot, len - 1);
        let mut i = 0;
        for j in 0..len - 1 {
            iif![slice[j] <= slice[len - 1]; { slice.swap(i, j); i += 1; }];
        }
        slice.swap(i, len - 1);
        i
    }
    pub(super) fn sort_quick_3way_partition<T: Ord + Clone>(slice: &mut [T]) -> (usize, usize) {
        let len = slice.len();
        let ipivot = len / 2;
        let pivot = slice[ipivot].clone();
        let (mut lt, mut gt, mut i) = (0, len, 0);
        while i < gt {
            match slice[i].cmp(&pivot) {
                Ordering::Less => {
                    slice.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Ordering::Greater => {
                    gt -= 1;
                    slice.swap(i, gt);
                }
                Ordering::Equal => i += 1,
            }
        }
        (lt, gt)
    }
    pub(super) fn sort_quick_hoare_partition<T: Ord + Clone>(slice: &mut [T]) -> usize {
        let len = slice.len();
        let ipivot = len / 2;
        let pivot = slice[ipivot].clone();
        let (mut i, mut j) = (0, len - 1);
        loop {
            sf! {
                while slice[i] < pivot { i += 1; }
                while slice[j] > pivot { j -= 1; }
            }
            iif![i >= j; return j];
            slice.swap(i, j);
        }
    }
}
