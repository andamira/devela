// devela::data::list::queue::reexports
//
//! Reexported items.
//

use crate::_reexport;

#[cfg(feature = "alloc")]
crate::impl_cdef![<T> Self::new() => VecDeque<T>]; // impl ConstDefault

_reexport! { rust: alloc::collections,
    tag: crate::TAG_DATA_STRUCTURE!(),
    doc: "A priority queue implemented with a binary heap.",
    BinaryHeap
}
_reexport! { rust: alloc::collections,
    tag: crate::TAG_DATA_STRUCTURE!(),
    doc: "A double-ended growable queue.",
    VecDeque
}
