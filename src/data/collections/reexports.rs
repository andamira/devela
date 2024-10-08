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
    impl_cdef![<T> Self::new() => BTreeSet<T>, LinkedList<T>, Vec<T>, VecDeque<T>];
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
    LinkedList
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
    BinaryHeap
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

/* from `hashbrown` or `std` */

macro_rules! hashbrown_or_std {
    (start) => {
        "<span class='stab portability'
        title='re-exported from either `hashbrown` or `std`'>`std?`</span>"
    };
    (end) => {
        "\n\n*Re-exported from either the [`hashmap`](https://docs.rs/hasmap) crate
        or from [`std::collections`](https::doc.rust-lang.org/std/collections)*.
        \n\n---"
    };
}
#[allow(unused_imports)]
use hashbrown_or_std;

// types from hashbrown have preference over those from std.
#[cfg(feature = "hashbrown")]
pub use hashbrown_reexports::*;
#[cfg(feature = "hashbrown")]
mod hashbrown_reexports {
    use super::{hashbrown_or_std, reexport};

    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash map implemented with quadratic probing and SIMD lookup.
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::HashMap;

    #[doc = hashbrown_or_std!(start)]
    /// A view into a single entry in a map, which may either be vacant or occupied.
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::hash_map::Entry as HashMapEntry;

    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash set implemented as a `HashMap` where the value is `()`
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::HashSet;
}

#[cfg(all(not(feature = "hashbrown"), feature = "std"))]
pub use std_reexports::*;
#[cfg(all(not(feature = "hashbrown"), feature = "std"))]
mod std_reexports {
    use super::{hashbrown_or_std, reexport};

    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash map implemented with quadratic probing and SIMD lookup.
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
    pub use std::collections::HashMap;

    #[doc = hashbrown_or_std!(start)]
    /// A view into a single entry in a map, which may either be vacant or occupied.
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
    pub use std::collections::hash_map::Entry as HashMapEntry;

    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash set implemented as a `HashMap` where the value is `()`
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
    pub use std::collections::HashSet;
}

#[cfg(any(feature = "std", feature = "hashbrown"))]
pub use aliases::*;
#[cfg(any(feature = "std", feature = "hashbrown"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "hashbrown", feature = "std"))))]
mod aliases {
    use super::{HashMap, HashSet};
    use crate::data::hash::HasherBuildFx;

    /// A [`HashMap`] using a default Fx hasher.
    ///
    /// To create with a reserved capacity,
    /// use `HashMapFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashMapFx<K, V> = HashMap<K, V, HasherBuildFx>;

    /// An [`HashSet`] using a default Fx hasher.
    ///
    /// To create with a reserved capacity,
    /// use `HashSetFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashSetFx<T> = HashSet<T, HasherBuildFx>;
}
