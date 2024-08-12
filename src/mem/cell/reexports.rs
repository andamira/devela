// devela::mem::cell::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

/* core */

reexport! { rust: core::cell,
    doc: "A mutable memory location.",
    Cell
}
reexport! { rust: core::cell,
    doc: "A value which is initialized on the first access.",
    LazyCell
}
reexport! { rust: core::cell,
    doc: "A cell which can nominally be written to only once.",
    OnceCell
}
reexport! { rust: core::cell,
    doc: "A mutable memory location with dynamically checked borrow rules.",
    RefCell
}
reexport! { rust: core::cell,
    doc: "A wrapper type for an inmutably borrowed value from a `RefCell<T>`",
    Ref
}
reexport! { rust: core::cell,
    doc: "A wrapper type for a mutably borrowed value from a `RefCell<T>`",
    RefMut
}
reexport! { rust: core::cell,
    doc: "The core primitive for interior mutability in Rust.",
    UnsafeCell
}
