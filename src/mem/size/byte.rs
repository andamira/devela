// devela::mem::size::byte
//
//! Functionality related to byte sizes.
//

use crate::code::iif;

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

    /// The size of a pointer in bytes, for the current platform.
    const PTR_BYTES: usize = size_of::<usize>();

    /// The size of a pointer in bits, for the current platform.
    const PTR_BITS: usize = usize::BITS as usize;

    /// True if the system's architecture is little-endian.
    const LITTLE_ENDIAN: bool = cfg!(target_endian = "little");

    /// True if the system's architecture is big-endian.
    const BIG_ENDIAN: bool = cfg!(target_endian = "big");

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

    /// Returns the size ratio between [`PTR_BYTES`][Self#associatedconstant.PTR_BYTES]
    /// and [`BYTE_SIZE`][Self#associatedconstant.BYTE_SIZE].
    ///
    /// For example: the ratio will be (1, 1) if both sizes are the same,
    /// (2, 1) if a pointer is double the byte size, and (1, 2) if a pointer is
    /// half the byte size.
    ///
    /// # Examples
    /// ```
    /// use devela::mem::ByteSized;
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
    /// For the `const` version see [`ptr_size_ratio`].
    #[inline]
    fn ptr_size_ratio(&self) -> [usize; 2] {
        ptr_size_ratio(Self::BYTE_SIZE)
    }
}

/* fn definitions */

/// Returns the ratio of a `usize` in respect to `other_size`.
///
/// For example: the ratio will be `(1, 1)` if both sizes are equal, `(2, 1)`
/// if the pointer size is double the other size, and `(1, 2)` if is is half
/// the other byte size.
///
/// # Examples
/// ```
/// use devela::mem::ptr_size_ratio;
///
/// assert_eq![ptr_size_ratio(0), [1, 0]];
/// assert_eq![ptr_size_ratio(size_of::<usize>()), [1, 1]];
/// assert_eq![ptr_size_ratio(size_of::<&str>()), [1, 2]];
/// assert_eq![ptr_size_ratio(size_of::<String>()), [1, 3]];
///
/// #[cfg(target_pointer_width = "64")]
/// assert_eq![ptr_size_ratio(size_of::<char>()), [2,1]];
/// ```
///
/// Note that when `other_size == 0` it returns `(1, 0)` which is an invalid ratio.
///
#[inline]
#[allow(dead_code)]
pub const fn ptr_size_ratio(other_size: usize) -> [usize; 2] {
    #[inline]
    const fn gcd(m: usize, n: usize) -> usize {
        iif![n == 0; m; gcd(n, m % n)]
    }
    let g = gcd(size_of::<usize>(), other_size);
    [size_of::<usize>() / g, other_size / g]
}

/// Returns the rounded up size in bytes from a size in bits.
///
/// This is equivalent to `(bit_size + 7) / 8` but handles potential overflow.
#[must_use]
#[inline]
pub const fn bytes_from_bits(bit_size: usize) -> usize {
    if let Some(t) = bit_size.checked_add(8 - 1) {
        t / 8
    } else {
        bytes_from_bits_cold()
    }
}
#[cold] #[rustfmt::skip]
const fn bytes_from_bits_cold() -> usize { usize::MAX / 8 }
