// devela_base_alloc::sys::mem::_reexport
//
//!
//

// use crate::Boxed; // TODO
use crate::{_TAG_LIFETIME, _TAG_MEM, _reexport};

_reexport! { rust: alloc::boxed, location: "sys::mem", tag: _TAG_MEM!(),
    doc: "A pointer type that uniquely owns a heap allocation of type `T`.

It is used as the underlying [`Storage`][super::Storage] for the [`Boxed`] marker
struct, just as a [`BareBox`][super::BareBox] is used as the storage for [`Bare`].

A special magic property of `Box` is that it allows moving with [*boxed], unlike
other `Deref` types. It is hoped that an eventual `DerefMove` trait will make it
possible for other types to opt in to move-from-deref.
",
    Box
}

_reexport! { rust: alloc::rc, location: "sys::mem", tag: _TAG_MEM!() _TAG_LIFETIME!(),
    doc: "A single-threaded reference-counting pointer.",
    Rc
}
_reexport! { rust: alloc::rc, location: "sys::mem", tag: _TAG_MEM!() _TAG_LIFETIME!(),
    doc: "A version of `Rc` that holds a non-owning ref to the managed allocation.",
    @Weak as RcWeak
}
