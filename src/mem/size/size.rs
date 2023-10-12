// devela::mem::size::size
//
//! Trait for stack memory byte size.
//

use super::{super::size_of_val, ptr_ratio, size_of};

impl<T> Size for T {}

/// Constant size information.
///
/// This trait is automatically implemented for every `Sized` type.
pub trait Size: Sized {
    /// The size of this type in bytes, in the stack.
    const BYTE_SIZE: usize = size_of::<Self>();

    /// The size of a pointer in bytes, for the current platform.
    const PTR_SIZE: usize = size_of::<usize>();

    /// Returns the size of this type in bytes, in the stack.
    ///
    /// Ignores any allocated resources in the heap.
    fn byte_size(&self) -> usize {
        size_of_val(self)
    }

    /// Returns the size ratio between [`PTR_SIZE`][Self#constant.PTR_SIZE] and
    /// [`BYTES`][Self#constant.BYTES].
    ///
    /// For example: the ratio will be (1, 1) if both sizes are the same,
    /// (2, 1) if a pointer is double the byte size, and (1, 2) if a pointer is
    /// half the byte size.
    ///
    /// # Examples
    /// ```
    /// use devela::all::Size;
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
    #[inline]
    fn ptr_ratio(&self) -> (usize, usize) {
        ptr_ratio(Self::BYTE_SIZE)
    }
}
