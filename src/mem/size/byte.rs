// devela::mem::size::byte
//
//! Functionality related to byte sizes.
//

use crate::Ptr;

impl<T> ByteSized for T {}

/// Type size information in bytes.
///
/// This trait is automatically implemented for every `Sized` type.
// (this allows to have associated constants depending on Self)
pub trait ByteSized: Sized {
    /// The alignment of this type in bytes.
    const BYTE_ALIGN: usize = align_of::<Self>();
    /// The size of this type in bytes.
    const BYTE_SIZE: usize = size_of::<Self>();

    /// Returns the alignment of this type in bytes.
    #[inline]
    fn byte_align(&self) -> usize {
        align_of_val(self)
    }

    /// Returns the size of this type in bytes.
    ///
    /// Ignores any allocated resources in the heap.
    #[inline]
    fn byte_size(&self) -> usize {
        size_of_val(self)
    }

    /// Returns the size ratio between [`Ptr::BYTES`]
    /// and [`BYTE_SIZE`][Self#associatedconstant.BYTE_SIZE].
    ///
    /// For example: the ratio will be (1, 1) if both sizes are the same,
    /// (2, 1) if a pointer is double the byte size, and (1, 2) if a pointer is
    /// half the byte size.
    ///
    /// # Example
    /// ```
    /// use devela::ByteSized;
    ///
    /// assert_eq![().ptr_size_ratio(), [1, 0]];
    /// assert_eq![1_usize.ptr_size_ratio(), [1, 1]];
    /// assert_eq!["slice".ptr_size_ratio(), [1, 2]];
    /// assert_eq![String::from("hello").ptr_size_ratio(), [1, 3]];
    ///
    /// #[cfg(target_pointer_width = "64")]
    /// {
    ///     assert_eq![0_u8.ptr_size_ratio(), [8, 1]];
    ///     assert_eq![0_u16.ptr_size_ratio(), [4, 1]];
    ///     assert_eq![0_u32.ptr_size_ratio(), [2, 1]];
    ///     assert_eq![0_u64.ptr_size_ratio(), [1, 1]];
    ///     assert_eq![0_u128.ptr_size_ratio(), [1, 2]];
    ///     assert_eq!['c'.ptr_size_ratio(), [2, 1]];
    ///     assert_eq!["slice".ptr_size_ratio(), [1, 2]];
    /// }
    /// ```
    ///
    /// For the `const` version see [`Ptr::size_ratio`].
    #[inline]
    fn ptr_size_ratio(&self) -> [usize; 2] {
        Ptr::size_ratio(Self::BYTE_SIZE)
    }
}

/// Returns the rounded up size in bytes from a size in bits.
///
/// This is equivalent to `(bit_size + 7) / 8` but handles potential overflow.
#[inline]
#[must_use]
pub const fn bytes_from_bits(bit_size: usize) -> usize {
    if let Some(t) = bit_size.checked_add(8 - 1) {
        t / 8
    } else {
        bytes_from_bits_cold()
    }
}
#[cold] #[rustfmt::skip]
const fn bytes_from_bits_cold() -> usize { usize::MAX / 8 }
