// devela::mem::reexports
//
//! Reexported items from `core`.
//

#[cfg(all(doc, feature = "alloc"))]
use super::Boxed;
use crate::code::reexport;

#[cfg(feature = "alloc")]
crate::code::impl_cdef![<T> Self::new() => crate::_alloc::rc::Weak<T>];
// crate::code::impl_cdef![<T> Self::new() => RcWeak<T>]; ??

/* type aliases */

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
/// A marker struct for a [`Storage`][super::Storage] type that wraps its data in a
/// [`BareBox`][super::BareBox]. Alias of [`()`][unit].
///
/// Equivalent to the [`Boxed`] marker struct which uses a [`Box`] for the underlying storage.
///
/// ---
pub type Bare = ();

/* box */

reexport! { rust: alloc::boxed,
    doc: "A pointer type that uniquely owns a heap allocation of type `T`.

It is used as the underlying [`Storage`][super::Storage] for the [`Boxed`] marker struct,
just as a [`BareBox`][super::BareBox] is used as the storage for [`Bare`].",
    Box
}

/* mem */

reexport! { rust: core::mem,
    doc: "A wrapper to inhibit compiler from automatically calling `T`’s destructor.",
    ManuallyDrop
}
reexport! { rust: core::mem,
    doc: "A wrapper type to construct uninitialized instances of `T`.",
    MaybeUninit
}
reexport! { rust: core::mem,
    doc: "Opaque type representing the discriminant of an enum.",
    @Discriminant as EnumDiscriminant
}
reexport! { rust: core::mem,
    doc: "Returns a value uniquely identifying the enum variant in `v`.",
    @discriminant as enum_discriminant
}
reexport! { rust: core::mem,
    doc: "Returns the minimum alignment of the type of the value that `val` points to in bytes.",
    @align_of as mem_align_of
}
reexport! { rust: core::mem,
    doc: "Returns the align of the pointed-to value in bytes.",
    @align_of_val as mem_align_of_val
}
reexport! { rust: core::mem,
    doc: "Disposes of a value.",
    @drop as mem_drop
}
reexport! { rust: core::mem,
    doc: "Takes ownership and “forgets” about `t` *without running its destructor*.",
    @forget as mem_forget
}
reexport! { rust: core::mem,
    doc: "Returns true if dropping values of type T matters.",
    @needs_drop as mem_needs_drop
}
reexport! { rust: core::mem,
    doc: "Moves `src` into `dest`, returning the previous `dest` value.",
    @replace as mem_replace
}
reexport! { rust: core::mem,
    doc: "Swaps the values at two locations, without deinitializing either one.",
    @swap as mem_swap
}
reexport! { rust: core::mem,
    doc: "Replaces `dest` with `T::default()`, returning the previous `dest` value.",
    @take as mem_take
}
reexport! { rust: core::mem,
    doc: "Reinterprets the bits of a value of one type as another type.",
    @transmute as mem_transmute
}
reexport! { rust: core::mem,
    doc: "Reads `src` as having type `&Dst` without moving the contained value.",
    @transmute_copy as mem_transmute_copy
}
reexport! { rust: core::mem,
    doc: "Returns the value of type `T` represented by the all-zero byte-pattern.",
    @zeroed as mem_zeroed
}

/* pin */

reexport! { rust: core::pin,
    doc: "Constructs a <code>[Pin]<[&mut] T></code>, by pinning a `value: T` locally.",
    pin
}
reexport! { rust: core::pin,
    doc: "A pointer which pins its pointee in place.",
    Pin
}

/* ptr */

// structs
reexport! { rust: core::ptr,
    doc: "`*mut T` but non-zero and *covariant*.",
    @NonNull as PtrNonNull
}

// macros
reexport! { rust: core::ptr,
    doc: "Create a `const` raw pointer to a place, without creating an intermediate reference.",
    @addr_of as ptr_addr_of
}
reexport! { rust: core::ptr,
    doc: "Create a `mut` raw pointer to a place, without creating an intermediate reference.",
    @addr_of_mut as ptr_addr_of_mut
}

