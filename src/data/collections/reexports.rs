// devela::data::collections::reexports
//
//! Reexported items.
//

use crate::code::reexport;

/* from `alloc` */

reexport! { rust: alloc::collections, local_module: "data",
    doc: "A doubly-linked list with owned nodes.",
    @LinkedList as AllocLinkedList
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "An ordered map based on a B-Tree.",
    @BTreeMap as AllocOrdMap
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "An ordered set based on a B-Tree.",
    @BTreeSet as AllocOrdSet
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "A priority queue implemented with a binary heap.",
    @BinaryHeap as AllocPrioQueue
}
reexport! { rust: alloc::vec, local_module: "data",
    doc: "A contiguous growable array.",
    Vec
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "A double-ended queue implemented with a growable ring buffer.",
    @VecDeque as AllocDeque
}

/// A contiguos growable array.
#[cfg(feature = "alloc")]
pub type AllocArray<T> = Vec<T>;

/* from `hashbrown` */

reexport! { "hashbrown" | hashbrown, features: "data", "alloc",
    doc: "An unordered hash map implemented with quadratic probing and SIMD lookup.",
    @HashMap as AllocMap
}
reexport! { "hashbrown" | hashbrown, features: "data", "alloc",
    doc: "An unordered hash set implemented as a `AllocMap` where the value is `()`.",
    @HashSet as AllocSet
}
