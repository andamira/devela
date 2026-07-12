// devela/src/ui/frame/id.rs
//
//! Defines [`UiId`], [`UiKey`], [`UiScope`].
//
// Boundary: keys are author-local inputs; resolved ids are the tokens shared with
// layout, routing, semantics, widgets, and presentation.
//

use crate::{NonMaxU64, SplitMix64, unwrap};

#[doc = crate::_tags!(ui uid)]
/// Stable author-provided UI identity seed.
#[doc = crate::_doc_meta!{
    location("ui/frame"),
    test_size_of(UiKey = 8|64; niche Option),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiKey(NonMaxU64);
impl UiKey {
    /// The root identity seed.
    pub const ROOT: Self = Self(unwrap![some_guaranteed_or_ub NonMaxU64::new(0)]);

    /// Constructs a key from its raw integer value.
    ///
    /// If `raw == u64::MAX`, it is lossily mapped to `u64::MAX - 1`.
    pub const fn new(raw: u64) -> Self {
        Self(NonMaxU64::new_lossy(raw))
    }
    /// Returns the raw integer value.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0.get()
    }
}

#[doc = crate::_tags!(ui uid)]
/// Resolved UI identity within a frame.
#[doc = crate::_doc_meta!{
    location("ui/frame"),
    test_size_of(UiId = 8|64; niche Option),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiId(NonMaxU64);
impl UiId {
    /// The resolved root identity.
    pub const ROOT: Self = Self(unwrap![some_guaranteed_or_ub NonMaxU64::new(0)]);

    /// Constructs an id from its raw integer value.
    ///
    /// If `raw == u64::MAX`, it is lossily mapped to `u64::MAX - 1`.
    pub const fn new(raw: u64) -> Self {
        Self(NonMaxU64::new_lossy(raw))
    }
    /// Returns the raw integer value.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0.get()
    }
}

#[doc = crate::_tags!(ui uid)]
/// Scoped UI identity namespace.
#[doc = crate::_doc_meta!{
    location("ui/frame"),
    test_size_of(UiScope = 8|64),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiScope {
    id: UiId,
}
impl UiScope {
    /// The root identity scope.
    pub const ROOT: Self = Self { id: UiId::ROOT };

    /// Constructs a scope from a resolved id.
    pub const fn new(id: UiId) -> Self {
        Self { id }
    }
    /// Returns the resolved id of this scope.
    pub const fn id(self) -> UiId {
        self.id
    }
    /// Resolves a key inside this scope.
    pub const fn resolve(self, key: UiKey) -> UiId {
        UiId::new(mix_u64(self.id.raw(), key.raw()))
    }
    /// Enters a child scope resolved from `key`.
    pub const fn enter(self, key: UiKey) -> Self {
        Self::new(self.resolve(key))
    }
}

/// Mixes a scope id and key into a resolved id.
//
// NOTES
// - `UiId::ROOT` stays the literal root id.
// - `resolve()` derives a child id from `(scope, key)`, so resolving
//   `UiKey::ROOT` inside `UiScope::ROOT` should not return `UiId::ROOT`.
// - `GOLDEN_GAMMA` prevents the all-zero pair from mixing back to zero.
const fn mix_u64(scope: u64, key: u64) -> u64 {
    SplitMix64::mix64(scope ^ key.wrapping_add(SplitMix64::GOLDEN_GAMMA))
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn key_and_id_preserve_raw_values_except_max() {
        assert_eq!(UiKey::new(0).raw(), 0);
        assert_eq!(UiId::new(0).raw(), 0);
        assert_eq!(UiKey::new(u64::MAX).raw(), u64::MAX - 1);
        assert_eq!(UiId::new(u64::MAX).raw(), u64::MAX - 1);
    }
    #[test]
    fn default_identity_values_are_root() {
        assert_eq!(UiKey::default(), UiKey::ROOT);
        assert_eq!(UiId::default(), UiId::ROOT);
        assert_eq!(UiScope::default(), UiScope::ROOT);
    }
    #[test]
    fn resolving_key_is_deterministic() {
        let scope = UiScope::ROOT;
        let key = UiKey::new(42);
        assert_eq!(scope.resolve(key), scope.resolve(key));
    }
    #[test]
    fn entering_scope_matches_resolved_id() {
        let scope = UiScope::ROOT;
        let key = UiKey::new(7);
        assert_eq!(scope.enter(key).id(), scope.resolve(key));
    }
    #[test]
    fn resolving_root_key_inside_root_scope_derives_child_id() {
        assert_ne!(UiScope::ROOT.resolve(UiKey::ROOT), UiId::ROOT);
    }
}
