// devela::data::sort
//
//! Helpers for sorting.
//
// TOC
// - Sort definition
// - impl Sort for exclusive array
// - impl Sort for exclusive array with lifetime
// - helper fns
// - impl Sort for primitives

use crate::code::iif;
#[cfg(feature = "alloc")]
use crate::data::{BTreeMap, Vec};
#[cfg(_some_sort_float)]
use crate::num::Compare;
#[cfg(_some_sort)]
use crate::{
    code::{cfor, paste},
    mem::cswap,
};

/// Provides sorting methods for arrays and slices of `T`.
///
/// It implements the following methods for sorting exclusive slices:
/// [`bubble`][Sort#bubble],
/// [`counting`][Sort#counting],
/// [`counting_buf`][Sort#counting_buf],
/// [`insertion`][Sort#insertion],
/// [`merge`][Sort#merge],
/// [`quick_lomuto`][Sort#quick_lomuto],
/// [`quick_hoare`][Sort#quick_hoare],
/// [`quick_3way`][Sort#quick_3way],
/// [`quick_selection`][Sort#quick_selection],
/// [`quick_shaker`][Sort#quick_shaker].
///
/// # Features
/// When the corresponding feature `_sort_f[32|64]` or `_sort_[iu][8|16|32|64|128]` is enabled,
/// It implements the following *const* methods for sorting copied arrays of primitives:
/// `bubble_array`, `insertion_array`, `selection_array`.
/// In the case of floating-point primitives it uses total ordering and the
/// methods will only be const if the `unsafe_const` feature is enabled.
///
/// # Examples
/// Sort copied arrays of primitives:
/// ```
/// # use devela::data::Sort;
/// # #[cfg(feature = "_sort_i32")]
/// # {
/// let mut arr = [4i32, 7, -5, 1, -13, 0]; // signed primitives
/// assert_eq![Sort(arr).bubble_array(),    [-13, -5, 0, 1, 4, 7]];
/// assert_eq![Sort(arr).insertion_array(), [-13, -5, 0, 1, 4, 7]];
/// assert_eq![Sort(arr).selection_array(), [-13, -5, 0, 1, 4, 7]];
/// # }
///
/// # #[cfg(feature = "_sort_u32")]
/// # {
/// let mut arr = [4u32, 7, 5, 1, 13, 0]; // unsigned primitives
/// assert_eq![Sort(arr).bubble_array(),    [0, 1, 4, 5, 7, 13]];
/// assert_eq![Sort(arr).insertion_array(), [0, 1, 4, 5, 7, 13]];
/// assert_eq![Sort(arr).selection_array(), [0, 1, 4, 5, 7, 13]];
/// # }
///
/// # #[cfg(feature = "_sort_f32")]
/// # {
/// let mut arr = [4.01f32, 7.9, -5.4, 1.0, 0.0, -0.0]; // floating-point primitives
/// assert_eq![Sort(arr).bubble_array(),    [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// assert_eq![Sort(arr).insertion_array(), [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// assert_eq![Sort(arr).selection_array(), [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// # }
/// ```
///
/// # Performance
/// The `_array` suffixed methods calls the [`cswap`] macro using the xor swap
/// algorithm, excep for the floting-point version which uses a temporary variable.
///
/// [`cswap`]: crate::mem::cswap
#[repr(transparent)]
pub struct Sort<T>(pub T);

