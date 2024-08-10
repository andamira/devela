// devela::mem::pod
//
//!
//
// See also: <https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html>

use core::{mem::MaybeUninit, num::*};

/// Indicates a type is Plain Old Data, and meets specific memory layout guarantees.
///
/// Types that implement this trait are guaranteed to be safe for certain
/// low-level memory operations, such as transmuting to and from byte slices,
/// copying, and interfacing with C code.
///
/// # Safety
///
/// Implementing `MemPod` is `unsafe` because it requires the implementer to ensure the
/// following invariants:
///
/// 1. **No Padding:** The type must not contain any padding bytes. This ensures that the
///    memory representation of the type is fully defined by its fields.
///
/// 2. **Safe to Transmute:** The type must be safe to transmute to and from a byte slice
///    (`&[u8]`). This requires that the type's memory layout is consistent and well-defined
///    across different platforms.
///
/// 3. **Copyable:** The type must implement `Copy`, meaning it can be duplicated simply by
///    copying its bits. This is a fundamental property of POD types.
///
/// 4. **Valid Bit Patterns:** Any bit pattern must represent a valid instance of the type.
///    This means that transmuting arbitrary bytes into the type must always produce a valid
///    value, and no bit pattern can cause undefined behavior when interpreted as this type.
///
/// # Implementing `MemPod`
///
/// When implementing `MemPod`, it is recommended to use `#[repr(C)]` or `#[repr(transparent)]`
/// on the type to ensure a well-defined and predictable memory layout. This is particularly
/// important when dealing with FFI or any situation where the exact memory layout of the type
/// is critical.
///
/// Note that only types that are `Copy` can implement `MemPod`. This is because POD types
/// inherently need to be trivially copyable without any additional logic (e.g. no destructors).
///
/// # Use Cases
///
/// - **FFI Compatibility:** `MemPod` types can be safely passed between Rust and C, as they
///   have a predictable memory layout.
/// - **Efficient Serialization:** `MemPod` types can be directly serialized to or deserialized
///   from a byte buffer, making them ideal for low-level data manipulation and storage.
/// - **Memory Safety:** By ensuring that types implementing `MemPod` adhere to strict memory
///   guarantees, you can safely perform low-level operations without risking undefined behavior.
///
/// # Examples
/// ```rust
/// use devela::mem::MemPod;
///
/// // Define a simple structure that can be safely used as POD.
/// #[derive(Copy, Clone)]
/// #[repr(C)]  // Ensure a predictable memory layout compatible with C.
/// struct MyStruct {
///     pub a: u8,
///     pub b: i16,
///     pub c: u32,
/// }
///
/// unsafe impl MemPod for MyStruct {}
/// ```
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
pub unsafe trait MemPod: Copy + 'static {
    /// Returns a new instance contrcuted from zeroes.
    #[must_use]
    fn zeroed() -> Self {
        // SAFETY. An all-zero value of `T: MemPod` is always valid.
        unsafe { core::mem::zeroed() }
    }

    /// Returns a new instance constructed from the given bytes.
    ///
    /// If the byte slice is too small, the remaining bytes will be 0.
    #[must_use]
    fn from_bytes(bytes: &[u8]) -> Self {
        let size = size_of::<Self>();
        let bytes_to_copy = core::cmp::min(bytes.len(), size);

        let mut new_self = MaybeUninit::<Self>::uninit();

        // SAFETY: We're only copying valid byte data into the uninitialized memory
        unsafe {
            // Copy the provided bytes into the memory
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                new_self.as_mut_ptr() as *mut u8,
                bytes_to_copy,
            );
            // If there are remaining bytes, fill them with zeros
            if bytes_to_copy < size {
                core::ptr::write_bytes(
                    (new_self.as_mut_ptr() as *mut u8).add(bytes_to_copy),
                    0,
                    size - bytes_to_copy,
                );
            }
            new_self.assume_init()
        }
    }

    /// Returns the instance's data as a slice of bytes.
    #[must_use]
    fn as_bytes(&self) -> &[u8] {
        let ptr = self as *const Self as *const u8;
        let len = size_of::<Self>();
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }

    /// Returns the instance's data as a mutable slice of bytes.
    #[must_use]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut Self as *mut u8;
        let len = size_of::<Self>();
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }
}

// Implement MemPod
#[rustfmt::skip]
macro_rules! mem_pod {
    // impl for types that are always POD.
    ($($t:ty),+) => { $( mem_pod![@$t]; )+ };
    (@$t:ty) => { unsafe impl MemPod for $t {} };

    // impl for types that are POD only when wrapped in an Option.
    (option: $($t:ty),+) => { $( mem_pod![@option: $t]; )+ };
    (@option: $t:ty) => { unsafe impl MemPod for Option<$t> {} };
}
pub(crate) use mem_pod;

#[rustfmt::skip]
mem_pod![(), u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64];

unsafe impl<T: MemPod, const N: usize> MemPod for [T; N] {}
unsafe impl<T: MemPod> MemPod for MaybeUninit<T> {}
unsafe impl<T: MemPod> MemPod for core::mem::ManuallyDrop<T> {}
unsafe impl<T: MemPod> MemPod for core::num::Wrapping<T> {}
unsafe impl<T: ?Sized + 'static> MemPod for core::marker::PhantomData<T> {}

unsafe impl<T: MemPod> MemPod for Option<T> {}
mem_pod![option:
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
];
