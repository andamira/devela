// devela::data::id::key::_reexport_dep

#![allow(unused_imports, unused_macros, reason = "dep_hashbrown|std feature-gate")]

/* from `hashbrown` or `std` */

#[allow(unused_macros)]
macro_rules! hashbrown_or_std {
    (start) => {
        "<span class='stab portability'
        title='re-exported from either `hashbrown` or `std`'>`≡std`</span>"
    };
    (end) => {
        "\n\n*Re-exported from either the [`hashmap`](https://docs.rs/hasmap) crate
        or from [`std::collections`](https::doc.rust-lang.org/std/collections)*.
        \n\n---"
    };
}
use hashbrown_or_std;

// types from hashbrown have preference over those from std.
#[cfg(feature = "dep_hashbrown")]
pub use hashbrown_reexports::*;
#[cfg(feature = "dep_hashbrown")]
mod hashbrown_reexports {
    use super::hashbrown_or_std;

    #[doc = crate::_tags!(hash data_structure)]
    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash map implemented with quadratic probing and SIMD lookup.
    #[doc = crate::_doc_location!("data/id")]
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::HashMap;

    #[doc = crate::_tags!(hash)]
    #[doc = hashbrown_or_std!(start)]
    /// A view into a single entry in a map, which may either be vacant or occupied.
    #[doc = crate::_doc_location!("data/id")]
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::hash_map::Entry as HashMapEntry;

    #[doc = crate::_tags!(hash)]
    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash set implemented as a `HashMap` where the value is `()`
    #[doc = crate::_doc_location!("data/id")]
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::HashSet;
}

#[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
pub use std_reexports::*;
#[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
mod std_reexports {
    #[doc = crate::_tags!(hash data_structure)]
    #[doc = super::hashbrown_or_std!(start)]
    /// An unordered hash map implemented with quadratic probing and SIMD lookup.
    #[doc = crate::_doc_location!("data/id")]
    #[doc = super::hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use std::collections::hash_map::HashMap;

    #[doc = crate::_tags!(hash)]
    #[doc = super::hashbrown_or_std!(start)]
    /// A view into a single entry in a map, which may either be vacant or occupied.
    #[doc = crate::_doc_location!("data/id")]
    #[doc = super::hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use std::collections::hash_map::Entry as HashMapEntry;

    #[doc = crate::_tags!(hash data_structure)]
    #[doc = super::hashbrown_or_std!(start)]
    /// An unordered hash set implemented as a `HashMap` where the value is `()`
    #[doc = crate::_doc_location!("data/id")]
    #[doc = super::hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use std::collections::HashSet;
}

// IMPROVE: move to base_std:

#[doc = crate::_tags!(hash data_structure)]
/// The `HashMap` in the standard library.
#[doc = crate::_doc_location!("data/id")]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
pub type HashMapStd<K, V> = std::collections::HashMap<K, V>;

#[doc = crate::_tags!(hash data_structure)]
/// The `HashSet` in the standard library.
#[doc = crate::_doc_location!("data/id")]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
pub type HashSetStd<T> = std::collections::HashSet<T>;

#[cfg(all(feature = "hash", any(feature = "std", feature = "dep_hashbrown")))]
pub use aliases::*;
#[cfg(all(feature = "hash", any(feature = "std", feature = "dep_hashbrown")))]
#[cfg_attr(
    nightly_doc,
    doc(cfg(all(
        feature = "hash",
        any(feature = "std", all(feature = "dep_hashbrown", feature = "hash"))
    )))
)]
mod aliases {
    use super::{HashMap, HashSet};
    use crate::HasherBuildFx;

    #[doc = crate::_tags!(hash data_structure)]
    /// A [`HashMap`] using a default Fx hasher.
    #[doc = crate::_doc_location!("data/id")]
    ///
    /// To create with a reserved capacity,
    /// use `HashMapFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashMapFx<K, V> = HashMap<K, V, HasherBuildFx>;

    #[doc = crate::_tags!(hash data_structure)]
    /// A [`HashSet`] using a default Fx hasher.
    #[doc = crate::_doc_location!("data/id")]
    ///
    /// To create with a reserved capacity,
    /// use `HashSetFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashSetFx<T> = HashSet<T, HasherBuildFx>;
}
