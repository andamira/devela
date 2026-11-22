#[doc = crate::_tags!(data_structure hash)]
/// The `HashMap` in the standard library.
#[doc = crate::_doc_meta!{location("data/store/key/map", struct HashMapStd)}]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
pub type HashMapStd<K, V> = ::std::collections::HashMap<K, V>;

#[cfg(all(feature = "hash", any(feature = "std", feature = "dep_hashbrown")))]
pub use other::*;
#[cfg(all(feature = "hash", any(feature = "std", feature = "dep_hashbrown")))]
#[cfg_attr(
    nightly_doc,
    doc(cfg(all(
        feature = "hash",
        any(feature = "std", all(feature = "dep_hashbrown", feature = "hash"))
    )))
)]
mod other {
    use crate::{HashMap, HasherBuildFx};

    #[doc = crate::_tags!(data_structure hash)]
    /// A [`HashMap`] using a default Fx hasher.
    #[doc = crate::_doc_meta!{location("data/store/key/map", type HashMapFx)}]
    ///
    /// To create with a reserved capacity,
    /// use `HashMapFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashMapFx<K, V> = HashMap<K, V, HasherBuildFx>;
}
