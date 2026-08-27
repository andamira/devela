// devela/src/data/store/key/set/_reexport_dep.rs

#![allow(unused_imports, unused_macros, reason = "dep_hashbrown|std feature-gate")]

/* from `hashbrown` or `std` */

#[allow(unused_macros)]
macro_rules! _hashbrown_or_std {
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
use _hashbrown_or_std;

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

#[doc = crate::_tags!(data_structure hash set)]
/// The `HashSet` in the standard library.
#[doc = crate::_doc_meta!{location("data/store/key/set", struct HashSetStd)}]
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
    use super::HashSet;
    use crate::HasherBuildFx;

    #[doc = crate::_tags!(data_structure hash set)]
    /// A [`HashSet`] using a default Fx hasher.
    #[doc = crate::_doc_meta!{location("data/store/key/set", type HashSetFx)}]
    ///
    /// To create with a reserved capacity,
    /// use `HashSetFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashSetFx<T> = HashSet<T, HasherBuildFx>;
}
