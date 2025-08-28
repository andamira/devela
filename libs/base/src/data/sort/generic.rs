// devela_base::data::sort::impl_generic
//
//! Implements sorting algorithms for exclusive generic arrays `[T: Ord; N]`.
//

use crate::{Sort, is};

impl<T: Ord> Sort<&mut [T]> {
    /// Sorts a slice using bubble sort.
    ///
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut data = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut data[..]).bubble();
    /// assert_eq![data, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn bubble(&mut self) {
        for i in 0..self.0.len() {
            for j in 0..self.0.len() - i - 1 {
                is![self.0[j] > self.0[j + 1]; self.0.swap(j, j + 1)];
            }
        }
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
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
    /// let values = [64, 4, 2, 8];
    /// let mut freq = [0; 4];
    /// Sort(&mut data[..]).counting_buf(&mut freq, &values);
    /// assert_eq![data, [64, 64, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 8, 8, 8, 8, 8]];
    /// assert_eq![freq, [2, 7, 3, 5]];
    /// ```
    /// # Panics
    /// Panics in debug if the length of `freq` and `values` is not the same.
    pub fn counting_buf(&mut self, freq: &mut [T], values: &[T]) -> Option<()>
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
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).insertion();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn insertion(&mut self) {
        for i in 1..self.0.len() {
            let mut j = i;
            while j > 0 && self.0[j - 1] > self.0[j] {
                self.0.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    /// Sorts a slice using selection sort.
    ///
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).selection();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn selection(&mut self) {
        let len = self.0.len();
        for i in 0..len - 1 {
            let mut min_index = i;
            for j in (i + 1)..len {
                is![self.0[j] < self.0[min_index]; min_index = j];
            }
            self.0.swap(min_index, i);
        }
    }

    /// Sorts a slice using shaker sort.
    ///
    /// Also known as cocktail sort and double quicksort.
    ///
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).shaker();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    pub fn shaker(&mut self)
    where
        T: Clone,
    {
        let (mut swapped, mut start, mut end) = (true, 0, self.0.len());
        while swapped {
            swapped = false;
            for i in start..end - 1 {
                is![self.0[i] > self.0[i + 1]; { self.0.swap(i, i + 1); swapped = true; }];
            }
            is![!swapped; break];
            swapped = false;
            end -= 1;
            for i in (start..end - 1).rev() {
                is![self.0[i] > self.0[i + 1]; { self.0.swap(i, i + 1); swapped = true; }];
            }
            start += 1;
        }
    }
}

// NOTE: The following quick_* methods use an internal associated function pattern
// in order to enable recursion while maintaining a clean method interface. This avoids
// multiple mutable borrows of `self` that would occur with direct recursive method calls.
impl<'a, T: Ord + 'a> Sort<&'a mut [T]> {
    /// Sorts a `slice` using quick sort with the Lomuto partition scheme.
    ///
    /// # Algorithm Characteristics
    /// - **Partitioning**: Uses a single pointer to maintain the partition boundary.
    /// - **Swaps**: Performs more swaps than Hoare scheme (typically 3× more).
    /// - **Pivot**: Always places the pivot in its final sorted position.
    ///
    /// # When to Use
    /// This variant is mostly for educational purposes.
    ///
    /// # Performance Comparison
    /// - vs Hoare: More swaps, simpler logic.
    /// - vs 3-way: Less efficient with duplicates, simpler implementation.
    ///
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).quick_lomuto();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline(always)] #[rustfmt::skip]
    pub fn quick_lomuto(&mut self) where T: Clone { Sort::quick_lomuto_internal(&mut self.0); }
    fn quick_lomuto_internal(slice: &mut [T]) {
        is![slice.len() < 2; return];
        let ipivot = helper::sort_quick_lomuto_partition(slice);
        Self::quick_lomuto_internal(&mut slice[0..ipivot]);
        Self::quick_lomuto_internal(&mut slice[ipivot + 1..]);
    }

    /// Sorts a `slice` using quick sort with the three-way partition scheme.
    ///
    /// # Algorithm Characteristics
    /// - **Partitioning**: Divides elements into three groups: less than, equal to,
    ///   and greater than the pivot.
    /// - **Duplicates**: Extremely efficient with many duplicate elements.
    /// - **Swaps**: Optimized to avoid swapping equal elements.
    /// - **Pivot**: All equal elements are placed in their final positions.
    ///
    /// # When to Use
    /// Ideal for data with many duplicate values or when you need to handle
    /// repeated elements efficiently.
    ///
    /// # Performance Comparison
    /// - vs Lomuto: Much better with duplicates, slightly more complex.
    /// - vs Hoare: Superior duplicate handling, similar performance on unique data.
    ///
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).quick_3way();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    /// Sorts the slice using quick sort with the three-way partition scheme.
    #[inline(always)] #[rustfmt::skip]
    pub fn quick_3way(&mut self) where T: Clone { Sort::quick_3way_internal(&mut self.0); }
    fn quick_3way_internal(slice: &mut [T])
    where
        T: Clone,
    {
        let len = slice.len();
        is![len < 2; return];
        let (lt, gt) = helper::sort_quick_3way_partition(slice);
        Self::quick_3way_internal(&mut slice[0..lt]);
        is![gt < len; Self::quick_3way_internal(&mut slice[gt..])];
    }

    /// Sorts a `slice` using quick sort with the Hoare partition scheme.
    ///
    /// # Algorithm Characteristics
    /// - **Partitioning**: Uses two pointers that move toward each other.
    /// - **Swaps**: Performs fewer swaps than Lomuto (typically 3× fewer).
    /// - **Pivot**: May not place the pivot in its final position initially.
    /// - **Cache**: Better cache performance due to locality of reference.
    ///
    /// # When to Use
    /// The preferred choice for general-purpose sorting where maximum performance
    /// is desired and data may contain few duplicates.
    ///
    /// # Performance Comparison
    /// - vs Lomuto: Fewer swaps, better cache performance, more complex.
    /// - vs 3-way: Better for unique data, less efficient with many duplicates.
    ///
    /// # Example
    /// ```
    /// # use devela_base::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).quick_hoare();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline(always)] #[rustfmt::skip]
    pub fn quick_hoare(&mut self) where T: Clone { Sort::quick_hoare_internal(&mut self.0); }
    fn quick_hoare_internal(slice: &mut [T])
    where
        T: Clone,
    {
        let len = slice.len();
        is![len < 2; return];
        let ipivot = helper::sort_quick_hoare_partition(slice);
        is![ipivot > 0; Self::quick_hoare_internal(&mut slice[0..ipivot])];
        is![ipivot + 1 < len; Self::quick_hoare_internal(&mut slice[ipivot + 1..])];
    }
}

// private helper fns
mod helper {
    use crate::{Ordering, is, sf};

    pub(super) fn sort_quick_lomuto_partition<T: Ord>(slice: &mut [T]) -> usize {
        let len = slice.len();
        let ipivot = len / 2;
        slice.swap(ipivot, len - 1);
        let mut i = 0;
        for j in 0..len - 1 {
            is![slice[j] <= slice[len - 1]; { slice.swap(i, j); i += 1; }];
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
            is![i >= j; return j];
            slice.swap(i, j);
        }
    }
}
