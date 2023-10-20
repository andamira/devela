// devela::collections::reexports
//
//! Reexported items.
//

use crate::codegen::reexport;

/* from `alloc` */

reexport! { rust: "alloc"|alloc::collections, local_module: "collections",
    doc: "A doubly-linked list with owned nodes.",
    @LinkedList as DoublyLinkedList
}
reexport! { rust: "alloc"|alloc::collections, local_module: "collections",
    doc: "An ordered map based on a B-Tree.",
    @BTreeMap as OrderedMap
}
reexport! { rust: "alloc"|alloc::collections, local_module: "collections",
    doc: "An ordered set based on a B-Tree.",
    @BTreeSet as OrderedSet
}
reexport! { rust: "alloc"|alloc::collections, local_module: "collections",
    doc: "A priority queue implemented with a binary heap.",
    @BinaryHeap as PriorityQueue
}
reexport! { rust: "alloc"|alloc::vec, local_module: "collections",
    doc: "A contiguous growable array type, written as `Vec<T>`, short for ‘vector’.",
    Vec
}
reexport! { rust: "alloc"|alloc::collections, local_module: "collections",
    doc: "A double-ended queue implemented with a growable ring buffer.",
    VecDeque
}

/* from `hashbrown` */

reexport! { "hashbrown" | hashbrown, features: "collections", "alloc",
    doc: "A hash map implemented with quadratic probing and SIMD lookup.",
    @HashMap as UnorderedMap
}
reexport! { "hashbrown" | hashbrown, features: "collections", "alloc",
    doc: "A hash set implemented as an `UnorderedSet` where the value is `()`.",
    @HashSet as UnorderedSet
}
