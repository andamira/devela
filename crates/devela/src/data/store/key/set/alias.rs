#[doc = crate::_tags!(data_structure hash set)]
/// The `HashSet` in the standard library.
#[doc = crate::_doc_meta!{location("data/store/key/set", struct HashSetStd)}]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
pub type HashSetStd<T> = std::collections::HashSet<T>;

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
    use crate::{HashSet, HasherBuildFx};

    #[doc = crate::_tags!(data_structure hash set)]
    /// A [`HashSet`] using a default Fx hasher.
    #[doc = crate::_doc_meta!{location("data/store/key/set", type HashSetFx)}]
    ///
    /// To create with a reserved capacity,
    /// use `HashSetFx::with_capacity_and_hasher(num, Default::default())`.
    pub type HashSetFx<T> = HashSet<T, HasherBuildFx>;
}
