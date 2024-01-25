// devela::data::collections::reexports
//
//! Reexported items.
//

use crate::code::reexport;

/* from `alloc` */

reexport! { rust: alloc::collections, local_module: "data",
    doc: "A doubly-linked list with owned nodes.",
    @LinkedList as DoublyLinkedList
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "An ordered map based on a B-Tree.",
    @BTreeMap as OrderedMap
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "An ordered set based on a B-Tree.",
    @BTreeSet as OrderedSet
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "A priority queue implemented with a binary heap.",
    @BinaryHeap as PriorityQueue
}
reexport! { rust: alloc::vec, local_module: "data",
    doc: "A contiguous growable array type, written as `Vec<T>`, short for ‘vector’.",
    Vec
}
reexport! { rust: alloc::collections, local_module: "data",
    doc: "A double-ended queue implemented with a growable ring buffer.",
    VecDeque
}

/* from `hashbrown` */

reexport! { "hashbrown" | hashbrown, features: "data", "alloc",
    doc: "A hash map implemented with quadratic probing and SIMD lookup.",
    @HashMap as UnorderedMap
}
reexport! { "hashbrown" | hashbrown, features: "data", "alloc",
    doc: "A hash set implemented as an `UnorderedSet` where the value is `()`.",
    @HashSet as UnorderedSet
}
