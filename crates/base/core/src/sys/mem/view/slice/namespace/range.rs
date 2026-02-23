// devela_base_core::sys::mem::view::slice::namespace::bytes

use crate::{Slice, is};

/// # `range*` API methods for subslicing.
impl<T> Slice<T> {
    // range_to

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..end]`.
    ///
    /// # Panics
    /// Panics if `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to(slice: &[T], end: usize) -> &[T] {
        slice.split_at(end).0
    }

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..end]`.
    ///
    /// Returns `None` if `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_checked(slice: &[T], end: usize) -> Option<&[T]> {
        match slice.split_at_checked(end) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_to_unchecked(slice: &[T], end: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(end).0 }
    }

    /// Returns an exclusive subslice up to the given `end` index.
    ///
    /// Equivalent to `&mut slice[..end]`.
    ///
    /// # Panics
    /// Panics if `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_mut(slice: &mut [T], end: usize) -> &mut [T] {
        slice.split_at_mut(end).0
    }

    /// Returns an exclusive subslice up to the given `end` index.
    ///
    /// Equivalent to `&mut slice[..end]`.
    ///
    /// Returns `None` if `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_mut_checked(slice: &mut [T], end: usize) -> Option<&mut [T]> {
        match slice.split_at_mut_checked(end) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }

    /// Returns an exclusive subslice up to the given `end` index.
    ///
    /// Equivalent to `&mut slice[..end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_to_mut_unchecked(slice: &mut [T], end: usize) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(end).0 }
    }

    // range_to_inclusive

    /// Returns a subslice up to and including the given `end` index.
    ///
    /// Equivalent to `&slice[..=end]`.
    ///
    /// # Panics
    /// Panics if `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive(slice: &[T], end: usize) -> &[T] {
        slice.split_at(end + 1).0
    }

    /// Returns a subslice up to and including the given `end` index.
    ///
    /// Equivalent to `&slice[..=end]`.
    ///
    /// Returns `None` if `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive_checked(slice: &[T], end: usize) -> Option<&[T]> {
        is![end < slice.len(), Some(slice.split_at(end + 1).0), None]
    }

    /// Returns an inclusive subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..=end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_to_inclusive_unchecked(slice: &[T], end: usize) -> &[T] {
        // SAFETY: The caller must guarantee that `end < slice.len()`
        unsafe { slice.split_at_unchecked(end + 1).0 }
    }

    /// Returns an exclusive subslice up to and including the given `end` index.
    ///
    /// Equivalent to `&slice[..=end]`.
    ///
    /// # Panics
    /// Panics if `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive_mut(slice: &mut [T], end: usize) -> &mut [T] {
        slice.split_at_mut(end + 1).0
    }

    /// Returns an exclusive subslice up to and including the given `end` index.
    ///
    /// Equivalent to `&slice[..=end]`.
    ///
    /// Returns `None` if `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive_mut_checked(slice: &mut [T], end: usize) -> Option<&mut [T]> {
        is![end < slice.len(), Some(slice.split_at_mut(end + 1).0), None]
    }

    /// Returns an inclusive subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..=end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_to_inclusive_mut_unchecked(slice: &mut [T], end: usize) -> &mut [T] {
        // SAFETY: The caller must guarantee that `end < slice.len()`
        unsafe { slice.split_at_mut_unchecked(end + 1).0 }
    }

    // range_from

    /// Returns a subslice starting from the given `start` index.
    ///
    /// Equivalent to `&slice[start..]`.
    ///
    /// # Panics
    /// Panics if `start` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_from(slice: &[T], start: usize) -> &[T] {
        slice.split_at(start).1
    }

    /// Returns a subslice starting from the given `start` index.
    ///
    /// Equivalent to `&slice[start..]`.
    ///
    /// Returns `None` if `start` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_from_checked(slice: &[T], start: usize) -> Option<&[T]> {
        match slice.split_at_checked(start) {
            Some((_, subslice)) => Some(subslice),
            None => None,
        }
    }

    /// Returns an exclusive subslice starting from the given `start` index.
    ///
    /// Equivalent to `&slice[start..]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `start` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_from_unchecked(slice: &[T], start: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(start).1 }
    }

    /// Returns an exclusive subslice starting from the given `start` index.
    ///
    /// Equivalent to `&mut slice[start..]`.
    ///
    /// # Panics
    /// Panics if `start` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_from_mut(slice: &mut [T], start: usize) -> &mut [T] {
        slice.split_at_mut(start).1
    }

    /// Returns an exclusive subslice starting from the given `start` index.
    ///
    /// Equivalent to `&mut slice[start..]`.
    ///
    /// Returns `None` if `start` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_from_mut_checked(slice: &mut [T], start: usize) -> Option<&mut [T]> {
        match slice.split_at_mut_checked(start) {
            Some((_, subslice)) => Some(subslice),
            None => None,
        }
    }

    /// Returns an exclusive subslice starting from the given `start` index.
    ///
    /// Equivalent to `&mut slice[start..]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `start` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_from_mut_unchecked(slice: &mut [T], start: usize) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(start).1 }
    }

    // range

    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&slice[start..end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range(slice: &[T], start: usize, end: usize) -> &[T] {
        slice.split_at(start).1.split_at(end - start).0
    }

    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&slice[start..end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    #[inline(always)]
    pub const fn range_checked(slice: &[T], start: usize, end: usize) -> Option<&[T]> {
        if start <= end && end <= slice.len() {
            #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
            return Some(slice.split_at(start).1.split_at(end - start).0);

            #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
            // SAFETY: `start` and `end` are checked to be within bounds and valid
            Some(unsafe { slice.split_at_unchecked(start).1.split_at_unchecked(end - start).0 })
        } else {
            None
        }
    }

    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&slice[start..end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `start` > `slice.len()` or `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_unchecked(slice: &[T], start: usize, end: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(start).1.split_at_unchecked(end - start).0 }
    }

    /// Returns an exclusive subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut slice[start..end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_mut(slice: &mut [T], start: usize, end: usize) -> &mut [T] {
        slice.split_at_mut(start).1.split_at_mut(end - start).0
    }

    /// Returns an exclusive subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut slice[start..end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    #[inline(always)]
    pub const fn range_mut_checked(slice: &mut [T], start: usize, end: usize) -> Option<&mut [T]> {
        if start <= end && end <= slice.len() {
            #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
            return Some(slice.split_at_mut(start).1.split_at_mut(end - start).0);
            #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
            // SAFETY: `start` and `end` are checked to be within bounds and valid
            Some(unsafe {
                slice.split_at_mut_unchecked(start).1.split_at_mut_unchecked(end - start).0
            })
        } else {
            None
        }
    }

    /// Returns an exclusive subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut slice[start..end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `start` > `end` or `end` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_mut_unchecked(slice: &mut [T], start: usize, end: usize) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(start).1.split_at_mut_unchecked(end - start).0 }
    }

    // range_inclusive

    /// Returns a subslice from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&slice[start..=end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive(slice: &[T], start: usize, end: usize) -> &[T] {
        slice.split_at(start).1.split_at(end - start + 1).0
    }

    /// Returns a subslice from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&slice[start..=end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` >= `slice.len()`.
    ///
    /// # Features
    /// This method makes use of the `unsafe_slice` feature if enabled.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive_checked(slice: &[T], start: usize, end: usize) -> Option<&[T]> {
        if start <= end && end < slice.len() {
            #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
            return Some(slice.split_at(start).1.split_at(end - start + 1).0);

            #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
            // SAFETY: `start` and `end` are checked to be within bounds and valid
            Some(unsafe { slice.split_at_unchecked(start).1.split_at_unchecked(end - start + 1).0 })
        } else {
            None
        }
    }

    /// Returns a subslice from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&slice[start..=end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `start` > `slice.len()` or `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_inclusive_unchecked(slice: &[T], start: usize, end: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(start).1.split_at_unchecked(end - start + 1).0 }
    }

    /// Returns a subslice from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&mut slice[start..=end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive_mut(slice: &mut [T], start: usize, end: usize) -> &mut [T] {
        slice.split_at_mut(start).1.split_at_mut(end - start + 1).0
    }

    /// Returns a subslice from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&mut slice[start..=end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` >= `slice.len()`.
    ///
    /// # Features
    /// This method makes use of the `unsafe_slice` feature if enabled.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive_mut_checked(
        slice: &mut [T],
        start: usize,
        end: usize,
    ) -> Option<&mut [T]> {
        if start <= end && end < slice.len() {
            #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
            return Some(slice.split_at_mut(start).1.split_at_mut(end - start + 1).0);

            #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
            // SAFETY: `start` and `end` are checked to be within bounds and valid
            Some(unsafe {
                slice.split_at_mut_unchecked(start).1.split_at_mut_unchecked(end - start + 1).0
            })
        } else {
            None
        }
    }

    /// Returns a subslice from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&mut slice[start..=end]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `start` > `slice.len()` or `end` >= `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn range_inclusive_mut_unchecked(
        slice: &mut [T],
        start: usize,
        end: usize,
    ) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(start).1.split_at_mut_unchecked(end - start + 1).0 }
    }
}
