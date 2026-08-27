// devela/src/lang/sem/cue.rs
//
//! Defines [`Cue`].
//

use crate::ConstInit;

#[doc = crate::_tags!(lang)]
/// A semantic prompt offered for contextual interpretation.
#[doc = crate::_doc_meta!{
    location("lang/sem/cue", struct Cue),
}]
/// A cue identifies something semantically relevant
/// and may carry an accompanying payload.
///
/// It does not prescribe how it is transported,
/// interpreted, acted upon, or represented.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cue<K, P = ()> {
    /// The semantic key.
    pub key: K,

    /// The accompanying payload.
    pub payload: P,
}

impl<K> Cue<K> {
    /// Creates a cue without an accompanying payload.
    pub const fn new(key: K) -> Self {
        Self { key, payload: () }
    }
}
impl<K, P> Cue<K, P> {
    /// Creates a cue with an accompanying payload.
    pub const fn with_payload(key: K, payload: P) -> Self {
        Self { key, payload }
    }
}

impl<K: ConstInit, P: ConstInit> ConstInit for Cue<K, P> {
    const INIT: Self = Self::with_payload(K::INIT, P::INIT);
}
impl<K: Default, P: Default> Default for Cue<K, P> {
    fn default() -> Self {
        Self::with_payload(K::default(), P::default())
    }
}
