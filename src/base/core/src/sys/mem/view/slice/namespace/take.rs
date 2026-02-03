// devela_base_core::sys::mem::view::slice::namespace::bytes

use crate::Slice;

/// # `take*` API methods for subslicing.
impl<T> Slice<T> {
    // take_first

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&slice[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_first(slice: &[T], n: usize) -> &[T] {
        slice.split_at(n).0
    }

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&slice[..n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_first_checked(slice: &[T], n: usize) -> Option<&[T]> {
        match slice.split_at_checked(n) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&slice[..n]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn take_first_unchecked(slice: &[T], n: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(n).0 }
    }

    /// Returns the first `n` elements of the exclusive slice.
    ///
    /// Equivalent to `&mut slice[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_first_mut(slice: &mut [T], n: usize) -> &mut [T] {
        slice.split_at_mut(n).0
    }

    /// Returns the first `n` elements of the exclusive slice.
    ///
    /// Equivalent to `&mut slice[..n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_first_mut_checked(slice: &mut [T], n: usize) -> Option<&mut [T]> {
        match slice.split_at_mut_checked(n) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }

    /// Returns the first `n` elements of the exclusive slice.
    ///
    /// Equivalent to `&mut slice[..n]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn take_first_mut_unchecked(slice: &mut [T], n: usize) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(n).0 }
    }

    // take_last

    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&slice[slice.len() - n..]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_last(slice: &[T], n: usize) -> &[T] {
        slice.split_at(slice.len() - n).1
    }

    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&slice[slice.len() - n..]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    #[inline(always)]
    pub const fn take_last_checked(slice: &[T], n: usize) -> Option<&[T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(base_safe_mem, not(feature = "unsafe_slice")))]
                return Some(slice.split_at(index).1);
                #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_unchecked(index).1 });
            }
            None => None,
        }
    }

    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&slice[slice.len() - n..]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn take_last_unchecked(slice: &[T], n: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(slice.len() - n).1 }
    }

    /// Returns the last `n` elements of the exclusive slice.
    ///
    /// Equivalent to `&mut slice[slice.len() - n..]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_last_mut(slice: &mut [T], n: usize) -> &mut [T] {
        slice.split_at_mut(slice.len() - n).1
    }

    /// Returns the last `n` elements of the exclusive slice.
    ///
    /// Equivalent to `&mut slice[slice.len() - n..]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    #[inline(always)]
    pub const fn take_last_mut_checked(slice: &mut [T], n: usize) -> Option<&mut [T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(base_safe_mem, not(feature = "unsafe_slice")))]
                return Some(slice.split_at_mut(index).1);
                #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_mut_unchecked(index).1 });
            }
            None => None,
        }
    }

    /// Returns the last `n` elements of the exclusive slice.
    ///
    /// Equivalent to `&mut slice[slice.len() - n..]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn take_last_mut_unchecked(slice: &mut [T], n: usize) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(slice.len() - n).1 }
    }

    // take_omit_last

    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&slice[..slice.len() - n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last(slice: &[T], n: usize) -> &[T] {
        slice.split_at(slice.len() - n).0
    }

    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&slice[..slice.len() - n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last_checked(slice: &[T], n: usize) -> Option<&[T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(base_safe_mem, not(feature = "unsafe_slice")))]
                return Some(slice.split_at(index).0);
                #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_unchecked(index).0 });
            }
            None => None,
        }
    }

    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&slice[..slice.len() - n]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const fn take_omit_last_unchecked(slice: &[T], n: usize) -> &[T] {
        unsafe { slice.split_at_unchecked(slice.len() - n).0 }
    }

    /// Returns the exclusive slice omitting the last `n` elements.
    ///
    /// Equivalent to `&mut slice[..slice.len() - n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last_mut(slice: &mut [T], n: usize) -> &mut [T] {
        slice.split_at_mut(slice.len() - n).0
    }

    /// Returns the exclusive slice omitting the last `n` elements.
    ///
    /// Equivalent to `&mut slice[..slice.len() - n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last_mut_checked(slice: &mut [T], n: usize) -> Option<&mut [T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(base_safe_mem, not(feature = "unsafe_slice")))]
                return Some(slice.split_at_mut(index).0);
                #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_mut_unchecked(index).0 });
            }
            None => None,
        }
    }

    /// Returns the exclusive slice omitting the last `n` elements.
    ///
    /// Equivalent to `&mut slice[..slice.len() - n]`.
    ///
    /// # Safety
    /// Results in *undefined behavior* if `n` > `slice.len()`.
    #[must_use]
    #[inline(always)]
    #[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const fn take_omit_last_mut_unchecked(slice: &mut [T], n: usize) -> &mut [T] {
        unsafe { slice.split_at_mut_unchecked(slice.len() - n).0 }
    }
}
