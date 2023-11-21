// devela::data::cmp::fns::sort
//
//! sorting algorithms
//
// TOC
// - generic:
//   - sort_bubble
//   - sort_counting
//   - sort_counting_buf
//   - sort_insertion
//   - sort_merge
//   - sort_quick_lomuto
//   - sort_quick_hoare
//   - sort_quick_3way
//   - sort_selection
//   - sort_shaker
//
// - sint|uint:
//   - sort_bubble
//   - sort_insertion
//   - sort_selection

use crate::{
    mem::cswap,
    meta::{cfor, iif, paste, sf},
};
#[cfg(feature = "alloc")]
use ::_alloc::{collections::BTreeMap, vec::Vec};
use core::cmp::Ordering;

/* generic sort */

/// Sorts a `slice` using bubble sort.
///
/// # Examples
/// ```
/// use devela::math::sort_bubble;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_bubble(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_bubble<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in 0..slice.len() - i - 1 {
            iif![slice[j] > slice[j + 1]; slice.swap(j, j + 1)];
        }
    }
}

/// Sorts a `slice` using counting sort, and returns the ordered frequencies.
///
/// Counting sort is particularly efficient when the range of input values is
/// small compared to the number of elements to be sorted.
///
/// # Examples
/// ```
/// use devela::math::sort_counting;
///
/// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
/// let freq = sort_counting(&mut data);
/// assert_eq![&data, &[2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 64, 64]];
/// assert_eq![&freq, &[3, 7, 5, 2]];
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn sort_counting<T: Ord + Clone>(slice: &mut [T]) -> Vec<usize> {
    let mut counts = BTreeMap::new();
    // Calculate the frequencies and save them
    for item in slice.iter() {
        let count = counts.entry(item.clone()).or_insert(0);
        *count += 1;
    }
    let freq: Vec<usize> = counts.values().cloned().collect();
    // Reconstruct the sorted slice
    let mut i = 0;
    for (item, &count) in counts.iter() {
        for _ in 0..count {
            slice[i] = item.clone();
            i += 1;
        }
    }
    freq
}

/// Sorts a `slice` using counting sort, and writes the frequencies, without allocating.
///
/// Counting sort is particularly efficient when the range of input values is
/// small compared to the number of elements to be sorted.
///
/// This implementation makes the following assumptions:
/// - `values` contains all distinct values present in `slice`.
/// - `freq` and `values` are of the same length.
/// - `freq` only contains zeros.
///
/// Returns `None` if `values` does not contain a value present in the slice,
/// or if `slice` has more elements than `freq` can accommodate.
///
/// Note that the frequencies in `freq` will be in the order of the sorted
/// distinct elements in `values`.
///
/// # Examples
/// ```
/// use devela::math::sort_counting_buf;
///
/// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
/// let values = [64, 4, 2, 8];
/// let mut freq = [0; 4];
/// sort_counting_buf(&mut data, &mut freq, &values);
/// assert_eq![&data, &[64, 64, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 8, 8, 8, 8, 8]];
/// assert_eq![&freq, &[2, 7, 3, 5]];
/// ```
/// # Panics
/// Panics in debug if the length of `freq` and `values` is not the same.
#[inline]
pub fn sort_counting_buf<T>(slice: &mut [T], freq: &mut [T], values: &[T]) -> Option<()>
where
    T: Ord + Clone + TryInto<usize> + TryFrom<usize>,
{
    debug_assert_eq![freq.len(), values.len()];
    // Calculate the frequencies
    for item in slice.iter() {
        let index = values.iter().position(|x| x == item)?;
        let count: usize = freq[index].clone().try_into().ok()?;
        freq[index] = T::try_from(count + 1).ok()?;
    }
    // Reconstruct the sorted slice
    let mut i = 0;
    for (index, count) in freq.iter().enumerate() {
        for _ in 0_usize..(*count).clone().try_into().ok()? {
            if i >= slice.len() {
                return None; // Out of bounds
            }
            slice[i] = values[index].clone();
            i += 1;
        }
    }
    Some(())
}

