use crate::{_reexport, _tags};

pub use {
    access_iter::*, codec_hash::*, layout_array::*, layout_array_owned_vec::*, layout_linked::*,
    layout_queue::*, store_key_map::*, store_key_set::*,
};

// - WAIT: [iter_map_windows](https://github.com/rust-lang/rust/issues/87155)
// - WAIT: [iter_array_chunks](https://github.com/rust-lang/rust/issues/100450)
// - WAIT: [iter_intersperse](https://github.com/rust-lang/rust/issues/79524)
// - WAIT: [step_trait](https://github.com/rust-lang/rust/issues/42168)
// - WAIT: [trusted_len](https://github.com/rust-lang/rust/issues/37572)
// - WAIT: [trusted_step](https://github.com/rust-lang/rust/issues/85731)
//
// NOTE: iterator implementation structs are not re-exported.
// NOTE: functions are re-exported as methods of the Iter namespace.
pub(crate) mod access_iter {
    use super::*;

    /* traits */

    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorDoubleEnded, tag: _tags!(iterator),
        doc: "An iterator able to yield elements from both ends.",
        @DoubleEndedIterator as IteratorDoubleEnded
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorExactSize, tag: _tags!(iterator),
        doc: "An iterator that knows its exact length.",
        @ExactSizeIterator as IteratorExactSize
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorExtend, tag: _tags!(iterator),
        doc: "Extend a collection with the contents of an iterator.",
        @Extend as IteratorExtend
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorFrom, tag: _tags!(iterator),
        doc: "Conversion from an [`Iterator`].",
        @FromIterator as IteratorFrom
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorFused, tag: _tags!(iterator),
        doc: "An iterator that always continues to yield `None` when exhausted.",
        @FusedIterator as IteratorFused
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorInto, tag: _tags!(iterator),
        doc: "Conversion into an [`Iterator`].",
        @IntoIterator as IteratorInto
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait Iterator, tag: _tags!(iterator),
        doc: "A trait for dealing with iterators.",
        Iterator
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorProduct, tag: _tags!(iterator num),
        doc: "Represents types that can be created by multiplying elements of an iterator.",
        @Product as IteratorProduct
    }
    _reexport! { rust: core::iter,
        location: "data/access/iter" => trait IteratorSum, tag: _tags!(iterator num),
        doc: "Represents types that can be created by summing up an iterator.",
        @Sum as IteratorSum
    }

    /* structs */

    _reexport! { rust: core::iter, extra_flags:(nightly_coro),
        location: "data/access/iter" => struct IterFromCoroutine, tag: _tags!(iterator runtime),
        doc: "Creates an iterator where each iteration calls the provided coroutine.",
        @FromCoroutine as IterFromCoroutine
    }
}

pub(crate) mod codec_hash {
    use super::*;

    /* structs */

    _reexport! { rust: core::hash,
        location: "data/codec/hash" => struct HasherBuildDefault, tag: _tags!(hash),
        doc: "Create a default [`HasherBuild`] instance for `T:`[`Hasher`]` + `[`Default`] types.",
        @BuildHasherDefault as HasherBuildDefault
    }
    _reexport! { rust: std::hash,
        location: "data/codec/hash" => struct RandomState, tag: _tags!(hash),
        doc: "The default state for [`HashMapStd`][crate::HashMapStd].",
        RandomState
    }
    /* traits */

    // NOTE: the following trait and the corresponding derive macro have the same name:
    _reexport! { rust: core::hash,
        location: "data/codec/hash" => trait Hash, tag: _tags!(hash),
        doc: "A trait for creating instances of [`Hasher`]. (Derivable)",
        Hash
    }
    _reexport! { rust: core::hash,
        location: "data/codec/hash" => trait Hasher, tag: _tags!(hash),
        doc: "A trait for hashing an arbitrary stream of bytes.",
        Hasher
    }
    _reexport! { rust: core::hash,
        location: "data/codec/hash" => trait HasherBuild, tag: _tags!(hash),
        doc: "A trait for creating instances of [`Hasher`].",
        @BuildHasher as HasherBuild
    }
}

pub(crate) mod layout_array {
    use super::*;

    /* functions */

    _reexport! { rust: core::array,
        location: "data/layout/array" => fn array_from_fn, tag: _tags!(data_structure),
        doc: "Creates an array `[T; N]`, where each `T` is returned from `cb` from its index.",
        @from_fn as array_from_fn
    }
    _reexport! { rust: core::array,
        location: "data/layout/array" => fn array_from_mut, tag: _tags!(data_structure),
        doc: "Converts a mutable reference to `T` into `&mut [T; 1]` (without copying).",
        @from_mut as array_from_mut
    }
    _reexport! { rust: core::array,
        location: "data/layout/array" => fn from_ref, tag: _tags!(data_structure),
        doc: "Converts a reference to `T` into `&[T; 1]` (without copying).",
        @from_ref as array_from_ref
    }
}
pub(crate) mod layout_array_owned_vec {
    use super::*;

    _reexport! { rust: alloc::vec,
        location: "data/layout/array" => struct Vec, tag: _tags!(data_structure),
        doc: "A contiguous growable array.",
        Vec
    }

    // NOTE: the following macro and the corresponding module have the same name:
    #[doc = _tags!(construction data_structure)]
    /// <span class='stab portability' title='re-exported from rust&#39;s `alloc`'>`alloc`</span>
    /// Creates a [`Vec`] containing the arguments.
    #[doc = crate::_doc_meta!{
        location("data/layout/array", macro vec_),
    }]
    #[doc = "*Re-exported from [`alloc::vec`][macro@alloc::vec]*."]
    #[doc = "\n\n---"]
    ///
    /// The reason of the `_` suffix is to avoid conflicting with Rust's prelude
    /// when glob importing from this crate. Since this macro has the same name
    /// as its sibling module `std::vec`, to be able to re-export
    /// just the macro we have to wrap it with our own.
    #[macro_export]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    #[cfg_attr(cargo_primary_package, doc(hidden))]
    macro_rules! vec_ { ($($tt:tt)*) => { ::alloc::vec![$($tt)*] } }
}
pub(crate) mod layout_linked {
    use super::*;

