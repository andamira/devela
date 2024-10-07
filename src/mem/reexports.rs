// devela::mem::reexports
//
//! Reexported items from `core`.
//

#[cfg(all(doc, feature = "alloc"))]
use super::Boxed;
use crate::code::reexport;

#[cfg(feature = "alloc")]
crate::code::impl_cdef![<T: ConstDefault> Self::new() => crate::_dep::_alloc::rc::Weak<T>];

/* borrow */

reexport! { rust: core::borrow,
    doc: "A trait for borrowing data.",
    Borrow
}
reexport! { rust: core::borrow,
    doc: "A trait for mutably borrowing data.",
    BorrowMut
}
reexport! { rust: alloc::borrow,
    doc: "A clone-on-write smart pointer.",
    Cow
}
reexport! { rust: alloc::borrow,
    doc: "A generalization of Clone to borrowed data.",
    ToOwned
}

/* box */

reexport! { rust: alloc::boxed,
    doc: "A pointer type that uniquely owns a heap allocation of type `T`.

It is used as the underlying [`Storage`][super::Storage] for the [`Boxed`] marker struct,
just as a [`BareBox`][super::BareBox] is used as the storage for [`Bare`].

A special magic property of `Box` is that it allows moving with [*boxed], unlike
other `Deref` types. It is hoped that an eventual `DerefMove` trait will make it
possible for other types to opt in to move-from-deref.
",
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

/* rc */

reexport! { rust: alloc::rc,
    doc: "A single-threaded reference-counting pointer.",
    Rc
}
reexport! { rust: alloc::rc,
    doc: "A version of `Rc` that holds a non-owning reference to the managed allocation.",
    @Rc as RcWeak
}
