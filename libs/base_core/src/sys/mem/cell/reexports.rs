// devela_base_core::sys::mem::cell::reexports
//
//!
//

use crate::{_TAG_ERROR, _TAG_INIT, _TAG_LIFETIME, _TAG_MEM, _reexport};

_reexport! { rust: core::cell,
    tag: _TAG_LIFETIME!() _TAG_ERROR!(),
    doc: "An error returned by [`RefCell::try_borrow`].",
    @BorrowError as RefCellBorrowError
}
_reexport! { rust: core::cell,
    tag: _TAG_LIFETIME!() _TAG_ERROR!(),
    doc: "An error returned by [`RefCell::try_borrow_mut`].",
    @BorrowMutError as RefCellBorrowMutError
}
_reexport! { rust: core::cell,
    tag: _TAG_MEM!(),
    doc: "A mutable memory location.",
    Cell
}
_reexport! { rust: core::cell,
    tag: _TAG_MEM!() _TAG_INIT!() ,
    doc: "A value which is initialized on the first access.",
    LazyCell
}
_reexport! { rust: core::cell,
    tag: _TAG_MEM!() _TAG_INIT!() ,
    doc: "A cell which can nominally be written to only once.",
    OnceCell
}
_reexport! { rust: core::cell,
    tag: _TAG_MEM!() _TAG_LIFETIME!(),
    doc: "A mutable memory location with dynamically checked borrow rules.",
    RefCell
}
_reexport! { rust: core::cell,
    tag: _TAG_LIFETIME!(),
    doc: "A wrapper type for an inmutably borrowed value from a `RefCell<T>`",
    Ref
}
_reexport! { rust: core::cell,
    tag: _TAG_LIFETIME!(),
    doc: "A wrapper type for a mutably borrowed value from a `RefCell<T>`",
    RefMut
}
_reexport! { rust: core::cell,
    tag: _TAG_MEM!(),
    doc: "The core primitive for interior mutability in Rust.",
    UnsafeCell
}
