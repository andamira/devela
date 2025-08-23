// devela::sys::mem::reexports
//
//! Reexported items from `core`.
//

#[cfg(all(doc, feature = "alloc"))]
use super::Boxed;
use crate::_reexport;

#[cfg(feature = "alloc")]
crate::impl_cdef![<T: ConstDefault> Self::new() => RcWeak<T>];

#[doc(inline)]
pub use crate::Sized;

/* box */

_reexport! { rust: alloc::boxed,
    doc: "A pointer type that uniquely owns a heap allocation of type `T`.

It is used as the underlying [`Storage`][super::Storage] for the [`Boxed`] marker
struct, just as a [`BareBox`][super::BareBox] is used as the storage for [`Bare`].

A special magic property of `Box` is that it allows moving with [*boxed], unlike
other `Deref` types. It is hoped that an eventual `DerefMove` trait will make it
possible for other types to opt in to move-from-deref.
",
    Box
}

/* mem structs */

_reexport! { rust: core::mem,
    doc: "A wrapper to inhibit compiler from automatically calling `T`’s destructor.",
    ManuallyDrop
}
_reexport! { rust: core::mem,
    doc: "Opaque type representing the discriminant of an enum.",
    Discriminant
}

/* mem unions */

_reexport! { rust: core::mem,
    doc: "A wrapper type to construct uninitialized instances of `T`.",
    MaybeUninit
}
/* mem macros */

_reexport! { rust: core::mem,
    doc: "Expands to the offset in bytes of a field from the beginning of the given type.",
    offset_of
}

/* mem functions */

// NOTE: can't namespace this in `Mem`.
_reexport! { rust: core::mem,
    doc: "Reinterprets the bits of a value of one type as another type.",
    transmute
}

/* rc */

_reexport! { rust: alloc::rc,
    doc: "A single-threaded reference-counting pointer.",
    Rc
}
_reexport! { rust: alloc::rc,
    doc: "A version of `Rc` that holds a non-owning reference to the managed allocation.",
    @Weak as RcWeak
}
