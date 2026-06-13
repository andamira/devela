// devela/src/data/layout/queue/_reexport_alloc.rs

use crate::{_reexport, _tags};

_reexport! { rust: alloc::collections, location: "data/layout/queue", tag: _tags!(data_structure),
    doc: "A priority queue implemented with a binary heap.",
    BinaryHeap
}
_reexport! { rust: alloc::collections, location: "data/layout/queue", tag: _tags!(data_structure),
    doc: "A double-ended growable queue.",
    VecDeque
}
