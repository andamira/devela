// devela::data::list::link::reexports
//
//! Reexported items.
//

crate::impl_cdef![<T> Self::new() => LinkedList<T>]; // impl ConstDefault

crate::reexport! { rust: alloc::collections,
    tag: crate::TAG_DATA_STRUCTURE!(),
    doc: "A doubly-linked list with owned nodes.",
    LinkedList
}
