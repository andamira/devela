// devela/src/sys/os/browser/web/access/snapshot.rs

use crate::{AsyncPoll, WebPermission, WebPermissionSet, is};
use crate::{PermissionError, PermissionQuery, PermissionState};

#[doc = crate::_tags!(web)]
/// A point-in-time classification of cached browser permission queries.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebPermissionSnapshot = 24|192),
}]
/// The contained sets form a partial partition of the queried permission kinds:
/// each represented permission belongs to exactly one query-result class, while
/// permissions absent from every class are not represented by the snapshot.
#[must_use]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct WebPermissionSnapshot {
    /// Permissions whose protected operations are authorized.
    pub granted: WebPermissionSet,

    /// Permissions whose authorization remains undecided or requestable.
    pub prompt: WebPermissionSet,

    /// Permissions whose protected operations are not authorized.
    pub denied: WebPermissionSet,

    /// Permissions whose queries have not resolved yet.
    pub pending: WebPermissionSet,

    /// Permissions that cannot be queried by the current browser.
    pub unsupported: WebPermissionSet,

    /// Permissions whose queries failed for another reason.
    pub failed: WebPermissionSet,
}
impl WebPermissionSnapshot {
    /// Returns an empty snapshot containing no cached queries.
    pub const fn new() -> Self {
        Self {
            granted: WebPermissionSet::new(),
            prompt: WebPermissionSet::new(),
            denied: WebPermissionSet::new(),
            pending: WebPermissionSet::new(),
            unsupported: WebPermissionSet::new(),
            failed: WebPermissionSet::new(),
        }
    }
    /// Returns the query classification of `permission`, if present.
    ///
    /// Snapshots produced by devela are always valid. For a manually
    /// constructed invalid snapshot, the first matching category is returned.
    #[must_use]
    pub const fn get(self, permission: WebPermission) -> Option<PermissionQuery> {
        if permission.is_in(self.granted) {
            Some(AsyncPoll::Ready(Ok(PermissionState::Granted)))
        } else if permission.is_in(self.prompt) {
            Some(AsyncPoll::Ready(Ok(PermissionState::Prompt)))
        } else if permission.is_in(self.denied) {
            Some(AsyncPoll::Ready(Ok(PermissionState::Denied)))
        } else if permission.is_in(self.pending) {
            Some(AsyncPoll::Pending)
        } else if permission.is_in(self.unsupported) {
            Some(AsyncPoll::Ready(Err(PermissionError::Unsupported)))
        } else if permission.is_in(self.failed) {
            Some(AsyncPoll::Ready(Err(PermissionError::Failed)))
        } else {
            None
        }
    }
    /// Returns every permission represented by this snapshot.
    pub const fn classified(self) -> WebPermissionSet {
        self.granted
            .union(self.prompt)
            .union(self.denied)
            .union(self.pending)
            .union(self.unsupported)
            .union(self.failed)
    }
    /// Returns permissions that resolved to an authorization state.
    pub const fn resolved(self) -> WebPermissionSet {
        self.granted.union(self.prompt).union(self.denied)
    }
    /// Returns permissions whose query completed, successfully or otherwise.
    pub const fn settled(self) -> WebPermissionSet {
        self.resolved().union(self.unsupported).union(self.failed)
    }
    /// Returns permissions from `scope` absent from this snapshot.
    pub const fn unclassified_in(self, scope: WebPermissionSet) -> WebPermissionSet {
        scope.difference(self.classified())
    }
    /// Returns permissions whose classifications differ from `previous`.
    ///
    /// The snapshots should normally have been produced from the same scope.
    pub const fn changed_since(self, previous: Self) -> WebPermissionSet {
        self.granted
            .symmetric_difference(previous.granted)
            .union(self.prompt.symmetric_difference(previous.prompt))
            .union(self.denied.symmetric_difference(previous.denied))
            .union(self.pending.symmetric_difference(previous.pending))
            .union(self.unsupported.symmetric_difference(previous.unsupported))
            .union(self.failed.symmetric_difference(previous.failed))
    }
    /// Returns whether the classification sets are mutually disjoint.
    pub const fn is_valid(self) -> bool {
        let mut classified = self.granted;
        is! { classified.intersects(self.prompt), return false }
        classified.insert(self.prompt);
        is! { classified.intersects(self.denied), return false }
        classified.insert(self.denied);
        is! { classified.intersects(self.pending), return false }
        classified.insert(self.pending);
        is! { classified.intersects(self.unsupported), return false }
        classified.insert(self.unsupported);
        is! { classified.intersects(self.failed), return false }
        true
    }

