// devela_base_alloc::data::list::queue::_reexport

use crate::{_TAG_DATA_STRUCTURE, _reexport};

_reexport! { rust: alloc::collections, location: "data/list/queue", tag: _TAG_DATA_STRUCTURE!(),
    doc: "A priority queue implemented with a binary heap.",
    BinaryHeap
}
_reexport! { rust: alloc::collections, location: "data/list/queue", tag: _TAG_DATA_STRUCTURE!(),
    doc: "A double-ended growable queue.",
    VecDeque
}
