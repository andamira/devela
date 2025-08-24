// devela_base_alloc::data::list::queue::reexports

use crate::_reexport;

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