    /* structures */

    _reexport! { rust: alloc::collections,
        location: "data/layout/linked" => struct LinkedList, tag: _tags!(data_structure),
        doc: "A doubly-linked list with owned nodes.",
        LinkedList
    }
}
pub(crate) mod layout_queue {
    use super::*;

    _reexport! { rust: alloc::collections,
        location: "data/layout/queue" => struct BinaryHeap, tag: _tags!(data_structure),
        doc: "A priority queue implemented with a binary heap.",
        BinaryHeap
    }
    _reexport! { rust: alloc::collections,
        location: "data/layout/queue" => struct VecDeque, tag: _tags!(data_structure),
        doc: "A double-ended growable queue.",
        VecDeque
    }
}

#[allow(unused_macros)]
macro_rules! _hashbrown_or_std {
    (start) => {
        "<span class='stab portability'
        title='re-exported from either `hashbrown` or `std`'>`≡std`</span>"
    };
    (end) => {
        "\n\n*Re-exported from either the [`hashbrown`](https://docs.rs/hashbrown) crate
        or from [`std::collections`](https://doc.rust-lang.org/std/collections)*.
        \n\n---"
    };
}
use _hashbrown_or_std;

pub(crate) mod store_key_map {
    use super::*;

    /* structs */

    _reexport! { rust: alloc::collections,
        location: "data/store/key/map" => struct BTreeMap, tag: _tags!(data_structure ord),
        doc: "An ordered map based on a B-Tree.",
        BTreeMap
    }
    _reexport! { rust: alloc::collections::btree_map,
        location: "data/store/key/map" => struct BTreeMapEntry, tag: _tags!(data_structure ord),
        doc: "An entry of an ordered map based on a B-Tree.",
        @Entry as BTreeMapEntry
    }

    /* from `hashbrown` or `std` */

    // types from hashbrown have preference over those from std.
    #[cfg(feature = "dep_hashbrown")]
    pub use hashbrown_reexports::*;
    #[cfg(feature = "dep_hashbrown")]
    mod hashbrown_reexports {
        use super::_hashbrown_or_std;

        #[doc = crate::_tags!(data_structure hash)]
        #[doc = _hashbrown_or_std!(start)]
        /// An unordered hash map with quadratic probing and SIMD lookup.
        #[doc = crate::_doc_meta!{
            location("data/store/key/map", struct HashMap),
        }]
        #[doc = _hashbrown_or_std!(end)]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use crate::_dep::hashbrown::HashMap;

        #[doc = crate::_tags!(data_structure hash)]
        #[doc = _hashbrown_or_std!(start)]
        /// A view into a single entry in a map, which may either be vacant or occupied.
        #[doc = crate::_doc_meta!{
            location("data/store/key/map", struct HashMapEntry),
        }]
        #[doc = _hashbrown_or_std!(end)]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use crate::_dep::hashbrown::hash_map::Entry as HashMapEntry;
    }

    #[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
    pub use std_reexports::*;
    #[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
    mod std_reexports {
        #[doc = crate::_tags!(data_structure hash)]
        #[doc = super::_hashbrown_or_std!(start)]
        /// An unordered hash map with quadratic probing and SIMD lookup.
        #[doc = crate::_doc_meta!{
            location("data/store/key/map", struct HashMap),
        }]
        #[doc = super::_hashbrown_or_std!(end)]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use std::collections::hash_map::HashMap;

        #[doc = crate::_tags!(data_structure hash)]
        #[doc = super::_hashbrown_or_std!(start)]
        /// A view into a single entry in a map, which may either be vacant or occupied.
        #[doc = crate::_doc_meta!{
            location("data/store/key/map", struct HashMapEntry),
        }]
        #[doc = super::_hashbrown_or_std!(end)]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use std::collections::hash_map::Entry as HashMapEntry;
    }
}
pub(crate) mod store_key_set {
    use super::*;

    /* structs: */

    _reexport! { rust: alloc::collections,
        location: "data/store/key/set" => struct BTreeSet, tag: _tags!(data_structure ord set),
        doc: "An ordered set based on a B-Tree.",
        BTreeSet
    }

    /* from `hashbrown` or `std` */

    // types from hashbrown have preference over those from std.
    #[cfg(feature = "dep_hashbrown")]
    pub use hashbrown_reexports::*;
    #[cfg(feature = "dep_hashbrown")]
    mod hashbrown_reexports {
        use super::_hashbrown_or_std;

        #[doc = crate::_tags!(data_structure hash set)]
        #[doc = _hashbrown_or_std!(start)]
        /// An unordered hash set implemented as a `HashMap` where the value is `()`
        #[doc = crate::_doc_meta!{location("data/store/key/set", struct HashSet)}]
        #[doc = _hashbrown_or_std!(end)]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use crate::_dep::hashbrown::HashSet;
    }

    #[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
    pub use std_reexports::*;
    #[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
    mod std_reexports {
        #[doc = crate::_tags!(data_structure hash set)]
        #[doc = super::_hashbrown_or_std!(start)]
        /// An unordered hash set implemented as a `HashMap` where the value is `()`
        #[doc = crate::_doc_meta!{location("data/store/key/set", struct HashSet)}]
        #[doc = super::_hashbrown_or_std!(end)]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use std::collections::HashSet;
    }
}
