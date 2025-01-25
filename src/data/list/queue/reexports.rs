// devela::data::list::queue::reexports
//
//! Reexported items.
//

crate::impl_cdef![<T> Self::new() => VecDeque<T>]; // impl ConstDefault

crate::reexport! { rust: alloc::collections,
    tag: crate::TAG_DATA_STRUCTURE!(),
    doc: "A double-ended growable queue.",
    VecDeque
}
