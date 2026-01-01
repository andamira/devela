// devela::data::key::reexports

crate::mod_path!(alloc +pub _c  "../../../libs/base_alloc/src/data/key/reexports.rs");

#[cfg(feature = "alloc")]
mod impls_alloc {
    use super::*;
    use crate::_impl_init;

    // impl ConstInit
    _impl_init![ConstInit: <T> Self::new() => BTreeSet<T>];
    _impl_init![ConstInit: <K, V> Self::new() => BTreeMap<K, V>];
}

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
#[allow(unused_imports)]
use hashbrown_or_std;

// types from hashbrown have preference over those from std.
#[cfg(feature = "dep_hashbrown")]
pub use hashbrown_reexports::*;
#[cfg(feature = "dep_hashbrown")]
mod hashbrown_reexports {
    use super::hashbrown_or_std;

    #[doc = crate::_TAG_DATA_STRUCTURE!()]
    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash map implemented with quadratic probing and SIMD lookup.
    #[doc = crate::_doc!(location: "data/key")]
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::HashMap;

    #[doc = crate::_TAG_DATA_STRUCTURE!()]
    #[doc = hashbrown_or_std!(start)]
    /// A view into a single entry in a map, which may either be vacant or occupied.
    #[doc = crate::_doc!(location: "data/key")]
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::hash_map::Entry as HashMapEntry;

    #[doc = hashbrown_or_std!(start)]
    /// An unordered hash set implemented as a `HashMap` where the value is `()`
    #[doc = crate::_doc!(location: "data/key")]
    #[doc = hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use crate::_dep::hashbrown::HashSet;
}

#[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
pub use std_reexports::*;
#[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
mod std_reexports {
    #[doc = super::hashbrown_or_std!(start)]
    /// An unordered hash map implemented with quadratic probing and SIMD lookup.
    #[doc = crate::_doc!(location: "data/key")]
    #[doc = super::hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use std::collections::hash_map::HashMap;

    #[doc = super::hashbrown_or_std!(start)]
    /// A view into a single entry in a map, which may either be vacant or occupied.
    #[doc = crate::_doc!(location: "data/key")]
    #[doc = super::hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use std::collections::hash_map::Entry as HashMapEntry;

    #[doc = super::hashbrown_or_std!(start)]
    /// An unordered hash set implemented as a `HashMap` where the value is `()`
    #[doc = crate::_doc!(location: "data/key")]
    #[doc = super::hashbrown_or_std!(end)]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
    pub use std::collections::HashSet;
}

/// The `HashMap` in the standard library.
#[doc = crate::_doc!(location: "data/key")]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
pub type HashMapStd<K, V> = std::collections::HashMap<K, V>;
/// The `HashSet` in the standard library.
#[doc = crate::_doc!(location: "data/key")]
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

    /// A [`HashMap`] using a default Fx hasher.
    #[doc = crate::_doc!(location: "data/key")]
    ///
    /// To create with a reserved capacity,
    /// use `HashMapFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashMapFx<K, V> = HashMap<K, V, HasherBuildFx>;

    /// A [`HashSet`] using a default Fx hasher.
    #[doc = crate::_doc!(location: "data/key")]
    ///
    /// To create with a reserved capacity,
    /// use `HashSetFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashSetFx<T> = HashSet<T, HasherBuildFx>;
}