// fns
reexport! { rust: core::ptr,
    doc: "Compares raw pointer addresses for equality, ignoring any metadata in fat pointers.",
    @addr_eq as ptr_addr_eq
}
reexport! { rust: core::ptr, // const unsafe
    doc: "Copies `count * size_of::<T>()` bytes from `src` to `dst`. Can overlap.",
    @copy as ptr_copy
}
reexport! { rust: core::ptr, // const unsafe
    doc: "Copies `count * size_of::<T>()` bytes from `src` to `dst`. Must *not* overlap.",
    @copy_nonoverlapping as ptr_copy_nonverlapping
}
reexport! { rust: core::ptr, // unsafe
    doc: "Executes the destructor (if any) of the pointed-to value.",
    @drop_in_place as ptr_drop_in_place
}
reexport! { rust: core::ptr,
    doc: "Compares raw pointers for equality.",
    @eq as ptr_eq
}
reexport! { rust: core::ptr, // const
    doc: "Convert an exclusive reference to a raw pointer.",
    @from_mut as ptr_from_mut
}
reexport! { rust: core::ptr, // const
    doc: "Convert a shared reference to a raw pointer.",
    @from_ref as ptr_from_ref
}
reexport! { rust: core::ptr,
    doc: "Hash a raw pointer.",
    @hash as ptr_hash
}
reexport! { rust: core::ptr, // const
    doc: "Creates a null raw pointer.",
    @null as ptr_null
}
reexport! { rust: core::ptr, // const
    doc: "Creates a null mutable raw pointer.",
    @null_mut as ptr_null_mut
}
reexport! { rust: core::ptr, // unsafe
    doc: "Reads the value from src without moving it.",
    @read as ptr_read
}
reexport! { rust: core::ptr, // unsafe
    doc: "Reads the value from src without moving it.",
    @read_unaligned as ptr_read_unaligned
}
reexport! { rust: core::ptr, // unsafe
    doc: "Performs a volatile read of the value from src without moving it.",
    @read_volatile as ptr_read_volatile
}
reexport! { rust: core::ptr, // unsafe
    doc: "Moves src into the pointed dst, returning the previous dst value.",
    @replace as ptr_replace
}
reexport! { rust: core::ptr,
    doc: "Forms a raw slice from a pointer and a length.",
    @slice_from_raw_parts as ptr_slice_from_raw_parts
}
reexport! { rust: core::ptr,
    doc: "Forms a mutable raw slice from a mutable pointer and a length.",
    @slice_from_raw_parts_mut as ptr_slice_from_raw_parts_mut
}
reexport! { rust: core::ptr, // unsafe
    doc: "Swaps the values at two mutable locations of the same type, without deinitializing.",
    @swap as ptr_swap
}
reexport! { rust: core::ptr, // unsafe
    doc: "Swaps the two regions of memory beginning at `x` and `y`. Must *not* overlap.",
    @swap_nonoverlapping as ptr_swap_nonoverlapping
}
reexport! { rust: core::ptr, // unsafe
    doc: "Overwrites a memory location with `src` without reading or dropping.",
    @write as ptr_write
}
reexport! { rust: core::ptr, // unsafe
    doc: "Sets `count * size_of::<T>()` bytes of memory starting at `dst` to `val`.",
    @write_bytes as ptr_write_bytes
}
reexport! { rust: core::ptr, // unsafe
    doc: "Overwrites a memory location with `src` without reading or dropping.",
    @write_unaligned as ptr_write_unaligned
}
reexport! { rust: core::ptr, // unsafe
    doc: "Performs a volatile write of a memory location with `src` without reading or dropping.",
    @write_volatile as ptr_write_volatile
}

/* rc */

reexport! { rust: alloc::rc,
    doc: "A single-threaded reference-counting pointer.",
    Rc
}
reexport! { rust: alloc::rc,
    doc: "A version of `Rc` that holds a non-owning reference to the managed allocation.",
    @Rc as RcWeak
}
