// devela::mem::size::size
//
//! Functionality related to byte sizes.
//

use super::super::{mem_align_of, mem_align_of_val, mem_size_of, mem_size_of_val, Mem};
use crate::meta::iif;

impl<T> Size for T {}

/// Type size information.
///
/// This trait is automatically implemented for every `Sized` type.
// (this allows use to have associated constants depending on Self)
pub trait Size: Mem + Sized {
    /// The alignment of this type in bytes.
    const BYTE_ALIGN: usize = mem_align_of::<Self>();

    /// The size of this type in bytes.
    const BYTE_SIZE: usize = mem_size_of::<Self>();

    /// The size of a pointer in bytes, for the current platform.
    const PTR_SIZE: usize = mem_size_of::<usize>();

    /// Returns the alignment of this type in bytes.
    #[inline]
    fn byte_align(&self) -> usize {
        mem_align_of_val(self)
    }

    /// Returns the size of this type in bytes.
    ///
    /// Ignores any allocated resources in the heap.
    #[inline]
    fn byte_size(&self) -> usize {
        mem_size_of_val(self)
    }

    /// Returns the size ratio between [`PTR_SIZE`][Self#associatedconstant.PTR_SIZE]
    /// and [`BYTE_SIZE`][Self#associatedconstant.BYTE_SIZE].
    ///
    /// For example: the ratio will be (1, 1) if both sizes are the same,
    /// (2, 1) if a pointer is double the byte size, and (1, 2) if a pointer is
    /// half the byte size.
    ///
    /// # Examples
    /// ```
    /// use devela::mem::Size;
    ///
    /// assert_eq![().ptr_ratio(), (1, 0)];
    /// assert_eq![1_usize.ptr_ratio(), (1, 1)];
    /// assert_eq!["slice".ptr_ratio(), (1, 2)];
    /// assert_eq![String::from("hello").ptr_ratio(), (1, 3)];
    ///
    /// #[cfg(target_pointer_width = "64")]
    /// {
    ///     assert_eq![0_u8.ptr_ratio(), (8, 1)];
    ///     assert_eq![0_u16.ptr_ratio(), (4, 1)];
    ///     assert_eq![0_u32.ptr_ratio(), (2, 1)];
    ///     assert_eq![0_u64.ptr_ratio(), (1, 1)];
    ///     assert_eq![0_u128.ptr_ratio(), (1, 2)];
    ///     assert_eq!['c'.ptr_ratio(), (2, 1)];
    ///     assert_eq!["slice".ptr_ratio(), (1, 2)];
    /// }
    /// ```
    ///
    /// For the `const` version see [`mem_ptr_ratio`].
    #[inline]
    fn ptr_ratio(&self) -> (usize, usize) {
        mem_ptr_ratio(Self::BYTE_SIZE)
    }
}

/// Returns the ratio of a `usize` in respect to `other_size`.
///
/// For example: the ratio will be `(1, 1)` if both sizes are equal, `(2, 1)`
/// if the pointer size is double the other size, and `(1, 2)` if is is half
/// the other byte size.
///
/// # Examples
/// ```
/// use devela::mem::{mem_ptr_ratio, mem_size_of};
///
/// assert_eq![mem_ptr_ratio(0), (1, 0)];
/// assert_eq![mem_ptr_ratio(mem_size_of::<usize>()), (1, 1)];
/// assert_eq![mem_ptr_ratio(mem_size_of::<&str>()), (1, 2)];
/// assert_eq![mem_ptr_ratio(mem_size_of::<String>()), (1, 3)];
///
/// #[cfg(target_pointer_width = "64")]
/// assert_eq![mem_ptr_ratio(mem_size_of::<char>()), (2,1)];
/// ```
///
/// Note that when `other_size == 0` it returns `(1, 0)` which is an invalid ratio.
///
#[inline]
pub const fn mem_ptr_ratio(other_size: usize) -> (usize, usize) {
    #[inline]
    const fn gcd(m: usize, n: usize) -> usize {
        iif![n == 0; m; gcd(n, m % n)]
    }
    let g = gcd(mem_size_of::<usize>(), other_size);
    (mem_size_of::<usize>() / g, other_size / g)
}