impl<T: Ord> Sort<&mut [T]> {
    /// Sorts a slice using bubble sort.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::Sort;
    /// let mut data = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut data[..]).bubble();
    /// assert_eq![data, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
    /// let freq = Sort(&mut data[..]).counting();
    /// assert_eq![data, [2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 64, 64]];
    /// assert_eq![freq, [3, 7, 5, 2]];
    /// ```
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
    /// # use devela::data::Sort;
    /// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
    /// let values = [64, 4, 2, 8];
    /// let mut freq = [0; 4];
    /// Sort(&mut data[..]).counting_buf(&mut freq, &values);
    /// assert_eq![data, [64, 64, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 8, 8, 8, 8, 8]];
    /// assert_eq![freq, [2, 7, 3, 5]];
    /// ```
    /// # Panics
    /// Panics in debug if the length of `freq` and `values` is not the same.
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).insertion();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).merge();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).selection();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort(&mut arr[..]).shaker();
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// // Sort(&mut arr[..]).quick_lomuto();
    /// Sort::quick_lomuto(&mut arr[..]);
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort::quick_3way(&mut arr);
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    /// # use devela::data::Sort;
    /// let mut arr = [4, 7, -5, 1, -13, 0];
    /// Sort::quick_hoare(&mut arr);
    /// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
    /// ```
    #[inline]
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
    use crate::code::{iif, sf};
    use core::cmp::Ordering;

    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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

/* impl Sort on primitives */

#[cfg(_some_sort)]
macro_rules! impl_sort {
    () => {
        impl_sort![signed i8:"_sort_i8", i16:"_sort_i16", i32:"_sort_i32",
            i64:"_sort_i64", i128:"_sort_i128", isize:"_sort_isize"];
        impl_sort![unsigned u8:"_sort_u8", u16:"_sort_u16", u32:"_sort_u32",
            u64:"_sort_u64", u128:"_sort_u128", usize:"_sort_usize"];
        impl_sort![float f32:"_sort_f32", f64:"_sort_f64"];
    };

    // $t: the input/output primitive type
    (signed   $( $t:ty : $cap:literal ),+) => { $( impl_sort![@signed $t:$cap]; )+ };
    (unsigned $( $t:ty : $cap:literal ),+) => { $( impl_sort![@unsigned $t:$cap]; )+ };
    (float    $( $t:ty : $cap:literal ),+) => { $( impl_sort![@float $t:$cap]; )+ };

    (@signed $t:ty : $cap:literal) => { paste! {
        /// Implement const sorting methods for arrays of primitives.
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap)]
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[inline] #[must_use]
            pub const fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        iif![arr[j] > arr[j+1]; cswap!(xor arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[inline] #[must_use]
            pub const fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && arr[j-1] > arr[j] {
                        cswap!(xor arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[inline] #[must_use]
            pub const fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        iif![arr[j] < arr[min_index]; min_index = j];
                    }];
                    cswap!(xor arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};

    (@unsigned $t:ty : $cap:literal) => { paste! {
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap)]
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[inline] #[must_use]
            pub const fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        iif![arr[j] > arr[j+1]; cswap!(xor arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[inline] #[must_use]
            pub const fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && arr[j-1] > arr[j] {
                        cswap!(xor arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using selection sort.
            #[inline] #[must_use]
            pub const fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        iif![arr[j] < arr[min_index]; min_index = j];
                    }];
                    cswap!(xor arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};

    (@float $t:ty : $cap:literal) => { paste! {
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap)]
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_data"), feature = "unsafe_const"))]
            pub const fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        iif![Compare(arr[j]).gt(arr[j+1]); cswap!(arr[j], arr[j+1])];
                    }];
                }];
                arr
            }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(any(feature = "safe_data", not(feature = "unsafe_const")))]
            pub fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        iif![Compare(arr[j]).gt(arr[j+1]); cswap!(arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_data"), feature = "unsafe_const"))]
            pub const fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && Compare(arr[j-1]).gt(arr[j]) {
                        cswap!(arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(any(feature = "safe_data", not(feature = "unsafe_const")))]
            pub fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && Compare(arr[j-1]).gt(arr[j]) {
                        cswap!(arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using selection sort.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_data"), feature = "unsafe_const"))]
            pub const fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        iif![Compare(arr[j]).lt(arr[min_index]); min_index = j];
                    }];
                    cswap!(arr[min_index], arr[i]);
                }];
                arr
            }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(any(feature = "safe_data", not(feature = "unsafe_const")))]
            pub fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        iif![Compare(arr[j]).lt(arr[min_index]); min_index = j];
                    }];
                    cswap!(arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};
}
#[cfg(_some_sort)]
impl_sort![];
