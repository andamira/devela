// devela::sys::mem::reexports
//
//! Reexported items from `core`.
//

#[cfg(all(doc, feature = "alloc"))]
use super::Boxed;
use crate::reexport;

#[cfg(feature = "alloc")]
crate::impl_cdef![<T: ConstDefault> Self::new() => RcWeak<T>];

#[doc(inline)]
pub use crate::Sized;

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
    Discriminant
}

reexport! { rust: core::mem,
    doc: "Expands to the offset in bytes of a field from the beginning of the given type.",
    offset_of
}

// NOTE: can't namespace this in `Mem`.
reexport! { rust: core::mem,
    doc: "Reinterprets the bits of a value of one type as another type.",
    transmute
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
    @Weak as RcWeak
}