/// Sorts a `slice` using insertion sort.
///
/// # Examples
/// ```
/// use devela::math::sort_insertion;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_insertion(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_insertion<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j, j - 1);
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
/// use devela::math::sort_merge;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_merge(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn sort_merge<T: Ord + Copy>(slice: &mut [T]) {
    let len = slice.len();
    let mut buffer = Vec::with_capacity(len);
    buffer.resize(len, slice[0]);
    sort_merge_internal(slice, &mut buffer);
}
#[inline]
#[cfg(feature = "alloc")]
fn sort_merge_internal<T: Ord + Copy>(slice: &mut [T], buffer: &mut [T]) {
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
fn sort_merge_merge<T: Ord + Copy>(left: &[T], right: &[T], slice: &mut [T]) {
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

/// Sorts a `slice` using quick sort with the Lomuto partition scheme.
///
/// It performs more swaps compared to the Hoare partition scheme.
///
/// # Examples
/// ```
/// use devela::math::sort_quick_lomuto;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_quick_lomuto(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_quick_lomuto<T: Ord>(slice: &mut [T]) {
    iif![slice.len() < 2; return];
    let ipivot = sort_quick_lomuto_partition(slice);
    sort_quick_lomuto(&mut slice[0..ipivot]);
    sort_quick_lomuto(&mut slice[ipivot + 1..]);
}
#[inline]
fn sort_quick_lomuto_partition<T: Ord>(slice: &mut [T]) -> usize {
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

/// Sorts a `slice` using quick sort with the Three way partition scheme.
///
/// It is more efficient when dealing with duplicate elements.
///
/// # Examples
/// ```
/// use devela::math::sort_quick_3way;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_quick_3way(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_quick_3way<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    iif![len < 2; return];
    let (lt, gt) = sort_quick_3way_partition(slice);
    sort_quick_3way(&mut slice[0..lt]);
    iif![gt < len; sort_quick_3way(&mut slice[gt..])];
}
#[inline]
fn sort_quick_3way_partition<T: Ord + Clone>(slice: &mut [T]) -> (usize, usize) {
    let len = slice.len();
    let ipivot = len / 2;
    let pivot = slice[ipivot].clone();
    let (mut lt, mut gt, mut i) = (0, len, 0);
    while i < gt {
        match slice[i].cmp(&pivot) {
            Ordering::Less => {
                slice.swap(lt, i);
                lt += 1;
                i += 1
            }
            Ordering::Greater => {
                gt -= 1;
                slice.swap(i, gt)
            }
            Ordering::Equal => i += 1,
        }
    }
    (lt, gt)
}

/// Sorts a `slice` using quick sort with the Hoare partition scheme.
///
/// It performs fewer swaps compared to the Lomuto partition scheme.
///
/// # Examples
/// ```
/// use devela::math::sort_quick_hoare;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_quick_hoare(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_quick_hoare<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    iif![len < 2; return];
    let ipivot = sort_quick_hoare_partition(slice);
    iif![ipivot > 0; sort_quick_hoare(&mut slice[0..ipivot])];
    iif![ipivot + 1 < len; sort_quick_hoare(&mut slice[ipivot + 1..])];
}
#[inline]
fn sort_quick_hoare_partition<T: Ord + Clone>(slice: &mut [T]) -> usize {
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

/// Sorts a `slice` using selection sort.
///
/// # Examples
/// ```
/// use devela::math::sort_selection;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_selection(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_selection<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len - 1 {
        let mut min_index = i;
        for j in (i + 1)..len {
            iif![slice[j] < slice[min_index]; min_index = j];
        }
        slice.swap(min_index, i);
    }
}

/// Sorts a `slice` using shaker sort.
///
/// Also known as cocktail sort and double quicksort.
///
/// # Examples
/// ```
/// use devela::math::sort_shaker;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// sort_shaker(&mut arr);
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[inline]
pub fn sort_shaker<T: Ord + Clone>(slice: &mut [T]) {
    let (mut swapped, mut start, mut end) = (true, 0, slice.len());
    while swapped {
        swapped = false;
        for i in start..end - 1 {
            iif![slice[i] > slice[i + 1]; { slice.swap(i, i + 1); swapped = true; }];
        }
        iif![!swapped; break];
        swapped = false;
        end -= 1;
        for i in (start..end - 1).rev() {
            iif![slice[i] > slice[i + 1]; { slice.swap(i, i + 1); swapped = true; }];
        }
        start += 1;
    }
}

