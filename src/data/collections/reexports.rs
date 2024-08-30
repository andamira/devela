// devela::data::collections::reexports
//
//! Reexported items.
//

use crate::code::reexport;

#[cfg(feature = "alloc")]
mod impls_alloc {
    use super::*;
    use crate::code::impl_cdef;

    // impl ConstDefault
    impl_cdef![<T> Self::new() => AllocOrdSet<T>, AllocLinkedList<T>, Vec<T>, VecDeque<T>];
    impl_cdef![<K, V> Self::new() => AllocOrdMap<K, V>];
}

/* from `array` */

reexport! { rust: core::array,
    doc: "A by-value [array] iterator.",
    @IntoIter as ArrayIntoIter
}

/* from `alloc` */

reexport! { rust: alloc::collections,
    doc: "A doubly-linked list with owned nodes.",
    @LinkedList as AllocLinkedList
}
reexport! { rust: alloc::collections,
    doc: "An ordered map based on a B-Tree.",
    @BTreeMap as AllocOrdMap
}
reexport! { rust: alloc::collections,
    doc: "An ordered set based on a B-Tree.",
    @BTreeSet as AllocOrdSet
}
reexport! { rust: alloc::collections,
    doc: "A priority queue implemented with a binary heap.",
    @BinaryHeap as AllocPrioQueue
}
reexport! { rust: alloc::vec,
    doc: "A contiguous growable array.",
    Vec
}
reexport! { rust: alloc::collections,
    doc: "A double-ended growable queue.",
    VecDeque
}

// NOTE: the macro and the module have the same name
//
/// <span class='stab portability' title='re-exported from rust&#39;s `alloc`'>`alloc`</span>
/// Creates a [`Vec`] containing the arguments.
///
#[doc = "*Re-exported from [`alloc::vec`][macro@crate::_dep::_alloc::vec]*."]
#[doc = "\n\n---"]
#[macro_export]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
macro_rules! vec { ($($tt:tt)*) => { crate::_dep::_alloc::vec![$($tt)*] } }
pub use vec;

/* from `hashbrown` */

#[cfg(feature = "hashbrown")]
pub use hashbrown_reexports::*;
#[cfg(feature = "hashbrown")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "hashbrown")))]
mod hashbrown_reexports {
    use super::reexport;
    use crate::data::hash::HasherBuildFx;

    reexport! { "hashbrown" | hashbrown, features: "alloc",
        doc: "An unordered hash map implemented with quadratic probing and SIMD lookup.",
        @HashMap as AllocMap
    }
    reexport! { "hashbrown" | hashbrown, features: "alloc",
        doc: "An unordered hash set implemented as a `AllocMap` where the value is `()`.",
        @HashSet as AllocSet
    }

    /// An [`AllocMap`] using a default Fx hasher.
    ///
    /// To create with a reserved capacity,
    /// use `AllocMapFx::with_capacity_and_hasher(num, Default::default())`.
    pub type AllocMapFx<K, V> = crate::_dep::hashbrown::HashMap<K, V, HasherBuildFx>;

    /// An [`AllocSet`] using a default Fx hasher.
    ///
    /// To create with a reserved capacity,
    /// use `AllocSetFx::with_capacity_and_hasher(num, Default::default())`.
    pub type AllocSetFx<K, V> = crate::_dep::hashbrown::HashSet<K, V, HasherBuildFx>;
}
