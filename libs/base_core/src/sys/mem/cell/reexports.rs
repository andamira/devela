// devela_base_core::sys::mem::cell::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: core::cell,
    tag: crate::_TAG_ERROR!(),
    doc: "An error returned by [`RefCell::try_borrow`].",
    @BorrowError as RefCellBorrowError
}
_reexport! { rust: core::cell,
    tag: crate::_TAG_ERROR!(),
    doc: "An error returned by [`RefCell::try_borrow_mut`].",
    @BorrowMutError as RefCellBorrowMutError
}
_reexport! { rust: core::cell,
    doc: "A mutable memory location.",
    Cell
}
_reexport! { rust: core::cell,
    doc: "A value which is initialized on the first access.",
    LazyCell
}
_reexport! { rust: core::cell,
    doc: "A cell which can nominally be written to only once.",
    OnceCell
}
_reexport! { rust: core::cell,
    doc: "A mutable memory location with dynamically checked borrow rules.",
    RefCell
}
_reexport! { rust: core::cell,
    doc: "A wrapper type for an inmutably borrowed value from a `RefCell<T>`",
    Ref
}
_reexport! { rust: core::cell,
    doc: "A wrapper type for a mutably borrowed value from a `RefCell<T>`",
    RefMut
}
_reexport! { rust: core::cell,
    doc: "The core primitive for interior mutability in Rust.",
    UnsafeCell
}