// signed|unsigned
// $t:   the input/output type
macro_rules! impl_ops {
    (signed $( $t:ty ),+) => { $( impl_ops![@signed $t]; )+ };
    (unsigned $( $t:ty ),+) => { $( impl_ops![@unsigned $t]; )+ };

    // implements signed ops
    (@signed $t:ty) => { paste! {
        /* signed sort */

        #[doc = "Returns a copied sorted array of [`" $t "`] using bubble sort."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::sort_bubble_" $t ";\n\n"]
        /// let mut arr = [4, 7, -5, 1, -13, 0];
        #[doc = "assert_eq![sort_bubble_" $t "(arr), [-13, -5, 0, 1, 4, 7]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sort_bubble_ $t>]<const N: usize>(arr: [$t; N]) -> [$t; N] {
            let mut arr = arr;
            cfor![i in 0..N => {
                cfor![j in 0..N-i-1 => {
                    iif![arr[j] > arr[j+1]; cswap!(arr[j], arr[j+1])];
                }];
            }];
            arr
        }

        #[doc = "Returns a copied sorted array of [`" $t "`] using insertion sort."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::sort_insertion_" $t ";\n\n"]
        /// let mut arr = [4, 7, -5, 1, -13, 0];
        #[doc = "assert_eq![sort_insertion_" $t "(arr), [-13, -5, 0, 1, 4, 7]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sort_insertion_ $t>]<const N: usize>(arr: [$t; N]) -> [$t; N] {
            let mut arr = arr;
            cfor![i in 1..N => {
                let mut j = i;
                while j > 0 && arr[j-1] > arr[j] {
                    cswap![arr[j], arr[j-1]];
                    j -= 1;
                }
            }];
            arr
        }

        #[doc = "Returns a copied sorted array of [`" $t "`] using selection sort."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::sort_selection_" $t ";\n\n"]
        /// let mut arr = [4, 7, -5, 1, -13, 0];
        #[doc = "assert_eq![sort_selection_" $t "(arr), [-13, -5, 0, 1, 4, 7]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sort_selection_ $t>]<const N: usize>(arr: [$t; N]) -> [$t; N] {
            let mut arr = arr;
            cfor![i in 0..N-1 => {
                let mut min_index = i;
                cfor![j in (i+1)..N => {
                    iif![arr[j] < arr[min_index]; min_index = j];
                }];
                cswap![arr[min_index], arr[i]];
            }];
            arr
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty) => { paste! {
        /* unsigned sort */

        #[doc = "Returns a copied sorted array of [`" $t "`] using bubble sort."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::sort_bubble_" $t ";\n\n"]
        /// let mut arr = [4, 7, 5, 1, 13, 0];
        #[doc = "assert_eq![sort_bubble_" $t "(arr), [0, 1, 4, 5, 7, 13]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sort_bubble_ $t>]<const N: usize>(arr: [$t; N]) -> [$t; N] {
            let mut arr = arr;
            cfor![i in 0..N => {
                cfor![j in 0..N-i-1 => {
                    iif![arr[j] > arr[j+1]; cswap!(arr[j], arr[j+1])];
                }];
            }];
            arr
        }

        #[doc = "Returns a copied sorted array of [`" $t "`] using insertion sort."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::sort_insertion_" $t ";\n\n"]
        /// let mut arr = [4, 7, 5, 1, 13, 0];
        #[doc = "assert_eq![sort_insertion_" $t "(arr), [0, 1, 4, 5, 7, 13]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sort_insertion_ $t>]<const N: usize>(arr: [$t; N]) -> [$t; N] {
            let mut arr = arr;
            cfor![i in 1..N => {
                let mut j = i;
                while j > 0 && arr[j-1] > arr[j] {
                    cswap![arr[j], arr[j-1]];
                    j -= 1;
                }
            }];
            arr
        }

        #[doc = "Returns a copied sorted array of [`" $t "`] using selection sort."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::sort_selection_" $t ";\n\n"]
        /// let mut arr = [4, 7, 5, 1, 13, 0];
        #[doc = "assert_eq![sort_selection_" $t "(arr), [0, 1, 4, 5, 7, 13]];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<sort_selection_ $t>]<const N: usize>(arr: [$t; N]) -> [$t; N] {
            let mut arr = arr;
            cfor![i in 0..N-1 => {
                let mut min_index = i;
                cfor![j in (i+1)..N => {
                    iif![arr[j] < arr[min_index]; min_index = j];
                }];
                cswap![arr[min_index], arr[i]];
            }];
            arr
        }
    }};
}
impl_ops![signed i8, i16, i32, i64, i128, isize];
impl_ops![unsigned u8, u16, u32, u64, u128, usize];