    /* private helpers */

    pub(super) fn query(permissions: WebPermissionSet) -> Self {
        let mut snapshot = Self::default();
        permissions.for_each(|permission| {
            snapshot.classify(permission, Some(permission.query()));
        });
        snapshot
    }
    pub(super) fn cached(permissions: WebPermissionSet) -> Self {
        let mut snapshot = Self::default();
        permissions.for_each(|permission| {
            snapshot.classify(permission, permission.cached());
        });
        snapshot
    }
    /// Reclassifies `permission` according to `query`.
    ///
    /// Passing `None` removes the permission from the snapshot.
    const fn classify(&mut self, permission: WebPermission, query: Option<PermissionQuery>) {
        let permission = permission.to_set();
        // Remove the preceding classification, if any.
        self.granted.remove(permission);
        self.prompt.remove(permission);
        self.denied.remove(permission);
        self.pending.remove(permission);
        self.unsupported.remove(permission);
        self.failed.remove(permission);
        // Insert the new classification.
        match query {
            None => {}
            Some(AsyncPoll::Pending) => {
                self.pending.insert(permission);
            }
            Some(AsyncPoll::Ready(Ok(PermissionState::Granted))) => {
                self.granted.insert(permission);
            }
            Some(AsyncPoll::Ready(Ok(PermissionState::Prompt))) => {
                self.prompt.insert(permission);
            }
            Some(AsyncPoll::Ready(Ok(PermissionState::Denied))) => {
                self.denied.insert(permission);
            }
            Some(AsyncPoll::Ready(Err(PermissionError::Unsupported))) => {
                self.unsupported.insert(permission);
            }
            Some(AsyncPoll::Ready(Err(PermissionError::Failed))) => {
                self.failed.insert(permission);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_snapshot_reclassifies() {
        let permission = WebPermission::Camera;
        let permission_set = permission.to_set();
        let mut snapshot = WebPermissionSnapshot::new();
        snapshot.classify(permission, Some(AsyncPoll::Pending));
        assert_eq!(snapshot.pending, permission_set);
        assert_eq!(snapshot.get(permission), Some(AsyncPoll::Pending));
        assert!(snapshot.is_valid());
        snapshot.classify(permission, Some(AsyncPoll::Ready(Ok(PermissionState::Granted))));
        assert!(snapshot.pending.is_empty());
        assert_eq!(snapshot.granted, permission_set);
        assert_eq!(snapshot.get(permission), Some(AsyncPoll::Ready(Ok(PermissionState::Granted))));
        assert!(snapshot.is_valid());
        snapshot.classify(permission, None);
        assert!(snapshot.classified().is_empty());
        assert_eq!(snapshot.get(permission), None);
    }
    #[test]
    fn permission_snapshot_detects_overlap() {
        let camera = WebPermission::Camera.to_set();
        let snapshot = WebPermissionSnapshot {
            granted: camera,
            denied: camera,
            ..WebPermissionSnapshot::new()
        };
        assert!(!snapshot.is_valid());
    }
    #[test]
    fn permission_snapshot_tracks_changes() {
        let permission = WebPermission::Camera;
        let camera = permission.to_set();
        let mut previous = WebPermissionSnapshot::new();
        previous.classify(permission, Some(AsyncPoll::Pending));
        let mut current = WebPermissionSnapshot::new();
        current.classify(permission, Some(AsyncPoll::Ready(Ok(PermissionState::Prompt))));
        assert_eq!(current.changed_since(previous), camera);
    }
}
