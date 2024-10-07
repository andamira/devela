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
    impl_cdef![<T> Self::new() => BTreeSet<T>, AllocLinkedList<T>, Vec<T>, VecDeque<T>];
    impl_cdef![<K, V> Self::new() => BTreeMap<K, V>];
}

/* from `array` */

reexport! { rust: core::array,
    doc: "A by-value [array] iterator.",
    @IntoIter as ArrayIntoIter
}
reexport! { rust: core::array,
    doc: "Creates an array `[T; N]`, where each `T` is returned from `cb` from its index.",
    @from_fn as array_from_fn
}
reexport! { rust: core::array,
    doc: "Converts a mutable reference to `T` into `&mut [T; 1]` (without copying).",
    @from_mut as array_from_mut
}
reexport! { rust: core::array,
    doc: "Converts a reference to `T` into `&[T; 1]` (without copying).",
    @from_ref as array_from_ref
}

/* from `alloc` */

reexport! { rust: alloc::collections,
    doc: "A doubly-linked list with owned nodes.",
    @LinkedList as AllocLinkedList
}
reexport! { rust: alloc::collections,
    doc: "An ordered map based on a B-Tree.",
    BTreeMap
}
reexport! { rust: alloc::collections::btree_map,
    doc: "An ordered map based on a B-Tree.",
    @Entry as BTreeMapEntry
}
reexport! { rust: alloc::collections,
    doc: "An ordered set based on a B-Tree.",
    BTreeSet
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
///
/// The reason of the `_` suffix is to avoid conflicting with the prelude
/// when glob importing from this crate. Since this macro has the same name
/// as its sibling module `std::vec`, in order to be able to re-export
/// only the macro we have to wrap it with our own.
///
/// This is for completion purposes. You can keep using the `vec!` macro.
#[macro_export]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! vec_ { ($($tt:tt)*) => { $crate::_dep::_alloc::vec![$($tt)*] } }
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use vec_;

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
