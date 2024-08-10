// devela::mem::pod
//
//!
//

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
/// 2. **Safe to Transmute:** The type must be safe to transmute to and from a byte slice
///    (`&[u8]`). This requires that the type's memory layout is consistent and well-defined
///    across different platforms.
/// 3. **Copyable:** The type must implement `Copy`, meaning it can be duplicated simply by
///    copying its bits. This is a fundamental property of POD types.
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
pub unsafe trait MemPod: Copy + Clone + Sized {}

// Implement MemPod
#[rustfmt::skip]
macro_rules! mem_pod {
    ($($t:ty),+) => { $( mem_pod![@$t]; )+ };
    (@$t:ty) => { unsafe impl MemPod for $t {} };
}
mem_pod![u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];
